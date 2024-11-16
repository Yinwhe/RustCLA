#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("../bindings.rs");

// fn main() {
//     unsafe {
//         let b = B::new();
//         println!("{:?}", b);
//     }
// }

pub fn OA_say_hello(this: *const A) {
    unsafe {
        let this = this as *mut A;
        (*this).say_hello();
    }
}

pub fn OB_say_hello(this: *const B) {
    unsafe {
        let this = this as *mut B;
        (*this).say_hello();
    }
}