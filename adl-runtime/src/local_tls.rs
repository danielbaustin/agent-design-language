#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::Write,
    net::IpAddr,
    path::{Component, Path, PathBuf},
    sync::{Mutex, OnceLock},
};

use ::time::{Duration, OffsetDateTime};
use base64::Engine;
use fs2::FileExt;
use rcgen::{CertificateParams, ExtendedKeyUsagePurpose, KeyPair};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use x509_parser::{extensions::GeneralName, parse_x509_certificate};

pub const LOCAL_TLS_BOOTSTRAP_SCHEMA: &str = "adl.runtime_v3.local_tls_bootstrap.v1";
pub const LOCAL_TLS_BOOTSTRAP_OUTCOME_SCHEMA: &str =
    "adl.runtime_v3.local_tls_bootstrap.outcome.v1";
const LOCAL_CERTIFICATE_VALIDITY_DAYS: i64 = 397;
const GENERATION_MANIFEST_SCHEMA: &str = "adl.runtime_v3.local_tls_generation.v1";
const CURRENT_GENERATION_MANIFEST: &str = "current-generation.json";
const GENERATIONS_DIR: &str = "generations";
#[cfg(test)]
static FORCE_POST_SWAP_MANIFEST_SYNC_FAILURE: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeTlsBootstrapConfig {
    pub schema: String,
    pub mode: RuntimeTlsBootstrapMode,
    #[serde(default)]
    pub state_root: Option<PathBuf>,
    #[serde(default)]
    pub tls_dir: Option<PathBuf>,
    pub certificate_chain_path: PathBuf,
    #[serde(default)]
    pub public_certificate_path: Option<PathBuf>,
    pub private_key_path: PathBuf,
    #[serde(default)]
    pub dns_names: Vec<String>,
    #[serde(default)]
    pub ip_addresses: Vec<IpAddr>,
    #[serde(default)]
    pub replace: bool,
}

