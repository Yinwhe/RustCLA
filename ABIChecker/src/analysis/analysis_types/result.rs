use std::fmt::Display;
use std::slice::Iter;

pub struct AResults {
    results: Vec<(AResult, AResultLevel)>,
}

pub enum AResultLevel {
    Warning,
    Error,
}

pub enum AResult {
    // functions issue
    ConventionIssue(u32, u32),
    SigIssue(SigMismatch),

    // structs issues
    StructIssue(u32, StructMismatch),
}

pub enum SigMismatch {
    ParamLen,
    ParamType(u32),
    RetType,
}

pub enum StructMismatch {
    SizeMismatch,
    TypeMismatch,
}

impl AResult {
    pub fn func_convention_issue(r: u32, c: u32) -> Self {
        AResult::ConventionIssue(r, c)
    }

    pub fn func_sig_issue(sig: SigMismatch) -> Self {
        AResult::SigIssue(sig)
    }

    pub fn func_param_type_issue(id: u32, param: StructMismatch) -> Self {
        AResult::StructIssue(id, param)
    }
}

impl Display for AResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConventionIssue(r, c) => {
                write!(f, "call convention mismatch, rust: {}, c: {}", r, c)
            }
            Self::SigIssue(sig) => match sig {
                SigMismatch::ParamLen => write!(f, "param length mismatch"),
                SigMismatch::ParamType(i) => write!(f, "param type mismatch at {}", i),
                SigMismatch::RetType => write!(f, "return type mismatch"),
            },
            Self::StructIssue(_, _) => write!(f, "type issue"),
        }
    }
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

    pub fn add_struct_issue(&mut self, id: u32, param: StructMismatch) {
        self.results.push((
            AResult::func_param_type_issue(id, param),
            AResultLevel::Error,
        ));
    }

    pub fn get_iters(&self) -> Iter<'_, (AResult, AResultLevel)> {
        self.results.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}
