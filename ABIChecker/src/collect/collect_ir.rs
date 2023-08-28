use cargo_metadata::{MetadataCommand, Package};
use infer;
use log::info;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

use crate::CARGO;

use super::helper;

pub fn collect_ir() -> Result<(), String> {
    clean_target()?;
    compile_with_bc()?;
    generate_llvm_bc()?;

    Ok(())
}

fn cargo() -> Command {
    Command::new(CARGO)
}

/// Clean target cache first.
fn clean_target() -> Result<(), String> {
    let mut cmd = cargo();
    cmd.arg("clean");

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "cargo failed with exit code {:?}",
            output.status.code()
        ));
    }

    Ok(())
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
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute cargo: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "cargo failed with exit code {:?}",
                output.status.code()
            ));
        }
    }

    Ok(())
}

fn generate_llvm_bc() -> Result<(), String> {
    let mut llvm_bitcode_paths = Vec::new();

    // Path to the root path of the project
    let root_path = std::env::current_dir().unwrap();

    // Get `*.ll` files in `target/debug/deps` and convert them to `*.bc`
    let deps_path = root_path.join("target/debug/deps");
    for entry in WalkDir::new(deps_path.clone()).follow_links(true) {
        let entry = entry.expect("Failed to get entry");
        let file_name = entry.file_name().to_str().expect("Failed to get file name");

        if file_name.ends_with(".ll") {
            let bc_file_name = file_name.replace(".ll", ".bc");
            let bc_file_path = deps_path.join(&bc_file_name);

            let output = Command::new("llvm-as")
                .arg(entry.path())
                .arg("-o")
                .arg(&bc_file_path)
                .output()
                .map_err(|e| format!("Failed to execute llvm-as: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "llvm-as failed with exit code {:?}",
                    output.status.code()
                ));
            }

            llvm_bitcode_paths.push(bc_file_path)
        }
    }

    // Get `*.o` files in `target/debug/build`
    // These files are generated by build scripts, usually through the `cc` crate
    // Note that some build scripts also generate object files, so we use `infer` crate
    // to determine whether it is really a LLVM bitcode
    let build_path = root_path.join("target/debug/build");
    for entry in WalkDir::new(build_path.clone()).follow_links(true) {
        let entry = entry.expect("Failed to get entry");
        // Make sure the path is a file instead of a directory
        if entry.path().is_file() {
            // Make sure the file type is known
            if let Some(kind) = infer::get_from_path(entry.path()).expect("Failed to get file type")
            {
                // Make sure the file type is LLVM bitcode
                if kind.mime_type() == "application/x-llvm" {
                    llvm_bitcode_paths.push(entry.path().to_path_buf())
                }
            }
        }
    }

    // Write LLVM bitcode paths to a file
    let file_path = Path::new("target/bitcode_paths");
    let mut file = File::create(file_path).expect("Failed to create file bitcode_paths");
    for bitcode_path in llvm_bitcode_paths {
        file.write_all(format!("{}\n", bitcode_path.to_string_lossy()).as_bytes())
            .unwrap();
    }

    Ok(())
}
