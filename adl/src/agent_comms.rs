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
const ACIP_PROOF_DEMO_SCHEMA_VERSION: &str = "acip.proof.demo.v1";
const ACIP_A2A_ADAPTER_SCHEMA_VERSION: &str = "acip.a2a.adapter.v1";
const ACIP_A2A_FIXTURE_SCHEMA_VERSION: &str = "acip.a2a.fixture.v1";
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipProofClassificationV1 {
    Proving,
    NonProving,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipA2aTrustClassV1 {
    Naked,
    Guest,
    Citizen,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipA2aTransportBoundaryStatusV1 {
    DeferredUntilTlsEquivalent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipA2aSecurityPostureV1 {
    TlsEquivalent,
    MutualTlsEquivalent,
    TlsOrMutualTlsEquivalent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipA2aAgentCardClaimV1 {
    pub agent_card_ref: String,
    pub agent_id_claim: String,
    pub display_name_claim: String,
    pub advertised_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipA2aCapabilityMappingV1 {
    pub external_capability_claim: String,
    pub adl_capability: String,
    pub policy_basis_ref: String,
    pub invocation_required: bool,
    pub direct_execution_forbidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipA2aTrustClassificationV1 {
    pub classification: AcipA2aTrustClassV1,
    pub basis_refs: Vec<String>,
    pub execution_authority_granted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipA2aInvokeBoundaryV1 {
    pub required_entrypoint: String,
    pub invocation_contract_schema_ref: String,
    pub trace_bundle_schema_ref: String,
    pub direct_execution_forbidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipA2aTransportBoundaryV1 {
    pub local_scope_only: bool,
    pub external_transport_status: AcipA2aTransportBoundaryStatusV1,
    pub required_security_posture: AcipA2aSecurityPostureV1,
    pub refusal_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipA2aAdapterBoundaryV1 {
    pub schema_version: String,
    pub adapter_id: String,
    pub purpose: String,
    pub identity_claim: AcipA2aAgentCardClaimV1,
    pub capability_mappings: Vec<AcipA2aCapabilityMappingV1>,
    pub trust_classification: AcipA2aTrustClassificationV1,
    pub invoke_boundary: AcipA2aInvokeBoundaryV1,
    pub transport_boundary: AcipA2aTransportBoundaryV1,
    pub trace_evidence_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipA2aNegativeCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub value: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipA2aFixtureSetV1 {
    pub schema_version: String,
    pub valid_adapters: Vec<AcipA2aAdapterBoundaryV1>,
    pub negative_cases: Vec<AcipA2aNegativeCaseV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipProofDemoPacketV1 {
    pub schema_version: String,
    pub demo_id: String,
    pub title: String,
    pub proof_classification: AcipProofClassificationV1,
    pub represented_modes: Vec<AcipIntentV1>,
    pub proves: Vec<String>,
    pub conversation: AcipConversationEnvelopeV1,
    pub coding_invocation: AcipCodingInvocationContractV1,
    pub coding_outcome: AcipCodingOutcomeV1,
    pub trace_bundle: AcipTraceBundleV1,
    pub reviewer_visible_artifact_refs: Vec<String>,
    pub public_visible_artifact_refs: Vec<String>,
    pub feature_doc_refs: Vec<String>,
    pub validation_commands: Vec<String>,
    pub non_proving_statements: Vec<String>,
}

pub mod transport {
    use super::*;
    include!("agent_comms/transport.inc");
}

pub mod dispatch {
    use super::*;

    pub mod invocation {
        use super::*;
        include!("agent_comms/dispatch/invocation.inc");
    }

    pub mod review {
        use super::*;
        include!("agent_comms/dispatch/review.inc");
    }

    pub mod coding {
        use super::*;
        include!("agent_comms/dispatch/coding.inc");
    }

    pub use coding::*;
    pub use invocation::*;
    pub use review::*;
}

pub mod orchestrate {
    use super::*;

    pub mod conformance {
        use super::*;
        include!("agent_comms/orchestrate/conformance.inc");
    }

    pub mod proof_demo {
        use super::*;
        include!("agent_comms/orchestrate/proof_demo.inc");
    }

    pub mod trace {
        #![allow(dead_code)]
        use super::*;
        include!("agent_comms/orchestrate/trace.inc");
    }

    pub use conformance::*;
    pub use proof_demo::*;
    pub use trace::*;
}

pub mod a2a {
    use super::*;
    include!("agent_comms/a2a.inc");
}

pub use a2a::*;
pub use dispatch::*;
pub use orchestrate::*;
pub use transport::*;

#[cfg(test)]
pub(crate) use dispatch::coding::{sample_coding_invocation_contract, sample_coding_outcome};
#[cfg(test)]
pub(crate) use dispatch::invocation::{sample_invocation_contract, sample_invocation_event};
#[cfg(test)]
pub(crate) use dispatch::review::{sample_review_invocation_contract, sample_review_outcome};
#[cfg(test)]
pub(crate) use orchestrate::trace::sample_trace_bundle;

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
    include!("agent_comms/tests.inc");
}
