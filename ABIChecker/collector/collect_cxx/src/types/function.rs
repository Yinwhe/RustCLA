use serde::{Serialize, Deserialize};

use super::CField;

#[derive(Debug, Serialize, Deserialize)]
pub struct CFunction {
    pub name: String,
    pub args: Vec<CField>,
    pub ret: Option<CField>,
}