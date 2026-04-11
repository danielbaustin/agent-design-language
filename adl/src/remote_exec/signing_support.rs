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
