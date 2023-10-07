use inkwell::{targets::TargetData, types::AnyType, values::FunctionValue};

use crate::analysis::structure::AType;

#[derive(Debug)]
pub struct AFunction<'ctx> {
    pub name: Option<String>,
    pub params: Vec<AType<'ctx>>,
    pub ret: Option<AType<'ctx>>,

    pub call_convention: u32,
}

impl<'ctx> AFunction<'ctx> {
    pub fn from_llvm_raw(func: &FunctionValue<'ctx>, target: &TargetData) -> Self {
        let name = func.get_name().to_str().unwrap().to_owned();
        let call_convention = func.get_call_conventions();

        // params info
        let mut fields = Vec::new();
        for param in func.get_type().get_param_types() {
            let ty = AType::from_anytype(param.as_any_type_enum(), target, false);

            fields.push(ty);
        }

        // return info
        let ret = if let Some(ret) = func.get_type().get_return_type() {
            let ty = AType::from_anytype(ret.as_any_type_enum(), target, false);
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
