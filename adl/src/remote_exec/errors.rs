use anyhow::Error;

use super::REMOTE_REQUEST_SIGNATURE_ALG_ED25519;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Deterministic security-envelope validation failures for remote execution.
///
/// Use [`SecurityEnvelopeError::code`] as the stable classifier in tests and
/// machine-readable handling.
pub enum SecurityEnvelopeError {
    UnsignedRequestRequired,
    MissingKeyId,
    UnsupportedRequestSignatureAlgorithm {
        alg: String,
    },
    MissingRequestSignature,
    MalformedRequestSignature {
        reason: String,
    },
    RequestSignatureMismatch,
    DisallowedAlgorithm {
        alg: String,
    },
    DisallowedKeySource {
        key_source: String,
    },
    MissingKeySource,
    PathTraversal {
        path: String,
    },
    PathNotFound {
        path: String,
    },
    PathNotCanonical {
        path: String,
    },
    SymlinkDisallowed {
        path: String,
    },
    SymlinkEscape {
        path: String,
    },
    SandboxIoError {
        path: String,
        operation: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Stable remote execute client-side error kinds.
///
/// This enum normalizes transport/protocol/remote failures for deterministic
/// handling and retry classification.
pub enum RemoteExecuteClientErrorKind {
    Timeout,
    Unreachable,
    BadStatus,
    InvalidJson,
    SchemaViolation,
    RemoteExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteExecuteClientError {
    pub kind: RemoteExecuteClientErrorKind,
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for RemoteExecuteClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RemoteExecuteClientError {}

impl RemoteExecuteClientError {
    pub(crate) fn new(
        kind: RemoteExecuteClientErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            message: message.into(),
        }
    }
}

pub fn retryability(err: &Error) -> Option<bool> {
    for cause in err.chain() {
        if cause.downcast_ref::<SecurityEnvelopeError>().is_some() {
            return Some(false);
        }
        if let Some(remote) = cause.downcast_ref::<RemoteExecuteClientError>() {
            return Some(match remote.kind {
                RemoteExecuteClientErrorKind::Timeout
                | RemoteExecuteClientErrorKind::Unreachable
                | RemoteExecuteClientErrorKind::BadStatus
                | RemoteExecuteClientErrorKind::InvalidJson => true,
                RemoteExecuteClientErrorKind::SchemaViolation => false,
                RemoteExecuteClientErrorKind::RemoteExecution => {
                    !matches!(
                        remote.code.as_str(),
                        "REMOTE_SCHEMA_VIOLATION"
                            | "SIGN_POLICY_UNSIGNED_REQUIRED"
                            | "SIGN_POLICY_MISSING_KEY_ID"
                            | "SIGN_POLICY_DISALLOWED_ALGORITHM"
                            | "SIGN_POLICY_DISALLOWED_KEY_SOURCE"
                            | "SIGN_POLICY_MISSING_KEY_SOURCE"
                    ) && !remote.code.starts_with("REMOTE_ENVELOPE_")
                }
            });
        }
    }
    None
}

pub fn stable_failure_kind(err: &Error) -> Option<&'static str> {
    for cause in err.chain() {
        if cause.downcast_ref::<SecurityEnvelopeError>().is_some() {
            return Some("policy_denied");
        }
        if let Some(remote) = cause.downcast_ref::<RemoteExecuteClientError>() {
            return Some(match remote.kind {
                RemoteExecuteClientErrorKind::Timeout => "timeout",
                RemoteExecuteClientErrorKind::SchemaViolation => "schema_error",
                RemoteExecuteClientErrorKind::Unreachable
                | RemoteExecuteClientErrorKind::BadStatus
                | RemoteExecuteClientErrorKind::InvalidJson => "io_error",
                RemoteExecuteClientErrorKind::RemoteExecution => "provider_error",
            });
        }
    }
    None
}

impl SecurityEnvelopeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnsignedRequestRequired => "REMOTE_ENVELOPE_UNSIGNED_REQUEST",
            Self::MissingKeyId => "REMOTE_ENVELOPE_MISSING_KEY_ID",
            Self::UnsupportedRequestSignatureAlgorithm { .. } => {
                "REMOTE_REQUEST_SIGNATURE_UNSUPPORTED_ALGORITHM"
            }
            Self::MissingRequestSignature => "REMOTE_REQUEST_SIGNATURE_MISSING",
            Self::MalformedRequestSignature { .. } => "REMOTE_REQUEST_SIGNATURE_MALFORMED",
            Self::RequestSignatureMismatch => "REMOTE_REQUEST_SIGNATURE_MISMATCH",
            Self::DisallowedAlgorithm { .. } => "REMOTE_ENVELOPE_DISALLOWED_ALGORITHM",
            Self::DisallowedKeySource { .. } => "REMOTE_ENVELOPE_DISALLOWED_KEY_SOURCE",
            Self::MissingKeySource => "REMOTE_ENVELOPE_MISSING_KEY_SOURCE",
            Self::PathTraversal { .. } => "REMOTE_ENVELOPE_PATH_TRAVERSAL",
            Self::PathNotFound { .. } => "REMOTE_ENVELOPE_PATH_NOT_FOUND",
            Self::PathNotCanonical { .. } => "REMOTE_ENVELOPE_PATH_NOT_CANONICAL",
            Self::SymlinkDisallowed { .. } => "REMOTE_ENVELOPE_SYMLINK_DISALLOWED",
            Self::SymlinkEscape { .. } => "REMOTE_ENVELOPE_SYMLINK_ESCAPE",
            Self::SandboxIoError { .. } => "REMOTE_ENVELOPE_SANDBOX_IO_ERROR",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::UnsignedRequestRequired => {
                "remote security envelope rejected unsigned request while signing is required"
                    .to_string()
            }
            Self::MissingKeyId => {
                "remote security envelope requires non-empty key_id when trust policy is enabled"
                    .to_string()
            }
            Self::UnsupportedRequestSignatureAlgorithm { alg } => format!(
                "remote request signing rejected unsupported signature algorithm '{alg}' (expected '{REMOTE_REQUEST_SIGNATURE_ALG_ED25519}')"
            ),
            Self::MissingRequestSignature => {
                "remote request signing required signature payload but none was provided"
                    .to_string()
            }
            Self::MalformedRequestSignature { reason } => {
                format!("remote request signature is malformed: {reason}")
            }
            Self::RequestSignatureMismatch => {
                "remote request signature verification failed (canonical request mismatch)"
                    .to_string()
            }
            Self::DisallowedAlgorithm { alg } => format!(
                "remote security envelope rejected signature algorithm '{alg}' per verification profile"
            ),
            Self::DisallowedKeySource { key_source } => format!(
                "remote security envelope rejected key source '{key_source}' per verification profile"
            ),
            Self::MissingKeySource => {
                "remote security envelope rejected request: missing key source for signature verification policy".to_string()
            }
            Self::PathTraversal { path } => format!(
                "remote security envelope rejected requested path with traversal/absolute components: '{path}'"
            ),
            Self::PathNotFound { path } => format!(
                "remote security envelope rejected requested path because the sandbox target/root was not found: '{path}'"
            ),
            Self::PathNotCanonical { path } => format!(
                "remote security envelope rejected requested path because sandbox canonicalization failed deterministically: '{path}'"
            ),
            Self::SymlinkDisallowed { path } => format!(
                "remote security envelope rejected requested path because symlink traversal is disabled by sandbox policy: '{path}'"
            ),
            Self::SymlinkEscape { path } => format!(
                "remote security envelope rejected requested path escaping sandbox root via symlink/canonicalization: '{path}'"
            ),
            Self::SandboxIoError { path, operation } => format!(
                "remote security envelope rejected requested path because sandbox validation hit an IO error during {operation}: '{path}'"
            ),
        }
    }
}

impl std::fmt::Display for SecurityEnvelopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}

impl std::error::Error for SecurityEnvelopeError {}
