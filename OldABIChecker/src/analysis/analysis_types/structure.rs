use std::{fmt::Display, hash::Hash};

use inkwell::{
    targets::TargetData,
    types::{AnyType, AnyTypeEnum, StructType},
};

/// Analysis structure, store llvm struct info.
/// `raw` means this structure is only from LLVM info, no source info integrated yet.
#[derive(Debug, Clone)]
pub struct AStruct<'ctx> {
    // is_raw: bool,
    // name: Option<String>,
    // is_union: Option<bool>,
    // is_enum: Option<bool>,
    fields: Vec<AField<'ctx>>,
    // alignment: u32,
}

#[derive(Debug, Clone)]
pub enum ATypeLazyStruct<'ctx> {
    Name(String, StructType<'ctx>),
    Struct(AStruct<'ctx>),
}

#[derive(Debug, Clone)]
pub struct AField<'ctx> {
    // name: Option<String>,
    // is_padding: Option<bool>,
    ty: AType<'ctx>,
    range: (u32, u32),
}

#[derive(Debug, Clone)]
pub enum AType<'ctx> {
    /// A contiguous homogeneous container type.
    ArrayType(Box<AType<'ctx>>, u32),
    /// A floating point type.
    FloatType(String),
    /// An integer type.
    IntType(String),
    /// A pointer type.
    PointerType(Box<AType<'ctx>>),
    /// A contiguous heterogeneous container type.
    StructType(ATypeLazyStruct<'ctx>),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType(Box<AType<'ctx>>),
}

impl<'ctx> AStruct<'ctx> {
    pub fn from_llvm_raw(st: &StructType<'ctx>, target: &TargetData) -> Self {
        // Only LLVM info currently, we call it raw structure
        // let is_raw = true;
        // let alignment = target.get_abi_alignment(st);
        let struct_end = target.get_abi_size(st);

        let mut fields = Vec::new();
        // Get field info
        for (index, ty) in st.get_field_types().into_iter().enumerate() {
            // field memory layout
            let start = target
                .offset_of_element(&st, index as u32)
                .expect("Fatal Error, get element offset failed");
            let end = start + target.get_abi_size(&ty);

            // field type
            let fty = AType::from_anytype(ty.as_any_type_enum(), target, true);

            fields.push(AField {
                // name: None,
                // is_padding: None,
                ty: fty,
                range: (start as u32, end as u32),
            });
        }

        let field_end = if let Some(last) = fields.last() {
            last.range.1
        } else {
            0
        } as u64;

        if struct_end - field_end != 0 {
            assert!(struct_end > field_end);
            let size = struct_end - field_end;

            fields.push(AField {
                ty: AType::ArrayType(Box::new(AType::IntType("\"i8\"".to_string())), size as u32),
                range: (field_end as u32, struct_end as u32),
            });
        }

        Self {
            // is_raw,
            // name: None,
            // is_union: None,
            // is_enum: None,
            fields,
            // alignment,
        }
    }

    #[inline]
    pub fn get_field(&self, index: usize) -> Option<AField> {
        self.fields.get(index).cloned()
    }

    // #[inline]
    // pub fn get_fields(&self) -> &Vec<AField> {
    //     &self.fields
    // }

    #[inline]
    pub fn get_range(&self) -> (u32, u32) {
        if self.fields.is_empty() {
            return (0, 0);
        }

        let start = self.fields.first().unwrap().range.0;
        let end = self.fields.last().unwrap().range.1;

        (start, end)
    }

    pub fn get_inner_one(&self) -> Option<&AField> {
        if self.fields.len() != 1 {
            return None;
        }

        let inner = self.fields.first().expect("Should not happen");
        if let Some(st) = inner.to_struct() {
            st.get_inner_one()
        } else {
            Some(inner)
        }
    }
}

impl<'ctx> ATypeLazyStruct<'ctx> {
    pub fn get_lazy(&self) -> (&String, &StructType<'ctx>) {
        match self {
            ATypeLazyStruct::Name(name, st) => (name, st),
            ATypeLazyStruct::Struct(_) => panic!("Fatal Error, struct is not lazy"),
        }
    }

    pub fn get_struct(&self) -> &AStruct<'ctx> {
        match self {
            ATypeLazyStruct::Name(_, _) => panic!("Fatal Error, struct is lazy"),
            ATypeLazyStruct::Struct(st) => st,
        }
    }
}

impl<'ctx> AField<'ctx> {
    pub fn temp_field(ty: AType<'ctx>, range: (u32, u32)) -> Self {
        AField {
            // name: None,
            // is_padding: None,
            ty,
            range,
        }
    }

