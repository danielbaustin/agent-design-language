use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::adl;

const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

#[derive(Debug, Clone, Serialize)]
struct CanonicalEnvelope {
    header: adl::SignedHeaderSpec,
    document: Value,
}

pub fn keygen(out_dir: &Path) -> Result<(PathBuf, PathBuf)> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create key directory '{}'", out_dir.display()))?;
    let mut rng = OsRng;
    let mut secret = [0_u8; 32];
    rng.fill_bytes(&mut secret);
    let signing = SigningKey::from_bytes(&secret);
    let verifying = signing.verifying_key();
    let priv_path = out_dir.join("ed25519-private.b64");
    let pub_path = out_dir.join("ed25519-public.b64");
    fs::write(&priv_path, B64.encode(signing.to_bytes()))
        .with_context(|| format!("failed to write '{}'", priv_path.display()))?;
    fs::write(&pub_path, B64.encode(verifying.to_bytes()))
        .with_context(|| format!("failed to write '{}'", pub_path.display()))?;
    Ok((priv_path, pub_path))
}

/// Sign an ADL file in-place (or to `--out`) using an Ed25519 private key.
///
/// The signed bytes are generated from a canonical JSON envelope containing:
/// - selected signed header fields
/// - the ADL document with the top-level `signature` field excluded
///
/// This exclusion is intentional so metadata updates to `signature.*` do not
/// recursively invalidate canonicalization.
pub fn sign_file(
    path: &Path,
    private_key_path: &Path,
    key_id: &str,
    out: Option<&Path>,
) -> Result<PathBuf> {
    let doc = load_doc(path)?;
    let signing = load_signing_key(private_key_path)?;
    let header = default_signed_header(&doc);
    let bytes = canonical_bytes(&doc, &header)?;
    let sig = signing.sign(&bytes);
    let verifying = signing.verifying_key();
    let signature = adl::SignatureSpec {
        alg: "ed25519".to_string(),
        key_id: key_id.to_string(),
        public_key_b64: Some(B64.encode(verifying.to_bytes())),
        sig_b64: B64.encode(sig.to_bytes()),
        signed_header: header,
    };

    let out_path = out
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.to_path_buf());
    let raw_yaml =
        fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))?;
    let mut raw: serde_yaml::Value =
        serde_yaml::from_str(&raw_yaml).context("failed to parse original YAML")?;
    let map = raw
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("top-level ADL YAML must be a mapping"))?;
    map.insert(
        serde_yaml::Value::String("signature".to_string()),
        serde_yaml::to_value(signature).context("failed to encode signature")?,
    );
    let yaml = serde_yaml::to_string(&raw).context("failed to serialize signed ADL")?;
    fs::write(&out_path, yaml)
        .with_context(|| format!("failed to write '{}'", out_path.display()))?;
    Ok(out_path)
}

/// Verify a signed ADL file against either:
/// - explicit `public_key_path`, or
/// - embedded `signature.public_key_b64`
///
/// Returns an error for unsigned files, unsupported algorithms, malformed keys,
/// or signature mismatch.
pub fn verify_file(path: &Path, public_key_path: Option<&Path>) -> Result<()> {
    let doc = load_doc(path)?;
    verify_doc(&doc, public_key_path)
}

/// Verify a parsed ADL document signature using Ed25519.
///
/// Security model:
/// - only `ed25519` is accepted in v0.5
/// - canonicalization is deterministic and excludes top-level `signature`
/// - this function validates integrity/authenticity, not authorization policy
pub fn verify_doc(doc: &adl::AdlDoc, public_key_path: Option<&Path>) -> Result<()> {
    let sig = doc
        .signature
        .as_ref()
        .ok_or_else(|| anyhow!("workflow is unsigned"))?;
    if !sig.alg.trim().eq_ignore_ascii_case("ed25519") {
        return Err(anyhow!(
            "unsupported signature alg '{}' (expected ed25519)",
            sig.alg
        ));
    }

    let public = if let Some(path) = public_key_path {
        load_verifying_key(path)?
    } else {
        let b64 = sig
            .public_key_b64
            .as_deref()
            .ok_or_else(|| anyhow!("signature.public_key_b64 missing and no --key provided"))?;
        decode_verifying_key_b64(b64)?
    };

    let bytes = canonical_bytes(doc, &sig.signed_header)?;
    let sig_bytes = B64
        .decode(sig.sig_b64.as_bytes())
        .context("invalid signature.sig_b64 base64")?;
    let signature = Signature::from_slice(&sig_bytes).context("invalid ed25519 signature bytes")?;
    public
        .verify(&bytes, &signature)
        .map_err(|err| anyhow!("signature verification failed: {err}"))?;
    Ok(())
}

