use serde::{Serialize, Deserialize};
#[cfg(feature = "my_crate")]
use cbindgen_rust_parser::Error as BindgenError;

mod function;
mod structure;
mod typedef;

pub use function::*;
pub use structure::*;
pub use typedef::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RInfo {
    pub structs: Vec<RStructType>,
    pub funcs: Vec<RFunction>,
    // pub opaques: ?
    // pub typedefs: Vec<TypeDef>,
}

#[cfg(feature = "my_crate")]
#[derive(Debug)]
pub enum CollectError {
    BindgenError(BindgenError),
    ResolveError(ResolveError)
}

#[cfg(feature = "my_crate")]
#[derive(Debug)]
pub enum ResolveError {
    UnsupportedType(String),
}

#[cfg(feature = "my_crate")]
impl From<BindgenError> for CollectError {
    fn from(err: BindgenError) -> Self {
        CollectError::BindgenError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// llvm basic types
pub enum RType {
    IntType(RIntType),
    FloatType,
    ArrayType,
    PointerType,
    StructType(RStructType),
    VecTorType,
}

impl RType {
    pub fn get_type_id(&self) -> u32 {
        match self {
            RType::ArrayType => 0,
            RType::FloatType => 1,
            RType::IntType(_) => 2,
            RType::PointerType => 3,
            RType::StructType(_) => 4,
            RType::VecTorType => 5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RIntType {
    SignedInt,
    UnsignedInt,
    SignedChar,
    UnsignedChar,
    Bool,
    RVoid,
}