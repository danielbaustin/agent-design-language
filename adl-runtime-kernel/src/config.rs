use std::{
    collections::{BTreeMap, BTreeSet},
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ComponentId;

pub const RUNTIME_CONFIG_SCHEMA: &str = "adl.runtime.config.v1";
pub const RUNTIME_INIT_SCHEMA: &str = "adl.runtime_v3.init.v1";

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
#[serde(deny_unknown_fields)]
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
    #[serde(default)]
    pub api: RuntimeApiInitConfig,
    #[serde(default)]
    pub observatory: ObservatoryInitConfig,
    #[serde(default)]
    pub agents: RuntimeAgentsInitConfig,
}

impl RuntimeInitConfig {
    pub fn local_development_default() -> Self {
        Self {
            schema: RUNTIME_INIT_SCHEMA.to_owned(),
            api: RuntimeApiInitConfig {
                address: default_runtime_api_address_option(),
                public_base_url: default_runtime_public_base_url(),
            },
            observatory: ObservatoryInitConfig {
                allowed_origins: default_local_observatory_origins(),
            },
            agents: RuntimeAgentsInitConfig::default(),
        }
    }

    pub fn load(path: Option<PathBuf>) -> Result<Self, RuntimeInitError> {
        if let Some(path) = path {
            return Self::from_path(path);
        }
        let config = Self::local_development_default();
        config.validate()?;
        Ok(config)
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
        self.api.validate()?;
        self.observatory.validate()?;
        self.agents.validate()?;
        Ok(())
    }

    pub fn socket_addrs(&self) -> Result<Vec<SocketAddr>, RuntimeInitError> {
        let address = self
            .api
            .address
            .as_deref()
            .unwrap_or(default_runtime_api_address());
        address
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

    pub fn agent_population(&self) -> crate::AgentPopulationFeed {
        let sample_count = self.agents.count.min(self.agents.sample_limit);
        let width = self.agents.count.max(1).to_string().len().max(4);
        let sample = (1..=sample_count)
            .map(|index| crate::AgentSample {
                id: format!("agent-{index:0width$}"),
                label: format!("Runtime agent {index}"),
                role: "runtime agent".to_owned(),
                state: "running".to_owned(),
                detail: format!("sample {index} of {}", self.agents.count),
            })
            .collect();
        crate::AgentPopulationFeed {
            total_count: self.agents.count,
            rendered_sample_count: sample_count,
            sample,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeAgentsInitConfig {
    #[serde(default = "default_runtime_agent_count")]
    pub count: u64,
    #[serde(default = "default_runtime_agent_sample_limit")]
    pub sample_limit: u64,
}

impl Default for RuntimeAgentsInitConfig {
    fn default() -> Self {
        Self {
            count: default_runtime_agent_count(),
            sample_limit: default_runtime_agent_sample_limit(),
        }
    }
}

impl RuntimeAgentsInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        if self.count == 0 || self.sample_limit == 0 || self.sample_limit > 100 {
            return Err(RuntimeInitError::InvalidAgentPopulation);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeApiInitConfig {
    #[serde(default = "default_runtime_api_address_option")]
    pub address: Option<String>,
    #[serde(default = "default_runtime_public_base_url")]
    pub public_base_url: String,
}

impl Default for RuntimeApiInitConfig {
    fn default() -> Self {
        Self {
            address: default_runtime_api_address_option(),
            public_base_url: default_runtime_public_base_url(),
        }
    }
}

impl RuntimeApiInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        validate_https_base_url("api.public_base_url", &self.public_base_url)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservatoryInitConfig {
    #[serde(default = "default_local_observatory_origins")]
    pub allowed_origins: Vec<String>,
}

impl Default for ObservatoryInitConfig {
    fn default() -> Self {
        Self {
            allowed_origins: default_local_observatory_origins(),
        }
    }
}

impl ObservatoryInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        validate_origin_list("observatory.allowed_origins", &self.allowed_origins)
    }
}

fn default_runtime_api_address() -> &'static str {
    "localhost:20997"
}

fn default_runtime_api_address_option() -> Option<String> {
    Some(default_runtime_api_address().to_owned())
}

fn default_runtime_public_base_url() -> String {
    "https://runtime-gateway-host".to_owned()
}

fn default_local_observatory_origins() -> Vec<String> {
    vec!["https://localhost:8765".to_owned()]
}

fn default_runtime_agent_count() -> u64 {
    1
}

fn default_runtime_agent_sample_limit() -> u64 {
    6
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
    #[error("runtime init agents.count and agents.sample_limit must be positive, with sample_limit <= 100")]
    InvalidAgentPopulation,
}
