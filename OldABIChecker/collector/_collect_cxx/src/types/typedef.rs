use super::CType;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub aliased: CType,
}
