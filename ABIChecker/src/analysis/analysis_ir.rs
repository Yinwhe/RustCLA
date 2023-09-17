use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;

use crate::analysis::function::AFunction;
use crate::analysis::structure::AType;
use crate::utils;

use super::ir_info::IRInfo;
use super::result::{AResult, SigMismatchType};

/// Top analysis function.
pub fn analysis_ir(
    path: PathBuf,
    targets: Vec<String>,
    target_machin: TargetMachine,
) -> Result<(), String> {
    let cx = Context::create();

    utils::info_prompt("Analysis IR", "forming ir information db...");
    let mut ir_info = resolve_from_bc(path, targets, &cx, target_machin)?;
    ir_info.get_ffi()?;

    let ffi_funcs = ir_info.ffi_functions();

    utils::info_prompt(
        "Analysis IR",
        &format!("checking ffi functions: {:?}", ffi_funcs),
    );
    analysis_funcs(ffi_funcs, &ir_info)?;

    Ok(())
}

/// Resolve the IR from the bitcode file and collect all modules.
fn resolve_from_bc<'a>(
    path: PathBuf,
    targets: Vec<String>,
    cx: &'a Context,
    target_machin: TargetMachine,
) -> Result<IRInfo<'a>, String> {
    let file = File::open(path).map_err(|e| format!("{}", e))?;
    let lines = BufReader::new(file).lines();

    // let cx = Context::create();
    let mut c_modules = Vec::new();
    let mut r_modules = Vec::new();

    // read each bitcode file and collect the IR
    for line in lines {
        let line = line.map_err(|e| format!("{}", e))?;
        let path = Path::new(&line);
        let module = Module::parse_bitcode_from_path(path, cx).map_err(|e| format!("{}", e))?;

        // Judge whether it is a C module or a Rust module
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".bc") {
            if targets.iter().any(|t| file_name.starts_with(t)) {
                r_modules.push(module)
            }
            // or it's just dependencies crates
        } else if file_name.ends_with(".o") {
            c_modules.push(module);
        } else {
            panic!("Unsupported file type: {}", file_name);
        }
    }

    let ir_info = IRInfo::new(r_modules, c_modules, target_machin.get_target_data());

    Ok(ir_info)
}

/// Analysis functions.
fn analysis_funcs(ffi_funcs: &Vec<String>, ir_info: &IRInfo) -> Result<(), String> {
    let target = ir_info.get_target_data();
    for f in ffi_funcs {
        utils::info_prompt("Analysis IR", &format!("checking function: {}", f));

        let rf = ir_info
            .r_func(f)
            .expect("Fatal, cannot find the rust function");
        let cf = ir_info
            .c_func(f)
            .expect("Fatal, cannot find the c/c++ function");

        // get analysis functions
        let rf = AFunction::from_llvm_raw(rf, target);
        let cf = AFunction::from_llvm_raw(cf, target);

        if let Some(err) = _analysis_funcs(&rf, &cf) {
            utils::error_prompt("Analysis IR", &format!("{}", err));
            println!("{:?}", rf);
            println!("{:?}", cf);

            continue;
        }

        if let Some(err) = _analysis_funcs_params(&rf, &cf) {
            utils::error_prompt("Analysis IR", &format!("{}", err));
            println!("{:?}", rf);
            println!("{:?}", cf);

            continue;
        }
    }

    Ok(())
}

/// Analysis function usage, that is, params, call convetions, return values, etc.
fn _analysis_funcs(rust_func: &AFunction, c_func: &AFunction) -> Option<AResult> {
    // println!("{:?}", rust_func);
    // println!("{:?}", c_func);

    // check call convention
    if rust_func.call_convention != c_func.call_convention {
        return Some(AResult::func_convention_issue(
            rust_func.call_convention,
            c_func.call_convention,
        ));
    }

    // check params
    if rust_func.params.len() != c_func.params.len() {
        return Some(AResult::func_sig_issue(SigMismatchType::ParamLen));
    }

    let len = rust_func.params.len();
    for i in 0..len {
        let r_param = &rust_func.params[i];
        let c_param = &c_func.params[i];

        if !AType::shallow_check(&r_param, &c_param) {
            return Some(AResult::func_sig_issue(SigMismatchType::ParamType(
                i as u32,
            )));
        }
    }

    // check return value
    if let (Some(r_ret), Some(c_ret)) = (&rust_func.ret, &c_func.ret) {
        if !AType::shallow_check(&r_ret, &c_ret) {
            return Some(AResult::func_sig_issue(SigMismatchType::RetType));
        }
    } else if rust_func.ret.is_none() && c_func.ret.is_none() {
        // do nothing
    } else {
        return Some(AResult::func_sig_issue(SigMismatchType::RetType));
    }

    None
}

fn _analysis_funcs_params(rust_func: &AFunction, c_func: &AFunction) -> Option<AResult> {
    let len = rust_func.params.len();

    for i in 0..len {
        let r_param = &rust_func.params[i];
        let c_param = &c_func.params[i];

        // TODO
    }

    None
}
