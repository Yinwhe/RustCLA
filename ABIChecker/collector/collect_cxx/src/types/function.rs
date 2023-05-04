use crate::CField;

// #[derive(Debug)]
pub struct CFunction {
    pub name: String,
    pub args: Vec<CField>,
    pub ret: Option<CField>,
}