use ffi;

pub fn get_hello() -> ffi::Hello {
    unsafe { ffi::Hello::new() }
}

#[test]
fn test() {
    let mut h = get_hello();
    unsafe {
        h.sayHello();
    }
}