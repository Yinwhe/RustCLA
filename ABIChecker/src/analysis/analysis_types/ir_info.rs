/// IRInfo acts as a IR Information database.
use inkwell::{module::Module, values::FunctionValue};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

pub struct IRInfo<'a> {
    r_modules: Vec<Module<'a>>,
    c_modules: Vec<Module<'a>>,
    r_functions: HashMap<String, FunctionValue<'a>>,
    c_functions: HashMap<String, FunctionValue<'a>>,
    ffi_functions: Vec<String>,
}

impl IRInfo<'_> {
    pub fn new<'a>(r: Vec<Module<'a>>, c: Vec<Module<'a>>) -> IRInfo<'a> {
        IRInfo {
            r_modules: r,
            c_modules: c,
            r_functions: HashMap::new(),
            c_functions: HashMap::new(),
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
        self.c_functions.get(name)
    }

    pub fn r_func(&self, name: &str) -> Option<&FunctionValue> {
        self.r_functions.get(name)
    }

    /// Locate FFI Functions.
    fn get_ffi_funcs(&mut self) -> Result<(), String> {
        self.prepare_functions()?;
        let rf_names = self.r_functions.keys().collect::<HashSet<&String>>();
        let cf_names = self.c_functions.keys().collect::<HashSet<&String>>();


        // println!("{:?}", rf_names);
        // println!("{:?}", cf_names);

        self.ffi_functions = rf_names
            .intersection(&cf_names)
            .map(|&name| name.to_owned())
            .collect::<Vec<String>>();

        // functions backup and mannually check
        let mut rf_debug = File::create("rf_debug.txt").unwrap();
        let s = serde_json::to_string(&rf_names).unwrap();
        writeln!(rf_debug, "{}", s).unwrap();

        let mut cf_debug = File::create("cf_debug.txt").unwrap();
        let s = serde_json::to_string(&cf_names).unwrap();
        writeln!(cf_debug, "{}", s).unwrap();

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
                if let Some(f) = self.r_functions.insert(name, f) {
                    return Err(format!("Duplicate function name: {:?}", f.get_name()));
                }
            }
        }

        for cm in &self.c_modules {
            for f in cm.get_functions() {
                let name = strip(f.get_name().to_str().map_err(|e| e.to_string())?).to_owned();
                if let Some(f) = self.c_functions.insert(name, f) {
                    return Err(format!("Duplicate function name: {:?}", f.get_name()));
                }
            }
        }

        Ok(())
    }
}