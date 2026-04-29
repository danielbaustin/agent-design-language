use anyhow::{anyhow, Context, Result};
use chrono::DateTime;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::collections::BTreeSet;
use std::path::{Component, Path};

const ACIP_MESSAGE_SCHEMA_VERSION: &str = "acip.message.v1";
const ACIP_CONVERSATION_SCHEMA_VERSION: &str = "acip.conversation.v1";
const ACIP_FIXTURE_SCHEMA_VERSION: &str = "acip.fixture.v1";
const ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION: &str = "acip.invocation.contract.v1";
const ACIP_INVOCATION_EVENT_SCHEMA_VERSION: &str = "acip.invocation.event.v1";
const ACIP_INVOCATION_FIXTURE_SCHEMA_VERSION: &str = "acip.invocation.fixture.v1";
const ACIP_CONFORMANCE_REPORT_SCHEMA_VERSION: &str = "acip.conformance.report.v1";
const ACIP_REVIEW_INVOCATION_SCHEMA_VERSION: &str = "acip.review.invocation.v1";
const ACIP_REVIEW_OUTCOME_SCHEMA_VERSION: &str = "acip.review.outcome.v1";
const ACIP_REVIEW_FIXTURE_SCHEMA_VERSION: &str = "acip.review.fixture.v1";
const ACIP_CODING_INVOCATION_SCHEMA_VERSION: &str = "acip.coding.invocation.v1";
const ACIP_CODING_OUTCOME_SCHEMA_VERSION: &str = "acip.coding.outcome.v1";
const ACIP_CODING_FIXTURE_SCHEMA_VERSION: &str = "acip.coding.fixture.v1";
const ACIP_TRACE_BUNDLE_SCHEMA_VERSION: &str = "acip.trace.bundle.v1";
const ACIP_TRACE_FIXTURE_SCHEMA_VERSION: &str = "acip.trace.fixture.v1";
const MAX_CONTENT_CHARS: usize = 4_000;
const MAX_INLINE_SUMMARY_CHARS: usize = 512;
const MAX_LIST_LEN: usize = 16;
const MAX_INLINE_BYTE_LENGTH: u64 = 4_096;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipAddressKindV1 {
    Agent,
    Group,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipIntentV1 {
    Conversation,
    Consultation,
    InvocationSetup,
    ReviewRequest,
    CodingRequest,
    Delegation,
    Negotiation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipVisibilityV1 {
    Private,
    Shared,
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipTraceRequirementV1 {
    None,
    Summary,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipInvocationStatusV1 {
    Requested,
    Completed,
    Refused,
    Failed,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipResponseChannelKindV1 {
    DirectReply,
    ArtifactReply,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipAddressV1 {
    pub kind: AcipAddressKindV1,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipPayloadRefV1 {
    pub payload_kind: String,
    pub payload_ref: String,
    pub media_type: String,
    pub content_sha256: String,
    pub byte_length: u64,
    pub inline_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipAttachmentRefV1 {
    pub name: String,
    pub media_type: String,
    pub content_sha256: String,
    pub byte_length: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipAuthorityScopeV1 {
    pub allowed_actions: Vec<String>,
    pub authority_basis_refs: Vec<String>,
    pub delegation_permitted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipMessageEnvelopeV1 {
    pub schema_version: String,
    pub message_id: String,
    pub conversation_id: String,
    pub timestamp_utc: String,
    pub monotonic_order: u64,
    pub sender: AcipAddressV1,
    pub recipient: AcipAddressV1,
    pub intent: AcipIntentV1,
    pub visibility: AcipVisibilityV1,
    pub trace_requirement: AcipTraceRequirementV1,
    pub content: String,
    pub payload_refs: Vec<AcipPayloadRefV1>,
    pub artifact_refs: Vec<String>,
    pub attachments: Vec<AcipAttachmentRefV1>,
    pub authority_scope: Option<AcipAuthorityScopeV1>,
    pub correlation_id: Option<String>,
    pub prior_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipConversationEnvelopeV1 {
    pub schema_version: String,
    pub messages: Vec<AcipMessageEnvelopeV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipNamedMessageFixtureV1 {
    pub name: String,
    pub message: AcipMessageEnvelopeV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipNegativeMessageCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub value: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipNegativeConversationCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub value: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipFixtureSetV1 {
    pub schema_version: String,
    pub valid_messages: Vec<AcipNamedMessageFixtureV1>,
    pub valid_conversation: AcipConversationEnvelopeV1,
    pub invalid_messages: Vec<AcipNegativeMessageCaseV1>,
    pub invalid_conversations: Vec<AcipNegativeConversationCaseV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationConstraintsV1 {
    pub policy_refs: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub prohibited_actions: Vec<String>,
    pub requires_redaction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipExpectedOutputV1 {
    pub output_id: String,
    pub output_kind: String,
    pub artifact_role: String,
    pub schema_ref: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationStopPolicyV1 {
    pub max_turns: u32,
    pub max_output_artifacts: u32,
    pub completion_condition: String,
    pub stop_on_refusal: bool,
    pub stop_on_failure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipResponseChannelV1 {
    pub kind: AcipResponseChannelKindV1,
    pub channel_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationContractV1 {
    pub schema_version: String,
    pub invocation_id: String,
    pub conversation_id: String,
    pub causal_message_id: String,
    pub caller: AcipAddressV1,
    pub target: AcipAddressV1,
    pub intent: AcipIntentV1,
    pub purpose: String,
    pub input_refs: Vec<String>,
    pub constraints: AcipInvocationConstraintsV1,
    pub expected_outputs: Vec<AcipExpectedOutputV1>,
    pub stop_policy: AcipInvocationStopPolicyV1,
    pub authority_scope: AcipAuthorityScopeV1,
    pub decision_event_ref: String,
    pub response_channel: AcipResponseChannelV1,
    pub trace_requirement: AcipTraceRequirementV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationEventV1 {
    pub schema_version: String,
    pub invocation_id: String,
    pub conversation_id: String,
    pub causal_message_id: String,
    pub caller: AcipAddressV1,
    pub target: AcipAddressV1,
    pub contract_ref: Option<String>,
    pub contract_sha256: Option<String>,
    pub decision_event_ref: String,
    pub input_refs: Vec<String>,
    pub output_refs: Vec<String>,
    pub status: AcipInvocationStatusV1,
    pub stop_reason: String,
    pub refusal_code: Option<String>,
    pub failure_code: Option<String>,
    pub evidence_refs: Vec<String>,
    pub trace_requirement: AcipTraceRequirementV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationNegativeCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub contract: JsonValue,
    pub event: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationFixtureSetV1 {
    pub schema_version: String,
    pub valid_contract: AcipInvocationContractV1,
    pub valid_completed_event: AcipInvocationEventV1,
    pub valid_refused_event: AcipInvocationEventV1,
    pub valid_failed_event: AcipInvocationEventV1,
    pub negative_cases: Vec<AcipInvocationNegativeCaseV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipConformanceSurfaceV1 {
    Message,
    Conversation,
    Invocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipConformanceFixtureClassV1 {
    pub fixture_name: String,
    pub surface: AcipConformanceSurfaceV1,
    pub mode_label: String,
    pub proves: String,
    pub feature_doc_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipConformanceNegativeClassV1 {
    pub case_name: String,
    pub surface: AcipConformanceSurfaceV1,
    pub proves: String,
    pub expected_error_substring: String,
    pub feature_doc_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipConformanceReportV1 {
    pub schema_version: String,
    pub valid_fixture_classes: Vec<AcipConformanceFixtureClassV1>,
    pub negative_fixture_classes: Vec<AcipConformanceNegativeClassV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipReviewDispositionV1 {
    Blessed,
    Blocked,
    NonProving,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipReviewHandoffOutcomeV1 {
    AllowPrFinish,
    FixFindingsAndRerunReview,
    OperatorWaiverRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipReviewIndependencePolicyV1 {
    pub writer_session_id: String,
    pub writer_model_ref: String,
    pub reviewer_session_id: String,
    pub reviewer_model_ref: String,
    pub forbid_same_session: bool,
    pub forbid_same_model_ref: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipReviewDispositionContractV1 {
    pub allowed_dispositions: Vec<AcipReviewDispositionV1>,
    pub blessed_handoff: AcipReviewHandoffOutcomeV1,
    pub blocked_handoff: AcipReviewHandoffOutcomeV1,
    pub non_proving_handoff: AcipReviewHandoffOutcomeV1,
    pub skipped_handoff: AcipReviewHandoffOutcomeV1,
    pub gate_result_required: bool,
    pub findings_required_when_blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipReviewInvocationContractV1 {
    pub schema_version: String,
    pub invocation: AcipInvocationContractV1,
    pub srp_ref: String,
    pub review_packet_ref: String,
    pub evidence_packet_refs: Vec<String>,
    pub independence_policy: AcipReviewIndependencePolicyV1,
    pub disposition_contract: AcipReviewDispositionContractV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipReviewOutcomeV1 {
    pub schema_version: String,
    pub invocation_id: String,
    pub review_result_ref: String,
    pub gate_result_ref: String,
    pub disposition: AcipReviewDispositionV1,
    pub handoff_outcome: AcipReviewHandoffOutcomeV1,
    pub reviewer_session_id: String,
    pub reviewer_model_ref: String,
    pub findings_ref: Option<String>,
    pub residual_risk_refs: Vec<String>,
    pub pr_open_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipReviewNegativeCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub contract: JsonValue,
    pub outcome: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipReviewFixtureSetV1 {
    pub schema_version: String,
    pub valid_contract: AcipReviewInvocationContractV1,
    pub valid_blessed_outcome: AcipReviewOutcomeV1,
    pub valid_blocked_outcome: AcipReviewOutcomeV1,
    pub negative_cases: Vec<AcipReviewNegativeCaseV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipCodingProviderLaneV1 {
    CodexIssueWorktree,
    ChatgptApi,
    ClaudeApi,
    LocalOllama,
    OtherProposalOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipCodingExecutionModeV1 {
    WorktreeEdit,
    UnappliedPatch,
    StructuredProposal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipCodingDispositionV1 {
    PatchReadyForReview,
    ProposalReadyForReview,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipCodingApprovalPolicyV1 {
    pub review_required_before_pr_finish: bool,
    pub required_review_schema_ref: String,
    pub writer_session_id: String,
    pub writer_model_ref: String,
    pub forbid_same_session_blessing: bool,
    pub forbid_same_model_blessing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipCodingInvocationContractV1 {
    pub schema_version: String,
    pub invocation: AcipInvocationContractV1,
    pub provider_lane: AcipCodingProviderLaneV1,
    pub execution_mode: AcipCodingExecutionModeV1,
    pub issue_ref: String,
    pub task_bundle_ref: String,
    pub issue_worktree_required: bool,
    pub allowed_edit_paths: Vec<String>,
    pub validation_commands: Vec<String>,
    pub patch_format: String,
    pub approval_policy: AcipCodingApprovalPolicyV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipCodingOutcomeV1 {
    pub schema_version: String,
    pub invocation_id: String,
    pub provider_lane: AcipCodingProviderLaneV1,
    pub execution_mode: AcipCodingExecutionModeV1,
    pub disposition: AcipCodingDispositionV1,
    pub primary_output_ref: String,
    pub validation_result_refs: Vec<String>,
    pub review_handoff_ref: String,
    pub writer_session_id: String,
    pub writer_model_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipCodingNegativeCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub contract: JsonValue,
    pub outcome: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipCodingFixtureSetV1 {
    pub schema_version: String,
    pub valid_codex_contract: AcipCodingInvocationContractV1,
    pub valid_codex_outcome: AcipCodingOutcomeV1,
    pub valid_patch_contract: AcipCodingInvocationContractV1,
    pub valid_patch_outcome: AcipCodingOutcomeV1,
    pub negative_cases: Vec<AcipCodingNegativeCaseV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipTraceEventKindV1 {
    MessageCreated,
    InvocationContractDeclared,
    DecisionRecorded,
    InvocationCompleted,
    InvocationRefused,
    InvocationFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipTraceAudienceV1 {
    Actor,
    Operator,
    Reviewer,
    Public,
    Observatory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipReplayPostureV1 {
    FixtureBackedDeterministic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipTraceEventV1 {
    pub event_id: String,
    pub conversation_id: String,
    pub invocation_id: Option<String>,
    pub event_kind: AcipTraceEventKindV1,
    pub source_message_id: Option<String>,
    pub contract_ref: Option<String>,
    pub decision_event_ref: Option<String>,
    pub invocation_status: Option<AcipInvocationStatusV1>,
    pub output_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub summary: String,
    pub requires_redaction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipTraceAudienceViewV1 {
    pub audience: AcipTraceAudienceV1,
    pub narrative_ref: String,
    pub visible_event_ids: Vec<String>,
    pub visible_artifact_refs: Vec<String>,
    pub redacted_elements: Vec<String>,
    pub allows_private_payload_refs: bool,
    pub allows_raw_tool_args: bool,
    pub allows_local_host_paths: bool,
    pub allows_rejected_alternative_details: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipReplayContractV1 {
    pub replay_posture: AcipReplayPostureV1,
    pub fixture_ref: String,
    pub fixture_case: String,
    pub deterministic_event_order: bool,
    pub deterministic_redaction_views: bool,
    pub remote_provider_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipTraceBundleV1 {
    pub schema_version: String,
    pub conversation_id: String,
    pub trace_events: Vec<AcipTraceEventV1>,
    pub audience_views: Vec<AcipTraceAudienceViewV1>,
    pub replay_contract: AcipReplayContractV1,
    pub evidence_packet_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipTraceNegativeCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub bundle: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipTraceFixtureSetV1 {
    pub schema_version: String,
    pub valid_completed_bundle: AcipTraceBundleV1,
    pub valid_refused_bundle: AcipTraceBundleV1,
    pub valid_failed_bundle: AcipTraceBundleV1,
    pub negative_cases: Vec<AcipTraceNegativeCaseV1>,
}

pub fn acip_message_envelope_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipMessageEnvelopeV1))
        .context("serialize ACIP message envelope v1 schema")
}

pub fn acip_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipFixtureSetV1))
        .context("serialize ACIP fixture set v1 schema")
}

pub fn acip_invocation_contract_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipInvocationContractV1))
        .context("serialize ACIP invocation contract v1 schema")
}

pub fn acip_invocation_event_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipInvocationEventV1))
        .context("serialize ACIP invocation event v1 schema")
}

pub fn acip_invocation_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipInvocationFixtureSetV1))
        .context("serialize ACIP invocation fixture set v1 schema")
}

pub fn acip_conformance_report_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipConformanceReportV1))
        .context("serialize ACIP conformance report v1 schema")
}

pub fn acip_review_invocation_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipReviewInvocationContractV1))
        .context("serialize ACIP review invocation contract v1 schema")
}

pub fn acip_review_outcome_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipReviewOutcomeV1))
        .context("serialize ACIP review outcome v1 schema")
}

pub fn acip_review_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipReviewFixtureSetV1))
        .context("serialize ACIP review fixture set v1 schema")
}

pub fn acip_coding_invocation_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipCodingInvocationContractV1))
        .context("serialize ACIP coding invocation contract v1 schema")
}

pub fn acip_coding_outcome_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipCodingOutcomeV1))
        .context("serialize ACIP coding outcome v1 schema")
}

pub fn acip_coding_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipCodingFixtureSetV1))
        .context("serialize ACIP coding fixture set v1 schema")
}

pub fn acip_trace_bundle_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipTraceBundleV1))
        .context("serialize ACIP trace bundle v1 schema")
}

pub fn acip_trace_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipTraceFixtureSetV1))
        .context("serialize ACIP trace fixture set v1 schema")
}

pub fn validate_acip_review_invocation_contract_v1_value(
    value: &JsonValue,
) -> Result<AcipReviewInvocationContractV1> {
    let contract: AcipReviewInvocationContractV1 = serde_json::from_value(value.clone())
        .context("parse ACIP review invocation contract v1")?;
    validate_acip_review_invocation_contract_v1(&contract)?;
    Ok(contract)
}

pub fn validate_acip_review_outcome_v1_value(
    contract: &AcipReviewInvocationContractV1,
    value: &JsonValue,
) -> Result<AcipReviewOutcomeV1> {
    let outcome: AcipReviewOutcomeV1 =
        serde_json::from_value(value.clone()).context("parse ACIP review outcome v1")?;
    validate_acip_review_outcome_v1(contract, &outcome)?;
    Ok(outcome)
}

pub fn validate_acip_coding_invocation_contract_v1_value(
    value: &JsonValue,
) -> Result<AcipCodingInvocationContractV1> {
    let contract: AcipCodingInvocationContractV1 = serde_json::from_value(value.clone())
        .context("parse ACIP coding invocation contract v1")?;
    validate_acip_coding_invocation_contract_v1(&contract)?;
    Ok(contract)
}

pub fn validate_acip_coding_outcome_v1_value(
    contract: &AcipCodingInvocationContractV1,
    value: &JsonValue,
) -> Result<AcipCodingOutcomeV1> {
    let outcome: AcipCodingOutcomeV1 =
        serde_json::from_value(value.clone()).context("parse ACIP coding outcome v1")?;
    validate_acip_coding_outcome_v1(contract, &outcome)?;
    Ok(outcome)
}

pub fn validate_acip_trace_bundle_v1_value(value: &JsonValue) -> Result<AcipTraceBundleV1> {
    let bundle: AcipTraceBundleV1 =
        serde_json::from_value(value.clone()).context("parse ACIP trace bundle v1")?;
    validate_acip_trace_bundle_v1(&bundle)?;
    Ok(bundle)
}

pub fn validate_acip_message_envelope_v1_value(value: &JsonValue) -> Result<AcipMessageEnvelopeV1> {
    let envelope: AcipMessageEnvelopeV1 =
        serde_json::from_value(value.clone()).context("parse ACIP message envelope v1")?;
    validate_acip_message_envelope_v1(&envelope)?;
    Ok(envelope)
}

pub fn validate_acip_message_envelope_v1(envelope: &AcipMessageEnvelopeV1) -> Result<()> {
    if envelope.schema_version != ACIP_MESSAGE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP message envelope requires schema_version '{}'",
            ACIP_MESSAGE_SCHEMA_VERSION
        ));
    }
    validate_id(&envelope.message_id, "message_id")?;
    validate_id(&envelope.conversation_id, "conversation_id")?;
    validate_timestamp(&envelope.timestamp_utc, "timestamp_utc")?;
    if envelope.monotonic_order == 0 {
        return Err(anyhow!("monotonic_order must be positive"));
    }
    validate_address(&envelope.sender, "sender")?;
    validate_address(&envelope.recipient, "recipient")?;
    if envelope.sender == envelope.recipient {
        return Err(anyhow!("sender and recipient must not be identical"));
    }
    if let Some(correlation_id) = &envelope.correlation_id {
        validate_id(correlation_id, "correlation_id")?;
    }
    if let Some(prior_message_id) = &envelope.prior_message_id {
        validate_id(prior_message_id, "prior_message_id")?;
    }
    if envelope.content.trim().is_empty()
        && envelope.payload_refs.is_empty()
        && envelope.attachments.is_empty()
    {
        return Err(anyhow!(
            "message must contain content, payload_refs, or attachments"
        ));
    }
    if envelope.content.chars().count() > MAX_CONTENT_CHARS {
        return Err(anyhow!(
            "content exceeds bounded inline posture of {MAX_CONTENT_CHARS} characters"
        ));
    }
    if envelope.payload_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("payload_refs exceeds bounded list length"));
    }
    if envelope.artifact_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("artifact_refs exceeds bounded list length"));
    }
    if envelope.attachments.len() > MAX_LIST_LEN {
        return Err(anyhow!("attachments exceeds bounded list length"));
    }
    for payload_ref in &envelope.payload_refs {
        validate_payload_ref(payload_ref)?;
    }
    for artifact_ref in &envelope.artifact_refs {
        validate_repo_relative_ref(artifact_ref, "artifact_refs[]")?;
    }
    for attachment in &envelope.attachments {
        validate_attachment_ref(attachment)?;
    }
    if let Some(scope) = &envelope.authority_scope {
        validate_authority_scope(scope)?;
        validate_message_intent_authority_alignment(&envelope.intent, scope)?;
    }
    Ok(())
}

pub fn validate_acip_conversation_envelope_v1_value(
    value: &JsonValue,
) -> Result<AcipConversationEnvelopeV1> {
    let envelope: AcipConversationEnvelopeV1 =
        serde_json::from_value(value.clone()).context("parse ACIP conversation envelope v1")?;
    validate_acip_conversation_envelope_v1(&envelope)?;
    Ok(envelope)
}

pub fn validate_acip_conversation_envelope_v1(envelope: &AcipConversationEnvelopeV1) -> Result<()> {
    if envelope.schema_version != ACIP_CONVERSATION_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP conversation envelope requires schema_version '{}'",
            ACIP_CONVERSATION_SCHEMA_VERSION
        ));
    }
    if envelope.messages.is_empty() {
        return Err(anyhow!(
            "ACIP conversation envelope requires at least one message"
        ));
    }
    let mut seen_message_ids = BTreeSet::new();
    let mut prior_order = 0_u64;
    let mut conversation_id: Option<&str> = None;
    for message in &envelope.messages {
        validate_acip_message_envelope_v1(message)?;
        if !seen_message_ids.insert(message.message_id.clone()) {
            return Err(anyhow!(
                "conversation contains duplicate message_id '{}'",
                message.message_id
            ));
        }
        if prior_order >= message.monotonic_order {
            return Err(anyhow!(
                "conversation messages must preserve strictly increasing monotonic_order"
            ));
        }
        prior_order = message.monotonic_order;
        if let Some(expected) = conversation_id {
            if expected != message.conversation_id {
                return Err(anyhow!(
                    "conversation contains message from a different conversation_id"
                ));
            }
        } else {
            conversation_id = Some(message.conversation_id.as_str());
        }
    }
    Ok(())
}

pub fn validate_acip_invocation_contract_v1_value(
    value: &JsonValue,
) -> Result<AcipInvocationContractV1> {
    let contract: AcipInvocationContractV1 =
        serde_json::from_value(value.clone()).context("parse ACIP invocation contract v1")?;
    validate_acip_invocation_contract_v1(&contract)?;
    Ok(contract)
}

