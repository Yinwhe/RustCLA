// /// Gets the value of a `name`.
// /// For example, get_arg_flag_value("--manifest-path")
// /// Supports two styles: `--name value` or `--name=value`
// pub fn get_arg_flag_value(name: &str) -> Option<String> {
//     // Stop searching at `--`.
//     let mut args = std::env::args().take_while(|val| val != "--");

//     while let Some(arg) = args.next() {
//         if arg.starts_with(name) {
//             // Strip leading `name`.
//             let suffix = &arg[name.len()..];
//             if suffix.is_empty() {
//                 // This argument is exactly `name`; the next one is the value.
//                 return args.next();
//             } else if suffix.starts_with('=') {
//                 // This argument is `name=value`; get the value.
//                 // Strip leading `=`.
//                 return Some(suffix[1..].to_owned());
//             }
//         }
//     }

//     None
// }

/// Remove hash subfix from a name.
pub fn strip_hash(name: &str) -> String {
    match name.rfind('-') {
        Some(index) => name[..index].to_owned(),
        None => name.to_owned(),
    }
}