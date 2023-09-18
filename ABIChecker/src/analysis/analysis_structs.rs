pub fn analysis_structs() {}

// fn deep_check(a: &AType, b: &AType) -> AResults {
//     let mut res = AResults::new();
//     match (a, b) {
//         (AType::ArrayType(a, alen), AType::ArrayType(b, blen)) => {
//             if alen != blen {
//                 return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
//             }

//             return deep_check(a, b);
//         }
//         (AType::FloatType(astr), AType::FloatType(bstr)) => {
//             if astr != bstr {
//                 return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
//             } else {
//                 None
//             }
//         }
//         (AType::IntType(astr), AType::IntType(bstr)) => {
//             if astr != bstr {
//                 return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
//             } else {
//                 None
//             }
//         }
//         (AType::PointerType(a), AType::PointerType(b)) => {
//             return deep_check(a, b);
//         }
//         (AType::StructType(a), AType::StructType(b)) => _analysis_struct(a, b),
//         (AType::VectorType(a), AType::VectorType(b)) => {
//             return deep_check(a, b);
//         }
//         _ => Some(AResult::ParamIssue(0, ParamMismatch::TypeMismatch)),
//     }
// }

// fn _analysis_struct(rstruct: &AStruct, cstruct: &AStruct) -> Option<AResult> {
//     enum AStatus {
//         Match,
//         RustRemain,
//         CppRemain,
//     }

//     let mut status = AStatus::Match;

//     let mut matches = Vec::new();

//     let mut r_index = 0;
//     let mut c_index = 0;

//     let mut r_match = Vec::new();
//     let mut c_match = Vec::new();

//     let mut processed_total_size = 0;
//     let mut accumulate = 0;

//     // Check size
//     while let (Some(rf), Some(cf)) = (rstruct.get_field(r_index), cstruct.get_field(c_index)) {
//         let (rf_size, cf_size) = (rf.get_size(), cf.get_size());
//         debug!("check size: {:?}, {:?}", rf, cf);

//         match status {
//             AStatus::Match => {
//                 // matches
//                 if rf_size == cf_size {
//                     processed_total_size += rf_size;

//                     // store matches
//                     matches.push((rf, cf));

//                     // renew
//                     r_index += 1;
//                     c_index += 1;
//                 } else if rf_size < cf_size {
//                     // rust fields smaller, c fields wonn't iter
//                     accumulate += rf_size;
//                     r_match.push(rf);

//                     // renew
//                     r_index += 1;
//                     status = AStatus::CppRemain;
//                 } else {
//                     // c fields smaller, rust fields wonn't iter
//                     accumulate += cf_size;
//                     c_match.push(cf);

//                     // renew
//                     c_index += 1;
//                     status = AStatus::RustRemain;
//                 }
//             }
//             AStatus::RustRemain => {
//                 if rf_size == cf_size + accumulate {
//                     // matches
//                     processed_total_size += rf_size;
//                     c_match.push(cf);

//                     // store matches
//                     let temp_f = AField::temp_st(c_match);

//                     matches.push((rf, temp_f));

//                     // renew
//                     c_match = Vec::new();
//                     r_index += 1;
//                     c_index += 1;
//                     accumulate = 0;

//                     status = AStatus::Match;
//                 } else if rf_size > cf_size + accumulate {
//                     // rust still remains
//                     accumulate += cf_size;
//                     c_match.push(cf);

//                     // renew
//                     c_index += 1;
//                 } else {
//                     // mismatch occurs
//                     return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
//                     // result.add_info(AnalysisResultContent::error(
//                     //     rf.range,
//                     //     cf.range,
//                     //     AnalysisResultType::SizeMismatch,
//                     //     format!("Size mismatch, rust field end in the mid of c++ field"),
//                     // ));
//                     // return result;
//                 }
//             }
//             AStatus::CppRemain => {
//                 if rf_size + accumulate == cf_size {
//                     // matches
//                     processed_total_size += cf_size;
//                     r_match.push(rf);

//                     // store matches
//                     let temp_f = AField::temp_st(r_match);

//                     matches.push((cf, temp_f));

//                     // renew
//                     r_match = Vec::new();
//                     r_index += 1;
//                     c_index += 1;
//                     accumulate = 0;

//                     status = AStatus::Match;
//                 } else if rf_size + accumulate < cf_size {
//                     // c still remains
//                     accumulate += rf_size;
//                     r_match.push(rf);

//                     // renew
//                     r_index += 1;
//                 } else {
//                     // mismatch occurs
//                     return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
//                     // result.add_info(AnalysisResultContent::error(
//                     //     rf.range,
//                     //     cf.range,
//                     //     AnalysisResultType::SizeMismatch,
//                     //     format!("Size mismatch, c++ field end in the mid of rust field"),
//                     // ));
//                     // return result;
//                 }
//             }
//         } // match ends
//     } // while ends

//     // Check remain rust fields
//     while let Some(rf) = rstruct.get_field(r_index) {
//         r_match.push(rf);
//         r_index += 1;
//     }

//     // Check remain c fields
//     while let Some(cf) = cstruct.get_field(c_index) {
//         c_match.push(cf);
//         c_index += 1;
//     }

//     // we have remaining unmatched fields
//     let r_remain = r_match;
//     let c_remain = c_match;

//     debug!(
//         "processed_size: {}\nmatches: {:#?}",
//         processed_total_size, matches
//     );

//     debug!("r_remain: {:#?}\nc_remain: {:#?}", r_remain, c_remain);

//     // unmatched fields, should be errors
//     if !r_remain.is_empty() || !c_remain.is_empty() {
//         return Some(AResult::ParamIssue(0, ParamMismatch::SizeMismatch));
//         // result.add_info(AnalysisResultContent::error(
//         //     (processed_total_size, 0),
//         //     (processed_total_size, 0),
//         //     AnalysisResultType::SizeMismatch,
//         //     format!("Unmatched fields found, can be errors"),
//         // ));
//         // // but no return
//     }

//     // type checks
//     for (rf, cf) in matches {
//         // 1 - normal compare
//         // 2 - array compare
//         // 3 - struct compare
//         let rty = &rf.ty;
//         let cty = &cf.ty;
//         match (rf.cmp_number(), cf.cmp_number()) {
//             (1, 1) => {
//                 deep_check(rty, cty);
//             }
//             (1, 2) => {}
//             (1, 3) => {}
//             (2, 1) => {}
//             (2, 2) => {}
//             (2, 3) => {}
//             (3, 1) => {}
//             (3, 2) => {}
//             (3, 3) => {}
//             _ => unreachable!(),
//         }
//     }

//     None
// }
