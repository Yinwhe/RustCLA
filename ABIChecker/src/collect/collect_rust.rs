use std::process::Command;

use inkwell::{context::Context, module::Module, targets::TargetData, values::FunctionValue};

use crate::{analysis_types::*, collect::helper::collect_mangles, target::host_target};

use collect_rust::{parse, RFunction, RStructType};

pub fn collect_info_from_rust_file(file: &str, cx: &Context) -> Result<Analysis, String> {
    let rinfo = if let Some(rinfo) = parse(file) {
        rinfo
    } else {
        return Err(format!("collect info from rust file fails"));
    };

    print!("{:#?}", rinfo);

    generate_bcfile(file)?;

    // parse bc code
    let module = match Module::parse_bitcode_from_path("rust.bc", cx) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!("parse bitcode from path fails, due to {:?}", e));
        }
    };

    let target_machine = host_target();

    // deal wtih structs
    let mut structs = Vec::new();

    for rst in rinfo.structs {
        match resolve_one_struct(rst, &module, target_machine.get_target_data()) {
            Ok(st) => {
                // resolve ok
                structs.push(st);
            }
            Err(msg) => { // rersolve error
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

    Ok(Analysis { structs, functions })
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

fn resolve_one_struct(
    rst: RStructType,
    module: &Module,
    target_data: TargetData,
) -> Result<AnalysisStruct, String> {
    let name = if let Some(name) = rst.get_name() {
        name
    } else {
        return Err(format!(
            "resolve AnalysisStruct fail, anonymous structtype unsupported"
        ));
    };

    let struct_type = if let Some(struct_type) = module.get_struct_type(&format!("{}", name)) {
        struct_type
    } else {
        return Err(format!(
            "resolve AnalysisStruct fail, struct type {} not found",
            name
        ));
    };

    let mut ast = AnalysisStruct::from_ctx_raw(struct_type, &target_data);

    if let Err(e) = __resolve_one_struct(&mut ast, &rst) {
        return Err(e);
    };

    ast.name = Some(name.to_string());

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

fn __resolve_one_struct(ast: &mut AnalysisStruct, rst: &RStructType) -> Result<(), String> {
    let name = ast.name.clone();

    match rst {
        RStructType::RStruct(rst) => {
            let mut index = 0;
            let len = rst.fields.len();
            for rf in &mut ast.fields {
                if index >= len {
                    // info parsed done, finsh the rest fields
                    if rf.can_be_anytype() {
                        rf.is_padding = true;
                    }
                    continue;
                }

                let f = &rst.fields[index];

                if f.ty.get_type_id() == rf.get_type_id() {
                    rf.name = f.name.clone();
                    rf.is_padding = false;
                    index += 1;

                    // recursive resolve
                    if let Some(st) = rf.get_struct_mut() {
                        let rst = f.get_struct().expect("Fatal error, should not happen");

                        if let Err(e) = __resolve_one_struct(st, rst) {
                            return Err(e);
                        }
                    }
                } else if rf.can_be_anytype() {
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
        RStructType::RUnion(_rst) => {
            ast.is_enum = false;
            ast.is_union = true;
            ast.name = name;

            return Ok(());
        }
        RStructType::REnum(_rst) => {
            ast.is_enum = true;
            ast.is_union = false;
            ast.name = name;

            return Ok(());
        }
    }
}
