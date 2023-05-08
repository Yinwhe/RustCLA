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
            0,
            0,
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
                    r_match.push(rf);
                    c_match.push(cf);
                    matches.push((r_match, c_match));

                    // renew
                    r_match = Vec::new();
                    c_match = Vec::new();
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
                    c_match.push(cf);
                    r_match.push(rf);
                    matches.push((r_match, c_match));

                    // renew
                    r_match = Vec::new();
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
                        r_index,
                        c_index,
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
                    c_match.push(cf);
                    r_match.push(rf);
                    matches.push((r_match, c_match));

                    // renew
                    r_match = Vec::new();
                    c_match = Vec::new();
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
                        r_index,
                        c_index,
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

    println!(
        "Debug\nprocessed_size: {}\nmatches: {:#?}",
        processed_total_size, matches
    );
    println!(
        "Debug:\nr_remain: {:?}\nc_remain: {:#?}",
        r_remain, c_remain
    );

    // TODO: What shall we do about unmatched fields?

    // Now we can do type checks
    while let Some((mut rm, mut cm)) = matches.pop() {
        assert!(rm.len() >= 1 && cm.len() >= 1);
        if rm.len() == 1 && cm.len() == 1 {
        } else if rm.len() == 1 {
        } else {
        }
    }

    unimplemented!()
}

pub fn function_analysis(
    rfunc: AnalysisFunction,
    cfunc: AnalysisFunction,
) -> AnalsisFunctionResult {
    let mut result = AnalsisFunctionResult::new(rfunc.clone(), cfunc.clone());

    // check arg len
    if rfunc.params.len() != cfunc.params.len() {
        result.add_info(AnalysisResultContent::error(
            0,
            0,
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
                i,
                i,
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
                0,
                0,
                AnalysisResultType::RetPassByMismatch,
                format!("Ret passby mismatch"),
            ));
        }

        return result;
    } else {
        result.add_info(AnalysisResultContent::error(
            0,
            0,
            AnalysisResultType::RetMismatch,
            format!("Ret type mismatch"),
        ));

        return result;
    }
}
