use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;

use crate::analysis::function::AFunction;
use crate::analysis::structure::{AField, AType};
use crate::utils;

use super::ir_info::IRInfo;
use super::result::{AResult, ParamMismatch, SigMismatch};
use super::structure::AStruct;

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
    _analysis_ir(ffi_funcs, &ir_info)?;

    Ok(())
}

/// Resolve the IR from the bitcode file and collect all modules.
fn resolve_from_bc<'a>(
    path: PathBuf,
    targets: Vec<String>,
    cx: &'a Context,
    target_machine: TargetMachine,
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

    let ir_info = IRInfo::new(r_modules, c_modules, target_machine);

    Ok(ir_info)
}

/// Analysis functions.
fn _analysis_ir(ffi_funcs: &Vec<String>, ir_info: &IRInfo) -> Result<(), String> {
    let target = &ir_info.get_target_data();
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
        
        utils::info_prompt("Analysis IR", "sig check done, checking params details...");
        if let Some((id, err)) = _analysis_funcs_params(&rf, &cf) {
            utils::error_prompt("Analysis IR", &format!("{}", err));
            println!("{:?}", rf.params[id as usize]);
            println!("{:?}", cf.params[id as usize]);

            continue;
        }
    }

    Ok(())
}

/// Analysis function usage, that is, params, call convetions, return values, etc.
fn _analysis_funcs(rust_func: &AFunction, c_func: &AFunction) -> Option<AResult> {
    debug!("{:?}", rust_func);
    debug!("{:?}", c_func);

    // check call convention
    if rust_func.call_convention != c_func.call_convention {
        return Some(AResult::func_convention_issue(
            rust_func.call_convention,
            c_func.call_convention,
        ));
    }

    // check params
    if rust_func.params.len() != c_func.params.len() {
        return Some(AResult::func_sig_issue(SigMismatch::ParamLen));
    }

    let len = rust_func.params.len();
    for i in 0..len {
        let r_param = &rust_func.params[i];
        let c_param = &c_func.params[i];

        if !shallow_check(&r_param, &c_param) {
            return Some(AResult::func_sig_issue(SigMismatch::ParamType(i as u32)));
        }
    }

    // check return value
    if let (Some(r_ret), Some(c_ret)) = (&rust_func.ret, &c_func.ret) {
        if !shallow_check(&r_ret, &c_ret) {
            return Some(AResult::func_sig_issue(SigMismatch::RetType));
        }
    } else if rust_func.ret.is_none() && c_func.ret.is_none() {
        // do nothing
    } else {
        return Some(AResult::func_sig_issue(SigMismatch::RetType));
    }

    None
}

fn _analysis_funcs_params(rust_func: &AFunction, c_func: &AFunction) -> Option<(u32, AResult)> {
    let len = rust_func.params.len();

    for i in 0..len {
        let r_param = &rust_func.params[i];
        let c_param = &c_func.params[i];

        // TODO
        deep_check(r_param, c_param);
    }

    None
}

fn shallow_check(a: &AType, b: &AType) -> bool {
    match (a, b) {
        (AType::ArrayType(_, _), AType::ArrayType(_, _)) => true,
        (AType::FloatType(_), AType::FloatType(_)) => true,
        (AType::IntType(_), AType::IntType(_)) => true,
        (AType::PointerType(_), AType::PointerType(_)) => true,
        (AType::StructType(_), AType::StructType(_)) => true,
        (AType::VectorType(_), AType::VectorType(_)) => true,
        _ => false,
    }
}

fn deep_check(a: &AType, b: &AType) -> Option<AResult> {
    match (a, b) {
        (AType::ArrayType(a, alen), AType::ArrayType(b, blen)) => {
            if alen != blen {
                return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
            }

            return deep_check(a, b);
        }
        (AType::FloatType(astr), AType::FloatType(bstr)) => {
            if astr != bstr {
                return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
            } else {
                None
            }
        }
        (AType::IntType(astr), AType::IntType(bstr)) => {
            if astr != bstr {
                return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
            } else {
                None
            }
        }
        (AType::PointerType(a), AType::PointerType(b)) => {
            return deep_check(a, b);
        }
        (AType::StructType(a), AType::StructType(b)) => _analysis_struct(a, b),
        (AType::VectorType(a), AType::VectorType(b)) => {
            return deep_check(a, b);
        }
        _ => Some(AResult::ParamIssue(0, ParamMismatch::TypeMismatch)),
    }
}

