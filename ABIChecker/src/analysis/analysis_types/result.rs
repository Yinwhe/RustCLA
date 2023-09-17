use std::fmt::Display;

pub enum AResult {
    // functions issue
    ConventionIssue(u32, u32),
    SigIssue(SigMismatch),

    // param issues
    ParamIssue(u32, ParamMismatch),
}

pub enum SigMismatch {
    ParamLen,
    ParamType(u32),
    RetType,
}

pub enum ParamMismatch {
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

    pub fn func_param_type_issue(id: u32, param: ParamMismatch) -> Self {
        AResult::ParamIssue(id, param)
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
            Self::ParamIssue(_, _) => write!(f, "type issue"),
        }
    }
}
