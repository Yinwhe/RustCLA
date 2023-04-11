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
    // test_b();
    // test_c();
    test_struct();
}

#[allow(unused)]
fn test_a() {
    // Test A
    unsafe {
        let mut a = A::new(100);
        a.func();

        // Must manually release memory
        // h.destruct();
    }
}

#[allow(unused)]
fn test_b() {
    // Test B
    unsafe {
        let mut b = B::new(100);
        b.func();
    }
}

#[allow(unused)]
fn test_c() {
    // Test C
    unsafe {
        let mut c = C::new(100);
        c.func();
    }
}


#[allow(unused)]
fn test_struct() {
    // Test struct
    unsafe {
        let mut s = getS();
        println!("{:?}", s);
    }
}

#[allow(unused)]
fn test_overload() {
    // Test overload
    unsafe {
        overload(1);
        overload1(1, 2);
    }
}