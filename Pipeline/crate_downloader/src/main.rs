use std::fs::OpenOptions;

use simplelog::*;

mod utils;

use utils::run;

const CRATESDIR: &str = "./on_process";
const THREADNUM: usize = 2;
const DBNAME: &str = "clas";

fn main() {
    // Prepare log file
    WriteLogger::init(
        LevelFilter::Warn,
        simplelog::Config::default(),
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("./crates_downloader.log")
            .unwrap(),
    )
    .unwrap();

    // Main Process
    run();
}
