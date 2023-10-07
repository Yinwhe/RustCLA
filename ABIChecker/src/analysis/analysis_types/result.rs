use std::fmt::Display;
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
    StructIssue(u32, u32, StructMismatch),
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

    pub fn add_struct_issue(&mut self, r_off: u32, c_off: u32, mis: StructMismatch, level: AResultLevel) {
        self.results.push((
            AResult::struct_issue(r_off, c_off, mis),
            level,
        ));
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

    pub fn struct_issue(r_off: u32, c_off: u32, mis: StructMismatch) -> Self {
        AResult::StructIssue(r_off, c_off, mis)
    }
}

// impl Display for AResult {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::ConventionIssue(r, c) => {
//                 write!(f, "call convention mismatch, rust side: {}, c/c++ side: {}", r, c)
//             }
//             Self::SigIssue(sig) => match sig {
//                 SigMismatch::ParamLen => write!(f, "param length mismatch"),
//                 SigMismatch::ParamType(i) => write!(f, "param type mismatch at {}", i),
//                 SigMismatch::RetType => write!(f, "return type mismatch"),
//             },
//             Self::StructIssue(_, _, _) => write!(f, "type issue"),
//         }
//     }
// }
