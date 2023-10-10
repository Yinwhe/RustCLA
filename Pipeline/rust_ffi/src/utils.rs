use std::env::current_dir;
use std::panic::catch_unwind;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{panic, thread};

use crossbeam::channel;
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use postgres::{Client, NoTls};
use regex::Regex;

struct VersionInfo {
    crate_id: i32,
    version_id: i32,
    crate_name: String,
    version_num: String,
}

const THREAD_DATA_SIZE: i64 = 20;

pub fn run_ffi(workers: usize, status: &str) {
    let conn = Arc::new(Mutex::new(
        Client::connect(
            "host=localhost dbname=clas user=postgres password=postgres",
            NoTls,
        )
        .unwrap(),
    ));

    println!("DB Prebuild...");
    prebuild(Arc::clone(&conn));

    // Create channel
    println!("Creating Channel");
    let (tx, rx) = channel::bounded(workers);

    // Create threads
    let mut handles = Vec::new();
    for i in 0..workers {
        let conn = Arc::clone(&conn);
        let rx = rx.clone();

        handles.push(thread::spawn(move || {
            while let Ok(versions) = rx.recv() {
                for v in versions {
                    let v = v as VersionInfo;
                    let old_hook = panic::take_hook();
                    panic::set_hook({
                        let conn = Arc::clone(&conn);
                        Box::new(move |info| {
                            let err_message = format!("{:?}", info);
                            error!(
                                "Thread {}: Panic occurs, version - {}, info: {}",
                                i, v.version_id, err_message
                            );
                            store_error(Arc::clone(&conn), v.version_id, err_message);
                        })
                    });
                    if catch_unwind(|| {
                        // MAIN OPERATION: ABICHECKER
                        match abichecker(&v) {
                            Ok(issue_found) => {
                                info!(
                                    "check version {} done, {} issues found",
                                    v.version_id, issue_found
                                );
                                store_success(Arc::clone(&conn), v.version_id, issue_found);
                            }
                            Err(e) => {
                                warn!("check version {} fails, due to error: {}", v.version_id, e);
                                store_error(Arc::clone(&conn), v.version_id, format!("{:?}", e));
                            }
                        }
                    })
                    .is_err()
                    {
                        error!("Thread {}: Panic occurs, version - {}", i, v.version_id);
                    }
                    panic::set_hook(old_hook); // Must after catch_unwind
                }
            }
        }));
    }

    // Send jobs
    loop {
        let conn = Arc::clone(&conn);
        let query = format!(
            r#"SELECT crate_id,version_id,crate_name,version_num FROM ffi_versions WHERE version_id in (
                SELECT version_id FROM abicheck_status WHERE status='{}' ORDER BY version_id asc LIMIT {}
                )"#,
            status, THREAD_DATA_SIZE
        );

        let rows = conn.lock().unwrap().query(&query, &[]).unwrap();

        if rows.is_empty() {
            break;
        } else {
            let query = format!(
                r#"UPDATE abicheck_status SET status='processing' WHERE version_id IN (
                    SELECT version_id FROM abicheck_status WHERE status='{}' ORDER BY version_id asc LIMIT {}
                )"#,
                status, THREAD_DATA_SIZE
            );

            conn.lock().unwrap().query(&query, &[]).unwrap();

            let versions: Vec<VersionInfo> = rows
                .iter()
                .map(|row| VersionInfo {
                    crate_id: row.get(0),
                    version_id: row.get(1),
                    crate_name: row.get(2),
                    version_num: row.get(3),
                })
                .collect();

            tx.send(versions).unwrap();
        }
    }

    std::mem::drop(tx);
    for handle in handles {
        // Unsolved problem
        if handle.join().is_err() {
            error!("!!!Thread Crash!!!")
        }
    }

    info!(r#"\\\ !Resolving Done! ///"#);
}

fn prebuild(conn: Arc<Mutex<Client>>) {
    conn.lock()
        .unwrap()
        .query(
            r#"CREATE TABLE IF NOT EXISTS abicheck_status(
            version_id INTEGER,
            status VARCHAR,
            status_details VARCHAR
        );"#,
            &[],
        )
        .unwrap_or_default();

    conn.lock()
        .unwrap()
        .query(
            r#"CREATE TABLE IF NOT EXISTS abicheck_issues(
            version_id INTEGER,
            issue_found VARCHAR
        );"#,
            &[],
        )
        .unwrap_or_default();
}

fn store_error(conn: Arc<Mutex<Client>>, version: i32, message: String) {
    let message = message.replace("'", "''");
    update_process_status(Arc::clone(&conn), version, "fail");
    update_process_details(Arc::clone(&conn), version, &message);
}

fn store_success(conn: Arc<Mutex<Client>>, version: i32, issue_found: usize) {
    update_process_status(Arc::clone(&conn), version, "done");
    update_issue_found(Arc::clone(&conn), version, issue_found);
}

fn update_process_status(conn: Arc<Mutex<Client>>, version_id: i32, status: &str) {
    // warn!("update status");
    conn.lock()
        .unwrap()
        .query(
            &format!(
                "UPDATE abicheck_status SET status = '{}' WHERE version_id = '{}';",
                status, version_id
            ),
            &[],
        )
        .expect("Update process status fails");
}

fn update_process_details(conn: Arc<Mutex<Client>>, version_id: i32, details: &str) {
    conn.lock()
        .unwrap()
        .query(
            &format!(
                "UPDATE abicheck_status SET status_details = '{}' WHERE version_id = '{}';",
                details, version_id
            ),
            &[],
        )
        .expect("Update process status fails");
}

fn update_issue_found(conn: Arc<Mutex<Client>>, version_id: i32, issue_found: usize) {
    conn.lock()
        .unwrap()
        .query(
            &format!(
                "INSERT INTO abicheck_issues (version_id, issue_found) VALUES ('{}', '{}');",
                version_id, issue_found
            ),
            &[],
        )
        .expect("Update issue found fails");
}

fn abichecker(version: &VersionInfo) -> Result<usize, String> {
    let path = current_dir().expect("Cannot get current directory");
    let path = path
        .parent()
        .expect("Cannot get parent directory")
        .join("crate_downloader")
        .join("on_process")
        .join(&version.crate_name)
        .join(&format!("{}-{}", version.crate_name, version.version_num));

    let mut cmd = Command::new("abi_checker");

    cmd.env(
        "RUSTUP_TOOLCHAIN",
        "nightly-2022-08-01-x86_64-unknown-linux-gnu",
    )
    .current_dir(path);

    let output = cmd.output();

    // warn!("{:?}", output);

    let output = output.expect("Cannot run abi_checker");
    if output.status.success() {
        let output = String::from_utf8(output.stdout).unwrap();
        static RE1: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\d+) errors found").unwrap());
        let issue_found = RE1
            .captures(&output)
            .expect("Cannot find issue found")
            .get(1)
            .expect("Cannot get issue found")
            .as_str()
            .parse::<usize>()
            .expect("Cannot parse issue found");

        return Ok(issue_found);
    } else {
        static RE2: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(collect|resolve) ir failed, due to: (.+)").unwrap());
        let output = String::from_utf8(output.stdout).unwrap();
        let err_message = RE2
            .captures(&output)
            .expect("Cannot find error message")
            .get(2)
            .expect("Cannot get error message")
            .as_str();

        return Err(err_message.to_string());
    }
}
