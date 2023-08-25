use super::LLVMType;


#[derive(Debug)]
pub enum RCStruct {
    RStruct(StructType),
    CStruct(StructType),
}

#[derive(Debug)]
pub enum StructType {
    Struct(Struct),
    Enum(Enum),
    Union(Union),
}

#[derive(Debug)]
pub struct Struct {
    pub name: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Enum {
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct Union {
    pub name: Option<String>,
}


#[derive(Debug)]
pub struct Field {
    pub name: Option<String>,
    pub ty: LLVMType,
}

impl StructType {
    pub fn set_name(&mut self, name: &str) {
        let name = name.to_string();
        match self {
            Self::Struct(st) => st.name = Some(name),
            Self::Enum(em) => em.name = Some(name),
            Self::Union(un) => un.name = Some(name)
        }
    }
}