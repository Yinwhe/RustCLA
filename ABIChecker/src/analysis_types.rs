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
    pub temp: bool, // _inner: BasicTypeEnum,
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
                temp: false, // _inner: ty,
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

    pub fn get_field(&self, index: usize) -> Option<AnalysisField> {
        self.fields.get(index).cloned()
    }

    pub fn get_alignment(&self) -> u32 {
        self.alignment
    }

    pub fn get_range(&self) -> (u32, u32) {
        let start = self.fields.first().unwrap().range.0;
        let end = self.fields.last().unwrap().range.1;

        (start, end)
    }

    pub fn temp(fields: Vec<AnalysisField>) -> Self {
        Self {
            name: None,
            is_union: false,
            is_enum: false,
            fields: fields,
            alignment: 0,
        }
    }

    pub fn get_fields_from_range(&self, start: u32, end: u32) -> Vec<AnalysisField> {
        let mut fields = Vec::new();
        let end = if end == 0 { u32::MAX } else { end };
        for f in &self.fields {
            if f.range.0 >= start && f.range.0 < end {
                fields.push(f.clone());
            }
        }
        fields
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
        let tys = match &self.ty {
            AnalysisFieldType::FloatType => "FloatType".to_string(),
            AnalysisFieldType::ArrayType => {
                if self.is_padding {
                    "Padding".to_string()
                } else {
                    "ArrayType".to_string()
                }
            }
            AnalysisFieldType::IntType => "IntType".to_string(),
            AnalysisFieldType::PointerType => "PointerType".to_string(),
            AnalysisFieldType::VectorType => "VectorType".to_string(),
            AnalysisFieldType::StructType(st) => format!("StructType {:?}", st.fields),
        };

        let name = if let Some(name) = &self.name {
            name
        } else {
            ""
        };

        write!(f, "{}({} {}~{})", name, tys, self.range.0, self.range.1)
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
        self.is_padding
    }

    pub fn get_struct_mut(&mut self) -> Option<&mut AnalysisStruct> {
        match &mut self.ty {
            AnalysisFieldType::StructType(st) => Some(st),
            _ => None,
        }
    }

    pub fn get_struct(&self) -> Option<&AnalysisStruct> {
        match &self.ty {
            AnalysisFieldType::StructType(st) => Some(st),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn is_normal(&self) -> bool {
        let id = self.get_type_id();
        id == 1 || id == 2 || id == 3 || id == 5
    }

    #[allow(unused)]
    pub fn is_struct(&self) -> bool {
        self.get_type_id() == 4
    }

    #[allow(unused)]
    pub fn is_array(&self) -> bool {
        self.get_type_id() == 0
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
    pub ty: AnalysisParametersType,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum AnalysisPassBy {
//     Value,
//     PointerOrReference,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisParametersType {
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

impl From<BasicTypeEnum<'_>> for AnalysisParametersType {
    fn from(value: BasicTypeEnum) -> Self {
        match value {
            BasicTypeEnum::ArrayType(_) => AnalysisParametersType::ArrayType,
            BasicTypeEnum::FloatType(_) => AnalysisParametersType::FloatType,
            BasicTypeEnum::IntType(_) => AnalysisParametersType::IntType,
            BasicTypeEnum::PointerType(_) => AnalysisParametersType::PointerType,
            BasicTypeEnum::StructType(_) => AnalysisParametersType::StructType,
            BasicTypeEnum::VectorType(_) => AnalysisParametersType::VectorType,
        }
    }
}

#[derive(Debug)]
pub struct AnalysisStructResult {
    #[allow(unused)]
    rst: AnalysisStruct,
    #[allow(unused)]
    cst: AnalysisStruct,
    infos: Vec<AnalysisResultContent>,
}

pub struct AnalsisFunctionResult {
    #[allow(unused)]
    rfunc: AnalysisFunction,
    #[allow(unused)]
    cfunc: AnalysisFunction,
    infos: Vec<AnalysisResultContent>,
}

#[derive(Debug, Clone)]
pub struct AnalysisResultContent {
    pub rloc: (u32, u32),
    pub cloc: (u32, u32),
    pub level: AnalysisResultLevel,
    pub ty: AnalysisResultType,
    pub cont: String,
}

#[derive(Debug, Clone)]
pub enum AnalysisResultLevel {
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub enum AnalysisResultType {
    // For struct
    AlignmentMismatch,
    SizeMismatch,
    TypeMismatch,

    // For function
    ArgsLengthMismatch,
    ArgsTypeMismatch,
    RetVoidMismatch,
    RetTypeMismatch,
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

    pub fn get_info(&self) -> Vec<AnalysisResultContent> {
        self.infos.clone()
    }

    pub fn merge(&mut self, res: &AnalysisStructResult) {
        self.infos.extend(res.get_info())
    }

    pub fn is_error(&self) -> bool {
        self.infos.iter().any(|info| info.is_error())
    }

    pub fn is_empty(&self) -> bool {
        self.infos.is_empty()
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

    #[allow(unused)]
    pub fn is_error(&self) -> bool {
        self.infos.iter().any(|info| info.is_error())
    }

    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.infos.is_empty()
    }
}

impl AnalysisResultContent {
    pub fn warn(rloc: (u32, u32), cloc: (u32, u32), ty: AnalysisResultType, cont: String) -> Self {
        Self {
            rloc,
            cloc,
            level: AnalysisResultLevel::Warn,
            ty,
            cont,
        }
    }

    pub fn error(rloc: (u32, u32), cloc: (u32, u32), ty: AnalysisResultType, cont: String) -> Self {
        Self {
            rloc,
            cloc,
            level: AnalysisResultLevel::Error,
            ty,
            cont,
        }
    }

    pub fn is_error(&self) -> bool {
        match self.level {
            AnalysisResultLevel::Warn => false,
            AnalysisResultLevel::Error => true,
        }
    }
}

// impl Display for AnalysisResultType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }
// }
