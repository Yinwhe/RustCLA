mod resolve;
mod types;

use log::warn;
use resolve::*;

pub fn parse_rust(file: &str) -> Option<Info> {
    match ResolverR::new(file) {
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

pub fn parse_cxx(file: &str) -> Option<Info> {
    match ResolverC::new(&file, &["-std=c++11"]) {
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
