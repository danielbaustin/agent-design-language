use super::execute;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(crate) const RUN_STATE_SCHEMA_VERSION: &str = "run_state.v1";
pub(crate) const PAUSE_STATE_SCHEMA_VERSION: &str = "pause_state.v1";

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunStateArtifact {
    pub(crate) schema_version: String,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) version: String,
    pub(crate) status: String,
    pub(crate) error_message: Option<String>,
    pub(crate) start_time_ms: u128,
    pub(crate) end_time_ms: u128,
    pub(crate) duration_ms: u128,
    pub(crate) execution_plan_hash: String,
    #[serde(default)]
    pub(crate) scheduler_max_concurrency: Option<usize>,
    #[serde(default)]
    pub(crate) scheduler_policy_source: Option<String>,
    #[serde(default)]
    pub(crate) steering_history: Vec<execute::SteeringRecord>,
    pub(crate) pause: Option<execute::PauseState>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PauseStateArtifact {
    pub(crate) schema_version: String,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) version: String,
    pub(crate) status: String,
    pub(crate) adl_path: String,
    pub(crate) execution_plan_hash: String,
    #[serde(default)]
    pub(crate) steering_history: Vec<execute::SteeringRecord>,
    pub(crate) pause: execute::PauseState,
}

fn normalize_pause_adl_ref(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::Normal(part) => parts.push(part.to_string_lossy().to_string()),
            std::path::Component::ParentDir => parts.push("..".to_string()),
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {}
        }
    }
    if parts.is_empty() {
        "<unknown>".to_string()
    } else {
        parts.join("/")
    }
}

pub(crate) fn sanitize_pause_adl_path(adl_path: &Path) -> String {
    if !adl_path.is_absolute() {
        return normalize_pause_adl_ref(adl_path);
    }
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(rel) = adl_path.strip_prefix(&cwd) {
            return normalize_pause_adl_ref(rel);
        }
    }
    if let Some(file_name) = adl_path.file_name() {
        return format!("external:/{}", file_name.to_string_lossy());
    }
    "external:/<unknown>".to_string()
}
