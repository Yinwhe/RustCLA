use serde::{Serialize, Deserialize};

use super::RField;

#[derive(Debug, Serialize, Deserialize)]
pub struct RFunction {
    pub name: String,
    pub args: Vec<RField>,
    pub ret: Option<RField>,
}