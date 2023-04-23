mod types;
mod resolver;

pub use types::*;
pub use resolver::{parse, bindgen_parse_one};

#[test]
fn test() {
    let res = bindgen_parse_one("test.hpp", &["-std=c++11"]);
}