#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use cxx::{CxxString, let_cxx_string};
#[deny(improper_ctypes, improper_ctypes_definitions)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::extra_unused_type_parameters,
    clippy::ptr_as_ptr,
    clippy::upper_case_acronyms,
    clippy::use_self,
)]
mod ffi {
    use super::RHello;
    #[doc(hidden)]
    unsafe impl ::cxx::private::RustType for RHello {}
    #[repr(C)]
    pub struct CHello {
        _private: ::cxx::private::Opaque,
    }
    unsafe impl ::cxx::ExternType for CHello {
        #[allow(unused_attributes)]
        #[doc(hidden)]
        type Id = (::cxx::C, ::cxx::H, ::cxx::e, ::cxx::l, ::cxx::l, ::cxx::o);
        type Kind = ::cxx::kind::Opaque;
    }
    pub fn getCHello(data: &::cxx::CxxString) -> ::cxx::UniquePtr<CHello> {
        extern "C" {
            #[link_name = "cxxbridge1$getCHello"]
            fn __getCHello(data: &::cxx::CxxString) -> *mut CHello;
        }
        unsafe { ::cxx::UniquePtr::from_raw(__getCHello(data)) }
    }
    impl CHello {
        pub fn hello(&self) {
            extern "C" {
                #[link_name = "cxxbridge1$CHello$hello"]
                fn __hello(_: &CHello);
            }
            unsafe { __hello(self) }
        }
    }
    unsafe impl ::cxx::private::UniquePtrTarget for CHello {
        fn __typename(
            f: &mut ::cxx::core::fmt::Formatter<'_>,
        ) -> ::cxx::core::fmt::Result {
            f.write_str("CHello")
        }
        fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
            extern "C" {
                #[link_name = "cxxbridge1$unique_ptr$CHello$null"]
                fn __null(
                    this: *mut ::cxx::core::mem::MaybeUninit<
                        *mut ::cxx::core::ffi::c_void,
                    >,
                );
            }
            let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
            unsafe { __null(&mut repr) }
            repr
        }
        unsafe fn __raw(
            raw: *mut Self,
        ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
            extern "C" {
                #[link_name = "cxxbridge1$unique_ptr$CHello$raw"]
                fn __raw(
                    this: *mut ::cxx::core::mem::MaybeUninit<
                        *mut ::cxx::core::ffi::c_void,
                    >,
                    raw: *mut ::cxx::core::ffi::c_void,
                );
            }
            let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
            __raw(&mut repr, raw.cast());
            repr
        }
        unsafe fn __get(
            repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
        ) -> *const Self {
            extern "C" {
                #[link_name = "cxxbridge1$unique_ptr$CHello$get"]
                fn __get(
                    this: *const ::cxx::core::mem::MaybeUninit<
                        *mut ::cxx::core::ffi::c_void,
                    >,
                ) -> *const ::cxx::core::ffi::c_void;
            }
            __get(&repr).cast()
        }
        unsafe fn __release(
            mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
        ) -> *mut Self {
            extern "C" {
                #[link_name = "cxxbridge1$unique_ptr$CHello$release"]
                fn __release(
                    this: *mut ::cxx::core::mem::MaybeUninit<
                        *mut ::cxx::core::ffi::c_void,
                    >,
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __release(&mut repr).cast()
        }
        unsafe fn __drop(
            mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$unique_ptr$CHello$drop"]
                fn __drop(
                    this: *mut ::cxx::core::mem::MaybeUninit<
                        *mut ::cxx::core::ffi::c_void,
                    >,
                );
            }
            __drop(&mut repr);
        }
    }
    #[doc(hidden)]
    const _: () = {
        let _ = {
            fn __AssertUnpin<
                T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
            >() {}
            __AssertUnpin::<RHello>
        };
        {
            #[doc(hidden)]
            fn __AssertSized<
                T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Sized,
            >() -> ::cxx::core::alloc::Layout {
                ::cxx::core::alloc::Layout::new::<T>()
            }
            #[doc(hidden)]
            #[export_name = "cxxbridge1$RHello$operator$sizeof"]
            extern "C" fn __sizeof_RHello() -> usize {
                __AssertSized::<RHello>().size()
            }
            #[doc(hidden)]
            #[export_name = "cxxbridge1$RHello$operator$alignof"]
            extern "C" fn __alignof_RHello() -> usize {
                __AssertSized::<RHello>().align()
            }
        }
        #[doc(hidden)]
        #[export_name = "cxxbridge1$getRHello"]
        unsafe extern "C" fn __getRHello(
            data: *mut ::cxx::private::RustString,
        ) -> *mut RHello {
            let __fn = "rcpp_cxx::ffi::getRHello";
            fn __getRHello(
                data: ::cxx::alloc::string::String,
            ) -> ::cxx::alloc::boxed::Box<RHello> {
                super::getRHello(data)
            }
            ::cxx::private::prevent_unwind(
                __fn,
                move || ::cxx::alloc::boxed::Box::into_raw(
                    __getRHello(::cxx::core::mem::take((*data).as_mut_string())),
                ),
            )
        }
        #[doc(hidden)]
        #[export_name = "cxxbridge1$RHello$hello"]
        unsafe extern "C" fn __RHello__hello(__self: &RHello) {
            let __fn = "rcpp_cxx::ffi::RHello::hello";
            fn __RHello__hello(__self: &RHello) {
                RHello::hello(__self)
            }
            ::cxx::private::prevent_unwind(__fn, move || __RHello__hello(__self))
        }
        let _: fn() = {
            trait __AmbiguousIfImpl<A> {
                fn infer() {}
            }
            impl<T> __AmbiguousIfImpl<()> for T
            where
                T: ?::cxx::core::marker::Sized,
            {}
            #[allow(dead_code)]
            struct __Invalid;
            impl<T> __AmbiguousIfImpl<__Invalid> for T
            where
                T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
            {}
            <CHello as __AmbiguousIfImpl<_>>::infer
        };
        #[doc(hidden)]
        unsafe impl ::cxx::private::ImplBox for RHello {}
        #[doc(hidden)]
        #[export_name = "cxxbridge1$box$RHello$alloc"]
        unsafe extern "C" fn RHello__box_alloc() -> *mut ::cxx::core::mem::MaybeUninit<
            RHello,
        > {
            ::cxx::alloc::boxed::Box::into_raw(
                ::cxx::alloc::boxed::Box::new(::cxx::core::mem::MaybeUninit::uninit()),
            )
        }
        #[doc(hidden)]
        #[export_name = "cxxbridge1$box$RHello$dealloc"]
        unsafe extern "C" fn RHello__box_dealloc(
            ptr: *mut ::cxx::core::mem::MaybeUninit<RHello>,
        ) {
            let _ = ::cxx::alloc::boxed::Box::from_raw(ptr);
        }
        #[doc(hidden)]
        #[export_name = "cxxbridge1$box$RHello$drop"]
        unsafe extern "C" fn RHello__box_drop(
            this: *mut ::cxx::alloc::boxed::Box<RHello>,
        ) {
            let __fn = "<rcpp_cxx::ffi::RHello as Drop>::drop";
            ::cxx::private::prevent_unwind(
                __fn,
                || ::cxx::core::ptr::drop_in_place(this),
            );
        }
    };
}
struct RHello {
    data: String,
}
impl RHello {
    pub fn new(data: String) -> RHello {
        RHello { data }
    }
    pub fn hello(&self) {
        {
            ::std::io::_print(
                format_args!("Hello from Rust! My data is {0}!\n", self.data),
            );
        };
    }
}
fn getRHello(data: String) -> Box<RHello> {
    Box::new(RHello::new(data))
}
fn main() {
    let mut cxx_stack_string = ::cxx::private::StackString::new();
    #[allow(unused_mut, unused_unsafe)]
    let mut data = match "CHelloName" {
        let_cxx_string => unsafe { cxx_stack_string.init(let_cxx_string) }
    };
    let h = ffi::getCHello(&data);
    h.hello();
}
