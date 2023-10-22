use std::io::{BufRead, BufReader};
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use clap::Parser;
use inkwell::targets::TargetMachine;
use rustc_version::{version_meta, Channel};
use target::host_target;

mod analysis;
mod collect;
mod target;
mod utils;

// #[macro_use]
// extern crate log;

#[derive(Parser)]
pub struct Args {
    #[clap(long, default_value = "false")]
    ir_files: bool,
    #[clap(long, default_value = "false")]
    clean_first: bool,
}

pub struct Opts {
    ir_files: bool,
    clean_first: bool,
    print: bool,

    bitcode_path: MaybeUninit<PathBuf>,
    target_machin: MaybeUninit<TargetMachine>,
}

pub fn binary() {
    pretty_env_logger::init();

    let args = Args::parse();
    let mut opts = Opts::from(args);
    let start = Instant::now();

    if let Err(e) = precheck() {
        utils::error_prompt("Precheck", &e, true);
        utils::error_prompt("Exit", &format!("time spent: {:?}", start.elapsed()), true);

        return;
    }

    // Collect IR
    utils::info_prompt("Collect", "start collecting ir", true);
    let (bitcode_path, _) = match collect::collect_ir(&opts) {
        Ok((bitcode_path, targets)) => (bitcode_path, targets),
        Err(e) => {
            utils::error_prompt(
                "Collect IR Error",
                &format!("collect ir failed, due to: {}", e),
                true,
            );
            utils::error_prompt("Exit", &format!("time spent: {:?}", start.elapsed()), true);

            return;
        }
    };

    // Resolve and analysis IR
    let target_machin = host_target();

    opts.bitcode_path.write(bitcode_path);
    opts.target_machin.write(target_machin);

    utils::info_prompt("Analysis", "start analyzing ir", true);
    let issue = match analysis::analysis_ir(opts) {
        Err(e) => {
            utils::error_prompt(
                "Resolve IR Error",
                &format!("resolve ir failed, due to: {}", e),
                true,
            );
            utils::error_prompt("Exit", &format!("time spent: {:?}", start.elapsed()), true);

            return;
        }
        Ok(issue) => issue,
    };

    utils::info_prompt(
        "Summarize",
        &format!("Detect:\n\tfunc warns: {}\n\tfunc errors: {}\n\tstruct warns: {}\n\tstruct errors: {}\n\trust modules: {}\n\tcxx modules: {}",
        issue.0, issue.1, issue.2, issue.3, issue.4, issue.5),
        true,
    );

    utils::info_prompt("Exit", &format!("time spent: {:?}", start.elapsed()), true);
}

/// Check ffi issues in library mode.
/// return: func warnings, func errors, struct warnings, struct errors, detect rust modules, detect cxx modules
pub fn lib() -> Result<(usize, usize, usize, usize, bool, bool), String> {
    let args = Args {
        ir_files: false,
        clean_first: false,
    };

    let mut opts = Opts::from(args);
    opts.print = false;
    opts.clean_first = true;

    precheck()?;

    // Collect IR
    let (bitcode_path, _) = match collect::collect_ir(&opts) {
        Ok((bitcode_path, targets)) => (bitcode_path, targets),
        Err(e) => {
            return Err(format!("collect ir failed, due to: {}", e));
        }
    };

    // Resolve and analysis IR
    let target_machin = host_target();

    opts.bitcode_path.write(bitcode_path);
    opts.target_machin.write(target_machin);

    let issues = match analysis::analysis_ir(opts) {
        Ok(issues) => issues,
        Err(e) => {
            return Err(format!("resolve ir failed, due to: {}", e));
        }
    };

    Ok(issues)
}

fn precheck() -> Result<(), String> {
    if let Err(e) = check_is_root() {
        return Err(format!("precheck failed, due to {}", e));
    }

    if let Err(e) = check_toolchain_sync() {
        return Err(format!("precheck failed, due to {}", e));
    }

    if let Err(e) = check_toolchain_version() {
        return Err(format!("precheck failed, due to {}", e));
    }

    Ok(())
}

fn check_is_root() -> Result<(), String> {
    let cargo_toml_path = Path::new("Cargo.toml");

    if !cargo_toml_path.exists() {
        return Err("Cargo.toml not found, not in root path".to_string());
    }

    Ok(())
}

/// If changing toolchain, return error.
fn check_toolchain_sync() -> Result<(), String> {
    let mut child = Command::new("cargo")
        .arg("-vV")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    // let stdout = BufReader::new(child.stdout.take().ok_or("fail to get child's stdout")?);
    let stderr = BufReader::new(child.stderr.take().ok_or("fail to get child's stderr")?);

    for line in stderr.lines() {
        let line = line.unwrap();

        if line.contains("syncing channel updates for") {
            child.kill().unwrap();
            return Err("Trying to switch toolchain".to_string());
        }
    }

    Ok(())
}

fn check_toolchain_version() -> Result<(), String> {
    let version = version_meta().map_err(|e| e.to_string())?;

    if version.channel != Channel::Nightly || version.semver.minor != 64 {
        return Err("Only support 2022-08-01 nightly toolchain".to_string());
    }

    // if version.semver != "nightly-2022-08-01" {
    //     return Err("Only support 2022-08-01 nightly toolchain".to_string());
    // }

    Ok(())
}

impl From<Args> for Opts {
    fn from(args: Args) -> Self {
        Opts {
            ir_files: args.ir_files,
            clean_first: args.clean_first,
            print: true,

            bitcode_path: MaybeUninit::uninit(),
            target_machin: MaybeUninit::uninit(),
        }
    }
}