impl RuntimeTlsBootstrapConfig {
    pub fn from_toml_str(text: &str) -> Result<Self, LocalTlsError> {
        let config: Self =
            toml::from_str(text).map_err(|error| LocalTlsError::Config(error.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn from_json_str(text: &str) -> Result<Self, LocalTlsError> {
        let config: Self =
            serde_json::from_str(text).map_err(|error| LocalTlsError::Config(error.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), LocalTlsError> {
        if self.schema != LOCAL_TLS_BOOTSTRAP_SCHEMA {
            return Err(LocalTlsError::UnsupportedSchema(self.schema.clone()));
        }
        match self.mode {
            RuntimeTlsBootstrapMode::ManagedExternal => {
                if self.replace {
                    return Err(LocalTlsError::Policy(
                        "managed_external TLS does not support local replacement".to_owned(),
                    ));
                }
                if self.certificate_chain_path.as_os_str().is_empty()
                    || self.private_key_path.as_os_str().is_empty()
                    || self.certificate_chain_path == self.private_key_path
                {
                    return Err(LocalTlsError::Policy(
                        "managed_external TLS requires distinct certificate and key paths"
                            .to_owned(),
                    ));
                }
                Ok(())
            }
            RuntimeTlsBootstrapMode::LocalSelfSigned => {
                let state_root = self
                    .state_root
                    .as_ref()
                    .ok_or_else(|| LocalTlsError::Policy("state_root is required".to_owned()))?;
                if !state_root.is_absolute() {
                    return Err(LocalTlsError::Policy(
                        "state_root must be an absolute configured path".to_owned(),
                    ));
                }
                let tls_dir = self
                    .tls_dir
                    .as_ref()
                    .ok_or_else(|| LocalTlsError::Policy("tls_dir is required".to_owned()))?;
                validate_relative_child("tls_dir", tls_dir)?;
                validate_relative_child("certificate_chain_path", &self.certificate_chain_path)?;
                validate_relative_child("private_key_path", &self.private_key_path)?;
                let public_certificate_path =
                    self.public_certificate_path.as_ref().ok_or_else(|| {
                        LocalTlsError::Policy("public_certificate_path is required".to_owned())
                    })?;
                validate_relative_child("public_certificate_path", public_certificate_path)?;
                if self.certificate_chain_path == self.private_key_path
                    || self.certificate_chain_path == *public_certificate_path
                    || self.private_key_path == *public_certificate_path
                {
                    return Err(LocalTlsError::Policy(
                        "local TLS certificate, public certificate, and key paths must be distinct"
                            .to_owned(),
                    ));
                }
                if self.dns_names.is_empty() && self.ip_addresses.is_empty() {
                    return Err(LocalTlsError::Policy(
                        "local_self_signed TLS requires at least one DNS or IP SAN".to_owned(),
                    ));
                }
                for name in &self.dns_names {
                    if name.trim().is_empty() || name.contains('/') || name.contains('\\') {
                        return Err(LocalTlsError::Policy(
                            "DNS SAN entries must be non-empty host names".to_owned(),
                        ));
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeTlsBootstrapMode {
    ManagedExternal,
    LocalSelfSigned,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimeTlsBootstrapOutcome {
    pub schema: String,
    pub mode: RuntimeTlsBootstrapMode,
    pub certificate_chain_path: PathBuf,
    pub public_certificate_path: Option<PathBuf>,
    pub private_key_path: PathBuf,
    pub certificate_sha256: Option<String>,
    pub reused_existing_identity: bool,
    pub replaced_existing_identity: bool,
    pub event: RuntimeTlsBootstrapEvent,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeTlsBootstrapEvent {
    ManagedExternalPreserved,
    LocalCertificateCreated,
    LocalCertificateReused,
    LocalCertificateReplaced,
}

#[derive(Debug)]
pub enum LocalTlsError {
    UnsupportedSchema(String),
    Config(String),
    Policy(String),
    LockBusy,
    Io(String),
    Generate(String),
    Rustls(String),
}

impl std::fmt::Display for LocalTlsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalTlsError::UnsupportedSchema(schema) => {
                write!(
                    formatter,
                    "unsupported local TLS bootstrap schema: {schema}"
                )
            }
            LocalTlsError::Config(error) => {
                write!(formatter, "invalid local TLS bootstrap config: {error}")
            }
            LocalTlsError::Policy(error) => {
                write!(
                    formatter,
                    "local TLS policy rejected configuration: {error}"
                )
            }
            LocalTlsError::LockBusy => {
                write!(
                    formatter,
                    "local TLS bootstrap is already active for this state root"
                )
            }
            LocalTlsError::Io(error) => write!(formatter, "local TLS I/O failed: {error}"),
            LocalTlsError::Generate(error) => {
                write!(
                    formatter,
                    "local TLS certificate generation failed: {error}"
                )
            }
            LocalTlsError::Rustls(error) => {
                write!(
                    formatter,
                    "local TLS material failed rustls validation: {error}"
                )
            }
        }
    }
}

impl std::error::Error for LocalTlsError {}

struct LocalTlsPaths {
    tls_root: PathBuf,
    generations_root: PathBuf,
    current_manifest: PathBuf,
    certificate_chain_name: PathBuf,
    public_certificate_name: PathBuf,
    private_key_name: PathBuf,
    lock_file: PathBuf,
}

#[derive(Clone, Debug)]
struct CommittedTlsGeneration {
    generation_id: String,
    certificate_chain: PathBuf,
    public_certificate: PathBuf,
    private_key: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GenerationManifest {
    schema: String,
    generation_id: String,
    certificate_chain_path: PathBuf,
    public_certificate_path: PathBuf,
    private_key_path: PathBuf,
    certificate_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ConfiguredSans {
    dns_names: BTreeSet<String>,
    ip_addresses: BTreeSet<IpAddr>,
}

impl ConfiguredSans {
    fn from_config(config: &RuntimeTlsBootstrapConfig) -> Self {
        Self {
            dns_names: config
                .dns_names
                .iter()
                .map(|name| name.to_ascii_lowercase())
                .collect(),
            ip_addresses: config.ip_addresses.iter().copied().collect(),
        }
    }
}

pub struct GeneratedTlsMaterial {
    pub certificate_pem: String,
    pub private_key_pem: String,
}

pub async fn bootstrap_runtime_tls(
    config: &RuntimeTlsBootstrapConfig,
) -> Result<RuntimeTlsBootstrapOutcome, LocalTlsError> {
    bootstrap_runtime_tls_with_generator(config, generate_local_material).await
}

pub async fn bootstrap_runtime_tls_with_generator<F>(
    config: &RuntimeTlsBootstrapConfig,
    generator: F,
) -> Result<RuntimeTlsBootstrapOutcome, LocalTlsError>
where
    F: FnOnce(&RuntimeTlsBootstrapConfig) -> Result<GeneratedTlsMaterial, LocalTlsError>,
{
    config.validate()?;
    match config.mode {
        RuntimeTlsBootstrapMode::ManagedExternal => {
            validate_rustls_pair(&config.certificate_chain_path, &config.private_key_path).await?;
            Ok(RuntimeTlsBootstrapOutcome {
                schema: LOCAL_TLS_BOOTSTRAP_OUTCOME_SCHEMA.to_owned(),
                mode: config.mode,
                certificate_chain_path: config.certificate_chain_path.clone(),
                public_certificate_path: config.public_certificate_path.clone(),
                private_key_path: config.private_key_path.clone(),
                certificate_sha256: sha256_file(&config.certificate_chain_path).ok(),
                reused_existing_identity: true,
                replaced_existing_identity: false,
                event: RuntimeTlsBootstrapEvent::ManagedExternalPreserved,
            })
        }
        RuntimeTlsBootstrapMode::LocalSelfSigned => {
            let paths = local_paths(config)?;
            fs::create_dir_all(&paths.tls_root)
                .map_err(|error| LocalTlsError::Io(error.to_string()))?;
            fs::create_dir_all(&paths.generations_root)
                .map_err(|error| LocalTlsError::Io(error.to_string()))?;
            let _guard = LocalBootstrapGuard::acquire(&paths.lock_file)?;
            let current = read_current_generation(&paths)?;
            if let Some(current) = current.as_ref() {
                validate_rustls_pair(&current.certificate_chain, &current.private_key).await?;
                let sans_result = verify_certificate_sans(&current.certificate_chain, config);
                if !config.replace {
                    sans_result?;
                }
                enforce_private_key_permissions(&current.private_key)?;
                ensure_public_certificate_copy(
                    &current.certificate_chain,
                    &current.public_certificate,
                )?;
                if !config.replace {
                    return Ok(local_outcome(
                        config.mode,
                        current,
                        true,
                        false,
                        RuntimeTlsBootstrapEvent::LocalCertificateReused,
                    ));
                }
            }
            let material = generator(config)?;
            verify_generated_certificate_sans(material.certificate_pem.as_bytes(), config)?;
            let generation = commit_generation(&paths, &material).await?;
            Ok(local_outcome(
                config.mode,
                &generation,
                false,
                current.is_some(),
                if current.is_some() {
                    RuntimeTlsBootstrapEvent::LocalCertificateReplaced
                } else {
                    RuntimeTlsBootstrapEvent::LocalCertificateCreated
                },
            ))
        }
    }
}

fn local_outcome(
    mode: RuntimeTlsBootstrapMode,
    generation: &CommittedTlsGeneration,
    reused: bool,
    replaced: bool,
    event: RuntimeTlsBootstrapEvent,
) -> RuntimeTlsBootstrapOutcome {
    RuntimeTlsBootstrapOutcome {
        schema: LOCAL_TLS_BOOTSTRAP_OUTCOME_SCHEMA.to_owned(),
        mode,
        certificate_chain_path: generation.certificate_chain.clone(),
        public_certificate_path: Some(generation.public_certificate.clone()),
        private_key_path: generation.private_key.clone(),
        certificate_sha256: sha256_file(&generation.certificate_chain).ok(),
        reused_existing_identity: reused,
        replaced_existing_identity: replaced,
        event,
    }
}

fn generate_local_material(
    config: &RuntimeTlsBootstrapConfig,
) -> Result<GeneratedTlsMaterial, LocalTlsError> {
    let mut names = config.dns_names.clone();
    names.extend(config.ip_addresses.iter().map(ToString::to_string));
    let mut params = CertificateParams::new(names)
        .map_err(|error| LocalTlsError::Generate(error.to_string()))?;
    let issued_at = OffsetDateTime::now_utc();
    params.not_before = issued_at - Duration::hours(1);
    params.not_after = issued_at + Duration::days(LOCAL_CERTIFICATE_VALIDITY_DAYS);
    params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    let key = KeyPair::generate().map_err(|error| LocalTlsError::Generate(error.to_string()))?;
    let cert = params
        .self_signed(&key)
        .map_err(|error| LocalTlsError::Generate(error.to_string()))?;
    Ok(GeneratedTlsMaterial {
        certificate_pem: cert.pem(),
        private_key_pem: key.serialize_pem(),
    })
}

async fn commit_generation(
    paths: &LocalTlsPaths,
    material: &GeneratedTlsMaterial,
) -> Result<CommittedTlsGeneration, LocalTlsError> {
    let generation_id = format!("generation-{}-{}", std::process::id(), unique_suffix());
    let temp_generation = paths.generations_root.join(format!(".{generation_id}.tmp"));
    let final_generation = paths.generations_root.join(&generation_id);
    if final_generation.exists() {
        return Err(LocalTlsError::Policy(
            "local TLS generation id collision".to_owned(),
        ));
    }
    fs::create_dir_all(&temp_generation).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    let cert_tmp = temp_generation.join(&paths.certificate_chain_name);
    let public_tmp = temp_generation.join(&paths.public_certificate_name);
    let key_tmp = temp_generation.join(&paths.private_key_name);
    let write_result = (|| {
        write_file(
            &cert_tmp,
            material.certificate_pem.as_bytes(),
            FileMode::Public,
        )?;
        write_file(
            &public_tmp,
            material.certificate_pem.as_bytes(),
            FileMode::Public,
        )?;
        write_file(
            &key_tmp,
            material.private_key_pem.as_bytes(),
            FileMode::Private,
        )
    })();
    if let Err(error) = write_result {
        remove_directory(&temp_generation);
        return Err(error);
    }
    if let Err(error) = enforce_private_key_permissions(&key_tmp) {
        remove_directory(&temp_generation);
        return Err(error);
    }
    if let Err(error) = validate_rustls_pair(&cert_tmp, &key_tmp).await {
        remove_directory(&temp_generation);
        return Err(error);
    }
    sync_generation_directories(&temp_generation, &[&cert_tmp, &public_tmp, &key_tmp])?;
    fs::rename(&temp_generation, &final_generation)
        .map_err(|error| LocalTlsError::Io(error.to_string()))?;
    sync_directory(&paths.generations_root)
        .map_err(|error| LocalTlsError::Io(error.to_string()))?;
    let generation = committed_generation_from_manifest_paths(
        &paths.tls_root,
        generation_id,
        PathBuf::from(GENERATIONS_DIR)
            .join(final_generation.file_name().ok_or_else(|| {
                LocalTlsError::Policy("local TLS generation path lost its file name".to_owned())
            })?)
            .join(&paths.certificate_chain_name),
        PathBuf::from(GENERATIONS_DIR)
            .join(final_generation.file_name().ok_or_else(|| {
                LocalTlsError::Policy("local TLS generation path lost its file name".to_owned())
            })?)
            .join(&paths.public_certificate_name),
        PathBuf::from(GENERATIONS_DIR)
            .join(final_generation.file_name().ok_or_else(|| {
                LocalTlsError::Policy("local TLS generation path lost its file name".to_owned())
            })?)
            .join(&paths.private_key_name),
    )?;
    commit_current_manifest(paths, &generation)?;
    Ok(generation)
}

fn ensure_public_certificate_copy(source: &Path, target: &Path) -> Result<(), LocalTlsError> {
    let bytes = fs::read(source).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    if target.exists()
        && fs::read(target).map_err(|error| LocalTlsError::Io(error.to_string()))? == bytes
    {
        return Ok(());
    }
    let parent = target.parent().ok_or_else(|| {
        LocalTlsError::Policy("public certificate path must have a parent directory".to_owned())
    })?;
    let nonce = format!("{}.{}", std::process::id(), unique_suffix());
    let candidate = parent.join(format!("public-certificate.{nonce}.tmp"));
    write_file(&candidate, &bytes, FileMode::Public)?;
    replace_file_atomically(&candidate, target).map_err(|error| {
        let _ = fs::remove_file(&candidate);
        LocalTlsError::Io(format!(
            "repair local TLS public certificate copy failed: {error}"
        ))
    })?;
    sync_directory(parent).map_err(|error| LocalTlsError::Io(error.to_string()))
}

async fn validate_rustls_pair(certificate: &Path, private_key: &Path) -> Result<(), LocalTlsError> {
    axum_server::tls_rustls::RustlsConfig::from_pem_file(certificate, private_key)
        .await
        .map(|_| ())
        .map_err(|error| LocalTlsError::Rustls(error.to_string()))
}

fn verify_certificate_sans(
    certificate: &Path,
    config: &RuntimeTlsBootstrapConfig,
) -> Result<(), LocalTlsError> {
    let bytes = fs::read(certificate).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    verify_generated_certificate_sans(&bytes, config)
}

fn verify_generated_certificate_sans(
    certificate_pem: &[u8],
    config: &RuntimeTlsBootstrapConfig,
) -> Result<(), LocalTlsError> {
    let expected = ConfiguredSans::from_config(config);
    let observed = certificate_sans(certificate_pem)?;
    if observed != expected {
        return Err(LocalTlsError::Policy(
            "local TLS certificate SANs do not match configured DNS/IP SANs; rerun with replace=true to intentionally replace the local identity"
                .to_owned(),
        ));
    }
    Ok(())
}

fn certificate_sans(certificate_pem: &[u8]) -> Result<ConfiguredSans, LocalTlsError> {
    let der = first_pem_certificate_der(certificate_pem)?;
    let (_, certificate) = parse_x509_certificate(&der).map_err(|error| {
        LocalTlsError::Policy(format!("could not parse local TLS certificate: {error}"))
    })?;
    let extension = certificate
        .subject_alternative_name()
        .map_err(|error| {
            LocalTlsError::Policy(format!("could not parse local TLS SAN extension: {error}"))
        })?
        .ok_or_else(|| {
            LocalTlsError::Policy("local TLS certificate is missing subjectAltName".to_owned())
        })?;
    let mut dns_names = BTreeSet::new();
    let mut ip_addresses = BTreeSet::new();
    for name in &extension.value.general_names {
        match name {
            GeneralName::DNSName(name) => {
                dns_names.insert(name.to_ascii_lowercase());
            }
            GeneralName::IPAddress(bytes) => match bytes {
                [a, b, c, d] => {
                    ip_addresses.insert(IpAddr::from([*a, *b, *c, *d]));
                }
                [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] => {
                    ip_addresses.insert(IpAddr::from([
                        *a, *b, *c, *d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p,
                    ]));
                }
                _ => {
                    return Err(LocalTlsError::Policy(
                        "local TLS certificate contains malformed IP SAN".to_owned(),
                    ));
                }
            },
            _ => {}
        }
    }
    Ok(ConfiguredSans {
        dns_names,
        ip_addresses,
    })
}

fn first_pem_certificate_der(certificate_pem: &[u8]) -> Result<Vec<u8>, LocalTlsError> {
    let text = std::str::from_utf8(certificate_pem)
        .map_err(|error| LocalTlsError::Policy(format!("certificate PEM is not UTF-8: {error}")))?;
    let mut in_certificate = false;
    let mut body = String::new();
    for line in text.lines() {
        match line.trim() {
            "-----BEGIN CERTIFICATE-----" => {
                in_certificate = true;
                body.clear();
            }
            "-----END CERTIFICATE-----" if in_certificate => {
                return base64::engine::general_purpose::STANDARD
                    .decode(body.as_bytes())
                    .map_err(|error| {
                        LocalTlsError::Policy(format!(
                            "certificate PEM is not valid base64: {error}"
                        ))
                    });
            }
            _ if in_certificate => body.push_str(line.trim()),
            _ => {}
        }
    }
    Err(LocalTlsError::Policy(
        "certificate PEM does not contain a certificate block".to_owned(),
    ))
}

fn read_current_generation(
    paths: &LocalTlsPaths,
) -> Result<Option<CommittedTlsGeneration>, LocalTlsError> {
    if !paths.current_manifest.exists() {
        return Ok(None);
    }
    let bytes =
        fs::read(&paths.current_manifest).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    let manifest: GenerationManifest =
        serde_json::from_slice(&bytes).map_err(|error| LocalTlsError::Policy(error.to_string()))?;
    if manifest.schema != GENERATION_MANIFEST_SCHEMA {
        return Err(LocalTlsError::Policy(
            "local TLS generation manifest has unsupported schema".to_owned(),
        ));
    }
    for (field, path) in [
        ("certificate_chain_path", &manifest.certificate_chain_path),
        ("public_certificate_path", &manifest.public_certificate_path),
        ("private_key_path", &manifest.private_key_path),
    ] {
        validate_relative_child(field, path)?;
    }
    let generation = committed_generation_from_manifest_paths(
        &paths.tls_root,
        manifest.generation_id,
        manifest.certificate_chain_path,
        manifest.public_certificate_path,
        manifest.private_key_path,
    )?;
    for path in [
        &generation.certificate_chain,
        &generation.public_certificate,
        &generation.private_key,
    ] {
        if !path.is_file() {
            return Err(LocalTlsError::Policy(format!(
                "local TLS current generation is incomplete: {} is missing",
                path.display()
            )));
        }
    }
    let observed_sha = sha256_file(&generation.certificate_chain)?;
    let manifest_sha = serde_json::from_slice::<GenerationManifest>(&bytes)
        .map_err(|error| LocalTlsError::Policy(error.to_string()))?
        .certificate_sha256;
    if observed_sha != manifest_sha {
        return Err(LocalTlsError::Policy(
            "local TLS current generation certificate digest does not match manifest".to_owned(),
        ));
    }
    Ok(Some(generation))
}

fn committed_generation_from_manifest_paths(
    tls_root: &Path,
    generation_id: String,
    certificate_chain_path: PathBuf,
    public_certificate_path: PathBuf,
    private_key_path: PathBuf,
) -> Result<CommittedTlsGeneration, LocalTlsError> {
    Ok(CommittedTlsGeneration {
        generation_id,
        certificate_chain: tls_root.join(certificate_chain_path),
        public_certificate: tls_root.join(public_certificate_path),
        private_key: tls_root.join(private_key_path),
    })
}

fn commit_current_manifest(
    paths: &LocalTlsPaths,
    generation: &CommittedTlsGeneration,
) -> Result<(), LocalTlsError> {
    let manifest = GenerationManifest {
        schema: GENERATION_MANIFEST_SCHEMA.to_owned(),
        generation_id: generation.generation_id.clone(),
        certificate_chain_path: strip_tls_root(&paths.tls_root, &generation.certificate_chain)?,
        public_certificate_path: strip_tls_root(&paths.tls_root, &generation.public_certificate)?,
        private_key_path: strip_tls_root(&paths.tls_root, &generation.private_key)?,
        certificate_sha256: sha256_file(&generation.certificate_chain)?,
    };
    let temporary = paths
        .tls_root
        .join(format!("current-generation.{}.json.tmp", unique_suffix()));
    write_file(
        &temporary,
        serde_json::to_vec_pretty(&manifest)
            .map_err(|error| LocalTlsError::Io(error.to_string()))?
            .as_slice(),
        FileMode::Public,
    )?;
    replace_file_atomically(&temporary, &paths.current_manifest).map_err(|error| {
        let _ = fs::remove_file(&temporary);
        LocalTlsError::Io(format!(
            "commit local TLS generation manifest failed: {error}"
        ))
    })?;
    let _ = sync_current_manifest_after_swap(paths);
    Ok(())
}

fn strip_tls_root(root: &Path, path: &Path) -> Result<PathBuf, LocalTlsError> {
    path.strip_prefix(root)
        .map(Path::to_path_buf)
        .map_err(|_| LocalTlsError::Policy("local TLS generation escaped tls_root".to_owned()))
}

fn sync_current_manifest_after_swap(paths: &LocalTlsPaths) -> Result<(), LocalTlsError> {
    #[cfg(test)]
    if FORCE_POST_SWAP_MANIFEST_SYNC_FAILURE.swap(false, Ordering::SeqCst) {
        return Err(LocalTlsError::Io(
            "forced post-swap manifest directory sync failure".to_owned(),
        ));
    }
    sync_directory(&paths.tls_root).map_err(|error| LocalTlsError::Io(error.to_string()))
}

fn local_paths(config: &RuntimeTlsBootstrapConfig) -> Result<LocalTlsPaths, LocalTlsError> {
    let state_root = config
        .state_root
        .as_ref()
        .ok_or_else(|| LocalTlsError::Policy("state_root is required".to_owned()))?;
    let tls_dir = config
        .tls_dir
        .as_ref()
        .ok_or_else(|| LocalTlsError::Policy("tls_dir is required".to_owned()))?;
    let public_certificate = config
        .public_certificate_path
        .as_ref()
        .ok_or_else(|| LocalTlsError::Policy("public_certificate_path is required".to_owned()))?;
    let tls_root = state_root.join(tls_dir);
    Ok(LocalTlsPaths {
        lock_file: tls_root.join(".bootstrap.lock"),
        generations_root: tls_root.join(GENERATIONS_DIR),
        current_manifest: tls_root.join(CURRENT_GENERATION_MANIFEST),
        certificate_chain_name: config.certificate_chain_path.clone(),
        public_certificate_name: public_certificate.clone(),
        private_key_name: config.private_key_path.clone(),
        tls_root,
    })
}

fn validate_relative_child(field: &'static str, path: &Path) -> Result<(), LocalTlsError> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(LocalTlsError::Policy(format!("{field} must be relative")));
    }
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(LocalTlsError::Policy(format!(
            "{field} must stay inside the configured state root"
        )));
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum FileMode {
    Public,
    Private,
}

fn write_file(path: &Path, bytes: &[u8], mode: FileMode) -> Result<(), LocalTlsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    }
    let mut options = OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(match mode {
            FileMode::Public => 0o644,
            FileMode::Private => 0o600,
        });
    }
    let mut file = options
        .open(path)
        .map_err(|error| LocalTlsError::Io(error.to_string()))?;
    #[cfg(windows)]
    if matches!(mode, FileMode::Private) {
        if let Err(error) = enforce_private_key_permissions(path) {
            drop(file);
            let _ = fs::remove_file(path);
            return Err(error);
        }
    }
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| LocalTlsError::Io(error.to_string()))?;
    Ok(())
}

fn sync_generation_directories(generation: &Path, files: &[&Path]) -> Result<(), LocalTlsError> {
    for file in files {
        let opened = File::open(file).map_err(|error| LocalTlsError::Io(error.to_string()))?;
        opened
            .sync_all()
            .map_err(|error| LocalTlsError::Io(error.to_string()))?;
    }
    sync_directory(generation).map_err(|error| LocalTlsError::Io(error.to_string()))
}

fn remove_directory(path: &Path) {
    let _ = fs::remove_dir_all(path);
}

#[cfg(unix)]
fn replace_file_atomically(candidate: &Path, target: &Path) -> Result<(), std::io::Error> {
    fs::rename(candidate, target)
}

#[cfg(windows)]
fn replace_file_atomically(candidate: &Path, target: &Path) -> Result<(), std::io::Error> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MoveFileExW, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH,
    };

    fn wide(path: &Path) -> Vec<u16> {
        path.as_os_str().encode_wide().chain(Some(0)).collect()
    }

    let candidate = wide(candidate);
    let target = wide(target);
    let moved = unsafe {
        MoveFileExW(
            candidate.as_ptr(),
            target.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if moved == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), std::io::Error> {
    fs::File::open(path)?.sync_all()
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), std::io::Error> {
    Ok(())
}

#[cfg(unix)]
fn enforce_private_key_permissions(path: &Path) -> Result<(), LocalTlsError> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let metadata = fs::metadata(path).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    if metadata.mode() & 0o777 != 0o600 {
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|error| LocalTlsError::Io(error.to_string()))?;
    }
    let repaired = fs::metadata(path).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    if repaired.mode() & 0o777 != 0o600 {
        return Err(LocalTlsError::Policy(
            "local TLS private key permissions are not 0600".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn enforce_private_key_permissions(path: &Path) -> Result<(), LocalTlsError> {
    use std::{ffi::c_void, os::windows::ffi::OsStrExt, ptr};
    use windows_sys::Win32::{
        Foundation::{CloseHandle, LocalFree, ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS, HANDLE},
        Security::{
            Authorization::{
                SetEntriesInAclW, SetNamedSecurityInfoW, EXPLICIT_ACCESS_W, SET_ACCESS,
                SE_FILE_OBJECT, TRUSTEE_IS_SID, TRUSTEE_IS_USER, TRUSTEE_W,
            },
            GetTokenInformation, TokenUser, DACL_SECURITY_INFORMATION, NO_INHERITANCE,
            PROTECTED_DACL_SECURITY_INFORMATION, TOKEN_QUERY, TOKEN_USER,
        },
        System::Threading::{GetCurrentProcess, OpenProcessToken},
    };

    let metadata = fs::metadata(path).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    if !metadata.is_file() {
        return Err(LocalTlsError::Policy(
            "local TLS private key must be a regular file".to_owned(),
        ));
    }

    struct Handle(HANDLE);
    impl Drop for Handle {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    CloseHandle(self.0);
                }
            }
        }
    }

    struct LocalAlloc(*mut c_void);
    impl Drop for LocalAlloc {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    LocalFree(self.0);
                }
            }
        }
    }

    unsafe {
        let mut raw_token: HANDLE = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut raw_token) == 0 {
            return Err(LocalTlsError::Io(format!(
                "open current process token failed: {}",
                std::io::Error::last_os_error()
            )));
        }
        let token = Handle(raw_token);

        let mut token_user_len = 0u32;
        let first =
            GetTokenInformation(token.0, TokenUser, ptr::null_mut(), 0, &mut token_user_len);
        if first != 0
            || std::io::Error::last_os_error().raw_os_error()
                != Some(ERROR_INSUFFICIENT_BUFFER as i32)
        {
            return Err(LocalTlsError::Io(format!(
                "size current process token user failed: {}",
                std::io::Error::last_os_error()
            )));
        }
        let mut token_user_bytes = vec![0u8; token_user_len as usize];
        if GetTokenInformation(
            token.0,
            TokenUser,
            token_user_bytes.as_mut_ptr().cast(),
            token_user_len,
            &mut token_user_len,
        ) == 0
        {
            return Err(LocalTlsError::Io(format!(
                "read current process token user failed: {}",
                std::io::Error::last_os_error()
            )));
        }
        let token_user = &*(token_user_bytes.as_ptr() as *const TOKEN_USER);

        let trustee = TRUSTEE_W {
            pMultipleTrustee: ptr::null_mut(),
            MultipleTrusteeOperation: 0,
            TrusteeForm: TRUSTEE_IS_SID,
            TrusteeType: TRUSTEE_IS_USER,
            ptstrName: token_user.User.Sid.cast(),
        };
        let access = EXPLICIT_ACCESS_W {
            grfAccessPermissions: windows_sys::Win32::Foundation::GENERIC_ALL,
            grfAccessMode: SET_ACCESS,
            grfInheritance: NO_INHERITANCE,
            Trustee: trustee,
        };
        let mut acl = ptr::null_mut();
        let acl_status = SetEntriesInAclW(1, &access, ptr::null(), &mut acl);
        if acl_status != ERROR_SUCCESS {
            return Err(LocalTlsError::Io(format!(
                "build restrictive private key ACL failed: {}",
                std::io::Error::from_raw_os_error(acl_status as i32)
            )));
        }
        let _acl = LocalAlloc(acl.cast());

        let mut wide_path = path
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let security_status = SetNamedSecurityInfoW(
            wide_path.as_mut_ptr(),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            ptr::null_mut(),
            ptr::null_mut(),
            acl,
            ptr::null_mut(),
        );
        if security_status != ERROR_SUCCESS {
            return Err(LocalTlsError::Io(format!(
                "apply restrictive private key ACL failed: {}",
                std::io::Error::from_raw_os_error(security_status as i32)
            )));
        }
    }

    Ok(())
}

#[cfg(not(any(unix, windows)))]
fn enforce_private_key_permissions(path: &Path) -> Result<(), LocalTlsError> {
    let metadata = fs::metadata(path).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    if !metadata.is_file() {
        return Err(LocalTlsError::Policy(
            "local TLS private key must be a regular file".to_owned(),
        ));
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String, LocalTlsError> {
    let bytes = fs::read(path).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn unique_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().to_string())
        .unwrap_or_else(|_| "0".to_owned())
}

struct LocalBootstrapGuard {
    path: PathBuf,
    file: File,
}

impl LocalBootstrapGuard {
    fn acquire(path: &Path) -> Result<Self, LocalTlsError> {
        let canonical_key = path.to_path_buf();
        let locks = in_process_locks();
        {
            let mut active = locks
                .lock()
                .map_err(|_| LocalTlsError::Policy("local TLS lock poisoned".to_owned()))?;
            if !active.insert(canonical_key.clone()) {
                return Err(LocalTlsError::LockBusy);
            }
        }
        match Self::acquire_inner(path, canonical_key.clone()) {
            Ok(guard) => Ok(guard),
            Err(error) => {
                let mut active = locks
                    .lock()
                    .map_err(|_| LocalTlsError::Policy("local TLS lock poisoned".to_owned()))?;
                active.remove(&canonical_key);
                Err(error)
            }
        }
    }

    fn acquire_inner(path: &Path, canonical_key: PathBuf) -> Result<Self, LocalTlsError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| LocalTlsError::Io(error.to_string()))?;
        }
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)
            .map_err(|error| LocalTlsError::Io(error.to_string()))?;
        file.try_lock_exclusive().map_err(|error| {
            if error.kind() == std::io::ErrorKind::WouldBlock {
                LocalTlsError::LockBusy
            } else {
                LocalTlsError::Io(error.to_string())
            }
        })?;
        Ok(Self {
            path: canonical_key,
            file,
        })
    }
}

