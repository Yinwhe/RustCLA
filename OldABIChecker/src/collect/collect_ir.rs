use cargo_metadata::{Metadata, MetadataCommand, Package};
use infer;
use walkdir::WalkDir;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

use crate::{utils, Opts};

use super::helper;

pub struct CollectOverriew {
    pub bitcode_path: PathBuf,
    pub targets: Vec<String>,
    pub use_cxx: bool,
    pub use_bindgen: bool,
    pub use_cbindgen: bool,
}

/// Top function.
pub fn collect_ir<'a>(opts: &Opts) -> Result<CollectOverriew, String> {
    if opts.clean_first {
        utils::info_prompt("Collect IR", "cleanning old target...", opts.print);
        clean_target()?;
    }

    utils::info_prompt(
        "Collect IR",
        "compiling with bitcode, this may take a while...",
        opts.print,
    );
    let (targets, use_cxx, use_bindgen, use_cbindgen) = compile_with_bc(opts)?;

    if targets.is_empty() {
        return Err("No target to build, and thus no check.".to_string());
    }

    // println!("Debug: {:?}", targets);

    utils::info_prompt("Collect IR", "collecting LLVM bitcode...", opts.print);
    let bitcode_path = generate_llvm_bc(opts, &targets)?;

    // return the path to the bitcode file and the targets.
    Ok(CollectOverriew {
        bitcode_path,
        targets,
        use_cxx,
        use_bindgen,
        use_cbindgen,
    })
}

#[allow(unused)]
/// Clean target cache first.
pub fn clean_target() -> Result<(), String> {
    let mut cmd = Command::new("cargo");
    cmd.arg("clean");

    let output = cmd
        .output()
        .map_err(|e| format!("failed to execute cargo: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "cargo failed with exit code {:?}, details: {:?}",
            output.status.code(),
            output.stderr
        ));
    }

    Ok(())
}
/// Compile the whole crates and generate LLVM bitcode.
fn compile_with_bc(opts: &Opts) -> Result<(Vec<String>, bool, bool, bool), String> {
    let meta = crate_meta()?;
    let mut names = Vec::new();
    let (mut use_cxx, mut use_bindgen, mut use_cbindgen) = (false, false, false);

    for member in meta.workspace_packages() {
        utils::info_prompt(
            "Compiling",
            &format!("start compiling {}...", &member.name),
            opts.print,
        );

        // check deps usage
        for dep in &member.dependencies {
            match dep.name.as_str() {
                "cxx" => use_cxx = true,
                "bindgen" => use_bindgen = true,
                "cbindgen" => use_cbindgen = true,
                _ => continue,
            }
        }

        let manifest_path = PathBuf::from(&member.manifest_path);
        let member_root = manifest_path
            .parent()
            .ok_or("failed to get member root path")?;

        for target in &member.targets {
            // let mut args = std::env::args().skip(2);
            let kind = target
                .kind
                .get(0)
                .ok_or("badly formatted cargo metadata: target::kind is an empty array")?;

            let mut cmd = Command::new("cargo");

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

            // Add cargo opts until first `--`.
            // while let Some(arg) = args.next() {
            //     if arg == "--" {
            //         break;
            //     }
            //     cmd.arg(arg);
            // }
            cmd.arg("--all-features");

            // Set these environment variables to generate LLVM bitcode
            cmd.env(
                "RUSTFLAGS",
                "-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld --emit=llvm-ir",
            );
            cmd.env("CC", "clang");
            cmd.env("CFLAGS", "-flto=thin");

            cmd.env("CXX", "clang");
            cmd.env("CXXFLAGS", "-flto=thin");
            cmd.env("LDFLAGS", "-Wl,-O2 -Wl,--as-needed");
            // library environment
            cmd.env("LIBRARY_PATH", "/usr/lib/gcc/x86_64-linux-gnu/12/:/usr/lib/gcc/x86_64-linux-gnu/12/../../../../x86_64-linux-gnu/lib/x86_64-linux-gnu/12/:/usr/lib/gcc/x86_64-linux-gnu/12/../../../../x86_64-linux-gnu/lib/x86_64-linux-gnu/:/usr/lib/gcc/x86_64-linux-gnu/12/../../../../x86_64-linux-gnu/lib/../lib/:/usr/lib/gcc/x86_64-linux-gnu/12/../../../x86_64-linux-gnu/12/:/usr/lib/gcc/x86_64-linux-gnu/12/../../../x86_64-linux-gnu/:/usr/lib/gcc/x86_64-linux-gnu/12/../../../../lib/:/lib/x86_64-linux-gnu/12/:/lib/x86_64-linux-gnu/:/lib/../lib/:/usr/lib/x86_64-linux-gnu/12/:/usr/lib/x86_64-linux-gnu/:/usr/lib/../lib/:/usr/lib/gcc/x86_64-linux-gnu/12/../../../../x86_64-linux-gnu/lib/:/usr/lib/gcc/x86_64-linux-gnu/12/../../../:/lib/:/usr/lib");

            // Set root dir
            cmd.current_dir(member_root);

            // Execute cmd
            let output = if opts.print {
                cmd.spawn()
                    .map_err(|e| format!("failed to spawn: {}", e))?
                    .wait_with_output()
                    .map_err(|e| format!("failed to wait cargo: {}", e))?
            } else {
                cmd.output()
                    .map_err(|e| format!("failed to execute cargo: {}", e))?
            };

            if !output.status.success() {
                return Err(format!(
                    "cargo failed with exit code {:?}, due to {}",
                    output.status.code(), String::from_utf8_lossy(&output.stderr)
                ));
            }

            names.push(target.name.replace("-", "_"));
        }
    }

    Ok((names, use_cxx, use_bindgen, use_cbindgen))
}

