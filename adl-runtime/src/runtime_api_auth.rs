use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use rand::RngCore;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

pub const CSM_RUNTIME_API_AUTH_SCHEMA: &str = "adl.csm.runtime_api.auth.v1";
pub const CSM_RUNTIME_API_AUTH_FILE: &str = "runtime_api_auth.json";
pub const CSM_RUNTIME_API_AUTH_EVENTS_FILE: &str = "runtime_api_auth_events.jsonl";
pub const CSM_RUNTIME_API_CREDENTIAL_TTL_SECS: u64 = 24 * 60 * 60;
pub const CSM_RUNTIME_API_GATEWAY_IDENTITY_SCHEMA: &str = "adl.csm.runtime_api.gateway_identity.v1";
pub const CSM_RUNTIME_API_GATEWAY_IDENTITY_AUDIENCE: &str = "csm-runtime-api";
pub const CSM_RUNTIME_API_GATEWAY_IDENTITY_MAX_TTL_SECS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeApiGatewayIdentityClaims {
    pub schema: String,
    pub issuer: String,
    pub principal: String,
    pub audience: String,
    pub authorization_scopes: Vec<String>,
    pub issued_at_epoch_secs: u64,
    pub expires_at_epoch_secs: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VerifiedRuntimeApiGatewayIdentity {
    pub schema: &'static str,
    pub issuer: String,
    pub principal_hash: String,
    pub audience: String,
    pub authorization_scopes: Vec<String>,
    pub issued_at_epoch_secs: u64,
    pub expires_at_epoch_secs: u64,
    pub credential_material_propagated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredCredential {
    schema: String,
    generation: u64,
    token: String,
    created_at_epoch_secs: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expires_at_epoch_secs: Option<u64>,
    #[serde(default)]
    revoked: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RuntimeApiCredentialMetadata {
    pub schema: &'static str,
    pub generation: u64,
    pub fingerprint: String,
    pub created_at_epoch_secs: u64,
    pub expires_at_epoch_secs: Option<u64>,
    pub revoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeApiAuthDecision {
    Authenticated(RuntimeApiCredentialMetadata),
    Rejected {
        reason: &'static str,
        metadata: Option<RuntimeApiCredentialMetadata>,
    },
    Unavailable {
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub struct RuntimeApiCredentialStore {
    path: PathBuf,
}

impl RuntimeApiCredentialStore {
    pub fn for_state_root(state_root: &Path) -> Self {
        Self {
            path: state_root.join(CSM_RUNTIME_API_AUTH_FILE),
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn ensure(&self) -> Result<RuntimeApiCredentialMetadata, String> {
        if self.path.exists() {
            return self.load().map(|(_, metadata)| metadata);
        }
        self.write_new(1)
    }

    pub fn metadata(&self) -> Result<Option<RuntimeApiCredentialMetadata>, String> {
        if !self.path.exists() {
            return Ok(None);
        }
        self.load().map(|(_, metadata)| Some(metadata))
    }

    pub fn authorize(&self, authorization: Option<&str>) -> RuntimeApiAuthDecision {
        let (expected, metadata) = match self.load() {
            Ok(value) => value,
            Err(reason) => return RuntimeApiAuthDecision::Unavailable { reason },
        };
        if metadata.revoked {
            return RuntimeApiAuthDecision::Rejected {
                reason: "credential_revoked",
                metadata: Some(metadata),
            };
        }
        if metadata
            .expires_at_epoch_secs
            .is_some_and(|expires| now_epoch_secs().is_ok_and(|now| now >= expires))
        {
            return RuntimeApiAuthDecision::Rejected {
                reason: "credential_expired",
                metadata: Some(metadata),
            };
        }
        let Some(raw) = authorization else {
            return RuntimeApiAuthDecision::Rejected {
                reason: "missing_bearer_token",
                metadata: Some(metadata),
            };
        };
        let Some(candidate) = raw.strip_prefix("Bearer ") else {
            return RuntimeApiAuthDecision::Rejected {
                reason: "malformed_authorization",
                metadata: Some(metadata),
            };
        };
        let equal = expected
            .expose_secret()
            .as_bytes()
            .ct_eq(candidate.as_bytes())
            .into();
        if equal {
            RuntimeApiAuthDecision::Authenticated(metadata)
        } else {
            RuntimeApiAuthDecision::Rejected {
                reason: "invalid_bearer_token",
                metadata: Some(metadata),
            }
        }
    }

    pub fn with_bearer_token<T>(&self, use_token: impl FnOnce(&str) -> T) -> Result<T, String> {
        self.bearer_token()
            .map(|token| use_token(token.expose_secret()))
    }

    pub fn sign_gateway_identity(
        &self,
        claims: &RuntimeApiGatewayIdentityClaims,
    ) -> Result<(String, String), String> {
        validate_gateway_identity_claims(claims, now_epoch_secs()?)?;
        let encoded = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(claims)
                .map_err(|err| format!("serialize runtime API gateway identity: {err}"))?,
        );
        let signature =
            self.with_bearer_token(|token| gateway_identity_signature(token, &encoded))??;
        Ok((encoded, signature))
    }

    pub fn verify_gateway_identity(
        &self,
        encoded_claims: Option<&str>,
        signature: Option<&str>,
    ) -> Result<Option<VerifiedRuntimeApiGatewayIdentity>, String> {
        match (encoded_claims, signature) {
            (None, None) => return Ok(None),
            (Some(_), None) | (None, Some(_)) => {
                return Err("gateway_identity_headers_incomplete".to_string())
            }
            (Some(_), Some(_)) => {}
        }
        let encoded_claims = encoded_claims.unwrap_or_default();
        let signature = signature.unwrap_or_default();
        let expected =
            self.with_bearer_token(|token| gateway_identity_signature(token, encoded_claims))??;
        if expected.as_bytes().ct_eq(signature.as_bytes()).unwrap_u8() != 1 {
            return Err("gateway_identity_signature_invalid".to_string());
        }
        let raw = URL_SAFE_NO_PAD
            .decode(encoded_claims)
            .map_err(|_| "gateway_identity_claims_malformed".to_string())?;
        let claims: RuntimeApiGatewayIdentityClaims = serde_json::from_slice(&raw)
            .map_err(|_| "gateway_identity_claims_malformed".to_string())?;
        validate_gateway_identity_claims(&claims, now_epoch_secs()?)?;
        Ok(Some(VerifiedRuntimeApiGatewayIdentity {
            schema: CSM_RUNTIME_API_GATEWAY_IDENTITY_SCHEMA,
            issuer: claims.issuer,
            principal_hash: token_fingerprint(&claims.principal),
            audience: claims.audience,
            authorization_scopes: claims.authorization_scopes,
            issued_at_epoch_secs: claims.issued_at_epoch_secs,
            expires_at_epoch_secs: claims.expires_at_epoch_secs,
            credential_material_propagated: false,
        }))
    }

    fn bearer_token(&self) -> Result<SecretString, String> {
        self.load().and_then(|(token, metadata)| {
            if metadata.revoked {
                Err("runtime API credential is revoked".to_string())
            } else if metadata
                .expires_at_epoch_secs
                .is_some_and(|expires| now_epoch_secs().is_ok_and(|now| now >= expires))
            {
                Err("runtime API credential is expired".to_string())
            } else {
                Ok(token)
            }
        })
    }

    pub fn rotate(&self) -> Result<RuntimeApiCredentialMetadata, String> {
        let generation = self
            .load()
            .map(|(_, metadata)| metadata.generation.saturating_add(1))
            .unwrap_or(1);
        self.write_new(generation)
    }

    pub fn revoke(&self) -> Result<RuntimeApiCredentialMetadata, String> {
        let (token, metadata) = self.load()?;
        let stored = StoredCredential {
            schema: CSM_RUNTIME_API_AUTH_SCHEMA.to_string(),
            generation: metadata.generation,
            token: token.expose_secret().to_string(),
            created_at_epoch_secs: metadata.created_at_epoch_secs,
            expires_at_epoch_secs: metadata.expires_at_epoch_secs,
            revoked: true,
        };
        write_private_json_atomic(&self.path, &stored)?;
        Ok(metadata_from_stored(&stored))
    }

    fn load(&self) -> Result<(SecretString, RuntimeApiCredentialMetadata), String> {
        validate_private_file(&self.path)?;
        let raw =
            fs::read(&self.path).map_err(|err| format!("read runtime API credential: {err}"))?;
        let stored: StoredCredential = serde_json::from_slice(&raw)
            .map_err(|err| format!("parse runtime API credential: {err}"))?;
        validate_stored(&stored)?;
        let metadata = metadata_from_stored(&stored);
        Ok((SecretString::new(stored.token), metadata))
    }

    fn write_new(&self, generation: u64) -> Result<RuntimeApiCredentialMetadata, String> {
        let parent = self
            .path
            .parent()
            .ok_or_else(|| "runtime API credential path has no parent".to_string())?;
        fs::create_dir_all(parent)
            .map_err(|err| format!("create runtime API credential directory: {err}"))?;
        let mut bytes = [0_u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let created_at_epoch_secs = now_epoch_secs()?;
        let stored = StoredCredential {
            schema: CSM_RUNTIME_API_AUTH_SCHEMA.to_string(),
            generation,
            token: URL_SAFE_NO_PAD.encode(bytes),
            created_at_epoch_secs,
            expires_at_epoch_secs: Some(
                created_at_epoch_secs.saturating_add(CSM_RUNTIME_API_CREDENTIAL_TTL_SECS),
            ),
            revoked: false,
        };
        write_private_json_atomic(&self.path, &stored)?;
        Ok(metadata_from_stored(&stored))
    }
}

fn gateway_identity_signature(token: &str, encoded_claims: &str) -> Result<String, String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(token.as_bytes())
        .map_err(|_| "initialize gateway identity signer".to_string())?;
    mac.update(encoded_claims.as_bytes());
    Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
}

fn validate_gateway_identity_claims(
    claims: &RuntimeApiGatewayIdentityClaims,
    now: u64,
) -> Result<(), String> {
    if claims.schema != CSM_RUNTIME_API_GATEWAY_IDENTITY_SCHEMA {
        return Err("gateway_identity_schema_invalid".to_string());
    }
    if claims.audience != CSM_RUNTIME_API_GATEWAY_IDENTITY_AUDIENCE {
        return Err("gateway_identity_audience_invalid".to_string());
    }
    if claims.issuer != "aws_api_gateway_authorizer" || claims.principal.trim().is_empty() {
        return Err("gateway_identity_principal_invalid".to_string());
    }
    if claims.authorization_scopes.is_empty()
        || claims
            .authorization_scopes
            .iter()
            .any(|scope| scope != "csm.runtime.read")
    {
        return Err("gateway_identity_scope_invalid".to_string());
    }
    if claims.issued_at_epoch_secs > now.saturating_add(30)
        || claims.expires_at_epoch_secs <= now
        || claims.expires_at_epoch_secs <= claims.issued_at_epoch_secs
        || claims
            .expires_at_epoch_secs
            .saturating_sub(claims.issued_at_epoch_secs)
            > CSM_RUNTIME_API_GATEWAY_IDENTITY_MAX_TTL_SECS
    {
        return Err("gateway_identity_lifetime_invalid".to_string());
    }
    Ok(())
}

fn validate_stored(stored: &StoredCredential) -> Result<(), String> {
    if stored.schema != CSM_RUNTIME_API_AUTH_SCHEMA {
        return Err(format!(
            "unsupported runtime API credential schema {}",
            stored.schema
        ));
    }
    if stored.generation == 0 || stored.token.len() < 32 {
        return Err("invalid runtime API credential material".to_string());
    }
    Ok(())
}

fn metadata_from_stored(stored: &StoredCredential) -> RuntimeApiCredentialMetadata {
    RuntimeApiCredentialMetadata {
        schema: CSM_RUNTIME_API_AUTH_SCHEMA,
        generation: stored.generation,
        fingerprint: token_fingerprint(&stored.token),
        created_at_epoch_secs: stored.created_at_epoch_secs,
        expires_at_epoch_secs: stored.expires_at_epoch_secs,
        revoked: stored.revoked,
    }
}

fn token_fingerprint(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    format!("sha256:{}", hex_prefix(&digest, 12))
}

fn hex_prefix(bytes: &[u8], chars: usize) -> String {
    let mut rendered = String::with_capacity(chars);
    for byte in bytes {
        rendered.push_str(&format!("{byte:02x}"));
        if rendered.len() >= chars {
            rendered.truncate(chars);
            break;
        }
    }
    rendered
}

fn now_epoch_secs() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|err| format!("read system time for runtime API credential: {err}"))
}

fn write_private_json_atomic(path: &Path, value: &StoredCredential) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "runtime API credential path has no parent".to_string())?;
    fs::create_dir_all(parent)
        .map_err(|err| format!("create runtime API credential parent: {err}"))?;
    let mut random_suffix = [0_u8; 16];
    OsRng.fill_bytes(&mut random_suffix);
    let tmp = path.with_extension(format!("tmp-{}", URL_SAFE_NO_PAD.encode(random_suffix)));
    let payload = serde_json::to_vec_pretty(value)
        .map_err(|err| format!("serialize runtime API credential: {err}"))?;
    let mut options = OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(&tmp)
        .map_err(|err| format!("open runtime API credential temp file: {err}"))?;
    file.write_all(&payload)
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.sync_all())
        .map_err(|err| format!("persist runtime API credential: {err}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600))
            .map_err(|err| format!("secure runtime API credential permissions: {err}"))?;
    }
    fs::rename(&tmp, path)
        .map_err(|err| format!("commit runtime API credential atomically: {err}"))?;
    validate_private_file(path)
}

fn validate_private_file(path: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|err| format!("inspect runtime API credential: {err}"))?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err("runtime API credential must be a regular non-symlink file".to_string());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o600 {
            return Err(format!(
                "runtime API credential permissions must be 0600, observed {mode:04o}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn credential_store_fails_closed_and_rotates_without_exposing_token() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        let first = store.ensure().unwrap();
        let first_token = store.bearer_token().unwrap();
        assert!(matches!(
            store.authorize(None),
            RuntimeApiAuthDecision::Rejected {
                reason: "missing_bearer_token",
                ..
            }
        ));
        assert!(matches!(
            store.authorize(Some("Bearer wrong")),
            RuntimeApiAuthDecision::Rejected {
                reason: "invalid_bearer_token",
                ..
            }
        ));
        assert!(matches!(
            store.authorize(Some(&format!("Bearer {}", first_token.expose_secret()))),
            RuntimeApiAuthDecision::Authenticated(_)
        ));
        let second = store.rotate().unwrap();
        assert_eq!(second.generation, first.generation + 1);
        assert_ne!(second.fingerprint, first.fingerprint);
        assert!(matches!(
            store.authorize(Some(&format!("Bearer {}", first_token.expose_secret()))),
            RuntimeApiAuthDecision::Rejected {
                reason: "invalid_bearer_token",
                ..
            }
        ));
        store.revoke().unwrap();
        assert!(matches!(
            store.authorize(Some("Bearer anything")),
            RuntimeApiAuthDecision::Rejected {
                reason: "credential_revoked",
                ..
            }
        ));
        let serialized = serde_json::to_string(&second).unwrap();
        assert!(!serialized.contains(first_token.expose_secret()));
    }

    #[test]
    fn credential_store_rejects_expired_material_for_server_and_client() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        store.write_new(1).unwrap();
        let (token, metadata) = store.load().unwrap();
        let expired = StoredCredential {
            schema: CSM_RUNTIME_API_AUTH_SCHEMA.to_string(),
            generation: metadata.generation,
            token: token.expose_secret().to_string(),
            created_at_epoch_secs: metadata.created_at_epoch_secs,
            expires_at_epoch_secs: Some(0),
            revoked: false,
        };
        write_private_json_atomic(store.path(), &expired).unwrap();
        assert!(matches!(
            store.authorize(Some("Bearer anything")),
            RuntimeApiAuthDecision::Rejected {
                reason: "credential_expired",
                ..
            }
        ));
        assert_eq!(
            store.with_bearer_token(str::to_string).unwrap_err(),
            "runtime API credential is expired"
        );
    }

    #[cfg(unix)]
    #[test]
    fn credential_store_rejects_group_or_world_readable_file() {
        use std::os::unix::fs::PermissionsExt;
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        store.ensure().unwrap();
        fs::set_permissions(store.path(), fs::Permissions::from_mode(0o644)).unwrap();
        assert!(matches!(
            store.authorize(None),
            RuntimeApiAuthDecision::Unavailable { reason }
                if reason.contains("permissions must be 0600")
        ));
    }

    #[test]
    fn credential_metadata_is_read_only_when_store_is_missing() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        assert_eq!(store.metadata().unwrap(), None);
        assert!(!store.path().exists());
    }

    #[test]
    fn gateway_identity_is_signed_verified_and_credential_free() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        store.ensure().unwrap();
        let now = now_epoch_secs().unwrap();
        let claims = RuntimeApiGatewayIdentityClaims {
            schema: CSM_RUNTIME_API_GATEWAY_IDENTITY_SCHEMA.to_string(),
            issuer: "aws_api_gateway_authorizer".to_string(),
            principal: "operator@example.invalid".to_string(),
            audience: CSM_RUNTIME_API_GATEWAY_IDENTITY_AUDIENCE.to_string(),
            authorization_scopes: vec!["csm.runtime.read".to_string()],
            issued_at_epoch_secs: now,
            expires_at_epoch_secs: now + 60,
        };
        let (encoded, signature) = store.sign_gateway_identity(&claims).unwrap();
        let verified = store
            .verify_gateway_identity(Some(&encoded), Some(&signature))
            .unwrap()
            .unwrap();
        assert_eq!(verified.issuer, "aws_api_gateway_authorizer");
        assert_eq!(verified.authorization_scopes, vec!["csm.runtime.read"]);
        assert!(!verified.credential_material_propagated);
        assert!(!serde_json::to_string(&verified)
            .unwrap()
            .contains("operator@example.invalid"));
        assert_eq!(
            store
                .verify_gateway_identity(Some(&encoded), Some("forged"))
                .unwrap_err(),
            "gateway_identity_signature_invalid"
        );
        assert_eq!(
            store
                .verify_gateway_identity(Some(&encoded), None)
                .unwrap_err(),
            "gateway_identity_headers_incomplete"
        );
    }
}
