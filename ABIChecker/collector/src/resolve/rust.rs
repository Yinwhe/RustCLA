use std::collections::HashMap;

use cbindgen_rust_parser::{
    Builder, Config, Error as BindgenError, Function, Item, ItemContainer, Parse, Path,
    PrimitiveType, Type,
};
use log::{debug, warn};

use crate::types::{
    CollectError, DetailIntType, Enum, Field, FunctionType, LLVMType, Struct, StructType, Union,
};

use super::Info;

pub struct Resolver {
    parse: Parse,
    structs: HashMap<String, StructType>,
    funcs: HashMap<String, FunctionType>,
    // typedefs: HashMap<String, TypeDef>,
}

impl Resolver {
    pub fn new(file: &str) -> Result<Self, CollectError> {
        let parse = cbindgen_parse_one(file)?;
        debug!("{:#?}", parse);
        Ok(Self {
            parse,
            structs: HashMap::new(),
            funcs: HashMap::new(),
            // typedefs: HashMap::new(),
        })
    }

    pub fn resolve_cbindgen_one(&mut self) -> Result<Info, CollectError> {
        // resolve structs
        for item in self.parse.structs.to_vec() {
            let name = item.name().to_owned();
            if let Some(st) = self.resolve_struct(item.container()) {
                // print!("{:#?}", st);
                self.structs.insert(name, st);
            } else {
                warn!("parse struct fails, struct: {:?}", item);
            }
        }

        for item in self.parse.enums.to_vec() {
            let name = item.name().to_owned();
            if let Some(st) = self.resolve_struct(item.container()) {
                // print!("{:#?}", st);
                self.structs.insert(name, st);
            } else {
                warn!("parse enum fails, enum: {:?}", item);
            }
        }

        for item in self.parse.unions.to_vec() {
            let name = item.name().to_owned();
            if let Some(st) = self.resolve_struct(item.container()) {
                // print!("{:#?}", st);
                self.structs.insert(name, st);
            } else {
                warn!("parse union fails, union: {:?}", item);
            }
        }

        // resolve funcs
        for func in self.parse.functions.to_vec() {
            if let Some(func) = self.resolve_function(func.clone()) {
                self.funcs.insert(func.name.clone(), func);
            } else {
                warn!("parse function fails, function: {:?}", func);
            }
        }

        // debug!("{:#?}", self.typedefs);
        debug!("{:#?}", self.structs);
        debug!("{:#?}", self.funcs);

        Ok(Info {
            structs: self.structs.drain().map(|(_k, v)| v).collect(),
            funcs: self.funcs.drain().map(|(_k, v)| v).collect(),
            // typedefs: self.typedefs.drain().map(|(_k, v)| v).collect(),
        })
    }

    fn resolve_btype_to_rtype(&self, ty: Type) -> Option<LLVMType> {
        match ty {
            Type::Ptr { .. } => Some(LLVMType::PointerType),
            Type::Array(..) => Some(LLVMType::ArrayType),
            Type::FuncPtr { .. } => Some(LLVMType::PointerType),
            Type::Path(path) => self.resolve_path(path.path()),
            Type::Primitive(prim) => match prim {
                PrimitiveType::Bool => Some(LLVMType::IntType(DetailIntType::Bool)),
                PrimitiveType::Char => Some(LLVMType::IntType(DetailIntType::SignedChar)),
                PrimitiveType::Char32 => Some(LLVMType::IntType(DetailIntType::SignedChar)),
                PrimitiveType::Double => Some(LLVMType::FloatType),
                PrimitiveType::Float => Some(LLVMType::FloatType),
                PrimitiveType::Integer { signed, .. } => {
                    if signed {
                        Some(LLVMType::IntType(DetailIntType::SignedInt))
                    } else {
                        Some(LLVMType::IntType(DetailIntType::UnsignedInt))
                    }
                }
                PrimitiveType::PtrDiffT => Some(LLVMType::PointerType),
                PrimitiveType::SChar => Some(LLVMType::IntType(DetailIntType::SignedChar)),
                PrimitiveType::UChar => Some(LLVMType::IntType(DetailIntType::UnsignedChar)),
                PrimitiveType::Void => Some(LLVMType::IntType(DetailIntType::Void)),
                _ => None,
            },
        }
    }

    fn resolve_path(&self, path: &Path) -> Option<LLVMType> {
        if let Some(mut item) = self.parse.structs.get_items(path) {
            assert!(item.len() == 1);
            let item = item.pop().expect("struct not found");
            self.resolve_struct(item).map(|st| LLVMType::StructType(st))
        } else if let Some(mut item) = self.parse.enums.get_items(path) {
            assert!(item.len() == 1);
            let item = item.pop().expect("enums not found");
            self.resolve_struct(item).map(|st| LLVMType::StructType(st))
        } else if let Some(mut item) = self.parse.unions.get_items(path) {
            assert!(item.len() == 1);
            let item = item.pop().expect("unions not found");
            self.resolve_struct(item).map(|st| LLVMType::StructType(st))
        } else if let Some(mut item) = self.parse.typedefs.get_items(path) {
            assert!(item.len() == 1);
            let item = item.pop().expect("typedefs not found");
            match item {
                ItemContainer::Typedef(ty) => self.resolve_btype_to_rtype(ty.aliased),
                _ => None,
            }
        } else {
            None
        }
    }

    fn resolve_struct(&self, item: ItemContainer) -> Option<StructType> {
        match item {
            ItemContainer::Struct(st) => {
                let name = st.name().to_owned();
                let mut fields = Vec::new();

                for f in st.fields {
                    let rtype = self.resolve_btype_to_rtype(f.ty)?;
                    fields.push(Field {
                        name: Some(f.name),
                        ty: rtype,
                    })
                }

                Some(StructType::Struct(Struct {
                    name: Some(name),
                    fields,
                }))
            }
            ItemContainer::Enum(em) => {
                let name = em.name().to_owned();
                Some(StructType::Enum(Enum { name: Some(name) }))
            }
            ItemContainer::Union(un) => {
                let name = un.name().to_owned();
                Some(StructType::Union(Union { name: Some(name) }))
            }
            _ => None,
        }
    }

    fn resolve_function(&self, func: Function) -> Option<FunctionType> {
        // print!("{:#?}", func);
        let name = func.path.name().to_owned();
        let mut args = Vec::new();

        for arg in func.args {
            args.push(Field {
                name: arg.name,
                ty: self.resolve_btype_to_rtype(arg.ty)?,
            })
        }

        let ret = {
            if let Some(ty) = self.resolve_btype_to_rtype(func.ret) {
                Some(Field { name: None, ty: ty })
            } else {
                None
            }
        };

        Some(FunctionType { name, args, ret })
    }
}

/// use `cbindgen` parser to parse rust codes
fn cbindgen_parse_one(file: &str) -> Result<Parse, BindgenError> {
    // let config = Config::from_s::from_file("config.toml").expect("config.toml not found");
    let mut config = Config::default();
    config.layout.packed = Some("__attribute__ ((packed))".to_string());
    config.layout.aligned_n = Some("__attribute__ ((aligned(n)))".to_string());

    Builder::new()
        .with_src(file)
        .with_std_types(false)
        .with_config(config)
        .parse()
}
