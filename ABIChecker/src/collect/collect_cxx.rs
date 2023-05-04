use std::process::Command;

use ansi_term::Color;
use collect_cxx::{parse, CInfo, CStruct};
use inkwell::{
    context::Context,
    module::Module,
    targets::{Target, TargetData},
};

use crate::{analysis_types::*, target::host_target};

pub fn collect_info_from_cpp_file<'ctx>(
    file: &str,
    clang_target: Option<&str>,
    cpp_standard: Option<&str>,
) -> Result<Analysis<'ctx>, String> {
    // let cinfo = parse(file, clang_target, cpp_standard);
    // let cinfo = CInfo::empty();
    generate_bcfile(file)?;
    
    // parse bc code
    let cx = Context::create();
    let module = match Module::parse_bitcode_from_path("cpp.bc", &cx) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!("parse bitcode from path fails, due to {:?}", e));
        }
    };

    let target_machine = host_target();

    for cst in cinfo.structs {
        let res = resolve_one(cst, &module, target_machine.get_target_data());
        // print!("{:#?}", res);
    }

    unimplemented!()
}

fn generate_bcfile(file: &str) -> Result<(), String> {
    let res = Command::new("clang")
        .args(&["-c", "-emit-llvm", "-o", "cpp.bc", file])
        .output()
        .expect("failed to execute process");

    if res.status.success() {
        Ok(())
    } else {
        Err(format!("generate bc file fails, due to {:?}", res.stderr))
    }
}

fn resolve_one<'ctx>(
    cst: CStruct,
    module: &Module<'ctx>,
    target_data: TargetData,
) -> Result<AnalysisStruct<'ctx>, String> {
    let name = if let Some(name) = cst.name {
        name
    } else {
        return Err(format!(
            "resolve AnalysisStruct fail, anonymous struct found"
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

        let mut ast = AnalysisStruct::from_ctx_raw(struct_type, &target_data);

        ast.is_union = Some(true);
        ast.is_enum = Some(false);
        ast.name = Some(name);
        ast.is_raw = false;

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

        let mut ast = AnalysisStruct::from_ctx_raw(struct_type, &target_data);

        let mut index = 0;
        let len = cst.fields.len();
        for rf in &mut ast.fields {
            let f = &cst.fields[index];

            if f.ty.get_type_id() == rf.get_type_id() {
                rf.name = f.name.clone();
                rf.is_padding = Some(false);
                index += 1
            } else if rf.can_be_anytype() {
                rf.is_padding = Some(true);
            }
        }

        if index != len {
            return Err(format!("resolve AnalysisStruct fail, {} info not match", name));
        }

        ast.is_raw = false;
        ast.is_enum = Some(false);
        ast.is_union = Some(false);
        ast.name = Some(name);

        return Ok(ast);
    }
}
