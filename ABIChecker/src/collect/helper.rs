use std::{collections::HashMap};
use cpp_demangle::Symbol;
use log::warn;

pub fn collect_mangles(mangled: Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for n in mangled {
        if let Some(index) = n.find('_') {
            if let Ok(sym) = Symbol::new(&n[index..]) {
                let ori = sym.to_string().trim_end_matches("()").to_owned();
                map.insert(ori, n);
                // println!("{:?} -> {:?}", n, ori);
            } else {
                warn!("mangled name {:?} is not a valid mangled name", n);
                map.insert(n.clone(), n);
            }
        } else {
            warn!("mangled name {:?} is not a valid mangled name", n);
            map.insert(n.clone(), n);
        }
    }

    map
}