impl Drop for LocalBootstrapGuard {
    fn drop(&mut self) {
        let _ = self.file.unlock();
        if let Ok(mut active) = in_process_locks().lock() {
            active.remove(&self.path);
        }
    }
}

fn in_process_locks() -> &'static Mutex<BTreeSet<PathBuf>> {
    static LOCKS: OnceLock<Mutex<BTreeSet<PathBuf>>> = OnceLock::new();
    LOCKS.get_or_init(|| Mutex::new(BTreeSet::new()))
}

#[cfg(test)]
mod manifest_commit_tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn local_config(root: PathBuf) -> RuntimeTlsBootstrapConfig {
        RuntimeTlsBootstrapConfig {
            schema: LOCAL_TLS_BOOTSTRAP_SCHEMA.to_owned(),
            mode: RuntimeTlsBootstrapMode::LocalSelfSigned,
            state_root: Some(root),
            tls_dir: Some(PathBuf::from("runtime-tls")),
            certificate_chain_path: PathBuf::from("runtime-local-chain.pem"),
            public_certificate_path: Some(PathBuf::from("runtime-local-public.pem")),
            private_key_path: PathBuf::from("runtime-local-key.pem"),
            dns_names: vec!["localhost".to_owned()],
            ip_addresses: vec![IpAddr::V4(Ipv4Addr::LOCALHOST)],
            replace: false,
        }
    }

    #[tokio::test]
    async fn post_swap_manifest_sync_failure_does_not_report_failed_bootstrap() {
        let temp = tempfile::tempdir().unwrap();
        let mut config = local_config(temp.path().to_path_buf());
        let first = bootstrap_runtime_tls(&config).await.unwrap();
        config.replace = true;
        FORCE_POST_SWAP_MANIFEST_SYNC_FAILURE.store(true, Ordering::SeqCst);

        let second = bootstrap_runtime_tls(&config)
            .await
            .expect("post-swap sync is not returned as a failed identity commit");

        assert_ne!(first.certificate_sha256, second.certificate_sha256);
        let after = bootstrap_runtime_tls(&local_config(temp.path().to_path_buf()))
            .await
            .unwrap();
        assert_eq!(second.certificate_sha256, after.certificate_sha256);
    }
}