/// Build the default signed-header fields for canonical signing.
///
/// Header fields are intentionally minimal and deterministic so the same
/// document content yields stable signed bytes across runs.
pub fn default_signed_header(doc: &adl::AdlDoc) -> adl::SignedHeaderSpec {
    let workflow_id = doc
        .run
        .workflow_ref
        .clone()
        .or_else(|| doc.run.workflow.as_ref().and_then(|w| w.id.clone()));
    adl::SignedHeaderSpec {
        adl_version: doc.version.clone(),
        workflow_id,
    }
}

/// Serialize deterministic canonical bytes for signing and verification.
///
/// Canonicalization recursively sorts object keys and strips top-level
/// `signature` to prevent self-referential signing.
pub fn canonical_bytes(doc: &adl::AdlDoc, header: &adl::SignedHeaderSpec) -> Result<Vec<u8>> {
    let mut unsigned = doc.clone();
    unsigned.signature = None;
    let mut json = serde_json::to_value(unsigned).context("failed to convert document to JSON")?;
    sort_value(&mut json);
    let envelope = CanonicalEnvelope {
        header: header.clone(),
        document: json,
    };
    serde_json::to_vec(&envelope).context("failed to serialize canonical envelope")
}

fn sort_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut sorted = BTreeMap::new();
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

fn load_doc(path: &Path) -> Result<adl::AdlDoc> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))?;
    serde_yaml::from_str(&content).with_context(|| format!("failed to parse '{}'", path.display()))
}

fn load_signing_key(path: &Path) -> Result<SigningKey> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read private key '{}'", path.display()))?;
    let bytes = B64
        .decode(raw.trim().as_bytes())
        .with_context(|| format!("invalid base64 private key '{}'", path.display()))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| anyhow!("private key must be exactly 32 bytes"))?;
    Ok(SigningKey::from_bytes(&arr))
}

fn load_verifying_key(path: &Path) -> Result<VerifyingKey> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read public key '{}'", path.display()))?;
    decode_verifying_key_b64(raw.trim())
}

fn decode_verifying_key_b64(raw_b64: &str) -> Result<VerifyingKey> {
    let bytes = B64
        .decode(raw_b64.as_bytes())
        .context("invalid base64 public key")?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| anyhow!("public key must be exactly 32 bytes"))?;
    VerifyingKey::from_bytes(&arr).context("invalid ed25519 public key")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_doc() -> adl::AdlDoc {
        let yaml = r#"
version: "0.5"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "phi4-mini"
tasks:
  t1:
    prompt:
      user: "hello"
run:
  name: "demo"
  workflow:
    kind: sequential
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
"#;
        serde_yaml::from_str(yaml).expect("sample yaml")
    }

    #[test]
    fn canonical_bytes_are_deterministic() {
        let doc = sample_doc();
        let header = default_signed_header(&doc);
        let a = canonical_bytes(&doc, &header).expect("canonical bytes");
        let b = canonical_bytes(&doc, &header).expect("canonical bytes");
        assert_eq!(a, b);
    }

    #[test]
    fn canonicalization_excludes_top_level_signature() {
        let mut doc = sample_doc();
        let header = default_signed_header(&doc);
        let unsigned = canonical_bytes(&doc, &header).expect("canonical bytes");

        doc.signature = Some(adl::SignatureSpec {
            alg: "ed25519".to_string(),
            key_id: "dev-local".to_string(),
            public_key_b64: Some("ZmFrZQ==".to_string()),
            sig_b64: "c2ln".to_string(),
            signed_header: header.clone(),
        });
        let with_sig = canonical_bytes(&doc, &header).expect("canonical bytes");
        assert_eq!(unsigned, with_sig);
    }
}
