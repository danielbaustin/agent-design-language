//! Security-envelope validation for remote execution requests.
//!
//! This module validates signing/verification flags, path sandbox policy, and
//! required key source constraints for inbound execute requests.
use crate::sandbox;
use crate::signing;

use super::signing_support::verify_execute_request_signature_v1;
use super::{ExecuteRequest, SecurityEnvelopeError};

/// Validate the request security envelope before execution.
pub fn validate_security_envelope(
    req: &ExecuteRequest,
) -> std::result::Result<(), SecurityEnvelopeError> {
    let env = req.security.as_ref().cloned().unwrap_or_default();
    let signature_payload = env.request_signature.clone();
    if env.require_signature && signature_payload.is_none() {
        return Err(SecurityEnvelopeError::MissingRequestSignature);
    }

    let derived_alg = signature_payload
        .as_ref()
        .map(|sig| sig.alg.clone())
        .or_else(|| env.signature_alg.clone());
    let derived_key_id = signature_payload
        .as_ref()
        .and_then(|sig| sig.key_id.clone())
        .or_else(|| env.key_id.clone());
    let derived_key_source = if signature_payload
        .as_ref()
        .and_then(|sig| sig.public_key_b64.as_ref())
        .is_some()
    {
        Some(signing::VerificationKeySource::Embedded)
    } else {
        match env.key_source.as_deref() {
            Some(raw) => match signing::VerificationKeySource::parse(raw) {
                Some(source) => Some(source),
                None => {
                    return Err(SecurityEnvelopeError::DisallowedKeySource {
                        key_source: raw.to_string(),
                    });
                }
            },
            None => None,
        }
    };
    let requested_algs = env
        .allowed_algs
        .iter()
        .map(|raw| raw.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let requested_key_sources = env
        .allowed_key_sources
        .iter()
        .map(|raw| {
            signing::VerificationKeySource::parse(raw).ok_or_else(|| {
                SecurityEnvelopeError::DisallowedKeySource {
                    key_source: raw.clone(),
                }
            })
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut receiver_profile = signing::VerificationProfile::default().canonicalized();
    let allowed_algs = if requested_algs.is_empty() {
        receiver_profile.allowed_algs.clone()
    } else {
        for alg in &requested_algs {
            if !receiver_profile.allowed_algs.iter().any(|base| base == alg) {
                return Err(SecurityEnvelopeError::DisallowedAlgorithm { alg: alg.clone() });
            }
        }
        requested_algs
    };
    let allowed_key_sources = if requested_key_sources.is_empty() {
        receiver_profile.allowed_key_sources.clone()
    } else {
        for source in &requested_key_sources {
            if !receiver_profile.allowed_key_sources.contains(source) {
                return Err(SecurityEnvelopeError::DisallowedKeySource {
                    key_source: source.as_str().to_string(),
                });
            }
        }
        requested_key_sources
    };
    receiver_profile.require_signature = env.require_signature;
    receiver_profile.require_key_id = env.require_key_id;
    receiver_profile.allowed_algs = allowed_algs;
    receiver_profile.allowed_key_sources = allowed_key_sources;
    let profile = signing::VerificationProfile {
        require_signature: receiver_profile.require_signature,
        require_key_id: receiver_profile.require_key_id,
        allowed_algs: receiver_profile.allowed_algs.clone(),
        allowed_key_sources: receiver_profile.allowed_key_sources.clone(),
    };
    let metadata = signing::VerificationMetadata {
        signed: signature_payload.is_some() || env.signed,
        key_id: derived_key_id.as_deref(),
        alg: derived_alg.as_deref(),
        key_source: derived_key_source,
    };
    if let Err(err) = signing::enforce_verification_profile(&metadata, &profile) {
        let mapped = match err.code {
            "SIGN_POLICY_UNSIGNED_REQUIRED" => SecurityEnvelopeError::UnsignedRequestRequired,
            "SIGN_POLICY_MISSING_KEY_ID" => SecurityEnvelopeError::MissingKeyId,
            "SIGN_POLICY_DISALLOWED_ALGORITHM" => SecurityEnvelopeError::DisallowedAlgorithm {
                alg: derived_alg.as_deref().unwrap_or("<unknown>").to_string(),
            },
            "SIGN_POLICY_DISALLOWED_KEY_SOURCE" => SecurityEnvelopeError::DisallowedKeySource {
                key_source: env
                    .key_source
                    .as_deref()
                    .or_else(|| derived_key_source.as_ref().map(|source| source.as_str()))
                    .unwrap_or("<unknown>")
                    .to_string(),
            },
            "SIGN_POLICY_MISSING_KEY_SOURCE" => SecurityEnvelopeError::MissingKeySource,
            _ => SecurityEnvelopeError::MissingKeySource,
        };
        return Err(mapped);
    }
    if let Some(sig) = signature_payload.as_ref() {
        verify_execute_request_signature_v1(req, sig)?;
    }

    if env.requested_paths.is_empty() {
        return Ok(());
    }

    let sandbox_root = env.sandbox_root.as_deref().unwrap_or(".");
    for rel in &env.requested_paths {
        let resolved = sandbox::resolve_relative_path_for_write_within_root(
            std::path::Path::new(sandbox_root),
            std::path::Path::new(rel),
        );
        if let Err(err) = resolved {
            return Err(map_sandbox_path_error(rel, &err));
        }
    }
    Ok(())
}

pub(super) fn map_sandbox_path_error(
    requested_path: &str,
    err: &sandbox::SandboxPathError,
) -> SecurityEnvelopeError {
    match err {
        sandbox::SandboxPathError::PathDenied { .. } => SecurityEnvelopeError::PathTraversal {
            path: requested_path.to_string(),
        },
        sandbox::SandboxPathError::PathNotFound { .. } => SecurityEnvelopeError::PathNotFound {
            path: requested_path.to_string(),
        },
        sandbox::SandboxPathError::PathNotCanonical { .. } => {
            SecurityEnvelopeError::PathNotCanonical {
                path: requested_path.to_string(),
            }
        }
        sandbox::SandboxPathError::SymlinkDisallowed { .. } => {
            SecurityEnvelopeError::SymlinkDisallowed {
                path: requested_path.to_string(),
            }
        }
        sandbox::SandboxPathError::EscapeAttempt { .. } => SecurityEnvelopeError::SymlinkEscape {
            path: requested_path.to_string(),
        },
        sandbox::SandboxPathError::IoError { operation, .. } => {
            SecurityEnvelopeError::SandboxIoError {
                path: requested_path.to_string(),
                operation,
            }
        }
    }
}