    pub fn temp_st(fields: Vec<AField<'ctx>>) -> Self {
        let st = AStruct {
            // is_raw: true,
            // name: None,
            // is_union: None,
            // is_enum: None,
            fields,
            // alignment: 0,
        };

        let range = st.get_range();

        AField {
            // name: None,
            // is_padding: None,
            ty: AType::StructType(ATypeLazyStruct::Struct(st)),
            range: range,
        }
    }

    #[inline]
    pub fn get_range(&self) -> (u32, u32) {
        self.range
    }

    #[inline]
    pub fn get_size(&self) -> u32 {
        self.range.1 - self.range.0
    }

    #[inline]
    pub fn get_type(&self) -> AType {
        self.ty.to_owned()
    }

    pub fn cmp_number(&self) -> u32 {
        self.ty.cmp_number()
    }

    pub fn can_be_padding(&self) -> Option<(String, u32)> {
        if let AType::ArrayType(ty, len) = &self.ty {
            if let AType::IntType(ty) = ty.as_ref() {
                return Some((ty.to_owned(), len.to_owned()));
            }
        }

        None
    }

    pub fn to_struct(&self) -> Option<&AStruct> {
        match &self.ty {
            AType::StructType(ATypeLazyStruct::Struct(st)) => Some(st),
            _ => None,
        }
    }
}

impl<'ctx> AType<'ctx> {
    /// Translate LLVM type to our AType.
    /// Will this function ends?
    pub fn from_anytype(
        value: AnyTypeEnum<'ctx>,
        target: &TargetData,
        extend_struct: bool,
    ) -> Self {
        match value {
            AnyTypeEnum::ArrayType(ty) => AType::ArrayType(
                Box::new(AType::from_anytype(
                    ty.get_element_type().as_any_type_enum(),
                    target,
                    extend_struct,
                )),
                ty.len(),
            ),
            AnyTypeEnum::FloatType(f) => AType::FloatType(f.to_string()),
            AnyTypeEnum::IntType(i) => AType::IntType(i.to_string()),
            AnyTypeEnum::PointerType(ptr) => {
                let ty = ptr.get_element_type();
                AType::PointerType(Box::new(AType::from_anytype(
                    ty.as_any_type_enum(),
                    target,
                    extend_struct,
                )))
            }
            AnyTypeEnum::StructType(st) => {
                if extend_struct {
                    AType::StructType(ATypeLazyStruct::Struct(AStruct::from_llvm_raw(&st, target)))
                } else {
                    let name = if let Some(name) = st.get_name() {
                        name.to_str()
                            .expect("Fatal Error, struct name is not valid utf8")
                            .to_string()
                    } else {
                        format!("unamed_{:?}", st)
                    };

                    AType::StructType(ATypeLazyStruct::Name(name, st))
                }
            }
            AnyTypeEnum::VectorType(vec) => AType::VectorType(Box::new(AType::from_anytype(
                vec.get_element_type().as_any_type_enum(),
                target,
                extend_struct,
            ))),
            // TODO: Why and how to handle these types?
            // Currently we just use int type to represent them, since
            // we can store string into it.
            AnyTypeEnum::FunctionType(fp) => AType::IntType(fp.to_string()),
            AnyTypeEnum::VoidType(v) => AType::IntType(v.to_string()),
        }
    }

    pub fn cmp_number(&self) -> u32 {
        match &self {
            AType::FloatType(_) => 1,
            AType::IntType(_) => 1,
            AType::PointerType(_) => 1,
            AType::VectorType(_) => 1,
            AType::ArrayType(_, _) => 2,
            AType::StructType(_) => 3,
        }
    }
}

impl PartialEq for ATypeLazyStruct<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get_lazy().0 == other.get_lazy().0
    }
}

impl Eq for ATypeLazyStruct<'_> {}

impl Hash for ATypeLazyStruct<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_lazy().0.hash(state);
    }
}

impl Display for AStruct<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = self
            .fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{{{}}}", fields)
    }
}

impl Display for AField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}> ({}, {})", self.ty, self.range.0, self.range.1)
    }
}

impl Display for AType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AType::ArrayType(ty, len) => write!(f, "[{} x {}]", len, ty),
            AType::FloatType(name) => write!(f, "{}", name),
            AType::IntType(name) => write!(f, "{}", name),
            AType::PointerType(ty) => write!(f, "{} *", ty),
            AType::StructType(st) => write!(f, "{}", st.get_struct()),
            AType::VectorType(ty) => write!(f, "[{} x ?]", ty),
        }
    }
}
