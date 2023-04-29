use std::fmt::Debug;

use crate::CType;

#[derive(Debug)]
pub struct CStruct {
    pub name: Option<String>,
    pub fields: Vec<CField>,
    // pub methods: Vec<CFunction>,
    pub is_packed: bool,
    pub is_union: bool,
}

impl CStruct {
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned())
    }
}


#[derive(Debug)]
pub struct CField {
    pub name: Option<String>,
    pub ty: CType,
}

/// Infact we do not support enum check
pub struct CEnum {
    pub name: Option<String>,
}