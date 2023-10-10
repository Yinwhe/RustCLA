use std::fs::{create_dir, remove_dir_all};
use std::io;
use std::path::Path;
use std::process::{Command, Output};
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{anyhow, Result};
use crossbeam::channel::{self};
use downloader::{Download, Downloader};
use log::{error, warn};
use pbr::MultiBar;
use postgres::{Client, NoTls};

use crate::{CRATESDIR, DBNAME, THREADNUM};

#[derive(Debug)]
pub struct CrateInfo {
    pub crate_id: i32,
    pub version_id: i32,
    pub crate_name: String,
    pub version_num: String,
    pub status: String,
}

pub fn run() {
    let conn = Arc::new(Mutex::new(
        Client::connect(
            &format!("host=localhost dbname={DBNAME} user=postgres password=postgres"),
            NoTls,
        )
        .unwrap(),
    ));

    let undownloaded_crates = find_undownloaded_crates(Arc::clone(&conn));
    let workers = THREADNUM;

    let mb = Arc::new(MultiBar::new());
    let mut mpb = mb.create_bar(undownloaded_crates.len() as u64);
    mpb.format("|=>_|");
    mpb.set(0);

    let (tx, rx) = channel::bounded(2 * workers);

    let mut handles = vec![];
    for i in 0..workers {
        let rx = rx.clone();
        let conn = Arc::clone(&conn);
        let mb = Arc::clone(&mb);

        // Thread Operation
        handles.push(thread::spawn(move || {
            let mut pb = mb.create_bar(1);
            let mut downloader = Downloader::builder()
                .download_folder(Path::new(CRATESDIR))
                .parallel_requests(1)
                .build()
                .expect("Fatal Error, build downloader fails!");

            while let Ok(crate_info) = rx.recv() {
                let crate_info: CrateInfo = crate_info;
                pb.format("[++-]");
                pb.set(0);
                pb.message(&(crate_info.crate_name));

                if let Err(e) = fetch_crate(
                    &mut downloader,
                    CRATESDIR,
                    &crate_info.crate_name,
                    &crate_info.version_num,
                ) {
                    warn!("Thread {}: Fetch fails: crate {:?}, {}", i, crate_info, e);
                    store_fails_info(Arc::clone(&conn), crate_info);
                } else if let Err(e) =
                    deal_with_crate(CRATESDIR, &crate_info.crate_name, &crate_info.version_num)
                {
                    warn!("Thread {}: Unzip fails: crate {:?}, {}", i, crate_info, e);
                    store_fails_info(Arc::clone(&conn), crate_info);
                } else {
                    store_success_info(Arc::clone(&conn), crate_info);
                }
            }

            pb.finish();
        }));
    }

    handles.push(thread::spawn(move || mb.listen()));

    // Send data to child thread
    for crate_info in undownloaded_crates {
        tx.send(crate_info).expect("Fatal error, send fails");
        mpb.inc();
    }

    std::mem::drop(tx);
    mpb.finish();

    for handle in handles {
        // Unsolved problem
        if handle.join().is_err() {
            error!("!!!Thread Crash!!!")
        }
    }

    println!(r#"\\\ Done! ///"#)
}

fn fetch_crate(
    downloader: &mut Downloader,
    store_path: &str,
    name: &str,
    version: &str,
) -> Result<()> {
    let mut dls = vec![];

    create_dir(Path::new(&format!("{}/{}", store_path, name))).unwrap_or_default();

    dls.push(
        Download::new(&format!(
            "https://crates.io/api/v1/crates/{name}/{version}/download",
        ))
        .file_name(Path::new(&format!("{name}/{version}.tgz"))),
    );

    let res = downloader.download(&dls)?;

    if res.first().unwrap().is_err() {
        return Err(anyhow!("Download error"));
    }

    return Ok(());
}

fn deal_with_crate(store_path: &str, name: &str, version: &str) -> io::Result<Output> {
    // Decompress
    Command::new("tar")
        .arg("-zxf")
        .arg(format!("{store_path}/{name}/{version}.tgz"))
        .arg("-C")
        .arg(format!("{store_path}/{name}"))
        .output()
}

fn find_undownloaded_crates(conn: Arc<Mutex<Client>>) -> Vec<CrateInfo> {
    conn.lock()
        .unwrap()
        .query(
            r#"CREATE TABLE IF NOT EXISTS public.download_status
            (
                crate_id INT,
                version_id INT,
                crate_name VARCHAR,
                version_num VARCHAR,
                status VARCHAR
            )"#,
            &[],
        )
        .unwrap();

    // Check if table is empty
    if conn
        .lock()
        .unwrap()
        .query("SELECT * FROM download_status LIMIT 1", &[])
        .unwrap()
        .first()
        .is_none()
    {
        // Empty: Select newest ffi_versions and insert into download_status
        conn.lock()
            .unwrap()
            .query(
                "
                INSERT INTO public.download_status 
                SELECT crate_id, version_id, crate_name, version_num, 'undownloaded' as status
                FROM ffi_versions
                WHERE version_id IN (
                    SELECT MAX(version_id)
                    FROM ffi_versions
                    GROUP BY crate_id
                );
                ",
                &[],
            )
            .unwrap();
        remove_dir_all(CRATESDIR).unwrap_or_default(); // Delete tmp crates file directory
        create_dir(Path::new(CRATESDIR)).unwrap_or_default(); // Crates file directory
    }

    let query = format!("SELECT * FROM download_status WHERE status = 'undownloaded'");
    let row = conn.lock().unwrap().query(&query, &[]).unwrap();
    row.iter()
        .map(|ver| CrateInfo {
            crate_id: ver.get(0),
            version_id: ver.get(1),
            crate_name: ver.get(2),
            version_num: ver.get(3),
            status: ver.get(4),
        })
        .collect()
}

fn store_fails_info(conn: Arc<Mutex<Client>>, crates: CrateInfo) {
    conn.lock()
        .unwrap()
        .query(
            &format!(
                "UPDATE download_status SET status = 'fail' WHERE crate_id = '{}' and version_id = '{}';",
                crates.crate_id, crates.version_id
            ),
            &[],
        )
        .expect("Fatal error, store info fails!");
}

fn store_success_info(conn: Arc<Mutex<Client>>, crates: CrateInfo) {
    conn.lock()
        .unwrap()
        .query(
            &format!(
                "UPDATE download_status SET status = 'success' WHERE crate_id = '{}' and version_id = '{}';",
                crates.crate_id, crates.version_id
            ),
            &[],
        )
        .expect("Fatal error, store info fails!");
}
