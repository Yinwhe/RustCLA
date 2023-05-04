use std::fmt::Debug;

use crate::RType;

#[derive(Debug)]
pub enum RStructType {
    RStruct(RStruct),
    REnum(REnum),
    RUnion(RUnion)
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

#[derive(Debug)]
pub struct REnum {
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct RUnion {
    pub name: Option<String>,
}