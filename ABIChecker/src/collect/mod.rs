// mod collect_cxx;
// mod collect_rust;
mod collect_ir;
mod ir_types;
mod helper;

// pub use self::collect_cxx::collect_info_from_cpp_file;
// pub use self::collect_rust::collect_info_from_rust_file;

pub use collect_ir::collect_ir;