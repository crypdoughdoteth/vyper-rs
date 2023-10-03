use std::{
    error::Error,
    fmt::Display,
};

#[derive(Debug)]
pub struct CompilerError {
    pub source: String,
}

impl CompilerError {
    pub(crate) fn new(reason: String) -> Self {
        Self { source: reason }
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to compile Vyper contract")
    }
}

impl Error for CompilerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(Err(&self.source).unwrap())
    }
}
