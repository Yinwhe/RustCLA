// // #[repr(C)]
// pub struct Foo{
//     a: i32,
// }

// #[repr(C, packed)]
// pub struct Bar {
//     something: i32,
//     subexpressions: Foo,
// }

// #[repr(C)]
// pub union T {
//     a: i32,
//     b: i64,
//     c: i64
// }

// #[repr(C)]
// pub enum E {
//     A(i32),
//     B(i64),
//     C(i64)
// }

// #[no_mangle]
// pub extern "C" fn root(b: Bar, t: T, e: E) {}

#[repr(C)]
pub enum A {
    A(i32),
    B(i64),
}

#[repr(C)]
pub struct T {
    a: i32,
    b: A
}