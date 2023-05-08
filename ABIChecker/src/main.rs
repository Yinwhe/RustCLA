use std::{collections::HashMap};

use inkwell::context::Context;
use target::host_target;

extern crate inkwell;

mod analysis;
mod analysis_types;
mod collect;
mod target;

fn main() {
    let c_cx = Context::create();
    let r_cx = Context::create();
    
    let cinfo = collect::collect_info_from_cpp_file("tests/test2/cpp.cc", None, None, &c_cx);
    println!("{:#?}", cinfo);
    let rinfo = collect::collect_info_from_rust_file("tests/test2/rust.rs", &r_cx);
    println!("{:#?}", rinfo);

    // println!("{:#?}", cinfo);
    // println!("{:#?}", rinfo);
    

    // let mut map = HashMap::new();
    // for st in cinfo.info_structs {
    //     map.insert(st.name.clone().unwrap(), st);
    // }

    // let target = host_target();

    // for st in rinfo.info_structs {
    //     let name = st.name.clone().unwrap();
    //     if let Some(c) = map.remove(&name) {
    //         let res = analysis::info_struct_analysis(st, c, target.get_target_data());
    //         println!("{:?}", res);
    //     }
    // }
}

// fn test() {
//     use inkwell::{context::Context, module::Module, targets::*, OptimizationLevel};
//     use crate::analysis::struct_layout_analysis;
//         // For test
//         let rust_cx = Context::create();
//         let rust_m =
//             Module::parse_bitcode_from_path("tests/i1/rust.bc", &rust_cx).expect("Parse bitcode fails");

//         let cpp_cx: Context = Context::create();
//         let cpp_m =
//             Module::parse_bitcode_from_path("tests/i1/cpp.bc", &cpp_cx).expect("Parse bitcode fails");

//         Target::initialize_native(&InitializationConfig::default())
//             .expect("Could not initialize native target");

//         let triple = TargetMachine::get_default_triple();
//         let cpu = TargetMachine::get_host_cpu_name().to_string();
//         let features = TargetMachine::get_host_cpu_features().to_string();

//         let target = Target::from_triple(&triple).expect("Could not create target from triple");
//         let target_machine = target
//             .create_target_machine(
//                 &triple,
//                 &cpu,
//                 &features,
//                 OptimizationLevel::None,
//                 RelocMode::Default,
//                 CodeModel::Default,
//             )
//             .expect("Could not create target machine");

//         let target_data = target_machine.get_target_data();

//         let rust_b = rust_m.get_struct_type("B").unwrap();
//         let cpp_b = cpp_m.get_struct_type("class.B").unwrap();

//         struct_layout_analysis(rust_b, cpp_b, target_data);
// }
