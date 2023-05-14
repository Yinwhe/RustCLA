#[repr(C)]
pub struct T {
    a: ::std::os::raw::c_char,
}

#[repr(C)]
pub struct A {
    a: ::std::os::raw::c_int,
    ptr: ::std::os::raw::c_void,
    t: T,
}

// #[repr(C)]
// pub struct B {
//     opaque: [::std::os::raw::c_int; 2]
// }


// #[no_mangle]
// pub extern "C" fn root(a: A, b: B) {

// }