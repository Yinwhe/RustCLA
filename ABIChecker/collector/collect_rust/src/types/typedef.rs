use serde::{Serialize, Deserialize};

use super::RType;

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub aliased: RType,
}

