use bindgen_cxx_parser::BindgenError;

mod function;
mod structure;
mod typedef;

pub use function::*;
pub use structure::*;
pub use typedef::*;

// #[derive(Debug)]
pub struct CInfo {
    pub structs: Vec<CStruct>,
    pub funcs: Vec<CFunction>,
    // pub opaques: i32,           // temp
    pub typedefs: Vec<TypeDef>,
}

impl CInfo {
    pub fn empty () -> Self {
        Self {
            structs: Vec::new(),
            funcs: Vec::new(),
            typedefs: Vec::new(),
        }
    }
}

// #[derive(Debug)]
pub enum CollectError {
    BindgenError(BindgenError),
    ResolveError(ResolveError)
}

// #[derive(Debug)]
pub enum ResolveError {
    UnsupportedType(String),
}

impl From<BindgenError> for CollectError {
    fn from(err: BindgenError) -> Self {
        CollectError::BindgenError(err)
    }
}

// #[derive(Debug)]
/// llvm basic types
pub enum CType {
    IntType,
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
            CType::IntType => 2,
            CType::PointerType => 3,
            CType::StructType(_) => 4,
            CType::VecTorType => 5,
        }
    }
}