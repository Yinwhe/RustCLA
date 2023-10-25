use std::slice::Iter;

pub struct AResults {
    results: Vec<(AResult, AResultLevel)>,
}

pub enum AResultLevel {
    Warning,
    Error,
}

#[derive(Debug)]
pub enum AResult {
    // functions issue
    ConventionIssue(u32, u32),
    SigIssue(SigMismatch),

    // structs issues
    StructIssue((u32, u32), (u32, u32), StructMismatch),
}

#[derive(Debug)]
pub enum SigMismatch {
    ParamLen,
    ParamType(u32),
    RetType,
}

#[derive(Debug)]
pub enum StructMismatch {
    SizeMismatch,
    TypeMismatch,
    IsPadding,
    IsOpaque,
}

impl AResults {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_func_convention_issue(&mut self, r: u32, c: u32) {
        self.results
            .push((AResult::func_convention_issue(r, c), AResultLevel::Warning));
    }

    pub fn add_func_sig_issue(&mut self, sig: SigMismatch) {
        self.results
            .push((AResult::func_sig_issue(sig), AResultLevel::Error));
    }

    pub fn add_struct_issue(
        &mut self,
        r_range: (u32, u32),
        c_range: (u32, u32),
        mis: StructMismatch,
        level: AResultLevel,
    ) {
        self.results
            .push((AResult::struct_issue(r_range, c_range, mis), level));
    }

    pub fn get_iters(&self) -> Iter<'_, (AResult, AResultLevel)> {
        self.results.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    pub fn extend(&mut self, other: AResults) {
        self.results.extend(other.results);
    }
}

impl AResult {
    pub fn func_convention_issue(r: u32, c: u32) -> Self {
        AResult::ConventionIssue(r, c)
    }

    pub fn func_sig_issue(mis: SigMismatch) -> Self {
        AResult::SigIssue(mis)
    }

    pub fn struct_issue(r_range: (u32, u32), c_range: (u32, u32), mis: StructMismatch) -> Self {
        AResult::StructIssue(r_range, c_range, mis)
    }
}

impl AResultLevel {
    #[allow(unused)]
    pub fn is_warning(&self) -> bool {
        match self {
            AResultLevel::Warning => true,
            _ => false,
        }
    }
    
    #[allow(unused)]
    pub fn is_error(&self) -> bool {
        match self {
            AResultLevel::Error => true,
            _ => false,
        }
    }
}
