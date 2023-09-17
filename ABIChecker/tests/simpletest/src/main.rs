mod utils;

include!("bindings.rs");

fn main() {
    utils::test();
    unsafe {
        hello();
        override_();
        let t = T {
            a: 1,
            b: 2,
            c: [3; 10],
        };
        override_1(t);
    }
}

#[no_mangle]
pub extern "C" fn c_hello() {
    println!("Hello, C!");
}