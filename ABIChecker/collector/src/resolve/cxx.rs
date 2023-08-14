use std::collections::HashMap;

use bindgen_cxx_parser::{
    comp::{CompInfo, FieldMethods},
    context::{BindgenContext, TypeId},
    int::IntKind,
    item_kind::ItemKind,
    ty::{Type, TypeKind},
    BindgenError,
};
use log::{debug, warn};

use super::Info;
use crate::types::{
    CollectError, DetailIntType, Field, FunctionType, LLVMType, Struct, StructType, Union,
};

pub struct Resolver {
    ctx: BindgenContext,
    structs: HashMap<String, StructType>,
    funcs: HashMap<String, FunctionType>,
    // typedefs: HashMap<String, TypeDef>,
}

impl Resolver {
    pub fn new(file: &str, clang_args: &[&str]) -> Result<Self, CollectError> {
        let mut ctx = bindgen_parse_one(file, clang_args)?;

        // collect unresolved typerefs
        ctx.resolve_typerefs();

        Ok(Self {
            ctx,
            structs: HashMap::new(),
            funcs: HashMap::new(),
            // typedefs: HashMap::new(),
        })
    }

    pub fn resolve_bindgen_one(&mut self) -> Result<Info, CollectError> {
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
                    match ty.kind() {
                        TypeKind::Comp(st) => {
                            if let Some(name) = ty.name() {
                                if let Some(mut cstruct) = self.resolve_struct(st) {
                                    cstruct.set_name(&name);
                                    self.structs.insert(name.to_owned(), cstruct);
                                } else {
                                    warn!("Struct resolve fails: {:?}", ty)
                                }
                            }
                            // else is anonymous struct, shall not be exported
                        }
                        // TypeKind::Enum(e) => {
                        //     println!("Enum {:#?}", e);
                        // }
                        // TypeKind::Opaque => {
                        //     println!("Opaque");
                        // }
                        _ => continue,
                    }
                }
                ItemKind::Function(func) => {
                    if !top_ids.contains(&item.parent_id()) {
                        // Non-top items shall not be parsed here
                        continue;
                    }
                    let name = func.name().to_owned();
                    let mut cfunc = FunctionType {
                        name,
                        args: Vec::new(),
                        ret: None,
                    };
                    if let Some((args, ret)) = self.resolve_function(func.signature()) {
                        cfunc.args = args;
                        cfunc.ret = ret;
                        self.funcs.insert(cfunc.name.clone(), cfunc);
                    } else {
                        warn!("Function resolve fails: {:?}", func);
                    }
                }

                ItemKind::Var(_var) => continue, // skip global vars
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

    fn resolve_alias(&self, alias: &TypeId) -> Option<LLVMType> {
        let ty = self.ctx.resolve_type(alias.clone());
        match ty.kind() {
            TypeKind::Alias(alias) => self.resolve_alias(&alias),
            // TypeKind::ResolvedTypeRef(alias) => self.resolve_alias(&alias),
            _ => self.resolve_btype_to_ctype(&ty),
        }
    }

