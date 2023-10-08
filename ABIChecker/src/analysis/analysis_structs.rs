use super::ir_info::IRInfo;
use super::result::{AResult, AResults};
use super::structure::{AStruct, AType, ATypeLazyStruct};

use crate::analysis::result::{AResultLevel, StructMismatch};
use crate::analysis::structure::AField;
use crate::utils;

pub fn analysis_structs<'t, 'ctx: 't>(
    ffi_structs: Vec<(ATypeLazyStruct<'ctx>, ATypeLazyStruct<'ctx>)>,
    ir_info: &'t IRInfo<'ctx>,
) -> Result<(), String> {
    let target = &ir_info.get_target_data();
    for (r, c) in ffi_structs {
        let (rname, rst) = r.get_lazy();
        let (cname, cst) = c.get_lazy();

        utils::info_prompt(
            "Analysis Structs",
            &format!("checking structs: {} {}", rname, cname),
        );

        let rst = AStruct::from_llvm_raw(&rst, target);
        let cst = AStruct::from_llvm_raw(&cst, target);

        let res = _analysis_struct(&rst, &cst);
        if !res.is_empty() {
            for (res, _) in res.get_iters() {
                show_error(res, &rst, &cst);
            }

            utils::warn_prompt(
                "Struct Details",
                &format!("\nrust struct info: {}\nc/c++ struct info: {}", rst, cst),
            );
            continue;
        }

        utils::info_prompt(
            "Analysis Structs",
            &format!("structs: {} {} passed", rname, cname),
        );
    }

    Ok(())
}

fn _analysis_struct(rstruct: &AStruct, cstruct: &AStruct) -> AResults {
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

    let mut res = AResults::new();

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
                    res.add_struct_issue(
                        processed_total_size,
                        processed_total_size,
                        StructMismatch::SizeMismatch,
                        AResultLevel::Error,
                    );
                    return res;
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
                    res.add_struct_issue(
                        processed_total_size,
                        processed_total_size,
                        StructMismatch::SizeMismatch,
                        AResultLevel::Error,
                    );
                    return res;
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
        res.add_struct_issue(
            processed_total_size,
            processed_total_size,
            StructMismatch::SizeMismatch,
            AResultLevel::Warning,
        );
        // result.add_info(AnalysisResultContent::error(
        //     (processed_total_size, 0),
        //     (processed_total_size, 0),
        //     AnalysisResultType::SizeMismatch,
        //     format!("Unmatched fields found, can be errors"),
        // ));
        // // but no return
    }

    // type checks
    for (rf, cf) in matches {
        res.extend(deep_check(&rf, &cf));
    }

    return res;
}

fn deep_check(a: &AField, b: &AField) -> AResults {
    // 1 - normal compare
    // 2 - array compare
    // 3 - struct compare
    match (a.cmp_number(), b.cmp_number()) {
        (1, 1) => normal_normal(a, b),
        (1, 2) => normal_array(a, b),
        (1, 3) => normal_struct(a, b),
        (2, 1) => array_normal(a, b),
        (2, 2) => array_array(a, b),
        (2, 3) => array_struct(a, b),
        (3, 1) => struct_normal(a, b),
        (3, 2) => struct_array(a, b),
        (3, 3) => struct_struct(a, b),
        _ => unreachable!(),
    }
}

fn normal_normal(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();
    match (a.get_type(), b.get_type()) {
        (AType::FloatType(r), AType::FloatType(c)) => {
            if r != c {
                res.add_struct_issue(
                    a.get_start(),
                    b.get_start(),
                    StructMismatch::TypeMismatch,
                    AResultLevel::Error,
                );
            }
        }

        (AType::IntType(r), AType::IntType(c)) => {
            if r != c {
                res.add_struct_issue(
                    a.get_start(),
                    b.get_start(),
                    StructMismatch::TypeMismatch,
                    AResultLevel::Error,
                );
            }
        }
        (AType::PointerType(r), AType::PointerType(c)) => {
            let r = AField::temp_field(*r, a.get_range());
            let c = AField::temp_field(*c, b.get_range());
            return deep_check(&r, &c);
        }
        (AType::VectorType(r), AType::VectorType(c)) => {
            let r = AField::temp_field(*r, a.get_range());
            let c = AField::temp_field(*c, b.get_range());
            return deep_check(&r, &c);
        }
        _ => {
            res.add_struct_issue(
                a.get_start(),
                b.get_start(),
                StructMismatch::TypeMismatch,
                AResultLevel::Error,
            );
        }
    }

    return res;
}

