use bindgen_cxx_parser::{context::BindgenContext, BindgenError};

use crate::types::{CInfo, CollectError};

pub fn parse(files: &[&str], clang_args: &[&str]) -> Result<CInfo, CollectError> {
    unimplemented!()
}

/// Use `bindgen` parser to parse cxx codes
pub fn bindgen_parse_one(file: &str, clang_args: &[&str]) -> Result<BindgenContext, BindgenError> {
    let builder = bindgen_cxx_parser::Builder::default()
        .clang_args(clang_args)
        .header(file);

    builder.generate()
}
