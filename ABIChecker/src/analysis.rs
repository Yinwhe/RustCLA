use crate::analysis_types::*;

/// Three parts: align, size, type
pub fn info_struct_analysis(
    rstruct: AnalysisStruct,
    cstruct: AnalysisStruct,
) -> AnalysisStructResult {
    println!("Debug:\nrstruct: {:?}\n cstruct: {:?}\n", rstruct, cstruct);
    enum AnalysisStatus {
        Match,
        RustRemain,
        CppRemain,
    }

    let mut status = AnalysisStatus::Match;
    let mut result = AnalysisStructResult::new(rstruct.clone(), cstruct.clone());

    // Check struct align first
    if rstruct.get_alignment() != cstruct.get_alignment() {
        result.add_info(AnalysisResultContent::warn(
            (0, 0),
            (0, 0),
            AnalysisResultType::AlignmentMismatch,
            format!("Alignment mismatch"),
        ));
    }

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

        match status {
            AnalysisStatus::Match => {
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
                    status = AnalysisStatus::CppRemain;
                } else {
                    // c fields smaller, rust fields wonn't iter
                    accumulate += cf_size;
                    c_match.push(cf);

                    // renew
                    c_index += 1;
                    status = AnalysisStatus::RustRemain;
                }
            }
            AnalysisStatus::RustRemain => {
                if rf_size == cf_size + accumulate {
                    // matches
                    processed_total_size += rf_size;

                    // store matches
                    let temp_st = AnalysisStruct::temp(c_match);

                    let temp_f = AnalysisField {
                        name: None,
                        is_padding: false,
                        range: temp_st.get_range(),
                        ty: AnalysisFieldType::StructType(temp_st),
                        temp: true,
                    };

                    matches.push((rf, temp_f));

                    // renew
                    c_match = Vec::new();
                    r_index += 1;
                    c_index += 1;

                    status = AnalysisStatus::Match;
                } else if rf_size > cf_size + accumulate {
                    // rust still remains
                    accumulate += cf_size;
                    c_match.push(cf);

                    // renew
                    c_index += 1;
                } else {
                    // mismatch occurs
                    result.add_info(AnalysisResultContent::error(
                        (processed_total_size, rf.range.1),
                        (processed_total_size, cf.range.1),
                        AnalysisResultType::SizeMismatch,
                        format!("Mismatch occurs"),
                    ));
                    return result;
                }
            }
            AnalysisStatus::CppRemain => {
                if rf_size + accumulate == cf_size {
                    // matches
                    processed_total_size += cf_size;

                    // store matches
                    let temp_st = AnalysisStruct::temp(r_match);

                    let temp_f = AnalysisField {
                        name: None,
                        is_padding: false,
                        range: temp_st.get_range(),
                        ty: AnalysisFieldType::StructType(temp_st),
                        temp: true,
                    };

                    matches.push((rf, temp_f));

                    // renew
                    r_match = Vec::new();
                    r_index += 1;
                    c_index += 1;

                    status = AnalysisStatus::Match;
                } else if rf_size + accumulate < cf_size {
                    // c still remains
                    accumulate += rf_size;
                    r_match.push(rf);

                    // renew
                    r_index += 1;
                } else {
                    // mismatch occurs
                    result.add_info(AnalysisResultContent::error(
                        (processed_total_size, rf.range.1),
                        (processed_total_size, cf.range.1),
                        AnalysisResultType::SizeMismatch,
                        format!("Mismatch occurs"),
                    ));
                    return result;
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

    debug!("r_remain: {:?}\nc_remain: {:#?}", r_remain, c_remain);

    // unmatched fields, should be errors
    if !r_remain.is_empty() || !c_remain.is_empty() {
        result.add_info(AnalysisResultContent::error(
            (processed_total_size, 0),
            (processed_total_size, 0),
            AnalysisResultType::SizeMismatch,
            format!("Unmatched fields found"),
        ));
        return result;
    }

    // Now we can do type checks
    while let Some((rf, cf)) = matches.pop() {
        if rf.is_normal() {
            if cf.is_normal() {
                // normal to normal
                if rf.get_type_id() == cf.get_type_id() {
                    // ok match
                    continue;
                } else {
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Normal type mismatch"),
                    ));
                    return result;
                }
            } else if cf.is_struct() {
                // normal to struct
                let st = cf.get_struct().expect("Should not happen");
                if st.fields.len() != 1 {
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Normal & Struct type mismatch"),
                    ));
                    return result;
                }

                if st.fields.first().expect("Should not happen").get_type_id() != rf.get_type_id() {
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Normal & Struct type mismatch"),
                    ));
                    return result;
                }
                // or ok
                continue;
            } else {
                // normal to array
                if cf.is_padding {
                    // error
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Normal & Array type mismatch"),
                    ));
                    return result;
                } else {
                    // warn, is opaque ?
                    result.add_info(AnalysisResultContent::warn(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Normal & Array type mismatch, is itended opaque"),
                    ));
                }
            }
        } else if rf.is_struct() {
            if cf.is_normal() {
                // struct to normal
                let st = rf.get_struct().expect("Should not happen");
                if st.fields.len() != 1 {
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Struct & Normal type mismatch"),
                    ));
                    return result;
                }

                if st.fields.first().expect("Should not happen").get_type_id() != cf.get_type_id() {
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Struct & Normal type mismatch"),
                    ));
                    return result;
                }
                // or ok
                continue;
            } else if cf.is_struct() {
                // struct to struct
                let rst = rf.get_struct().expect("Should not happen").clone();
                let cst = cf.get_struct().expect("Should not happen").clone();

                let sub_res = info_struct_analysis(rst, cst);
                result.merge(&sub_res);

                if sub_res.is_error() {
                    return result;
                }
            } else {
                // struct to array
                if cf.is_padding {
                    // error
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Struct & Array type mismatch"),
                    ));
                    return result;
                } else {
                    // warn, is opaque ?
                    result.add_info(AnalysisResultContent::warn(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Struct & Array type mismatch, is itended opaque"),
                    ));
                }
            }
        } else {
            if cf.is_normal() {
                // array to normal
                if rf.is_padding {
                    // error
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Array & Normal type mismatch"),
                    ));
                    return result;
                } else {
                    // warn, is opaque ?
                    result.add_info(AnalysisResultContent::warn(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Array & Normal type mismatch, is itended opaque"),
                    ));
                }
            } else if cf.is_struct() {
                // array to struct
                if rf.is_padding {
                    // error
                    result.add_info(AnalysisResultContent::error(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Array & Struct type mismatch"),
                    ));
                    return result;
                } else {
                    // warn, is opaque ?
                    result.add_info(AnalysisResultContent::warn(
                        rf.range,
                        cf.range,
                        AnalysisResultType::TypeMismatch,
                        format!("Array & Struct type mismatch, is itended opaque"),
                    ));
                }
            } else {
                // array to array
                if rf.is_padding && cf.is_padding {
                    // ok
                    continue;
                }

                if !rf.is_padding && cf.is_padding {
                    // ok
                    continue;
                }

                // or, warn
                result.add_info(AnalysisResultContent::warn(
                    rf.range,
                    cf.range,
                    AnalysisResultType::TypeMismatch,
                    format!("Array & Array type mismatch, is itended opaque"),
                ));
            }
        }
    }

    return result;
}

