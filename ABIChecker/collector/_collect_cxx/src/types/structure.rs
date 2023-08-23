use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use super::CType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CStruct {
    pub name: Option<String>,
    pub fields: Vec<CField>,
    // pub methods: Vec<CFunction>,
    pub is_packed: bool,
    pub is_union: bool,
}

#[cfg(feature = "my_crate")]
impl CStruct {
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CField {
    pub name: Option<String>,
    pub ty: CType,
}

impl CField {
    pub fn get_struct(&self) -> Option<&CStruct> {
        if let CType::StructType(st) = &self.ty {
            Some(st)
        } else {
            None
        }
    }
}