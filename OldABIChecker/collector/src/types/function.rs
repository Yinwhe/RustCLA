use super::Field;

#[derive(Debug)]
pub struct FunctionType {
    pub name: String,
    pub args: Vec<Field>,
    pub ret: Option<Field>,
}