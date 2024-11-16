use super::ir_info::IRInfo;
use super::result::{AResult, AResults};
use super::structure::{AStruct, AType, ATypeLazyStruct};

use crate::analysis::result::{AResultLevel, StructMismatch};
use crate::analysis::structure::AField;
use crate::utils;

pub fn analysis_structs<'t, 'ctx: 't>(
    ffi_structs: Vec<(ATypeLazyStruct<'ctx>, ATypeLazyStruct<'ctx>)>,
    ir_info: &'t IRInfo<'ctx>,
    print: bool,
) -> Result<(usize, usize), String> {
    let mut warns = 0;
    let mut errors = 0;
    let target = &ir_info.get_target_data();

    for (r, c) in ffi_structs {
        let (rname, rst) = r.get_lazy();
        let (cname, cst) = c.get_lazy();

        utils::info_prompt(
            "Analysis Structs",
            &format!("checking structs: {} {}", rname, cname),
            print,
        );

        let rst = AStruct::from_llvm_raw(&rst, target);
        let cst = AStruct::from_llvm_raw(&cst, target);

        let res = _analysis_struct(&rst, &cst);
        // println!("Out");
        if !res.is_empty() {
            if print {
                for (res, level) in res.get_iters() {
                    show_error(res, level, &rst, &cst);
                }

                utils::detail_prompt(
                    "Struct Details",
                    &format!(
                        "\nrust struct info: {} ({}, {})\nc/c++ struct info: {} ({}, {})",
                        rst,
                        rst.get_range().0,
                        rst.get_range().1,
                        cst,
                        cst.get_range().0,
                        cst.get_range().1
                    ),
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

        utils::info_prompt(
            "Analysis Structs",
            &format!("structs: {} {} passed", rname, cname),
            print,
        );
    }

    Ok((warns, errors))
}

fn _analysis_struct(rstruct: &AStruct, cstruct: &AStruct) -> AResults {
    // println!("In, check struct: {} {}", rstruct, cstruct);
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
                        (processed_total_size, processed_total_size + rf_size),
                        (
                            processed_total_size,
                            processed_total_size + cf_size + accumulate,
                        ),
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
                        (
                            processed_total_size,
                            processed_total_size + rf_size + accumulate,
                        ),
                        (processed_total_size, processed_total_size + cf_size),
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

    // println!("r_remain: {:#?}\nc_remain: {:#?}", r_remain, c_remain);

    // unmatched fields, should be errors
    if !r_remain.is_empty() || !c_remain.is_empty() {
        let r_end = r_remain
            .last()
            .map(|f| f.get_range().1)
            .unwrap_or(processed_total_size);
        let c_end = c_remain
            .last()
            .map(|f| f.get_range().1)
            .unwrap_or(processed_total_size);

        res.add_struct_issue(
            (processed_total_size, r_end),
            (processed_total_size, c_end),
            StructMismatch::SizeMismatch,
            AResultLevel::Error,
        );
    }

    // type checks
    for (rf, cf) in matches {
        // println!("check match {} {}", rf, cf);
        res.extend(deep_check(&rf, &cf));
    }

    // println!("Out");
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
                    a.get_range(),
                    b.get_range(),
                    StructMismatch::TypeMismatch,
                    AResultLevel::Error,
                );
            }
        }

        (AType::IntType(r), AType::IntType(c)) => {
            if r != c {
                res.add_struct_issue(
                    a.get_range(),
                    b.get_range(),
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
                a.get_range(),
                b.get_range(),
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
        a.get_range(),
        b.get_range(),
        StructMismatch::TypeMismatch,
        AResultLevel::Error,
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
            a.get_range(),
            b.get_range(),
            StructMismatch::TypeMismatch,
            AResultLevel::Error,
        );
    }

    return res;
}

fn array_normal(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();
    res.add_struct_issue(
        a.get_range(),
        b.get_range(),
        StructMismatch::TypeMismatch,
        AResultLevel::Error,
    );

    return res;
}

fn array_array(a: &AField, b: &AField) -> AResults {
    let a_padding_ty = a.can_be_padding().map(|inner| inner.0);
    let b_padding_ty = b.can_be_padding().map(|inner| inner.0);

    let mut res = AResults::new();

    // both can be paddings, so it could be.
    if let (Some(aty), Some(bty)) = (&a_padding_ty,&b_padding_ty) {
        if aty != bty {
            res.add_struct_issue(
                a.get_range(),
                b.get_range(),
                StructMismatch::IsPadding,
                AResultLevel::Warning,
            );
        }

        return res;
    }

    // or only on can be, thus maybe it's opaque
    if (a_padding_ty.is_some() as i8 ^ b_padding_ty.is_some() as i8) == 1 {
        res.add_struct_issue(
            a.get_range(),
            b.get_range(),
            StructMismatch::IsOpaque,
            AResultLevel::Warning,
        );

        return res;
    }

    // or
    return deep_check(a, b);
}

fn array_struct(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();

    let st = b.to_struct().expect("Fatal, should be a struct");

    if a.can_be_padding().is_some() {
        res.add_struct_issue(
            a.get_range(),
            b.get_range(),
            StructMismatch::IsOpaque,
            AResultLevel::Warning,
        );

        return res;
    }

    if let Some(b) = st.get_inner_one() {
        return deep_check(a, b);
    } else {
        res.add_struct_issue(
            a.get_range(),
            b.get_range(),
            StructMismatch::TypeMismatch,
            AResultLevel::Error,
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
            a.get_range(),
            b.get_range(),
            StructMismatch::TypeMismatch,
            AResultLevel::Error,
        );
    }

    return res;
}

fn struct_array(a: &AField, b: &AField) -> AResults {
    let mut res = AResults::new();

    let st = a.to_struct().expect("Fatal, should be a struct");

    if b.can_be_padding().is_some() {
        res.add_struct_issue(
            a.get_range(),
            b.get_range(),
            StructMismatch::IsOpaque,
            AResultLevel::Warning,
        );

        return res;
    }

    if let Some(a) = st.get_inner_one() {
        return deep_check(a, b);
    } else {
        res.add_struct_issue(
            a.get_range(),
            b.get_range(),
            StructMismatch::TypeMismatch,
            AResultLevel::Error,
        );
    }

    return res;
}

fn struct_struct(a: &AField, b: &AField) -> AResults {
    let a = a.to_struct().expect("Fatal, should be a struct");
    let b = b.to_struct().expect("Fatal, should be a struct");

    return _analysis_struct(a, b);
}

fn show_error(res: &AResult, level: &AResultLevel, _rst: &AStruct, _cst: &AStruct) {
    match res {
        AResult::StructIssue(r_range, c_range, mis) => {
            let details = match mis {
                StructMismatch::SizeMismatch => "size mismatch",
                StructMismatch::TypeMismatch => "type mismatch",
                StructMismatch::IsPadding => "possibly padding",
                StructMismatch::IsOpaque => "possibly opaque",
            };

            match level {
                AResultLevel::Error => utils::error_prompt("Issue Found", details, true),
                AResultLevel::Warning => utils::warn_prompt("Issue Found", details, true),
            }

            utils::print(&format!("rust side: {:?}", r_range), true);
            utils::print(&format!("c/c++ side: {:?}", c_range), true);
        }
        _ => unreachable!(),
    }
}
