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
    ArrayType,
    /// A floating point type.
    FloatType,
    /// An integer type.
    IntType,
    /// A pointer type.
    PointerType,
    /// A contiguous heterogeneous container type.
    StructType,
    /// A contiguous homogeneous "SIMD" container type.
    VectorType,
}

impl AStruct {
    pub fn from_llvm_raw(st: StructType, target: TargetData) -> Self {
        // Only LLVM info currently, we call it raw structure
        let is_raw = true;
        let alignment = target.get_abi_alignment(&st);
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
            let fty = Self::get_type(ty, &target);

            fields.push(
                AField {
                    name: None,
                    is_padding: None,
                    ty: fty,
                    range: (start as u32, end as u32),
                }
            );
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

    /// Translate LLVM type to our AType
    fn get_type(ty: BasicTypeEnum, target_data: &TargetData) -> AType {
        match ty {
            BasicTypeEnum::ArrayType(_) => AType::ArrayType,
            BasicTypeEnum::FloatType(_) => AType::FloatType,
            BasicTypeEnum::IntType(_) => AType::IntType,
            BasicTypeEnum::PointerType(_) => AType::PointerType,
            BasicTypeEnum::StructType(_) => AType::StructType,
            BasicTypeEnum::VectorType(_) => AType::VectorType,
        }
    } 
}
