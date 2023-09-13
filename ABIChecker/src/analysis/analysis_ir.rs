use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use inkwell::context::Context;
use inkwell::module::Module;

use crate::analysis::function::AFunction;
use crate::analysis::structure::AType;
use crate::utils;

use super::ir_info::IRInfo;
use super::result::AResult;

/// Top analysis function.
pub fn analysis_ir(path: PathBuf, targets: Vec<String>) -> Result<(), String>{
    let cx = Context::create();

    utils::info_prompt("Analysis IR", "forming ir information db...");
    let mut ir_info = resolve_from_bc(path, targets, &cx)?;
    ir_info.get_ffi()?;

    let ffi_funcs = ir_info.ffi_functions();

    utils::info_prompt("Analysis IR", &format!("checking ffi functions: {:?}", ffi_funcs));
    analysis_funcs(ffi_funcs, &ir_info)?;

    Ok(())
}


/// Resolve the IR from the bitcode file and collect all modules.
fn resolve_from_bc<'a>(path: PathBuf, targets: Vec<String>, cx: &'a Context) -> Result<IRInfo<'a>, String> {
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

    let ir_info = IRInfo::new(r_modules, c_modules);

    Ok(ir_info)
}

/// Analysis functions.
fn analysis_funcs(ffi_funcs: &Vec<String>, ir_info: &IRInfo) -> Result<(), String> {
    for f in ffi_funcs {
        utils::info_prompt("Analysis IR", &format!("checking function: {}", f));

        let rf = ir_info.r_func(f).expect("Fatal, cannot find the rust function");
        let cf = ir_info.c_func(f).expect("Fatal, cannot find the c/c++ function");

        // println!("rf: {} {} {} {}", rf.count_basic_blocks(), rf.is_null(), rf.is_undef(), rf.verify(false));
        // println!("cf: {} {} {} {}", cf.count_basic_blocks(), cf.is_null(), cf.is_undef(), cf.verify(false));
        // get analysis functions
        let rf = AFunction::from_llvm_raw(rf);
        let cf = AFunction::from_llvm_raw(cf);

        if let Some(err) = _analysis_funcs(&rf, &cf) {
            // TODO

            return Ok(());
        }

        if let Some(err) = _analysis_funcs_params(&rf, &cf) {

            return Ok(());
        }
    }

    Ok(())
}

/// Analysis function usage, that is, params, call convetions, return values, etc.
fn _analysis_funcs(rust_func: &AFunction, c_func: &AFunction) -> Option<AResult> {
    println!("{:?}", rust_func);
    println!("{:?}", c_func);

    // check call convention
    if rust_func.call_convention != c_func.call_convention {
        return Some(AResult::func_convention_issue());
    }
    
    // check params
    if rust_func.params.len() != c_func.params.len() {
        return Some(AResult::func_sig_issue());
    }

    let len = rust_func.params.len();
    for i in 0..len {
        let r_param = &rust_func.params[i];
        let c_param = &c_func.params[i];

        if !AType::shallow_check(&r_param.ty, &c_param.ty) {
            return Some(AResult::func_sig_issue());
        }
    }

    // check return value
    if rust_func.ret.is_some() && c_func.ret.is_some() {
        let r_ret = rust_func.ret.as_ref().unwrap();
        let c_ret = c_func.ret.as_ref().unwrap();

        if !AType::shallow_check(&r_ret.ty, &c_ret.ty) {
            return Some(AResult::func_sig_issue());
        }
    } else if rust_func.ret.is_none() && c_func.ret.is_none() {
        // do nothing
    } else {
        return Some(AResult::func_sig_issue());
    }

    None
}

fn _analysis_funcs_params(rust_func: &AFunction, c_func: &AFunction) -> Option<AResult> {
    let len = rust_func.params.len();

    None
}