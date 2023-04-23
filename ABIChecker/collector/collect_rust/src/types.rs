use cbindgen_rust_parser::Error as BindgenError;

pub struct RInfo{
    pub structs: Vec<RStruct>,
    pub funcs: Vec<RFunction>
}

pub struct RStruct {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: Vec<RFunction>,
}

// pub enum CField {
    
// }

pub struct RFunction {
    pub name: String,
    pub args: Vec<String>,
}

pub enum CollectError {
    BindgenError(BindgenError),
}