use std::collections::HashMap;

use cbindgen_rust_parser::{
    Builder, Config, Error as BindgenError, Item, ItemContainer, Parse, Path, PrimitiveType, Type, Function,
};

use crate::{CollectError, REnum, RField, RFunction, RStruct, RStructType, RType, TypeDef};

pub struct Resolver {
    parse: Parse,
    structs: Vec<RStructType>,
    funcs: Vec<RFunction>,
    typedefs: HashMap<String, TypeDef>,
}

impl Resolver {
    pub fn new(file: &str) -> Result<Self, CollectError> {
        let parse = cbindgen_parse_one(file)?;
        println!("{:#?}", parse);
        Ok(Self {
            parse,
            structs: Vec::new(),
            funcs: Vec::new(),
            typedefs: HashMap::new(),
        })
    }

    pub fn resolve_cbindgen_one(&mut self) {
        // resolve structs
        for item in self.parse.structs.to_vec() {
            if let Some(st) = self.resolve_struct(item.container()) {
                // print!("{:#?}", st);
                self.structs.push(st);
            } else {
                // TODO
            }
        }

        for item in self.parse.enums.to_vec() {
            if let Some(st) = self.resolve_struct(item.container()) {
                // print!("{:#?}", st);
                self.structs.push(st);
            } else {
                // TODO
            }
        }

        for item in self.parse.unions.to_vec() {
            if let Some(st) = self.resolve_struct(item.container()) {
                // print!("{:#?}", st);
                self.structs.push(st);
            } else {
                // TODO
            }
        }

        // resolve funcs
        for func in self.parse.functions.to_vec() {
            if let Some(func) = self.resolve_function(func) {
                self.funcs.push(func);
            } else {
                // TODO
            }
        }
    }

    fn resolve_btype_to_rtype(&self, ty: Type) -> Option<RType> {
        match ty {
            Type::Ptr { .. } => Some(RType::PointerType),
            Type::Array(..) => Some(RType::ArrayType),
            Type::FuncPtr { .. } => Some(RType::PointerType),
            Type::Path(path) => self.resolve_path(path.path()),
            Type::Primitive(prim) => match prim {
                PrimitiveType::Bool => Some(RType::IntType),
                PrimitiveType::Char => Some(RType::IntType),
                PrimitiveType::Char32 => Some(RType::IntType),
                PrimitiveType::Double => Some(RType::FloatType),
                PrimitiveType::Float => Some(RType::FloatType),
                PrimitiveType::Integer { .. } => Some(RType::IntType),
                PrimitiveType::PtrDiffT => Some(RType::PointerType),
                PrimitiveType::SChar => Some(RType::IntType),
                PrimitiveType::UChar => Some(RType::IntType),
                _ => None,
            },
        }
    }

    fn resolve_path(&self, path: &Path) -> Option<RType> {
        if let Some(mut item) = self.parse.structs.get_items(path) {
            assert!(item.len() == 1);
            let item = item.pop().expect("struct not found");
            self.resolve_struct(item).map(|st| RType::StructType(st))
        } else if let Some(mut item) = self.parse.enums.get_items(path) {
            assert!(item.len() == 1);
            let item = item.pop().expect("enums not found");
            self.resolve_struct(item).map(|st| RType::StructType(st))
        } else if let Some(mut item) = self.parse.unions.get_items(path) {
            assert!(item.len() == 1);
            let item = item.pop().expect("unions not found");
            self.resolve_struct(item).map(|st| RType::StructType(st))
        } else {
            None
        }
    }

    fn resolve_struct(&self, item: ItemContainer) -> Option<RStructType> {
        match item {
            ItemContainer::Struct(st) => {
                let name = st.name().to_owned();
                let mut fields = Vec::new();

                for f in st.fields {
                    let rtype = self.resolve_btype_to_rtype(f.ty)?;
                    fields.push(RField {
                        name: Some(f.name),
                        ty: rtype,
                    })
                }

                Some(RStructType::RStruct(RStruct {
                    name: Some(name),
                    fields,
                }))
            }
            ItemContainer::Enum(em) => {
                let name = em.name().to_owned();
                Some(RStructType::REnum(REnum { name: Some(name) }))
            }
            ItemContainer::Union(un) => {
                let name = un.name().to_owned();
                Some(RStructType::REnum(REnum { name: Some(name) }))
            }
            _ => None,
        }
    }

    fn resolve_function(&self, func: Function) -> Option<RFunction> {
        let name = func.path.name().to_owned();
        let mut args = Vec::new();

        for arg in func.args {
            args.push(RField {
                name: arg.name,
                ty: self.resolve_btype_to_rtype(arg.ty)?,
            })
        }

        let ret = {
            if let Some(ty) = self.resolve_btype_to_rtype(func.ret) {
                Some(RField {name: None, ty: ty})
            } else {
                None
            }
        };

        Some(RFunction { name, args, ret })
    }
}

/// use `cbindgen` parser to parse rust codes
fn cbindgen_parse_one(file: &str) -> Result<Parse, BindgenError> {
    let config = Config::from_file("config.toml").expect("config.toml not found");
    Builder::new()
        .with_src(file)
        .with_std_types(false)
        .with_config(config)
        .parse()
}
