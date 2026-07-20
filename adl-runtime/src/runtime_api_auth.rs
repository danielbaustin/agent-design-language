use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use fs2::FileExt;
use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use rand::RngCore;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

pub const CSM_RUNTIME_API_AUTH_SCHEMA: &str = "adl.csm.runtime_api.auth.v1";
pub const CSM_RUNTIME_API_AUTH_FILE: &str = "runtime_api_auth.json";
pub const CSM_RUNTIME_API_AUTH_LOCK_FILE: &str = "runtime_api_auth.lock";
pub const CSM_RUNTIME_API_AUTH_EVENTS_FILE: &str = "runtime_api_auth_events.jsonl";
pub const CSM_RUNTIME_API_CREDENTIAL_TTL_SECS: u64 = 24 * 60 * 60;
pub const CSM_RUNTIME_API_CREDENTIAL_RENEWAL_WINDOW_SECS: u64 = 15 * 60;
pub const CSM_RUNTIME_API_CREDENTIAL_OVERLAP_SECS: u64 = 5 * 60;
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    previous: Option<PreviousCredential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreviousCredential {
    generation: u64,
    token: String,
    created_at_epoch_secs: u64,
    expires_at_epoch_secs: u64,
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

enum AuthorizationPreparation {
    Decision(RuntimeApiAuthDecision),
    Ensure,
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
        self.with_mutation_lock(|| self.ensure_locked())
    }

    fn ensure_locked(&self) -> Result<RuntimeApiCredentialMetadata, String> {
        if self.path.exists() {
            let (_, metadata) = self.load()?;
            if metadata.revoked {
                return Err(
                    "runtime API credential is revoked; explicit rotation required".to_string(),
                );
            }
            let renew = metadata.expires_at_epoch_secs.is_some_and(|expires| {
                now_epoch_secs().is_ok_and(|now| {
                    expires <= now.saturating_add(CSM_RUNTIME_API_CREDENTIAL_RENEWAL_WINDOW_SECS)
                })
            });
            if renew {
                return self.rotate_at_locked(now_epoch_secs()?);
            }
            return Ok(metadata);
        }
        self.write_new(1, None, now_epoch_secs()?)
    }

    pub fn metadata(&self) -> Result<Option<RuntimeApiCredentialMetadata>, String> {
        if !self.path.exists() {
            return Ok(None);
        }
        self.load().map(|(_, metadata)| Some(metadata))
    }

    pub fn authorize(&self, authorization: Option<&str>) -> RuntimeApiAuthDecision {
        let preparation = self.with_read_lock(|| {
            if !self.path.exists() {
                return Ok(AuthorizationPreparation::Ensure);
            }
            let stored = self.load_stored()?;
            if stored.revoked {
                return self
                    .authorize_stored(stored, authorization, None)
                    .map(AuthorizationPreparation::Decision);
            }
            let renew = stored.expires_at_epoch_secs.is_some_and(|expires| {
                now_epoch_secs().is_ok_and(|now| {
                    expires <= now.saturating_add(CSM_RUNTIME_API_CREDENTIAL_RENEWAL_WINDOW_SECS)
                })
            });
            if renew {
                Ok(AuthorizationPreparation::Ensure)
            } else {
                self.authorize_stored(stored, authorization, None)
                    .map(AuthorizationPreparation::Decision)
            }
        });
        match preparation {
            Ok(AuthorizationPreparation::Decision(decision)) => decision,
            Ok(AuthorizationPreparation::Ensure) => {
                let ensure_error = self.ensure().err();
                match self.with_read_lock(|| self.authorize_locked(authorization, ensure_error)) {
                    Ok(decision) => decision,
                    Err(reason) => RuntimeApiAuthDecision::Unavailable { reason },
                }
            }
            Err(reason) => RuntimeApiAuthDecision::Unavailable { reason },
        }
    }

    fn authorize_locked(
        &self,
        authorization: Option<&str>,
        ensure_error: Option<String>,
    ) -> Result<RuntimeApiAuthDecision, String> {
        self.authorize_stored(self.load_stored()?, authorization, ensure_error)
    }

    fn authorize_stored(
        &self,
        stored: StoredCredential,
        authorization: Option<&str>,
        ensure_error: Option<String>,
    ) -> Result<RuntimeApiAuthDecision, String> {
        let metadata = metadata_from_stored(&stored);
        if stored.revoked {
            return Ok(RuntimeApiAuthDecision::Rejected {
                reason: "credential_revoked",
                metadata: Some(metadata),
            });
        }
        if let Some(reason) = ensure_error {
            return Err(reason);
        }
        if metadata
            .expires_at_epoch_secs
            .is_some_and(|expires| now_epoch_secs().is_ok_and(|now| now >= expires))
        {
            return Ok(RuntimeApiAuthDecision::Rejected {
                reason: "credential_expired",
                metadata: Some(metadata),
            });
        }
        let Some(raw) = authorization else {
            return Ok(RuntimeApiAuthDecision::Rejected {
                reason: "missing_bearer_token",
                metadata: Some(metadata),
            });
        };
        let Some(candidate) = raw.strip_prefix("Bearer ") else {
            return Ok(RuntimeApiAuthDecision::Rejected {
                reason: "malformed_authorization",
                metadata: Some(metadata),
            });
        };
        let current_equal = stored.token.as_bytes().ct_eq(candidate.as_bytes()).into();
        if current_equal {
            Ok(RuntimeApiAuthDecision::Authenticated(metadata))
        } else if stored.previous.as_ref().is_some_and(|previous| {
            now_epoch_secs().is_ok_and(|now| now < previous.expires_at_epoch_secs)
                && previous.token.as_bytes().ct_eq(candidate.as_bytes()).into()
        }) {
            Ok(RuntimeApiAuthDecision::Authenticated(
                metadata_from_previous(
                    stored
                        .previous
                        .as_ref()
                        .expect("checked previous credential"),
                ),
            ))
        } else {
            Ok(RuntimeApiAuthDecision::Rejected {
                reason: "invalid_bearer_token",
                metadata: Some(metadata),
            })
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
        credential: &RuntimeApiCredentialMetadata,
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
        let expected = self.with_read_lock(|| {
            let stored = self.load_stored()?;
            if stored.revoked {
                return Err("runtime API credential is revoked".to_string());
            }
            let now = now_epoch_secs()?;
            let token = if stored.generation == credential.generation
                && stored
                    .expires_at_epoch_secs
                    .is_none_or(|expires| now < expires)
            {
                &stored.token
            } else if let Some(previous) = stored.previous.as_ref().filter(|previous| {
                previous.generation == credential.generation && now < previous.expires_at_epoch_secs
            }) {
                &previous.token
            } else {
                return Err("gateway_identity_credential_generation_invalid".to_string());
            };
            gateway_identity_signature(token, encoded_claims)
        })?;
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
        self.with_mutation_lock(|| self.rotate_at_locked(now_epoch_secs()?))
    }

    fn rotate_at_locked(&self, now: u64) -> Result<RuntimeApiCredentialMetadata, String> {
        let current = self.load_stored()?;
        if current.revoked {
            return Err("runtime API credential is revoked; explicit reset required".to_string());
        }
        let previous = current
            .expires_at_epoch_secs
            .unwrap_or(u64::MAX)
            .min(now.saturating_add(CSM_RUNTIME_API_CREDENTIAL_OVERLAP_SECS))
            .checked_sub(now)
            .filter(|remaining| *remaining > 0)
            .map(|remaining| PreviousCredential {
                generation: current.generation,
                token: current.token,
                created_at_epoch_secs: current.created_at_epoch_secs,
                expires_at_epoch_secs: now.saturating_add(remaining),
            });
        let previous_generation = previous.as_ref().map(|value| value.generation);
        let overlap_seconds = previous
            .as_ref()
            .map_or(0, |value| value.expires_at_epoch_secs.saturating_sub(now));
        let metadata = self.write_new(current.generation.saturating_add(1), previous, now)?;
        self.append_generation_event(
            "credential_rotated",
            &metadata,
            previous_generation,
            overlap_seconds,
        )?;
        Ok(metadata)
    }

    pub fn revoke(&self) -> Result<RuntimeApiCredentialMetadata, String> {
        self.with_mutation_lock(|| self.revoke_locked())
    }

    fn revoke_locked(&self) -> Result<RuntimeApiCredentialMetadata, String> {
        let (token, metadata) = self.load()?;
        let stored = StoredCredential {
            schema: CSM_RUNTIME_API_AUTH_SCHEMA.to_string(),
            generation: metadata.generation,
            token: token.expose_secret().to_string(),
            created_at_epoch_secs: metadata.created_at_epoch_secs,
            expires_at_epoch_secs: metadata.expires_at_epoch_secs,
            revoked: true,
            previous: None,
        };
        write_private_json_atomic(&self.path, &stored)?;
        let metadata = metadata_from_stored(&stored);
        self.append_generation_event("credential_revoked", &metadata, None, 0)?;
        Ok(metadata)
    }

    fn with_mutation_lock<T>(
        &self,
        operation: impl FnOnce() -> Result<T, String>,
    ) -> Result<T, String> {
        self.with_credential_lock(true, operation)
    }

    fn with_read_lock<T>(
        &self,
        operation: impl FnOnce() -> Result<T, String>,
    ) -> Result<T, String> {
        self.with_credential_lock(false, operation)
    }

    fn with_credential_lock<T>(
        &self,
        exclusive: bool,
        operation: impl FnOnce() -> Result<T, String>,
    ) -> Result<T, String> {
        let parent = self
            .path
            .parent()
            .ok_or_else(|| "runtime API credential path has no parent".to_string())?;
        fs::create_dir_all(parent)
            .map_err(|err| format!("create runtime API credential directory: {err}"))?;
        let lock_path = parent.join(CSM_RUNTIME_API_AUTH_LOCK_FILE);
        let mut options = OpenOptions::new();
        options.create(true).read(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let lock = options
            .open(lock_path)
            .map_err(|err| format!("open runtime API credential lock: {err}"))?;
        if exclusive {
            FileExt::lock_exclusive(&lock)
                .map_err(|err| format!("lock runtime API credential mutation: {err}"))?;
        } else {
            FileExt::lock_shared(&lock)
                .map_err(|err| format!("lock runtime API credential read: {err}"))?;
        }
        let result = operation();
        let unlock = FileExt::unlock(&lock)
            .map_err(|err| format!("unlock runtime API credential mutation: {err}"));
        match (result, unlock) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
        }
    }

    fn load(&self) -> Result<(SecretString, RuntimeApiCredentialMetadata), String> {
        let stored = self.load_stored()?;
        let metadata = metadata_from_stored(&stored);
        Ok((SecretString::new(stored.token), metadata))
    }

    fn load_stored(&self) -> Result<StoredCredential, String> {
        validate_private_file(&self.path)?;
        let raw =
            fs::read(&self.path).map_err(|err| format!("read runtime API credential: {err}"))?;
        let stored: StoredCredential = serde_json::from_slice(&raw)
            .map_err(|err| format!("parse runtime API credential: {err}"))?;
        validate_stored(&stored)?;
        Ok(stored)
    }

    fn write_new(
        &self,
        generation: u64,
        previous: Option<PreviousCredential>,
        created_at_epoch_secs: u64,
    ) -> Result<RuntimeApiCredentialMetadata, String> {
        let parent = self
            .path
            .parent()
            .ok_or_else(|| "runtime API credential path has no parent".to_string())?;
        fs::create_dir_all(parent)
            .map_err(|err| format!("create runtime API credential directory: {err}"))?;
        let mut bytes = [0_u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let stored = StoredCredential {
            schema: CSM_RUNTIME_API_AUTH_SCHEMA.to_string(),
            generation,
            token: URL_SAFE_NO_PAD.encode(bytes),
            created_at_epoch_secs,
            expires_at_epoch_secs: Some(
                created_at_epoch_secs.saturating_add(CSM_RUNTIME_API_CREDENTIAL_TTL_SECS),
            ),
            revoked: false,
            previous,
        };
        write_private_json_atomic(&self.path, &stored)?;
        Ok(metadata_from_stored(&stored))
    }

    fn append_generation_event(
        &self,
        event: &str,
        metadata: &RuntimeApiCredentialMetadata,
        previous_generation: Option<u64>,
        overlap_seconds: u64,
    ) -> Result<(), String> {
        let path = self
            .path
            .parent()
            .ok_or_else(|| "runtime API credential path has no parent".to_string())?
            .join(CSM_RUNTIME_API_AUTH_EVENTS_FILE);
        let record = serde_json::json!({
            "schema": "adl.csm.runtime_api.credential_event.v1",
            "observed_at_epoch_secs": now_epoch_secs()?,
            "event": event,
            "generation": metadata.generation,
            "previous_generation": previous_generation,
            "overlap_seconds": overlap_seconds,
            "secret_retained": false
        });
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|err| format!("open runtime API credential event log: {err}"))?;
        serde_json::to_writer(&mut file, &record)
            .map_err(|err| format!("serialize runtime API credential event: {err}"))?;
        file.write_all(b"\n")
            .and_then(|_| file.sync_data())
            .map_err(|err| format!("persist runtime API credential event: {err}"))
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
    if stored.previous.as_ref().is_some_and(|previous| {
        previous.generation == 0
            || previous.generation >= stored.generation
            || previous.token.len() < 32
            || previous.expires_at_epoch_secs <= stored.created_at_epoch_secs
    }) {
        return Err("invalid previous runtime API credential material".to_string());
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

fn metadata_from_previous(previous: &PreviousCredential) -> RuntimeApiCredentialMetadata {
    RuntimeApiCredentialMetadata {
        schema: CSM_RUNTIME_API_AUTH_SCHEMA,
        generation: previous.generation,
        fingerprint: token_fingerprint(&previous.token),
        created_at_epoch_secs: previous.created_at_epoch_secs,
        expires_at_epoch_secs: Some(previous.expires_at_epoch_secs),
        revoked: false,
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
    fn ordinary_authorization_uses_the_shared_credential_lock() {
        use std::sync::mpsc;
        use std::time::Duration;

        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        store.ensure().unwrap();
        let token = store.with_bearer_token(str::to_string).unwrap();
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .open(root.path().join(CSM_RUNTIME_API_AUTH_LOCK_FILE))
            .unwrap();
        FileExt::lock_shared(&lock).unwrap();
        let (tx, rx) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            tx.send(store.authorize(Some(&format!("Bearer {token}"))))
                .unwrap();
        });

        assert!(matches!(
            rx.recv_timeout(Duration::from_millis(250)).unwrap(),
            RuntimeApiAuthDecision::Authenticated(_)
        ));
        FileExt::unlock(&lock).unwrap();
        worker.join().unwrap();
    }

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
            RuntimeApiAuthDecision::Authenticated(metadata) if metadata.generation == first.generation
        ));
        let second_token = store.with_bearer_token(str::to_string).unwrap();
        store.revoke().unwrap();
        assert!(matches!(
            store.authorize(Some(&format!("Bearer {}", first_token.expose_secret()))),
            RuntimeApiAuthDecision::Rejected {
                reason: "credential_revoked",
                ..
            }
        ));
        assert!(matches!(
            store.authorize(Some(&format!("Bearer {second_token}"))),
            RuntimeApiAuthDecision::Rejected {
                reason: "credential_revoked",
                ..
            }
        ));
        assert!(store.ensure().is_err());
        let serialized = serde_json::to_string(&second).unwrap();
        assert!(!serialized.contains(first_token.expose_secret()));
        let events =
            fs::read_to_string(root.path().join(CSM_RUNTIME_API_AUTH_EVENTS_FILE)).unwrap();
        assert!(events.contains("credential_rotated"));
        assert!(events.contains("credential_revoked"));
        assert!(!events.contains(first_token.expose_secret()));
    }

    #[test]
    fn credential_store_recovers_expired_non_revoked_material_without_overlap() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        store.write_new(1, None, now_epoch_secs().unwrap()).unwrap();
        let (token, metadata) = store.load().unwrap();
        let expired = StoredCredential {
            schema: CSM_RUNTIME_API_AUTH_SCHEMA.to_string(),
            generation: metadata.generation,
            token: token.expose_secret().to_string(),
            created_at_epoch_secs: metadata.created_at_epoch_secs,
            expires_at_epoch_secs: Some(0),
            revoked: false,
            previous: None,
        };
        write_private_json_atomic(store.path(), &expired).unwrap();
        assert!(matches!(
            store.authorize(Some(&format!("Bearer {}", token.expose_secret()))),
            RuntimeApiAuthDecision::Rejected {
                reason: "invalid_bearer_token",
                ..
            }
        ));
        let recovered = store.metadata().unwrap().unwrap();
        assert_eq!(recovered.generation, metadata.generation + 1);
        assert!(store.load_stored().unwrap().previous.is_none());
        assert_ne!(
            store.with_bearer_token(str::to_string).unwrap(),
            token.expose_secret().as_str()
        );
    }

    #[test]
    fn credential_store_renews_before_expiry() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        let first = store.ensure().unwrap();
        let (token, metadata) = store.load().unwrap();
        let near_expiry = StoredCredential {
            schema: CSM_RUNTIME_API_AUTH_SCHEMA.to_string(),
            generation: metadata.generation,
            token: token.expose_secret().to_string(),
            created_at_epoch_secs: metadata.created_at_epoch_secs,
            expires_at_epoch_secs: Some(now_epoch_secs().unwrap() + 60),
            revoked: false,
            previous: None,
        };
        write_private_json_atomic(store.path(), &near_expiry).unwrap();
        let renewed = store.ensure().unwrap();
        assert_eq!(renewed.generation, first.generation + 1);
        assert!(renewed.expires_at_epoch_secs.unwrap() > now_epoch_secs().unwrap() + 60);
        let events =
            fs::read_to_string(root.path().join(CSM_RUNTIME_API_AUTH_EVENTS_FILE)).unwrap();
        let rotation: serde_json::Value =
            serde_json::from_str(events.lines().last().unwrap()).unwrap();
        assert!(rotation["overlap_seconds"].as_u64().unwrap() <= 60);
    }

    #[test]
    fn one_second_rotation_overlap_uses_the_replacement_timestamp() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        let now = now_epoch_secs().unwrap();
        store.write_new(1, None, now).unwrap();
        let mut current = store.load_stored().unwrap();
        current.expires_at_epoch_secs = Some(now + 1);
        write_private_json_atomic(store.path(), &current).unwrap();

        let rotated = store.rotate_at_locked(now).unwrap();

        assert_eq!(rotated.created_at_epoch_secs, now);
        let stored = store.load_stored().unwrap();
        assert_eq!(stored.previous.unwrap().expires_at_epoch_secs, now + 1);
    }

    #[test]
    fn terminal_revoke_cannot_be_resurrected_by_concurrent_rotation() {
        use std::sync::{Arc, Barrier};

        let root = tempdir().unwrap();
        let store = Arc::new(RuntimeApiCredentialStore::for_state_root(root.path()));
        store.ensure().unwrap();
        let workers = 16;
        let barrier = Arc::new(Barrier::new(workers + 1));
        let mut rotations = Vec::new();
        for _ in 0..workers {
            let store = Arc::clone(&store);
            let barrier = Arc::clone(&barrier);
            rotations.push(std::thread::spawn(move || {
                barrier.wait();
                store.rotate()
            }));
        }
        barrier.wait();
        store.revoke().unwrap();
        for rotation in rotations {
            let _ = rotation.join().unwrap();
        }

        let stored = store.load_stored().unwrap();
        assert!(stored.revoked);
        assert!(stored.previous.is_none());
        assert!(store.rotate().is_err());
    }

    #[test]
    fn authorization_decision_serializes_with_terminal_revoke() {
        use std::sync::{mpsc, Arc, Barrier};
        use std::time::Duration;

        let root = tempdir().unwrap();
        let store = Arc::new(RuntimeApiCredentialStore::for_state_root(root.path()));
        store.ensure().unwrap();
        let token = store.with_bearer_token(str::to_string).unwrap();
        let authorization = format!("Bearer {token}");
        let decision_ready = Arc::new(Barrier::new(2));
        let release_decision = Arc::new(Barrier::new(2));

        let reader = {
            let store = Arc::clone(&store);
            let decision_ready = Arc::clone(&decision_ready);
            let release_decision = Arc::clone(&release_decision);
            std::thread::spawn(move || {
                store.with_read_lock(|| {
                    let decision = store.authorize_locked(Some(&authorization), None)?;
                    decision_ready.wait();
                    release_decision.wait();
                    Ok(decision)
                })
            })
        };
        decision_ready.wait();

        let (revoked_tx, revoked_rx) = mpsc::channel();
        let revoker = {
            let store = Arc::clone(&store);
            std::thread::spawn(move || revoked_tx.send(store.revoke()).unwrap())
        };
        assert!(revoked_rx.recv_timeout(Duration::from_millis(50)).is_err());
        release_decision.wait();

        assert!(matches!(
            reader.join().unwrap().unwrap(),
            RuntimeApiAuthDecision::Authenticated(_)
        ));
        revoked_rx
            .recv_timeout(Duration::from_secs(1))
            .unwrap()
            .unwrap();
        revoker.join().unwrap();
        assert!(matches!(
            store.authorize(Some(&format!("Bearer {token}"))),
            RuntimeApiAuthDecision::Rejected {
                reason: "credential_revoked",
                ..
            }
        ));
    }

    #[test]
    fn expired_overlap_rejects_previous_generation() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        store.ensure().unwrap();
        let first_token = store.with_bearer_token(str::to_string).unwrap();
        store.rotate().unwrap();
        let mut stored = store.load_stored().unwrap();
        let now = now_epoch_secs().unwrap();
        stored.created_at_epoch_secs = now.saturating_sub(10);
        stored.previous.as_mut().unwrap().expires_at_epoch_secs = now.saturating_sub(1);
        write_private_json_atomic(store.path(), &stored).unwrap();

        assert!(matches!(
            store.authorize(Some(&format!("Bearer {first_token}"))),
            RuntimeApiAuthDecision::Rejected {
                reason: "invalid_bearer_token",
                ..
            }
        ));
    }

    #[test]
    fn authorization_renews_a_long_lived_server_before_expiry() {
        let root = tempdir().unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(root.path());
        let first = store.ensure().unwrap();
        let (token, metadata) = store.load().unwrap();
        let near_expiry = StoredCredential {
            schema: CSM_RUNTIME_API_AUTH_SCHEMA.to_string(),
            generation: metadata.generation,
            token: token.expose_secret().to_string(),
            created_at_epoch_secs: metadata.created_at_epoch_secs,
            expires_at_epoch_secs: Some(now_epoch_secs().unwrap() + 60),
            revoked: false,
            previous: None,
        };
        write_private_json_atomic(store.path(), &near_expiry).unwrap();
        let decision = store.authorize(Some(&format!("Bearer {}", token.expose_secret())));
        assert!(matches!(
            decision,
            RuntimeApiAuthDecision::Authenticated(metadata)
                if metadata.generation == first.generation
        ));
        assert_eq!(
            store.metadata().unwrap().unwrap().generation,
            first.generation + 1
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
        let first = store.ensure().unwrap();
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
            .verify_gateway_identity(Some(&encoded), Some(&signature), &first)
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
                .verify_gateway_identity(Some(&encoded), Some("forged"), &first)
                .unwrap_err(),
            "gateway_identity_signature_invalid"
        );
        assert_eq!(
            store
                .verify_gateway_identity(Some(&encoded), None, &first)
                .unwrap_err(),
            "gateway_identity_headers_incomplete"
        );

        let second = store.rotate().unwrap();
        assert!(store
            .verify_gateway_identity(Some(&encoded), Some(&signature), &first)
            .unwrap()
            .is_some());
        assert_eq!(
            store
                .verify_gateway_identity(Some(&encoded), Some(&signature), &second)
                .unwrap_err(),
            "gateway_identity_signature_invalid"
        );
        store.revoke().unwrap();
        assert_eq!(
            store
                .verify_gateway_identity(Some(&encoded), Some(&signature), &first)
                .unwrap_err(),
            "runtime API credential is revoked"
        );
    }
}