pub fn validate_acip_invocation_contract_v1(contract: &AcipInvocationContractV1) -> Result<()> {
    if contract.schema_version != ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP invocation contract requires schema_version '{}'",
            ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION
        ));
    }
    validate_id(&contract.invocation_id, "invocation_id")?;
    validate_id(&contract.conversation_id, "conversation_id")?;
    validate_id(&contract.causal_message_id, "causal_message_id")?;
    validate_address(&contract.caller, "caller")?;
    validate_address(&contract.target, "target")?;
    if contract.caller == contract.target {
        return Err(anyhow!("caller and target must not be identical"));
    }
    match contract.intent {
        AcipIntentV1::InvocationSetup
        | AcipIntentV1::ReviewRequest
        | AcipIntentV1::CodingRequest
        | AcipIntentV1::Delegation => {}
        _ => {
            return Err(anyhow!(
                "invocation contract intent must be invocation_setup, review_request, coding_request, or delegation"
            ))
        }
    }
    validate_non_empty(&contract.purpose, "purpose")?;
    if contract.input_refs.is_empty() {
        return Err(anyhow!("input_refs must not be empty"));
    }
    if contract.input_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("input_refs exceeds bounded list length"));
    }
    for input_ref in &contract.input_refs {
        validate_repo_relative_ref(input_ref, "input_refs[]")?;
    }
    validate_invocation_constraints(&contract.constraints)?;
    validate_invocation_authority_alignment(contract)?;
    if contract.expected_outputs.is_empty() {
        return Err(anyhow!("expected_outputs must not be empty"));
    }
    if contract.expected_outputs.len() > MAX_LIST_LEN {
        return Err(anyhow!("expected_outputs exceeds bounded list length"));
    }
    validate_expected_outputs(&contract.expected_outputs)?;
    validate_stop_policy(&contract.stop_policy)?;
    validate_authority_scope(&contract.authority_scope)?;
    validate_gate_decision_ref(&contract.decision_event_ref, "decision_event_ref")?;
    validate_response_channel(&contract.response_channel)?;
    Ok(())
}

pub fn validate_acip_invocation_event_v1_value(value: &JsonValue) -> Result<AcipInvocationEventV1> {
    let event: AcipInvocationEventV1 =
        serde_json::from_value(value.clone()).context("parse ACIP invocation event v1")?;
    validate_acip_invocation_event_v1(&event)?;
    Ok(event)
}

pub fn validate_acip_invocation_event_v1(event: &AcipInvocationEventV1) -> Result<()> {
    if event.schema_version != ACIP_INVOCATION_EVENT_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP invocation event requires schema_version '{}'",
            ACIP_INVOCATION_EVENT_SCHEMA_VERSION
        ));
    }
    validate_id(&event.invocation_id, "invocation_id")?;
    validate_id(&event.conversation_id, "conversation_id")?;
    validate_id(&event.causal_message_id, "causal_message_id")?;
    validate_address(&event.caller, "caller")?;
    validate_address(&event.target, "target")?;
    validate_gate_decision_ref(&event.decision_event_ref, "decision_event_ref")?;
    if event.input_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("input_refs exceeds bounded list length"));
    }
    for input_ref in &event.input_refs {
        validate_repo_relative_ref(input_ref, "input_refs[]")?;
    }
    if event.output_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("output_refs exceeds bounded list length"));
    }
    for output_ref in &event.output_refs {
        validate_repo_relative_ref(output_ref, "output_refs[]")?;
    }
    if let Some(contract_ref) = &event.contract_ref {
        validate_repo_relative_ref(contract_ref, "contract_ref")?;
    }
    if let Some(contract_sha256) = &event.contract_sha256 {
        validate_sha256(contract_sha256, "contract_sha256")?;
    }
    if event.contract_ref.is_none() && event.contract_sha256.is_none() {
        return Err(anyhow!(
            "invocation event must carry contract_ref or contract_sha256"
        ));
    }
    validate_non_empty(&event.stop_reason, "stop_reason")?;
    if event.evidence_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("evidence_refs exceeds bounded list length"));
    }
    for evidence_ref in &event.evidence_refs {
        validate_repo_relative_ref(evidence_ref, "evidence_refs[]")?;
    }
    validate_invocation_status_consistency(event)?;
    Ok(())
}

pub fn validate_acip_invocation_event_against_contract(
    contract: &AcipInvocationContractV1,
    event: &AcipInvocationEventV1,
) -> Result<()> {
    validate_acip_invocation_contract_v1(contract)?;
    validate_acip_invocation_event_v1(event)?;

    if contract.invocation_id != event.invocation_id {
        return Err(anyhow!(
            "invocation event must match contract invocation_id"
        ));
    }
    if contract.conversation_id != event.conversation_id {
        return Err(anyhow!(
            "invocation event must match contract conversation_id"
        ));
    }
    if contract.causal_message_id != event.causal_message_id {
        return Err(anyhow!(
            "invocation event must match contract causal_message_id"
        ));
    }
    if contract.caller != event.caller || contract.target != event.target {
        return Err(anyhow!(
            "invocation event caller and target must match contract identity binding"
        ));
    }
    if contract.decision_event_ref != event.decision_event_ref {
        return Err(anyhow!(
            "invocation event must preserve the contract decision_event_ref"
        ));
    }
    if contract.input_refs != event.input_refs {
        return Err(anyhow!(
            "invocation event must preserve the contract input_refs"
        ));
    }
    if contract.trace_requirement != event.trace_requirement {
        return Err(anyhow!(
            "invocation event must preserve the contract trace_requirement"
        ));
    }

    let required_output_count = contract
        .expected_outputs
        .iter()
        .filter(|expected| expected.required)
        .count();

    match event.status {
        AcipInvocationStatusV1::Requested => {}
        AcipInvocationStatusV1::Completed => {
            if event.output_refs.len() > contract.stop_policy.max_output_artifacts as usize {
                return Err(anyhow!(
                    "invocation event exceeds stop_policy.max_output_artifacts"
                ));
            }
            if event.output_refs.len() < required_output_count {
                return Err(anyhow!(
                    "completed invocation must satisfy declared required output contracts"
                ));
            }
            for expected in contract
                .expected_outputs
                .iter()
                .filter(|expected| expected.required)
            {
                let matched = event.output_refs.iter().any(|output_ref| {
                    output_ref.contains(&expected.output_id)
                        || output_ref.contains(&expected.output_kind)
                });
                if !matched {
                    return Err(anyhow!(
                        "completed invocation must satisfy declared required output contracts"
                    ));
                }
            }
        }
        AcipInvocationStatusV1::Partial => {
            if event.output_refs.len() > contract.stop_policy.max_output_artifacts as usize {
                return Err(anyhow!(
                    "invocation event exceeds stop_policy.max_output_artifacts"
                ));
            }
            if event.output_refs.is_empty() {
                return Err(anyhow!(
                    "partial invocation must emit at least one output ref"
                ));
            }
        }
        AcipInvocationStatusV1::Refused | AcipInvocationStatusV1::Failed => {}
    }
    Ok(())
}

pub fn validate_acip_fixture_set_v1(fixtures: &AcipFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP fixture set requires schema_version '{}'",
            ACIP_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.valid_messages.is_empty() {
        return Err(anyhow!("ACIP fixture set requires valid_messages"));
    }
    if fixtures.invalid_messages.is_empty() {
        return Err(anyhow!("ACIP fixture set requires invalid_messages"));
    }
    if fixtures.invalid_conversations.is_empty() {
        return Err(anyhow!("ACIP fixture set requires invalid_conversations"));
    }

    let mut seen_names = BTreeSet::new();
    for fixture in &fixtures.valid_messages {
        validate_id(&fixture.name, "valid_messages[].name")?;
        if !seen_names.insert(fixture.name.clone()) {
            return Err(anyhow!(
                "ACIP fixture set contains duplicate valid fixture '{}'",
                fixture.name
            ));
        }
        validate_acip_message_envelope_v1(&fixture.message)?;
    }
    validate_acip_conversation_envelope_v1(&fixtures.valid_conversation)?;

    for case in &fixtures.invalid_messages {
        validate_negative_case_name(&case.name, "invalid_messages[].name")?;
        validate_negative_case(case.value.clone(), &case.expected_error_substring, true)?;
    }
    for case in &fixtures.invalid_conversations {
        validate_negative_case_name(&case.name, "invalid_conversations[].name")?;
        validate_negative_case(case.value.clone(), &case.expected_error_substring, false)?;
    }
    Ok(())
}

pub fn validate_acip_invocation_fixture_set_v1(
    fixtures: &AcipInvocationFixtureSetV1,
) -> Result<()> {
    if fixtures.schema_version != ACIP_INVOCATION_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP invocation fixture set requires schema_version '{}'",
            ACIP_INVOCATION_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!(
            "ACIP invocation fixture set requires negative_cases"
        ));
    }

    validate_acip_invocation_contract_v1(&fixtures.valid_contract)?;
    validate_acip_invocation_event_against_contract(
        &fixtures.valid_contract,
        &fixtures.valid_completed_event,
    )?;
    validate_acip_invocation_event_against_contract(
        &fixtures.valid_contract,
        &fixtures.valid_refused_event,
    )?;
    validate_acip_invocation_event_against_contract(
        &fixtures.valid_contract,
        &fixtures.valid_failed_event,
    )?;

    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        let contract = validate_acip_invocation_contract_v1_value(&case.contract);
        match (&case.event, contract) {
            (Some(event_value), Ok(contract)) => {
                let event = validate_acip_invocation_event_v1_value(event_value);
                let result = match event {
                    Ok(event) => validate_acip_invocation_event_against_contract(&contract, &event),
                    Err(error) => Err(error),
                };
                validate_negative_result(result, &case.expected_error_substring)?;
            }
            (Some(_), Err(error)) | (None, Err(error)) => {
                validate_negative_result(Err(error), &case.expected_error_substring)?;
            }
            (None, Ok(_)) => {
                return Err(anyhow!(
                    "negative case '{}' unexpectedly validated without event failure",
                    case.name
                ));
            }
        }
    }

    Ok(())
}

pub fn validate_acip_conformance_report_v1(report: &AcipConformanceReportV1) -> Result<()> {
    if report.schema_version != ACIP_CONFORMANCE_REPORT_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP conformance report requires schema_version '{}'",
            ACIP_CONFORMANCE_REPORT_SCHEMA_VERSION
        ));
    }
    if report.valid_fixture_classes.is_empty() {
        return Err(anyhow!(
            "ACIP conformance report requires valid_fixture_classes"
        ));
    }
    if report.negative_fixture_classes.is_empty() {
        return Err(anyhow!(
            "ACIP conformance report requires negative_fixture_classes"
        ));
    }

    let mut seen_valid = BTreeSet::new();
    for class in &report.valid_fixture_classes {
        validate_id(&class.fixture_name, "valid_fixture_classes[].fixture_name")?;
        validate_id(&class.mode_label, "valid_fixture_classes[].mode_label")?;
        validate_non_empty(&class.proves, "valid_fixture_classes[].proves")?;
        validate_repo_relative_ref(
            &class.feature_doc_ref,
            "valid_fixture_classes[].feature_doc_ref",
        )?;
        if !seen_valid.insert(class.fixture_name.clone()) {
            return Err(anyhow!(
                "ACIP conformance report contains duplicate valid fixture '{}'",
                class.fixture_name
            ));
        }
    }

    let mut seen_negative = BTreeSet::new();
    for class in &report.negative_fixture_classes {
        validate_id(&class.case_name, "negative_fixture_classes[].case_name")?;
        validate_non_empty(&class.proves, "negative_fixture_classes[].proves")?;
        validate_non_empty(
            &class.expected_error_substring,
            "negative_fixture_classes[].expected_error_substring",
        )?;
        validate_repo_relative_ref(
            &class.feature_doc_ref,
            "negative_fixture_classes[].feature_doc_ref",
        )?;
        if !seen_negative.insert(class.case_name.clone()) {
            return Err(anyhow!(
                "ACIP conformance report contains duplicate negative fixture '{}'",
                class.case_name
            ));
        }
    }

    let required_valid = [
        "conversation",
        "consultation",
        "invocation_setup",
        "review_request",
        "coding_request",
        "coding_agent_handoff",
        "delegation",
        "negotiation",
        "operator_request",
        "broadcast",
        "shared_conversation_thread",
        "governed_invocation_contract",
    ];
    for required in required_valid {
        if !seen_valid.contains(required) {
            return Err(anyhow!(
                "ACIP conformance report missing required valid fixture '{}'",
                required
            ));
        }
    }

    let required_negative = [
        "identity_drift",
        "missing_recipient",
        "hidden_invocation",
        "malformed_payload_refs",
        "unsupported_visibility",
        "raw_local_path_refs",
        "authority_escalation",
        "stale_ordering",
        "missing_gate_rejects_governed_invocation",
        "ambiguous_stop_policy_rejected",
        "unsafe_input_refs_rejected",
        "status_refusal_inconsistency_rejected",
        "output_contract_mismatch_rejected",
    ];
    for required in required_negative {
        if !seen_negative.contains(required) {
            return Err(anyhow!(
                "ACIP conformance report missing required negative fixture '{}'",
                required
            ));
        }
    }

    Ok(())
}

pub fn validate_acip_review_invocation_contract_v1(
    contract: &AcipReviewInvocationContractV1,
) -> Result<()> {
    if contract.schema_version != ACIP_REVIEW_INVOCATION_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP review invocation contract requires schema_version '{}'",
            ACIP_REVIEW_INVOCATION_SCHEMA_VERSION
        ));
    }
    validate_acip_invocation_contract_v1(&contract.invocation)?;
    if contract.invocation.intent != AcipIntentV1::ReviewRequest {
        return Err(anyhow!(
            "ACIP review invocation specialization requires invocation.intent 'review_request'"
        ));
    }
    validate_repo_relative_ref(&contract.srp_ref, "srp_ref")?;
    validate_repo_relative_ref(&contract.review_packet_ref, "review_packet_ref")?;
    if contract.evidence_packet_refs.is_empty() {
        return Err(anyhow!(
            "ACIP review invocation specialization requires evidence_packet_refs"
        ));
    }
    for reference in &contract.evidence_packet_refs {
        validate_repo_relative_ref(reference, "evidence_packet_refs[]")?;
    }
    if !contract
        .invocation
        .input_refs
        .iter()
        .any(|reference| reference == &contract.review_packet_ref)
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires invocation.input_refs to include review_packet_ref"
        ));
    }
    for reference in &contract.evidence_packet_refs {
        if !contract
            .invocation
            .input_refs
            .iter()
            .any(|input| input == reference)
        {
            return Err(anyhow!(
                "ACIP review invocation specialization requires invocation.input_refs to include every evidence_packet_refs entry"
            ));
        }
    }
    if !contract
        .invocation
        .constraints
        .policy_refs
        .iter()
        .any(|policy_ref| policy_ref == &contract.srp_ref)
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires constraints.policy_refs to include srp_ref"
        ));
    }
    if !contract
        .invocation
        .constraints
        .required_capabilities
        .iter()
        .any(|capability| capability == "review")
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires constraints.required_capabilities to include 'review'"
        ));
    }
    if !contract
        .invocation
        .constraints
        .prohibited_actions
        .iter()
        .any(|action| action == "merge")
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires constraints.prohibited_actions to include 'merge'"
        ));
    }
    if !contract
        .invocation
        .constraints
        .prohibited_actions
        .iter()
        .any(|action| action == "push")
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires constraints.prohibited_actions to include 'push'"
        ));
    }
    validate_review_independence_policy(&contract.independence_policy)?;
    validate_review_disposition_contract(&contract.disposition_contract)?;
    Ok(())
}

pub fn validate_acip_review_outcome_v1(
    contract: &AcipReviewInvocationContractV1,
    outcome: &AcipReviewOutcomeV1,
) -> Result<()> {
    validate_acip_review_invocation_contract_v1(contract)?;
    if outcome.schema_version != ACIP_REVIEW_OUTCOME_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP review outcome requires schema_version '{}'",
            ACIP_REVIEW_OUTCOME_SCHEMA_VERSION
        ));
    }
    validate_id(&outcome.invocation_id, "invocation_id")?;
    validate_repo_relative_ref(&outcome.review_result_ref, "review_result_ref")?;
    validate_repo_relative_ref(&outcome.gate_result_ref, "gate_result_ref")?;
    validate_id(&outcome.reviewer_session_id, "reviewer_session_id")?;
    validate_id(&outcome.reviewer_model_ref, "reviewer_model_ref")?;
    if contract.disposition_contract.gate_result_required && outcome.gate_result_ref.is_empty() {
        return Err(anyhow!(
            "ACIP review outcome requires gate_result_ref when gate_result_required is true"
        ));
    }
    if contract.disposition_contract.findings_required_when_blocked
        && outcome.disposition == AcipReviewDispositionV1::Blocked
        && outcome.findings_ref.is_none()
    {
        return Err(anyhow!(
            "blocked review outcome must carry findings_ref under the disposition contract"
        ));
    }
    if let Some(findings_ref) = &outcome.findings_ref {
        validate_repo_relative_ref(findings_ref, "findings_ref")?;
    }
    for reference in &outcome.residual_risk_refs {
        validate_repo_relative_ref(reference, "residual_risk_refs[]")?;
    }
    if contract.invocation.invocation_id != outcome.invocation_id {
        return Err(anyhow!(
            "review outcome must match the invocation contract invocation_id"
        ));
    }
    if contract.independence_policy.reviewer_session_id != outcome.reviewer_session_id {
        return Err(anyhow!(
            "review outcome must preserve the reviewer_session_id declared by the invocation contract"
        ));
    }
    if contract.independence_policy.reviewer_model_ref != outcome.reviewer_model_ref {
        return Err(anyhow!(
            "review outcome must preserve the reviewer_model_ref declared by the invocation contract"
        ));
    }
    if !contract
        .invocation
        .expected_outputs
        .iter()
        .any(|output| output.required && output.output_kind == "review_result")
    {
        return Err(anyhow!(
            "review invocation specialization requires expected_outputs to declare required review_result output"
        ));
    }
    if !contract
        .invocation
        .expected_outputs
        .iter()
        .any(|output| output.required && output.output_kind == "gate_result")
    {
        return Err(anyhow!(
            "review invocation specialization requires expected_outputs to declare required gate_result output"
        ));
    }
    if !outcome.review_result_ref.ends_with("review_result.json") {
        return Err(anyhow!(
            "review outcome review_result_ref must match the declared review_result output contract"
        ));
    }
    if !outcome.gate_result_ref.ends_with("gate_result.json") {
        return Err(anyhow!(
            "review outcome gate_result_ref must match the declared gate_result output contract"
        ));
    }

    let expected_handoff = match outcome.disposition {
        AcipReviewDispositionV1::Blessed => &contract.disposition_contract.blessed_handoff,
        AcipReviewDispositionV1::Blocked => &contract.disposition_contract.blocked_handoff,
        AcipReviewDispositionV1::NonProving => &contract.disposition_contract.non_proving_handoff,
        AcipReviewDispositionV1::Skipped => &contract.disposition_contract.skipped_handoff,
    };
    if &outcome.handoff_outcome != expected_handoff {
        return Err(anyhow!(
            "review outcome handoff must match the disposition contract mapping"
        ));
    }
    if outcome.pr_open_allowed {
        if outcome.disposition != AcipReviewDispositionV1::Blessed {
            return Err(anyhow!("only blessed review outcomes may allow PR finish"));
        }
        if outcome.handoff_outcome != AcipReviewHandoffOutcomeV1::AllowPrFinish {
            return Err(anyhow!(
                "review outcome allowing PR finish must use handoff outcome 'allow_pr_finish'"
            ));
        }
    } else if outcome.disposition == AcipReviewDispositionV1::Blessed
        && outcome.handoff_outcome == AcipReviewHandoffOutcomeV1::AllowPrFinish
    {
        return Err(anyhow!(
            "blessed review outcome with allow_pr_finish handoff must set pr_open_allowed"
        ));
    }

    Ok(())
}

pub fn validate_acip_review_fixture_set_v1(fixtures: &AcipReviewFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_REVIEW_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP review fixture set requires schema_version '{}'",
            ACIP_REVIEW_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!("ACIP review fixture set requires negative_cases"));
    }
    validate_acip_review_invocation_contract_v1(&fixtures.valid_contract)?;
    validate_acip_review_outcome_v1(&fixtures.valid_contract, &fixtures.valid_blessed_outcome)?;
    validate_acip_review_outcome_v1(&fixtures.valid_contract, &fixtures.valid_blocked_outcome)?;
    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        let contract_result = validate_acip_review_invocation_contract_v1_value(&case.contract);
        match &case.outcome {
            Some(outcome_value) => match contract_result {
                Ok(contract) => validate_negative_result(
                    validate_acip_review_outcome_v1_value(&contract, outcome_value).map(|_| ()),
                    &case.expected_error_substring,
                )?,
                Err(error) => validate_negative_result(Err(error), &case.expected_error_substring)?,
            },
            None => validate_negative_result(
                contract_result.map(|_| ()),
                &case.expected_error_substring,
            )?,
        }
    }
    Ok(())
}

