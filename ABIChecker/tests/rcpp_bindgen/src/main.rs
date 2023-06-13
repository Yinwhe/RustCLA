#![allow(non_camel_case_types)]
#![allow(unused)]
include!("../bindings.rs");


// Or impl Drop for Hello
impl Drop for A {
    fn drop(&mut self) {
        unsafe {
            self.destruct();
        }
    }
}

fn main() {
    test_a();
}

#[allow(unused)]
fn test_a() {
    // Test A
    unsafe {
        let mut a = A::new(100);
        a.func();

        // Must manually release memory
        // a.destruct();
    }
}