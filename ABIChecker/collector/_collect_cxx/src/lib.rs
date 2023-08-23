mod resolver;
mod types;

use clap::Parser;
use log::{debug, warn};
pub use resolver::Resolver;
pub use types::*;

#[derive(Parser, Debug)]
pub struct Args {
    file: String,
    include_path: Vec<String>,
}

/// parse and collect cxx info from file.
pub fn parse(opts: Args) -> Option<CInfo> {
    let mut args = Vec::new();
    // if let Some(target) = clang_target {
    //     args.push_str(&format!("--target={}", target));
    // }
    // if let Some(standard) = cpp_standard {
    //     args.psush_str(&format!("--std={}", standard));
    // } else {
    args.push("-std=c++11".to_string());
    // }

    debug!("opts: {:?}", opts);

    for i in opts.include_path {
        args.push(format!("-I{}", i));
    }

    debug!("args: {:?}", args);

    match Resolver::new(
        &opts.file,
        // &[],
        args.iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    ) {
        Ok(mut resolve) => match resolve.resolve_bindgen_one() {
            Ok(cinfo) => Some(cinfo),
            Err(err) => {
                warn!("Failed to resolve file: {}, due to {:?}", opts.file, err);
                None
            }
        },
        Err(err) => {
            warn!("Failed to parse file: {}, due to {:?}", opts.file, err);
            None
        }
    }
}
