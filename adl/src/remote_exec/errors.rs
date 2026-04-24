//! Error surfaces and stability classifications for remote execution paths.
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

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Error};

    use super::{
        retryability, stable_failure_kind, RemoteExecuteClientError, RemoteExecuteClientErrorKind,
        SecurityEnvelopeError,
    };

    #[test]
    fn security_envelope_error_code_and_message_cover_policy_variants() {
        let unsupported = SecurityEnvelopeError::UnsupportedRequestSignatureAlgorithm {
            alg: "rsa".to_string(),
        };
        assert_eq!(
            unsupported.code(),
            "REMOTE_REQUEST_SIGNATURE_UNSUPPORTED_ALGORITHM"
        );
        assert!(unsupported
            .message()
            .contains("unsupported signature algorithm 'rsa'"));

        let disallowed_alg = SecurityEnvelopeError::DisallowedAlgorithm {
            alg: "ed448".to_string(),
        };
        assert_eq!(
            disallowed_alg.code(),
            "REMOTE_ENVELOPE_DISALLOWED_ALGORITHM"
        );
        assert!(disallowed_alg.message().contains("ed448"));

        let disallowed_source = SecurityEnvelopeError::DisallowedKeySource {
            key_source: "vault".to_string(),
        };
        assert_eq!(
            disallowed_source.code(),
            "REMOTE_ENVELOPE_DISALLOWED_KEY_SOURCE"
        );
        assert!(disallowed_source.message().contains("vault"));

        let missing_source = SecurityEnvelopeError::MissingKeySource;
        assert_eq!(missing_source.code(), "REMOTE_ENVELOPE_MISSING_KEY_SOURCE");
        assert!(missing_source.message().contains("missing key source"));
    }

    #[test]
    fn security_envelope_error_code_and_message_cover_path_variants() {
        let traversal = SecurityEnvelopeError::PathTraversal {
            path: "../etc/passwd".to_string(),
        };
        assert_eq!(traversal.code(), "REMOTE_ENVELOPE_PATH_TRAVERSAL");
        assert!(traversal.message().contains("../etc/passwd"));

        let not_found = SecurityEnvelopeError::PathNotFound {
            path: "missing".to_string(),
        };
        assert_eq!(not_found.code(), "REMOTE_ENVELOPE_PATH_NOT_FOUND");
        assert!(not_found
            .message()
            .contains("sandbox target/root was not found"));

        let not_canonical = SecurityEnvelopeError::PathNotCanonical {
            path: "foo".to_string(),
        };
        assert_eq!(not_canonical.code(), "REMOTE_ENVELOPE_PATH_NOT_CANONICAL");
        assert!(not_canonical.message().contains("canonicalization failed"));

        let symlink_disallowed = SecurityEnvelopeError::SymlinkDisallowed {
            path: "link".to_string(),
        };
        assert_eq!(
            symlink_disallowed.code(),
            "REMOTE_ENVELOPE_SYMLINK_DISALLOWED"
        );
        assert!(symlink_disallowed
            .message()
            .contains("symlink traversal is disabled"));

        let symlink_escape = SecurityEnvelopeError::SymlinkEscape {
            path: "escape".to_string(),
        };
        assert_eq!(symlink_escape.code(), "REMOTE_ENVELOPE_SYMLINK_ESCAPE");
        assert!(symlink_escape.message().contains("escaping sandbox root"));

        let io_error = SecurityEnvelopeError::SandboxIoError {
            path: "target".to_string(),
            operation: "canonicalize",
        };
        assert_eq!(io_error.code(), "REMOTE_ENVELOPE_SANDBOX_IO_ERROR");
        assert!(io_error.message().contains("canonicalize"));
    }

    #[test]
    fn retryability_distinguishes_remote_error_kinds() {
        let timeout = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::Timeout,
            "REMOTE_TIMEOUT",
            "timed out",
        ));
        assert_eq!(retryability(&timeout), Some(true));

        let schema = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::SchemaViolation,
            "REMOTE_SCHEMA_VIOLATION",
            "bad payload",
        ));
        assert_eq!(retryability(&schema), Some(false));

        let retryable_remote = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::RemoteExecution,
            "REMOTE_PROVIDER_FAILURE",
            "backend error",
        ));
        assert_eq!(retryability(&retryable_remote), Some(true));

        let policy_remote = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::RemoteExecution,
            "SIGN_POLICY_MISSING_KEY_SOURCE",
            "policy denied",
        ));
        assert_eq!(retryability(&policy_remote), Some(false));

        let envelope_remote = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::RemoteExecution,
            "REMOTE_ENVELOPE_PATH_TRAVERSAL",
            "path rejected",
        ));
        assert_eq!(retryability(&envelope_remote), Some(false));

        let policy = Error::new(SecurityEnvelopeError::MissingKeySource);
        assert_eq!(retryability(&policy), Some(false));

        let unrelated = anyhow!("plain outer error");
        assert_eq!(retryability(&unrelated), None);
    }

    #[test]
    fn stable_failure_kind_maps_policy_transport_and_remote_variants() {
        let policy = Error::new(SecurityEnvelopeError::UnsignedRequestRequired);
        assert_eq!(stable_failure_kind(&policy), Some("policy_denied"));

        let timeout = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::Timeout,
            "REMOTE_TIMEOUT",
            "timed out",
        ));
        assert_eq!(stable_failure_kind(&timeout), Some("timeout"));

        let schema = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::SchemaViolation,
            "REMOTE_SCHEMA_VIOLATION",
            "schema",
        ));
        assert_eq!(stable_failure_kind(&schema), Some("schema_error"));

        let io = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::BadStatus,
            "REMOTE_BAD_STATUS",
            "bad status",
        ));
        assert_eq!(stable_failure_kind(&io), Some("io_error"));

        let provider = Error::new(RemoteExecuteClientError::new(
            RemoteExecuteClientErrorKind::RemoteExecution,
            "REMOTE_PROVIDER_FAILURE",
            "backend",
        ));
        assert_eq!(stable_failure_kind(&provider), Some("provider_error"));

        let unrelated = anyhow!("plain outer error");
        assert_eq!(stable_failure_kind(&unrelated), None);
    }
}