pub fn validate_acip_coding_invocation_contract_v1(
    contract: &AcipCodingInvocationContractV1,
) -> Result<()> {
    if contract.schema_version != ACIP_CODING_INVOCATION_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP coding invocation contract requires schema_version '{}'",
            ACIP_CODING_INVOCATION_SCHEMA_VERSION
        ));
    }
    validate_acip_invocation_contract_v1(&contract.invocation)?;
    if contract.invocation.intent != AcipIntentV1::CodingRequest {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires invocation.intent 'coding_request'"
        ));
    }
    validate_repo_relative_ref(&contract.issue_ref, "issue_ref")?;
    validate_repo_relative_ref(&contract.task_bundle_ref, "task_bundle_ref")?;
    if contract.allowed_edit_paths.is_empty() {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires allowed_edit_paths"
        ));
    }
    for path in &contract.allowed_edit_paths {
        validate_repo_relative_ref(path, "allowed_edit_paths[]")?;
    }
    if contract.validation_commands.is_empty() {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires validation_commands"
        ));
    }
    for command in &contract.validation_commands {
        validate_non_empty(command, "validation_commands[]")?;
    }
    validate_non_empty(&contract.patch_format, "patch_format")?;
    if !contract
        .invocation
        .input_refs
        .iter()
        .any(|reference| reference == &contract.task_bundle_ref)
    {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires invocation.input_refs to include task_bundle_ref"
        ));
    }
    if !contract
        .invocation
        .constraints
        .required_capabilities
        .iter()
        .any(|capability| capability == "share_artifact")
    {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires constraints.required_capabilities to include 'share_artifact'"
        ));
    }
    for action in ["merge", "push", "self_review"] {
        if !contract
            .invocation
            .constraints
            .prohibited_actions
            .iter()
            .any(|prohibited| prohibited == action)
        {
            return Err(anyhow!(
                "ACIP coding invocation specialization requires constraints.prohibited_actions to include '{}'",
                action
            ));
        }
    }
    if contract.invocation.authority_scope.delegation_permitted {
        return Err(anyhow!(
            "ACIP coding invocation specialization must not permit delegation inside the authority_scope"
        ));
    }
    if !contract.invocation.stop_policy.stop_on_refusal {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires stop_policy.stop_on_refusal to be true"
        ));
    }
    if !contract.invocation.stop_policy.stop_on_failure {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires stop_policy.stop_on_failure to be true"
        ));
    }
    if !contract.approval_policy.review_required_before_pr_finish {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires review before PR finish"
        ));
    }
    if contract.approval_policy.required_review_schema_ref != ACIP_REVIEW_INVOCATION_SCHEMA_VERSION
    {
        return Err(anyhow!(
            "ACIP coding invocation specialization must route to review schema '{}'",
            ACIP_REVIEW_INVOCATION_SCHEMA_VERSION
        ));
    }
    validate_id(
        &contract.approval_policy.writer_session_id,
        "approval_policy.writer_session_id",
    )?;
    validate_id(
        &contract.approval_policy.writer_model_ref,
        "approval_policy.writer_model_ref",
    )?;
    if !contract.approval_policy.forbid_same_session_blessing {
        return Err(anyhow!(
            "ACIP coding invocation specialization must forbid same-session blessing"
        ));
    }
    if !contract.approval_policy.forbid_same_model_blessing {
        return Err(anyhow!(
            "ACIP coding invocation specialization must forbid same-model blessing"
        ));
    }

    match contract.provider_lane {
        AcipCodingProviderLaneV1::CodexIssueWorktree => {
            if !contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "code_edit")
            {
                return Err(anyhow!(
                    "codex_issue_worktree lane requires constraints.required_capabilities to include 'code_edit'"
                ));
            }
            if contract.execution_mode != AcipCodingExecutionModeV1::WorktreeEdit {
                return Err(anyhow!(
                    "codex_issue_worktree lane requires execution_mode 'worktree_edit'"
                ));
            }
            if !contract.issue_worktree_required {
                return Err(anyhow!(
                    "codex_issue_worktree lane requires issue_worktree_required to be true"
                ));
            }
        }
        AcipCodingProviderLaneV1::ChatgptApi => {
            if contract.execution_mode == AcipCodingExecutionModeV1::WorktreeEdit {
                return Err(anyhow!(
                    "only codex_issue_worktree lane may use execution_mode 'worktree_edit'"
                ));
            }
            if contract.issue_worktree_required {
                return Err(anyhow!(
                    "proposal-only coding lanes must set issue_worktree_required to false"
                ));
            }
            if contract.execution_mode == AcipCodingExecutionModeV1::StructuredProposal {
                return Err(anyhow!(
                    "chatgpt_api lane does not permit execution_mode 'structured_proposal'"
                ));
            }
            if contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "code_edit")
            {
                return Err(anyhow!(
                    "proposal-only coding lanes must not claim 'code_edit' capability"
                ));
            }
            if !contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "propose_change")
            {
                return Err(anyhow!(
                    "proposal-only coding lanes require constraints.required_capabilities to include 'propose_change'"
                ));
            }
        }
        AcipCodingProviderLaneV1::ClaudeApi
        | AcipCodingProviderLaneV1::LocalOllama
        | AcipCodingProviderLaneV1::OtherProposalOnly => {
            if contract.execution_mode == AcipCodingExecutionModeV1::WorktreeEdit {
                return Err(anyhow!(
                    "only codex_issue_worktree lane may use execution_mode 'worktree_edit'"
                ));
            }
            if contract.issue_worktree_required {
                return Err(anyhow!(
                    "proposal-only coding lanes must set issue_worktree_required to false"
                ));
            }
            if contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "code_edit")
            {
                return Err(anyhow!(
                    "proposal-only coding lanes must not claim 'code_edit' capability"
                ));
            }
            if !contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "propose_change")
            {
                return Err(anyhow!(
                    "proposal-only coding lanes require constraints.required_capabilities to include 'propose_change'"
                ));
            }
        }
    }

    let required_output_kind = match contract.execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => {
            if contract.patch_format != "patch_manifest_v1" {
                return Err(anyhow!(
                    "worktree_edit coding invocation requires patch_format 'patch_manifest_v1'"
                ));
            }
            "patch_manifest"
        }
        AcipCodingExecutionModeV1::UnappliedPatch => {
            if contract.patch_format != "unified_diff" {
                return Err(anyhow!(
                    "unapplied_patch coding invocation requires patch_format 'unified_diff'"
                ));
            }
            "patch_diff"
        }
        AcipCodingExecutionModeV1::StructuredProposal => {
            if contract.patch_format != "structured_proposal_v1" {
                return Err(anyhow!(
                    "structured_proposal coding invocation requires patch_format 'structured_proposal_v1'"
                ));
            }
            "structured_proposal"
        }
    };

    if !contract
        .invocation
        .expected_outputs
        .iter()
        .any(|output| output.required && output.output_kind == required_output_kind)
    {
        return Err(anyhow!(
            "coding invocation specialization requires expected_outputs to declare required {} output",
            required_output_kind
        ));
    }
    for output_kind in ["validation_summary", "review_handoff"] {
        if !contract
            .invocation
            .expected_outputs
            .iter()
            .any(|output| output.required && output.output_kind == output_kind)
        {
            return Err(anyhow!(
                "coding invocation specialization requires expected_outputs to declare required {} output",
                output_kind
            ));
        }
    }

    Ok(())
}

pub fn validate_acip_coding_outcome_v1(
    contract: &AcipCodingInvocationContractV1,
    outcome: &AcipCodingOutcomeV1,
) -> Result<()> {
    validate_acip_coding_invocation_contract_v1(contract)?;
    if outcome.schema_version != ACIP_CODING_OUTCOME_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP coding outcome requires schema_version '{}'",
            ACIP_CODING_OUTCOME_SCHEMA_VERSION
        ));
    }
    validate_id(&outcome.invocation_id, "invocation_id")?;
    validate_repo_relative_ref(&outcome.primary_output_ref, "primary_output_ref")?;
    if outcome.validation_result_refs.is_empty() {
        return Err(anyhow!(
            "ACIP coding outcome requires validation_result_refs"
        ));
    }
    for reference in &outcome.validation_result_refs {
        validate_repo_relative_ref(reference, "validation_result_refs[]")?;
    }
    if !outcome
        .validation_result_refs
        .iter()
        .any(|reference| reference.ends_with("validation_summary.json"))
    {
        return Err(anyhow!(
            "coding outcome validation_result_refs must include the declared validation_summary output contract"
        ));
    }
    validate_repo_relative_ref(&outcome.review_handoff_ref, "review_handoff_ref")?;
    validate_id(&outcome.writer_session_id, "writer_session_id")?;
    validate_id(&outcome.writer_model_ref, "writer_model_ref")?;

    if contract.invocation.invocation_id != outcome.invocation_id {
        return Err(anyhow!(
            "coding outcome must match the invocation contract invocation_id"
        ));
    }
    if contract.provider_lane != outcome.provider_lane {
        return Err(anyhow!(
            "coding outcome must preserve the provider_lane declared by the invocation contract"
        ));
    }
    if contract.execution_mode != outcome.execution_mode {
        return Err(anyhow!(
            "coding outcome must preserve the execution_mode declared by the invocation contract"
        ));
    }
    if contract.approval_policy.writer_session_id != outcome.writer_session_id {
        return Err(anyhow!(
            "coding outcome must preserve the writer_session_id declared by the approval policy"
        ));
    }
    if contract.approval_policy.writer_model_ref != outcome.writer_model_ref {
        return Err(anyhow!(
            "coding outcome must preserve the writer_model_ref declared by the approval policy"
        ));
    }
    if !outcome.review_handoff_ref.ends_with("review_handoff.json") {
        return Err(anyhow!(
            "coding outcome review_handoff_ref must match the declared review_handoff output contract"
        ));
    }

    match contract.execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => {
            if outcome.disposition != AcipCodingDispositionV1::PatchReadyForReview {
                return Err(anyhow!(
                    "worktree_edit coding outcome must use disposition 'patch_ready_for_review'"
                ));
            }
            if !outcome.primary_output_ref.ends_with("patch_manifest.json") {
                return Err(anyhow!(
                    "coding outcome primary_output_ref must match the declared patch_manifest output contract"
                ));
            }
        }
        AcipCodingExecutionModeV1::UnappliedPatch => {
            if outcome.disposition != AcipCodingDispositionV1::ProposalReadyForReview {
                return Err(anyhow!(
                    "unapplied_patch coding outcome must use disposition 'proposal_ready_for_review'"
                ));
            }
            if !outcome.primary_output_ref.ends_with(".diff") {
                return Err(anyhow!(
                    "coding outcome primary_output_ref must match the declared patch_diff output contract"
                ));
            }
        }
        AcipCodingExecutionModeV1::StructuredProposal => {
            if outcome.disposition != AcipCodingDispositionV1::ProposalReadyForReview {
                return Err(anyhow!(
                    "structured_proposal coding outcome must use disposition 'proposal_ready_for_review'"
                ));
            }
            if !outcome.primary_output_ref.ends_with("proposal.json") {
                return Err(anyhow!(
                    "coding outcome primary_output_ref must match the declared structured_proposal output contract"
                ));
            }
        }
    }

    Ok(())
}

pub fn validate_acip_coding_fixture_set_v1(fixtures: &AcipCodingFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_CODING_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP coding fixture set requires schema_version '{}'",
            ACIP_CODING_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!("ACIP coding fixture set requires negative_cases"));
    }
    validate_acip_coding_invocation_contract_v1(&fixtures.valid_codex_contract)?;
    validate_acip_coding_outcome_v1(
        &fixtures.valid_codex_contract,
        &fixtures.valid_codex_outcome,
    )?;
    validate_acip_coding_invocation_contract_v1(&fixtures.valid_patch_contract)?;
    validate_acip_coding_outcome_v1(
        &fixtures.valid_patch_contract,
        &fixtures.valid_patch_outcome,
    )?;
    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        let contract_result = validate_acip_coding_invocation_contract_v1_value(&case.contract);
        match &case.outcome {
            Some(outcome_value) => match contract_result {
                Ok(contract) => validate_negative_result(
                    validate_acip_coding_outcome_v1_value(&contract, outcome_value).map(|_| ()),
                    &case.expected_error_substring,
                )?,
                Err(error) => validate_negative_result(Err(error), &case.expected_error_substring)?,
            },
            None => validate_negative_result(
                contract_result.map(|_| ()),
                &case.expected_error_substring,
            )?,
        }
    }
    Ok(())
}

pub fn acip_fixture_set_v1() -> AcipFixtureSetV1 {
    let consultation = sample_message(
        "msg-consultation-0001",
        "conv-consult-001",
        1,
        AcipIntentV1::Consultation,
        "Can you compare these two planning options and call out the stronger evidence chain?",
    );
    let invocation_setup = sample_message_with_payloads(
        "msg-invoke-0001",
        "conv-invoke-001",
        1,
        AcipIntentV1::InvocationSetup,
        "Please prepare the bounded invocation contract and preserve policy hooks.",
        vec![payload_ref(
            "structured_prompt",
            "runtime/comms/contracts/invocation_setup.stp.json",
            "application/json",
            1_812,
            Some("Bounded invocation contract with explicit stop conditions."),
        )],
    );
    let review_request = sample_message_with_payloads(
        "msg-review-0001",
        "conv-review-001",
        1,
        AcipIntentV1::ReviewRequest,
        "Please review the attached packet for correctness, completeness, and policy drift.",
        vec![payload_ref(
            "review_packet",
            "runtime/comms/review/review_packet.json",
            "application/json",
            2_104,
            Some("Review packet plus SRP references."),
        )],
    );
    let coding_request = sample_message_with_payloads(
        "msg-coding-0001",
        "conv-coding-001",
        1,
        AcipIntentV1::CodingRequest,
        "Please implement the bounded change without widening the issue scope.",
        vec![payload_ref(
            "task_bundle",
            "runtime/comms/coding/task_bundle.json",
            "application/json",
            1_944,
            Some("Bounded coding task bundle with acceptance criteria."),
        )],
    );
    let delegation = sample_message_with_payloads(
        "msg-delegate-0001",
        "conv-delegate-001",
        1,
        AcipIntentV1::Delegation,
        "I am delegating a bounded artifact-generation task and preserving parent accountability.",
        vec![payload_ref(
            "delegation_contract",
            "runtime/comms/delegation/delegation_contract.json",
            "application/json",
            1_688,
            Some("Delegation contract with explicit review return path."),
        )],
    );
    let negotiation = sample_message(
        "msg-negotiate-0001",
        "conv-negotiate-001",
        1,
        AcipIntentV1::Negotiation,
        "I can take the trace task if you cover conformance fixtures and we keep one shared authority basis.",
    );
    let mut coding_handoff = sample_message_with_payloads(
        "msg-handoff-0001",
        "conv-handoff-001",
        1,
        AcipIntentV1::CodingRequest,
        "Handing this bounded coding task to the implementation lane with explicit artifact return expectations.",
        vec![payload_ref(
            "task_bundle",
            "runtime/comms/coding/handoff_task_bundle.json",
            "application/json",
            1_732,
            Some("Coding handoff bundle plus expected patch outputs."),
        )],
    );
    coding_handoff.recipient.id = "coder.agent".to_string();
    coding_handoff.correlation_id = Some("handoff-0001".to_string());

    let mut operator_request = sample_message_with_payloads(
        "msg-operator-0001",
        "conv-operator-001",
        1,
        AcipIntentV1::Consultation,
        "Operators, please review the bounded demo packet and confirm it is safe to publish internally.",
        vec![payload_ref(
            "demo_packet",
            "runtime/comms/operator/demo_packet.json",
            "application/json",
            1_204,
            Some("Demo packet for operator review without governed invocation."),
        )],
    );
    operator_request.recipient = AcipAddressV1 {
        kind: AcipAddressKindV1::Group,
        id: "operators".to_string(),
    };
    operator_request.visibility = AcipVisibilityV1::Shared;
    operator_request.trace_requirement = AcipTraceRequirementV1::Full;

    let mut broadcast = sample_message(
        "msg-broadcast-0001",
        "conv-broadcast-001",
        1,
        AcipIntentV1::Conversation,
        "Broadcast: the conformance suite is green and no governed action is requested in this message.",
    );
    broadcast.recipient = AcipAddressV1 {
        kind: AcipAddressKindV1::Group,
        id: "all_agents".to_string(),
    };
    broadcast.visibility = AcipVisibilityV1::Public;
    broadcast.trace_requirement = AcipTraceRequirementV1::Summary;
    broadcast.authority_scope = None;

    let conversation = AcipConversationEnvelopeV1 {
        schema_version: ACIP_CONVERSATION_SCHEMA_VERSION.to_string(),
        messages: vec![
            sample_message(
                "msg-conversation-0001",
                "conv-shared-001",
                1,
                AcipIntentV1::Conversation,
                "Hi, I have code that needs review. Do you have time?",
            ),
            sample_message(
                "msg-conversation-0002",
                "conv-shared-001",
                2,
                AcipIntentV1::Consultation,
                "Yes. Share the bounded packet and I will keep the review surface narrow.",
            ),
            sample_message_with_payloads(
                "msg-conversation-0003",
                "conv-shared-001",
                3,
                AcipIntentV1::ReviewRequest,
                "Here is the packet and the trace anchor.",
                vec![payload_ref(
                    "review_packet",
                    "runtime/comms/review/packet.json",
                    "application/json",
                    1_096,
                    Some("Packet plus trace anchor."),
                )],
            ),
        ],
    };

    AcipFixtureSetV1 {
        schema_version: ACIP_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_messages: vec![
            AcipNamedMessageFixtureV1 {
                name: "conversation".to_string(),
                message: sample_message(
                    "msg-conversation-standalone-0001",
                    "conv-standalone-001",
                    1,
                    AcipIntentV1::Conversation,
                    "Can we talk through the transition plan before we bind an invocation?",
                ),
            },
            AcipNamedMessageFixtureV1 {
                name: "consultation".to_string(),
                message: consultation,
            },
            AcipNamedMessageFixtureV1 {
                name: "invocation_setup".to_string(),
                message: invocation_setup,
            },
            AcipNamedMessageFixtureV1 {
                name: "review_request".to_string(),
                message: review_request,
            },
            AcipNamedMessageFixtureV1 {
                name: "coding_request".to_string(),
                message: coding_request,
            },
            AcipNamedMessageFixtureV1 {
                name: "delegation".to_string(),
                message: delegation,
            },
            AcipNamedMessageFixtureV1 {
                name: "negotiation".to_string(),
                message: negotiation,
            },
            AcipNamedMessageFixtureV1 {
                name: "coding_agent_handoff".to_string(),
                message: coding_handoff,
            },
            AcipNamedMessageFixtureV1 {
                name: "operator_request".to_string(),
                message: operator_request,
            },
            AcipNamedMessageFixtureV1 {
                name: "broadcast".to_string(),
                message: broadcast,
            },
        ],
        valid_conversation: conversation,
        invalid_messages: vec![
            AcipNegativeMessageCaseV1 {
                name: "identity_drift".to_string(),
                expected_error_substring: "sender and recipient must not be identical".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0001",
                    "conversation_id": "conv-bad-001",
                    "timestamp_utc": "2026-04-28T19:00:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "planner.agent"},
                    "intent": "consultation",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "Please help.",
                    "payload_refs": [],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": null,
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "missing_recipient".to_string(),
                expected_error_substring: "recipient.id must not be empty".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0002",
                    "conversation_id": "conv-bad-002",
                    "timestamp_utc": "2026-04-28T19:01:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": ""},
                    "intent": "conversation",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "Missing recipient should fail closed.",
                    "payload_refs": [],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": null,
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "hidden_invocation".to_string(),
                expected_error_substring:
                    "message intent 'conversation' must not claim authority action 'invoke'"
                        .to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0003",
                    "conversation_id": "conv-bad-003",
                    "timestamp_utc": "2026-04-28T19:02:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "coder.agent"},
                    "intent": "conversation",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "This looks conversational but is really trying to sneak governed work through.",
                    "payload_refs": [{
                        "payload_kind": "task_bundle",
                        "payload_ref": "runtime/comms/coding/task_bundle.json",
                        "media_type": "application/json",
                        "content_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "byte_length": 612,
                        "inline_summary": "Hidden invocation attempt."
                    }],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["invoke", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/invocation_basis.json"],
                        "delegation_permitted": false
                    },
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "malformed_payload_refs".to_string(),
                expected_error_substring:
                    "content_sha256 must be a 64-character hexadecimal sha256".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0004",
                    "conversation_id": "conv-bad-004",
                    "timestamp_utc": "2026-04-28T19:03:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "visibility": "shared",
                    "trace_requirement": "summary",
                    "content": "Please review this packet.",
                    "payload_refs": [{
                        "payload_kind": "review_packet",
                        "payload_ref": "runtime/comms/review/review_packet.json",
                        "media_type": "application/json",
                        "content_sha256": "not-a-sha",
                        "byte_length": 512,
                        "inline_summary": "Malformed payload hash."
                    }],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["review", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/review_basis.json"],
                        "delegation_permitted": false
                    },
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "unsupported_visibility".to_string(),
                expected_error_substring: "unknown variant".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0005",
                    "conversation_id": "conv-bad-005",
                    "timestamp_utc": "2026-04-28T19:04:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "group", "id": "operators"},
                    "intent": "conversation",
                    "visibility": "operator_only",
                    "trace_requirement": "none",
                    "content": "This should fail.",
                    "payload_refs": [],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": null,
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "raw_local_path_refs".to_string(),
                expected_error_substring: "artifact_refs[] must be a repository-relative path"
                    .to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0006",
                    "conversation_id": "conv-bad-006",
                    "timestamp_utc": "2026-04-28T19:05:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "coder.agent"},
                    "intent": "coding_request",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "Please implement the change.",
                    "payload_refs": [],
                    "artifact_refs": ["/Users/daniel/private/runtime_patch.diff"],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["invoke", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/invocation_basis.json"],
                        "delegation_permitted": false
                    },
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "authority_escalation".to_string(),
                expected_error_substring:
                    "message intent 'consultation' must not claim authority action 'delegate'"
                        .to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0007",
                    "conversation_id": "conv-bad-007",
                    "timestamp_utc": "2026-04-28T19:06:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "delegate.agent"},
                    "intent": "consultation",
                    "visibility": "shared",
                    "trace_requirement": "full",
                    "content": "This tries to smuggle delegated authority through a consultation surface.",
                    "payload_refs": [],
                    "artifact_refs": ["runtime/comms/delegation/contract.json"],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["delegate", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/delegation/basis.json"],
                        "delegation_permitted": true
                    },
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
        ],
        invalid_conversations: vec![AcipNegativeConversationCaseV1 {
            name: "stale_ordering".to_string(),
            expected_error_substring:
                "conversation messages must preserve strictly increasing monotonic_order"
                    .to_string(),
            value: json!({
                "schema_version": ACIP_CONVERSATION_SCHEMA_VERSION,
                "messages": [
                    sample_message("msg-order-0001", "conv-order-001", 2, AcipIntentV1::Conversation, "Late."),
                    sample_message("msg-order-0002", "conv-order-001", 1, AcipIntentV1::Conversation, "Early.")
                ]
            }),
        }],
    }
}

