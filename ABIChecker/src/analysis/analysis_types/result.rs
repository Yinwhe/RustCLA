pub enum AResult {
    FuncIssue(FuncIssue),
    StructIssue,
}

pub enum FuncIssue {
    ConventionMismatch,
    SigMismatch,
}

pub enum StructIssue {
}

impl AResult {
    pub fn func_convention_issue() -> Self {
        AResult::FuncIssue(FuncIssue::ConventionMismatch)
    }

    pub fn func_sig_issue() -> Self {
        AResult::FuncIssue(FuncIssue::SigMismatch)
    }
}