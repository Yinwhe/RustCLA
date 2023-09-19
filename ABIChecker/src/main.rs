use std::time::Instant;

use clap::Parser;
use target::host_target;

mod analysis;
mod collect;
mod target;
mod utils;

#[macro_use]
extern crate log;

pub const CARGO: &str = "cargo";
pub const RUSTC: &str = "rustc";
pub const CLANG: &str = "clang";

// #[derive(Parser)]
// pub struct Args {
//     #[clap(short, long)]
//     rustfile: String,
//     #[clap(short, long)]
//     cppfile: String,
//     #[clap(long, default_value = "false")]
//     no_pad_info: bool,
//     // #[clap(long)]
//     cpp_include: Vec<String>,
// }

// Currently we are still testing ir collection
fn main() {
    pretty_env_logger::init();

    let start = Instant::now();

    // Collect IR
    utils::info_prompt("Collect", "start collecting ir");
    let (bitcode_path, targets) = match collect::collect_ir() {
        Ok((bitcode_path, targets)) => (bitcode_path, targets),
        Err(e) => {
            utils::error_prompt(
                "Collect IR Error",
                &format!("collect ir failed, due to: {}", e),
            );
            utils::error_prompt(
                "Exit",
                &format!("time spent: {:?}", start.elapsed()),
            );

            return;
        }
    };

    // Resolve and analysis IR
    let target_machin = host_target();

    utils::info_prompt("Analysis", "start analyzing ir");
    if let Err(e) = analysis::analysis_ir(bitcode_path, targets, target_machin) {
        utils::error_prompt(
            "Resolve IR Error",
            &format!("resolve ir failed, due to: {}", e),
        );
        utils::error_prompt(
            "Exit",
            &format!("time spent: {:?}", start.elapsed()),
        );

        return;
    }

    utils::info_prompt(
        "Exit",
        &format!("time spent: {:?}", start.elapsed()),
    );
}

// fn main() {
//     let start = Instant::now();
//     pretty_env_logger::init();
//     let args = Args::parse();

//     let c_cx = Context::create();
//     let r_cx = Context::create();

//     let start_collect = start.elapsed();
//     let rinfo = collect::collect_info_from_rust_file(
//         &args.rustfile,
//         &r_cx,
//         &args,
//     );

//     let cinfo = collect::collect_info_from_cpp_file(
//         &args.cppfile,
//         &c_cx,
//         &args,
//     );
//     let end_collect = start.elapsed();

//     debug!("cinfo {:#?}", cinfo);
//     debug!("rinfo {:#?}", rinfo);

//     let cinfo = match cinfo {
//         Ok(cinfo) => cinfo,
//         Err(msg) => {
//             println!("{} {}", Color::Red.paint("[Error]"), msg);
//             return;
//         }
//     };

//     let rinfo = match rinfo {
//         Ok(rinfo) => rinfo,
//         Err(msg) => {
//             println!("{} {}", Color::Red.paint("[Error]"), msg);
//             return;
//         }
//     };

//     out_crinfos(&rinfo, &cinfo);

//     let start_analysis = start.elapsed();
//     let s = analysis_struct(rinfo.structs, cinfo.structs);
//     let f = analysis_funcs(rinfo.functions, cinfo.functions);
//     let end_analysis = start.elapsed();

//     print!("{}", Color::Green.paint("[Analysis] analysis done\n"));
//     print!("Analyze {} structs and {} functions\n", s, f);
//     print!("Time spent on collect info: {:?}\n", end_collect - start_collect);
//     print!("Time spent on analysis: {:?}\n", end_analysis - start_analysis);
// }

// fn analysis_struct(rsts: Vec<AnalysisStruct>, csts: Vec<AnalysisStruct>) -> u32{
//     let mut rst_map = HashMap::new();
//     let mut cst_map = HashMap::new();

//     for rst in rsts {
//         rst_map.insert(rst.name.clone().unwrap(), rst);
//     }

//     for cst in csts {
//         cst_map.insert(cst.name.clone().unwrap(), cst);
//     }

//     let rst_names: HashSet<&String> = rst_map.keys().collect();
//     let cst_names: HashSet<&String> = cst_map.keys().collect();

//     let todos: Vec<&String> = rst_names.intersection(&cst_names).cloned().collect();
//     let len = todos.len();

//     print!(
//         "\n{} structs to be analyzed: {:?}\n",
//         Color::Green.paint("[Analysis]"),
//         todos
//     );

//     for todo in todos {
//         print!(
//             "{} start analyze {}\n",
//             Color::Green.paint(format!("[Analysis Struct {}]", todo)),
//             todo
//         );

//         let rst = rst_map.get(todo).expect("Should not happen");
//         let cst = cst_map.get(todo).expect("Should not happen");

//         let res = info_struct_analysis(rst.clone(), cst.clone(), false);

//         debug!("res: {:#?}", res);