/// Find all the LLVM bitcodes and gather their pathes in a file.
fn generate_llvm_bc(opts: &Opts, targets: &Vec<String>) -> Result<PathBuf, String> {
    let mut llvm_bitcode_paths = Vec::new();

    // Path to the root path of the project
    let root_path = std::env::current_dir().map_err(|e| format!("{}", e))?;

    // Get `*.ll` files in `target/debug/deps` and convert them to `*.bc`
    let deps_path = root_path.join("target/debug/deps");
    for entry in WalkDir::new(deps_path.clone()).follow_links(true) {
        let entry = entry.map_err(|e| format!("failed to walk dir: {}", e))?;
        // println!("{:?}", entry);
        let file_name = entry
            .file_name()
            .to_str()
            .ok_or("failed to walk dir: get file name errors")?;

        if file_name.ends_with(".ll") {
            // only check what we need
            let name_only = helper::strip_hash(file_name);
            if !targets.contains(&name_only) {
                continue;
            }

            let bc_file_name = file_name.replace(".ll", ".bc");
            let bc_file_path = deps_path.join(bc_file_name);

            let output = Command::new("llvm-as")
                .arg(entry.path())
                .arg("-o")
                .arg(&bc_file_path)
                .output()
                .map_err(|e| format!("failed to execute llvm-as: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "llvm-as failed with exit code {:?}, due to {:?}",
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
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

    let loop_dirs: Vec<PathBuf> = WalkDir::new(build_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(_) => None,
            Err(e) => {
                if let Some(p) = e.loop_ancestor() {
                    Some(p.to_owned())
                } else {
                    None
                }
            }
        })
        .collect();

    for entry in WalkDir::new(build_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_entry(|dir| {
            // skip loop dirs
            if loop_dirs.contains(&dir.path().to_path_buf()) {
                return false;
            }

            // root is ok
            if dir.depth() == 0 {
                return true;
            } else if dir.depth() == 1 {
                // skip non-needed dirs
                let name = helper::strip_hash(
                    dir.file_name()
                        .to_str()
                        .expect("Fatal, failed to get dir name."),
                )
                .replace("-", "_");

                // println!("Debug<for entry> {:?} {:?}", name, targets);

                if targets.contains(&name) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return true;
            }
        })
    {
        let entry = entry.map_err(|e| format!("failed to walk dir: {}", e))?;

        // Make sure the path is a file instead of a directory
        if entry.path().is_file() {
            // Make sure the file type is known
            if let Some(kind) = infer::get_from_path(entry.path())
                .map_err(|e| format!("failed to infer file type: {}", e))?
            {
                // Make sure the file type is LLVM bitcode
                if kind.mime_type() == "application/x-llvm" {
                    llvm_bitcode_paths.push(entry.path().to_path_buf())
                }
            }
        }
    }

    // Write LLVM bitcode paths to a file
    let file_path = root_path.join("target/bitcode_paths");
    let mut file = File::create(&file_path)
        .map_err(|e| format!("failed to create bitcode path file: {}", e))?;
    for bitcode_path in llvm_bitcode_paths {
        file.write_all(format!("{}\n", &bitcode_path.to_string_lossy()).as_bytes())
            .map_err(|e| e.to_string())?;

        if opts.ir_files {
            // Prepare llvm ir codes for debug.
            let file_name = helper::strip_hash(
                &bitcode_path
                    .file_stem()
                    .ok_or("Failed to get file stem")?
                    .to_string_lossy()
                    .to_string(),
            ) + ".ll";

            // let mut cmd = Command::new("llvm-dis");
            // cmd.arg(format!("{}", &bitcode_path.to_string_lossy()))
            //     .arg("-o")
            //     .arg(root_path.join(file_name));

            // println!("{:?}", cmd);
            // let res = cmd.output();
            // println!("{:?}", res);
            let _ = Command::new("llvm-dis")
                .arg(format!("{}", &bitcode_path.to_string_lossy()))
                .arg("-o")
                .arg(root_path.join(file_name))
                .output();
        }
    }

    Ok(file_path)
}

fn crate_meta() -> Result<Metadata, String> {
    // let mut cmd = Command::new("rustup");
    // cmd.args(["toolchain", "list"]);

    // cmd.spawn();

    let cmd = MetadataCommand::new();

    let metadata = match cmd.exec() {
        Ok(metadata) => metadata,
        Err(e) => {
            return Err(format!(
                "Could not obtain Cargo metadata; likely an ill-formed manifest: {}",
                e
            ))
        }
    };

    Ok(metadata)
}

/// Get the top level crate that we need to analyze.
/// The returned "package" contains one or many crates (targets).
fn _current_crate() -> Result<Package, String> {
    // Path to the `Cargo.toml` file
    // let manifest_path = helper::get_arg_flag_value("--manifest-path")
    //     .map(|m| Path::new(&m).canonicalize().unwrap());

    let cmd = MetadataCommand::new();
    // if let Some(ref manifest_path) = manifest_path {
    //     cmd.manifest_path(manifest_path);
    // }

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

    // Historically, if there is no root package, it is a workspace.

    metadata
        .root_package()
        .cloned()
        .ok_or("Could not get root package.".to_string())
}
