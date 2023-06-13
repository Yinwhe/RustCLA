// rust code
#[repr(C)]
pub struct A {
    pub ptr: *mut ::std::os::raw::c_void,
    pub m_buf: ::std::os::raw::c_int,
}
extern "C" {
    #[link_name = "\u{1}_ZN1A5helloEv"]
    pub fn A_hello(this: *mut A);
}
impl A {
    pub unsafe fn hello(&mut self) {
        A_hello(self)
    }
}
extern "C" {
    pub fn hellohello(a: A);
}
