use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::adl;

const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

#[derive(Debug, Clone, Serialize)]
struct CanonicalEnvelope {
    header: adl::SignedHeaderSpec,
    document: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationKeySource {
    Embedded,
    ExplicitKey,
}

impl VerificationKeySource {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "embedded" => Some(Self::Embedded),
            "explicit_key" => Some(Self::ExplicitKey),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Embedded => "embedded",
            Self::ExplicitKey => "explicit_key",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationProfile {
    pub require_signature: bool,
    pub require_key_id: bool,
    pub allowed_algs: Vec<String>,
    pub allowed_key_sources: Vec<VerificationKeySource>,
}

impl Default for VerificationProfile {
    fn default() -> Self {
        Self {
            require_signature: true,
            require_key_id: false,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec![
                VerificationKeySource::Embedded,
                VerificationKeySource::ExplicitKey,
            ],
        }
    }
}

impl VerificationProfile {
    pub fn canonicalized(mut self) -> Self {
        self.allowed_algs = self
            .allowed_algs
            .into_iter()
            .map(|v| canonical_alg(&v))
            .filter(|v| !v.is_empty())
            .collect();
        self.allowed_algs.sort();
        self.allowed_algs.dedup();
        self.allowed_key_sources.sort();
        self.allowed_key_sources.dedup();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerificationMetadata<'a> {
    pub signed: bool,
    pub key_id: Option<&'a str>,
    pub alg: Option<&'a str>,
    pub key_source: Option<VerificationKeySource>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationErrorKind {
    PolicyViolation,
    SignatureMismatch,
    MalformedSignatureMaterial,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationError {
    pub kind: VerificationErrorKind,
    pub code: &'static str,
    pub message: String,
}

impl VerificationError {
    fn policy(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: VerificationErrorKind::PolicyViolation,
            code,
            message: message.into(),
        }
    }

    fn malformed(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: VerificationErrorKind::MalformedSignatureMaterial,
            code,
            message: message.into(),
        }
    }

    fn mismatch(message: impl Into<String>) -> Self {
        Self {
            kind: VerificationErrorKind::SignatureMismatch,
            code: "SIGNATURE_MISMATCH",
            message: message.into(),
        }
    }
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for VerificationError {}

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
    verify_doc_with_profile(doc, public_key_path, &VerificationProfile::default())
        .map_err(|err| anyhow!("{err}"))
}

pub fn verify_doc_with_profile(
    doc: &adl::AdlDoc,
    public_key_path: Option<&Path>,
    profile: &VerificationProfile,
) -> std::result::Result<(), VerificationError> {
    let profile = profile.clone().canonicalized();
    let key_source = if public_key_path.is_some() {
        Some(VerificationKeySource::ExplicitKey)
    } else if doc
        .signature
        .as_ref()
        .and_then(|sig| sig.public_key_b64.as_ref())
        .is_some()
    {
        Some(VerificationKeySource::Embedded)
    } else {
        None
    };
    let metadata = VerificationMetadata {
        signed: doc.signature.is_some(),
        key_id: doc.signature.as_ref().map(|sig| sig.key_id.as_str()),
        alg: doc.signature.as_ref().map(|sig| sig.alg.as_str()),
        key_source,
    };
    enforce_verification_profile(&metadata, &profile)?;

    let sig = doc.signature.as_ref().ok_or_else(|| {
        VerificationError::policy(
            "SIGN_POLICY_UNSIGNED_REQUIRED",
            "workflow is unsigned and verification profile requires signature",
        )
    })?;

    let public = if let Some(path) = public_key_path {
        load_verifying_key(path).map_err(|err| {
            VerificationError::malformed(
                "SIGN_MALFORMED_PUBLIC_KEY",
                format!(
                    "failed to load explicit public key '{}': {err:#}",
                    path.display()
                ),
            )
        })?
    } else {
        let b64 = sig.public_key_b64.as_deref().ok_or_else(|| {
            VerificationError::policy(
                "SIGN_POLICY_MISSING_KEY_SOURCE",
                "signature.public_key_b64 missing and no explicit key provided",
            )
        })?;
        decode_verifying_key_b64(b64).map_err(|err| {
            VerificationError::malformed(
                "SIGN_MALFORMED_PUBLIC_KEY",
                format!("invalid embedded public key: {err:#}"),
            )
        })?
    };

    let bytes = canonical_bytes(doc, &sig.signed_header).map_err(|err| {
        VerificationError::malformed(
            "SIGN_MALFORMED_CANONICAL_ENVELOPE",
            format!("failed to canonicalize signed envelope: {err:#}"),
        )
    })?;
    let sig_bytes = B64.decode(sig.sig_b64.as_bytes()).map_err(|_| {
        VerificationError::malformed(
            "SIGN_MALFORMED_SIGNATURE",
            "invalid signature.sig_b64 base64",
        )
    })?;
    let signature = Signature::from_slice(&sig_bytes).map_err(|_| {
        VerificationError::malformed(
            "SIGN_MALFORMED_SIGNATURE",
            "invalid ed25519 signature bytes",
        )
    })?;
    public.verify(&bytes, &signature).map_err(|err| {
        VerificationError::mismatch(format!("signature verification failed: {err}"))
    })?;
    Ok(())
}

pub fn enforce_verification_profile(
    metadata: &VerificationMetadata<'_>,
    profile: &VerificationProfile,
) -> std::result::Result<(), VerificationError> {
    let profile = profile.clone().canonicalized();
    if profile.require_signature && !metadata.signed {
        return Err(VerificationError::policy(
            "SIGN_POLICY_UNSIGNED_REQUIRED",
            "workflow is unsigned and verification profile requires signature",
        ));
    }
    if !metadata.signed {
        return Ok(());
    }
    if profile.require_key_id
        && metadata
            .key_id
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .is_none()
    {
        return Err(VerificationError::policy(
            "SIGN_POLICY_MISSING_KEY_ID",
            "verification profile requires non-empty signature.key_id",
        ));
    }

    let alg = canonical_alg(metadata.alg.unwrap_or_default());
    if alg.is_empty() {
        return Err(VerificationError::malformed(
            "SIGN_MALFORMED_ALGORITHM",
            "signature.alg must not be empty",
        ));
    }
    if !profile.allowed_algs.is_empty() && !profile.allowed_algs.iter().any(|v| v == &alg) {
        return Err(VerificationError::policy(
            "SIGN_POLICY_DISALLOWED_ALGORITHM",
            format!("signature.alg '{alg}' is not permitted by verification profile"),
        ));
    }

    let key_source = metadata.key_source.ok_or_else(|| {
        VerificationError::policy(
            "SIGN_POLICY_MISSING_KEY_SOURCE",
            "no permitted key source available for signature verification",
        )
    })?;
    if !profile.allowed_key_sources.is_empty() && !profile.allowed_key_sources.contains(&key_source)
    {
        return Err(VerificationError::policy(
            "SIGN_POLICY_DISALLOWED_KEY_SOURCE",
            format!(
                "key source '{}' is not permitted by verification profile",
                key_source.as_str()
            ),
        ));
    }
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

fn canonical_alg(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
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
    use ed25519_dalek::SigningKey;

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

    fn signed_doc_and_pubkey() -> (adl::AdlDoc, String) {
        let mut doc = sample_doc();
        let signing = SigningKey::from_bytes(&[7_u8; 32]);
        let pub_b64 = B64.encode(signing.verifying_key().to_bytes());
        let header = default_signed_header(&doc);
        let bytes = canonical_bytes(&doc, &header).expect("canonical bytes");
        let sig = signing.sign(&bytes);
        doc.signature = Some(adl::SignatureSpec {
            alg: "ed25519".to_string(),
            key_id: "dev-local".to_string(),
            public_key_b64: Some(pub_b64.clone()),
            sig_b64: B64.encode(sig.to_bytes()),
            signed_header: header,
        });
        (doc, pub_b64)
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

    #[test]
    fn profile_rejects_missing_key_id_when_required() {
        let mut doc = sample_doc();
        doc.signature = Some(adl::SignatureSpec {
            alg: "ed25519".to_string(),
            key_id: "".to_string(),
            public_key_b64: Some("ZmFrZQ==".to_string()),
            sig_b64: "c2ln".to_string(),
            signed_header: default_signed_header(&doc),
        });
        let profile = VerificationProfile {
            require_signature: true,
            require_key_id: true,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec![VerificationKeySource::Embedded],
        };
        let err = verify_doc_with_profile(&doc, None, &profile).expect_err("missing key_id");
        assert_eq!(err.kind, VerificationErrorKind::PolicyViolation);
        assert_eq!(err.code, "SIGN_POLICY_MISSING_KEY_ID");
    }

    #[test]
    fn profile_rejects_disallowed_algorithm() {
        let mut doc = sample_doc();
        doc.signature = Some(adl::SignatureSpec {
            alg: "rsa".to_string(),
            key_id: "dev-local".to_string(),
            public_key_b64: Some("ZmFrZQ==".to_string()),
            sig_b64: "c2ln".to_string(),
            signed_header: default_signed_header(&doc),
        });
        let profile = VerificationProfile {
            require_signature: true,
            require_key_id: false,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec![VerificationKeySource::Embedded],
        };
        let err = verify_doc_with_profile(&doc, None, &profile).expect_err("disallowed alg");
        assert_eq!(err.kind, VerificationErrorKind::PolicyViolation);
        assert_eq!(err.code, "SIGN_POLICY_DISALLOWED_ALGORITHM");
    }

    #[test]
    fn profile_rejects_disallowed_key_source() {
        let (doc, _pub_b64) = signed_doc_and_pubkey();
        let profile = VerificationProfile {
            require_signature: true,
            require_key_id: false,
            allowed_algs: vec!["ed25519".to_string()],
            allowed_key_sources: vec![VerificationKeySource::ExplicitKey],
        };
        let err = verify_doc_with_profile(&doc, None, &profile).expect_err("disallowed source");
        assert_eq!(err.kind, VerificationErrorKind::PolicyViolation);
        assert_eq!(err.code, "SIGN_POLICY_DISALLOWED_KEY_SOURCE");
    }

    #[test]
    fn signature_mismatch_is_distinct_from_policy_violation() {
        let (mut doc, _pub_b64) = signed_doc_and_pubkey();
        doc.tasks.get_mut("t1").expect("task").prompt.user = Some("tampered".to_string());
        let err = verify_doc_with_profile(&doc, None, &VerificationProfile::default())
            .expect_err("tamper should fail");
        assert_eq!(err.kind, VerificationErrorKind::SignatureMismatch);
        assert_eq!(err.code, "SIGNATURE_MISMATCH");
    }
}
