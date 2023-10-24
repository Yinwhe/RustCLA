use simplelog::*;
use std::fs::OpenOptions;
use utils::run_ffi;

mod utils;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Warn,
            simplelog::Config::default(),
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open("./rust_ffi.log")
                .unwrap(),
        ),
    ])
    .unwrap();

    run_ffi(1, "undone");
}
