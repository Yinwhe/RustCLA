use std::path::PathBuf;

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

    let target_machine = unsafe { opts.target_machine.assume_init() };

    utils::info_prompt("Analysis IR", "forming ir information db...", true);
    let mut ir_info = resolve_from_bc(opts.c_ir_file, opts.r_ir_file, &cx, target_machine)?;
    ir_info.get_ffi_funcs()?;

    let ffi_funcs = ir_info.ffi_functions();

    utils::info_prompt(
        "Analysis IR",
        &format!("checking ffi functions: {:?}", ffi_funcs),
        true,
    );
    let (func_warns, func_errors) = analysis_funcs(ffi_funcs, &mut ir_info, true)?;

    let ffi_structs = ir_info.ffi_structs();
    let ffi_struct_names = ffi_structs
        .iter()
        .map(|(r, c)| (r.get_lazy().0, c.get_lazy().0))
        .collect::<Vec<(&String, &String)>>();
    utils::info_prompt(
        "Analysis IR",
        &format!("checking ffi structs: {:?}", ffi_struct_names),
        true,
    );
    let (struct_warns, struct_errors) = analysis_structs(ffi_structs, &mut ir_info, true)?;

    utils::info_prompt(
        "Summarize",
        &format!("{} errors found", func_errors + struct_errors),
        true,
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
    c_path: PathBuf,
    r_path: PathBuf,
    cx: &'ctx Context,
    target_machinee: TargetMachine,
) -> Result<IRInfo<'ctx>, String> {
    let c_module = Module::parse_bitcode_from_path(c_path, cx).map_err(|e| format!("{}", e))?;
    let r_module = Module::parse_bitcode_from_path(r_path, cx).map_err(|e| format!("{}", e))?;

    // let cx = Context::create();
    let mut c_modules = Vec::new();
    let mut r_modules = Vec::new();

    c_modules.push(c_module);
    r_modules.push(r_module);

    let ir_info = IRInfo::new(r_modules, c_modules, target_machinee);

    Ok(ir_info)
}
