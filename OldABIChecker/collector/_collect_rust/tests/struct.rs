#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iJIT_Method_Load_V2 {
    pub method_id: ::std::os::raw::c_uint,
    pub method_name: *mut ::std::os::raw::c_char,
    pub method_load_address: *mut ::std::os::raw::c_void,
    pub method_size: ::std::os::raw::c_uint,
    pub line_number_size: ::std::os::raw::c_uint,
    pub line_number_table: pLineNumberInfo,
    pub class_file_name: *mut ::std::os::raw::c_char,
    pub source_file_name: *mut ::std::os::raw::c_char,
    pub module_name: *mut ::std::os::raw::c_char,
}

pub type pLineNumberInfo = *mut _LineNumberInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _LineNumberInfo {
    #[doc = "<\\brief Offset from the begining of the code region."]
    pub Offset: ::std::os::raw::c_uint,
    #[doc = "<\\brief Matching source line number offset (from beginning of source file)."]
    pub LineNumber: ::std::os::raw::c_uint,
}