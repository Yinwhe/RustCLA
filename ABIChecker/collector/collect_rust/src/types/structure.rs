use std::fmt::Debug;

use crate::RType;

#[derive(Debug)]
pub enum RStructType {
    RStruct(RStruct),
    REnum(REnum),
    RUnion(RUnion)
}

impl  RStructType {
    pub fn get_name(&self) -> Option<&str> {
        match self {
            RStructType::RStruct(rst) => rst.name.as_deref(),
            RStructType::REnum(re) => re.name.as_deref(),
            RStructType::RUnion(ru) => ru.name.as_deref(),
        }
    }
}

#[derive(Debug)]
pub struct RStruct {
    pub name: Option<String>,
    pub fields: Vec<RField>,
    // pub methods: Vec<RFunction>,
    // pub is_packed: bool,
    // pub is_union: bool,
}

impl RStruct {
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned())
    }
}


#[derive(Debug)]
pub struct RField {
    pub name: Option<String>,
    pub ty: RType,
}

impl RField {
    pub fn get_struct(&self) -> Option<&RStructType> {
        if let RType::StructType(st) = &self.ty {
            Some(st)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct REnum {
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct RUnion {
    pub name: Option<String>,
}