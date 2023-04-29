use bindgen_cxx_parser::BindgenError;

mod function;
mod structure;
mod typedef;

pub use function::*;
pub use structure::*;
pub use typedef::*;

#[derive(Debug)]
pub struct CInfo {
    pub structs: Vec<CStruct>,
    pub funcs: Vec<CFunction>,
    pub opaques: i32,           // temp
    pub typedefs: Vec<TypeDef>,
}

#[derive(Debug)]
pub enum CollectError {
    BindgenError(BindgenError),
    ResolveError(ResolveError)
}

#[derive(Debug)]
pub enum ResolveError {
    UnsupportedType(String),
}

impl From<BindgenError> for CollectError {
    fn from(err: BindgenError) -> Self {
        CollectError::BindgenError(err)
    }
}

#[derive(Debug)]
/// llvm basic types
pub enum CType {
    IntType,
    FloatType,
    ArrayType,
    PointerType,
    StructType(CStruct),
    VecTorType,
}
