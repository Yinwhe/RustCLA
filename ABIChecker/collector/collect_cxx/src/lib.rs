mod resolver;
mod types;

use log::{debug, warn};
pub use resolver::Resolver;
pub use types::*;

/// parse and collect cxx info from file.
///
/// - `clang_target` is the target triple of the compiler, default to host.
/// - `cpp_standard` is the c++ standard, default to c++11.
pub fn parse(file: &str, clang_target: Option<&str>, cpp_standard: Option<&str>) -> Option<CInfo> {
    let mut args = String::new();
    if let Some(target) = clang_target {
        args.push_str(&format!(" --target={} ", target));
    }
    if let Some(standard) = cpp_standard {
        args.push_str(&format!(" --std={} ", standard));
    } else {
        args.push_str(" --std=c++11 ");
    }

    debug!("args: {}", args);

    match Resolver::new(file, &[&args]) {
        Ok(mut resolve) => match resolve.resolve_bindgen_one() {
            Ok(cinfo) => Some(cinfo),
            Err(err) => {
                warn!("Failed to resolve file: {}, due to {:?}", file, err);
                None
            }
        },
        Err(err) => {
            warn!("Failed to parse file: {}, due to {:?}", file, err);
            None
        }
    }
}
