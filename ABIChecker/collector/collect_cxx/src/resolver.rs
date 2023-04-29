use std::collections::HashMap;

use bindgen_cxx_parser::{
    comp::{CompInfo, FieldMethods},
    context::{BindgenContext, TypeId},
    item_kind::ItemKind,
    ty::{Type, TypeKind},
    BindgenError,
};

use crate::{
    types::{CInfo, CollectError},
    CField, CFunction, CStruct, CType, TypeDef,
};

pub struct Resolver {
    ctx: BindgenContext,
    structs: HashMap<String, CStruct>,
    funcs: HashMap<String, CFunction>,
    typedefs: HashMap<String, TypeDef>,
}

impl Resolver {
    pub fn new(file: &str, clang_args: &[&str]) -> Result<Self, CollectError> {
        let mut ctx = bindgen_parse_one(file, clang_args)?;
        // collect unresolved typerefs
        ctx.resolve_typerefs();
        // println!("{:#?}", ctx);
        Ok(Self {
            ctx,
            structs: HashMap::new(),
            funcs: HashMap::new(),
            typedefs: HashMap::new(),
        })
    }

    pub fn resolve_bindgen_one(&mut self) -> Result<CInfo, CollectError> {
        let items = self.ctx.items();
        let mut top_ids = Vec::new();
        for (item_id, item) in items {
            match item.kind() {
                ItemKind::Module(_module) => {
                    top_ids.push(item_id);
                }
                ItemKind::Type(ty) => {
                    if !top_ids.contains(&item.parent_id()) {
                        // Non-top items shall not be parsed here
                        continue;
                    }
                    // println!("{:?} {:#?}", _item_id, ty);
                    match ty.kind() {
                        TypeKind::Alias(alias) => {
                            if let Some(ctype) = self.resolve_alias(alias) {
                                let name = ty.name().expect("Alais get name fails").to_string();
                                // println!("Alias: {} {:?}", name, ctype);
                                self.typedefs.insert(
                                    name.clone(),
                                    TypeDef {
                                        name,
                                        aliased: ctype,
                                    },
                                );
                            } else {
                                // TODO
                            }
                        }
                        TypeKind::Comp(st) => {
                            if let Some(name) = ty.name() {
                                if let Some(mut cstruct) = self.resolve_struct(st) {
                                    cstruct.set_name(&name);
                                    self.structs.insert(name.to_owned(), cstruct);
                                } else {
                                    // TODO
                                }
                            }
                            // else is anonymous struct, shall not be exported
                        }
                        // TypeKind::Enum(e) => {
                        // }
                        TypeKind::Opaque => {}
                        TypeKind::Function(_f) => {
                            unreachable!("what is this?")
                        }
                        _ => continue,
                    }
                }
                ItemKind::Function(func) => {
                    // println!("Function: {:#?}", func);
                }
                ItemKind::Var(_var) => continue, // skip global vars
            }
        }

        println!("{:#?}", self.typedefs);
        println!("{:#?}", self.structs);
        println!("{:#?}", self.funcs);
        unimplemented!()
    }

    fn resolve_alias(&self, alias: &TypeId) -> Option<CType> {
        let ty = self.ctx.resolve_type(alias.clone());
        match ty.kind() {
            TypeKind::Alias(alias) => self.resolve_alias(&alias),
            // TypeKind::ResolvedTypeRef(alias) => self.resolve_alias(&alias),
            _ => self.resolve_btype_to_ctype(&ty),
        }
    }

    fn resolve_btype_to_ctype(&self, ty: &Type) -> Option<CType> {
        match ty.kind() {
            TypeKind::Array(_, _) => Some(CType::ArrayType),
            TypeKind::BlockPointer(_) => Some(CType::PointerType),
            TypeKind::Comp(st) => self
                .resolve_struct(st)
                .map(|inner| CType::StructType(inner)),
            TypeKind::Complex(_) => Some(CType::FloatType),
            TypeKind::Float(_) => Some(CType::FloatType),
            TypeKind::Int(_) => Some(CType::IntType),
            TypeKind::NullPtr => Some(CType::PointerType),
            TypeKind::Pointer(_) => Some(CType::PointerType),
            TypeKind::Reference(_) => Some(CType::PointerType),
            TypeKind::Vector(_, _) => Some(CType::VecTorType),
            TypeKind::Alias(alias) => self.resolve_alias(alias),
            TypeKind::ResolvedTypeRef(ty) => {
                let ty = self.ctx.resolve_type(ty.clone());
                self.resolve_btype_to_ctype(ty)
            }
            _ => None,
        }
    }

    fn resolve_struct(&self, st: &CompInfo) -> Option<CStruct> {
        let mut fields = Vec::new();
        let layout = st.layout(&self.ctx);
        let is_packed = st.is_packed(&self.ctx, layout.as_ref());
        let is_union = st.is_union();

        for f in st.raw_fields() {
            let name = f.name().map(|inner| inner.to_owned());
            let ty = self.ctx.resolve_type(f.ty());

            let ty = self.resolve_btype_to_ctype(ty)?;

            // println!("{:?}, {:?}", name, ty);

            fields.push(CField { name, ty })
        }

        // println!("{:#?}", fields);
        Some(CStruct {
            name: None,
            fields,
            is_packed,
            is_union,
        })
    }
}

#[allow(unused)]
pub fn parse(files: &[&str], clang_args: &[&str]) -> Result<CInfo, CollectError> {
    unimplemented!()
}

/// Use `bindgen` parser to parse cxx codes
fn bindgen_parse_one(file: &str, clang_args: &[&str]) -> Result<BindgenContext, BindgenError> {
    let builder = bindgen_cxx_parser::Builder::default()
        .clang_args(clang_args)
        .header(file);

    builder.generate()
}
