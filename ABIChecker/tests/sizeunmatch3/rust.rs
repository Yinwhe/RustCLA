// #[repr(C)]
// pub struct A {
//     pub ptr: *mut ::std::os::raw::c_void,
//     pub val: [::std::os::raw::c_uint; 1],
//     pub t: [T; 2]
// }

// extern "C" {
//     #[link_name = "\u{1}_Z5hello1A"]
//     pub fn hello(a: A);
// }

// #[repr(C)]
// pub union T {
//     a: i64,
//     b: [i32; 3]
// }

#[repr(C)]
pub enum A {
    One(i32),
    Two(u64),
}

#[no_mangle]
pub extern "C" fn root(a: A) {
}