use serde::{Serialize, Deserialize};
#[cfg(feature = "my_crate")]
use bindgen_cxx_parser::BindgenError;

mod function;
mod structure;
mod typedef;

pub use function::*;
pub use structure::*;
pub use typedef::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CInfo {
    pub structs: Vec<CStruct>,
    pub funcs: Vec<CFunction>,
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
pub enum CType {
    IntType(CIntType),
    FloatType,
    ArrayType,
    PointerType,
    StructType(CStruct),
    VecTorType,
}

impl CType {
    pub fn get_type_id(&self) -> u32 {
        match self {
            CType::ArrayType => 0,
            CType::FloatType => 1,
            CType::IntType(_) => 2,
            CType::PointerType => 3,
            CType::StructType(_) => 4,
            CType::VecTorType => 5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CIntType {
    SignedInt,
    UnsignedInt,
    SignedChar,
    UnsignedChar,
    Bool,
    CVoid,
    CEnum,
}