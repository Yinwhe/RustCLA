use std::{collections::HashMap, process::Command};

use inkwell::{context::Context, module::Module, targets::TargetData, values::FunctionValue};

use crate::{analysis_types::*, target::host_target};

use collect_rust::{parse, RFunction, RStructType};

pub fn collect_info_from_rust_file<'cx>(
    file: &str,
    cx: &'cx Context,
) -> Result<Analysis<'cx>, String> {
    let rinfo = if let Some(rinfo) = parse(file) {
        rinfo
    } else {
        return Err(format!("collect info from rust file fails"));
    };

    generate_bcfile(file)?;
    let map = collect_mangles()?;

    // parse bc code
    let module = match Module::parse_bitcode_from_path("rust.bc", cx) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!("parse bitcode from path fails, due to {:?}", e));
        }
    };

    let target_machine = host_target();

    // deal wtih structs
    let mut info_structs = Vec::new();
    let mut raw_structs = Vec::new();

    for rst in rinfo.structs {
        match resolve_one_struct(rst, &module, target_machine.get_target_data()) {
            Ok(st) => {
                // resolve ok
                info_structs.push(st);
            }
            Err((msg, None)) => { // rersolve error
                 // TODO
            }
            Err((msg, Some(st))) => {
                // resolve fail, but raw struct is ok
                // TODO
                raw_structs.push(st);
            }
        }
    }

    // deal with functions
    let mut functions = Vec::new();

    for rfunc in rinfo.funcs {
        if let Some(Some(funcv)) = map
            .get(&rfunc.name)
            .map(|mangled| module.get_function(mangled))
        {
            match resolve_one_func(rfunc, &funcv) {
                Ok(func) => {
                    functions.push(func);
                }
                Err(msg) => {
                    // TODO
                }
            }
        } else {
            // TODO
        }
    }

    Ok(Analysis {
        info_structs,
        raw_structs,
        functions,
    })
}

fn generate_bcfile(file: &str) -> Result<(), String> {
    let res = Command::new("rustc")
        .args(&["--emit=llvm-bc", "--crate-type=rlib", "-o", "rust.bc", file])
        .output()
        .expect("failed to execute process");

    if res.status.success() {
        Ok(())
    } else {
        Err(format!("generate bc file fails, due to {:?}", res.stderr))
    }
}

fn collect_mangles() -> Result<HashMap<String, String>, String> {
    let ori = Command::new("llvm-nm")
        .args(["-C", "rust.bc"])
        .output()
        .expect("failed to execute process");

    let mangled = Command::new("llvm-nm")
        .args(["rust.bc"])
        .output()
        .expect("failed to execute process");

    if !ori.status.success() || !mangled.status.success() {
        return Err(format!(
            "collect mangles fails, due to {}, {}",
            String::from_utf8_lossy(&ori.stderr),
            String::from_utf8_lossy(&mangled.stderr)
        ));
    }

    let mut map = HashMap::new();

    let ori = String::from_utf8_lossy(&ori.stdout);
    let ori: Vec<&str> = ori.split('\n').filter(|s| !s.is_empty()).collect();

    let mangled = String::from_utf8_lossy(&mangled.stdout);
    let mangled: Vec<&str> = mangled.split('\n').filter(|s| !s.is_empty()).collect();

    assert!(ori.len() == mangled.len());

    for i in 0..ori.len() {
        map.insert(
            ori[i]
                .split(' ')
                .last()
                .unwrap()
                .trim_end_matches("()")
                .to_string(),
            mangled[i]
                .split(' ')
                .last()
                .unwrap()
                .trim_end_matches("()")
                .to_string(),
        );
    }

    Ok(map)
}

fn __resolve_one_struct(ast: &mut AnalysisStruct, rst: &RStructType) -> Result<(), String> {
    let name = ast.name.clone().expect("Should not happen");

    match rst {
        RStructType::RStruct(rst) => {
            let mut index = 0;
            let len = rst.fields.len();
            for rf in &mut ast.fields {
                if index >= len {
                    // info parsed done, finsh the rest fields
                    if rf.can_be_anytype() {
                        rf.is_padding = Some(true);
                    }
                    continue;
                }

                let f = &rst.fields[index];

                if f.ty.get_type_id() == rf.get_type_id() {
                    rf.name = f.name.clone();
                    rf.is_padding = Some(false);
                    index += 1;

                    // recursive resolve
                    if let Some(st) = rf.get_struct_mut() {
                        let rst = f.get_struct().expect("Fatal error, should not happen");

                        if let Err(e) = __resolve_one_struct(st, rst) {
                            return Err(e);
                        }
                    }
                } else if rf.can_be_anytype() {
                    rf.is_padding = Some(true);
                }
            }

            if index != len {
                return Err(format!(
                    "resolve AnalysisStruct fail, {} info not match",
                    name
                ));
            }

            ast.is_raw = false;
            ast.is_enum = Some(false);
            ast.is_union = Some(false);
            ast.name = Some(name);

            return Ok(());
        }
        RStructType::RUnion(_rst) => {
            ast.is_raw = false;
            ast.is_enum = Some(false);
            ast.is_union = Some(true);
            ast.name = Some(name);

            return Ok(());
        }
        RStructType::REnum(_rst) => {
            ast.is_raw = false;
            ast.is_enum = Some(true);
            ast.is_union = Some(false);
            ast.name = Some(name);

            return Ok(());
        }
    }
}

fn resolve_one_struct<'ctx>(
    rst: RStructType,
    module: &Module<'ctx>,
    target_data: TargetData,
) -> Result<AnalysisStruct<'ctx>, (String, Option<AnalysisStruct<'ctx>>)> {
    let name = if let Some(name) = rst.get_name() {
        name
    } else {
        return Err((
            format!("resolve AnalysisStruct fail, anonymous structtype unsupported"),
            None,
        ));
    };

    let struct_type = if let Some(struct_type) = module.get_struct_type(&format!("{}", name)) {
        struct_type
    } else {
        return Err((
            format!(
                "resolve AnalysisStruct fail, struct type {} not found",
                name
            ),
            None,
        ));
    };

    let mut ast = AnalysisStruct::from_ctx_raw(struct_type, &target_data);

    if let Err(e) = __resolve_one_struct(&mut ast, &rst) {
        return Err((e, Some(ast)));
    };

    Ok(ast)
}

fn resolve_one_func(rfunc: RFunction, funcv: &FunctionValue) -> Result<AnalysisFunction, String> {
    let name = rfunc.name;
    let mut params = Vec::new();

    // assert!(cfunc.args.len() == funcv.count_params() as usize);
    for param in funcv.get_params() {
        let p = if param.is_pointer_value() {
            AnalysisParameters {
                name: None,
                pass_by: AnalysisPassBy::PointerOrReference,
            }
        } else {
            AnalysisParameters {
                name: None,
                pass_by: AnalysisPassBy::Value,
            }
        };

        params.push(p);
    }

    let ret = if let Some(retv) = funcv.get_type().get_return_type() {
        if retv.is_pointer_type() {
            Some(AnalysisParameters {
                name: None,
                pass_by: AnalysisPassBy::PointerOrReference,
            })
        } else {
            Some(AnalysisParameters {
                name: None,
                pass_by: AnalysisPassBy::Value,
            })
        }
    } else {
        None
    };

    Ok(AnalysisFunction { name, params, ret })
}
