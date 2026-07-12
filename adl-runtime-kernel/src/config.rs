use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ComponentId;

pub const RUNTIME_CONFIG_SCHEMA: &str = "adl.runtime.config.v1";

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
