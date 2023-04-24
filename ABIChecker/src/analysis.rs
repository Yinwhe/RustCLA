use collect_cxx::*;
use collect_rust::*;
use inkwell::{module::Module, targets::TargetData, types::BasicTypeEnum, types::StructType};

use crate::analysis_types::*;

use std::collections::HashMap;

pub fn analysis_struct(rm: Module, cm: Module, rinfo: RInfo, cinfo: CInfo) {
    let mut rust_struct = HashMap::new();
    let mut cpp_struct = HashMap::new();

    for rs in rinfo.structs {
        let s = match rm.get_struct_type(&rs.name) {
            Some(s) => s,
            None => continue,
        };

        rust_struct.insert(rs.name, s);
    }

    for cs in cinfo.structs {
        let s = match cm.get_struct_type(&format!("class.{}", cs.name)) {
            Some(s) => s,
            None => match cm.get_struct_type(&format!("struct.{}", cs.name)) {
                Some(s) => s,
                None => continue,
            },
        };

        cpp_struct.insert(cs.name, s);
    }
}
/// Three parts: align, size, type
pub fn struct_layout_analysis(rstruct: StructType, cstruct: StructType, target_data: TargetData) {
    println!("Debug:\nrstruct: {:?}\n cstruct: {:?}\n", rstruct, cstruct);
    enum AnalysisStatus {
        Match,
        RustRemain,
        CppRemain,
    }

    let mut status = AnalysisStatus::Match;

    let rstruct = AnalysisStruct::from_ctx(rstruct, &target_data);
    let cstruct = AnalysisStruct::from_ctx(cstruct, &target_data);

    let mut matches = Vec::new();

    let mut r_iters = rstruct.get_fields_iters();
    let mut c_iters = cstruct.get_fields_iters();

    let mut r_process = r_iters.next();
    let mut c_process = c_iters.next();

    let mut r_match = Vec::new();
    let mut c_match = Vec::new();

    let mut processed_total_size = 0;
    let mut accumulate = 0;

    while r_process.is_some() && c_process.is_some() {
        let (rf, cf) = (r_process.unwrap(), c_process.unwrap());
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
                    r_process = r_iters.next();
                    c_process = c_iters.next();
                } else if rf_size < cf_size {
                    // rust fields smaller, c fields wonn't iter
                    accumulate += rf_size;
                    r_match.push(rf);

                    // renew
                    r_process = r_iters.next();
                    c_process = Some(cf);
                    status = AnalysisStatus::CppRemain;
                } else {
                    // c fields smaller, rust fields wonn't iter
                    accumulate += cf_size;
                    c_match.push(cf);

                    // renew
                    r_process = Some(rf);
                    c_process = c_iters.next();
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
                    r_process = r_iters.next();
                    c_process = c_iters.next();

                    status = AnalysisStatus::Match;
                } else if rf_size > cf_size + accumulate {
                    // rust still remains
                    accumulate += cf_size;
                    c_match.push(cf);

                    // renew
                    r_process = Some(rf);
                    c_process = c_iters.next();
                } else {
                    // TODO: mismatch occurs
                    panic!("Mismatch occurs")
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
                    r_process = r_iters.next();
                    c_process = c_iters.next();

                    status = AnalysisStatus::Match;
                } else if rf_size + accumulate < cf_size {
                    // c still remains
                    accumulate += rf_size;
                    r_match.push(rf);

                    // renew
                    r_process = r_iters.next();
                    c_process = Some(cf);
                } else {
                    // TODO: mismatch occurs
                    panic!("Mismatch occurs")
                }
            }
        } // match ends
    } // while ends

    // Check remain rust fields
    while let Some(rf) = r_process {
        r_match.push(rf);
        r_process = r_iters.next();
    }

    // Check remain c fields
    while let Some(cf) = c_process {
        c_match.push(cf);
        c_process = c_iters.next();
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

    // Now we can do type checks

}
