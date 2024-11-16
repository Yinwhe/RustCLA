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

pub use analysis::AnalysisOverriew;
pub use collect::CollectOverriew;

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
    let cview = match collect::collect_ir(&opts) {
        Ok(view) => view,
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

    opts.bitcode_path.write(cview.bitcode_path);
    opts.target_machin.write(target_machin);

    utils::info_prompt("Analysis", "start analyzing ir", true);
    let aview = match analysis::analysis_ir(opts) {
        Err(e) => {
            utils::error_prompt(
                "Resolve IR Error",
                &format!("resolve ir failed, due to: {}", e),
                true,
            );
            utils::error_prompt("Exit", &format!("time spent: {:?}", start.elapsed()), true);

            return;
        }
        Ok(aview) => aview,
    };

    utils::info_prompt(
        "Summarize",
        &format!(
            "Detect:
\tfunc warns: {}
\tfunc errors: {}
\tstruct warns: {}
\tstruct errors: {}
\trust modules: {}
\tcxx modules: {}
\tuse cxx: {}
\tuse bindgen: {}
\tuse cbindgen: {}",
            aview.func_warns,
            aview.func_errors,
            aview.struct_warns,
            aview.struct_errors,
            aview.has_rust_modules,
            aview.has_cxx_modules,
            cview.use_cxx,
            cview.use_bindgen,
            cview.use_cbindgen,
        ),
        true,
    );

    utils::info_prompt("Exit", &format!("time spent: {:?}", start.elapsed()), true);
}

/// Check ffi issues in library mode.
/// return: func warnings, func errors, struct warnings, struct errors, detect rust modules, detect cxx modules
pub fn lib() -> Result<(CollectOverriew, AnalysisOverriew), String> {
    let args = Args {
        ir_files: false,
        clean_first: false,
    };

    let mut opts = Opts::from(args);
    opts.print = false;
    opts.clean_first = true;

    precheck()?;

    // Collect IR
    let cview = match collect::collect_ir(&opts) {
        Ok(cview) => cview,
        Err(e) => {
            return Err(format!("collect ir failed, due to: {}", e));
        }
    };

    // Resolve and analysis IR
    let target_machin = host_target();

    opts.bitcode_path.write(cview.bitcode_path.clone());
    opts.target_machin.write(target_machin);

    let aview = match analysis::analysis_ir(opts) {
        Ok(issues) => issues,
        Err(e) => {
            return Err(format!("resolve ir failed, due to: {}", e));
        }
    };

    Ok((cview, aview))
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
