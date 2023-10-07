/// IRInfo acts as a IR Information database.
use inkwell::targets::{TargetData, TargetMachine};
use inkwell::{module::Module, values::FunctionValue};

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

use super::structure::ATypeLazyStruct;

pub struct IRInfo<'ctx> {
    target_machine: TargetMachine,
    r_modules: Vec<Module<'ctx>>,
    c_modules: Vec<Module<'ctx>>,
    rdef_functions: HashMap<String, FunctionValue<'ctx>>,
    rdec_functions: HashMap<String, FunctionValue<'ctx>>,
    cdef_functions: HashMap<String, FunctionValue<'ctx>>,
    cdec_functions: HashMap<String, FunctionValue<'ctx>>,
    ffi_functions: Vec<String>,
    ffi_structs: HashSet<(ATypeLazyStruct<'ctx>, ATypeLazyStruct<'ctx>)>,
}

impl<'ctx> IRInfo<'ctx> {
    pub fn new(r: Vec<Module<'ctx>>, c: Vec<Module<'ctx>>, target: TargetMachine) -> IRInfo<'ctx> {
        IRInfo {
            target_machine: target,
            r_modules: r,
            c_modules: c,
            rdef_functions: HashMap::new(),
            rdec_functions: HashMap::new(),
            cdef_functions: HashMap::new(),
            cdec_functions: HashMap::new(),
            ffi_functions: Vec::new(),
            ffi_structs: HashSet::new(),
        }
    }

    pub fn ffi_functions(&mut self) -> Vec<String> {
        self.ffi_functions.drain(0..).collect()
    }

    pub fn ffi_structs(&mut self) -> Vec<(ATypeLazyStruct<'ctx>, ATypeLazyStruct<'ctx>)> {
        self.ffi_structs.drain().collect()
    }

    pub fn get_c_func<'t>(&'t self, name: &str) -> Option<FunctionValue<'ctx>>
    where
        'ctx: 't,
    {
        self.cdef_functions
            .get(name)
            .or(self.cdec_functions.get(name))
            .cloned()
    }

    pub fn get_r_func<'t>(&'t self, name: &str) -> Option<FunctionValue<'ctx>>
    where
        'ctx: 't,
    {
        self.rdef_functions
            .get(name)
            .or(self.rdec_functions.get(name))
            .cloned()
    }

    // pub fn get_c_struct(&self, name: &str) -> Option<StructType> {
    //     for m in &self.c_modules {
    //         if let Some(s) = m.get_struct_type(name) {
    //             return Some(s);
    //         }
    //     }

    //     None
    // }

    // pub fn get_r_struct(&self, name: &str) -> Option<StructType> {
    //     for m in &self.r_modules {
    //         if let Some(s) = m.get_struct_type(name) {
    //             return Some(s);
    //         }
    //     }

    //     None
    // }

    pub fn get_target_data(&self) -> TargetData {
        self.target_machine.get_target_data()
    }

    pub fn add_ffi_structs(&mut self, ffis: Vec<(ATypeLazyStruct<'ctx>, ATypeLazyStruct<'ctx>)>) {
        self.ffi_structs.extend(ffis);
    }

    /// Locate FFI Functions.
    pub fn get_ffi_funcs(&mut self) -> Result<(), String> {
        self.prepare_functions()?;

        let r_use_c = self
            .rdec_functions
            .keys()
            .collect::<HashSet<&String>>()
            .intersection(&self.cdef_functions.keys().collect::<HashSet<&String>>())
            .map(|&name| name.to_owned())
            .collect::<Vec<String>>();

        let c_use_r = self
            .cdec_functions
            .keys()
            .collect::<HashSet<&String>>()
            .intersection(&self.rdef_functions.keys().collect::<HashSet<&String>>())
            .map(|&name| name.to_owned())
            .collect::<Vec<String>>();

        let ffi_funcs = Vec::from_iter(r_use_c.into_iter().chain(c_use_r.into_iter()));
        self.ffi_functions = ffi_funcs;

        // functions backup and mannually check
        // let mut rf_debug = File::create("rf_debug.txt").unwrap();
        // let s = serde_json::to_string(&rf_names).unwrap();
        // writeln!(rf_debug, "{}", s).unwrap();

        // let mut cf_debug = File::create("cf_debug.txt").unwrap();
        // let s = serde_json::to_string(&cf_names).unwrap();
        // writeln!(cf_debug, "{}", s).unwrap();

        Ok(())
    }

    fn prepare_functions(&mut self) -> Result<(), String> {
        // strip C++ prefix characters from function names, correctness not sure.
        fn strip(name: &str) -> &str {
            if name.starts_with("\u{1}") {
                &name[1..]
            } else {
                name
            }
        }

        for rm in &self.r_modules {
            for f in rm.get_functions() {
                let name = strip(f.get_name().to_str().map_err(|e| e.to_string())?).to_owned();
                let ret = if f.count_basic_blocks() == 0 {
                    self.rdec_functions.insert(name, f)
                } else {
                    self.rdef_functions.insert(name, f)
                };

                if let Some(f) = ret {
                    return Err(format!("Duplicate function name: {:?}", f.get_name()));
                }
            }
        }

        for cm in &self.c_modules {
            for f in cm.get_functions() {
                let name = strip(f.get_name().to_str().map_err(|e| e.to_string())?).to_owned();
                let ret = if f.count_basic_blocks() == 0 {
                    self.cdec_functions.insert(name, f)
                } else {
                    self.cdef_functions.insert(name, f)
                };

                if let Some(f) = ret {
                    return Err(format!("Duplicate function name: {:?}", f.get_name()));
                }
            }
        }

        Ok(())
    }
}
