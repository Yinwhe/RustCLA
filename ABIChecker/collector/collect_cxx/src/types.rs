use bindgen_cxx_parser::BindgenError;

pub struct CInfo{
    structs: Vec<CStruct>,
    funcs: Vec<CFunction>
}

pub struct CStruct {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: Vec<CFunction>,
}

// pub enum CField {
    
// }

pub struct CFunction {
    pub name: String,
    pub args: Vec<String>,
}

pub enum CollectError {
    BindgenError(BindgenError),
}