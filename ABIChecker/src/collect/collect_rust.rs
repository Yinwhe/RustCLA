use inkwell::{context::Context, module::Module, targets::TargetData, values::FunctionValue};
use std::{fs, process::Command};

use super::{helper::collect_mangles, rtypes::*};
use crate::{analysis_types::*, target::host_target, HOME, RUSTC, Args};

const COLLECT_RUST: &str = "$HOME/.abi_checker/collect_rust";

pub fn collect_info_from_rust_file(
    file: &str,
    cx: &Context,
    args: &Args
) -> Result<Analysis, String> {
    let res = Command::new("sh")
        .args(["-c", &format!("{} {}", COLLECT_RUST, file)])
        .output()
        .expect("failed to execute process");

    let rinfo = if res.status.success() {
        let rinfo: RInfo = serde_json::from_slice(&res.stdout).unwrap();
        rinfo
    } else {
        return Err(format!(
            "collect info from rust file fails, due to {:?}",
            String::from_utf8_lossy(&res.stderr)
        ));
    };

    if !args.no_pad_info {
        add_root(file, &rinfo)?;
    }
    generate_bcfile(&format!("{HOME}/.abi_checker/rust.rs"))?;

    // parse bc code
    let module = match Module::parse_bitcode_from_path(format!("{HOME}/.abi_checker/rust.bc"), cx) {
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
            Err(msg) => {
                // rersolve error
                // TODO
                warn!("collect rust struct fails: {:?}", msg);
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
                    warn!("collect rust function fails: {:?}", msg);
                }
            }
        } else {
            // TODO
            warn!(
                "collect rust function fails: func {} not found in binarycode",
                rfunc.name
            );
        }
    }

    Ok(Analysis { structs, functions })
}

fn add_root(file: &str, rinfo: &RInfo) -> Result<(), String> {
    let mut buf = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => return Err(format!("add root fails, due to {:?}", e)),
    };

    let stnames: Vec<String> = rinfo
        .structs
        .iter()
        .filter_map(|st| st.get_name().map(|s| s.to_string()))
        // .filter(|n| !n.starts_with("_"))
        .collect();

    let mut index = 0;
    let len = stnames.len();
    while index < stnames.len() {
        if len - index >= 4 {
            let a = &stnames[index];
            let b = &stnames[index + 1];
            let c = &stnames[index + 2];
            let d = &stnames[index + 3];
            let s = format!(
                "\n#[no_mangle]\nextern \"C\" fn root{}(a: {}, b: {}, c: {}, d: {}){{}}\n",
                index, a, b, c, d
            );
            buf.push_str(&s);
            index += 4;
        } else {
            let a = &stnames[index];
            let s = format!("\n#[no_mangle]\nextern \"C\" fn root{}(a: {}){{}}\n", index, a);
            buf.push_str(&s);
            index += 1;
        }
    }

    if let Err(e) = fs::write(&format!("{HOME}/.abi_checker/rust.rs"), buf) {
        return Err(format!("add root fails, due to {:?}", e));
    }

    Ok(())
}

fn generate_bcfile(file: &str) -> Result<(), String> {
    let res = Command::new("sh")
        .args(&[
            "-c",
            &format!(
                "{} --emit=llvm-bc --crate-type=rlib -o $HOME/.abi_checker/rust.bc {}",
                RUSTC, file
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
    rst: RStructType,
    module: &Module,
    target_data: TargetData,
) -> Result<AnalysisStruct, String> {
    let name = if let Some(name) = rst.get_name() {
        name
    } else {
        return Err(format!(
            "resolve AnalysisStruct fail, anonymous struct type unsupported"
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

    let mut ast = AnalysisStruct::from_ctx_raw(struct_type, 0, &target_data);

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

fn fix_detail_types(raw_field: &mut AnalysisField, info_field: &RField) {
    assert!(raw_field.get_type_id() == info_field.ty.get_type_id());
    match (&mut raw_field.ty, &info_field.ty) {
        (AnalysisFieldType::IntType(_), RType::IntType(rik)) => {
            let nty = AIntType::from(rik);
            raw_field.ty = AnalysisFieldType::IntType(nty)
        }
        _ => trace!("Unimplement Yet"),
    }
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
                    if rf.is_array() {
                        rf.is_padding = true;
                    }
                    continue;
                }

                let f = &rst.fields[index];

                if f.ty.get_type_id() == rf.get_type_id() {
                    fix_detail_types(rf, f);
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
        RStructType::RUnion(_rst) => {
            ast.is_enum = false;
            ast.is_union = true;
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

            return Ok(());
        }
        RStructType::REnum(_rst) => {
            ast.is_enum = true;
            ast.is_union = false;
            ast.name = name;

            if let Some(f) = ast.fields.first() {
                let mut tag = f.clone();
                tag.ty = AnalysisFieldType::IntType(AIntType::CEnum);
                tag.name = Some("tag".to_string());

                let start = ast.fields[1].range.0;
                let end = ast.fields.last().unwrap().range.1;
                let mut payload = AnalysisField::padding(start, end);
                payload.is_padding = false;

                ast.fields.clear();
                ast.fields.push(tag);
                ast.fields.push(payload);
            }

            return Ok(());
        }
    }
}
