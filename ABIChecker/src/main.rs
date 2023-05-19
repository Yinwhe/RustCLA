use std::collections::{HashMap, HashSet};

use analysis_types::{Analysis, AnalysisFunction, AnalysisStruct};
use ansi_term::Color;
use clap::Parser;
use inkwell::context::Context;

use crate::{
    analysis::{function_analysis, info_struct_analysis},
    analysis_types::AnalysisResultType,
};

mod analysis;
mod analysis_types;
mod collect;
mod target;

#[macro_use]
extern crate log;

pub const HOME: &str = env!("HOME");
pub const CLANG: &str = "clang";
pub const RUSTC: &str = "rustc";


#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    rustfile: String,
    #[clap(short, long)]
    cppfile: String,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    let c_cx = Context::create();
    let r_cx = Context::create();
    
    let rinfo = collect::collect_info_from_rust_file(
        &args.rustfile,
        &r_cx,
    );

    let cinfo = collect::collect_info_from_cpp_file(
        &args.cppfile,
        None,
        None,
        &c_cx,
    );

    debug!("cinfo {:#?}", cinfo);
    debug!("rinfo {:#?}", rinfo);

    let cinfo = match cinfo {
        Ok(cinfo) => cinfo,
        Err(msg) => {
            println!("{} {}", Color::Red.paint("[Error]"), msg);
            return;
        }
    };

    let rinfo = match rinfo {
        Ok(rinfo) => rinfo,
        Err(msg) => {
            println!("{} {}", Color::Red.paint("[Error]"), msg);
            return;
        }
    };

    out_crinfos(&rinfo, &cinfo);

    analysis_struct(rinfo.structs, cinfo.structs);
    analysis_funcs(rinfo.functions, cinfo.functions);

    print!("{}", Color::Green.paint("[Analysis] analysis done"));
}

fn analysis_struct(rsts: Vec<AnalysisStruct>, csts: Vec<AnalysisStruct>) {
    let mut rst_map = HashMap::new();
    let mut cst_map = HashMap::new();

    for rst in rsts {
        rst_map.insert(rst.name.clone().unwrap(), rst);
    }

    for cst in csts {
        cst_map.insert(cst.name.clone().unwrap(), cst);
    }

    let rst_names: HashSet<&String> = rst_map.keys().collect();
    let cst_names: HashSet<&String> = cst_map.keys().collect();

    let todos: Vec<&String> = rst_names.intersection(&cst_names).cloned().collect();

    print!(
        "\n{} structs to be analyzed: {:?}\n",
        Color::Green.paint("[Analysis]"),
        todos
    );

    for todo in todos {
        print!(
            "{} start analyze {}\n",
            Color::Green.paint(format!("[Analysis Struct {}]", todo)),
            todo
        );

        let rst = rst_map.get(todo).expect("Should not happen");
        let cst = cst_map.get(todo).expect("Should not happen");

        let res = info_struct_analysis(rst.clone(), cst.clone(), false);

        debug!("res: {:#?}", res);

        if res.is_empty() {
            print!(
                "{} is ok\n",
                Color::Green.paint(format!("[Analysis Struct {}]", todo))
            );
        } else {
            for info in res.get_info() {
                let (paint, level) = if info.is_error() {
                    (Color::Red, "Error")
                } else {
                    (Color::Yellow, "Warn")
                };

                // title
                print!(
                    "{} {:?} : {}\n",
                    paint.paint(format!("[Analysis Struct {} {}]", todo, level)),
                    info.ty,
                    info.cont
                );
                match info.ty {
                    AnalysisResultType::AlignmentMismatch => {
                        print!(
                            "{} : {:?}\n",
                            paint.paint("Struct Alignment in rust side"),
                            rst.get_alignment()
                        );
                        print!(
                            "{} : {:?}\n",
                            paint.paint("Struct Alignment in cpp side"),
                            cst.get_alignment()
                        );
                    }
                    _ => {
                        print!(
                            "{} : {:?}\n",
                            paint.paint("Struct fields in rust side"),
                            rst.get_fields_from_range(info.rloc.0, info.rloc.1)
                        );
                        print!(
                            "{} : {:?}\n",
                            paint.paint("Struct fields in cpp side"),
                            cst.get_fields_from_range(info.rloc.0, info.rloc.1)
                        );
                    }
                }
            }
            // all infos
            print!(
                "{} : {:?}\n",
                Color::Blue.paint("All struct fields from rust side"),
                rst.fields
            );
            print!(
                "{} : {:?}\n",
                Color::Blue.paint("All struct fields from cpp side"),
                cst.fields
            );
        }
    }
}

fn analysis_funcs(rfuncs: Vec<AnalysisFunction>, cfuncs: Vec<AnalysisFunction>) {
    let mut rfunc_map = HashMap::new();
    let mut cfunc_map = HashMap::new();

    for rfunc in rfuncs {
        rfunc_map.insert(rfunc.name.clone(), rfunc);
    }

    for cfunc in cfuncs {
        cfunc_map.insert(cfunc.name.clone(), cfunc);
    }

    let rfunc_names: HashSet<&String> = rfunc_map.keys().collect();
    let cfunc_names: HashSet<&String> = cfunc_map.keys().collect();

    let todos: Vec<&String> = rfunc_names.intersection(&cfunc_names).cloned().collect();

    print!(
        "\n{} functions to be analyzed: {:?}\n",
        Color::Green.paint("[Analysis]"),
        todos
    );

    for todo in todos {
        print!(
            "{} start analyze {}\n",
            Color::Green.paint(format!("[Analysis Function {}]", todo)),
            todo
        );

        let rfunc = rfunc_map.get(todo).expect("Should not happen");
        let cfunc = cfunc_map.get(todo).expect("Should not happen");

        let res = function_analysis(rfunc.clone(), cfunc.clone());

        if res.is_empty() {
            print!(
                "{} is ok\n",
                Color::Green.paint(format!("[Analysis Function {}]", todo))
            );
        } else {
            for info in res.get_infos() {
                assert!(info.is_error());

                print!(
                    "{} {:?}\n",
                    Color::Red.paint(format!("[Analysis Function {} Error]", todo)),
                    info.ty
                );
            }
            print!("{} {:?}\n", Color::Blue.paint("Function sig in rust side"), rfunc);
            print!("{} {:?}\n", Color::Blue.paint("Function sig in cpp side"), cfunc)
        }
    }
}

fn out_crinfos(rinfo: &Analysis, cinfo: &Analysis) {
    // output cinfo
    print!(
        "\n{}\n",
        Color::Green.paint("[Collect] collect info from C files")
    );

    let stnames: Vec<String> = cinfo
        .structs
        .iter()
        .map(|st| st.name.clone().expect("Anonymous Struct"))
        .collect();

    print!("structs: {:?}\n", stnames);

    let fnames: Vec<String> = cinfo.functions.iter().map(|f| f.name.clone()).collect();

    print!("functions: {:?}\n", fnames);

    // output rinfo
    print!(
        "\n{}\n",
        Color::Green.paint("[Collect] collect info from Rust files")
    );

    let stnames: Vec<String> = rinfo
        .structs
        .iter()
        .map(|st| st.name.clone().expect("Anonymous Function"))
        .collect();

    print!("structs: {:?}\n", stnames);

    let fnames: Vec<String> = rinfo.functions.iter().map(|f| f.name.clone()).collect();

    print!("functions: {:?}\n", fnames);
}
