/// This module will parse codes and resolve them into
/// structure we need.

use crate::types::{StructType, FunctionType};

mod cxx;
mod rust;


#[derive(Debug)]
pub struct Info {
    pub structs: Vec<StructType>,
    pub funcs: Vec<FunctionType>,
    // pub opaques: ?
    // pub typedefs: ?,
}

pub use cxx::Resolver as ResolverC;
pub use rust::Resolver as ResolverR;