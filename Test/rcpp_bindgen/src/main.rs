#![allow(non_camel_case_types)]
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
    unsafe {
        let mut a = A::new(100);
        a.func();

        // Must manually release memory
        // h.destruct();
    }

}
