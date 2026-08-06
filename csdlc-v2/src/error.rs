use serde::Serialize;
use strum::{AsRefStr, Display, EnumIter, EnumString};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, V2Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Display, EnumString, AsRefStr, EnumIter)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorCode {
    Io,
    InvalidInput,
    InvalidTransition,
    StaleGeneration,
    StaleDigest,
    GitFailure,
    UnsafeCheckout,
    ReconciliationRequired,
    InvalidManifest,
    ValidationFailed,
    RemoteFailure,
    FieldOwnership,
    CardInvalid,
    CorruptRecord,
    InterruptedTransaction,
}

impl ErrorCode {
    pub fn exit_code(self) -> i32 {
        match self {
            Self::InvalidInput => 64,
            Self::InvalidManifest => 64,
            Self::InvalidTransition | Self::FieldOwnership => 65,
            Self::StaleGeneration | Self::StaleDigest => 66,
            Self::CardInvalid | Self::CorruptRecord => 68,
            Self::InterruptedTransaction => 69,
            Self::UnsafeCheckout => 73,
            Self::Io | Self::GitFailure | Self::RemoteFailure => 74,
            Self::ReconciliationRequired => 75,
            Self::ValidationFailed => 76,
        }
    }
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct V2Error {
    pub code: ErrorCode,
    pub message: String,
}

impl V2Error {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for V2Error {
    fn from(value: std::io::Error) -> Self {
        Self::new(ErrorCode::Io, value.to_string())
    }
}

impl From<serde_json::Error> for V2Error {
    fn from(value: serde_json::Error) -> Self {
        Self::new(ErrorCode::CorruptRecord, value.to_string())
    }
}
