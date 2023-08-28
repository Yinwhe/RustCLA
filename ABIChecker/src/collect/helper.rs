use cpp_demangle::Symbol;
use log::warn;
use std::collections::HashMap;

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

/// Gets the value of a `name`.
/// For example, get_arg_flag_value("--manifest-path")
/// Supports two styles: `--name value` or `--name=value`
pub fn get_arg_flag_value(name: &str) -> Option<String> {
    // Stop searching at `--`.
    let mut args = std::env::args().take_while(|val| val != "--");

    while let Some(arg) = args.next() {
        if arg.starts_with(name) {
            // Strip leading `name`.
            let suffix = &arg[name.len()..];
            if suffix.is_empty() {
                // This argument is exactly `name`; the next one is the value.
                return args.next();
            } else if suffix.starts_with('=') {
                // This argument is `name=value`; get the value.
                // Strip leading `=`.
                return Some(suffix[1..].to_owned());
            }
        }
    }

    None
}