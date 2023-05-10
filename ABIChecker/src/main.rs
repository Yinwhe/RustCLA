use std::collections::HashMap;

use analysis_types::Analysis;
use ansi_term::Color;
use inkwell::context::Context;
use lazy_static::lazy_static;
use target::host_target;

mod analysis;
mod analysis_types;
mod collect;
mod target;

#[macro_use]
extern crate log;

pub const HOME: &str = env!("HOME");
pub const CLANG: &str = "clang";
pub const RUSTC: &str = "rustc";

fn main() {
    pretty_env_logger::init();

    let c_cx = Context::create();
    let r_cx = Context::create();

    let cinfo = collect::collect_info_from_cpp_file("/home/ubuntu/Desktop/RustCLA/ABIChecker/tests/test2/cpp.cc", None, None, &c_cx);
    let rinfo = collect::collect_info_from_rust_file("/home/ubuntu/Desktop/RustCLA/ABIChecker/tests/test2/rust.rs", &r_cx);

    debug!("cinfo {:#?}", cinfo);
    debug!("rinfo {:#?}", rinfo);

    let cinfo = match cinfo {
        Ok(cinfo) => cinfo,
        Err(msg) => {
            println!(
                "{} {}",
                Color::Red.paint("[Error]"),
                msg
            );
            return;
        }
    };

    let rinfo = match rinfo {
        Ok(rinfo) => rinfo,
        Err(msg) => {
            println!(
                "{} {}",
                Color::Red.paint("[Error]"),
                msg
            );
            return;
        }
    };

    out_crinfos(&cinfo, &rinfo);
    // let mut map = HashMap::new();
    // for st in cinfo.info_structs {
    //     map.insert(st.name.clone().unwrap(), st);
    // }

    // let target = host_target();

    // for st in rinfo.info_structs {
    //     let name = st.name.clone().unwrap();
    //     if let Some(c) = map.remove(&name) {
    //         let res = analysis::info_struct_analysis(st, c, target.get_target_data());
    //         println!("{:?}", res);
    //     }
    // }
}

fn out_crinfos(cinfo: &Analysis, rinfo: &Analysis) {
    // output cinfo
    print!("\n{}\n", Color::Green.paint("[Collect] collect info from C files"));
    
    let stnames: Vec<String> = cinfo
        .structs
        .iter()
        .filter_map(|st| st.name.clone())
        .collect();

    print!("structs: {:?}\n", stnames);

    let fnames: Vec<String> = cinfo
        .functions
        .iter()
        .map(|f| f.name.clone())
        .collect();

    print!("functions: {:?}\n", fnames);
    
    // output rinfo
    print!("\n{}\n",  Color::Green.paint("[Collect] collect info from Rust files"));

    let stnames: Vec<String> = rinfo
        .structs
        .iter()
        .filter_map(|st| st.name.clone())
        .collect();

    print!("structs: {:?}\n", stnames);

    let fnames: Vec<String> = rinfo
        .functions
        .iter()
        .map(|f| f.name.clone())
        .collect();

    print!("functions: {:?}\n", fnames);
}
