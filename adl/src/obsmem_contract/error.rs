/// Stable machine-readable error taxonomy for ObsMem contract boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsMemContractErrorCode {
    ContractVersionMismatch,
    InvalidRequest,
    InvalidQuery,
    PrivacyViolation,
    BackendUnavailable,
}

impl ObsMemContractErrorCode {
    /// Return the stable wire/log error code string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContractVersionMismatch => "OBSMEM_CONTRACT_VERSION_MISMATCH",
            Self::InvalidRequest => "OBSMEM_INVALID_REQUEST",
            Self::InvalidQuery => "OBSMEM_INVALID_QUERY",
            Self::PrivacyViolation => "OBSMEM_PRIVACY_VIOLATION",
            Self::BackendUnavailable => "OBSMEM_BACKEND_UNAVAILABLE",
        }
    }
}

/// Structured contract error with stable code plus human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObsMemContractError {
    pub code: ObsMemContractErrorCode,
    pub message: String,
}

impl ObsMemContractError {
    /// Construct a new contract error.
    pub fn new(code: ObsMemContractErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ObsMemContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for ObsMemContractError {}