pub fn acip_invocation_fixture_set_v1() -> AcipInvocationFixtureSetV1 {
    let valid_contract = sample_invocation_contract();

    AcipInvocationFixtureSetV1 {
        schema_version: ACIP_INVOCATION_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_contract: valid_contract.clone(),
        valid_completed_event: sample_invocation_event(
            &valid_contract,
            AcipInvocationStatusV1::Completed,
            vec![
                "runtime/comms/invocation/review_report.json".to_string(),
                "runtime/comms/invocation/review_summary.json".to_string(),
            ],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        ),
        valid_refused_event: sample_invocation_event(
            &valid_contract,
            AcipInvocationStatusV1::Refused,
            Vec::new(),
            "policy_refusal".to_string(),
            Some("operator_review_required".to_string()),
            None,
            vec!["runtime/comms/invocation/evidence/refusal_note.json".to_string()],
        ),
        valid_failed_event: sample_invocation_event(
            &valid_contract,
            AcipInvocationStatusV1::Failed,
            Vec::new(),
            "output_validation_failed".to_string(),
            None,
            Some("output_contract_mismatch".to_string()),
            vec!["runtime/comms/invocation/evidence/failure_packet.json".to_string()],
        ),
        negative_cases: vec![
            AcipInvocationNegativeCaseV1 {
                name: "missing_gate_rejects_governed_invocation".to_string(),
                expected_error_substring: "missing field `decision_event_ref`".to_string(),
                contract: json!({
                    "schema_version": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                    "invocation_id": "invoke-0002",
                    "conversation_id": "conv-invoke-001",
                    "causal_message_id": "msg-review-0001",
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "purpose": "Perform bounded review work.",
                    "input_refs": ["runtime/comms/invocation/review_packet.json"],
                    "constraints": {
                        "policy_refs": ["runtime/comms/policy/review_policy.json"],
                        "required_capabilities": ["review"],
                        "prohibited_actions": ["merge"],
                        "requires_redaction": true
                    },
                    "expected_outputs": [{
                        "output_id": "review_report",
                        "output_kind": "review_report",
                        "artifact_role": "primary_report",
                        "schema_ref": "schemas/review_report.schema.json",
                        "required": true
                    }],
                    "stop_policy": {
                        "max_turns": 1,
                        "max_output_artifacts": 2,
                        "completion_condition": "emit refusal or report",
                        "stop_on_refusal": true,
                        "stop_on_failure": true
                    },
                    "authority_scope": {
                        "allowed_actions": ["review", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/review_basis.json"],
                        "delegation_permitted": false
                    },
                    "response_channel": {
                        "kind": "direct_reply",
                        "channel_ref": "reply.review.thread"
                    },
                    "trace_requirement": "full"
                }),
                event: None,
            },
            AcipInvocationNegativeCaseV1 {
                name: "ambiguous_stop_policy_rejected".to_string(),
                expected_error_substring: "stop_policy.max_turns must be positive".to_string(),
                contract: json!({
                    "schema_version": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                    "invocation_id": "invoke-0003",
                    "conversation_id": "conv-invoke-001",
                    "causal_message_id": "msg-review-0001",
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "purpose": "Perform bounded review work.",
                    "input_refs": ["runtime/comms/invocation/review_packet.json"],
                    "constraints": {
                        "policy_refs": ["runtime/comms/policy/review_policy.json"],
                        "required_capabilities": ["review"],
                        "prohibited_actions": ["merge"],
                        "requires_redaction": true
                    },
                    "expected_outputs": [{
                        "output_id": "review_report",
                        "output_kind": "review_report",
                        "artifact_role": "primary_report",
                        "schema_ref": "schemas/review_report.schema.json",
                        "required": true
                    }],
                    "stop_policy": {
                        "max_turns": 0,
                        "max_output_artifacts": 2,
                        "completion_condition": "emit refusal or report",
                        "stop_on_refusal": true,
                        "stop_on_failure": true
                    },
                    "authority_scope": {
                        "allowed_actions": ["review", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/review_basis.json"],
                        "delegation_permitted": false
                    },
                    "decision_event_ref": "gate.review-0001",
                    "response_channel": {
                        "kind": "direct_reply",
                        "channel_ref": "reply.review.thread"
                    },
                    "trace_requirement": "full"
                }),
                event: None,
            },
            AcipInvocationNegativeCaseV1 {
                name: "unsafe_input_refs_rejected".to_string(),
                expected_error_substring: "input_refs[] must be a repository-relative path"
                    .to_string(),
                contract: json!({
                    "schema_version": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                    "invocation_id": "invoke-0004",
                    "conversation_id": "conv-invoke-001",
                    "causal_message_id": "msg-review-0001",
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "purpose": "Perform bounded review work.",
                    "input_refs": ["/Users/daniel/private/review_packet.json"],
                    "constraints": {
                        "policy_refs": ["runtime/comms/policy/review_policy.json"],
                        "required_capabilities": ["review"],
                        "prohibited_actions": ["merge"],
                        "requires_redaction": true
                    },
                    "expected_outputs": [{
                        "output_id": "review_report",
                        "output_kind": "review_report",
                        "artifact_role": "primary_report",
                        "schema_ref": "schemas/review_report.schema.json",
                        "required": true
                    }],
                    "stop_policy": {
                        "max_turns": 1,
                        "max_output_artifacts": 2,
                        "completion_condition": "emit refusal or report",
                        "stop_on_refusal": true,
                        "stop_on_failure": true
                    },
                    "authority_scope": {
                        "allowed_actions": ["review", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/review_basis.json"],
                        "delegation_permitted": false
                    },
                    "decision_event_ref": "gate.review-0001",
                    "response_channel": {
                        "kind": "direct_reply",
                        "channel_ref": "reply.review.thread"
                    },
                    "trace_requirement": "full"
                }),
                event: None,
            },
            AcipInvocationNegativeCaseV1 {
                name: "status_refusal_inconsistency_rejected".to_string(),
                expected_error_substring:
                    "completed invocation event must not carry refusal_code or failure_code"
                        .to_string(),
                contract: serde_json::to_value(valid_contract.clone()).expect("contract json"),
                event: Some(json!({
                    "schema_version": ACIP_INVOCATION_EVENT_SCHEMA_VERSION,
                    "invocation_id": valid_contract.invocation_id,
                    "conversation_id": valid_contract.conversation_id,
                    "causal_message_id": valid_contract.causal_message_id,
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "contract_ref": "runtime/comms/invocation/contracts/review_request.json",
                    "contract_sha256": "89abcdef0123456789abcdef0123456789abcdef0123456789abcdef01234567",
                    "decision_event_ref": valid_contract.decision_event_ref,
                    "input_refs": valid_contract.input_refs,
                    "output_refs": ["runtime/comms/invocation/review_report.json"],
                    "status": "completed",
                    "stop_reason": "completed_output_contract",
                    "refusal_code": "should-not-be-here",
                    "failure_code": null,
                    "evidence_refs": ["runtime/comms/invocation/evidence/review_trace.json"],
                    "trace_requirement": "full"
                })),
            },
            AcipInvocationNegativeCaseV1 {
                name: "output_contract_mismatch_rejected".to_string(),
                expected_error_substring:
                    "completed invocation must satisfy declared required output contracts"
                        .to_string(),
                contract: serde_json::to_value(valid_contract.clone()).expect("contract json"),
                event: Some(json!({
                    "schema_version": ACIP_INVOCATION_EVENT_SCHEMA_VERSION,
                    "invocation_id": valid_contract.invocation_id,
                    "conversation_id": valid_contract.conversation_id,
                    "causal_message_id": valid_contract.causal_message_id,
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "contract_ref": "runtime/comms/invocation/contracts/review_request.json",
                    "contract_sha256": "89abcdef0123456789abcdef0123456789abcdef0123456789abcdef01234567",
                    "decision_event_ref": valid_contract.decision_event_ref,
                    "input_refs": valid_contract.input_refs,
                    "output_refs": ["runtime/comms/invocation/operator_summary.json"],
                    "status": "completed",
                    "stop_reason": "completed_output_contract",
                    "refusal_code": null,
                    "failure_code": null,
                    "evidence_refs": ["runtime/comms/invocation/evidence/review_trace.json"],
                    "trace_requirement": "full"
                })),
            },
        ],
    }
}

pub fn acip_conformance_report_v1() -> AcipConformanceReportV1 {
    let fixture_set = acip_fixture_set_v1();
    let invocation_fixture_set = acip_invocation_fixture_set_v1();
    let feature_doc_ref = "docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md".to_string();

    let mut valid_fixture_classes = fixture_set
        .valid_messages
        .iter()
        .map(|fixture| {
            let (surface, mode_label, proves) = match fixture.name.as_str() {
                "conversation" => (
                    AcipConformanceSurfaceV1::Message,
                    "conversation",
                    "ACIP supports bounded non-governed conversation without hidden invocation authority.",
                ),
                "consultation" => (
                    AcipConformanceSurfaceV1::Message,
                    "consultation",
                    "ACIP supports advisory consultation with explicit identity and share-only authority.",
                ),
                "invocation_setup" => (
                    AcipConformanceSurfaceV1::Message,
                    "invocation_setup",
                    "ACIP can stage governed invocation setup as a first-class mode before contract execution.",
                ),
                "review_request" => (
                    AcipConformanceSurfaceV1::Message,
                    "review_request",
                    "ACIP supports reviewer-facing governed requests without collapsing into generic chat.",
                ),
                "coding_request" => (
                    AcipConformanceSurfaceV1::Message,
                    "coding_request",
                    "ACIP supports coding-agent requests with bounded task-bundle payloads.",
                ),
                "coding_agent_handoff" => (
                    AcipConformanceSurfaceV1::Message,
                    "handoff",
                    "ACIP supports explicit agent-to-agent coding handoff without redefining the core transport.",
                ),
                "delegation" => (
                    AcipConformanceSurfaceV1::Message,
                    "delegation",
                    "ACIP supports explicit delegation requests with parent-accountability semantics.",
                ),
                "negotiation" => (
                    AcipConformanceSurfaceV1::Message,
                    "negotiation",
                    "ACIP supports negotiation as a first-class communication mode.",
                ),
                "operator_request" => (
                    AcipConformanceSurfaceV1::Message,
                    "operator_request",
                    "ACIP supports operator-group requests without requiring live-provider orchestration.",
                ),
                "broadcast" => (
                    AcipConformanceSurfaceV1::Message,
                    "broadcast",
                    "ACIP supports group broadcast without smuggling governed authority.",
                ),
                other => (
                    AcipConformanceSurfaceV1::Message,
                    other,
                    "ACIP preserves a deterministic valid message fixture.",
                ),
            };
            AcipConformanceFixtureClassV1 {
                fixture_name: fixture.name.clone(),
                surface,
                mode_label: mode_label.to_string(),
                proves: proves.to_string(),
                feature_doc_ref: feature_doc_ref.clone(),
            }
        })
        .collect::<Vec<_>>();

    valid_fixture_classes.push(AcipConformanceFixtureClassV1 {
        fixture_name: "shared_conversation_thread".to_string(),
        surface: AcipConformanceSurfaceV1::Conversation,
        mode_label: "conversation_thread".to_string(),
        proves: "ACIP preserves monotonic multi-message conversation sequencing across mixed conversation, consultation, and review-request turns.".to_string(),
        feature_doc_ref: feature_doc_ref.clone(),
    });
    valid_fixture_classes.push(AcipConformanceFixtureClassV1 {
        fixture_name: "governed_invocation_contract".to_string(),
        surface: AcipConformanceSurfaceV1::Invocation,
        mode_label: "invocation".to_string(),
        proves: "ACIP preserves governed invocation contract, refusal, failure, and completed-output semantics under explicit Freedom Gate linkage.".to_string(),
        feature_doc_ref: feature_doc_ref.clone(),
    });

    let mut negative_fixture_classes = fixture_set
        .invalid_messages
        .iter()
        .map(|case| AcipConformanceNegativeClassV1 {
            case_name: case.name.clone(),
            surface: AcipConformanceSurfaceV1::Message,
            proves: format!(
                "ACIP fails closed for '{}' without leaking host-local or authority-drift semantics.",
                case.name
            ),
            expected_error_substring: case.expected_error_substring.clone(),
            feature_doc_ref: feature_doc_ref.clone(),
        })
        .collect::<Vec<_>>();
    negative_fixture_classes.extend(fixture_set.invalid_conversations.iter().map(|case| {
        AcipConformanceNegativeClassV1 {
            case_name: case.name.clone(),
            surface: AcipConformanceSurfaceV1::Conversation,
            proves: format!(
                "ACIP conversation sequencing rejects '{}' deterministically.",
                case.name
            ),
            expected_error_substring: case.expected_error_substring.clone(),
            feature_doc_ref: feature_doc_ref.clone(),
        }
    }));
    negative_fixture_classes.extend(invocation_fixture_set.negative_cases.iter().map(|case| {
        AcipConformanceNegativeClassV1 {
            case_name: case.name.clone(),
            surface: AcipConformanceSurfaceV1::Invocation,
            proves: format!(
                "ACIP governed invocation rejects '{}' with a stable failure reason.",
                case.name
            ),
            expected_error_substring: case.expected_error_substring.clone(),
            feature_doc_ref: feature_doc_ref.clone(),
        }
    }));

    AcipConformanceReportV1 {
        schema_version: ACIP_CONFORMANCE_REPORT_SCHEMA_VERSION.to_string(),
        valid_fixture_classes,
        negative_fixture_classes,
    }
}

pub fn acip_review_fixture_set_v1() -> AcipReviewFixtureSetV1 {
    let valid_contract = sample_review_invocation_contract();
    AcipReviewFixtureSetV1 {
        schema_version: ACIP_REVIEW_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_blessed_outcome: sample_review_outcome(
            &valid_contract,
            AcipReviewDispositionV1::Blessed,
            AcipReviewHandoffOutcomeV1::AllowPrFinish,
            Some("runtime/comms/review/findings/clean_review.json".to_string()),
            vec!["runtime/comms/review/residual_risk/none.json".to_string()],
            true,
        ),
        valid_blocked_outcome: sample_review_outcome(
            &valid_contract,
            AcipReviewDispositionV1::Blocked,
            AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
            Some("runtime/comms/review/findings/blocking_findings.json".to_string()),
            vec!["runtime/comms/review/residual_risk/needs_fix.json".to_string()],
            false,
        ),
        negative_cases: vec![
            AcipReviewNegativeCaseV1 {
                name: "missing_srp_policy_link_rejected".to_string(),
                expected_error_substring: "requires constraints.policy_refs to include srp_ref"
                    .to_string(),
                contract: {
                    let mut value =
                        serde_json::to_value(sample_review_invocation_contract()).expect("json");
                    value["invocation"]["constraints"]["policy_refs"] =
                        json!(["runtime/comms/policy/review_policy.json"]);
                    value
                },
                outcome: None,
            },
            AcipReviewNegativeCaseV1 {
                name: "same_session_self_review_rejected".to_string(),
                expected_error_substring:
                    "review invocation independence policy forbids same-session review".to_string(),
                contract: {
                    let mut value =
                        serde_json::to_value(sample_review_invocation_contract()).expect("json");
                    value["independence_policy"]["reviewer_session_id"] =
                        json!("writer-session-codex");
                    value
                },
                outcome: None,
            },
            AcipReviewNegativeCaseV1 {
                name: "same_model_self_blessing_rejected".to_string(),
                expected_error_substring:
                    "review invocation independence policy forbids same-model review".to_string(),
                contract: {
                    let mut value =
                        serde_json::to_value(sample_review_invocation_contract()).expect("json");
                    value["independence_policy"]["reviewer_model_ref"] = json!("gpt-5-codex");
                    value
                },
                outcome: None,
            },
            AcipReviewNegativeCaseV1 {
                name: "merge_authority_gap_rejected".to_string(),
                expected_error_substring:
                    "requires constraints.prohibited_actions to include 'merge'".to_string(),
                contract: {
                    let mut value =
                        serde_json::to_value(sample_review_invocation_contract()).expect("json");
                    value["invocation"]["constraints"]["prohibited_actions"] =
                        json!(["push", "destructive_edit"]);
                    value
                },
                outcome: None,
            },
            AcipReviewNegativeCaseV1 {
                name: "blocked_outcome_requires_findings_ref".to_string(),
                expected_error_substring: "blocked review outcome must carry findings_ref"
                    .to_string(),
                contract: serde_json::to_value(sample_review_invocation_contract()).expect("json"),
                outcome: Some({
                    let mut value = serde_json::to_value(sample_review_outcome(
                        &valid_contract,
                        AcipReviewDispositionV1::Blocked,
                        AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
                        Some("runtime/comms/review/findings/blocking_findings.json".to_string()),
                        vec!["runtime/comms/review/residual_risk/needs_fix.json".to_string()],
                        false,
                    ))
                    .expect("json");
                    value["findings_ref"] = JsonValue::Null;
                    value
                }),
            },
            AcipReviewNegativeCaseV1 {
                name: "non_blessed_outcome_cannot_allow_pr_finish".to_string(),
                expected_error_substring: "only blessed review outcomes may allow PR finish"
                    .to_string(),
                contract: serde_json::to_value(sample_review_invocation_contract()).expect("json"),
                outcome: Some(
                    serde_json::to_value(sample_review_outcome(
                        &valid_contract,
                        AcipReviewDispositionV1::Blocked,
                        AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
                        Some("runtime/comms/review/findings/blocking_findings.json".to_string()),
                        vec!["runtime/comms/review/residual_risk/needs_fix.json".to_string()],
                        true,
                    ))
                    .expect("json"),
                ),
            },
        ],
        valid_contract,
    }
}

pub fn acip_coding_fixture_set_v1() -> AcipCodingFixtureSetV1 {
    let valid_codex_contract = sample_coding_invocation_contract(
        AcipCodingProviderLaneV1::CodexIssueWorktree,
        AcipCodingExecutionModeV1::WorktreeEdit,
    );
    let valid_patch_contract = sample_coding_invocation_contract(
        AcipCodingProviderLaneV1::ChatgptApi,
        AcipCodingExecutionModeV1::UnappliedPatch,
    );

    AcipCodingFixtureSetV1 {
        schema_version: ACIP_CODING_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_codex_outcome: sample_coding_outcome(&valid_codex_contract),
        valid_patch_outcome: sample_coding_outcome(&valid_patch_contract),
        negative_cases: vec![
            AcipCodingNegativeCaseV1 {
                name: "non_codex_worktree_edit_rejected".to_string(),
                expected_error_substring:
                    "only codex_issue_worktree lane may use execution_mode 'worktree_edit'"
                        .to_string(),
                contract: serde_json::to_value(sample_coding_invocation_contract(
                    AcipCodingProviderLaneV1::ChatgptApi,
                    AcipCodingExecutionModeV1::WorktreeEdit,
                ))
                .expect("json"),
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "proposal_lane_code_edit_capability_rejected".to_string(),
                expected_error_substring:
                    "proposal-only coding lanes must not claim 'code_edit' capability"
                        .to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::ChatgptApi,
                        AcipCodingExecutionModeV1::UnappliedPatch,
                    ))
                    .expect("json");
                    value["invocation"]["constraints"]["required_capabilities"] =
                        json!(["code_edit", "share_artifact"]);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "chatgpt_structured_proposal_rejected".to_string(),
                expected_error_substring:
                    "chatgpt_api lane does not permit execution_mode 'structured_proposal'"
                        .to_string(),
                contract: serde_json::to_value(sample_coding_invocation_contract(
                    AcipCodingProviderLaneV1::ChatgptApi,
                    AcipCodingExecutionModeV1::StructuredProposal,
                ))
                .expect("json"),
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "proposal_lane_worktree_flag_rejected".to_string(),
                expected_error_substring:
                    "proposal-only coding lanes must set issue_worktree_required to false"
                        .to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::ClaudeApi,
                        AcipCodingExecutionModeV1::StructuredProposal,
                    ))
                    .expect("json");
                    value["issue_worktree_required"] = json!(true);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "codex_requires_issue_worktree_rejected".to_string(),
                expected_error_substring:
                    "codex_issue_worktree lane requires issue_worktree_required to be true"
                        .to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["issue_worktree_required"] = json!(false);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "missing_task_bundle_input_rejected".to_string(),
                expected_error_substring:
                    "requires invocation.input_refs to include task_bundle_ref".to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["invocation"]["input_refs"] = json!([
                        "runtime/comms/coding/issue_context.md",
                        "runtime/comms/coding/current_scope.json"
                    ]);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "missing_merge_prohibition_rejected".to_string(),
                expected_error_substring:
                    "requires constraints.prohibited_actions to include 'merge'".to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["invocation"]["constraints"]["prohibited_actions"] =
                        json!(["push", "self_review"]);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "review_schema_drift_rejected".to_string(),
                expected_error_substring: "must route to review schema 'acip.review.invocation.v1'"
                    .to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["approval_policy"]["required_review_schema_ref"] =
                        json!("acip.review.invocation.v0");
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "outcome_output_contract_mismatch_rejected".to_string(),
                expected_error_substring: "must match the declared patch_diff output contract"
                    .to_string(),
                contract: serde_json::to_value(valid_patch_contract.clone()).expect("json"),
                outcome: Some({
                    let mut value =
                        serde_json::to_value(sample_coding_outcome(&valid_patch_contract))
                            .expect("json");
                    value["primary_output_ref"] = json!("runtime/comms/coding/proposal.json");
                    value
                }),
            },
            AcipCodingNegativeCaseV1 {
                name: "validation_summary_binding_rejected".to_string(),
                expected_error_substring:
                    "validation_result_refs must include the declared validation_summary output contract"
                        .to_string(),
                contract: serde_json::to_value(valid_codex_contract.clone()).expect("json"),
                outcome: Some({
                    let mut value =
                        serde_json::to_value(sample_coding_outcome(&valid_codex_contract))
                            .expect("json");
                    value["validation_result_refs"] =
                        json!(["runtime/comms/coding/some_other_artifact.json"]);
                    value
                }),
            },
            AcipCodingNegativeCaseV1 {
                name: "writer_identity_drift_rejected".to_string(),
                expected_error_substring:
                    "must preserve the writer_session_id declared by the approval policy"
                        .to_string(),
                contract: serde_json::to_value(valid_codex_contract.clone()).expect("json"),
                outcome: Some({
                    let mut value =
                        serde_json::to_value(sample_coding_outcome(&valid_codex_contract))
                            .expect("json");
                    value["writer_session_id"] = json!("someone-else");
                    value
                }),
            },
            AcipCodingNegativeCaseV1 {
                name: "stop_boundary_drift_rejected".to_string(),
                expected_error_substring:
                    "requires stop_policy.stop_on_refusal to be true".to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["invocation"]["stop_policy"]["stop_on_refusal"] = json!(false);
                    value
                },
                outcome: None,
            },
        ],
        valid_codex_contract,
        valid_patch_contract,
    }
}

