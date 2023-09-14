use inkwell::{
    targets::TargetData,
    types::{BasicTypeEnum, StructType},
};

/// Analysis structure, store llvm struct info.
/// `raw` means this structure is only from LLVM info, no source info integrated yet.
#[derive(Debug)]
pub struct AStruct {
    pub is_raw: bool,

    pub name: Option<String>,
    pub is_union: Option<bool>,
    pub is_enum: Option<bool>,

    pub fields: Vec<AField>,
    pub alignment: u32,
}

#[derive(Debug)]
pub struct AField {
    pub name: Option<String>,
    pub is_padding: Option<bool>,

    pub ty: AType,
    pub range: (u32, u32),
}

#[derive(Debug)]
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
    StructType(Box<AStruct>),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType(Box<AType>),
}

impl AStruct {
    pub fn from_llvm_raw(st: &StructType, target: &TargetData) -> Self {
        // Only LLVM info currently, we call it raw structure
        let is_raw = true;
        let alignment = target.get_abi_alignment(st);
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
            let fty = AType::from_btype(ty, target);

            fields.push(AField {
                name: None,
                is_padding: None,
                ty: fty,
                range: (start as u32, end as u32),
            });
        }

        Self {
            is_raw,
            name: None,
            is_union: None,
            is_enum: None,
            fields,
            alignment,
        }
    }
}

impl AType {
    /// Translate LLVM type to our AType
    pub fn from_btype(value: BasicTypeEnum, target: &TargetData) -> Self {
        match value {
            BasicTypeEnum::ArrayType(ty) => AType::ArrayType(
                Box::new(AType::from_btype(ty.get_element_type(), target)),
                ty.len(),
            ),
            BasicTypeEnum::FloatType(f) => AType::FloatType(f.to_string()),
            BasicTypeEnum::IntType(i) => AType::IntType(i.to_string()),
            BasicTypeEnum::PointerType(ptr) => {
                // not sure if this is correct
                let ty = ptr.array_type(1).get_element_type();
                println!("DEBUG: {:?}", ptr);
                AType::PointerType(Box::new(AType::from_btype(ty, target)))
            }
            BasicTypeEnum::StructType(st) => {
                AType::StructType(Box::new(AStruct::from_llvm_raw(&st, target)))
            }
            BasicTypeEnum::VectorType(vec) => {
                AType::VectorType(Box::new(AType::from_btype(vec.get_element_type(), target)))
            }
        }
    }

    pub fn shallow_check(a: &AType, b: &AType) -> bool {
        match (a, b) {
            (AType::ArrayType(_, _), AType::ArrayType(_, _)) => true,
            (AType::FloatType(_), AType::FloatType(_)) => true,
            (AType::IntType(_), AType::IntType(_)) => true,
            (AType::PointerType(_), AType::PointerType(_)) => true,
            (AType::StructType(_), AType::StructType(_)) => true,
            (AType::VectorType(_), AType::VectorType(_)) => true,
            _ => false,
        }
    }

    pub fn deep_check(a: &AType, b: &AType) -> bool {
        match (a, b) {
            (AType::ArrayType(a, alen), AType::ArrayType(b, blen)) => {
                if alen != blen {
                    return false;
                } else {
                    return AType::deep_check(a, b);
                }
            }
            (AType::FloatType(astr), AType::FloatType(bstr)) => astr == bstr,
            (AType::IntType(astr), AType::IntType(bstr)) => astr == bstr,
            (AType::PointerType(a), AType::PointerType(b)) => {
                unimplemented!()
            }
            (AType::StructType(a), AType::StructType(b)) => {
                unimplemented!()
            }
            (AType::VectorType(a), AType::VectorType(b)) => {
                unimplemented!()
            }
            _ => false,
        }
    }
}
