use std::fmt::Debug;

use inkwell::{
    targets::TargetData,
    types::{BasicTypeEnum, StructType},
};

#[derive(Debug)]
pub struct AnalysisStruct<'ctx> {
    fields: Vec<AnalysisField<'ctx>>,
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
        Self { fields }
    }

    pub fn get_fields_iters(self) -> impl Iterator<Item = AnalysisField<'ctx>> {
        self.fields.into_iter()
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
}
