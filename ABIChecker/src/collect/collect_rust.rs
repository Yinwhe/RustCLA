use std::process::Command;

use inkwell::{
    context::Context,
    module::Module,
    targets::TargetData,
};

use crate::{analysis_types::*, target::host_target};

use collect_rust::{parse, RStructType};

pub fn collect_info_from_rust_file<'cx>(file: &str, cx: &'cx Context) -> Result<Analysis<'cx>, String> {
    let rinfo = if let Some(rinfo) = parse(file) {
        rinfo
    } else {
        return Err(format!("collect info from rust file fails"));
    };

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
    let mut info_structs = Vec::new();
    let mut raw_structs = Vec::new();

    for rst in rinfo.structs {
        match resolve_one(rst, &module, target_machine.get_target_data()) {
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

    Ok(Analysis {
        info_structs,
        raw_structs,
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

fn resolve_one<'ctx>(
    rst: RStructType,
    module: &Module<'ctx>,
    target_data: TargetData,
) -> Result<AnalysisStruct<'ctx>, (String, Option<AnalysisStruct<'ctx>>)> {
    match rst {
        RStructType::RStruct(rst) => {
            let name = if let Some(name) = rst.name {
                name
            } else {
                return Err((
                    format!("resolve AnalysisStruct fail, anonymous struct unsupported"),
                    None,
                ));
            };
            let struct_type = if let Some(struct_type) =
                module.get_struct_type(&format!("{}", name))
            {
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
                    index += 1
                } else if rf.can_be_anytype() {
                    rf.is_padding = Some(true);
                }
            }

            if index != len {
                return Err((
                    format!("resolve AnalysisStruct fail, {} info not match", name),
                    Some(ast),
                ));
            }

            ast.is_raw = false;
            ast.is_enum = Some(false);
            ast.is_union = Some(false);
            ast.name = Some(name);

            return Ok(ast);
        }
        RStructType::REnum(rem) => {
            unimplemented!()
        }
        RStructType::RUnion(run) => {
            unimplemented!()
        }
    }
}