#[cfg(all(test, windows))]
mod windows_acl_tests {
    use super::*;
    use std::{
        ffi::c_void,
        net::{IpAddr, Ipv4Addr},
        os::windows::ffi::OsStrExt,
        ptr,
    };
    use windows_sys::Win32::{
        Foundation::{CloseHandle, LocalFree, ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS, HANDLE},
        Security::{
            Authorization::{
                GetExplicitEntriesFromAclW, GetNamedSecurityInfoW, SE_FILE_OBJECT, TRUSTEE_IS_SID,
            },
            EqualSid, GetSecurityDescriptorControl, GetTokenInformation, TokenUser,
            DACL_SECURITY_INFORMATION, SE_DACL_PROTECTED, TOKEN_QUERY, TOKEN_USER,
        },
        System::Threading::{GetCurrentProcess, OpenProcessToken},
    };

    fn local_config(root: PathBuf) -> RuntimeTlsBootstrapConfig {
        RuntimeTlsBootstrapConfig {
            schema: LOCAL_TLS_BOOTSTRAP_SCHEMA.to_owned(),
            mode: RuntimeTlsBootstrapMode::LocalSelfSigned,
            state_root: Some(root),
            tls_dir: Some(PathBuf::from("runtime-tls")),
            certificate_chain_path: PathBuf::from("runtime-local-chain.pem"),
            public_certificate_path: Some(PathBuf::from("runtime-local-public.pem")),
            private_key_path: PathBuf::from("runtime-local-key.pem"),
            dns_names: vec!["localhost".to_owned()],
            ip_addresses: vec![IpAddr::V4(Ipv4Addr::LOCALHOST)],
            replace: false,
        }
    }

