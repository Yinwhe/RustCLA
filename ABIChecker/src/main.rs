/*
use inkwell::{context::Context, module::Module, targets::*, OptimizationLevel};

fn main() {
    let rust_cx = Context::create();
    let rust_m =
        Module::parse_bitcode_from_path("tests/i1/rust.bc", &rust_cx).expect("Parse bitcode fails");

    let cpp_cx: Context = Context::create();
    let cpp_m =
        Module::parse_bitcode_from_path("tests/i1/cpp.bc", &cpp_cx).expect("Parse bitcode fails");

    Target::initialize_native(&InitializationConfig::default())
        .expect("Could not initialize native target");

    let triple = TargetMachine::get_default_triple();
    let cpu = TargetMachine::get_host_cpu_name().to_string();
    let features = TargetMachine::get_host_cpu_features().to_string();

    let target = Target::from_triple(&triple).expect("Could not create target from triple");
    let target_machine = target
        .create_target_machine(
            &triple,
            &cpu,
            &features,
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        )
        .expect("Could not create target machine");

    let target_data = target_machine.get_target_data();

    let cpp_b = cpp_m.get_struct_type("class.B").unwrap();
    let rust_b = rust_m.get_struct_type("B").unwrap();

    // CPP
    // println!("{:?}", target_data.offset_of_element(&cpp_b, 1));
    // println!("{:?}", target_data.offset_of_element(&rust_b, 1));

    println!("{:#?}", cpp_b.get_field_types());
    println!("{:#?}", rust_b.get_field_types());
}

*/

extern crate inkwell;
extern crate bindgen_cxx_parser;

fn main() {
    let a = 1;
}