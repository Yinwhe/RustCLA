use inkwell::values::FunctionValue;

use crate::analysis::structure::AType;

use super::structure::AField;

#[derive(Debug)]
pub struct AFunction {
    pub name: Option<String>,
    pub params: Vec<AField>,
    pub ret: Option<AField>,
}

impl AFunction {
    pub fn from_llvm_raw(func: &FunctionValue) -> Self {
        let name = func.get_name().to_str().unwrap().to_owned();

        // params info
        let mut fields = Vec::new();
        for param in func.get_type().get_param_types() {
            let ty = AType::from(param);

            fields.push(AField {
                name: None,
                is_padding: None,
                ty,
                range: (0, 0),
            })
        }

        // return info
        let ret = if let Some(ret) = func.get_type().get_return_type() {
            let ty = AType::from(ret);
            Some(AField {
                name: None,
                is_padding: None,
                ty,
                range: (0, 0),
            })
        } else {
            None
        };

        AFunction {
            name: Some(name),
            params: fields,
            ret,
        }
    }
}
