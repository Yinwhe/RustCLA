use infer;
use log::info;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

mod utils;

fn cargo() -> Command {
    Command::new(std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo")))
}

fn show_error(msg: String) -> ! {
    eprintln!("fatal error: {}", msg);
    std::process::exit(1)
}


fn main() {
    // Initialize logger
    pretty_env_logger::init();

    info!(
        "cargo-ffi-checker startup command line: {:?}",
        env::args().collect::<Vec<String>>()
    );

    info!("1. Argument ffi-checker detected, constructing cargo commands and setting environment variables...");
    // This arm runs when `cargo ffi-checker` is called. We call `cargo check` for each applicable target,
    // but with the `RUSTC` env var set to the `cargo-ffi-checker` binary so that we come back in the other branch.
    let mut ffi_checker_args = Vec::new();
    in_cargo_ffi_checker(&mut ffi_checker_args);
    generate_llvm_bitcode();
    info!("Entry points collection and LLVM bitcode generation finished!");
}


// This will construct command line like:
// `cargo rustc --bin some_crate_name -v`
// And set the following environment variables:
// `RUSTC_WRAPPER` is set to `cargo-ffi-checker` itself so the execution will come back to the second branch as described above
// `FFI_CHECKER_ARGS` is set to the user-provided arguments for `entry_collector`
// `FFI_CHECKER_TOP_CRATE_NAME` is set to the name of the crate being analyzed
// `FFI_CHECKER_VERBOSE` is set if `-v` is provided
fn in_cargo_ffi_checker(ffi_checker_args: &mut Vec<String>) {
    let verbose: bool = utils::has_arg_flag("-v");

    // This returns a package, which is one or more crates described by a single Cargo.toml
    // E.g., this FFI project is a package that contains 4 crates, a rust-ffi-checker library, and three binaries.
    let current_crate = current_crate();

    // Enumerate each crate referenced by the current project
    for target in current_crate.targets.into_iter() {
        info!("target: {:?}", target);

        let mut args = std::env::args().skip(2);
        let kind = target
            .kind
            .get(0)
            .expect("badly formatted cargo metadata: target::kind is an empty array");

        // Now we run `cargo rustc $FLAGS $ARGS`, giving the user the
        // chance to add additional arguments. `FLAGS` is set to identify
        // this target.  The user gets to control what gets actually passed to `entry_collector`.
        let mut cmd = cargo();
        // Although using `check` may speed up the analysis than using `rustc`, it may cause compilation error:
        // "error: crate `libc` required to be available in rlib format, but was not found in this form"
        // So we use "rustc" here
        cmd.arg("rustc");
        match kind.as_str() {
            "bin" => {
                cmd.arg("--bin").arg(target.name.clone());
            }
            "lib" => {
                cmd.arg("--lib");
            }
            _ => continue,
        }

        // Add cargo args until first `--`.
        while let Some(arg) = args.next() {
            if arg == "--" {
                break;
            }
            cmd.arg(arg);
        }

        // Serialize the remaining args into a special environment variable.
        // This will be read by `inside_cargo_rustc` when we go to invoke
        // our actual target crate.
        // Since we're using "cargo check", we have no other way of passing
        // these arguments.
        // We also add `MIR_CHECKER_TOP_CRATE_NAME` to specify the top-level
        // crate name that we want to analyze, by doing this we can dispatch
        // dependencies to the real `rustc` and top-level crate to `mir-checker`
        // let args_vec: Vec<String> = args.collect();
        *ffi_checker_args = args.collect();
        // cmd.env(
        //     "FFI_CHECKER_ARGS",
        //     serde_json::to_string(&args_vec).expect("failed to serialize args"),
        // );
        // info!(
        //     "Setting env: FFI_CHECKER_ARGS={}",
        //     serde_json::to_string(&args_vec).expect("failed to serialize args")
        // );

        cmd.env("FFI_CHECKER_TOP_CRATE_NAME", target.name.clone());
        info!(
            "Setting env: FFI_CHECKER_TOP_CRATE_NAME={}",
            target.name.clone()
        );

        // // Replace the rustc executable through RUSTC_WRAPPER environment variable
        // let path = std::env::current_exe().expect("current executable path invalid");
        // cmd.env("RUSTC_WRAPPER", path.clone());
        // info!("Setting env: RUSTC_WRAPPER={:?}", path);

        // Set these environment variables to generate LLVM bitcode
        cmd.env(
            "RUSTFLAGS",
            "-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld --emit=llvm-ir",
        );
        cmd.env("CC", "clang");
        cmd.env("CFLAGS", "-flto=thin");
        cmd.env("LDFLAGS", "-Wl,-O2 -Wl,--as-needed");

        info!("Command line: {:?}", cmd);
        if verbose {
            cmd.env("FFI_CHECKER_VERBOSE", ""); // this makes `inside_cargo_rustc` verbose.
            eprintln!("+ {:?}", cmd);
        }

        // Execute cmd
        let exit_status = cmd
            .spawn()
            .expect("could not run cargo")
            .wait()
            .expect("failed to wait for cargo?");

        if !exit_status.success() {
            std::process::exit(exit_status.code().unwrap_or(-1))
        }
    }
}

// Get the top level crate that we need to analyze
// The returned "package" contains one or many crates (targets)
fn current_crate() -> cargo_metadata::Package {
    // We need to get the manifest, and then the metadata, to enumerate targets.

    // Path to the `Cargo.toml` file
    let manifest_path =
        utils::get_arg_flag_value("--manifest-path").map(|m| Path::new(&m).canonicalize().unwrap());

    let mut cmd = cargo_metadata::MetadataCommand::new();
    if let Some(ref manifest_path) = manifest_path {
        cmd.manifest_path(manifest_path);
    }
    let mut metadata = if let Ok(metadata) = cmd.exec() {
        metadata
    } else {
        show_error("Could not obtain Cargo metadata; likely an ill-formed manifest".to_string());
    };

    let current_dir = std::env::current_dir();

    let package_index = metadata
        .packages
        .iter()
        .position(|package| {
            let package_manifest_path = Path::new(&package.manifest_path);
            if let Some(ref manifest_path) = manifest_path {
                package_manifest_path == manifest_path
            } else {
                let current_dir = current_dir
                    .as_ref()
                    .expect("could not read current directory");
                let package_manifest_directory = package_manifest_path
                    .parent()
                    .expect("could not find parent directory of package manifest");
                package_manifest_directory == current_dir
            }
        })
        .unwrap_or_else(|| {
            show_error(
                "This seems to be a workspace, which is not supported by cargo-miri".to_string(),
            )
        });
    let package = metadata.packages.remove(package_index);

    package
}



/// Prepare all the LLVM bitcode (convert `.ll` to `.bc` if needed)
/// Write the paths of bitcode in files
fn generate_llvm_bitcode() {
    let mut llvm_bitcode_paths = Vec::new();

    // Path to the root path of the project
    let root_path = std::env::current_dir().unwrap();

    // Get `*.ll` files in `target/debug/deps` and convert them to `*.bc`
    let deps_path = root_path.join("target/debug/deps");
    for entry in WalkDir::new(deps_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_str().unwrap();
        if f_name.ends_with(".ll") {
            // Convert LLVM IR (*.ll) into LLVM bitcode (*.bc)
            let mut llvm_as_cmd = Command::new("llvm-as");
            let f_name_owned = f_name.to_string();
            // Replace the suffix ".ll" with ".bc"
            // TODO the code is messy, and should be simplified
            let mut f_name_bc = f_name_owned
                .chars()
                .take(f_name_owned.len() - 3)
                .collect::<String>();
            f_name_bc.push_str(".bc");
            llvm_as_cmd.arg(deps_path.join(f_name));
            llvm_as_cmd.args(["-o", deps_path.join(f_name_bc.clone()).to_str().unwrap()]);

            // info!("llvm-as command: {:?}", llvm_as_cmd);

            match llvm_as_cmd.status() {
                Ok(exit) => {
                    if !exit.success() {
                        std::process::exit(exit.code().unwrap_or(42));
                    }
                }
                Err(ref e) => panic!("error during llvm-as run: {:?}", e),
            }

            llvm_bitcode_paths.push(deps_path.join(f_name_bc))
        }
    }

    // Get `*.o` files in `target/debug/build`
    // These files are generated by build scripts, usually through the `cc` crate
    // Note that some build scripts also generate object files, so we use `infer` crate
    // to determine whether it is really a LLVM bitcode
    let build_path = root_path.join("target/debug/build");
    for entry in WalkDir::new(build_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        // Make sure the path is a file instead of a directory
        if entry.path().is_file() {
            // Make sure the file type is known
            if let Some(kind) = infer::get_from_path(entry.path()).expect("file read successfully")
            {
                // Make sure the file type is LLVM bitcode
                if kind.mime_type() == "application/x-llvm" {
                    llvm_bitcode_paths.push(entry.path().to_path_buf())
                }
            }
        }
    }

    // info!("LLVM bitcode paths: {:?}", llvm_bitcode_paths);

    // Write LLVM bitcode paths to a file
    let file_path = Path::new("target/bitcode_paths");
    let mut file = File::create(file_path).expect("Failed to create file bitcode_paths");
    for bitcode_path in llvm_bitcode_paths {
        file.write_all(format!("{}\n", bitcode_path.to_string_lossy()).as_bytes())
            .unwrap();
    }
}
