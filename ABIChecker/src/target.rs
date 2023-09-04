// use inkwell::{targets::*, OptimizationLevel};

// /// Default target
// pub fn host_target() -> TargetMachine {
//     Target::initialize_native(&InitializationConfig::default())
//         .expect("Could not initialize native target");

//     let triple = TargetMachine::get_default_triple();
//     let cpu = TargetMachine::get_host_cpu_name().to_string();
//     let features = TargetMachine::get_host_cpu_features().to_string();

//     let target = Target::from_triple(&triple).expect("Could not create target from triple");
//     let target_machine = target
//         .create_target_machine(
//             &triple,
//             &cpu,
//             &features,
//             OptimizationLevel::None,
//             RelocMode::Default,
//             CodeModel::Default,
//         )
//         .expect("Could not create target machine");


//     target_machine
// }