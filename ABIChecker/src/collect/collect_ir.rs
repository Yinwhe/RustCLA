use cargo_metadata::{MetadataCommand, Package};
use infer;
use log::info;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

use super::helper;

pub fn collect_ir() -> Result<(), String> {
    compile_with_bc()?;
    generate_llvm_bc();

    Ok(())
}

fn cargo() -> Command {
    Command::new(std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo")))
}

// Get the top level crate that we need to analyze
// The returned "package" contains one or many crates (targets)
fn current_crate() -> Result<Package, String> {
    // Path to the `Cargo.toml` file
    let manifest_path = helper::get_arg_flag_value("--manifest-path")
        .map(|m| Path::new(&m).canonicalize().unwrap());

    let mut cmd = MetadataCommand::new();
    if let Some(ref manifest_path) = manifest_path {
        cmd.manifest_path(manifest_path);
    }

    let metadata = if let Ok(metadata) = cmd.exec() {
        metadata
    } else {
        return Err("Could not obtain Cargo metadata; likely an ill-formed manifest".to_string());
    };

    // let current_dir = std::env::current_dir();

    // let package_index = metadata
    //     .packages
    //     .iter()
    //     .position(|package| {
    //         let package_manifest_path = Path::new(&package.manifest_path);
    //         if let Some(ref manifest_path) = manifest_path {
    //             package_manifest_path == manifest_path
    //         } else {
    //             let current_dir = current_dir
    //                 .as_ref()
    //                 .expect("could not read current directory");
    //             let package_manifest_directory = package_manifest_path
    //                 .parent()
    //                 .expect("could not find parent directory of package manifest");
    //             package_manifest_directory == current_dir
    //         }
    //     }).unwrap();
    //     // .unwrap_or_else(|| {
    //     //     show_error(
    //     //         "This seems to be a workspace, which is not supported by cargo-miri".to_string(),
    //     //     )
    //     // });
    // let package = metadata.packages.remove(package_index);

    // println!("Debug: {:?}", metadata.root_package());

    metadata
        .root_package()
        .cloned()
        .ok_or("Could not get root package.".to_string())
}

fn compile_with_bc() -> Result<(), String> {
    let to_be_build = current_crate()?;

    for target in to_be_build.targets.into_iter() {
        info!("target: {:?}", target);

        let mut args = std::env::args().skip(2);

        info!("args: {:?}", args);

        let kind = target
            .kind
            .get(0)
            .expect("badly formatted cargo metadata: target::kind is an empty array");

        let mut cmd = cargo();

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

        // Set these environment variables to generate LLVM bitcode
        cmd.env(
            "RUSTFLAGS",
            "-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld --emit=llvm-ir",
        );
        cmd.env("CC", "clang");
        cmd.env("CFLAGS", "-flto=thin");
        cmd.env("LDFLAGS", "-Wl,-O2 -Wl,--as-needed");

        info!("Command line: {:?}", cmd);

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

    Ok(())
}

fn generate_llvm_bc() {
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
