//! Canonicalization and signing helpers for remote execute requests.
//!
//! These helpers centralize deterministic request encoding and optional signing
//! from environment-provided private keys.
use anyhow::{anyhow, Context, Result};
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde_json::{Map, Value};

use super::{
    ExecuteRequest, ExecuteSecurityEnvelope, RemoteRequestSignatureV1, SecurityEnvelopeError, B64,
    REMOTE_REQUEST_SIGNATURE_ALG_ED25519, REMOTE_REQUEST_SIGNATURE_SCHEMA_V1,
};

fn sort_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut sorted = std::collections::BTreeMap::new();
            for (k, mut v) in std::mem::take(map) {
                sort_value(&mut v);
                sorted.insert(k, v);
            }
            let mut out = Map::new();
            for (k, v) in sorted {
                out.insert(k, v);
            }
            *map = out;
        }
        Value::Array(items) => {
            for item in items {
                sort_value(item);
            }
        }
        _ => {}
    }
}

pub fn canonical_request_bytes(req: &ExecuteRequest) -> Result<Vec<u8>> {
    let mut canonical = req.clone();
    if let Some(sec) = canonical.security.as_mut() {
        sec.request_signature = None;
    }
    let mut value = serde_json::to_value(canonical)
        .context("failed to convert execute request to canonical JSON")?;
    sort_value(&mut value);
    serde_json::to_vec(&value).context("failed to serialize canonical execute request")
}

pub fn sign_execute_request_v1(
    req: &ExecuteRequest,
    private_key_b64: &str,
    key_id: Option<&str>,
) -> Result<RemoteRequestSignatureV1> {
    let key_bytes = B64
        .decode(private_key_b64.trim().as_bytes())
        .context("invalid base64 private key for remote request signing")?;
    let key_arr: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| anyhow!("remote request private key must be exactly 32 bytes"))?;
    let signing = SigningKey::from_bytes(&key_arr);
    let canonical = canonical_request_bytes(req)?;
    let sig = signing.sign(&canonical);
    Ok(RemoteRequestSignatureV1 {
        schema_version: REMOTE_REQUEST_SIGNATURE_SCHEMA_V1.to_string(),
        alg: REMOTE_REQUEST_SIGNATURE_ALG_ED25519.to_string(),
        key_id: key_id.map(|v| v.to_string()),
        public_key_b64: Some(B64.encode(signing.verifying_key().to_bytes())),
        sig_b64: B64.encode(sig.to_bytes()),
    })
}

pub(super) fn attach_request_signature(
    req: &mut ExecuteRequest,
    private_key_b64: &str,
    key_id: Option<&str>,
) -> Result<()> {
    {
        let env = req
            .security
            .get_or_insert_with(ExecuteSecurityEnvelope::default);
        env.signed = true;
        if let Some(id) = key_id {
            env.key_id = Some(id.to_string());
        }
        env.signature_alg = Some(REMOTE_REQUEST_SIGNATURE_ALG_ED25519.to_string());
        env.key_source = Some("embedded".to_string());
    }

    let signature = sign_execute_request_v1(req, private_key_b64, key_id)?;
    let env = req
        .security
        .get_or_insert_with(ExecuteSecurityEnvelope::default);
    env.request_signature = Some(signature);
    Ok(())
}

