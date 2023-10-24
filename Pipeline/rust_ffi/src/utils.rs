use std::env::{current_dir, set_current_dir};
use std::fs::remove_dir_all;
use std::panic::catch_unwind;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{panic, thread};

use abi_checker::{lib, AnalysisOverriew, CollectOverriew};
use crossbeam::channel;
use log::{error, info, warn};
use postgres::{Client, NoTls};

const ON_PROCESS: &str = "/home/ubuntu/Workspace/RustCLA/Pipeline/crate_downloader/on_process";

/// func warnnings, func errors, struct warnnings, struct errors, detect rust ir, detect cxx ir

struct VersionInfo {
    _crate_id: i32,
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

    // do_clean(conn);
    // return ;

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

        let root_path = PathBuf::from(ON_PROCESS);
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
                        info!("Thread {}: check version {} start", i, v.version_id);
                        match abichecker(&v, &root_path) {
                            Ok((cview, aview)) => {
                                info!(
                                    "Thread {}: check version {} done, {:?} issues found",
                                    i, v.version_id, aview,
                                );
                                store_success(Arc::clone(&conn), v.version_id, (cview, aview));
                            }
                            Err(e) => {
                                warn!(
                                    "Thread {}: check version {} fails, due to error: {}",
                                    i, v.version_id, e
                                );
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
                SELECT version_id FROM abicheck_status WHERE status='{}' ORDER BY version_id desc LIMIT {}
                )"#,
            status, THREAD_DATA_SIZE
        );

        let rows = conn.lock().unwrap().query(&query, &[]).unwrap();

        if rows.is_empty() {
            break;
        } else {
            let query = format!(
                r#"UPDATE abicheck_status SET status='processing' WHERE version_id IN (
                    SELECT version_id FROM abicheck_status WHERE status='{}' ORDER BY version_id desc LIMIT {}
                )"#,
                status, THREAD_DATA_SIZE
            );

            conn.lock().unwrap().query(&query, &[]).unwrap();

            let versions: Vec<VersionInfo> = rows
                .iter()
                .map(|row| VersionInfo {
                    _crate_id: row.get(0),
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
            func_warns INTEGER,
            func_errors INTEGER,
            struct_warns INTEGER,
            struct_errors INTEGER,
            has_rust_ir BOOL,
            has_cxx_ir BOOL,
            use_cxx BOOL,
            use_bindgen BOOL,
            use_cbindgen BOOL
        );"#,
            &[],
        )
        .unwrap_or_default();

    // Check if table is empty
    if conn
        .lock()
        .unwrap()
        .query("SELECT * FROM abicheck_status LIMIT 1", &[])
        .unwrap()
        .first()
        .is_none()
    {
        // Empty: Select newest ffi_versions and insert into download_status
        conn.lock()
            .unwrap()
            .query(
                "
                INSERT INTO abicheck_status
                SELECT version_id, 'undone'
                FROM download_status
                ",
                &[],
            )
            .unwrap();
    } else {
        conn.lock()
            .unwrap()
            .query(
                &format!(
                    r#"UPDATE abicheck_status SET status='undone' WHERE status = 'processing'"#
                ),
                &[],
            )
            .unwrap();
    }

    // Clear all envs but PATH
    std::env::vars().for_each(|(key, _)| {
        if key != "PATH" {
            std::env::remove_var(key);
        }
    });

    let path = current_dir()
        .expect("Cannot get current directory")
        .parent()
        .expect("Cannot get parent directory")
        .join("crate_downloader")
        .join("on_process");

    let mut cmd = Command::new("rustup");
    cmd.args([
        "override",
        "set",
        "nightly-2022-08-01-x86_64-unknown-linux-gnu",
    ])
    .current_dir(path);

    let output = cmd.output().expect("Cannot run rustup");
    if !output.status.success() {
        panic!("Cannot set rustup override");
    }
}

fn store_error(conn: Arc<Mutex<Client>>, version: i32, message: String) {
    let message = message.replace("'", "''");
    update_process_status(Arc::clone(&conn), version, "fail");
    update_process_details(Arc::clone(&conn), version, &message);
}

fn store_success(
    conn: Arc<Mutex<Client>>,
    version: i32,
    issue_found: (CollectOverriew, AnalysisOverriew),
) {
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

fn update_issue_found(
    conn: Arc<Mutex<Client>>,
    version_id: i32,
    issue_found: (CollectOverriew, AnalysisOverriew),
) {
    conn.lock()
        .unwrap()
        .query(
            &format!(
                "INSERT INTO abicheck_issues (version_id, func_warns, func_errors, struct_warns, struct_errors, has_rust_ir, has_cxx_ir, use_cxx, use_bindgen, use_cbindgen)
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');",
                version_id, issue_found.1.func_warns, issue_found.1.func_errors, issue_found.1.struct_warns, issue_found.1.struct_errors, issue_found.1.has_rust_modules, issue_found.1.has_cxx_modules, issue_found.0.use_cxx, issue_found.0.use_bindgen, issue_found.0.use_cbindgen
            ),
            &[],
        )
        .expect("Update issue found fails");
}

fn abichecker(
    version: &VersionInfo,
    root_path: &PathBuf,
) -> Result<(CollectOverriew, AnalysisOverriew), String> {
    let root = root_path
        .join(&version.crate_name)
        .join(&format!("{}-{}", version.crate_name, version.version_num));
    let target = root.join("target");

    set_current_dir(root).map_err(|e| e.to_string())?;
    let res = abi_checker::lib();

    remove_dir_all(target).unwrap_or_default();

    return res;
}

pub fn do_clean(conn: Arc<Mutex<Client>>) {
    let query = format!(
        r#"SELECT crate_id,version_id,crate_name,version_num FROM ffi_versions WHERE version_id in (
            SELECT version_id FROM abicheck_status WHERE status != 'undone'
        );"#,
    );

    let rows = conn.lock().unwrap().query(&query, &[]).unwrap();

    let versions: Vec<VersionInfo> = rows
        .iter()
        .map(|row| VersionInfo {
            _crate_id: row.get(0),
            version_id: row.get(1),
            crate_name: row.get(2),
            version_num: row.get(3),
        })
        .collect();

    for version in versions {
        let target = PathBuf::from(ON_PROCESS)
            .join(&version.crate_name)
            .join(&format!("{}-{}", version.crate_name, version.version_num))
            .join("target");

        info!("Cleaning version {}...", version.version_id);
        remove_dir_all(target).unwrap_or_default();
    }

    info!("Cleaning done!");
}
