use crate::analysis::function::AFunction;
use crate::analysis::structure::AType;
use crate::utils;

use super::ir_info::IRInfo;
use super::result::{AResult, AResultLevel, AResults, SigMismatch};
use super::structure::ATypeLazyStruct;

/// Analysis functions.
pub fn analysis_funcs<'t, 'ctx: 't>(
    ffi_funcs: Vec<String>,
    ir_info: &'t mut IRInfo<'ctx>,
    print: bool,
) -> Result<(usize, usize), String> {
    let mut warns = 0;
    let mut errors = 0;
    let target = &ir_info.get_target_data();

    // let mut vec = Vec::new();
    for f in ffi_funcs {
        utils::info_prompt(
            "Analysis Funcs",
            &format!("checking function: {}", f),
            print,
        );

        let rf = ir_info
            .get_r_func(&f)
            .expect("Fatal, cannot find the rust function");

        let cf = ir_info
            .get_c_func(&f)
            .expect("Fatal, cannot find the c/c++ function");

        // get analysis functions
        let rf = AFunction::from_llvm_raw(&rf, target);
        let cf = AFunction::from_llvm_raw(&cf, target);

        let res = _analysis_funcs(&rf, &cf);
        if !res.is_empty() {
            if print {
                for (res, level) in res.get_iters() {
                    show_error(res, level, &rf, &cf);
                }

                utils::detail_prompt(
                    "Func Details",
                    &format!("rust func info: {:?}\nc/c++ func info: {:?}", rf, cf),
                    print,
                );
            }

            res.get_iters().for_each(|(_, level)| {
                if level.is_error() {
                    errors += 1;
                } else {
                    warns += 1;
                }
            });

            continue;
        }

        // get ffi structs
        let ffis = fetch_ffi_structs(rf, cf);
        ir_info.add_ffi_structs(ffis);

        utils::info_prompt("Analysis Funcs", &format!("function {} passed", f), print);
    }

    Ok((warns, errors))
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

fn fetch_ffi_structs<'ctx>(
    rust_func: AFunction<'ctx>,
    c_func: AFunction<'ctx>,
) -> Vec<(ATypeLazyStruct<'ctx>, ATypeLazyStruct<'ctx>)> {
    let mut ffis = Vec::new();

    for (rty, cty) in rust_func.params.into_iter().zip(c_func.params.into_iter()) {
        if let Some(ffi) = _fetch_ffi_structs(rty, cty) {
            ffis.push(ffi);
        }
    }

    ffis
}

fn _fetch_ffi_structs<'ctx>(
    r: AType<'ctx>,
    c: AType<'ctx>,
) -> Option<(ATypeLazyStruct<'ctx>, ATypeLazyStruct<'ctx>)> {
    match (r, c) {
        (AType::ArrayType(rty, _), AType::ArrayType(cty, _)) => {
            return _fetch_ffi_structs(*rty, *cty);
        }
        (AType::PointerType(rty), AType::PointerType(cty)) => {
            return _fetch_ffi_structs(*rty, *cty);
        }
        (AType::StructType(rst), AType::StructType(cst)) => {
            return Some((rst, cst));
        }
        (AType::VectorType(rty), AType::VectorType(cty)) => {
            return _fetch_ffi_structs(*rty, *cty);
        }
        _ => {
            // nothing todo
        }
    }

    None
}

fn show_error(res: &AResult, _level: &AResultLevel, rf: &AFunction, cf: &AFunction) {
    match res {
        AResult::ConventionIssue(r, c) => {
            utils::error_prompt("Issue Found", "call convention mismatch", true);
            utils::print(&format!("rust side: {}, c/c++ side: {}", r, c), true);
        }
        AResult::SigIssue(sig) => match sig {
            SigMismatch::ParamLen => {
                utils::error_prompt("Issue Found", "param length mismatch", true);
                utils::print(
                    &format!(
                        "rust side: {}, c/c++ side: {}",
                        rf.params.len(),
                        cf.params.len()
                    ),
                    true,
                );
            }
            SigMismatch::ParamType(i) => {
                utils::error_prompt("Issue Found", "param type mismatch", true);
                utils::print(
                    &format!(
                        "rust side: {:?}, c/c++ side: {:?}",
                        rf.params[*i as usize], cf.params[*i as usize]
                    ),
                    true,
                );
            }
            SigMismatch::RetType => {
                utils::error_prompt("Issue Found", "return type mismatch", true);
                utils::print(
                    &format!("rust side: {:?}, c/c++ side: {:?}", rf.ret, cf.ret),
                    true,
                );
            }
        },
        _ => unreachable!(),
    }
}
