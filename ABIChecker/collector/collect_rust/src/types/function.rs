use crate::RField;

#[derive(Debug)]
pub struct RFunction {
    pub name: String,
    pub mangled_name: String,
    pub args: Vec<RField>,
    pub ret: Option<RField>,
}