//         if res.is_empty() {
//             print!(
//                 "{} is ok\n",
//                 Color::Green.paint(format!("[Analysis Struct {}]", todo))
//             );
//         } else {
//             for info in res.get_info() {
//                 let (paint, level) = if info.is_error() {
//                     (Color::Red, "Error")
//                 } else {
//                     (Color::Yellow, "Warn")
//                 };

//                 // title
//                 print!(
//                     "{} {:?} : {}\n",
//                     paint.paint(format!("[Analysis Struct {} {}]", todo, level)),
//                     info.ty,
//                     info.cont
//                 );
//                 match info.ty {
//                     AnalysisResultType::AlignmentMismatch => {
//                         print!(
//                             "{} : {:?}\n",
//                             paint.paint("Struct Alignment in rust side"),
//                             rst.get_alignment()
//                         );
//                         print!(
//                             "{} : {:?}\n",
//                             paint.paint("Struct Alignment in cpp side"),
//                             cst.get_alignment()
//                         );
//                     }
//                     _ => {
//                         print!(
//                             "{} : {:?}\n",
//                             paint.paint("Struct fields in rust side"),
//                             rst.get_fields_from_range(info.rloc.0, info.rloc.1)
//                         );
//                         print!(
//                             "{} : {:?}\n",
//                             paint.paint("Struct fields in cpp side"),
//                             cst.get_fields_from_range(info.rloc.0, info.rloc.1)
//                         );
//                     }
//                 }
//             }
//             // all infos
//             print!(
//                 "{} : {:?}\n",
//                 Color::Blue.paint("All struct fields from rust side"),
//                 rst.fields
//             );
//             print!(
//                 "{} : {:?}\n",
//                 Color::Blue.paint("All struct fields from cpp side"),
//                 cst.fields
//             );
//         }
//     }

//     len as u32
// }

// fn analysis_funcs(rfuncs: Vec<AnalysisFunction>, cfuncs: Vec<AnalysisFunction>) -> u32 {
//     let mut rfunc_map = HashMap::new();
//     let mut cfunc_map = HashMap::new();

//     for rfunc in rfuncs {
//         rfunc_map.insert(rfunc.name.clone(), rfunc);
//     }

//     for cfunc in cfuncs {
//         cfunc_map.insert(cfunc.name.clone(), cfunc);
//     }

//     let rfunc_names: HashSet<&String> = rfunc_map.keys().collect();
//     let cfunc_names: HashSet<&String> = cfunc_map.keys().collect();

//     let todos: Vec<&String> = rfunc_names.intersection(&cfunc_names).cloned().collect();
//     let len = todos.len();

//     print!(
//         "\n{} functions to be analyzed: {:?}\n",
//         Color::Green.paint("[Analysis]"),
//         todos
//     );

//     for todo in todos {
//         print!(
//             "{} start analyze {}\n",
//             Color::Green.paint(format!("[Analysis Function {}]", todo)),
//             todo
//         );

//         let rfunc = rfunc_map.get(todo).expect("Should not happen");
//         let cfunc = cfunc_map.get(todo).expect("Should not happen");

//         let res = function_analysis(rfunc.clone(), cfunc.clone());

//         if res.is_empty() {
//             print!(
//                 "{} is ok\n",
//                 Color::Green.paint(format!("[Analysis Function {}]", todo))
//             );
//         } else {
//             for info in res.get_infos() {
//                 assert!(info.is_error());

//                 print!(
//                     "{} {:?}\n",
//                     Color::Red.paint(format!("[Analysis Function {} Error]", todo)),
//                     info.ty
//                 );
//             }
//             print!("{} {:?}\n", Color::Blue.paint("Function sig in rust side"), rfunc);
//             print!("{} {:?}\n", Color::Blue.paint("Function sig in cpp side"), cfunc)
//         }
//     }

//     len as u32
// }

// fn out_crinfos(rinfo: &Analysis, cinfo: &Analysis) {
//     // output cinfo
//     print!(
//         "\n{}\n",
//         Color::Green.paint("[Collect] collect info from C files")
//     );

//     let stnames: Vec<String> = cinfo
//         .structs
//         .iter()
//         .map(|st| st.name.clone().expect("Anonymous Struct"))
//         .collect();

//     print!("structs: {:?}\n", stnames);

//     let fnames: Vec<String> = cinfo.functions.iter().map(|f| f.name.clone()).collect();

//     print!("functions: {:?}\n", fnames);

//     // output rinfo
//     print!(
//         "\n{}\n",
//         Color::Green.paint("[Collect] collect info from Rust files")
//     );

//     let stnames: Vec<String> = rinfo
//         .structs
//         .iter()
//         .map(|st| st.name.clone().expect("Anonymous Function"))
//         .collect();

//     print!("structs: {:?}\n", stnames);

//     let fnames: Vec<String> = rinfo.functions.iter().map(|f| f.name.clone()).collect();

//     print!("functions: {:?}\n", fnames);
// }