    #[tokio::test]
    async fn windows_private_key_acl_is_protected_current_user_only() {
        let temp = tempfile::tempdir().unwrap();
        let outcome = bootstrap_runtime_tls(&local_config(temp.path().to_path_buf()))
            .await
            .unwrap();
        assert_windows_private_key_acl(&outcome.private_key_path);
    }

    fn assert_windows_private_key_acl(path: &Path) {
        struct Handle(HANDLE);
        impl Drop for Handle {
            fn drop(&mut self) {
                if !self.0.is_null() {
                    unsafe {
                        CloseHandle(self.0);
                    }
                }
            }
        }
        struct LocalAlloc(*mut c_void);
        impl Drop for LocalAlloc {
            fn drop(&mut self) {
                if !self.0.is_null() {
                    unsafe {
                        LocalFree(self.0);
                    }
                }
            }
        }

        unsafe {
            let mut raw_token: HANDLE = ptr::null_mut();
            assert_ne!(
                OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut raw_token),
                0
            );
            let token = Handle(raw_token);
            let mut token_user_len = 0u32;
            let first =
                GetTokenInformation(token.0, TokenUser, ptr::null_mut(), 0, &mut token_user_len);
            assert_eq!(first, 0);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(ERROR_INSUFFICIENT_BUFFER as i32)
            );
            let mut token_user_bytes = vec![0u8; token_user_len as usize];
            assert_ne!(
                GetTokenInformation(
                    token.0,
                    TokenUser,
                    token_user_bytes.as_mut_ptr().cast(),
                    token_user_len,
                    &mut token_user_len,
                ),
                0
            );
            let token_user = &*(token_user_bytes.as_ptr() as *const TOKEN_USER);