fn normal_array(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();
    res.add_struct_issue(
        a.get_start(),
        b.get_start(),
        StructMismatch::TypeMismatch,
        AResultLevel::Warning,
    );

    return res;
}

fn normal_struct(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();

    let st = b.to_struct().expect("Fatal, should be a struct");

    if let Some(b) = st.get_inner_one() {
        return deep_check(a, b);
    } else {
        res.add_struct_issue(
            a.get_start(),
            b.get_start(),
            StructMismatch::TypeMismatch,
            AResultLevel::Error,
        );
    }

    return res;
}

fn array_normal(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();
    res.add_struct_issue(
        a.get_start(),
        b.get_start(),
        StructMismatch::TypeMismatch,
        AResultLevel::Warning,
    );

    return res;
}

fn array_array(a: &AField, b: &AField) -> AResults {
    match (a.get_type(), b.get_type()) {
        (AType::ArrayType(r, _), AType::ArrayType(c, _)) => {
            let r = AField::temp_field(*r, a.get_range());
            let c = AField::temp_field(*c, b.get_range());
            return deep_check(&r, &c);
        }
        _ => unreachable!(),
    }
}

fn array_struct(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();

    let st = b.to_struct().expect("Fatal, should be a struct");

    if let Some(b) = st.get_inner_one() {
        return deep_check(a, b);
    } else {
        res.add_struct_issue(
            a.get_start(),
            b.get_start(),
            StructMismatch::TypeMismatch,
            AResultLevel::Warning,
        );
    }

    return res;
}

fn struct_normal(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();

    let st = a.to_struct().expect("Fatal, should be a struct");

    if let Some(a) = st.get_inner_one() {
        return deep_check(a, b);
    } else {
        res.add_struct_issue(
            a.get_start(),
            b.get_start(),
            StructMismatch::TypeMismatch,
            AResultLevel::Error,
        );
    }

    return res;
}

fn struct_array(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();

    let st = a.to_struct().expect("Fatal, should be a struct");

    if let Some(a) = st.get_inner_one() {
        return deep_check(a, b);
    } else {
        res.add_struct_issue(
            a.get_start(),
            b.get_start(),
            StructMismatch::TypeMismatch,
            AResultLevel::Warning,
        );
    }

    return res;
}

fn struct_struct(a: &AField, b: &AField) -> AResults {
    let a = a.to_struct().expect("Fatal, should be a struct");
    let b = b.to_struct().expect("Fatal, should be a struct");

    return _analysis_struct(a, b);
}

fn show_error(res: &AResult, rst: &AStruct, cst: &AStruct) {
    match res {
        AResult::StructIssue(r_off, c_off, mis) => {
            let rf = rst.get_fields_from_offset(*r_off);
            let cf = cst.get_fields_from_offset(*c_off);
            let details = match mis {
                StructMismatch::SizeMismatch => "size mismatch",
                StructMismatch::TypeMismatch => "type mismatch",
            };

            utils::error_prompt("Issue Found", details);
            if let Some(rf) = rf {
                println!("rust side: {}", rf);
            } else {
                println!("rust side: no field found");
            }

            if let Some(cf) = cf {
                println!("c/c++ side: {}", cf);
            } else {
                println!("c/c++ side: no field found");
            }
        }
        _ => unreachable!(),
    }
}
