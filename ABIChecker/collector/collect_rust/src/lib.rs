extern crate cbindgen_rust_parser;

mod resolver;
mod types;

use log::warn;

pub use resolver::Resolver;
pub use types::*;

pub fn parse(file: &str) -> Option<RInfo> {
    match Resolver::new(file) {
        Ok(mut resolver) => match resolver.resolve_cbindgen_one() {
            Ok(rinfo) => Some(rinfo),
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
