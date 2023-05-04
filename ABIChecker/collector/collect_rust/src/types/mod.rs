use cbindgen_rust_parser::Error as BindgenError;

mod function;
mod structure;
mod typedef;

pub use function::*;
pub use structure::*;
pub use typedef::*;

#[derive(Debug)]
pub struct RInfo {
    pub structs: Vec<RStruct>,
    pub funcs: Vec<RFunction>,
    // pub opaques: i32,           // temp
    pub typedefs: Vec<TypeDef>,
}

impl RInfo {
    pub fn empty () -> Self {
        Self {
            structs: Vec::new(),
            funcs: Vec::new(),
            typedefs: Vec::new(),
        }
    }
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
pub enum RType {
    IntType,
    FloatType,
    ArrayType,
    PointerType,
    StructType(RStructType),
    VecTorType,
}
