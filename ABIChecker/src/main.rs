use std::collections::HashMap;

use analysis_types::Analysis;
use inkwell::context::Context;
use target::host_target;

mod analysis;
mod analysis_types;
mod collect;
mod target;

#[macro_use]
extern crate log;


fn main() {
    pretty_env_logger::init();

    let c_cx = Context::create();
    let r_cx = Context::create();

    let cinfo = collect::collect_info_from_cpp_file("tests/test2/cpp.cc", None, None, &c_cx);
    let rinfo = collect::collect_info_from_rust_file("tests/test2/rust.rs", &r_cx);

    debug!("cinfo {:#?}", cinfo);
    debug!("rinfo {:#?}", rinfo);

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

fn out_crinfos(cinfo: Analysis, rinfo: Analysis) {}
