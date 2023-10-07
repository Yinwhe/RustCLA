use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;

use crate::utils;

use super::analysis_funcs::analysis_funcs;
use super::analysis_structs::analysis_structs;
use super::ir_info::IRInfo;

/// Top analysis function.
pub fn analysis_ir(
    path: PathBuf,
    targets: Vec<String>,
    target_machin: TargetMachine,
) -> Result<(), String> {
    let cx = Context::create();

    utils::info_prompt("Analysis IR", "forming ir information db...");
    let mut ir_info = resolve_from_bc(path, targets, &cx, target_machin)?;
    ir_info.get_ffi_funcs()?;

    let ffi_funcs = ir_info.ffi_functions();

    utils::info_prompt(
        "Analysis IR",
        &format!("checking ffi functions: {:?}", ffi_funcs),
    );
    analysis_funcs(ffi_funcs, &mut ir_info)?;

    let ffi_structs = ir_info.ffi_structs();
    let ffi_struct_names = ffi_structs
        .iter()
        .map(|(r, c)| (r.get_lazy().0, c.get_lazy().0))
        .collect::<Vec<(&String, &String)>>();
    utils::info_prompt(
        "Analysis IR",
        &format!("checking ffi structs: {:?}", ffi_struct_names),
    );
    analysis_structs(ffi_structs, &mut ir_info)?;

    Ok(())
}

/// Resolve the IR from the bitcode file and collect all modules.
fn resolve_from_bc<'ctx>(
    path: PathBuf,
    targets: Vec<String>,
    cx: &'ctx Context,
    target_machine: TargetMachine,
) -> Result<IRInfo<'ctx>, String> {
    let file = File::open(path).map_err(|e| format!("{}", e))?;
    let lines = BufReader::new(file).lines();

    // let cx = Context::create();
    let mut c_modules = Vec::new();
    let mut r_modules = Vec::new();

    // read each bitcode file and collect the IR
    for line in lines {
        let line = line.map_err(|e| format!("{}", e))?;
        let path = Path::new(&line);
        let module = Module::parse_bitcode_from_path(path, cx).map_err(|e| format!("{}", e))?;

        // Judge whether it is a C module or a Rust module
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".bc") {
            if targets.iter().any(|t| file_name.starts_with(t)) {
                r_modules.push(module)
            }
            // or it's just dependencies crates
        } else if file_name.ends_with(".o") {
            c_modules.push(module);
        } else {
            panic!("Unsupported file type: {}", file_name);
        }
    }

    let ir_info = IRInfo::new(r_modules, c_modules, target_machine);

    Ok(ir_info)
}
