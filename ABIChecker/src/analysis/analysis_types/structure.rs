use inkwell::{
    targets::TargetData,
    types::{AnyType, AnyTypeEnum, StructType},
};

/// Analysis structure, store llvm struct info.
/// `raw` means this structure is only from LLVM info, no source info integrated yet.
#[derive(Debug, Clone)]
pub struct AStruct {
    // is_raw: bool,
    name: Option<String>,
    // is_union: Option<bool>,
    // is_enum: Option<bool>,
    fields: Vec<AField>,
    // alignment: u32,
}

#[derive(Debug, Clone)]
pub enum ATypeLazyStruct {
    Name(String),
    Struct(AStruct),
}

#[derive(Debug, Clone)]
pub struct AField {
    name: Option<String>,
    // is_padding: Option<bool>,
    ty: AType,
    range: (u32, u32),
}

#[derive(Debug, Clone)]
pub enum AType {
    /// A contiguous homogeneous container type.
    ArrayType(Box<AType>, u32),
    /// A floating point type.
    FloatType(String),
    /// An integer type.
    IntType(String),
    /// A pointer type.
    PointerType(Box<AType>),
    /// A contiguous heterogeneous container type.
    StructType(ATypeLazyStruct),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType(Box<AType>),
}

impl AStruct {
    pub fn from_llvm_raw(st: &StructType, target: &TargetData) -> Self {
        // Only LLVM info currently, we call it raw structure
        // let is_raw = true;
        // let alignment = target.get_abi_alignment(st);
        // let size = target.get_abi_size(&st);

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
                name: None,
                // is_padding: None,
                ty: fty,
                range: (start as u32, end as u32),
            });
        }

        Self {
            // is_raw,
            name: None,
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

    #[inline]
    pub fn get_fields(&self) -> &Vec<AField> {
        &self.fields
    }

    #[inline]
    pub fn get_range(&self) -> (u32, u32) {
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

    pub fn get_fields_from_offset(&self, offset: u32) -> &AField {
        for f in &self.fields {
            if f.get_start() >= offset {
                return f;
            }
        }

        unreachable!()
    }
}

impl ATypeLazyStruct {
    pub fn get_name(&self) -> String {
        match self {
            ATypeLazyStruct::Name(name) => name.clone(),
            ATypeLazyStruct::Struct(st) => st.name.clone().unwrap(),
        }
    }
}

impl AField {
    pub fn temp_field(ty: AType, range: (u32, u32)) -> Self {
        AField {
            name: None,
            // is_padding: None,
            ty,
            range,
        }
    }

    pub fn temp_st(fields: Vec<AField>) -> Self {
        let st = AStruct {
            // is_raw: true,
            name: None,
            // is_union: None,
            // is_enum: None,
            fields,
            // alignment: 0,
        };

        let range = st.get_range();

        AField {
            name: None,
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

    #[inline]
    pub fn get_start(&self) -> u32 {
        self.range.0
    }

    pub fn cmp_number(&self) -> u32 {
        self.ty.cmp_number()
    }

    pub fn to_struct(&self) -> Option<&AStruct> {
        match &self.ty {
            AType::StructType(ATypeLazyStruct::Struct(st)) => Some(st),
            _ => None,
        }
    }
}

impl AType {
    /// Translate LLVM type to our AType.
    /// Will this function ends?
    pub fn from_anytype(value: AnyTypeEnum, target: &TargetData, extend_struct: bool) -> Self {
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
                    let name = st
                        .get_name()
                        .expect("Fatal Error, struct has no name")
                        .to_str()
                        .expect("Fatal Error, struct name is not valid utf8")
                        .to_string();
                    AType::StructType(ATypeLazyStruct::Name(name))
                }
            }
            AnyTypeEnum::VectorType(vec) => AType::VectorType(Box::new(AType::from_anytype(
                vec.get_element_type().as_any_type_enum(),
                target,
                extend_struct,
            ))),
            _ => unimplemented!(),
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
