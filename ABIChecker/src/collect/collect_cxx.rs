use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use inkwell::{context::Context, module::Module, targets::TargetData, values::FunctionValue};
use lazy_static::lazy_static;

use crate::{analysis_types::*, target::host_target};

use super::{ctypes::*, helper::collect_mangles};

const COLLECT_CXX: &str = "collector/collect_cxx/collect_cxx";

lazy_static! {
    static ref CX: Arc<Mutex<Context>> = Arc::new(Mutex::new(Context::create()));
}

#[inline]
pub fn collect_info_from_cpp_file<'cx>(
    file: &str,
    clang_target: Option<&str>,
    cpp_standard: Option<&str>,
    cx: &'cx Context,
) -> Result<Analysis, String> {
    let res = Command::new(COLLECT_CXX)
        .arg(file)
        .output()
        .expect("failed to execute process");

    let cinfo = if res.status.success() {
        let cinfo: CInfo = serde_json::from_slice(&res.stdout).unwrap();
        cinfo
    } else {
        return Err(format!(
            "collect info from cpp file fails, due to {:?}",
            String::from_utf8_lossy(&res.stderr)
        ));
    };

    generate_bcfile(file)?;

    // parse bc code
    let module = match Module::parse_bitcode_from_path("cpp.bc", cx) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!("parse bitcode from path fails, due to {:?}", e));
        }
    };

    let target_machine = host_target();

    // deal wtih structs
    let mut structs = Vec::new();

    for cst in cinfo.structs {
        match resolve_one_struct(cst, &module, target_machine.get_target_data()) {
            Ok(st) => {
                // resolve ok
                structs.push(st);
            }
            Err(msg) => { // rersolve error

                // TODO
            }
        }
    }

    // deal with functions
    let mut functions = Vec::new();

    let names: Vec<String> = module
        .get_functions()
        .map(|f| f.get_name().to_string_lossy().into_owned())
        .collect();
    let map = collect_mangles(names);

    for cfunc in cinfo.funcs {
        if let Some(Some(funcv)) = map
            .get(&cfunc.name)
            .map(|mangled| module.get_function(mangled))
        {
            match resolve_one_func(cfunc, &funcv) {
                Ok(func) => {
                    functions.push(func);
                }
                Err(msg) => {
                    // TODO
                }
            }
        } else {
            // TODO
        }
    }

    Ok(Analysis { structs, functions })
}

fn generate_bcfile(file: &str) -> Result<(), String> {
    let res = Command::new("clang")
        .args(&["-c", "-emit-llvm", "-o", "cpp.bc", file])
        .output()
        .expect("failed to execute process");

    if res.status.success() {
        Ok(())
    } else {
        Err(format!("generate bc file fails, due to {:?}", res.stderr))
    }
}

fn resolve_one_struct(
    cst: CStruct,
    module: &Module,
    target_data: TargetData,
) -> Result<AnalysisStruct, String> {
    let name = if let Some(name) = cst.name.clone() {
        name
    } else {
        return Err(format!(
            "resolve AnalysisStruct fail, anonymous struct unsupported"
        ));
    };

    if cst.is_union {
        let struct_type =
            if let Some(struct_type) = module.get_struct_type(&format!("union.{}", name)) {
                struct_type
            } else {
                return Err(format!(
                    "resolve AnalysisStruct fail, union type {} not found",
                    name
                ));
            };

        let mut ast = AnalysisStruct::from_ctx_raw(struct_type, &target_data);

        ast.is_union = true;
        ast.is_enum = false;
        ast.name = Some(name);

        return Ok(ast);
    } else {
        let struct_type =
            if let Some(struct_type) = module.get_struct_type(&format!("class.{}", name)) {
                struct_type
            } else if let Some(struct_type) = module.get_struct_type(&format!("struct.{}", name)) {
                struct_type
            } else {
                return Err(format!(
                    "resolve AnalysisStruct fail, struct type {} not found",
                    name
                ));
            };

        let mut ast = AnalysisStruct::from_ctx_raw(struct_type, &target_data);

        if let Err(e) = __resolve_one_struct(&mut ast, &cst) {
            return Err(e);
        } else {
            return Ok(ast);
        }
    }
}

fn __resolve_one_struct(ast: &mut AnalysisStruct, cst: &CStruct) -> Result<(), String> {
    let name = cst.name.clone().expect("Fatal error, should not happen");

    let mut index = 0;
    let len = cst.fields.len();
    for rf in &mut ast.fields {
        if index >= len {
            // info parsed done, finsh the remaining fields
            if rf.can_be_anytype() {
                rf.is_padding = true;
            }
            continue;
        }

        let f = &cst.fields[index];

        if f.ty.get_type_id() == rf.get_type_id() {
            rf.name = f.name.clone();
            rf.is_padding = false;
            index += 1;

            // recursive resolve
            if let Some(st) = rf.get_struct_mut() {
                let cst = f.get_struct().expect("Fatal error, should not happen");

                if let Err(e) = __resolve_one_struct(st, cst) {
                    return Err(e);
                }
            }
        } else if rf.can_be_anytype() {
            rf.is_padding = true;
        }
    }

    if index != len {
        return Err(format!(
            "resolve AnalysisStruct fail, {} info not match",
            name
        ));
    }

    ast.is_enum = false;
    ast.is_union = false;
    ast.name = Some(name);

    return Ok(());
}

fn resolve_one_func(cfunc: CFunction, funcv: &FunctionValue) -> Result<AnalysisFunction, String> {
    let name = cfunc.name;
    let mut params = Vec::new();

    // assert!(cfunc.args.len() == funcv.count_params() as usize);
    for param in funcv.get_params() {
        let p = if param.is_pointer_value() {
            AnalysisParameters {
                name: None,
                pass_by: AnalysisPassBy::PointerOrReference,
            }
        } else {
            AnalysisParameters {
                name: None,
                pass_by: AnalysisPassBy::Value,
            }
        };

        params.push(p);
    }

    let ret = if let Some(retv) = funcv.get_type().get_return_type() {
        if retv.is_pointer_type() {
            Some(AnalysisParameters {
                name: None,
                pass_by: AnalysisPassBy::PointerOrReference,
            })
        } else {
            Some(AnalysisParameters {
                name: None,
                pass_by: AnalysisPassBy::Value,
            })
        }
    } else {
        None
    };

    Ok(AnalysisFunction { name, params, ret })
}
