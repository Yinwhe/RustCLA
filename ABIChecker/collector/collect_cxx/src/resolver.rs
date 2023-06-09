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

use crate::{
    types::{CInfo, CollectError},
    CField, CFunction, CIntType, CStruct, CType,
};

pub struct Resolver {
    ctx: BindgenContext,
    structs: HashMap<String, CStruct>,
    funcs: HashMap<String, CFunction>,
    // typedefs: HashMap<String, TypeDef>,
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
            // typedefs: HashMap::new(),
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
                    // println!("Function 2 {:#?}", func);
                    if !top_ids.contains(&item.parent_id()) {
                        // Non-top items shall not be parsed here
                        continue;
                    }
                    let name = func.name().to_owned();
                    let mut cfunc = CFunction {
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

        Ok(CInfo {
            structs: self.structs.drain().map(|(_k, v)| v).collect(),
            funcs: self.funcs.drain().map(|(_k, v)| v).collect(),
            // typedefs: self.typedefs.drain().map(|(_k, v)| v).collect(),
        })
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
            TypeKind::Int(ik) => {
                let ik = match ik {
                    IntKind::Bool => CIntType::Bool,

                    IntKind::SChar => CIntType::SignedChar,
                    IntKind::WChar => CIntType::SignedChar,
                    IntKind::UChar => CIntType::UnsignedChar,
                    IntKind::Char { is_signed } => {
                        if *is_signed {
                            CIntType::SignedChar
                        } else {
                            CIntType::UnsignedChar
                        }
                    }

                    IntKind::Short => CIntType::SignedInt,
                    IntKind::I8 => CIntType::SignedInt,
                    IntKind::I16 => CIntType::SignedInt,
                    IntKind::I32 => CIntType::SignedInt,
                    IntKind::I64 => CIntType::SignedInt,
                    IntKind::I128 => CIntType::SignedInt,
                    IntKind::Int => CIntType::SignedInt,
                    IntKind::Long => CIntType::SignedInt,
                    IntKind::LongLong => CIntType::SignedInt,

                    IntKind::UShort => CIntType::UnsignedInt,
                    IntKind::U8 => CIntType::UnsignedInt,
                    IntKind::U16 => CIntType::UnsignedInt,
                    IntKind::U32 => CIntType::UnsignedInt,
                    IntKind::U64 => CIntType::UnsignedInt,
                    IntKind::U128 => CIntType::UnsignedInt,
                    IntKind::UInt => CIntType::UnsignedInt,
                    IntKind::ULong => CIntType::UnsignedInt,
                    IntKind::ULongLong => CIntType::UnsignedInt,
                    
                    IntKind::Custom {  is_signed, .. } => {
                        if *is_signed {
                            CIntType::SignedInt
                        } else {
                            CIntType::UnsignedInt
                        }
                    }
                };

                Some(CType::IntType(ik))
            }
            TypeKind::NullPtr => Some(CType::PointerType),
            TypeKind::Pointer(_) => Some(CType::PointerType),
            TypeKind::Reference(_) => Some(CType::PointerType),
            TypeKind::Vector(_, _) => Some(CType::VecTorType),
            TypeKind::Alias(alias) => self.resolve_alias(alias),
            TypeKind::Void => Some(CType::IntType(CIntType::CVoid)),
            // Currently we assume c enum as integer
            TypeKind::Enum(_) => Some(CType::IntType(CIntType::CEnum)),
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

    fn resolve_function(&self, fsig: TypeId) -> Option<(Vec<CField>, Option<CField>)> {
        let fsig = self.ctx.resolve_type(fsig);
        if let TypeKind::Function(fsig) = fsig.kind() {
            let mut args = Vec::new();
            // resolve arguments
            for (name, ty) in fsig.argument_types() {
                let ty = self.ctx.resolve_type(ty.clone());
                let ctype = self.resolve_btype_to_ctype(&ty)?;

                args.push(CField {
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
                    Some(CField { name: None, ty: ty })
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