pub fn maybe_attach_request_signature_from_env(req: &mut ExecuteRequest) -> Result<()> {
    let require_signature = req
        .security
        .as_ref()
        .map(|env| env.require_signature)
        .unwrap_or(false);
    let private_key = std::env::var("ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    let key_id = std::env::var("ADL_REMOTE_REQUEST_SIGNING_KEY_ID")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .or_else(|| req.security.as_ref().and_then(|env| env.key_id.clone()));

    match (require_signature, private_key) {
        (true, None) => {
            return Err(anyhow!(
                "REMOTE_REQUEST_SIGNATURE_MISSING: signing is required but ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64 is not set"
            ));
        }
        (_, None) => return Ok(()),
        (_, Some(private_key_b64)) => {
            attach_request_signature(req, &private_key_b64, key_id.as_deref())?;
        }
    }
    Ok(())
}

pub(super) fn verify_execute_request_signature_v1(
    req: &ExecuteRequest,
    sig: &RemoteRequestSignatureV1,
) -> std::result::Result<(), SecurityEnvelopeError> {
    if sig.schema_version.trim() != REMOTE_REQUEST_SIGNATURE_SCHEMA_V1 {
        return Err(SecurityEnvelopeError::MalformedRequestSignature {
            reason: format!(
                "unexpected schema_version '{}' (expected '{}')",
                sig.schema_version, REMOTE_REQUEST_SIGNATURE_SCHEMA_V1
            ),
        });
    }
    if !sig
        .alg
        .trim()
        .eq_ignore_ascii_case(REMOTE_REQUEST_SIGNATURE_ALG_ED25519)
    {
        return Err(
            SecurityEnvelopeError::UnsupportedRequestSignatureAlgorithm {
                alg: sig.alg.clone(),
            },
        );
    }
    let pub_b64 = sig.public_key_b64.as_deref().ok_or_else(|| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "missing public_key_b64".to_string(),
        }
    })?;
    let pub_bytes = B64.decode(pub_b64.as_bytes()).map_err(|_| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "invalid base64 public_key_b64".to_string(),
        }
    })?;
    let pub_arr: [u8; 32] =
        pub_bytes
            .try_into()
            .map_err(|_| SecurityEnvelopeError::MalformedRequestSignature {
                reason: "public key must be exactly 32 bytes".to_string(),
            })?;
    let public = VerifyingKey::from_bytes(&pub_arr).map_err(|_| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "invalid ed25519 public key".to_string(),
        }
    })?;
    let sig_bytes = B64.decode(sig.sig_b64.as_bytes()).map_err(|_| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "invalid base64 sig_b64".to_string(),
        }
    })?;
    let parsed_sig = Signature::from_slice(&sig_bytes).map_err(|_| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: "invalid ed25519 signature bytes".to_string(),
        }
    })?;
    let canonical = canonical_request_bytes(req).map_err(|err| {
        SecurityEnvelopeError::MalformedRequestSignature {
            reason: format!("failed to canonicalize request: {err:#}"),
        }
    })?;
    public
        .verify(&canonical, &parsed_sig)
        .map_err(|_| SecurityEnvelopeError::RequestSignatureMismatch)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    use base64::Engine;
    use ed25519_dalek::SigningKey;

    use super::{
        attach_request_signature, canonical_request_bytes, maybe_attach_request_signature_from_env,
        sign_execute_request_v1, verify_execute_request_signature_v1, ExecuteRequest,
        ExecuteSecurityEnvelope, RemoteRequestSignatureV1, SecurityEnvelopeError, B64,
        REMOTE_REQUEST_SIGNATURE_ALG_ED25519, REMOTE_REQUEST_SIGNATURE_SCHEMA_V1,
    };
    use crate::adl::ProviderSpec;
    use crate::remote_exec::{ExecuteInputsPayload, ExecuteStepPayload};

    fn base_request() -> ExecuteRequest {
        ExecuteRequest {
            protocol_version: "0.1".to_string(),
            run_id: "run-1".to_string(),
            workflow_id: "wf-1".to_string(),
            step_id: "step-1".to_string(),
            step: ExecuteStepPayload {
                kind: "agent".to_string(),
                provider: "provider".to_string(),
                prompt: "hello".to_string(),
                tools: vec!["web".to_string()],
                provider_spec: ProviderSpec {
                    id: None,
                    profile: None,
                    kind: "openai".to_string(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::new(),
                },
                model_override: None,
            },
            inputs: ExecuteInputsPayload::default(),
            timeout_ms: 1_000,
            security: None,
        }
    }

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn canonical_request_bytes_omit_signature_and_sort_keys() {
        let mut req = base_request();
        req.inputs.inputs.insert("b".to_string(), "2".to_string());
        req.inputs.inputs.insert("a".to_string(), "1".to_string());
        req.security = Some(ExecuteSecurityEnvelope {
            signed: true,
            signature_alg: Some("ed25519".to_string()),
            request_signature: Some(RemoteRequestSignatureV1 {
                schema_version: REMOTE_REQUEST_SIGNATURE_SCHEMA_V1.to_string(),
                alg: REMOTE_REQUEST_SIGNATURE_ALG_ED25519.to_string(),
                key_id: Some("key-1".to_string()),
                public_key_b64: Some("pub".to_string()),
                sig_b64: "sig".to_string(),
            }),
            ..ExecuteSecurityEnvelope::default()
        });

        let canonical = canonical_request_bytes(&req).unwrap();
        let rendered = String::from_utf8(canonical.clone()).unwrap();
        assert!(rendered.contains("\"a\":\"1\",\"b\":\"2\""));
        let parsed: serde_json::Value = serde_json::from_slice(&canonical).unwrap();
        assert_eq!(
            parsed["security"]["request_signature"],
            serde_json::Value::Null
        );
    }

    #[test]
    fn maybe_attach_request_signature_from_env_requires_key_when_policy_demands_it() {
        let _guard = env_lock().lock().unwrap();
        std::env::remove_var("ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64");
        std::env::remove_var("ADL_REMOTE_REQUEST_SIGNING_KEY_ID");

        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            ..ExecuteSecurityEnvelope::default()
        });

        let err = maybe_attach_request_signature_from_env(&mut req).unwrap_err();
        assert!(err.to_string().contains("REMOTE_REQUEST_SIGNATURE_MISSING"));
    }

    #[test]
    fn maybe_attach_request_signature_from_env_attaches_signature_and_env_metadata() {
        let _guard = env_lock().lock().unwrap();
        let signing = SigningKey::from_bytes(&[7_u8; 32]);
        std::env::set_var(
            "ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64",
            B64.encode(signing.to_bytes()),
        );
        std::env::set_var("ADL_REMOTE_REQUEST_SIGNING_KEY_ID", "env-key");

        let mut req = base_request();
        req.security = Some(ExecuteSecurityEnvelope {
            require_signature: true,
            ..ExecuteSecurityEnvelope::default()
        });

        maybe_attach_request_signature_from_env(&mut req).unwrap();
        let security = req.security.as_ref().unwrap();
        assert!(security.signed);
        assert_eq!(security.key_id.as_deref(), Some("env-key"));
        assert_eq!(
            security.signature_alg.as_deref(),
            Some(REMOTE_REQUEST_SIGNATURE_ALG_ED25519)
        );
        assert_eq!(security.key_source.as_deref(), Some("embedded"));
        assert!(security.request_signature.is_some());

        std::env::remove_var("ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64");
        std::env::remove_var("ADL_REMOTE_REQUEST_SIGNING_KEY_ID");
    }

    #[test]
    fn sign_and_verify_round_trip_succeeds() {
        let mut req = base_request();
        let private_key_b64 = B64.encode([9_u8; 32]);
        attach_request_signature(&mut req, &private_key_b64, Some("key-9")).unwrap();

        let signature = req
            .security
            .as_ref()
            .and_then(|security| security.request_signature.clone())
            .unwrap();
        verify_execute_request_signature_v1(&req, &signature).unwrap();
    }

    #[test]
    fn verify_execute_request_signature_rejects_malformed_variants() {
        let req = base_request();
        let private_key_b64 = B64.encode([11_u8; 32]);
        let good = sign_execute_request_v1(&req, &private_key_b64, Some("key-11")).unwrap();

        let wrong_schema = RemoteRequestSignatureV1 {
            schema_version: "wrong".to_string(),
            ..good.clone()
        };
        assert!(matches!(
            verify_execute_request_signature_v1(&req, &wrong_schema),
            Err(SecurityEnvelopeError::MalformedRequestSignature { .. })
        ));

        let wrong_alg = RemoteRequestSignatureV1 {
            alg: "rsa".to_string(),
            ..good.clone()
        };
        assert!(matches!(
            verify_execute_request_signature_v1(&req, &wrong_alg),
            Err(SecurityEnvelopeError::UnsupportedRequestSignatureAlgorithm { .. })
        ));

        let missing_pub = RemoteRequestSignatureV1 {
            public_key_b64: None,
            ..good.clone()
        };
        assert!(matches!(
            verify_execute_request_signature_v1(&req, &missing_pub),
            Err(SecurityEnvelopeError::MalformedRequestSignature { .. })
        ));

        let bad_pub_b64 = RemoteRequestSignatureV1 {
            public_key_b64: Some("@@@".to_string()),
            ..good.clone()
        };
        assert!(matches!(
            verify_execute_request_signature_v1(&req, &bad_pub_b64),
            Err(SecurityEnvelopeError::MalformedRequestSignature { .. })
        ));

        let bad_sig_b64 = RemoteRequestSignatureV1 {
            sig_b64: "@@@".to_string(),
            ..good
        };
        assert!(matches!(
            verify_execute_request_signature_v1(&req, &bad_sig_b64),
            Err(SecurityEnvelopeError::MalformedRequestSignature { .. })
        ));
    }
}
