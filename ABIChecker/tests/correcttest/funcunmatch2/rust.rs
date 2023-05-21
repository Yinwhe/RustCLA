#[repr(C)]
pub enum A {
    One(i32),
    Two(u64),
}

extern "C" {
    fn hello(a: A);
}

#[no_mangle]
pub extern "C" fn root(a: A) {
    unsafe {
        hello(a);
    }
}