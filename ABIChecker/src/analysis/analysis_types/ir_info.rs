use inkwell::targets::TargetData;
/// IRInfo acts as a IR Information database.
use inkwell::{module::Module, values::FunctionValue};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

pub struct IRInfo<'a> {
    r_modules: Vec<Module<'a>>,
    c_modules: Vec<Module<'a>>,
    target: TargetData,
    rdef_functions: HashMap<String, FunctionValue<'a>>,
    rdec_functions: HashMap<String, FunctionValue<'a>>,
    cdef_functions: HashMap<String, FunctionValue<'a>>,
    cdec_functions: HashMap<String, FunctionValue<'a>>,
    ffi_functions: Vec<String>,
}

impl IRInfo<'_> {
    pub fn new<'a>(r: Vec<Module<'a>>, c: Vec<Module<'a>>, target: TargetData) -> IRInfo<'a> {
        IRInfo {
            r_modules: r,
            c_modules: c,
            target,
            rdef_functions: HashMap::new(),
            rdec_functions: HashMap::new(),
            cdef_functions: HashMap::new(),
            cdec_functions: HashMap::new(),
            ffi_functions: Vec::new(),
        }
    }

    pub fn get_ffi(&mut self) -> Result<(), String> {
        self.get_ffi_funcs()?;

        Ok(())
    }

    pub fn ffi_functions(&self) -> &Vec<String> {
        &self.ffi_functions
    }

    pub fn c_func(&self, name: &str) -> Option<&FunctionValue> {
        self.cdef_functions
            .get(name)
            .or(self.cdec_functions.get(name))
    }

    pub fn r_func(&self, name: &str) -> Option<&FunctionValue> {
        self.rdef_functions
            .get(name)
            .or(self.rdec_functions.get(name))
    }

    pub fn get_target_data(&self) -> &TargetData {
        &self.target
    }

    /// Locate FFI Functions.
    fn get_ffi_funcs(&mut self) -> Result<(), String> {
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

    fn get_ffi_structs(&mut self) {
        unimplemented!()
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
