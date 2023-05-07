use std::fmt::Debug;

use inkwell::{
    targets::TargetData,
    types::{BasicTypeEnum, StructType},
};

#[derive(Debug)]
pub struct Analysis<'ctx> {
    pub info_structs: Vec<AnalysisStruct<'ctx>>,
    pub raw_structs: Vec<AnalysisStruct<'ctx>>,
    // pub info_functions: Vec<AnalysisFunction>,
}

impl<'ctx> Analysis<'ctx> {
    pub fn new(
        info_structs: Vec<AnalysisStruct<'ctx>>,
        raw_structs: Vec<AnalysisStruct<'ctx>>,
    ) -> Self {
        Self {
            info_structs,
            raw_structs,
            // info_functions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisStruct<'ctx> {
    pub is_raw: bool,
    pub name: Option<String>,
    pub is_union: Option<bool>,
    pub is_enum: Option<bool>,

    pub fields: Vec<AnalysisField<'ctx>>,
    pub alignment: u32,
}

#[derive(Clone)]
pub struct AnalysisField<'ctx> {
    pub name: Option<String>,
    pub is_padding: Option<bool>,

    pub ty: AnalysisFieldType<'ctx>,
    pub range: (u32, u32),
    _inner: BasicTypeEnum<'ctx>,
}

#[derive(Debug, Clone)]
pub enum AnalysisFieldType<'ctx> {
    /// A contiguous homogeneous container type.
    ArrayType,
    /// A floating point type.
    FloatType,
    /// An integer type.
    IntType,
    /// A pointer type.
    PointerType,
    /// A contiguous heterogeneous container type.
    StructType(AnalysisStruct<'ctx>),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType,
}

impl<'ctx> AnalysisFieldType<'ctx> {
    pub fn get_type_id(&self) -> u32 {
        match self {
            AnalysisFieldType::ArrayType => 0,
            AnalysisFieldType::FloatType => 1,
            AnalysisFieldType::IntType => 2,
            AnalysisFieldType::PointerType => 3,
            AnalysisFieldType::StructType(_) => 4,
            AnalysisFieldType::VectorType => 5,
        }
    }
}


impl<'ctx> AnalysisStruct<'ctx> {
    pub fn from_ctx_raw(st: StructType<'ctx>, target_data: &TargetData) -> Self {
        let mut fields = Vec::new();
        let alignment = target_data.get_abi_alignment(&st);

        for (index, ty) in st.get_field_types().into_iter().enumerate() {
            let start = target_data
                .offset_of_element(&st, index as u32)
                .expect("Fatal Error, get element offset failed") as u32;
            let end = start + target_data.get_store_size(&ty) as u32;

            let fty = Self::get_type(ty.clone(), target_data);

            fields.push(AnalysisField {
                name: None,
                is_padding: None,

                ty: fty,
                range: (start, end),
                _inner: ty,
            });
        }

        Self {
            is_raw: true,
            name: None,
            is_union: None,
            is_enum: None,

            fields,
            alignment,
        }
    }

    pub fn get_field(&self, index: usize) -> Option<&AnalysisField<'ctx>> {
        self.fields.get(index)
    }

    pub fn get_fields_iters(self) -> impl Iterator<Item = AnalysisField<'ctx>> {
        self.fields.into_iter()
    }

    pub fn get_alignment(&self) -> u32 {
        self.alignment
    }

    fn get_type(ty: BasicTypeEnum<'ctx>, target_data: &TargetData) -> AnalysisFieldType<'ctx> {
        match ty {
            BasicTypeEnum::ArrayType(_) => AnalysisFieldType::ArrayType,
            BasicTypeEnum::FloatType(_) => AnalysisFieldType::FloatType,
            BasicTypeEnum::IntType(_) => AnalysisFieldType::IntType,
            BasicTypeEnum::PointerType(_) => AnalysisFieldType::PointerType,
            BasicTypeEnum::StructType(st) => AnalysisFieldType::StructType(Self::from_ctx_raw(st, target_data)),
            BasicTypeEnum::VectorType(_) => AnalysisFieldType::VectorType,
        }
    }
}

impl Debug for AnalysisField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ type: {:#?}, range: ({}, {}), name: {:?}, is_padding: {:?}}}",
            self.ty, self.range.0, self.range.1, self.name, self.is_padding
        )
    }
}

impl<'ctx> AnalysisField<'ctx> {
    pub fn get_size(&self) -> u32 {
        self.range.1 - self.range.0
    }

    pub fn get_type_id(&self) -> u32 {
        self.ty.get_type_id()
    }

    pub fn can_be_anytype(&self) -> bool {
        // We assume padding and opaque types
        // can be any types.
        self._inner.is_array_type()
    }

    pub fn get_struct_mut(&mut self) -> Option<&mut AnalysisStruct<'ctx>> {
        match &mut self.ty {
            AnalysisFieldType::StructType(st) => Some(st),
            _ => None,
        }
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

#[derive(Debug)]
pub struct AnalysisResult<'ctx> {
    rst: AnalysisStruct<'ctx>,
    cst: AnalysisStruct<'ctx>,
    infos: Vec<AnalysisResultContent>,
}

#[derive(Debug)]
pub struct AnalysisResultContent {
    a_loc: usize,
    b_loc: usize,
    level: AnalysisResultLevel,
    ty: AnalysisResultType,
    cont: String,
}

#[derive(Debug)]
pub enum AnalysisResultLevel {
    Warn,
    Error,
}

#[derive(Debug)]
pub enum AnalysisResultType {
    AlignmentMismatch,
    SizeMismatch,
    TypeMismatch,
}

impl<'ctx> AnalysisResult<'ctx> {
    pub fn new(rst: AnalysisStruct<'ctx>, cst: AnalysisStruct<'ctx>) -> Self {
        Self {
            rst,
            cst,
            infos: Vec::new(),
        }
    }

    pub fn add_info(&mut self, info: AnalysisResultContent) {
        self.infos.push(info);
    }

    pub fn get_infos(&self) -> impl Iterator<Item = &AnalysisResultContent> {
        self.infos.iter()
    }

    pub fn get_rst(&self) -> &AnalysisStruct<'ctx> {
        &self.rst
    }

    pub fn get_cst(&self) -> &AnalysisStruct<'ctx> {
        &self.cst
    }
}

impl AnalysisResultContent {
    pub fn warn(a_loc: usize, b_loc: usize, ty: AnalysisResultType, cont: String) -> Self {
        Self {
            a_loc,
            b_loc,
            level: AnalysisResultLevel::Warn,
            ty,
            cont,
        }
    }

    pub fn error(a_loc: usize, b_loc: usize, ty: AnalysisResultType, cont: String) -> Self {
        Self {
            a_loc,
            b_loc,
            level: AnalysisResultLevel::Error,
            ty,
            cont,
        }
    }
}
