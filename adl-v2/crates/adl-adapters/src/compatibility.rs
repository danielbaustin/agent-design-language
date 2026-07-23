use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityInput {
    pub version: String,
    pub provider: String,
    pub model: Option<String>,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityError {
    UnknownVersion,
    UnknownProvider,
    LossyPayload,
}

#[derive(Debug, Clone)]
pub struct CompatibilityAdapter {
    versions: Vec<String>,
    providers: Vec<String>,
}

impl CompatibilityAdapter {
    pub fn new(versions: Vec<String>, providers: Vec<String>) -> Self {
        Self {
            versions,
            providers,
        }
    }

    pub fn translate(
        &self,
        input: CompatibilityInput,
    ) -> Result<CompatibilityInput, CompatibilityError> {
        if !self.versions.contains(&input.version) {
            return Err(CompatibilityError::UnknownVersion);
        }
        if !self.providers.contains(&input.provider) {
            return Err(CompatibilityError::UnknownProvider);
        }
        if !matches!(input.payload, Value::Object(_)) {
            return Err(CompatibilityError::LossyPayload);
        }
        Ok(input)
    }
}
