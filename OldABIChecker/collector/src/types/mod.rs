/// This module defines types we used to express structures and
/// functions in rust or c/c++

use bindgen_cxx_parser::BindgenError as CSideBindgenError;
use cbindgen_rust_parser::Error as RSideBindgenError;

mod structure;
mod function;

pub use structure::*;
pub use function::*;

#[derive(Debug)]
pub enum CollectError {
    BindgenError(BindgenError),
    // ResolveError(ResolveError)
}


// #[derive(Debug)]
// pub enum ResolveError {
//     UnsupportedType(String),
// }

#[derive(Debug)]
pub enum BindgenError {
    CSide(CSideBindgenError),
    RSide(RSideBindgenError),
}



#[derive(Debug)]
pub enum LLVMType {
    IntType(DetailIntType),
    FloatType,
    ArrayType,
    PointerType,
    StructType(StructType),
    VecTorType,
}


#[derive(Debug)]
pub enum DetailIntType {
    SignedInt,
    UnsignedInt,
    SignedChar,
    UnsignedChar,
    Bool,
    Void,
    CEnum,
}


impl From<CSideBindgenError> for CollectError {
    fn from(err: CSideBindgenError) -> Self {
        Self::BindgenError(BindgenError::CSide(err))
    }
}

impl From<RSideBindgenError> for CollectError {
    fn from(err: RSideBindgenError) -> Self {
        Self::BindgenError(BindgenError::RSide(err))
    }
}