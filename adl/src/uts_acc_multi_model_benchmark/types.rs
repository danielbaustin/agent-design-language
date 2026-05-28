use serde::{Deserialize, Serialize};

use crate::model_identity::{
    BenchmarkIdentityV1, EvaluatorIdentityV1, LaneIdentityV1, ModelIdentityV1,
};

pub const UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/uts_acc_multi_model_benchmark_report.json";
pub const UTS_ACC_MULTI_MODEL_BENCHMARK_SUMMARY_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/uts_acc_multi_model_benchmark_summary.md";
pub const UTS_ACC_MULTI_MODEL_BENCHMARK_PROMPT_VERSION: &str =
    "wp02.uts_acc_multi_model_benchmark.v1.1";

pub(crate) const UTS_ACC_MULTI_MODEL_BENCHMARK_SCHEMA_VERSION: &str =
    "uts_acc_multi_model_benchmark.v1";
pub(crate) const UTS_ACC_MULTI_MODEL_BENCHMARK_ISSUE_NUMBER: u32 = 3076;
pub(crate) const LOCAL_PROVIDER_ID: &str = "local_ollama_http";
pub(crate) const REMOTE_PROVIDER_ID: &str = "remote_ollama_http";
pub(crate) const TOOL_PROPOSAL_MODE: &str = "adl_json_proposal";
pub(crate) const PROVIDER_COMPLETE_MAX_ATTEMPTS: usize = 1;
pub(crate) const PROVIDER_RETRY_DELAY_MILLIS: u64 = 1500;
pub(crate) const DEFAULT_TASK_PANEL_PATH: &str = "tools/benchmark/uts_33_task_panel.json";
pub(crate) const UTS_ACC_MULTI_MODEL_BENCHMARK_RUNNER_VERSION: &str =
    "uts_acc_multi_model_benchmark_runner.v1.2";

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TaskPanelFile {
    pub tasks: Vec<TaskPanelEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TaskPanelEntry {
    pub id: String,
    pub tool_name: Option<String>,
    pub kind: String,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsAccMultiModelBenchmarkPromptRecord {
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub interface_mode: &'static str,
    pub prompt_contract_summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UtsAccMultiModelSelectionSource {
    ExplicitInput,
    ExplicitEnv,
    RuntimeDiscovery,
    RuntimeDiscoveryEmpty,
    DefaultFallback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsAccBenchmarkTaskRecord {
    pub id: &'static str,
    pub scenario: &'static str,
    pub prompt_summary: &'static str,
    pub expected_behavior: &'static str,
    pub rubric_dimensions: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UtsAccBenchmarkTaskKind {
    MustCompile { expected_tool: &'static str },
    FailClosed { allowed_tool: &'static str },
}

#[derive(Debug, Clone)]
pub(crate) struct UtsAccBenchmarkTaskFixture {
    pub record: UtsAccBenchmarkTaskRecord,
    pub prompt: String,
    pub kind: UtsAccBenchmarkTaskKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UtsAccMultiModelRunStatus {
    Evaluated,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UtsAccBenchmarkClassification {
    ValidUsable,
    SchemaCloseRepairable,
    JsonValidButUtsInvalid,
    UtsValidButAccInvalid,
    Unusable,
    Refused,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsAccBenchmarkTaskResult {
    pub task_id: &'static str,
    pub expected_behavior: &'static str,
    pub classification: UtsAccBenchmarkClassification,
    pub json_valid: bool,
    pub proposal_tool_name: Option<String>,
    pub compiler_decision: Option<crate::uts_acc_compiler::UtsAccCompilerDecisionV1>,
    pub compiler_rejection_code: Option<crate::uts_acc_compiler::UtsAccCompilerRejectionCodeV1>,
    pub authority_humility: bool,
    pub duration_ms: Option<u64>,
    pub passed: bool,
    pub raw_response_excerpt: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsAccBenchmarkScorecard {
    pub valid_usable_count: usize,
    pub schema_close_repairable_count: usize,
    pub json_valid_but_uts_invalid_count: usize,
    pub uts_valid_but_acc_invalid_count: usize,
    pub unusable_count: usize,
    pub refused_count: usize,
    pub passed_count: usize,
    pub total_cases: usize,
    pub supports_governed_tool_use: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsAccBenchmarkConditions {
    pub provider_id: String,
    pub model_id: String,
    pub transport: String,
    pub live_model: bool,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsAccBenchmarkModelResult {
    pub candidate_id: String,
    pub run_status: UtsAccMultiModelRunStatus,
    pub skip_reason: Option<String>,
    pub model_identity: ModelIdentityV1,
    pub conditions: UtsAccBenchmarkConditions,
    pub scorecard: Option<UtsAccBenchmarkScorecard>,
    pub cases: Vec<UtsAccBenchmarkTaskResult>,
    pub failure_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OllamaPsEntry {
    pub model_id: String,
    pub until: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UtsAccMultiModelBenchmarkReport {
    pub schema_version: &'static str,
    pub prompt_record: UtsAccMultiModelBenchmarkPromptRecord,
    pub benchmark_identity: BenchmarkIdentityV1,
    pub evaluator_identity: EvaluatorIdentityV1,
    pub lane_identities: Vec<LaneIdentityV1>,
    pub evidence_status: UtsAccMultiModelBenchmarkEvidenceStatus,
    pub selection_source: UtsAccMultiModelSelectionSource,
    pub selected_models: Vec<String>,
    pub task_count: usize,
    pub candidate_count: usize,
    pub evaluated_count: usize,
    pub usable_model_count: usize,
    pub tasks: Vec<UtsAccBenchmarkTaskRecord>,
    pub models: Vec<UtsAccBenchmarkModelResult>,
    pub non_claims: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UtsAccMultiModelBenchmarkEvidenceStatus {
    BlockedNoLocalModels,
    NonProvingNoPassingModels,
    ProvingAtLeastOneModelPassed,
}
