use super::structure::AField;

#[derive(Debug)]
pub struct AFunction {
    pub name: Option<String>,
    pub params: Vec<AField>,
    pub ret: Option<AField>,
}
