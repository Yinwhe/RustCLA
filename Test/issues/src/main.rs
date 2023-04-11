#![allow(non_camel_case_types)]
#![allow(unused)]
include!("../bindings.rs");

fn main() {
    let b = unsafe { B::new() };
    println!("{:?}", b);
}