fn _analysis_struct(rstruct: &AStruct, cstruct: &AStruct) -> Option<AResult> {
    enum AStatus {
        Match,
        RustRemain,
        CppRemain,
    }

    let mut status = AStatus::Match;

    let mut matches = Vec::new();

    let mut r_index = 0;
    let mut c_index = 0;

    let mut r_match = Vec::new();
    let mut c_match = Vec::new();

    let mut processed_total_size = 0;
    let mut accumulate = 0;

    // Check size
    while let (Some(rf), Some(cf)) = (rstruct.get_field(r_index), cstruct.get_field(c_index)) {
        let (rf_size, cf_size) = (rf.get_size(), cf.get_size());
        debug!("check size: {:?}, {:?}", rf, cf);

        match status {
            AStatus::Match => {
                // matches
                if rf_size == cf_size {
                    processed_total_size += rf_size;

                    // store matches
                    matches.push((rf, cf));

                    // renew
                    r_index += 1;
                    c_index += 1;
                } else if rf_size < cf_size {
                    // rust fields smaller, c fields wonn't iter
                    accumulate += rf_size;
                    r_match.push(rf);

                    // renew
                    r_index += 1;
                    status = AStatus::CppRemain;
                } else {
                    // c fields smaller, rust fields wonn't iter
                    accumulate += cf_size;
                    c_match.push(cf);

                    // renew
                    c_index += 1;
                    status = AStatus::RustRemain;
                }
            }
            AStatus::RustRemain => {
                if rf_size == cf_size + accumulate {
                    // matches
                    processed_total_size += rf_size;
                    c_match.push(cf);

                    // store matches
                    let temp_f = AField::temp_st(c_match);

                    matches.push((rf, temp_f));

                    // renew
                    c_match = Vec::new();
                    r_index += 1;
                    c_index += 1;
                    accumulate = 0;

                    status = AStatus::Match;
                } else if rf_size > cf_size + accumulate {
                    // rust still remains
                    accumulate += cf_size;
                    c_match.push(cf);

                    // renew
                    c_index += 1;
                } else {
                    // mismatch occurs
                    return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
                    // result.add_info(AnalysisResultContent::error(
                    //     rf.range,
                    //     cf.range,
                    //     AnalysisResultType::SizeMismatch,
                    //     format!("Size mismatch, rust field end in the mid of c++ field"),
                    // ));
                    // return result;
                }
            }
            AStatus::CppRemain => {
                if rf_size + accumulate == cf_size {
                    // matches
                    processed_total_size += cf_size;
                    r_match.push(rf);

                    // store matches
                    let temp_f = AField::temp_st(r_match);

                    matches.push((cf, temp_f));

                    // renew
                    r_match = Vec::new();
                    r_index += 1;
                    c_index += 1;
                    accumulate = 0;

                    status = AStatus::Match;
                } else if rf_size + accumulate < cf_size {
                    // c still remains
                    accumulate += rf_size;
                    r_match.push(rf);

                    // renew
                    r_index += 1;
                } else {
                    // mismatch occurs
                    return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
                    // result.add_info(AnalysisResultContent::error(
                    //     rf.range,
                    //     cf.range,
                    //     AnalysisResultType::SizeMismatch,
                    //     format!("Size mismatch, c++ field end in the mid of rust field"),
                    // ));
                    // return result;
                }
            }
        } // match ends
    } // while ends

    // Check remain rust fields
    while let Some(rf) = rstruct.get_field(r_index) {
        r_match.push(rf);
        r_index += 1;
    }

    // Check remain c fields
    while let Some(cf) = cstruct.get_field(c_index) {
        c_match.push(cf);
        c_index += 1;
    }

    // we have remaining unmatched fields
    let r_remain = r_match;
    let c_remain = c_match;

    debug!(
        "processed_size: {}\nmatches: {:#?}",
        processed_total_size, matches
    );

    debug!("r_remain: {:#?}\nc_remain: {:#?}", r_remain, c_remain);

    // unmatched fields, should be errors
    if !r_remain.is_empty() || !c_remain.is_empty() {
        return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
        // result.add_info(AnalysisResultContent::error(
        //     (processed_total_size, 0),
        //     (processed_total_size, 0),
        //     AnalysisResultType::SizeMismatch,
        //     format!("Unmatched fields found, can be errors"),
        // ));
        // // but no return
    }
    
    None
}