pub fn function_analysis(
    rfunc: AnalysisFunction,
    cfunc: AnalysisFunction,
) -> AnalsisFunctionResult {
    let mut result = AnalsisFunctionResult::new(rfunc.clone(), cfunc.clone());

    // check arg len
    if rfunc.params.len() != cfunc.params.len() {
        result.add_info(AnalysisResultContent::error(
            (0, 0),
            (0, 0),
            AnalysisResultType::ArgsLengthMismatch,
            format!("Args len mismatch"),
        ));
        return result;
    }

    // check arg passby
    for i in 0..rfunc.params.len() {
        let rp = &rfunc.params[i];
        let cp = &cfunc.params[i];
        if rp.pass_by != cp.pass_by {
            result.add_info(AnalysisResultContent::error(
                (i as u32, 0),
                (i as u32, 0),
                AnalysisResultType::ArgsPassByMismatch,
                format!("Args passby mismatch"),
            ));
        }
    }

    // check ret type
    if rfunc.ret.is_none() && cfunc.ret.is_none() {
        return result;
    } else if rfunc.ret.is_some() && cfunc.ret.is_some() {
        let rr = rfunc.ret.unwrap();
        let cr = cfunc.ret.unwrap();

        if rr.pass_by != cr.pass_by {
            result.add_info(AnalysisResultContent::error(
                (0, 0),
                (0, 0),
                AnalysisResultType::RetPassByMismatch,
                format!("Ret passby mismatch"),
            ));
        }

        return result;
    } else {
        result.add_info(AnalysisResultContent::error(
            (0, 0),
            (0, 0),
            AnalysisResultType::RetMismatch,
            format!("Ret type mismatch"),
        ));

        return result;
    }
}
