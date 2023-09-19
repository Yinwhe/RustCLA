use crate::analysis::function::AFunction;
use crate::analysis::structure::AType;
use crate::utils;

use super::ir_info::IRInfo;
use super::result::{AResult, AResults, SigMismatch};

/// Analysis functions.
pub fn analysis_funcs(ffi_funcs: Vec<String>, ir_info: &mut IRInfo) -> Result<(), String> {
    let target = &ir_info.get_target_data();
    for f in ffi_funcs {
        utils::info_prompt("Analysis Funcs", &format!("checking function: {}", f));

        let rf = ir_info
            .r_func(&f)
            .expect("Fatal, cannot find the rust function");

        let cf = ir_info
            .c_func(&f)
            .expect("Fatal, cannot find the c/c++ function");

        // get analysis functions
        let rf = AFunction::from_llvm_raw(rf, target);
        let cf = AFunction::from_llvm_raw(cf, target);

        let res = _analysis_funcs(&rf, &cf);
        if !res.is_empty() {
            for (res, _) in res.get_iters() {
                show_error(res, &rf, &cf);
            }
            continue;
        }

        // get ffi structs
        let ffis = fetch_ffi_structs(&rf, &cf);
        ir_info.add_ffi_structs(ffis);

        utils::info_prompt("Analysis Funcs", &format!("function {} passed", f));
    }

    Ok(())
}

/// Analysis function usage, that is, params, call convetions, return values, etc.
fn _analysis_funcs(rust_func: &AFunction, c_func: &AFunction) -> AResults {
    let mut res = AResults::new();
    // check call convention
    if rust_func.call_convention != c_func.call_convention {
        res.add_func_convention_issue(rust_func.call_convention, c_func.call_convention);
    }

    // check params
    if rust_func.params.len() != c_func.params.len() {
        res.add_func_sig_issue(SigMismatch::ParamLen);
        return res;
    }

    let len = rust_func.params.len();
    for i in 0..len {
        let r_param = &rust_func.params[i];
        let c_param = &c_func.params[i];

        if !shallow_check(&r_param, &c_param) {
            res.add_func_sig_issue(SigMismatch::ParamType(i as u32));
        }
    }

    // check return value
    if let (Some(r_ret), Some(c_ret)) = (&rust_func.ret, &c_func.ret) {
        if !shallow_check(&r_ret, &c_ret) {
            res.add_func_sig_issue(SigMismatch::RetType);
        }
    } else if rust_func.ret.is_none() && c_func.ret.is_none() {
        // do nothing
    } else {
        res.add_func_sig_issue(SigMismatch::RetType);
    }

    res
}

/// Check type, but will ends on defined structs or minist types.
/// Complex structs will be take as ffi structs and analysis later.
fn shallow_check(r: &AType, c: &AType) -> bool {
    match (r, c) {
        (AType::ArrayType(rty, rlen), AType::ArrayType(cty, clen)) => {
            if rlen != clen {
                return false;
            }

            return shallow_check(rty, cty);
        }
        (AType::FloatType(rstr), AType::FloatType(cstr)) => rstr == cstr,
        (AType::IntType(rstr), AType::IntType(cstr)) => rstr == cstr,
        (AType::PointerType(rptr), AType::PointerType(cptr)) => {
            return shallow_check(rptr, cptr);
        }
        (AType::StructType(_), AType::StructType(_)) => true,
        (AType::VectorType(rty), AType::VectorType(cty)) => {
            return shallow_check(rty, cty);
        }
        _ => false,
    }
}

fn fetch_ffi_structs(rust_func: &AFunction, c_func: &AFunction) -> Vec<(String, String)> {
    let mut ffis = Vec::new();
    let len = rust_func.params.len();

    for i in 0..len {
        let r_param = &rust_func.params[i];
        let c_param = &c_func.params[i];

        if let Some(ffi) = _fetch_ffi_structs(r_param, c_param) {
            ffis.push(ffi);
        }
    }

    ffis
}

fn _fetch_ffi_structs(r: &AType, c: &AType) -> Option<(String, String)> {
    match (r, c) {
        (AType::ArrayType(rty, _), AType::ArrayType(cty, _)) => {
            return _fetch_ffi_structs(&rty, &cty);
        }
        (AType::PointerType(rty), AType::PointerType(cty)) => {
            return _fetch_ffi_structs(&rty, &cty);
        }
        (AType::StructType(rst), AType::StructType(cst)) => {
            return Some((rst.get_name(), cst.get_name()));
        }
        (AType::VectorType(rty), AType::VectorType(cty)) => {
            return _fetch_ffi_structs(&rty, &cty);
        }
        _ => {
            // nothing todo
        }
    }

    None
}

fn show_error(res: &AResult, rf: &AFunction, cf: &AFunction) {
    match res {
        AResult::ConventionIssue(r, c) => {
            utils::error_prompt("Issue Found", "call convention mismatch");
            println!("rust side: {}, c/c++ side: {}", r, c);
        }
        AResult::SigIssue(sig) => match sig {
            SigMismatch::ParamLen => {
                utils::error_prompt("Issue Found", "param length mismatch");
                println!(
                    "rust side: {}, c/c++ side: {}",
                    rf.params.len(),
                    cf.params.len()
                );
            }
            SigMismatch::ParamType(i) => {
                utils::error_prompt("Issue Found", "param type mismatch");
                println!(
                    "rust side: {:?}, c/c++ side: {:?}",
                    rf.params[*i as usize], cf.params[*i as usize]
                );
            }
            SigMismatch::RetType => {
                utils::error_prompt("Issue Found", "return type mismatch");
                println!("rust side: {:?}, c/c++ side: {:?}", rf.ret, cf.ret);
            }
        },
        _ => unreachable!(),
    }
}
