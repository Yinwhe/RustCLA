use std::collections::{HashMap, HashSet};

use inkwell::{module::Module, values::FunctionValue};

pub struct IRInfo<'a> {
    r_modules: Vec<Module<'a>>,
    c_modules: Vec<Module<'a>>,
    r_functions: HashMap<String, FunctionValue<'a>>,
    c_functions: HashMap<String, FunctionValue<'a>>,
}

impl IRInfo<'_> {
    pub fn new<'a>(r: Vec<Module<'a>>, c: Vec<Module<'a>>) -> IRInfo<'a> {
        IRInfo {
            r_modules: r,
            c_modules: c,
            r_functions: HashMap::new(),
            c_functions: HashMap::new(),
        }
    }

    /// Locate FFI Functions.
    pub fn get_ffi(&mut self) -> Result<(), String> {
        self.prepare_functions()?;
        let rf_names = self.r_functions.keys().collect::<HashSet<&String>>();
        let cf_names = self.c_functions.keys().collect::<HashSet<&String>>();

        println!("{:?}", rf_names);
        println!("{:?}", cf_names);

        // let ffi_funcs = rf_names.intersection(&cf_names);

        unimplemented!()
    }

    fn prepare_functions(&mut self) -> Result<(), String> {
        // strip non-ascii characters from function names
        fn strip(name: &str) -> String {
            name.chars().filter(|c| c.is_ascii()).collect()
        }

        for rm in &self.r_modules {
            for f in rm.get_functions() {
                let name = strip(f.get_name().to_str().map_err(|e| e.to_string())?);
                if let Some(f) = self.r_functions.insert(name, f) {
                    return Err(format!("Duplicate function name: {:?}", f.get_name()));
                }
            }
        }

        for cm in &self.c_modules {
            for f in cm.get_functions() {
                let name = strip(f.get_name().to_str().map_err(|e| e.to_string())?);
                if let Some(f) = self.c_functions.insert(name, f) {
                    return Err(format!("Duplicate function name: {:?}", f.get_name()));
                }
            }
        }

        Ok(())
    }

    pub fn test(&mut self) {
        for rm in &self.r_modules {
            for f in rm.get_functions() {
                println!("{:?}", f);
            }
        }

        for cm in &self.c_modules {
            for f in cm.get_functions() {
                println!("{:?}", f);
            }
        }
    }
}