    fn resolve_btype_to_ctype(&self, ty: &Type) -> Option<LLVMType> {
        match ty.kind() {
            TypeKind::Array(_, _) => Some(LLVMType::ArrayType),
            TypeKind::BlockPointer(_) => Some(LLVMType::PointerType),
            TypeKind::Comp(st) => self
                .resolve_struct(st)
                .map(|inner| LLVMType::StructType(inner)),
            TypeKind::Complex(_) => Some(LLVMType::FloatType),
            TypeKind::Float(_) => Some(LLVMType::FloatType),
            TypeKind::Int(ik) => {
                let ik = match ik {
                    IntKind::Bool => DetailIntType::Bool,

                    IntKind::SChar => DetailIntType::SignedChar,
                    IntKind::WChar => DetailIntType::SignedChar,
                    IntKind::UChar => DetailIntType::UnsignedChar,
                    IntKind::Char { is_signed } => {
                        if *is_signed {
                            DetailIntType::SignedChar
                        } else {
                            DetailIntType::UnsignedChar
                        }
                    }

                    IntKind::Short => DetailIntType::SignedInt,
                    IntKind::I8 => DetailIntType::SignedInt,
                    IntKind::I16 => DetailIntType::SignedInt,
                    IntKind::I32 => DetailIntType::SignedInt,
                    IntKind::I64 => DetailIntType::SignedInt,
                    IntKind::I128 => DetailIntType::SignedInt,
                    IntKind::Int => DetailIntType::SignedInt,
                    IntKind::Long => DetailIntType::SignedInt,
                    IntKind::LongLong => DetailIntType::SignedInt,

                    IntKind::UShort => DetailIntType::UnsignedInt,
                    IntKind::U8 => DetailIntType::UnsignedInt,
                    IntKind::U16 => DetailIntType::UnsignedInt,
                    IntKind::U32 => DetailIntType::UnsignedInt,
                    IntKind::U64 => DetailIntType::UnsignedInt,
                    IntKind::U128 => DetailIntType::UnsignedInt,
                    IntKind::UInt => DetailIntType::UnsignedInt,
                    IntKind::ULong => DetailIntType::UnsignedInt,
                    IntKind::ULongLong => DetailIntType::UnsignedInt,

                    IntKind::Custom { is_signed, .. } => {
                        if *is_signed {
                            DetailIntType::SignedInt
                        } else {
                            DetailIntType::UnsignedInt
                        }
                    }
                };

                Some(LLVMType::IntType(ik))
            }
            TypeKind::NullPtr => Some(LLVMType::PointerType),
            TypeKind::Pointer(_) => Some(LLVMType::PointerType),
            TypeKind::Reference(_) => Some(LLVMType::PointerType),
            TypeKind::Vector(_, _) => Some(LLVMType::VecTorType),
            TypeKind::Alias(alias) => self.resolve_alias(alias),
            TypeKind::Void => Some(LLVMType::IntType(DetailIntType::Void)),
            // Currently we assume c enum as integer
            TypeKind::Enum(_) => Some(LLVMType::IntType(DetailIntType::CEnum)),
            TypeKind::ResolvedTypeRef(ty) => {
                let ty = self.ctx.resolve_type(ty.clone());
                self.resolve_btype_to_ctype(ty)
            }
            _ => None,
        }
    }

    fn resolve_struct(&self, st: &CompInfo) -> Option<StructType> {
        let mut fields = Vec::new();
        let layout = st.layout(&self.ctx);
        let _is_packed = st.is_packed(&self.ctx, layout.as_ref());
        let is_union = st.is_union();

        if is_union {
            Some(StructType::Union(Union { name: None }))
        } else {
            for f in st.raw_fields() {
                let name = f.name().map(|inner| inner.to_owned());
                let ty = self.ctx.resolve_type(f.ty());

                let ty = self.resolve_btype_to_ctype(ty)?;

                fields.push(Field { name, ty })
            }

            // println!("{:#?}", fields);
            Some(StructType::Struct(Struct { name: None, fields }))
        }
    }

    fn resolve_function(&self, fsig: TypeId) -> Option<(Vec<Field>, Option<Field>)> {
        let fsig = self.ctx.resolve_type(fsig);
        if let TypeKind::Function(fsig) = fsig.kind() {
            let mut args = Vec::new();
            // resolve arguments
            for (name, ty) in fsig.argument_types() {
                let ty = self.ctx.resolve_type(ty.clone());
                let ctype = self.resolve_btype_to_ctype(&ty)?;

                args.push(Field {
                    name: name.to_owned(),
                    ty: ctype,
                });
            }

            let ret = {
                let ty = self.ctx.resolve_type(fsig.return_type());
                if ty.is_void() {
                    None
                } else {
                    let ty = self.resolve_btype_to_ctype(ty)?;
                    Some(Field { name: None, ty: ty })
                }
            };

            Some((args, ret))
        } else {
            None
        }
    }
}

/// Use `bindgen` parser to parse cxx codes
fn bindgen_parse_one(file: &str, clang_args: &[&str]) -> Result<BindgenContext, BindgenError> {
    let builder = bindgen_cxx_parser::Builder::default()
        .clang_args(clang_args)
        .header(file);

    builder.generate()
}
