use std::{
    collections::{BTreeMap, BTreeSet},
    net::{SocketAddr, ToSocketAddrs},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ComponentId;

pub const RUNTIME_CONFIG_SCHEMA: &str = "adl.runtime.config.v1";
pub const RUNTIME_INIT_SCHEMA: &str = "adl.runtime_v3.init.v1";
const MAX_RUNTIME_INIT_MILLIS: u64 = 600_000;
const MAX_RUNTIME_INIT_CAPACITY: usize = 1_000_000;
const MAX_GUARDIAN_RESTART_BUDGET: u32 = 10_000;
const MAX_OBSERVABILITY_FILE_BYTES: u64 = 1024 * 1024 * 1024;
const MAX_OBSERVABILITY_RETAINED_FILES: usize = 128;
const MAX_GUARDIAN_CONFIGURATION_EXIT_CODES: usize = 16;
const MAX_GUARDIAN_LEASE_AUTH_ATTEMPTS: u32 = 32;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CanonicalValue {
    Bool(bool),
    Integer(i64),
    Text(String),
    List(Vec<CanonicalValue>),
    Map(BTreeMap<String, CanonicalValue>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentConfig {
    pub id: ComponentId,
    pub factory: String,
    #[serde(default)]
    pub dependencies: Vec<ComponentId>,
    #[serde(default)]
    pub parameters: BTreeMap<String, CanonicalValue>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WeatherConfig {
    pub sample_millis: u64,
    pub history_capacity: usize,
    pub disk_warning_free_bytes: u64,
    pub disk_stop_free_bytes: u64,
    pub disk_recover_free_bytes: u64,
    pub memory_warning_used_basis_points: u16,
    pub memory_stop_used_basis_points: u16,
    pub memory_recover_used_basis_points: u16,
    pub cpu_warning_basis_points: u16,
    pub cpu_stop_basis_points: u16,
    pub cpu_recover_basis_points: u16,
    pub checkpoint_deadline_millis: u64,
    pub snapshot_concurrency: usize,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        Self {
            sample_millis: 1_000,
            history_capacity: 60,
            disk_warning_free_bytes: 5 * 1024 * 1024 * 1024,
            disk_stop_free_bytes: 2 * 1024 * 1024 * 1024,
            disk_recover_free_bytes: 8 * 1024 * 1024 * 1024,
            memory_warning_used_basis_points: 8_500,
            memory_stop_used_basis_points: 9_500,
            memory_recover_used_basis_points: 7_500,
            cpu_warning_basis_points: 9_000,
            cpu_stop_basis_points: 9_800,
            cpu_recover_basis_points: 8_000,
            checkpoint_deadline_millis: 5_000,
            snapshot_concurrency: 4,
        }
    }
}

impl WeatherConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.sample_millis == 0
            || self.history_capacity == 0
            || self.checkpoint_deadline_millis == 0
            || self.snapshot_concurrency == 0
        {
            return Err(ConfigError::ZeroBound);
        }
        if !(self.disk_stop_free_bytes < self.disk_warning_free_bytes
            && self.disk_warning_free_bytes < self.disk_recover_free_bytes)
        {
            return Err(ConfigError::ThresholdOrder("disk"));
        }
        validate_high_thresholds(
            "memory",
            self.memory_recover_used_basis_points,
            self.memory_warning_used_basis_points,
            self.memory_stop_used_basis_points,
        )?;
        validate_high_thresholds(
            "cpu",
            self.cpu_recover_basis_points,
            self.cpu_warning_basis_points,
            self.cpu_stop_basis_points,
        )
    }
}

fn validate_high_thresholds(
    resource: &'static str,
    recover: u16,
    warning: u16,
    stop: u16,
) -> Result<(), ConfigError> {
    if stop > 10_000 || !(recover < warning && warning < stop) {
        return Err(ConfigError::ThresholdOrder(resource));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeConfig {
    pub schema: String,
    #[serde(default)]
    pub weather: WeatherConfig,
    pub components: Vec<ComponentConfig>,
}

impl RuntimeConfig {
    pub fn from_json(bytes: &[u8]) -> Result<Self, ConfigError> {
        let value: serde_json::Value =
            serde_json::from_slice(bytes).map_err(|error| ConfigError::Json(error.to_string()))?;
        let schema = value
            .get("schema")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        if schema != RUNTIME_CONFIG_SCHEMA {
            return Err(ConfigError::UnsupportedSchema(schema.to_owned()));
        }
        let config: Self =
            serde_json::from_value(value).map_err(|error| ConfigError::Json(error.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.schema != RUNTIME_CONFIG_SCHEMA {
            return Err(ConfigError::UnsupportedSchema(self.schema.clone()));
        }
        self.weather.validate()?;
        let mut ids = BTreeSet::new();
        for component in &self.components {
            if component.id.as_str().trim().is_empty() || component.factory.trim().is_empty() {
                return Err(ConfigError::EmptyIdentity);
            }
            if !ids.insert(component.id.clone()) {
                return Err(ConfigError::DuplicateComponent(component.id.clone()));
            }
            let dependencies = component.dependencies.iter().collect::<BTreeSet<_>>();
            if dependencies.len() != component.dependencies.len()
                || dependencies.contains(&component.id)
            {
                return Err(ConfigError::InvalidDependencies(component.id.clone()));
            }
            for key in component.parameters.keys() {
                let normalized = key.to_ascii_lowercase();
                if ["secret", "password", "token", "credential", "api_key"]
                    .iter()
                    .any(|term| normalized.contains(term))
                {
                    return Err(ConfigError::SecretInCanonicalConfig {
                        component: component.id.clone(),
                        key: key.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn canonical_json(&self) -> Result<String, ConfigError> {
        self.validate()?;
        let mut effective = self.clone();
        effective
            .components
            .sort_by(|left, right| left.id.cmp(&right.id));
        for component in &mut effective.components {
            component.dependencies.sort();
        }
        serde_json::to_string(&effective).map_err(|error| ConfigError::Json(error.to_string()))
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ConfigError {
    #[error("unsupported runtime configuration schema: {0}")]
    UnsupportedSchema(String),
    #[error("invalid runtime configuration JSON: {0}")]
    Json(String),
    #[error("component identity and factory names must be non-empty")]
    EmptyIdentity,
    #[error("duplicate configured component: {0}")]
    DuplicateComponent(ComponentId),
    #[error("component has duplicate or self dependencies: {0}")]
    InvalidDependencies(ComponentId),
    #[error("canonical configuration cannot contain secret field {component}.{key}")]
    SecretInCanonicalConfig { component: ComponentId, key: String },
    #[error("resource thresholds are not ordered for {0}")]
    ThresholdOrder(&'static str),
    #[error("sampling, history, checkpoint, and concurrency bounds must be non-zero")]
    ZeroBound,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInitConfig {
    pub schema: String,
    pub state_root: PathBuf,
    pub binaries: RuntimeBinariesInitConfig,
    pub paths: RuntimePathsInitConfig,
    pub api: RuntimeApiInitConfig,
    pub kernel: RuntimeKernelInitConfig,
    pub credentials: RuntimeCredentialInitConfig,
    pub shutdown: RuntimeShutdownInitConfig,
    pub guardian: RuntimeGuardianInitConfig,
    pub qualification: RuntimeQualificationInitConfig,
    pub observatory: ObservatoryInitConfig,
    pub observability_pipeline: RuntimeObservabilityInitConfig,
    pub weather: WeatherConfig,
}

impl RuntimeInitConfig {
    pub fn load(path: Option<PathBuf>) -> Result<Self, RuntimeInitError> {
        let path = path.ok_or(RuntimeInitError::MissingInitFile)?;
        Self::from_path(path)
    }

    pub fn from_path(path: PathBuf) -> Result<Self, RuntimeInitError> {
        let text = std::fs::read_to_string(&path)
            .map_err(|error| RuntimeInitError::Read(path.clone(), error.to_string()))?;
        Self::from_toml_str(&text)
    }

    pub fn from_toml_str(text: &str) -> Result<Self, RuntimeInitError> {
        let config: Self =
            toml::from_str(text).map_err(|error| RuntimeInitError::Toml(error.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), RuntimeInitError> {
        if self.schema != RUNTIME_INIT_SCHEMA {
            return Err(RuntimeInitError::UnsupportedSchema(self.schema.clone()));
        }
        validate_absolute_path("state_root", &self.state_root)?;
        self.binaries.validate()?;
        self.paths.validate()?;
        self.kernel.validate()?;
        let tls_root = self.paths.tls_root(&self.state_root);
        let credential_root = self.paths.credentials_root(&self.state_root);
        validate_non_empty_trimmed("api.address", &self.api.address)?;
        if self.socket_addrs()?.iter().any(SocketAddr::is_ipv6) {
            return Err(RuntimeInitError::Policy(
                "api.address must resolve only to IPv4".to_owned(),
            ));
        }
        validate_https_base_url("api.public_base_url", &self.api.public_base_url)?;
        if self.api.bind_attempts == 0 || self.api.bind_attempts > 100 {
            return Err(RuntimeInitError::Policy(
                "api.bind_attempts must be between 1 and 100".to_owned(),
            ));
        }
        for (field, value) in [
            ("api.bind_retry_millis", self.api.bind_retry_millis),
            (
                "api.websocket_auth_timeout_millis",
                self.api.websocket_auth_timeout_millis,
            ),
            (
                "api.websocket_refresh_millis",
                self.api.websocket_refresh_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        validate_bounded_capacity(
            "api.websocket_max_frame_bytes",
            self.api.websocket_max_frame_bytes,
        )?;
        validate_distinct_paths(
            "api.tls.certificate_chain_path",
            &self.api.tls.certificate_chain_path,
            "api.tls.private_key_path",
            &self.api.tls.private_key_path,
        )?;
        validate_child_path(
            "api.tls.certificate_chain_path",
            &tls_root,
            &self.api.tls.certificate_chain_path,
        )?;
        validate_child_path(
            "api.tls.private_key_path",
            &tls_root,
            &self.api.tls.private_key_path,
        )?;
        for (field, value) in [
            (
                "credentials.control_key_id",
                &self.credentials.control_key_id,
            ),
            (
                "credentials.control_principal",
                &self.credentials.control_principal,
            ),
            (
                "credentials.operation_key_id",
                &self.credentials.operation_key_id,
            ),
            (
                "credentials.continuity_key_id",
                &self.credentials.continuity_key_id,
            ),
        ] {
            validate_non_empty_trimmed(field, value)?;
        }
        validate_non_empty_trimmed("credentials.sntp_server", &self.credentials.sntp_server)?;
        for (field, path) in [
            (
                "credentials.control_public_key_path",
                &self.credentials.control_public_key_path,
            ),
            (
                "credentials.operation_public_key_path",
                &self.credentials.operation_public_key_path,
            ),
            (
                "credentials.continuity_signing_key_path",
                &self.credentials.continuity_signing_key_path,
            ),
            (
                "credentials.observatory_token_path",
                &self.credentials.observatory_token_path,
            ),
        ] {
            validate_child_path(field, &credential_root, path)?;
        }
        self.shutdown.validate()?;
        self.guardian.validate()?;
        self.qualification.validate()?;
        validate_origin_list(
            "observatory.allowed_origins",
            &self.observatory.allowed_origins,
        )?;
        self.observability_pipeline.validate()?;
        self.weather
            .validate()
            .map_err(|error| RuntimeInitError::Weather(error.to_string()))?;
        Ok(())
    }

    pub fn socket_addrs(&self) -> Result<Vec<SocketAddr>, RuntimeInitError> {
        self.api
            .address
            .to_socket_addrs()
            .map(|addrs| addrs.collect::<Vec<_>>())
            .map_err(|error| RuntimeInitError::BindAddress(error.to_string()))
            .and_then(|addrs| {
                if addrs.is_empty() {
                    Err(RuntimeInitError::BindAddress(
                        "no socket addresses resolved".to_owned(),
                    ))
                } else {
                    Ok(addrs)
                }
            })
    }

    pub fn observatory_allowed_origins(&self) -> Vec<String> {
        self.observatory.allowed_origins.clone()
    }

    pub fn runtime_observability(&self) -> &RuntimeObservabilityInitConfig {
        &self.observability_pipeline
    }

    pub fn guardian_shutdown_grace_millis(&self) -> u64 {
        self.shutdown
            .checkpoint_deadline_millis
            .saturating_add(self.shutdown.kernel_grace_millis)
            .saturating_add(self.shutdown.api_drain_millis)
            .saturating_add(self.shutdown.guardian_margin_millis)
    }

    pub fn state_root(&self) -> &PathBuf {
        &self.state_root
    }

    pub fn continuity_root(&self) -> PathBuf {
        self.paths.continuity_root(&self.state_root)
    }

    pub fn continuity_identity_projection(&self) -> Result<serde_json::Value, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(credentials) = value
            .get_mut("credentials")
            .and_then(serde_json::Value::as_object_mut)
        {
            credentials.remove("continuity_min_generation");
        }
        if let Some(observability) = value
            .get_mut("observability_pipeline")
            .and_then(serde_json::Value::as_object_mut)
        {
            observability.remove("lifecycle_run");
            observability.remove("lifecycle_cycle");
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeBinariesInitConfig {
    pub kernel_path: PathBuf,
}

impl RuntimeBinariesInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        validate_absolute_path("binaries.kernel_path", &self.kernel_path)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimePathsInitConfig {
    pub continuity_dir: PathBuf,
    pub tls_dir: PathBuf,
    pub credentials_dir: PathBuf,
    pub observability_dir: PathBuf,
}

impl RuntimePathsInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        validate_relative_runtime_path("paths.continuity_dir", &self.continuity_dir)?;
        validate_relative_runtime_path("paths.tls_dir", &self.tls_dir)?;
        validate_relative_runtime_path("paths.credentials_dir", &self.credentials_dir)?;
        validate_relative_runtime_path("paths.observability_dir", &self.observability_dir)?;
        Ok(())
    }

    pub fn continuity_root(&self, state_root: &Path) -> PathBuf {
        state_root.join(&self.continuity_dir)
    }

    pub fn tls_root(&self, state_root: &Path) -> PathBuf {
        state_root.join(&self.tls_dir)
    }

    pub fn credentials_root(&self, state_root: &Path) -> PathBuf {
        state_root.join(&self.credentials_dir)
    }

    pub fn observability_root(&self, state_root: &Path) -> PathBuf {
        state_root.join(&self.observability_dir)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeKernelInitConfig {
    pub recorder_capacity: usize,
    pub control_history_capacity: usize,
    pub checkpoint_channel_capacity: usize,
    pub component_readiness_timeout_millis: u64,
    pub observability_poll_millis: u64,
    pub weather_stale_after_millis: u64,
    pub guardian_lease_connect_millis: u64,
    pub guardian_lease_auth_millis: u64,
    pub trusted_time_sample_timeout_millis: u64,
    pub trusted_time_max_offset_millis: u64,
    pub trusted_time_max_round_trip_millis: u64,
    pub trusted_time_retry_millis: u64,
    pub trusted_time_refresh_millis: u64,
}

impl RuntimeKernelInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        for (field, value) in [
            ("kernel.recorder_capacity", self.recorder_capacity),
            (
                "kernel.control_history_capacity",
                self.control_history_capacity,
            ),
            (
                "kernel.checkpoint_channel_capacity",
                self.checkpoint_channel_capacity,
            ),
        ] {
            validate_bounded_capacity(field, value)?;
        }
        for (field, value) in [
            (
                "kernel.component_readiness_timeout_millis",
                self.component_readiness_timeout_millis,
            ),
            (
                "kernel.observability_poll_millis",
                self.observability_poll_millis,
            ),
            (
                "kernel.weather_stale_after_millis",
                self.weather_stale_after_millis,
            ),
            (
                "kernel.guardian_lease_connect_millis",
                self.guardian_lease_connect_millis,
            ),
            (
                "kernel.guardian_lease_auth_millis",
                self.guardian_lease_auth_millis,
            ),
            (
                "kernel.trusted_time_sample_timeout_millis",
                self.trusted_time_sample_timeout_millis,
            ),
            (
                "kernel.trusted_time_max_offset_millis",
                self.trusted_time_max_offset_millis,
            ),
            (
                "kernel.trusted_time_max_round_trip_millis",
                self.trusted_time_max_round_trip_millis,
            ),
            (
                "kernel.trusted_time_retry_millis",
                self.trusted_time_retry_millis,
            ),
            (
                "kernel.trusted_time_refresh_millis",
                self.trusted_time_refresh_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeApiInitConfig {
    pub address: String,
    pub public_base_url: String,
    pub bind_attempts: u32,
    pub bind_retry_millis: u64,
    pub websocket_auth_timeout_millis: u64,
    pub websocket_refresh_millis: u64,
    pub websocket_max_frame_bytes: usize,
    pub tls: RuntimeTlsInitConfig,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeTlsInitConfig {
    pub certificate_chain_path: PathBuf,
    pub private_key_path: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeCredentialInitConfig {
    pub control_public_key_path: PathBuf,
    pub control_key_id: String,
    pub control_principal: String,
    pub operation_public_key_path: PathBuf,
    pub operation_key_id: String,
    pub continuity_signing_key_path: PathBuf,
    pub continuity_key_id: String,
    pub observatory_token_path: PathBuf,
    pub continuity_min_generation: u64,
    pub sntp_server: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeShutdownInitConfig {
    pub checkpoint_deadline_millis: u64,
    pub kernel_grace_millis: u64,
    pub api_drain_millis: u64,
    pub guardian_margin_millis: u64,
}

impl RuntimeShutdownInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        for (field, value) in [
            (
                "shutdown.checkpoint_deadline_millis",
                self.checkpoint_deadline_millis,
            ),
            ("shutdown.kernel_grace_millis", self.kernel_grace_millis),
            ("shutdown.api_drain_millis", self.api_drain_millis),
            (
                "shutdown.guardian_margin_millis",
                self.guardian_margin_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeGuardianInitConfig {
    pub restart_budget: u32,
    pub backoff_base_millis: u64,
    pub backoff_cap_millis: u64,
    pub healthy_window_millis: u64,
    pub lease_auth_timeout_millis: u64,
    pub lease_auth_attempts: u32,
    pub capture_max_bytes: u64,
    pub capture_drain_grace_millis: u64,
    pub configuration_exit_codes: Vec<i32>,
}

impl RuntimeGuardianInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        if self.restart_budget > MAX_GUARDIAN_RESTART_BUDGET {
            return Err(RuntimeInitError::Policy(format!(
                "guardian.restart_budget exceeds {MAX_GUARDIAN_RESTART_BUDGET}"
            )));
        }
        validate_bounded_millis("guardian.backoff_base_millis", self.backoff_base_millis)?;
        validate_bounded_millis("guardian.backoff_cap_millis", self.backoff_cap_millis)?;
        validate_bounded_millis("guardian.healthy_window_millis", self.healthy_window_millis)?;
        validate_bounded_millis(
            "guardian.lease_auth_timeout_millis",
            self.lease_auth_timeout_millis,
        )?;
        validate_bounded_millis(
            "guardian.capture_drain_grace_millis",
            self.capture_drain_grace_millis,
        )?;
        if self.lease_auth_attempts == 0
            || self.lease_auth_attempts > MAX_GUARDIAN_LEASE_AUTH_ATTEMPTS
        {
            return Err(RuntimeInitError::Policy(format!(
                "guardian.lease_auth_attempts must be in 1..={MAX_GUARDIAN_LEASE_AUTH_ATTEMPTS}"
            )));
        }
        if self.capture_max_bytes == 0 || self.capture_max_bytes > MAX_OBSERVABILITY_FILE_BYTES {
            return Err(RuntimeInitError::Policy(format!(
                "guardian.capture_max_bytes must be in 1..={MAX_OBSERVABILITY_FILE_BYTES}"
            )));
        }
        if self.backoff_cap_millis < self.backoff_base_millis {
            return Err(RuntimeInitError::Policy(
                "guardian.backoff_cap_millis must be >= backoff_base_millis".to_owned(),
            ));
        }
        if self.configuration_exit_codes.is_empty()
            || self.configuration_exit_codes.len() > MAX_GUARDIAN_CONFIGURATION_EXIT_CODES
            || self.configuration_exit_codes.iter().any(|code| *code < 0)
        {
            return Err(RuntimeInitError::Policy(
                "guardian.configuration_exit_codes must be a non-empty bounded list of positive exit codes".to_owned(),
            ));
        }
        let unique = self
            .configuration_exit_codes
            .iter()
            .collect::<BTreeSet<_>>();
        if unique.len() != self.configuration_exit_codes.len() {
            return Err(RuntimeInitError::Policy(
                "guardian.configuration_exit_codes must be unique".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeQualificationInitConfig {
    pub readiness_timeout_millis: u64,
    pub readiness_poll_millis: u64,
    pub shutdown_wait_millis: u64,
}

impl RuntimeQualificationInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        for (field, value) in [
            (
                "qualification.readiness_timeout_millis",
                self.readiness_timeout_millis,
            ),
            (
                "qualification.readiness_poll_millis",
                self.readiness_poll_millis,
            ),
            (
                "qualification.shutdown_wait_millis",
                self.shutdown_wait_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        if self.readiness_poll_millis >= self.readiness_timeout_millis {
            return Err(RuntimeInitError::Policy(
                "qualification.readiness_poll_millis must be less than readiness_timeout_millis"
                    .to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservatoryInitConfig {
    pub allowed_origins: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeObservabilityInitConfig {
    pub vector_binary_path: PathBuf,
    pub service_name: String,
    pub revision: String,
    pub guardian_id: String,
    pub lifecycle_suite: String,
    pub lifecycle_run: String,
    pub lifecycle_cycle: String,
    pub trace_filter: String,
    pub otlp_endpoint: Option<String>,
    pub otlp_timeout_millis: u64,
    pub vector_startup_attempts: u32,
    pub vector_startup_backoff_millis: u64,
    pub vector_shutdown_limit_millis: u64,
    pub drain_timeout_millis: u64,
    pub vector_config_path: PathBuf,
    pub ingress_spool_path: PathBuf,
    pub master_log_path: PathBuf,
    pub audit_path: PathBuf,
    pub sequence_checkpoint_path: PathBuf,
    pub vector_data_dir: PathBuf,
    pub spool_max_bytes: u64,
    pub spool_retained_files: usize,
}

impl RuntimeObservabilityInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        validate_absolute_path(
            "observability_pipeline.vector_binary_path",
            &self.vector_binary_path,
        )?;
        for (field, value) in [
            ("observability_pipeline.service_name", &self.service_name),
            ("observability_pipeline.revision", &self.revision),
            ("observability_pipeline.guardian_id", &self.guardian_id),
            (
                "observability_pipeline.lifecycle_suite",
                &self.lifecycle_suite,
            ),
            ("observability_pipeline.lifecycle_run", &self.lifecycle_run),
            (
                "observability_pipeline.lifecycle_cycle",
                &self.lifecycle_cycle,
            ),
            ("observability_pipeline.trace_filter", &self.trace_filter),
        ] {
            validate_non_empty_trimmed(field, value)?;
        }
        for (field, value) in [
            (
                "observability_pipeline.otlp_timeout_millis",
                self.otlp_timeout_millis,
            ),
            (
                "observability_pipeline.vector_startup_backoff_millis",
                self.vector_startup_backoff_millis,
            ),
            (
                "observability_pipeline.drain_timeout_millis",
                self.drain_timeout_millis,
            ),
            (
                "observability_pipeline.vector_shutdown_limit_millis",
                self.vector_shutdown_limit_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        if self.vector_startup_attempts == 0 || self.vector_startup_attempts > 10 {
            return Err(RuntimeInitError::Policy(
                "observability_pipeline.vector_startup_attempts must be between 1 and 10"
                    .to_owned(),
            ));
        }
        if self.vector_shutdown_limit_millis >= self.drain_timeout_millis {
            return Err(RuntimeInitError::Policy(
                "observability_pipeline.vector_shutdown_limit_millis must be less than drain_timeout_millis"
                    .to_owned(),
            ));
        }
        if self.spool_max_bytes == 0 || self.spool_max_bytes > MAX_OBSERVABILITY_FILE_BYTES {
            return Err(RuntimeInitError::Policy(format!(
                "observability_pipeline.spool_max_bytes must be between 1 and {MAX_OBSERVABILITY_FILE_BYTES}"
            )));
        }
        if self.spool_retained_files == 0
            || self.spool_retained_files > MAX_OBSERVABILITY_RETAINED_FILES
        {
            return Err(RuntimeInitError::Policy(format!(
                "observability_pipeline.spool_retained_files must be between 1 and {MAX_OBSERVABILITY_RETAINED_FILES}"
            )));
        }
        if let Some(endpoint) = self.otlp_endpoint.as_deref() {
            validate_observability_otlp_endpoint(endpoint)?;
        }
        for (field, path) in [
            ("vector_config_path", &self.vector_config_path),
            ("ingress_spool_path", &self.ingress_spool_path),
            ("master_log_path", &self.master_log_path),
            ("audit_path", &self.audit_path),
            ("sequence_checkpoint_path", &self.sequence_checkpoint_path),
            ("vector_data_dir", &self.vector_data_dir),
        ] {
            validate_relative_runtime_path(field, path)?;
        }
        Ok(())
    }
}

fn validate_non_empty_trimmed(field: &'static str, value: &str) -> Result<(), RuntimeInitError> {
    if value.trim().is_empty() || value != value.trim() {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be non-empty without surrounding whitespace"
        )));
    }
    Ok(())
}

fn validate_bounded_millis(field: &'static str, value: u64) -> Result<(), RuntimeInitError> {
    if value == 0 || value > MAX_RUNTIME_INIT_MILLIS {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be between 1 and {MAX_RUNTIME_INIT_MILLIS}"
        )));
    }
    Ok(())
}

fn validate_bounded_capacity(field: &'static str, value: usize) -> Result<(), RuntimeInitError> {
    if value == 0 || value > MAX_RUNTIME_INIT_CAPACITY {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be between 1 and {MAX_RUNTIME_INIT_CAPACITY}"
        )));
    }
    Ok(())
}

fn validate_relative_runtime_path(
    field: &'static str,
    path: &Path,
) -> Result<(), RuntimeInitError> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be a non-empty path relative to state_root"
        )));
    }
    if path.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_)
        )
    }) {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must not escape state_root"
        )));
    }
    Ok(())
}

fn validate_observability_otlp_endpoint(value: &str) -> Result<(), RuntimeInitError> {
    let uri = parse_http_uri(value)?;
    if uri.scheme_str() == Some("https") {
        return Ok(());
    }
    let host = uri.host().unwrap_or_default();
    if uri.scheme_str() == Some("http")
        && matches!(host, "localhost" | "127.0.0.1" | "::1" | "[::1]")
    {
        return Ok(());
    }
    Err(RuntimeInitError::Observability(
        "otlp_endpoint must be HTTPS or loopback HTTP".to_owned(),
    ))
}

fn validate_absolute_path(field: &'static str, path: &Path) -> Result<(), RuntimeInitError> {
    if path.as_os_str().is_empty() || !path.is_absolute() {
        return Err(RuntimeInitError::RelativePath(field));
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(RuntimeInitError::RelativePath(field));
    }
    Ok(())
}

fn validate_child_path(
    field: &'static str,
    root: &Path,
    path: &Path,
) -> Result<(), RuntimeInitError> {
    validate_absolute_path(field, path)?;
    if root.exists() && path.exists() {
        let canonical_root = root
            .canonicalize()
            .map_err(|_| RuntimeInitError::PathOutsideStateRoot(field))?;
        let canonical_path = path
            .canonicalize()
            .map_err(|_| RuntimeInitError::PathOutsideStateRoot(field))?;
        if !canonical_path.starts_with(canonical_root) {
            return Err(RuntimeInitError::PathOutsideStateRoot(field));
        }
    } else if !lexically_contains(root, path) {
        return Err(RuntimeInitError::PathOutsideStateRoot(field));
    }
    Ok(())
}

fn validate_distinct_paths(
    left_field: &'static str,
    left: &Path,
    right_field: &'static str,
    right: &Path,
) -> Result<(), RuntimeInitError> {
    if left == right {
        return Err(RuntimeInitError::InvalidTlsPaths);
    }
    validate_absolute_path(left_field, left)?;
    validate_absolute_path(right_field, right)?;
    Ok(())
}

fn lexically_contains(root: &Path, path: &Path) -> bool {
    if path.components().any(|component| {
        matches!(
            component,
            std::path::Component::CurDir | std::path::Component::ParentDir
        )
    }) {
        return false;
    }
    path.starts_with(root)
}

fn validate_https_base_url(field: &'static str, value: &str) -> Result<(), RuntimeInitError> {
    let uri = parse_http_uri(value)?;
    if uri.scheme_str() != Some("https") {
        return Err(RuntimeInitError::InvalidHttpsBaseUrl {
            field,
            value: value.to_owned(),
        });
    }
    if uri.query().is_some() {
        return Err(RuntimeInitError::InvalidHttpsBaseUrl {
            field,
            value: value.to_owned(),
        });
    }
    Ok(())
}

fn validate_origin_list(field: &'static str, origins: &[String]) -> Result<(), RuntimeInitError> {
    if origins.is_empty() {
        return Err(RuntimeInitError::NoAllowedOrigins(field));
    }
    let mut seen = BTreeSet::new();
    for origin in origins {
        if !seen.insert(origin.clone()) {
            return Err(RuntimeInitError::DuplicateOrigin(origin.clone()));
        }
        validate_origin(origin)?;
    }
    Ok(())
}

fn validate_origin(value: &str) -> Result<(), RuntimeInitError> {
    if value == "*" || value.len() > 512 || value.bytes().any(|byte| byte.is_ascii_control()) {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    }
    let uri = parse_http_uri(value)?;
    if uri.scheme_str() != Some("https") || uri.path() != "/" || uri.query().is_some() {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    }
    Ok(())
}

fn parse_http_uri(value: &str) -> Result<axum::http::Uri, RuntimeInitError> {
    let uri = value
        .parse::<axum::http::Uri>()
        .map_err(|_| RuntimeInitError::InvalidOrigin(value.to_owned()))?;
    let Some(scheme) = uri.scheme_str() else {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    };
    if scheme != "http" && scheme != "https" {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    }
    if uri.authority().is_none() {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    }
    Ok(uri)
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RuntimeInitError {
    #[error("runtime serve requires an explicit init file")]
    MissingInitFile,
    #[error("runtime init file could not be read at {0}: {1}")]
    Read(PathBuf, String),
    #[error("invalid runtime init TOML: {0}")]
    Toml(String),
    #[error("unsupported runtime init schema: {0}")]
    UnsupportedSchema(String),
    #[error("runtime init {0} must not be empty")]
    NoAllowedOrigins(&'static str),
    #[error("runtime init {field} must be an HTTPS origin/base URL: {value}")]
    InvalidHttpsBaseUrl { field: &'static str, value: String },
    #[error("runtime init contains a duplicate observatory origin: {0}")]
    DuplicateOrigin(String),
    #[error("runtime init observatory origin is invalid: {0}")]
    InvalidOrigin(String),
    #[error("runtime init bind address did not resolve: {0}")]
    BindAddress(String),
    #[error("runtime init TLS certificate and private-key paths must be non-empty and distinct")]
    InvalidTlsPaths,
    #[error("runtime init {0} must be an absolute path without parent traversal")]
    RelativePath(&'static str),
    #[error("runtime init {0} must stay inside state_root")]
    PathOutsideStateRoot(&'static str),
    #[error("runtime init weather configuration is invalid: {0}")]
    Weather(String),
    #[error("runtime init observability pipeline configuration is invalid: {0}")]
    Observability(String),
    #[error("runtime init policy is invalid: {0}")]
    Policy(String),
}
