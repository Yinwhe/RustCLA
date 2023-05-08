use std::fmt::Debug;

use inkwell::{
    targets::TargetData,
    types::{BasicTypeEnum, StructType},
};

#[derive(Debug)]
pub struct Analysis {
    pub structs: Vec<AnalysisStruct>,
    pub functions: Vec<AnalysisFunction>,
}

#[derive(Debug, Clone)]
pub struct AnalysisStruct {
    pub name: Option<String>,
    pub is_union: bool,
    pub is_enum: bool,

    pub fields: Vec<AnalysisField>,
    pub alignment: u32,
}

#[derive(Clone)]
pub struct AnalysisField {
    pub name: Option<String>,
    pub is_padding: bool,

    pub ty: AnalysisFieldType,
    pub range: (u32, u32),
    // _inner: BasicTypeEnum,
}

#[derive(Debug, Clone)]
pub enum AnalysisFieldType {
    /// A contiguous homogeneous container type.
    ArrayType,
    /// A floating point type.
    FloatType,
    /// An integer type.
    IntType,
    /// A pointer type.
    PointerType,
    /// A contiguous heterogeneous container type.
    StructType(AnalysisStruct),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType,
}

impl AnalysisFieldType {
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

impl AnalysisStruct {
    pub fn from_ctx_raw(st: StructType, target_data: &TargetData) -> Self {
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
                is_padding: false,

                ty: fty,
                range: (start, end),
                // _inner: ty,
            });
        }

        Self {
            name: None,
            is_union: true,
            is_enum: true,

            fields,
            alignment,
        }
    }

    pub fn get_field(&self, index: usize) -> Option<&AnalysisField> {
        self.fields.get(index)
    }

    pub fn get_fields_iters(self) -> impl Iterator<Item = AnalysisField> {
        self.fields.into_iter()
    }

    pub fn get_alignment(&self) -> u32 {
        self.alignment
    }

    fn get_type(ty: BasicTypeEnum, target_data: &TargetData) -> AnalysisFieldType {
        match ty {
            BasicTypeEnum::ArrayType(_) => AnalysisFieldType::ArrayType,
            BasicTypeEnum::FloatType(_) => AnalysisFieldType::FloatType,
            BasicTypeEnum::IntType(_) => AnalysisFieldType::IntType,
            BasicTypeEnum::PointerType(_) => AnalysisFieldType::PointerType,
            BasicTypeEnum::StructType(st) => {
                AnalysisFieldType::StructType(Self::from_ctx_raw(st, target_data))
            }
            BasicTypeEnum::VectorType(_) => AnalysisFieldType::VectorType,
        }
    }
}

impl Debug for AnalysisField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ type: {:#?}, range: ({}, {}), name: {:?}, is_padding: {:?}}}",
            self.ty, self.range.0, self.range.1, self.name, self.is_padding
        )
    }
}

impl AnalysisField {
    pub fn get_size(&self) -> u32 {
        self.range.1 - self.range.0
    }

    pub fn get_type_id(&self) -> u32 {
        self.ty.get_type_id()
    }

    pub fn can_be_anytype(&self) -> bool {
        // We assume padding and opaque types
        // can be any types.
        self.ty.get_type_id() == 0
    }

    pub fn get_struct_mut(&mut self) -> Option<&mut AnalysisStruct> {
        match &mut self.ty {
            AnalysisFieldType::StructType(st) => Some(st),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisFunction {
    pub name: String,
    pub params: Vec<AnalysisParameters>,
    pub ret: Option<AnalysisParameters>,
}

#[derive(Debug, Clone)]
pub struct AnalysisParameters {
    pub name: Option<String>,
    pub pass_by: AnalysisPassBy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisPassBy {
    Value,
    PointerOrReference,
}

#[derive(Debug)]
pub struct AnalysisStructResult {
    rst: AnalysisStruct,
    cst: AnalysisStruct,
    infos: Vec<AnalysisResultContent>,
}

pub struct AnalsisFunctionResult {
    rfunc: AnalysisFunction,
    cfunc: AnalysisFunction,
    infos: Vec<AnalysisResultContent>,
}

#[derive(Debug)]
pub struct AnalysisResultContent {
    rloc: usize,
    cloc: usize,
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
    // For struct
    AlignmentMismatch,
    SizeMismatch,
    TypeMismatch,

    // For function
    ArgsLengthMismatch,
    ArgsPassByMismatch,
    RetMismatch,
    RetPassByMismatch
}

impl AnalysisStructResult {
    pub fn new(rst: AnalysisStruct, cst: AnalysisStruct) -> Self {
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

    pub fn get_rst(&self) -> &AnalysisStruct {
        &self.rst
    }

    pub fn get_cst(&self) -> &AnalysisStruct {
        &self.cst
    }
}

impl AnalsisFunctionResult {
    pub fn new(rfunc: AnalysisFunction, cfunc: AnalysisFunction) -> Self {
        Self {
            rfunc,
            cfunc,
            infos: Vec::new(),
        }
    }

    pub fn add_info(&mut self, info: AnalysisResultContent) {
        self.infos.push(info);
    }

    pub fn get_infos(&self) -> impl Iterator<Item = &AnalysisResultContent> {
        self.infos.iter()
    }

    pub fn get_rfunc(&self) -> &AnalysisFunction {
        &self.rfunc
    }

    pub fn get_cfunc(&self) -> &AnalysisFunction {
        &self.cfunc
    }
}

impl AnalysisResultContent {
    pub fn warn(rloc: usize, cloc: usize, ty: AnalysisResultType, cont: String) -> Self {
        Self {
            rloc,
            cloc,
            level: AnalysisResultLevel::Warn,
            ty,
            cont,
        }
    }

    pub fn error(rloc: usize, cloc: usize, ty: AnalysisResultType, cont: String) -> Self {
        Self {
            rloc,
            cloc,
            level: AnalysisResultLevel::Error,
            ty,
            cont,
        }
    }
}