pub fn acip_trace_fixture_set_v1() -> AcipTraceFixtureSetV1 {
    AcipTraceFixtureSetV1 {
        schema_version: ACIP_TRACE_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_completed_bundle: sample_trace_bundle(AcipInvocationStatusV1::Completed),
        valid_refused_bundle: sample_trace_bundle(AcipInvocationStatusV1::Refused),
        valid_failed_bundle: sample_trace_bundle(AcipInvocationStatusV1::Failed),
        negative_cases: vec![
            AcipTraceNegativeCaseV1 {
                name: "public_view_private_state_leak_rejected".to_string(),
                expected_error_substring:
                    "reviewer, public, and observatory views must not allow private payload refs"
                        .to_string(),
                bundle: {
                    let mut value = serde_json::to_value(sample_trace_bundle(
                        AcipInvocationStatusV1::Completed,
                    ))
                    .expect("json");
                    value["audience_views"][3]["allows_private_payload_refs"] = json!(true);
                    value
                },
            },
            AcipTraceNegativeCaseV1 {
                name: "missing_decision_event_rejected".to_string(),
                expected_error_substring:
                    "decision_recorded trace event must carry invocation_id, contract_ref, and decision_event_ref"
                        .to_string(),
                bundle: {
                    let mut value = serde_json::to_value(sample_trace_bundle(
                        AcipInvocationStatusV1::Completed,
                    ))
                    .expect("json");
                    value["trace_events"][2]["decision_event_ref"] = JsonValue::Null;
                    value
                },
            },
            AcipTraceNegativeCaseV1 {
                name: "terminal_event_must_require_redaction".to_string(),
                expected_error_substring: "invocation_refused trace event must require redaction"
                    .to_string(),
                bundle: {
                    let mut value =
                        serde_json::to_value(sample_trace_bundle(AcipInvocationStatusV1::Refused))
                            .expect("json");
                    value["trace_events"][3]["requires_redaction"] = json!(false);
                    value
                },
            },
            AcipTraceNegativeCaseV1 {
                name: "host_path_leakage_in_summary_rejected".to_string(),
                expected_error_substring: "summary must not leak protected trace content '/users/'"
                    .to_string(),
                bundle: {
                    let mut value = serde_json::to_value(sample_trace_bundle(
                        AcipInvocationStatusV1::Completed,
                    ))
                    .expect("json");
                    value["trace_events"][0]["summary"] =
                        json!("Captured local path /Users/daniel/private/trace.json for replay.");
                    value
                },
            },
            AcipTraceNegativeCaseV1 {
                name: "remote_replay_dependency_rejected".to_string(),
                expected_error_substring:
                    "ACIP replay contract must remain fixture-backed and local for v1".to_string(),
                bundle: {
                    let mut value = serde_json::to_value(sample_trace_bundle(
                        AcipInvocationStatusV1::Completed,
                    ))
                    .expect("json");
                    value["replay_contract"]["remote_provider_required"] = json!(true);
                    value
                },
            },
        ],
    }
}

fn sample_message(
    message_id: &str,
    conversation_id: &str,
    monotonic_order: u64,
    intent: AcipIntentV1,
    content: &str,
) -> AcipMessageEnvelopeV1 {
    let authority_scope = default_message_authority_scope(&intent);
    AcipMessageEnvelopeV1 {
        schema_version: ACIP_MESSAGE_SCHEMA_VERSION.to_string(),
        message_id: message_id.to_string(),
        conversation_id: conversation_id.to_string(),
        timestamp_utc: format!("2026-04-28T19:{:02}:00Z", monotonic_order.saturating_sub(1)),
        monotonic_order,
        sender: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "planner.agent".to_string(),
        },
        recipient: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "reviewer.agent".to_string(),
        },
        intent,
        visibility: AcipVisibilityV1::Private,
        trace_requirement: AcipTraceRequirementV1::Summary,
        content: content.to_string(),
        payload_refs: Vec::new(),
        artifact_refs: vec!["runtime/comms/trace/thread_anchor.json".to_string()],
        attachments: Vec::new(),
        authority_scope: Some(authority_scope),
        correlation_id: None,
        prior_message_id: None,
    }
}

fn sample_message_with_payloads(
    message_id: &str,
    conversation_id: &str,
    monotonic_order: u64,
    intent: AcipIntentV1,
    content: &str,
    payload_refs: Vec<AcipPayloadRefV1>,
) -> AcipMessageEnvelopeV1 {
    let mut message = sample_message(
        message_id,
        conversation_id,
        monotonic_order,
        intent,
        content,
    );
    message.payload_refs = payload_refs;
    message.authority_scope = Some(default_message_authority_scope(&message.intent));
    message
}

fn default_message_authority_scope(intent: &AcipIntentV1) -> AcipAuthorityScopeV1 {
    match intent {
        AcipIntentV1::Conversation => AcipAuthorityScopeV1 {
            allowed_actions: vec!["consult".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec![
                "runtime/comms/authority/conversation_basis.json".to_string()
            ],
            delegation_permitted: false,
        },
        AcipIntentV1::Consultation => AcipAuthorityScopeV1 {
            allowed_actions: vec!["consult".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec![
                "runtime/comms/authority/consultation_basis.json".to_string()
            ],
            delegation_permitted: false,
        },
        AcipIntentV1::InvocationSetup | AcipIntentV1::CodingRequest => AcipAuthorityScopeV1 {
            allowed_actions: vec!["invoke".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/invocation_basis.json".to_string()],
            delegation_permitted: false,
        },
        AcipIntentV1::ReviewRequest => AcipAuthorityScopeV1 {
            allowed_actions: vec!["review".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/review_basis.json".to_string()],
            delegation_permitted: false,
        },
        AcipIntentV1::Delegation => AcipAuthorityScopeV1 {
            allowed_actions: vec!["delegate".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/delegation_basis.json".to_string()],
            delegation_permitted: true,
        },
        AcipIntentV1::Negotiation => AcipAuthorityScopeV1 {
            allowed_actions: vec!["negotiate".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/negotiation_basis.json".to_string()],
            delegation_permitted: false,
        },
    }
}

fn sample_invocation_contract() -> AcipInvocationContractV1 {
    AcipInvocationContractV1 {
        schema_version: ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION.to_string(),
        invocation_id: "invoke-0001".to_string(),
        conversation_id: "conv-review-001".to_string(),
        causal_message_id: "msg-review-0001".to_string(),
        caller: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "planner.agent".to_string(),
        },
        target: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "reviewer.agent".to_string(),
        },
        intent: AcipIntentV1::ReviewRequest,
        purpose: "Perform a bounded review and emit the declared report artifacts.".to_string(),
        input_refs: vec![
            "runtime/comms/invocation/review_packet.json".to_string(),
            "runtime/comms/invocation/trace_anchor.json".to_string(),
        ],
        constraints: AcipInvocationConstraintsV1 {
            policy_refs: vec!["runtime/comms/policy/review_policy.json".to_string()],
            required_capabilities: vec!["review".to_string(), "share_artifact".to_string()],
            prohibited_actions: vec!["merge".to_string(), "destructive_edit".to_string()],
            requires_redaction: true,
        },
        expected_outputs: vec![
            AcipExpectedOutputV1 {
                output_id: "review_report".to_string(),
                output_kind: "review_report".to_string(),
                artifact_role: "primary_report".to_string(),
                schema_ref: Some("schemas/review_report.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "review_summary".to_string(),
                output_kind: "review_summary".to_string(),
                artifact_role: "operator_summary".to_string(),
                schema_ref: None,
                required: false,
            },
        ],
        stop_policy: AcipInvocationStopPolicyV1 {
            max_turns: 1,
            max_output_artifacts: 2,
            completion_condition: "emit refusal or satisfy required outputs".to_string(),
            stop_on_refusal: true,
            stop_on_failure: true,
        },
        authority_scope: AcipAuthorityScopeV1 {
            allowed_actions: vec!["review".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/review_basis.json".to_string()],
            delegation_permitted: false,
        },
        decision_event_ref: "gate.review-0001".to_string(),
        response_channel: AcipResponseChannelV1 {
            kind: AcipResponseChannelKindV1::DirectReply,
            channel_ref: "reply.review.thread".to_string(),
        },
        trace_requirement: AcipTraceRequirementV1::Full,
    }
}

fn sample_invocation_event(
    contract: &AcipInvocationContractV1,
    status: AcipInvocationStatusV1,
    output_refs: Vec<String>,
    stop_reason: String,
    refusal_code: Option<String>,
    failure_code: Option<String>,
    evidence_refs: Vec<String>,
) -> AcipInvocationEventV1 {
    AcipInvocationEventV1 {
        schema_version: ACIP_INVOCATION_EVENT_SCHEMA_VERSION.to_string(),
        invocation_id: contract.invocation_id.clone(),
        conversation_id: contract.conversation_id.clone(),
        causal_message_id: contract.causal_message_id.clone(),
        caller: contract.caller.clone(),
        target: contract.target.clone(),
        contract_ref: Some("runtime/comms/invocation/contracts/review_request.json".to_string()),
        contract_sha256: Some(
            "89abcdef0123456789abcdef0123456789abcdef0123456789abcdef01234567".to_string(),
        ),
        decision_event_ref: contract.decision_event_ref.clone(),
        input_refs: contract.input_refs.clone(),
        output_refs,
        status,
        stop_reason,
        refusal_code,
        failure_code,
        evidence_refs,
        trace_requirement: contract.trace_requirement.clone(),
    }
}

fn sample_review_invocation_contract() -> AcipReviewInvocationContractV1 {
    let mut invocation = sample_invocation_contract();
    invocation.input_refs = vec![
        "runtime/comms/review/review_packet.json".to_string(),
        "runtime/comms/review/static_analysis_summary.json".to_string(),
        "runtime/comms/review/validation_evidence.json".to_string(),
    ];
    invocation.constraints.policy_refs = vec![
        "runtime/comms/review/srp.md".to_string(),
        "runtime/comms/policy/review_policy.json".to_string(),
    ];
    invocation.constraints.prohibited_actions = vec![
        "merge".to_string(),
        "push".to_string(),
        "destructive_edit".to_string(),
    ];
    invocation.expected_outputs = vec![
        AcipExpectedOutputV1 {
            output_id: "review_result".to_string(),
            output_kind: "review_result".to_string(),
            artifact_role: "primary_review_result".to_string(),
            schema_ref: Some("schemas/pr_review_result.schema.json".to_string()),
            required: true,
        },
        AcipExpectedOutputV1 {
            output_id: "gate_result".to_string(),
            output_kind: "gate_result".to_string(),
            artifact_role: "pr_open_gate".to_string(),
            schema_ref: Some("schemas/pr_review_gate.schema.json".to_string()),
            required: true,
        },
    ];
    invocation.stop_policy.max_output_artifacts = 2;

    AcipReviewInvocationContractV1 {
        schema_version: ACIP_REVIEW_INVOCATION_SCHEMA_VERSION.to_string(),
        invocation,
        srp_ref: "runtime/comms/review/srp.md".to_string(),
        review_packet_ref: "runtime/comms/review/review_packet.json".to_string(),
        evidence_packet_refs: vec![
            "runtime/comms/review/review_packet.json".to_string(),
            "runtime/comms/review/static_analysis_summary.json".to_string(),
            "runtime/comms/review/validation_evidence.json".to_string(),
        ],
        independence_policy: AcipReviewIndependencePolicyV1 {
            writer_session_id: "writer-session-codex".to_string(),
            writer_model_ref: "gpt-5-codex".to_string(),
            reviewer_session_id: "reviewer-session-fixture".to_string(),
            reviewer_model_ref: "fixture-reviewer".to_string(),
            forbid_same_session: true,
            forbid_same_model_ref: true,
        },
        disposition_contract: AcipReviewDispositionContractV1 {
            allowed_dispositions: vec![
                AcipReviewDispositionV1::Blessed,
                AcipReviewDispositionV1::Blocked,
                AcipReviewDispositionV1::NonProving,
                AcipReviewDispositionV1::Skipped,
            ],
            blessed_handoff: AcipReviewHandoffOutcomeV1::AllowPrFinish,
            blocked_handoff: AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
            non_proving_handoff: AcipReviewHandoffOutcomeV1::OperatorWaiverRequired,
            skipped_handoff: AcipReviewHandoffOutcomeV1::OperatorWaiverRequired,
            gate_result_required: true,
            findings_required_when_blocked: true,
        },
    }
}

fn sample_review_outcome(
    contract: &AcipReviewInvocationContractV1,
    disposition: AcipReviewDispositionV1,
    handoff_outcome: AcipReviewHandoffOutcomeV1,
    findings_ref: Option<String>,
    residual_risk_refs: Vec<String>,
    pr_open_allowed: bool,
) -> AcipReviewOutcomeV1 {
    AcipReviewOutcomeV1 {
        schema_version: ACIP_REVIEW_OUTCOME_SCHEMA_VERSION.to_string(),
        invocation_id: contract.invocation.invocation_id.clone(),
        review_result_ref: "runtime/comms/review/review_result.json".to_string(),
        gate_result_ref: "runtime/comms/review/gate_result.json".to_string(),
        disposition,
        handoff_outcome,
        reviewer_session_id: contract.independence_policy.reviewer_session_id.clone(),
        reviewer_model_ref: contract.independence_policy.reviewer_model_ref.clone(),
        findings_ref,
        residual_risk_refs,
        pr_open_allowed,
    }
}

fn sample_coding_invocation_contract(
    provider_lane: AcipCodingProviderLaneV1,
    execution_mode: AcipCodingExecutionModeV1,
) -> AcipCodingInvocationContractV1 {
    let mut invocation = sample_invocation_contract();
    invocation.invocation_id = match execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => "invoke-coding-worktree-0001".to_string(),
        AcipCodingExecutionModeV1::UnappliedPatch => "invoke-coding-patch-0001".to_string(),
        AcipCodingExecutionModeV1::StructuredProposal => "invoke-coding-proposal-0001".to_string(),
    };
    invocation.conversation_id = "conv-coding-001".to_string();
    invocation.causal_message_id = "msg-coding-0001".to_string();
    invocation.target.id = "coding.agent".to_string();
    invocation.intent = AcipIntentV1::CodingRequest;
    invocation.purpose =
        "Perform bounded coding work and return a reviewable patch or proposal surface."
            .to_string();
    invocation.input_refs = vec![
        "runtime/comms/coding/task_bundle.json".to_string(),
        "runtime/comms/coding/issue_context.md".to_string(),
        "runtime/comms/coding/current_scope.json".to_string(),
    ];
    invocation.constraints.policy_refs = vec![
        "runtime/comms/coding/workflow_conductor_policy.md".to_string(),
        "runtime/comms/policy/coding_policy.json".to_string(),
    ];
    invocation.constraints.prohibited_actions = vec![
        "merge".to_string(),
        "push".to_string(),
        "self_review".to_string(),
    ];
    invocation.stop_policy.max_turns = 4;
    invocation.stop_policy.max_output_artifacts = 3;
    invocation.stop_policy.completion_condition =
        "emit reviewable coding output and validation evidence".to_string();
    invocation.authority_scope.allowed_actions =
        vec!["invoke".to_string(), "share_artifact".to_string()];
    invocation.authority_scope.authority_basis_refs =
        vec!["runtime/comms/authority/coding_basis.json".to_string()];
    invocation.authority_scope.delegation_permitted = false;
    invocation.decision_event_ref = "gate.coding-0001".to_string();
    invocation.response_channel = AcipResponseChannelV1 {
        kind: AcipResponseChannelKindV1::ArtifactReply,
        channel_ref: "runtime/comms/coding/outcome.json".to_string(),
    };
    invocation.expected_outputs = match execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => vec![
            AcipExpectedOutputV1 {
                output_id: "patch_manifest".to_string(),
                output_kind: "patch_manifest".to_string(),
                artifact_role: "worktree_change_manifest".to_string(),
                schema_ref: Some("schemas/coding_patch_manifest.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "validation_summary".to_string(),
                output_kind: "validation_summary".to_string(),
                artifact_role: "validation_summary".to_string(),
                schema_ref: Some("schemas/validation_summary.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "review_handoff".to_string(),
                output_kind: "review_handoff".to_string(),
                artifact_role: "review_handoff".to_string(),
                schema_ref: Some("schemas/review_handoff.schema.json".to_string()),
                required: true,
            },
        ],
        AcipCodingExecutionModeV1::UnappliedPatch => vec![
            AcipExpectedOutputV1 {
                output_id: "patch_diff".to_string(),
                output_kind: "patch_diff".to_string(),
                artifact_role: "proposed_patch".to_string(),
                schema_ref: Some("schemas/unified_diff.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "validation_summary".to_string(),
                output_kind: "validation_summary".to_string(),
                artifact_role: "validation_summary".to_string(),
                schema_ref: Some("schemas/validation_summary.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "review_handoff".to_string(),
                output_kind: "review_handoff".to_string(),
                artifact_role: "review_handoff".to_string(),
                schema_ref: Some("schemas/review_handoff.schema.json".to_string()),
                required: true,
            },
        ],
        AcipCodingExecutionModeV1::StructuredProposal => vec![
            AcipExpectedOutputV1 {
                output_id: "structured_proposal".to_string(),
                output_kind: "structured_proposal".to_string(),
                artifact_role: "proposed_change_plan".to_string(),
                schema_ref: Some("schemas/coding_proposal.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "validation_summary".to_string(),
                output_kind: "validation_summary".to_string(),
                artifact_role: "validation_summary".to_string(),
                schema_ref: Some("schemas/validation_summary.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "review_handoff".to_string(),
                output_kind: "review_handoff".to_string(),
                artifact_role: "review_handoff".to_string(),
                schema_ref: Some("schemas/review_handoff.schema.json".to_string()),
                required: true,
            },
        ],
    };

    let (writer_session_id, writer_model_ref) = match provider_lane {
        AcipCodingProviderLaneV1::CodexIssueWorktree => (
            "writer-session-codex".to_string(),
            "gpt-5-codex".to_string(),
        ),
        AcipCodingProviderLaneV1::ChatgptApi => {
            ("writer-session-openai".to_string(), "gpt-5-api".to_string())
        }
        AcipCodingProviderLaneV1::ClaudeApi => (
            "writer-session-anthropic".to_string(),
            "claude-api".to_string(),
        ),
        AcipCodingProviderLaneV1::LocalOllama => (
            "writer-session-ollama".to_string(),
            "gemma3-local".to_string(),
        ),
        AcipCodingProviderLaneV1::OtherProposalOnly => (
            "writer-session-other".to_string(),
            "other-provider".to_string(),
        ),
    };

    invocation.constraints.required_capabilities = match provider_lane {
        AcipCodingProviderLaneV1::CodexIssueWorktree => {
            vec!["code_edit".to_string(), "share_artifact".to_string()]
        }
        AcipCodingProviderLaneV1::ChatgptApi
        | AcipCodingProviderLaneV1::ClaudeApi
        | AcipCodingProviderLaneV1::LocalOllama
        | AcipCodingProviderLaneV1::OtherProposalOnly => {
            vec!["propose_change".to_string(), "share_artifact".to_string()]
        }
    };

    AcipCodingInvocationContractV1 {
        schema_version: ACIP_CODING_INVOCATION_SCHEMA_VERSION.to_string(),
        invocation,
        provider_lane: provider_lane.clone(),
        execution_mode: execution_mode.clone(),
        issue_ref: "issues/2627".to_string(),
        task_bundle_ref: "runtime/comms/coding/task_bundle.json".to_string(),
        issue_worktree_required: provider_lane == AcipCodingProviderLaneV1::CodexIssueWorktree,
        allowed_edit_paths: vec![
            "adl/src/agent_comms.rs".to_string(),
            "docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md".to_string(),
            "docs/milestones/v0.90.5/features/CODING_AGENT_RUNNER.md".to_string(),
            "docs/milestones/v0.90.5/features/LOCAL_MODEL_PR_REVIEWER_TOOL.md".to_string(),
            "docs/milestones/v0.90.5/features/README.md".to_string(),
            "docs/milestones/v0.90.5/FEATURE_DOCS_v0.90.5.md".to_string(),
        ],
        validation_commands: vec![
            "cargo fmt --check".to_string(),
            "cargo test agent_comms --lib".to_string(),
        ],
        patch_format: match execution_mode {
            AcipCodingExecutionModeV1::WorktreeEdit => "patch_manifest_v1".to_string(),
            AcipCodingExecutionModeV1::UnappliedPatch => "unified_diff".to_string(),
            AcipCodingExecutionModeV1::StructuredProposal => "structured_proposal_v1".to_string(),
        },
        approval_policy: AcipCodingApprovalPolicyV1 {
            review_required_before_pr_finish: true,
            required_review_schema_ref: ACIP_REVIEW_INVOCATION_SCHEMA_VERSION.to_string(),
            writer_session_id,
            writer_model_ref,
            forbid_same_session_blessing: true,
            forbid_same_model_blessing: true,
        },
    }
}

fn sample_coding_outcome(contract: &AcipCodingInvocationContractV1) -> AcipCodingOutcomeV1 {
    let (disposition, primary_output_ref) = match contract.execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => (
            AcipCodingDispositionV1::PatchReadyForReview,
            "runtime/comms/coding/patch_manifest.json".to_string(),
        ),
        AcipCodingExecutionModeV1::UnappliedPatch => (
            AcipCodingDispositionV1::ProposalReadyForReview,
            "runtime/comms/coding/proposed_changes.diff".to_string(),
        ),
        AcipCodingExecutionModeV1::StructuredProposal => (
            AcipCodingDispositionV1::ProposalReadyForReview,
            "runtime/comms/coding/proposal.json".to_string(),
        ),
    };

    AcipCodingOutcomeV1 {
        schema_version: ACIP_CODING_OUTCOME_SCHEMA_VERSION.to_string(),
        invocation_id: contract.invocation.invocation_id.clone(),
        provider_lane: contract.provider_lane.clone(),
        execution_mode: contract.execution_mode.clone(),
        disposition,
        primary_output_ref,
        validation_result_refs: vec!["runtime/comms/coding/validation_summary.json".to_string()],
        review_handoff_ref: "runtime/comms/coding/review_handoff.json".to_string(),
        writer_session_id: contract.approval_policy.writer_session_id.clone(),
        writer_model_ref: contract.approval_policy.writer_model_ref.clone(),
    }
}

fn sample_trace_bundle(status: AcipInvocationStatusV1) -> AcipTraceBundleV1 {
    let contract = sample_invocation_contract();
    let terminal_event = match status {
        AcipInvocationStatusV1::Completed => AcipTraceEventV1 {
            event_id: "trace-0004".to_string(),
            conversation_id: contract.conversation_id.clone(),
            invocation_id: Some(contract.invocation_id.clone()),
            event_kind: AcipTraceEventKindV1::InvocationCompleted,
            source_message_id: Some(contract.causal_message_id.clone()),
            contract_ref: Some("runtime/comms/invocation/contracts/review_request.json".to_string()),
            decision_event_ref: Some(contract.decision_event_ref.clone()),
            invocation_status: Some(AcipInvocationStatusV1::Completed),
            output_refs: vec!["runtime/comms/invocation/review_report.json".to_string()],
            evidence_refs: vec!["runtime/comms/invocation/evidence/completed_trace.json".to_string()],
            summary: "Invocation completed with declared review output contract satisfied."
                .to_string(),
            requires_redaction: false,
        },
        AcipInvocationStatusV1::Refused => AcipTraceEventV1 {
            event_id: "trace-0004".to_string(),
            conversation_id: contract.conversation_id.clone(),
            invocation_id: Some(contract.invocation_id.clone()),
            event_kind: AcipTraceEventKindV1::InvocationRefused,
            source_message_id: Some(contract.causal_message_id.clone()),
            contract_ref: Some("runtime/comms/invocation/contracts/review_request.json".to_string()),
            decision_event_ref: Some(contract.decision_event_ref.clone()),
            invocation_status: Some(AcipInvocationStatusV1::Refused),
            output_refs: Vec::new(),
            evidence_refs: vec!["runtime/comms/invocation/evidence/refusal_trace.json".to_string()],
            summary: "Invocation refused at the governed boundary with bounded reviewer-visible evidence."
                .to_string(),
            requires_redaction: true,
        },
        AcipInvocationStatusV1::Failed => AcipTraceEventV1 {
            event_id: "trace-0004".to_string(),
            conversation_id: contract.conversation_id.clone(),
            invocation_id: Some(contract.invocation_id.clone()),
            event_kind: AcipTraceEventKindV1::InvocationFailed,
            source_message_id: Some(contract.causal_message_id.clone()),
            contract_ref: Some("runtime/comms/invocation/contracts/review_request.json".to_string()),
            decision_event_ref: Some(contract.decision_event_ref.clone()),
            invocation_status: Some(AcipInvocationStatusV1::Failed),
            output_refs: Vec::new(),
            evidence_refs: vec!["runtime/comms/invocation/evidence/failure_trace.json".to_string()],
            summary: "Invocation failed after decision with bounded failure evidence and no raw payload leak."
                .to_string(),
            requires_redaction: true,
        },
        _ => unreachable!("trace fixture only supports terminal statuses"),
    };

    let visible_artifact_refs = match status {
        AcipInvocationStatusV1::Completed => vec![
            "runtime/comms/trace/reviewer_trace.json".to_string(),
            "runtime/comms/invocation/review_report.json".to_string(),
        ],
        AcipInvocationStatusV1::Refused => vec![
            "runtime/comms/trace/reviewer_trace.json".to_string(),
            "runtime/comms/invocation/evidence/refusal_trace.json".to_string(),
        ],
        AcipInvocationStatusV1::Failed => vec![
            "runtime/comms/trace/reviewer_trace.json".to_string(),
            "runtime/comms/invocation/evidence/failure_trace.json".to_string(),
        ],
        _ => unreachable!("trace fixture only supports terminal statuses"),
    };

    AcipTraceBundleV1 {
        schema_version: ACIP_TRACE_BUNDLE_SCHEMA_VERSION.to_string(),
        conversation_id: contract.conversation_id.clone(),
        trace_events: vec![
            AcipTraceEventV1 {
                event_id: "trace-0001".to_string(),
                conversation_id: contract.conversation_id.clone(),
                invocation_id: None,
                event_kind: AcipTraceEventKindV1::MessageCreated,
                source_message_id: Some(contract.causal_message_id.clone()),
                contract_ref: None,
                decision_event_ref: None,
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec!["runtime/comms/trace/message_anchor.json".to_string()],
                summary: "Message created with bounded conversation anchor and trace requirement."
                    .to_string(),
                requires_redaction: false,
            },
            AcipTraceEventV1 {
                event_id: "trace-0002".to_string(),
                conversation_id: contract.conversation_id.clone(),
                invocation_id: Some(contract.invocation_id.clone()),
                event_kind: AcipTraceEventKindV1::InvocationContractDeclared,
                source_message_id: Some(contract.causal_message_id.clone()),
                contract_ref: Some(
                    "runtime/comms/invocation/contracts/review_request.json".to_string(),
                ),
                decision_event_ref: Some(contract.decision_event_ref.clone()),
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec!["runtime/comms/trace/contract_anchor.json".to_string()],
                summary:
                    "Invocation contract declared with explicit output, stop, and authority bounds."
                        .to_string(),
                requires_redaction: false,
            },
            AcipTraceEventV1 {
                event_id: "trace-0003".to_string(),
                conversation_id: contract.conversation_id.clone(),
                invocation_id: Some(contract.invocation_id.clone()),
                event_kind: AcipTraceEventKindV1::DecisionRecorded,
                source_message_id: Some(contract.causal_message_id.clone()),
                contract_ref: Some(
                    "runtime/comms/invocation/contracts/review_request.json".to_string(),
                ),
                decision_event_ref: Some(contract.decision_event_ref.clone()),
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec!["runtime/comms/trace/gate_result.json".to_string()],
                summary: "Freedom Gate decision recorded before terminal invocation state."
                    .to_string(),
                requires_redaction: false,
            },
            terminal_event,
        ],
        audience_views: vec![
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Actor,
                narrative_ref: "runtime/comms/trace/actor_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-0001".to_string(),
                    "trace-0002".to_string(),
                    "trace-0003".to_string(),
                    "trace-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/trace/actor_view.json".to_string(),
                    "runtime/comms/trace/private_payload_summary.json".to_string(),
                ],
                redacted_elements: vec!["secret_values".to_string()],
                allows_private_payload_refs: true,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Operator,
                narrative_ref: "runtime/comms/trace/operator_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-0001".to_string(),
                    "trace-0002".to_string(),
                    "trace-0003".to_string(),
                    "trace-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/trace/operator_view.json".to_string(),
                    "runtime/comms/trace/redacted_payload_digest.json".to_string(),
                ],
                redacted_elements: vec![
                    "raw_tool_args".to_string(),
                    "local_host_paths".to_string(),
                ],
                allows_private_payload_refs: true,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Reviewer,
                narrative_ref: "runtime/comms/trace/reviewer_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-0002".to_string(),
                    "trace-0003".to_string(),
                    "trace-0004".to_string(),
                ],
                visible_artifact_refs: visible_artifact_refs.clone(),
                redacted_elements: vec![
                    "private_payload_refs".to_string(),
                    "raw_tool_args".to_string(),
                    "rejected_alternative_details".to_string(),
                ],
                allows_private_payload_refs: false,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Public,
                narrative_ref: "runtime/comms/trace/public_view.json".to_string(),
                visible_event_ids: vec!["trace-0003".to_string(), "trace-0004".to_string()],
                visible_artifact_refs: vec!["runtime/comms/trace/public_summary.json".to_string()],
                redacted_elements: vec![
                    "private_payload_refs".to_string(),
                    "raw_tool_args".to_string(),
                    "local_host_paths".to_string(),
                    "rejected_alternative_details".to_string(),
                ],
                allows_private_payload_refs: false,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Observatory,
                narrative_ref: "runtime/comms/trace/observatory_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-0001".to_string(),
                    "trace-0003".to_string(),
                    "trace-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/trace/observatory_summary.json".to_string(),
                    "runtime/comms/trace/redacted_payload_digest.json".to_string(),
                ],
                redacted_elements: vec![
                    "private_payload_refs".to_string(),
                    "raw_tool_args".to_string(),
                    "rejected_alternative_details".to_string(),
                ],
                allows_private_payload_refs: false,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
        ],
        replay_contract: AcipReplayContractV1 {
            replay_posture: AcipReplayPostureV1::FixtureBackedDeterministic,
            fixture_ref: "runtime/comms/fixtures/acip_invocation_fixture_set_v1.json".to_string(),
            fixture_case: match status {
                AcipInvocationStatusV1::Completed => "completed".to_string(),
                AcipInvocationStatusV1::Refused => "refused".to_string(),
                AcipInvocationStatusV1::Failed => "failed".to_string(),
                _ => unreachable!("trace fixture only supports terminal statuses"),
            },
            deterministic_event_order: true,
            deterministic_redaction_views: true,
            remote_provider_required: false,
        },
        evidence_packet_refs: vec![
            "runtime/comms/trace/message_anchor.json".to_string(),
            "runtime/comms/trace/gate_result.json".to_string(),
            match status {
                AcipInvocationStatusV1::Completed => {
                    "runtime/comms/invocation/evidence/completed_trace.json".to_string()
                }
                AcipInvocationStatusV1::Refused => {
                    "runtime/comms/invocation/evidence/refusal_trace.json".to_string()
                }
                AcipInvocationStatusV1::Failed => {
                    "runtime/comms/invocation/evidence/failure_trace.json".to_string()
                }
                _ => unreachable!("trace fixture only supports terminal statuses"),
            },
        ],
    }
}

fn validate_acip_trace_event_v1(
    event: &AcipTraceEventV1,
    expected_conversation_id: &str,
) -> Result<()> {
    validate_id(&event.event_id, "event_id")?;
    if event.conversation_id != expected_conversation_id {
        return Err(anyhow!(
            "trace event conversation_id must match the bundle conversation_id"
        ));
    }
    if let Some(invocation_id) = &event.invocation_id {
        validate_id(invocation_id, "invocation_id")?;
    }
    if let Some(source_message_id) = &event.source_message_id {
        validate_id(source_message_id, "source_message_id")?;
    }
    if let Some(contract_ref) = &event.contract_ref {
        validate_repo_relative_ref(contract_ref, "contract_ref")?;
    }
    if let Some(decision_event_ref) = &event.decision_event_ref {
        validate_gate_decision_ref(decision_event_ref, "decision_event_ref")?;
    }
    if event.output_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("output_refs exceeds bounded list length"));
    }
    for reference in &event.output_refs {
        validate_repo_relative_ref(reference, "output_refs[]")?;
    }
    if event.evidence_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("evidence_refs exceeds bounded list length"));
    }
    for reference in &event.evidence_refs {
        validate_repo_relative_ref(reference, "evidence_refs[]")?;
    }
    validate_non_empty(&event.summary, "summary")?;
    if event.summary.chars().count() > MAX_INLINE_SUMMARY_CHARS {
        return Err(anyhow!(
            "summary exceeds bounded inline posture of {MAX_INLINE_SUMMARY_CHARS} characters"
        ));
    }
    ensure_safe_trace_summary(&event.summary, "summary")?;

    match event.event_kind {
        AcipTraceEventKindV1::MessageCreated => {
            if event.source_message_id.is_none() {
                return Err(anyhow!(
                    "message_created trace event must carry source_message_id"
                ));
            }
            if event.invocation_id.is_some() || event.contract_ref.is_some() {
                return Err(anyhow!(
                    "message_created trace event must not carry invocation-only fields"
                ));
            }
        }
        AcipTraceEventKindV1::InvocationContractDeclared => {
            if event.invocation_id.is_none() || event.contract_ref.is_none() {
                return Err(anyhow!(
                    "invocation_contract_declared trace event must carry invocation_id and contract_ref"
                ));
            }
            if event.decision_event_ref.is_none() {
                return Err(anyhow!(
                    "invocation_contract_declared trace event must carry decision_event_ref"
                ));
            }
        }
        AcipTraceEventKindV1::DecisionRecorded => {
            if event.invocation_id.is_none()
                || event.contract_ref.is_none()
                || event.decision_event_ref.is_none()
            {
                return Err(anyhow!(
                    "decision_recorded trace event must carry invocation_id, contract_ref, and decision_event_ref"
                ));
            }
        }
        AcipTraceEventKindV1::InvocationCompleted => {
            if event.invocation_status != Some(AcipInvocationStatusV1::Completed) {
                return Err(anyhow!(
                    "invocation_completed trace event must carry invocation_status 'completed'"
                ));
            }
            if event.invocation_id.is_none()
                || event.contract_ref.is_none()
                || event.output_refs.is_empty()
                || event.decision_event_ref.is_none()
            {
                return Err(anyhow!(
                    "invocation_completed trace event must carry invocation_id, contract_ref, decision_event_ref, and output_refs"
                ));
            }
        }
        AcipTraceEventKindV1::InvocationRefused => {
            if event.invocation_status != Some(AcipInvocationStatusV1::Refused) {
                return Err(anyhow!(
                    "invocation_refused trace event must carry invocation_status 'refused'"
                ));
            }
            if event.invocation_id.is_none()
                || event.contract_ref.is_none()
                || event.evidence_refs.is_empty()
                || event.decision_event_ref.is_none()
            {
                return Err(anyhow!(
                    "invocation_refused trace event must carry invocation_id, contract_ref, decision_event_ref, and evidence_refs"
                ));
            }
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "invocation_refused trace event must not carry output_refs"
                ));
            }
            if !event.requires_redaction {
                return Err(anyhow!(
                    "invocation_refused trace event must require redaction"
                ));
            }
        }
        AcipTraceEventKindV1::InvocationFailed => {
            if event.invocation_status != Some(AcipInvocationStatusV1::Failed) {
                return Err(anyhow!(
                    "invocation_failed trace event must carry invocation_status 'failed'"
                ));
            }
            if event.invocation_id.is_none()
                || event.contract_ref.is_none()
                || event.evidence_refs.is_empty()
                || event.decision_event_ref.is_none()
            {
                return Err(anyhow!(
                    "invocation_failed trace event must carry invocation_id, contract_ref, decision_event_ref, and evidence_refs"
                ));
            }
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "invocation_failed trace event must not carry output_refs"
                ));
            }
            if !event.requires_redaction {
                return Err(anyhow!(
                    "invocation_failed trace event must require redaction"
                ));
            }
        }
    }

    Ok(())
}

