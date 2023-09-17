use inkwell::{targets::TargetData, values::FunctionValue};

use crate::analysis::structure::AType;

#[derive(Debug)]
pub struct AFunction {
    pub name: Option<String>,
    pub params: Vec<AType>,
    pub ret: Option<AType>,

    pub call_convention: u32,
}

impl AFunction {
    pub fn from_llvm_raw(func: &FunctionValue, target: &TargetData) -> Self {
        let name = func.get_name().to_str().unwrap().to_owned();
        let call_convention = func.get_call_conventions();

        // params info
        let mut fields = Vec::new();
        for param in func.get_type().get_param_types() {
            let ty = AType::from_btype(param, target);

            fields.push(ty);
        }

        // return info
        let ret = if let Some(ret) = func.get_type().get_return_type() {
            let ty = AType::from_btype(ret, target);
            Some(ty)
        } else {
            None
        };

        AFunction {
            name: Some(name),
            params: fields,
            ret,

            call_convention,
        }
    }
}
