use crate::CType;

#[derive(Debug)]
pub struct TypeDef {
    pub name: String,
    pub aliased: CType,
}

