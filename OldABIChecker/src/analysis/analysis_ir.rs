use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;

use crate::{utils, Opts};

use super::analysis_funcs::analysis_funcs;
use super::analysis_structs::analysis_structs;
use super::ir_info::IRInfo;

#[derive(Debug)]
pub struct AnalysisOverriew {
    pub func_warns: usize,
    pub func_errors: usize,
    pub struct_warns: usize,
    pub struct_errors: usize,
    pub has_rust_modules: bool,
    pub has_cxx_modules: bool,
}

/// Top analysis function.
pub fn analysis_ir(opts: Opts) -> Result<AnalysisOverriew, String> {
    let cx = Context::create();

    let path = unsafe { opts.bitcode_path.assume_init() };
    let target_machin = unsafe { opts.target_machin.assume_init() };

    utils::info_prompt("Analysis IR", "forming ir information db...", opts.print);
    let mut ir_info = resolve_from_bc(path, &cx, target_machin)?;
    ir_info.get_ffi_funcs()?;

    let ffi_funcs = ir_info.ffi_functions();

    utils::info_prompt(
        "Analysis IR",
        &format!("checking ffi functions: {:?}", ffi_funcs),
        opts.print,
    );
    let (func_warns, func_errors) = analysis_funcs(ffi_funcs, &mut ir_info, opts.print)?;

    let ffi_structs = ir_info.ffi_structs();
    let ffi_struct_names = ffi_structs
        .iter()
        .map(|(r, c)| (r.get_lazy().0, c.get_lazy().0))
        .collect::<Vec<(&String, &String)>>();
    utils::info_prompt(
        "Analysis IR",
        &format!("checking ffi structs: {:?}", ffi_struct_names),
        opts.print,
    );
    let (struct_warns, struct_errors) = analysis_structs(ffi_structs, &mut ir_info, opts.print)?;

    utils::info_prompt(
        "Summarize",
        &format!("{} errors found", func_errors + struct_errors),
        opts.print,
    );

    Ok(AnalysisOverriew {
        func_warns,
        func_errors,
        struct_warns,
        struct_errors,
        has_rust_modules: ir_info.has_rust_modules(),
        has_cxx_modules: ir_info.has_cxx_modules(),
    })
}

/// Resolve the IR from the bitcode file and collect all modules.
fn resolve_from_bc<'ctx>(
    path: PathBuf,
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
        let file_name = path
            .file_name()
            .ok_or("Invalid file name")?
            .to_str()
            .ok_or("Invalid file name")?;
        if file_name.ends_with(".bc") {
            r_modules.push(module)
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
