use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use inkwell::targets::TargetMachine;
use target::host_target;

mod analysis;
mod target;
mod utils;

#[derive(Parser)]
pub struct Args {
    /// C LLVM IR Files
    #[clap(short, long)]
    c_ir_file: PathBuf,
    /// Rust LLVM IR Files
    #[clap(short, long)]
    r_ir_file: PathBuf,
}

#[derive(Debug)]
pub struct Opts {
    c_ir_file: PathBuf,
    r_ir_file: PathBuf,

    target_machine: MaybeUninit<TargetMachine>,
}

impl From<Args> for Opts {
    fn from(args: Args) -> Self {
        Opts {
            c_ir_file: args.c_ir_file,
            r_ir_file: args.r_ir_file,

            target_machine: MaybeUninit::uninit(),
        }
    }
}

pub fn binary() {
    let args = Args::parse();
    let mut opts = Opts::from(args);
    let start = Instant::now();

    if let Err(e) = precheck() {
        utils::error_prompt("Precheck", &e, true);
        utils::error_prompt("Exit", &format!("time spent: {:?}", start.elapsed()), true);

        return;
    }

    // Resolve and analysis IR
    let target_machine = host_target();
    opts.target_machine.write(target_machine);

    utils::info_prompt("Analysis", "start analyzing ir...", true);
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
\tcxx modules: {}",
            aview.func_warns,
            aview.func_errors,
            aview.struct_warns,
            aview.struct_errors,
            aview.has_rust_modules,
            aview.has_cxx_modules,
        ),
        true,
    );

    utils::info_prompt("Exit", &format!("time spent: {:?}", start.elapsed()), true);
}

/// Always Ok here.
fn precheck() -> Result<(), String> {
    // Now we have no prechecks
    Ok(())
}
