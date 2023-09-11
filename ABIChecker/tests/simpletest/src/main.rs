mod utils;

include!("bindings.rs");

fn main() {
    utils::test();
    unsafe {
        hello();
        override_();
        override_1(1);
    }
}

#[no_mangle]
pub extern "C" fn c_hello() {
    println!("Hello, C!");
}