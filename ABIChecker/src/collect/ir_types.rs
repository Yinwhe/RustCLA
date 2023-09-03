use inkwell::{context::Context, module::Module};

pub struct IRInfo<'a> {
    pub cx: Context,
    pub c_modules: Vec<Module<'a>>,
    pub r_modules: Vec<Module<'a>>,
}

impl IRInfo<'_> {
    
}