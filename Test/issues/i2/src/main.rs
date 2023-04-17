#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("../bindings.rs");

fn main() {
    unsafe{
        // let foo = MakeFoo();
        let bar = MakeBar();

        // println!("{:?}", foo);
        // println!("{:?}", bar);
    }
}