use std::fmt;

pub type Result<T> = std::result::Result<T, RecordError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    Bounds,
    Canonical,
    DuplicateField,
    InvalidEnvelope,
    InvalidSignature,
    InvalidRecord,
    Replay,
    Trust,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordError {
    pub code: ErrorCode,
    pub message: &'static str,
}

impl RecordError {
    pub(crate) const fn new(code: ErrorCode, message: &'static str) -> Self {
        Self { code, message }
    }
}

impl fmt::Display for RecordError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}: {}", self.code, self.message)
    }
}

impl std::error::Error for RecordError {}