fn validate_acip_trace_audience_view_v1(
    view: &AcipTraceAudienceViewV1,
    known_event_ids: &BTreeSet<String>,
) -> Result<()> {
    validate_repo_relative_ref(&view.narrative_ref, "narrative_ref")?;
    if view.visible_event_ids.is_empty() {
        return Err(anyhow!("trace audience view must carry visible_event_ids"));
    }
    for event_id in &view.visible_event_ids {
        validate_id(event_id, "visible_event_ids[]")?;
        if !known_event_ids.contains(event_id) {
            return Err(anyhow!(
                "trace audience view references unknown event_id '{}'",
                event_id
            ));
        }
    }
    for reference in &view.visible_artifact_refs {
        validate_repo_relative_ref(reference, "visible_artifact_refs[]")?;
        if matches!(
            view.audience,
            AcipTraceAudienceV1::Reviewer
                | AcipTraceAudienceV1::Public
                | AcipTraceAudienceV1::Observatory
        ) {
            ensure_redacted_trace_ref(reference, "visible_artifact_refs[]")?;
        }
    }
    if view.redacted_elements.is_empty() {
        return Err(anyhow!(
            "trace audience view must declare redacted_elements"
        ));
    }
    for element in &view.redacted_elements {
        validate_id(element, "redacted_elements[]")?;
    }
    match view.audience {
        AcipTraceAudienceV1::Actor | AcipTraceAudienceV1::Operator => {}
        AcipTraceAudienceV1::Reviewer
        | AcipTraceAudienceV1::Public
        | AcipTraceAudienceV1::Observatory => {
            ensure_redacted_trace_ref(&view.narrative_ref, "narrative_ref")?;
            if view.allows_private_payload_refs {
                return Err(anyhow!(
                    "reviewer, public, and observatory views must not allow private payload refs"
                ));
            }
            if view.allows_raw_tool_args {
                return Err(anyhow!(
                    "reviewer, public, and observatory views must not allow raw tool args"
                ));
            }
            if view.allows_local_host_paths {
                return Err(anyhow!(
                    "reviewer, public, and observatory views must not allow local host paths"
                ));
            }
            if view.allows_rejected_alternative_details {
                return Err(anyhow!(
                    "reviewer, public, and observatory views must not allow rejected alternative details"
                ));
            }
        }
    }
    Ok(())
}

fn validate_acip_replay_contract_v1(contract: &AcipReplayContractV1) -> Result<()> {
    validate_repo_relative_ref(&contract.fixture_ref, "fixture_ref")?;
    validate_id(&contract.fixture_case, "fixture_case")?;
    if !contract.deterministic_event_order {
        return Err(anyhow!(
            "ACIP replay contract requires deterministic_event_order"
        ));
    }
    if !contract.deterministic_redaction_views {
        return Err(anyhow!(
            "ACIP replay contract requires deterministic_redaction_views"
        ));
    }
    if contract.remote_provider_required {
        return Err(anyhow!(
            "ACIP replay contract must remain fixture-backed and local for v1"
        ));
    }
    Ok(())
}

fn ensure_safe_trace_summary(value: &str, field: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    for forbidden in [
        "secret",
        "token",
        "password",
        "prompt",
        "tool_args",
        "raw tool arguments",
        "raw tool args",
        "private_state",
        "private state",
        "rejected_alternative",
        "/users/",
        "/home/",
        "/tmp/",
        "/var/folders/",
        "c:\\",
    ] {
        if lowered.contains(forbidden) {
            return Err(anyhow!(
                "{field} must not leak protected trace content '{}'",
                forbidden
            ));
        }
    }
    Ok(())
}

fn ensure_redacted_trace_ref(value: &str, field: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    for forbidden in [
        "private_state",
        "raw_args",
        "raw_result",
        "prompt",
        "secret",
        "rejected_alternative",
    ] {
        if lowered.contains(forbidden) {
            return Err(anyhow!(
                "{field} must not expose unredacted trace ref '{}'",
                forbidden
            ));
        }
    }
    Ok(())
}

fn trace_event_kind_str(kind: &AcipTraceEventKindV1) -> &'static str {
    match kind {
        AcipTraceEventKindV1::MessageCreated => "message_created",
        AcipTraceEventKindV1::InvocationContractDeclared => "invocation_contract_declared",
        AcipTraceEventKindV1::DecisionRecorded => "decision_recorded",
        AcipTraceEventKindV1::InvocationCompleted => "invocation_completed",
        AcipTraceEventKindV1::InvocationRefused => "invocation_refused",
        AcipTraceEventKindV1::InvocationFailed => "invocation_failed",
    }
}

fn trace_audience_str(audience: &AcipTraceAudienceV1) -> &'static str {
    match audience {
        AcipTraceAudienceV1::Actor => "actor",
        AcipTraceAudienceV1::Operator => "operator",
        AcipTraceAudienceV1::Reviewer => "reviewer",
        AcipTraceAudienceV1::Public => "public",
        AcipTraceAudienceV1::Observatory => "observatory",
    }
}

fn payload_ref(
    payload_kind: &str,
    payload_ref: &str,
    media_type: &str,
    byte_length: u64,
    inline_summary: Option<&str>,
) -> AcipPayloadRefV1 {
    AcipPayloadRefV1 {
        payload_kind: payload_kind.to_string(),
        payload_ref: payload_ref.to_string(),
        media_type: media_type.to_string(),
        content_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .to_string(),
        byte_length,
        inline_summary: inline_summary.map(str::to_string),
    }
}

fn validate_address(address: &AcipAddressV1, field: &str) -> Result<()> {
    validate_id(&address.id, &format!("{field}.id"))?;
    Ok(())
}

fn validate_payload_ref(payload_ref: &AcipPayloadRefV1) -> Result<()> {
    validate_id(&payload_ref.payload_kind, "payload_kind")?;
    validate_repo_relative_ref(&payload_ref.payload_ref, "payload_ref")?;
    validate_non_empty(&payload_ref.media_type, "media_type")?;
    validate_sha256(&payload_ref.content_sha256, "content_sha256")?;
    if payload_ref.byte_length == 0 {
        return Err(anyhow!("byte_length must be positive"));
    }
    if let Some(summary) = &payload_ref.inline_summary {
        if summary.chars().count() > MAX_INLINE_SUMMARY_CHARS {
            return Err(anyhow!(
                "inline_summary exceeds bounded inline posture of {MAX_INLINE_SUMMARY_CHARS} characters"
            ));
        }
        if payload_ref.byte_length > MAX_INLINE_BYTE_LENGTH {
            return Err(anyhow!(
                "inline_summary may only summarize bounded payloads up to {MAX_INLINE_BYTE_LENGTH} bytes"
            ));
        }
    }
    Ok(())
}

fn validate_attachment_ref(attachment: &AcipAttachmentRefV1) -> Result<()> {
    validate_id(&attachment.name, "attachment.name")?;
    validate_non_empty(&attachment.media_type, "attachment.media_type")?;
    validate_sha256(&attachment.content_sha256, "attachment.content_sha256")?;
    if attachment.byte_length == 0 {
        return Err(anyhow!("attachment.byte_length must be positive"));
    }
    Ok(())
}

fn validate_authority_scope(scope: &AcipAuthorityScopeV1) -> Result<()> {
    if scope.allowed_actions.is_empty() {
        return Err(anyhow!("authority_scope.allowed_actions must not be empty"));
    }
    if scope.authority_basis_refs.is_empty() {
        return Err(anyhow!(
            "authority_scope.authority_basis_refs must not be empty"
        ));
    }
    let supported = [
        "consult",
        "invoke",
        "review",
        "delegate",
        "negotiate",
        "share_artifact",
    ];
    let mut seen = BTreeSet::new();
    for action in &scope.allowed_actions {
        validate_id(action, "authority_scope.allowed_actions")?;
        if !supported.contains(&action.as_str()) {
            return Err(anyhow!(
                "unsupported authority_scope.allowed_actions '{}'",
                action
            ));
        }
        if !seen.insert(action.clone()) {
            return Err(anyhow!(
                "authority_scope.allowed_actions contains duplicate '{}'",
                action
            ));
        }
    }
    for basis_ref in &scope.authority_basis_refs {
        validate_repo_relative_ref(basis_ref, "authority_scope.authority_basis_refs[]")?;
    }
    Ok(())
}

fn validate_message_intent_authority_alignment(
    intent: &AcipIntentV1,
    scope: &AcipAuthorityScopeV1,
) -> Result<()> {
    let allow_only = |permitted: &[&str]| -> Result<()> {
        for action in &scope.allowed_actions {
            if !permitted.contains(&action.as_str()) {
                return Err(anyhow!(
                    "message intent '{}' must not claim authority action '{}'",
                    intent.as_str(),
                    action
                ));
            }
        }
        Ok(())
    };
    let requires = |action: &str| -> Result<()> {
        if !scope
            .allowed_actions
            .iter()
            .any(|allowed| allowed == action)
        {
            return Err(anyhow!(
                "message intent '{}' requires authority_scope.allowed_actions to include '{}'",
                intent.as_str(),
                action
            ));
        }
        Ok(())
    };

    match intent {
        AcipIntentV1::Conversation => allow_only(&["consult", "share_artifact"])?,
        AcipIntentV1::Consultation => allow_only(&["consult", "share_artifact"])?,
        AcipIntentV1::Negotiation => allow_only(&["consult", "negotiate", "share_artifact"])?,
        AcipIntentV1::InvocationSetup => {
            allow_only(&["invoke", "share_artifact"])?;
            requires("invoke")?;
        }
        AcipIntentV1::ReviewRequest => {
            allow_only(&["review", "share_artifact"])?;
            requires("review")?;
        }
        AcipIntentV1::CodingRequest => {
            allow_only(&["invoke", "share_artifact"])?;
            requires("invoke")?;
        }
        AcipIntentV1::Delegation => {
            allow_only(&["delegate", "share_artifact"])?;
            requires("delegate")?;
            if !scope.delegation_permitted {
                return Err(anyhow!(
                    "message intent 'delegation' requires authority_scope.delegation_permitted"
                ));
            }
        }
    }

    Ok(())
}

