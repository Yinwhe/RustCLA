use std::process::Command;

use inkwell::{context::Context, module::Module, targets::TargetData, values::FunctionValue};

use super::{ctypes::*, helper::collect_mangles};
use crate::{analysis_types::*, target::host_target, CLANG, HOME};

const COLLECT_CXX: &str = "$HOME/.abi_checker/collect_cxx";

#[inline]
pub fn collect_info_from_cpp_file<'cx>(
    file: &str,
    _clang_target: Option<&str>,
    _cpp_standard: Option<&str>,
    cx: &'cx Context,
) -> Result<Analysis, String> {
    let res = Command::new("sh")
        .args(&["-c", &format!("{} {}", COLLECT_CXX, file)])
        .output()
        .expect("failed to execute process");

    let cinfo = if res.status.success() {
        let cinfo: CInfo = serde_json::from_slice(&res.stdout).unwrap();
        cinfo
    } else {
        return Err(format!(
            "collect info from cpp file fails, due to {:?}",
            String::from_utf8_lossy(&res.stderr)
        ));
    };
    
    generate_bcfile(file)?;

    // parse bc code
    let module = match Module::parse_bitcode_from_path(format!("{HOME}/.abi_checker/cpp.bc"), cx) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!("parse bitcode from path fails, due to {:?}", e));
        }
    };

    let target_machine = host_target();

    // deal wtih structs
    let mut structs = Vec::new();

    for cst in cinfo.structs {
        match resolve_one_struct(cst, &module, target_machine.get_target_data()) {
            Ok(st) => {
                // resolve ok
                structs.push(st);
            }
            Err(msg) => { // rersolve error
                warn!("collect cpp struct info fails: {:?}", msg);
                // TODO
            }
        }
    }

    // deal with functions
    let mut functions = Vec::new();

    let names: Vec<String> = module
        .get_functions()
        .map(|f| f.get_name().to_string_lossy().into_owned())
        .collect();
    let map = collect_mangles(names);

    for cfunc in cinfo.funcs {
        if let Some(Some(funcv)) = map
            .get(&cfunc.name)
            .map(|mangled| module.get_function(mangled))
        {
            match resolve_one_func(cfunc, &funcv) {
                Ok(func) => {
                    functions.push(func);
                }
                Err(msg) => {
                    // TODO
                    warn!("collect cpp function info fails: {:?}", msg);
                }
            }
        } else {
            // TODO
            warn!("collect cpp function fails: func {} not found in binarycode", cfunc.name);
        }
    }

    Ok(Analysis { structs, functions })
}

fn generate_bcfile(file: &str) -> Result<(), String> {
    let res = Command::new("sh")
        .args(&[
            "-c",
            &format!(
                "{} -c -emit-llvm -o $HOME/.abi_checker/cpp.bc {}",
                CLANG, file
            ),
        ])
        .output()
        .expect("failed to execute process");

    if res.status.success() {
        Ok(())
    } else {
        Err(format!(
            "generate bc file fails, due to {:?}",
            String::from_utf8_lossy(&res.stderr)
        ))
    }
}

fn resolve_one_struct(
    cst: CStruct,
    module: &Module,
    target_data: TargetData,
) -> Result<AnalysisStruct, String> {
    let name = if let Some(name) = cst.name.clone() {
        name
    } else {
        return Err(format!(
            "resolve AnalysisStruct fail, anonymous struct unsupported"
        ));
    };

    if cst.is_union {
        let struct_type =
            if let Some(struct_type) = module.get_struct_type(&format!("union.{}", name)) {
                struct_type
            } else {
                return Err(format!(
                    "resolve AnalysisStruct fail, union type {} not found",
                    name
                ));
            };

        let mut ast = AnalysisStruct::from_ctx_raw(struct_type, 0, &target_data);

        ast.is_union = true;
        ast.is_enum = false;
        ast.name = Some(name);

        return Ok(ast);
    } else {
        let struct_type =
            if let Some(struct_type) = module.get_struct_type(&format!("class.{}", name)) {
                struct_type
            } else if let Some(struct_type) = module.get_struct_type(&format!("struct.{}", name)) {
                struct_type
            } else {
                return Err(format!(
                    "resolve AnalysisStruct fail, struct type {} not found",
                    name
                ));
            };

        let mut ast = AnalysisStruct::from_ctx_raw(struct_type, 0, &target_data);

        if let Err(e) = __resolve_one_struct(&mut ast, &cst) {
            return Err(e);
        } else {
            return Ok(ast);
        }
    }
}

fn fix_detail_types(raw_field: &mut AnalysisField, info_field: &CField) {
    assert!(raw_field.get_type_id() == info_field.ty.get_type_id());
    match (&mut raw_field.ty, &info_field.ty) {
        (AnalysisFieldType::IntType(_), CType::IntType(cik)) => {
            let nty = AIntType::from(cik);
            raw_field.ty = AnalysisFieldType::IntType(nty)
        }
        _ => trace!("Unimplement Yet")
    }
}

fn __resolve_one_struct(ast: &mut AnalysisStruct, cst: &CStruct) -> Result<(), String> {
    let name = cst.name.clone();

    // for union type, no need to match fields
    if cst.is_union {
        ast.is_union = true;
        ast.is_enum = false;
        ast.name = name;

        if !ast.fields.is_empty() {
            let start = ast.fields[0].range.0;
            let end = ast.fields.last().unwrap().range.1;

            let mut all = AnalysisField::padding(start, end);
            all.is_padding = false;
            all.name = Some("payload".to_string());

            ast.fields.clear();
            ast.fields.push(all);
        }

        return  Ok(());
    }

    let mut index = 0;
    let len = cst.fields.len();
    for rf in &mut ast.fields {
        if index >= len {
            // info parsed done, finsh the remaining fields
            if rf.is_array() {
                rf.is_padding = true;
            }
            continue;
        }

        let f = &cst.fields[index];

        if f.ty.get_type_id() == rf.get_type_id() {
            println!("match");
            fix_detail_types(rf, f);
            rf.name = f.name.clone();
            rf.is_padding = false;
            index += 1;

            // recursive resolve
            if let Some(st) = rf.get_struct_mut() {
                let cst = f.get_struct().expect("Fatal error, should not happen");

                if let Err(e) = __resolve_one_struct(st, cst) {
                    return Err(e);
                }
            }
        } else if rf.is_array() {
            rf.is_padding = true;
        }
    }

    if index != len {
        return Err(format!(
            "resolve AnalysisStruct fail, {:?} info not match",
            name
        ));
    }

    ast.is_enum = false;
    ast.is_union = false;
    ast.name = name;

    return Ok(());
}

fn resolve_one_func(cfunc: CFunction, funcv: &FunctionValue) -> Result<AnalysisFunction, String> {
    let name = cfunc.name;
    let mut params = Vec::new();

    // assert!(cfunc.args.len() == funcv.count_params() as usize);
    for param in funcv.get_params() {
        let p = AnalysisParameters {
            name: None,
            ty: param.get_type().into(),
        };

        params.push(p);
    }

    let ret = if let Some(retv) = funcv.get_type().get_return_type() {
        Some(AnalysisParameters {
            name: None,
            ty: retv.into(),
        })
    } else {
        None
    };

    Ok(AnalysisFunction { name, params, ret })
}
