use std::fmt::Debug;

use inkwell::{
    targets::TargetData,
    types::{BasicTypeEnum, StructType},
};

#[derive(Debug)]
pub struct AnalysisStruct<'ctx> {
    fields: Vec<AnalysisField<'ctx>>,
    alignment: u32,
}

#[derive(Clone)]
pub struct AnalysisField<'ctx> {
    type_id: u32,
    range: (u32, u32),
    _inner: BasicTypeEnum<'ctx>,
}

impl<'ctx> AnalysisStruct<'ctx> {
    pub fn from_ctx(st: StructType<'ctx>, target_data: &TargetData) -> Self {
        let mut fields = Vec::new();
        let alignment = target_data.get_abi_alignment(&st);

        for (index, ty) in st.get_field_types().into_iter().enumerate() {
            let id = Self::get_type_id(&ty);
            let start = target_data
                .offset_of_element(&st, index as u32)
                .expect("Fatal Error, get element offset failed") as u32;
            let end = start + target_data.get_store_size(&ty) as u32;

            fields.push(AnalysisField {
                type_id: id,
                range: (start, end),
                _inner: ty,
            });
        }
        Self { fields, alignment }
    }

    pub fn get_fields_iters(self) -> impl Iterator<Item = AnalysisField<'ctx>> {
        self.fields.into_iter()
    }

    pub fn get_alignment(&self) -> u32 {
        self.alignment
    }

    fn get_type_id(ty: &BasicTypeEnum) -> u32 {
        match ty {
            &BasicTypeEnum::ArrayType(_) => 0,
            &BasicTypeEnum::FloatType(_) => 1,
            &BasicTypeEnum::IntType(_) => 2,
            &BasicTypeEnum::PointerType(_) => 3,
            &BasicTypeEnum::StructType(_) => 4,
            &BasicTypeEnum::VectorType(_) => 5,
        }
    }
}

impl Debug for AnalysisField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self.type_id {
            0 => "ArrayType",
            1 => "FloatType",
            2 => "IntType",
            3 => "PointerType",
            4 => "StructType",
            5 => "VectorType",
            _ => unreachable!(),
        };
        write!(
            f,
            "{{ type: {}, range: ({}, {})}}",
            ty, self.range.0, self.range.1
        )
    }
}

impl AnalysisField<'_> {
    pub fn get_size(&self) -> u32 {
        self.range.1 - self.range.0
    }

    pub fn get_type_id(&self) -> u32 {
        self.type_id
    }

    pub fn can_be_anytype(&self) -> bool {
        // We assume padding and opaque types
        // can be any types.
        self._inner.is_array_type()
    }

    pub fn type_match(a: &AnalysisField<'_>, b: &AnalysisField<'_>) -> bool {
        if a.get_type_id() == b.get_type_id() {
            true
        } else {
            if a.can_be_anytype() || b.can_be_anytype() {
                true
            } else {
                false
            }
        }
    }
}

pub enum AnalysisResult<'ctx> {
    Ok,
    Warn((AnalysisStruct<'ctx>, AnalysisStruct<'ctx>), Vec<AnalysisResultContent>),
    Error(),
}

pub struct AnalysisResultContent {
    a_loc: u32,
    b_loc: u32,
    cont: String
}