use crate::acc::{AccGrantStatusV1, AdlCapabilityContractV1};
use crate::tool_registry::ToolRegistryV1;
use crate::uts::UtsSideEffectClassV1;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

mod core;
mod fixtures;
mod frontend;
#[cfg(test)]
mod tests;

pub use core::compile_uts_to_acc_v1;
pub use fixtures::{
    wp09_compiler_input_fixture, wp09_compiler_registry_fixture, wp09_policy_context_fixture,
    wp09_proposal_fixture,
};
pub use frontend::normalize_tool_proposal_arguments_v1;

pub(crate) const WP10_MAX_ARGUMENT_BYTES_V1: usize = 4096;
pub(crate) const WP10_MAX_STRING_BYTES_V1: usize = 1024;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsAccCompilerDecisionV1 {
    AccEmitted,
    RejectionEmitted,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsAccCompilerEvidenceStageV1 {
    Validation,
    Normalization,
    RegistryBinding,
    Policy,
    Rejection,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsAccCompilerRejectionCodeV1 {
    InvalidUts,
    InvalidProposal,
    RegistryBindingRejected,
    AmbiguousProposal,
    UnsatisfiableAuthority,
    ResourceConstraintUnsatisfied,
    PrivacyConstraintUnsatisfied,
    VisibilityConstraintUnsatisfied,
    ReplayConstraintUnsatisfied,
    ExecutionConstraintUnsatisfied,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsArgumentNormalizationErrorCodeV1 {
    MalformedValue,
    InjectionString,
    PathTraversal,
    OversizedPayload,
    MissingRequiredArgument,
    AmbiguousDefault,
    UnexpectedAdditionalField,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsAccCompilerEvidenceV1 {
    pub stage: UtsAccCompilerEvidenceStageV1,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolProposalV1 {
    pub proposal_id: String,
    pub tool_name: String,
    pub tool_version: String,
    pub adapter_id: String,
    pub arguments: BTreeMap<String, JsonValue>,
    pub dry_run_requested: bool,
    pub ambiguous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsAccDelegationContextV1 {
    pub delegation_id: String,
    pub grantor_actor_id: String,
    pub delegate_actor_id: String,
    pub depth: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsAccPolicyContextV1 {
    pub actor_id: String,
    pub role: String,
    pub standing: String,
    pub authenticated: bool,
    pub grant_id: String,
    pub grantor_actor_id: String,
    pub grant_status: AccGrantStatusV1,
    #[serde(default)]
    pub delegation: Option<UtsAccDelegationContextV1>,
    pub allowed_side_effects: Vec<UtsSideEffectClassV1>,
    pub allowed_resource_scopes: Vec<String>,
    pub allow_sensitive_data: bool,
    pub visibility_constructible: bool,
    pub replay_allowed: bool,
    pub execution_approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UtsAccCompilerInputV1 {
    pub proposal: ToolProposalV1,
    pub registry: ToolRegistryV1,
    pub policy_context: UtsAccPolicyContextV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsAccRejectionRecordV1 {
    pub code: UtsAccCompilerRejectionCodeV1,
    pub message: String,
    pub evidence: Vec<UtsAccCompilerEvidenceV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UtsAccCompilerOutcomeV1 {
    pub decision: UtsAccCompilerDecisionV1,
    #[serde(default)]
    pub acc: Option<AdlCapabilityContractV1>,
    #[serde(default)]
    pub rejection: Option<UtsAccRejectionRecordV1>,
    pub evidence: Vec<UtsAccCompilerEvidenceV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsArgumentNormalizationErrorV1 {
    pub code: UtsArgumentNormalizationErrorCodeV1,
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsArgumentNormalizationReportV1 {
    pub errors: Vec<UtsArgumentNormalizationErrorV1>,
}

impl UtsArgumentNormalizationReportV1 {
    pub fn codes(&self) -> Vec<UtsArgumentNormalizationErrorCodeV1> {
        self.errors.iter().map(|error| error.code.clone()).collect()
    }
}

pub(crate) fn push_normalization_error(
    errors: &mut Vec<UtsArgumentNormalizationErrorV1>,
    code: UtsArgumentNormalizationErrorCodeV1,
    field: impl Into<String>,
    message: impl Into<String>,
) {
    errors.push(UtsArgumentNormalizationErrorV1 {
        code,
        field: field.into(),
        message: message.into(),
    });
}

pub(crate) fn evidence(
    stage: UtsAccCompilerEvidenceStageV1,
    detail: impl Into<String>,
) -> UtsAccCompilerEvidenceV1 {
    UtsAccCompilerEvidenceV1 {
        stage,
        detail: detail.into(),
    }
}

pub(crate) fn reject(
    code: UtsAccCompilerRejectionCodeV1,
    message: impl Into<String>,
    mut evidence_log: Vec<UtsAccCompilerEvidenceV1>,
) -> UtsAccCompilerOutcomeV1 {
    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Rejection,
        format!("{:?}", code),
    ));
    UtsAccCompilerOutcomeV1 {
        decision: UtsAccCompilerDecisionV1::RejectionEmitted,
        acc: None,
        rejection: Some(UtsAccRejectionRecordV1 {
            code,
            message: message.into(),
            evidence: evidence_log.clone(),
        }),
        evidence: evidence_log,
    }
}