pub fn validate_acip_trace_bundle_v1(bundle: &AcipTraceBundleV1) -> Result<()> {
    if bundle.schema_version != ACIP_TRACE_BUNDLE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP trace bundle requires schema_version '{}'",
            ACIP_TRACE_BUNDLE_SCHEMA_VERSION
        ));
    }
    validate_id(&bundle.conversation_id, "conversation_id")?;
    if bundle.trace_events.is_empty() {
        return Err(anyhow!("ACIP trace bundle requires trace_events"));
    }
    if bundle.trace_events.len() > MAX_LIST_LEN {
        return Err(anyhow!("trace_events exceeds bounded list length"));
    }
    let mut seen_event_ids = BTreeSet::new();
    let mut seen_event_kinds = BTreeSet::new();
    let mut terminal_events = 0_usize;
    let mut canonical_invocation_id: Option<&str> = None;
    let mut canonical_contract_ref: Option<&str> = None;
    let mut canonical_decision_ref: Option<&str> = None;
    for (index, event) in bundle.trace_events.iter().enumerate() {
        validate_acip_trace_event_v1(event, &bundle.conversation_id)?;
        if !seen_event_ids.insert(event.event_id.clone()) {
            return Err(anyhow!(
                "trace bundle contains duplicate event_id '{}'",
                event.event_id
            ));
        }
        if !seen_event_kinds.insert(event.event_kind.clone()) {
            return Err(anyhow!(
                "trace bundle must not contain duplicate event kind '{}'",
                trace_event_kind_str(&event.event_kind)
            ));
        }
        match index {
            0 if event.event_kind != AcipTraceEventKindV1::MessageCreated => {
                return Err(anyhow!(
                    "trace bundle must preserve canonical event order: message_created first"
                ))
            }
            1 if event.event_kind != AcipTraceEventKindV1::InvocationContractDeclared => {
                return Err(anyhow!(
                    "trace bundle must preserve canonical event order: invocation_contract_declared second"
                ))
            }
            2 if event.event_kind != AcipTraceEventKindV1::DecisionRecorded => {
                return Err(anyhow!(
                    "trace bundle must preserve canonical event order: decision_recorded third"
                ))
            }
            _ => {}
        }
        if !matches!(event.event_kind, AcipTraceEventKindV1::MessageCreated) {
            let Some(invocation_id) = event.invocation_id.as_deref() else {
                return Err(anyhow!(
                    "trace bundle non-message events must carry invocation_id"
                ));
            };
            let Some(contract_ref) = event.contract_ref.as_deref() else {
                return Err(anyhow!(
                    "trace bundle non-message events must carry contract_ref"
                ));
            };
            let Some(decision_ref) = event.decision_event_ref.as_deref() else {
                return Err(anyhow!(
                    "trace bundle non-message events must carry decision_event_ref"
                ));
            };
            if let Some(expected) = canonical_invocation_id {
                if expected != invocation_id {
                    return Err(anyhow!(
                        "trace bundle must preserve one canonical invocation_id across non-message events"
                    ));
                }
            } else {
                canonical_invocation_id = Some(invocation_id);
            }
            if let Some(expected) = canonical_contract_ref {
                if expected != contract_ref {
                    return Err(anyhow!(
                        "trace bundle must preserve one canonical contract_ref across non-message events"
                    ));
                }
            } else {
                canonical_contract_ref = Some(contract_ref);
            }
            if let Some(expected) = canonical_decision_ref {
                if expected != decision_ref {
                    return Err(anyhow!(
                        "trace bundle must preserve one canonical decision_event_ref across non-message events"
                    ));
                }
            } else {
                canonical_decision_ref = Some(decision_ref);
            }
        }
        match event.event_kind {
            AcipTraceEventKindV1::InvocationCompleted
            | AcipTraceEventKindV1::InvocationRefused
            | AcipTraceEventKindV1::InvocationFailed => {
                terminal_events += 1;
            }
            _ => {}
        }
    }
    for required in [
        AcipTraceEventKindV1::MessageCreated,
        AcipTraceEventKindV1::InvocationContractDeclared,
        AcipTraceEventKindV1::DecisionRecorded,
    ] {
        if !seen_event_kinds.contains(&required) {
            return Err(anyhow!(
                "trace bundle missing required event kind '{}'",
                trace_event_kind_str(&required)
            ));
        }
    }
    if terminal_events != 1 {
        return Err(anyhow!(
            "trace bundle must contain exactly one terminal event kind"
        ));
    }
    let last_event = bundle
        .trace_events
        .last()
        .expect("trace bundle checked non-empty above");
    if !matches!(
        last_event.event_kind,
        AcipTraceEventKindV1::InvocationCompleted
            | AcipTraceEventKindV1::InvocationRefused
            | AcipTraceEventKindV1::InvocationFailed
    ) {
        return Err(anyhow!(
            "trace bundle must preserve canonical event order: terminal invocation event last"
        ));
    }
    validate_acip_replay_contract_v1(&bundle.replay_contract)?;
    if bundle.evidence_packet_refs.is_empty() {
        return Err(anyhow!("ACIP trace bundle requires evidence_packet_refs"));
    }
    for reference in &bundle.evidence_packet_refs {
        validate_repo_relative_ref(reference, "evidence_packet_refs[]")?;
    }
    if bundle.audience_views.len() != 5 {
        return Err(anyhow!(
            "ACIP trace bundle requires exactly five canonical audience_views"
        ));
    }
    let mut seen_audiences = BTreeSet::new();
    for view in &bundle.audience_views {
        validate_acip_trace_audience_view_v1(view, &seen_event_ids)?;
        if !seen_audiences.insert(view.audience.clone()) {
            return Err(anyhow!(
                "trace bundle contains duplicate audience view '{}'",
                trace_audience_str(&view.audience)
            ));
        }
    }
    for required in [
        AcipTraceAudienceV1::Actor,
        AcipTraceAudienceV1::Operator,
        AcipTraceAudienceV1::Reviewer,
        AcipTraceAudienceV1::Public,
        AcipTraceAudienceV1::Observatory,
    ] {
        if !seen_audiences.contains(&required) {
            return Err(anyhow!(
                "trace bundle missing required audience view '{}'",
                trace_audience_str(&required)
            ));
        }
    }
    Ok(())
}

pub fn validate_acip_trace_fixture_set_v1(fixtures: &AcipTraceFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_TRACE_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP trace fixture set requires schema_version '{}'",
            ACIP_TRACE_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!("ACIP trace fixture set requires negative_cases"));
    }
    validate_acip_trace_bundle_v1(&fixtures.valid_completed_bundle)?;
    validate_acip_trace_bundle_v1(&fixtures.valid_refused_bundle)?;
    validate_acip_trace_bundle_v1(&fixtures.valid_failed_bundle)?;
    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        validate_negative_result(
            validate_acip_trace_bundle_v1_value(&case.bundle).map(|_| ()),
            &case.expected_error_substring,
        )?;
    }
    Ok(())
}

fn validate_invocation_constraints(constraints: &AcipInvocationConstraintsV1) -> Result<()> {
    if constraints.policy_refs.is_empty() {
        return Err(anyhow!("constraints.policy_refs must not be empty"));
    }
    if constraints.policy_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!(
            "constraints.policy_refs exceeds bounded list length"
        ));
    }
    for policy_ref in &constraints.policy_refs {
        validate_repo_relative_ref(policy_ref, "constraints.policy_refs[]")?;
    }
    if constraints.required_capabilities.is_empty() {
        return Err(anyhow!(
            "constraints.required_capabilities must not be empty"
        ));
    }
    if constraints.required_capabilities.len() > MAX_LIST_LEN {
        return Err(anyhow!(
            "constraints.required_capabilities exceeds bounded list length"
        ));
    }
    let mut seen_capabilities = BTreeSet::new();
    for capability in &constraints.required_capabilities {
        validate_id(capability, "constraints.required_capabilities[]")?;
        if !seen_capabilities.insert(capability.clone()) {
            return Err(anyhow!(
                "constraints.required_capabilities contains duplicate '{}'",
                capability
            ));
        }
    }
    let mut seen_prohibited = BTreeSet::new();
    for action in &constraints.prohibited_actions {
        validate_id(action, "constraints.prohibited_actions[]")?;
        if !seen_prohibited.insert(action.clone()) {
            return Err(anyhow!(
                "constraints.prohibited_actions contains duplicate '{}'",
                action
            ));
        }
    }
    Ok(())
}

fn validate_invocation_authority_alignment(contract: &AcipInvocationContractV1) -> Result<()> {
    let allows = |action: &str| {
        contract
            .authority_scope
            .allowed_actions
            .iter()
            .any(|allowed| allowed == action)
    };

    match contract.intent {
        AcipIntentV1::InvocationSetup | AcipIntentV1::CodingRequest => {
            if !allows("invoke") {
                return Err(anyhow!(
                    "invocation contract intent '{}' requires authority_scope.allowed_actions to include 'invoke'",
                    contract.intent.as_str()
                ));
            }
        }
        AcipIntentV1::ReviewRequest => {
            if !allows("review") {
                return Err(anyhow!(
                    "invocation contract intent 'review_request' requires authority_scope.allowed_actions to include 'review'"
                ));
            }
        }
        AcipIntentV1::Delegation => {
            if !allows("delegate") {
                return Err(anyhow!(
                    "invocation contract intent 'delegation' requires authority_scope.allowed_actions to include 'delegate'"
                ));
            }
            if !contract.authority_scope.delegation_permitted {
                return Err(anyhow!(
                    "invocation contract intent 'delegation' requires authority_scope.delegation_permitted"
                ));
            }
        }
        AcipIntentV1::Conversation | AcipIntentV1::Consultation | AcipIntentV1::Negotiation => {
            unreachable!("validated above")
        }
    }

    Ok(())
}

fn validate_expected_outputs(outputs: &[AcipExpectedOutputV1]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for output in outputs {
        validate_id(&output.output_id, "expected_outputs[].output_id")?;
        validate_id(&output.output_kind, "expected_outputs[].output_kind")?;
        validate_id(&output.artifact_role, "expected_outputs[].artifact_role")?;
        if let Some(schema_ref) = &output.schema_ref {
            validate_repo_relative_ref(schema_ref, "expected_outputs[].schema_ref")?;
        }
        if !seen.insert(output.output_id.clone()) {
            return Err(anyhow!(
                "expected_outputs contains duplicate output_id '{}'",
                output.output_id
            ));
        }
    }
    Ok(())
}

fn validate_stop_policy(stop_policy: &AcipInvocationStopPolicyV1) -> Result<()> {
    if stop_policy.max_turns == 0 {
        return Err(anyhow!("stop_policy.max_turns must be positive"));
    }
    if stop_policy.max_output_artifacts == 0 {
        return Err(anyhow!("stop_policy.max_output_artifacts must be positive"));
    }
    validate_non_empty(
        &stop_policy.completion_condition,
        "stop_policy.completion_condition",
    )?;
    Ok(())
}

fn validate_response_channel(channel: &AcipResponseChannelV1) -> Result<()> {
    match channel.kind {
        AcipResponseChannelKindV1::DirectReply => {
            validate_id(&channel.channel_ref, "response_channel.channel_ref")
        }
        AcipResponseChannelKindV1::ArtifactReply => {
            validate_repo_relative_ref(&channel.channel_ref, "response_channel.channel_ref")
        }
    }
}

fn validate_review_independence_policy(policy: &AcipReviewIndependencePolicyV1) -> Result<()> {
    validate_id(
        &policy.writer_session_id,
        "independence_policy.writer_session_id",
    )?;
    validate_id(
        &policy.writer_model_ref,
        "independence_policy.writer_model_ref",
    )?;
    validate_id(
        &policy.reviewer_session_id,
        "independence_policy.reviewer_session_id",
    )?;
    validate_id(
        &policy.reviewer_model_ref,
        "independence_policy.reviewer_model_ref",
    )?;
    if policy.forbid_same_session && policy.writer_session_id == policy.reviewer_session_id {
        return Err(anyhow!(
            "review invocation independence policy forbids same-session review"
        ));
    }
    if policy.forbid_same_model_ref && policy.writer_model_ref == policy.reviewer_model_ref {
        return Err(anyhow!(
            "review invocation independence policy forbids same-model review"
        ));
    }
    Ok(())
}

fn validate_review_disposition_contract(contract: &AcipReviewDispositionContractV1) -> Result<()> {
    if contract.allowed_dispositions.is_empty() {
        return Err(anyhow!(
            "review disposition contract requires allowed_dispositions"
        ));
    }
    let mut seen = BTreeSet::new();
    for disposition in &contract.allowed_dispositions {
        if !seen.insert(disposition.clone()) {
            return Err(anyhow!(
                "review disposition contract contains duplicate allowed_dispositions entries"
            ));
        }
    }
    for required in [
        AcipReviewDispositionV1::Blessed,
        AcipReviewDispositionV1::Blocked,
        AcipReviewDispositionV1::NonProving,
        AcipReviewDispositionV1::Skipped,
    ] {
        if !seen.contains(&required) {
            return Err(anyhow!(
                "review disposition contract must include every canonical review disposition"
            ));
        }
    }
    if contract.blessed_handoff != AcipReviewHandoffOutcomeV1::AllowPrFinish {
        return Err(anyhow!(
            "review disposition contract must map blessed review to allow_pr_finish"
        ));
    }
    if contract.blocked_handoff == AcipReviewHandoffOutcomeV1::AllowPrFinish
        || contract.non_proving_handoff == AcipReviewHandoffOutcomeV1::AllowPrFinish
        || contract.skipped_handoff == AcipReviewHandoffOutcomeV1::AllowPrFinish
    {
        return Err(anyhow!(
            "only blessed review may map to allow_pr_finish in the disposition contract"
        ));
    }
    Ok(())
}

fn validate_gate_decision_ref(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let Some(suffix) = trimmed
        .strip_prefix("gate.")
        .or_else(|| trimmed.strip_prefix("gate:"))
    else {
        return Err(anyhow!(
            "{field} must link to a Freedom Gate decision token"
        ));
    };
    if suffix.is_empty() {
        return Err(anyhow!(
            "{field} must include a Freedom Gate decision identifier"
        ));
    }
    if !suffix
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must use a stable Freedom Gate decision token"
        ));
    }
    Ok(())
}

impl AcipIntentV1 {
    fn as_str(&self) -> &'static str {
        match self {
            AcipIntentV1::Conversation => "conversation",
            AcipIntentV1::Consultation => "consultation",
            AcipIntentV1::InvocationSetup => "invocation_setup",
            AcipIntentV1::ReviewRequest => "review_request",
            AcipIntentV1::CodingRequest => "coding_request",
            AcipIntentV1::Delegation => "delegation",
            AcipIntentV1::Negotiation => "negotiation",
        }
    }
}

fn validate_invocation_status_consistency(event: &AcipInvocationEventV1) -> Result<()> {
    match event.status {
        AcipInvocationStatusV1::Requested => {
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "requested invocation event must not carry output_refs"
                ));
            }
            if event.refusal_code.is_some() || event.failure_code.is_some() {
                return Err(anyhow!(
                    "requested invocation event must not carry refusal_code or failure_code"
                ));
            }
        }
        AcipInvocationStatusV1::Completed => {
            if event.output_refs.is_empty() {
                return Err(anyhow!("completed invocation event must carry output_refs"));
            }
            if event.refusal_code.is_some() || event.failure_code.is_some() {
                return Err(anyhow!(
                    "completed invocation event must not carry refusal_code or failure_code"
                ));
            }
        }
        AcipInvocationStatusV1::Refused => {
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "refused invocation event must not carry output_refs"
                ));
            }
            if event.refusal_code.is_none() {
                return Err(anyhow!("refused invocation event must carry refusal_code"));
            }
            if event.failure_code.is_some() {
                return Err(anyhow!(
                    "refused invocation event must not carry failure_code"
                ));
            }
            if event.evidence_refs.is_empty() {
                return Err(anyhow!("refused invocation event must carry evidence_refs"));
            }
        }
        AcipInvocationStatusV1::Failed => {
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "failed invocation event must not carry output_refs"
                ));
            }
            if event.failure_code.is_none() {
                return Err(anyhow!("failed invocation event must carry failure_code"));
            }
            if event.refusal_code.is_some() {
                return Err(anyhow!(
                    "failed invocation event must not carry refusal_code"
                ));
            }
            if event.evidence_refs.is_empty() {
                return Err(anyhow!("failed invocation event must carry evidence_refs"));
            }
        }
        AcipInvocationStatusV1::Partial => {
            if event.output_refs.is_empty() {
                return Err(anyhow!("partial invocation event must carry output_refs"));
            }
            if event.refusal_code.is_some() || event.failure_code.is_some() {
                return Err(anyhow!(
                    "partial invocation event must not carry refusal_code or failure_code"
                ));
            }
        }
    }
    Ok(())
}

fn validate_negative_case(value: JsonValue, expected: &str, is_message: bool) -> Result<()> {
    validate_non_empty(expected, "expected_error_substring")?;
    let result = if is_message {
        validate_acip_message_envelope_v1_value(&value).map(|_| ())
    } else {
        validate_acip_conversation_envelope_v1_value(&value).map(|_| ())
    };
    match result {
        Ok(()) => Err(anyhow!("negative case unexpectedly validated")),
        Err(error) => {
            let text = format!("{error:#}");
            if !text.contains(expected) {
                return Err(anyhow!(
                    "negative case expected error containing '{}', found '{}'",
                    expected,
                    text
                ));
            }
            Ok(())
        }
    }
}

fn validate_negative_result(result: Result<()>, expected: &str) -> Result<()> {
    match result {
        Ok(()) => Err(anyhow!("negative case unexpectedly validated")),
        Err(error) => {
            let text = format!("{error:#}");
            if !text.contains(expected) {
                return Err(anyhow!(
                    "negative case expected error containing '{}', found '{}'",
                    expected,
                    text
                ));
            }
            Ok(())
        }
    }
}

fn validate_negative_case_name(value: &str, field: &str) -> Result<()> {
    validate_id(value, field)
}

fn validate_id(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.starts_with('/')
        || trimmed.starts_with('\\')
        || trimmed.contains('\\')
        || trimmed.contains(':')
        || trimmed.contains("..")
    {
        return Err(anyhow!("{field} must be a stable identifier"));
    }
    Ok(())
}

fn validate_non_empty(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn validate_timestamp(value: &str, field: &str) -> Result<()> {
    validate_non_empty(value, field)?;
    let parsed = DateTime::parse_from_rfc3339(value)
        .with_context(|| format!("{field} must be an RFC3339-style UTC timestamp"))?;
    if parsed.offset().local_minus_utc() != 0 {
        return Err(anyhow!("{field} must be a UTC timestamp ending in Z"));
    }
    Ok(())
}

fn validate_sha256(value: &str, field: &str) -> Result<()> {
    if value.len() != 64 || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character hexadecimal sha256"));
    }
    Ok(())
}

