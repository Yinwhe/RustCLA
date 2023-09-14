use std::fmt::Display;

pub enum AResult {
    // functions issue
    ConventionMismatch(u32, u32),
    SigMismatch(SigMismatchType),

    // struct issues
    StructIssue,
}

pub enum SigMismatchType {
    ParamLen,
    ParamType(u32),
    RetType,
}

impl AResult {
    pub fn func_convention_issue(r: u32, c: u32) -> Self {
        AResult::ConventionMismatch(r, c)
    }

    pub fn func_sig_issue(sig: SigMismatchType) -> Self {
        AResult::SigMismatch(sig)
    }
}

impl Display for AResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConventionMismatch(r, c) => {
                write!(f, "call convention mismatch, rust: {}, c: {}", r, c)
            }
            Self::SigMismatch(sig) => match sig {
                SigMismatchType::ParamLen => write!(f, "param length mismatch"),
                SigMismatchType::ParamType(i) => write!(f, "param type mismatch at {}", i),
                SigMismatchType::RetType => write!(f, "return type mismatch"),
            },
            Self::StructIssue => write!(f, "struct issue"),
        }
    }
}