            let mut wide_path = path
                .as_os_str()
                .encode_wide()
                .chain(Some(0))
                .collect::<Vec<_>>();
            let mut dacl = ptr::null_mut();
            let mut descriptor = ptr::null_mut();
            let status = GetNamedSecurityInfoW(
                wide_path.as_mut_ptr(),
                SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut dacl,
                ptr::null_mut(),
                &mut descriptor,
            );
            assert_eq!(status, ERROR_SUCCESS);
            let _descriptor = LocalAlloc(descriptor.cast());
            let mut control = 0u16;
            let mut revision = 0u32;
            assert_ne!(
                GetSecurityDescriptorControl(descriptor, &mut control, &mut revision),
                0
            );
            assert_ne!(control & SE_DACL_PROTECTED, 0);

            let mut count = 0u32;
            let mut entries = ptr::null_mut();
            let entries_status = GetExplicitEntriesFromAclW(dacl, &mut count, &mut entries);
            assert_eq!(entries_status, ERROR_SUCCESS);
            let _entries = LocalAlloc(entries.cast());
            assert_eq!(count, 1);
            let entry = *entries;
            assert_eq!(entry.Trustee.TrusteeForm, TRUSTEE_IS_SID);
            assert_ne!(
                EqualSid(entry.Trustee.ptstrName.cast(), token_user.User.Sid),
                0
            );
        }
    }
}