fn validate_repo_relative_ref(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.starts_with('/')
        || trimmed.starts_with('\\')
        || trimmed.contains('\\')
        || trimmed.contains(':')
    {
        return Err(anyhow!("{field} must be a repository-relative path"));
    }
    for component in Path::new(trimmed).components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            _ => {
                return Err(anyhow!(
                    "{field} must be a repository-relative path without traversal"
                ))
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acip_fixture_set_v1_contract_is_stable() {
        let schema = acip_message_envelope_v1_schema_json().expect("schema");
        assert!(schema.contains("\"message_id\""));
        assert!(schema.contains("\"conversation_id\""));
        assert!(schema.contains("\"authority_scope\""));
    }

    #[test]
    fn acip_fixture_set_v1_matches_required_modes_and_negative_cases() {
        let fixtures = acip_fixture_set_v1();
        validate_acip_fixture_set_v1(&fixtures).expect("fixtures should validate");

        let valid_names: Vec<_> = fixtures
            .valid_messages
            .iter()
            .map(|fixture| fixture.name.as_str())
            .collect();
        assert_eq!(
            valid_names,
            vec![
                "conversation",
                "consultation",
                "invocation_setup",
                "review_request",
                "coding_request",
                "delegation",
                "negotiation",
                "coding_agent_handoff",
                "operator_request",
                "broadcast"
            ]
        );

        let invalid_names: Vec<_> = fixtures
            .invalid_messages
            .iter()
            .map(|case| case.name.as_str())
            .collect();
        assert_eq!(
            invalid_names,
            vec![
                "identity_drift",
                "missing_recipient",
                "hidden_invocation",
                "malformed_payload_refs",
                "unsupported_visibility",
                "raw_local_path_refs",
                "authority_escalation"
            ]
        );
    }

    #[test]
    fn acip_conversation_envelope_rejects_stale_ordering() {
        let mut conversation = acip_fixture_set_v1().valid_conversation;
        conversation.messages.swap(0, 1);
        let error = validate_acip_conversation_envelope_v1(&conversation)
            .expect_err("ordering should fail");
        assert!(error
            .to_string()
            .contains("strictly increasing monotonic_order"));
    }

    #[test]
    fn acip_message_envelope_rejects_host_path_leakage() {
        let mut message = acip_fixture_set_v1().valid_messages[0].message.clone();
        message.artifact_refs = vec!["/Users/daniel/private/trace.json".to_string()];
        let error =
            validate_acip_message_envelope_v1(&message).expect_err("absolute path should fail");
        assert!(error
            .to_string()
            .contains("artifact_refs[] must be a repository-relative path"));
    }

    #[test]
    fn acip_fixture_schema_json_is_available() {
        let schema = acip_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(schema.contains("\"valid_messages\""));
        assert!(schema.contains("\"invalid_conversations\""));
    }

    #[test]
    fn acip_invocation_contract_and_event_schemas_are_available() {
        let contract_schema = acip_invocation_contract_v1_schema_json().expect("contract schema");
        let event_schema = acip_invocation_event_v1_schema_json().expect("event schema");
        assert!(contract_schema.contains("\"decision_event_ref\""));
        assert!(event_schema.contains("\"stop_reason\""));
    }

    #[test]
    fn acip_conformance_report_schema_and_report_are_available() {
        let schema = acip_conformance_report_v1_schema_json().expect("conformance schema");
        assert!(schema.contains("\"valid_fixture_classes\""));
        assert!(schema.contains("\"negative_fixture_classes\""));

        let report = acip_conformance_report_v1();
        validate_acip_conformance_report_v1(&report).expect("conformance report should validate");
        assert!(report
            .valid_fixture_classes
            .iter()
            .any(|class| class.fixture_name == "coding_agent_handoff"));
        assert!(report
            .valid_fixture_classes
            .iter()
            .any(|class| class.fixture_name == "shared_conversation_thread"));
        assert!(report
            .valid_fixture_classes
            .iter()
            .any(|class| class.fixture_name == "governed_invocation_contract"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "missing_gate_rejects_governed_invocation"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "ambiguous_stop_policy_rejected"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "unsafe_input_refs_rejected"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "status_refusal_inconsistency_rejected"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "output_contract_mismatch_rejected"));
    }

    #[test]
    fn acip_conformance_report_requires_full_cross_surface_matrix() {
        let mut report = acip_conformance_report_v1();
        report
            .valid_fixture_classes
            .retain(|class| class.fixture_name != "governed_invocation_contract");
        let valid_error = validate_acip_conformance_report_v1(&report)
            .expect_err("missing invocation proof fixture should fail");
        assert!(valid_error
            .to_string()
            .contains("missing required valid fixture 'governed_invocation_contract'"));

        let mut report = acip_conformance_report_v1();
        report
            .negative_fixture_classes
            .retain(|class| class.case_name != "output_contract_mismatch_rejected");
        let negative_error = validate_acip_conformance_report_v1(&report)
            .expect_err("missing invocation negative proof should fail");
        assert!(negative_error
            .to_string()
            .contains("missing required negative fixture 'output_contract_mismatch_rejected'"));
    }

    #[test]
    fn acip_review_specialization_schemas_and_fixtures_are_available() {
        let contract_schema = acip_review_invocation_v1_schema_json().expect("review schema");
        let outcome_schema = acip_review_outcome_v1_schema_json().expect("outcome schema");
        let fixture_schema = acip_review_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(contract_schema.contains("\"srp_ref\""));
        assert!(outcome_schema.contains("\"handoff_outcome\""));
        assert!(fixture_schema.contains("\"valid_blessed_outcome\""));

        let fixtures = acip_review_fixture_set_v1();
        validate_acip_review_fixture_set_v1(&fixtures).expect("review fixtures should validate");
        assert_eq!(fixtures.negative_cases.len(), 6);
    }

    #[test]
    fn acip_review_specialization_rejects_self_review_and_merge_authority_gaps() {
        let fixtures = acip_review_fixture_set_v1();

        let same_session = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "same_session_self_review_rejected")
            .expect("same session case");
        let same_session_contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(same_session.contract.clone()).expect("contract");
        let same_session_error =
            validate_acip_review_invocation_contract_v1(&same_session_contract)
                .expect_err("same-session review should fail");
        assert!(same_session_error
            .to_string()
            .contains("forbids same-session review"));

        let same_model = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "same_model_self_blessing_rejected")
            .expect("same model case");
        let same_model_contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(same_model.contract.clone()).expect("contract");
        let same_model_error = validate_acip_review_invocation_contract_v1(&same_model_contract)
            .expect_err("same-model review should fail");
        assert!(same_model_error
            .to_string()
            .contains("forbids same-model review"));

        let merge_gap = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "merge_authority_gap_rejected")
            .expect("merge gap case");
        let merge_gap_contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(merge_gap.contract.clone()).expect("contract");
        let merge_gap_error = validate_acip_review_invocation_contract_v1(&merge_gap_contract)
            .expect_err("missing merge prohibition should fail");
        assert!(merge_gap_error
            .to_string()
            .contains("requires constraints.prohibited_actions to include 'merge'"));
    }

    #[test]
    fn acip_review_specialization_rejects_invalid_outcome_contract_mappings() {
        let fixtures = acip_review_fixture_set_v1();

        let blocked_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "blocked_outcome_requires_findings_ref")
            .expect("blocked outcome case");
        let contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(blocked_case.contract.clone()).expect("contract");
        let outcome: AcipReviewOutcomeV1 =
            serde_json::from_value(blocked_case.outcome.clone().expect("outcome"))
                .expect("outcome");
        let blocked_error = validate_acip_review_outcome_v1(&contract, &outcome)
            .expect_err("blocked review without findings should fail");
        assert!(blocked_error
            .to_string()
            .contains("blocked review outcome must carry findings_ref"));

        let non_blessed_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "non_blessed_outcome_cannot_allow_pr_finish")
            .expect("non blessed case");
        let contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(non_blessed_case.contract.clone()).expect("contract");
        let outcome: AcipReviewOutcomeV1 =
            serde_json::from_value(non_blessed_case.outcome.clone().expect("outcome"))
                .expect("outcome");
        let handoff_error = validate_acip_review_outcome_v1(&contract, &outcome)
            .expect_err("non-blessed review should not allow PR finish");
        assert!(
            handoff_error
                .to_string()
                .contains("review outcome handoff must match the disposition contract mapping")
                || handoff_error
                    .to_string()
                    .contains("only blessed review outcomes may allow PR finish")
        );
    }

    #[test]
    fn acip_review_specialization_binds_wrapper_refs_to_invocation_contract() {
        let mut contract = sample_review_invocation_contract();
        contract.review_packet_ref = "runtime/comms/review/drifted_packet.json".to_string();
        let packet_error = validate_acip_review_invocation_contract_v1(&contract)
            .expect_err("drifted review packet ref should fail");
        assert!(packet_error
            .to_string()
            .contains("invocation.input_refs to include review_packet_ref"));

        let contract = sample_review_invocation_contract();
        let mut outcome = sample_review_outcome(
            &contract,
            AcipReviewDispositionV1::Blessed,
            AcipReviewHandoffOutcomeV1::AllowPrFinish,
            Some("runtime/comms/review/findings/clean_review.json".to_string()),
            vec!["runtime/comms/review/residual_risk/none.json".to_string()],
            true,
        );
        outcome.gate_result_ref = "runtime/comms/review/not_the_gate.json".to_string();
        let output_error = validate_acip_review_outcome_v1(&contract, &outcome)
            .expect_err("drifted gate result ref should fail");
        assert!(output_error
            .to_string()
            .contains("gate_result_ref must match the declared gate_result output contract"));
    }

    #[test]
    fn acip_coding_specialization_schemas_and_fixtures_are_available() {
        let contract_schema = acip_coding_invocation_v1_schema_json().expect("coding schema");
        let outcome_schema = acip_coding_outcome_v1_schema_json().expect("outcome schema");
        let fixture_schema = acip_coding_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(contract_schema.contains("\"provider_lane\""));
        assert!(outcome_schema.contains("\"review_handoff_ref\""));
        assert!(fixture_schema.contains("\"valid_codex_outcome\""));

        let fixtures = acip_coding_fixture_set_v1();
        validate_acip_coding_fixture_set_v1(&fixtures).expect("coding fixtures should validate");
        assert_eq!(fixtures.negative_cases.len(), 12);
    }

    #[test]
    fn acip_coding_specialization_rejects_non_codex_worktree_and_review_bypass() {
        let fixtures = acip_coding_fixture_set_v1();

        let non_codex = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "non_codex_worktree_edit_rejected")
            .expect("non codex case");
        let non_codex_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(non_codex.contract.clone()).expect("contract");
        let non_codex_error = validate_acip_coding_invocation_contract_v1(&non_codex_contract)
            .expect_err("non-codex worktree edit should fail");
        assert!(non_codex_error
            .to_string()
            .contains("only codex_issue_worktree lane may use execution_mode 'worktree_edit'"));

        let review_schema = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "review_schema_drift_rejected")
            .expect("review schema case");
        let review_schema_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(review_schema.contract.clone()).expect("contract");
        let review_schema_error =
            validate_acip_coding_invocation_contract_v1(&review_schema_contract)
                .expect_err("review schema drift should fail");
        assert!(review_schema_error
            .to_string()
            .contains("must route to review schema 'acip.review.invocation.v1'"));

        let capability_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "proposal_lane_code_edit_capability_rejected")
            .expect("capability case");
        let capability_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(capability_case.contract.clone()).expect("contract");
        let capability_error = validate_acip_coding_invocation_contract_v1(&capability_contract)
            .expect_err("proposal-only lane should not claim code_edit");
        assert!(capability_error
            .to_string()
            .contains("proposal-only coding lanes must not claim 'code_edit' capability"));

        let chatgpt_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "chatgpt_structured_proposal_rejected")
            .expect("chatgpt case");
        let chatgpt_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(chatgpt_case.contract.clone()).expect("contract");
        let chatgpt_error = validate_acip_coding_invocation_contract_v1(&chatgpt_contract)
            .expect_err("chatgpt structured proposal should fail");
        assert!(chatgpt_error
            .to_string()
            .contains("chatgpt_api lane does not permit execution_mode 'structured_proposal'"));

        let worktree_flag_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "proposal_lane_worktree_flag_rejected")
            .expect("worktree flag case");
        let worktree_flag_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(worktree_flag_case.contract.clone()).expect("contract");
        let worktree_flag_error =
            validate_acip_coding_invocation_contract_v1(&worktree_flag_contract)
                .expect_err("proposal-only lane should not request issue worktree");
        assert!(worktree_flag_error
            .to_string()
            .contains("proposal-only coding lanes must set issue_worktree_required to false"));
    }

    #[test]
    fn acip_coding_specialization_binds_output_contract_and_writer_identity() {
        let patch_contract = sample_coding_invocation_contract(
            AcipCodingProviderLaneV1::ChatgptApi,
            AcipCodingExecutionModeV1::UnappliedPatch,
        );
        let mut patch_outcome = sample_coding_outcome(&patch_contract);
        patch_outcome.primary_output_ref = "runtime/comms/coding/proposal.json".to_string();
        let patch_error = validate_acip_coding_outcome_v1(&patch_contract, &patch_outcome)
            .expect_err("patch output drift should fail");
        assert!(patch_error
            .to_string()
            .contains("must match the declared patch_diff output contract"));

        let worktree_contract = sample_coding_invocation_contract(
            AcipCodingProviderLaneV1::CodexIssueWorktree,
            AcipCodingExecutionModeV1::WorktreeEdit,
        );
        let mut worktree_outcome = sample_coding_outcome(&worktree_contract);
        worktree_outcome.writer_session_id = "drifted-session".to_string();
        let identity_error = validate_acip_coding_outcome_v1(&worktree_contract, &worktree_outcome)
            .expect_err("writer identity drift should fail");
        assert!(identity_error
            .to_string()
            .contains("must preserve the writer_session_id declared by the approval policy"));

        let mut validation_outcome = sample_coding_outcome(&worktree_contract);
        validation_outcome.validation_result_refs =
            vec!["runtime/comms/coding/not_validation.json".to_string()];
        let validation_error =
            validate_acip_coding_outcome_v1(&worktree_contract, &validation_outcome)
                .expect_err("validation summary drift should fail");
        assert!(validation_error.to_string().contains(
            "validation_result_refs must include the declared validation_summary output contract"
        ));

        let mut stop_contract = sample_coding_invocation_contract(
            AcipCodingProviderLaneV1::CodexIssueWorktree,
            AcipCodingExecutionModeV1::WorktreeEdit,
        );
        stop_contract.invocation.stop_policy.stop_on_failure = false;
        let stop_error = validate_acip_coding_invocation_contract_v1(&stop_contract)
            .expect_err("stop policy drift should fail");
        assert!(stop_error
            .to_string()
            .contains("requires stop_policy.stop_on_failure to be true"));
    }

    #[test]
    fn acip_trace_bundle_schemas_and_fixtures_are_available() {
        let bundle_schema = acip_trace_bundle_v1_schema_json().expect("bundle schema");
        assert!(bundle_schema.contains("AcipTraceBundleV1"));
        let fixture_schema = acip_trace_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(fixture_schema.contains("AcipTraceFixtureSetV1"));

        let fixtures = acip_trace_fixture_set_v1();
        validate_acip_trace_fixture_set_v1(&fixtures).expect("trace fixtures should validate");
        assert_eq!(fixtures.negative_cases.len(), 5);
    }

    #[test]
    fn acip_trace_bundle_requires_terminal_mapping_and_replay_posture() {
        let completed = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        validate_acip_trace_bundle_v1(&completed).expect("completed bundle should validate");

        let refused = sample_trace_bundle(AcipInvocationStatusV1::Refused);
        validate_acip_trace_bundle_v1(&refused).expect("refused bundle should validate");

        let mut invalid = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        invalid.trace_events[3].event_kind = AcipTraceEventKindV1::InvocationFailed;
        let terminal_error =
            validate_acip_trace_bundle_v1(&invalid).expect_err("terminal mismatch should fail");
        assert!(terminal_error
            .to_string()
            .contains("invocation_failed trace event must carry invocation_status 'failed'"));

        let mut duplicate_terminal = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        duplicate_terminal.trace_events.push(AcipTraceEventV1 {
            event_id: "trace-0005".to_string(),
            conversation_id: duplicate_terminal.conversation_id.clone(),
            invocation_id: Some("invoke-0001".to_string()),
            event_kind: AcipTraceEventKindV1::InvocationCompleted,
            source_message_id: Some("msg-review-0001".to_string()),
            contract_ref: Some(
                "runtime/comms/invocation/contracts/review_request.json".to_string(),
            ),
            decision_event_ref: Some("gate.review-0001".to_string()),
            invocation_status: Some(AcipInvocationStatusV1::Completed),
            output_refs: vec!["runtime/comms/invocation/review_report.json".to_string()],
            evidence_refs: vec![
                "runtime/comms/invocation/evidence/completed_trace.json".to_string()
            ],
            summary: "Duplicate terminal event for regression coverage.".to_string(),
            requires_redaction: false,
        });
        let duplicate_error = validate_acip_trace_bundle_v1(&duplicate_terminal)
            .expect_err("duplicate terminal event should fail");
        assert!(duplicate_error
            .to_string()
            .contains("trace bundle must not contain duplicate event kind 'invocation_completed'"));

        let mut drift_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        drift_bundle.trace_events[2].decision_event_ref = Some("gate.review-drifted".to_string());
        let drift_error =
            validate_acip_trace_bundle_v1(&drift_bundle).expect_err("decision drift should fail");
        assert!(drift_error.to_string().contains(
            "trace bundle must preserve one canonical decision_event_ref across non-message events"
        ));

        let mut missing_contract_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        missing_contract_bundle.trace_events[2].contract_ref = None;
        let missing_contract_error = validate_acip_trace_bundle_v1(&missing_contract_bundle)
            .expect_err("missing contract ref should fail closed");
        assert!(missing_contract_error.to_string().contains(
            "decision_recorded trace event must carry invocation_id, contract_ref, and decision_event_ref"
        ));

        let mut order_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        order_bundle.trace_events.swap(1, 2);
        let order_error = validate_acip_trace_bundle_v1(&order_bundle)
            .expect_err("out-of-order trace event should fail");
        assert!(order_error
            .to_string()
            .contains("trace bundle must preserve canonical event order"));

        let mut replay_invalid = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        replay_invalid.replay_contract.deterministic_redaction_views = false;
        let replay_error = validate_acip_trace_bundle_v1(&replay_invalid)
            .expect_err("non-deterministic redaction view should fail");
        assert!(replay_error
            .to_string()
            .contains("ACIP replay contract requires deterministic_redaction_views"));
    }

    #[test]
    fn acip_trace_bundle_redaction_views_fail_closed_on_leakage() {
        let mut bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        bundle.audience_views[2]
            .visible_artifact_refs
            .push("runtime/comms/private_state/raw_args.json".to_string());
        let reviewer_error =
            validate_acip_trace_bundle_v1(&bundle).expect_err("reviewer leak should fail");
        assert!(reviewer_error.to_string().contains(
            "visible_artifact_refs[] must not expose unredacted trace ref 'private_state'"
        ));

        let mut narrative_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        narrative_bundle.audience_views[2].narrative_ref =
            "runtime/comms/trace/private_state_dump.json".to_string();
        let narrative_error = validate_acip_trace_bundle_v1(&narrative_bundle)
            .expect_err("narrative leak should fail");
        assert!(narrative_error
            .to_string()
            .contains("narrative_ref must not expose unredacted trace ref 'private_state'"));

        let mut summary_bundle = sample_trace_bundle(AcipInvocationStatusV1::Failed);
        summary_bundle.trace_events[3].summary =
            "Failure packet contained secret token and raw operator prompt.".to_string();
        let summary_error = validate_acip_trace_bundle_v1(&summary_bundle)
            .expect_err("protected summary leak should fail");
        assert!(summary_error
            .to_string()
            .contains("summary must not leak protected trace content 'secret'"));

        let mut path_bundle = sample_trace_bundle(AcipInvocationStatusV1::Failed);
        path_bundle.trace_events[3].summary =
            "Failure packet copied from /var/folders/tmp and raw tool arguments for replay."
                .to_string();
        let path_error = validate_acip_trace_bundle_v1(&path_bundle)
            .expect_err("workstation path leak should fail");
        assert!(path_error
            .to_string()
            .contains("summary must not leak protected trace content 'raw tool arguments'"));
    }

    #[test]
    fn acip_invocation_fixture_set_matches_required_contract_and_negative_cases() {
        let fixtures = acip_invocation_fixture_set_v1();
        validate_acip_invocation_fixture_set_v1(&fixtures)
            .expect("invocation fixtures should validate");
        assert_eq!(fixtures.negative_cases.len(), 5);
    }

    #[test]
    fn acip_invocation_requires_gate_linkage_for_governed_work() {
        let mut value = serde_json::to_value(sample_invocation_contract()).expect("contract json");
        value
            .as_object_mut()
            .expect("object")
            .remove("decision_event_ref");
        let error = validate_acip_invocation_contract_v1_value(&value)
            .expect_err("missing gate decision should fail");
        assert!(format!("{error:#}").contains("missing field `decision_event_ref`"));
    }

    #[test]
    fn acip_invocation_event_rejects_output_contract_mismatch() {
        let contract = sample_invocation_contract();
        let event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/operator_summary.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        let error = validate_acip_invocation_event_against_contract(&contract, &event)
            .expect_err("missing outputs should fail");
        assert!(error
            .to_string()
            .contains("completed invocation must satisfy declared required output contracts"));
    }

    #[test]
    fn acip_invocation_event_preserves_contract_input_and_trace_binding() {
        let contract = sample_invocation_contract();
        let mut event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        event.input_refs = vec!["runtime/comms/invocation/drifted_input.json".to_string()];
        let input_error = validate_acip_invocation_event_against_contract(&contract, &event)
            .expect_err("drifted input refs should fail");
        assert!(input_error
            .to_string()
            .contains("invocation event must preserve the contract input_refs"));

        let mut trace_event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        trace_event.trace_requirement = AcipTraceRequirementV1::Summary;
        let trace_error = validate_acip_invocation_event_against_contract(&contract, &trace_event)
            .expect_err("trace drift should fail");
        assert!(trace_error
            .to_string()
            .contains("invocation event must preserve the contract trace_requirement"));
    }

    #[test]
    fn acip_invocation_event_respects_max_output_artifact_bound() {
        let contract = sample_invocation_contract();
        let event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec![
                "runtime/comms/invocation/review_report.json".to_string(),
                "runtime/comms/invocation/review_summary.json".to_string(),
                "runtime/comms/invocation/overflow.json".to_string(),
            ],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        let error = validate_acip_invocation_event_against_contract(&contract, &event)
            .expect_err("too many outputs should fail");
        assert!(error
            .to_string()
            .contains("invocation event exceeds stop_policy.max_output_artifacts"));
    }

    #[test]
    fn acip_invocation_refusal_requires_evidence_and_no_outputs() {
        let contract = sample_invocation_contract();
        let event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Refused,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "policy_refusal".to_string(),
            Some("operator_review_required".to_string()),
            None,
            Vec::new(),
        );
        let error = validate_acip_invocation_event_v1(&event)
            .expect_err("refused invocation with outputs and no evidence should fail");
        assert!(error
            .to_string()
            .contains("refused invocation event must not carry output_refs"));
    }

    #[test]
    fn acip_invocation_contract_and_event_reject_unknown_fields() {
        let mut contract_value =
            serde_json::to_value(sample_invocation_contract()).expect("contract json");
        contract_value["unexpected_field"] = json!("surprise");
        let contract_error = validate_acip_invocation_contract_v1_value(&contract_value)
            .expect_err("unknown contract field should fail");
        assert!(format!("{contract_error:#}").contains("unknown field"));

        let contract = sample_invocation_contract();
        let mut event_value = serde_json::to_value(sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        ))
        .expect("event json");
        event_value["unexpected_field"] = json!("surprise");
        let event_error = validate_acip_invocation_event_v1_value(&event_value)
            .expect_err("unknown event field should fail");
        assert!(format!("{event_error:#}").contains("unknown field"));
    }

    #[test]
    fn acip_invocation_contract_requires_authority_scope_alignment() {
        let mut delegation_contract = sample_invocation_contract();
        delegation_contract.intent = AcipIntentV1::Delegation;
        delegation_contract.authority_scope.allowed_actions = vec!["review".to_string()];
        delegation_contract.authority_scope.delegation_permitted = false;
        let delegation_error = validate_acip_invocation_contract_v1(&delegation_contract)
            .expect_err("delegation intent without delegation authority should fail");
        assert!(delegation_error
            .to_string()
            .contains("requires authority_scope.allowed_actions to include 'delegate'"));

        let mut coding_contract = sample_invocation_contract();
        coding_contract.intent = AcipIntentV1::CodingRequest;
        coding_contract.authority_scope.allowed_actions = vec!["review".to_string()];
        let coding_error = validate_acip_invocation_contract_v1(&coding_contract)
            .expect_err("coding intent without invoke authority should fail");
        assert!(coding_error
            .to_string()
            .contains("requires authority_scope.allowed_actions to include 'invoke'"));
    }

    #[test]
    fn acip_message_envelope_rejects_unknown_fields() {
        let mut value =
            serde_json::to_value(acip_fixture_set_v1().valid_messages[0].message.clone())
                .expect("json");
        value["unexpected_field"] = json!("surprise");
        let error =
            validate_acip_message_envelope_v1_value(&value).expect_err("unknown field should fail");
        assert!(format!("{error:#}").contains("unknown field"));
    }

    #[test]
    fn acip_message_envelope_rejects_invalid_timestamp_shape() {
        let mut message = acip_fixture_set_v1().valid_messages[0].message.clone();
        message.timestamp_utc = "not-a-timeTstill-badZ".to_string();
        let error = validate_acip_message_envelope_v1(&message).expect_err("timestamp should fail");
        assert!(error.to_string().contains("RFC3339-style UTC timestamp"));
    }

    #[test]
    fn acip_message_envelope_rejects_hidden_governed_authority_and_escalation() {
        let fixtures = acip_fixture_set_v1();
        let hidden = fixtures
            .invalid_messages
            .iter()
            .find(|case| case.name == "hidden_invocation")
            .expect("hidden invocation case");
        let hidden_error = validate_acip_message_envelope_v1_value(&hidden.value)
            .expect_err("hidden invocation should fail");
        assert!(hidden_error
            .to_string()
            .contains("message intent 'conversation' must not claim authority action 'invoke'"));

        let escalation = fixtures
            .invalid_messages
            .iter()
            .find(|case| case.name == "authority_escalation")
            .expect("authority escalation case");
        let escalation_error = validate_acip_message_envelope_v1_value(&escalation.value)
            .expect_err("authority escalation should fail");
        assert!(escalation_error
            .to_string()
            .contains("message intent 'consultation' must not claim authority action 'delegate'"));
    }
}
