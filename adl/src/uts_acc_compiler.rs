use crate::acc::{
    acc_v1_redaction_examples, acc_v1_visibility_matrix, validate_acc_v1, AccActorIdentityV1,
    AccActorKindV1, AccAuthorityEvidenceKindV1, AccAuthorityEvidenceV1, AccAuthorityGrantV1,
    AccCapabilityRequirementV1, AccConfirmationRequirementV1, AccDecisionV1, AccDelegationStepV1,
    AccExecutionSemanticsV1, AccFailurePolicyV1, AccFreedomGateDecisionV1,
    AccFreedomGateRequirementV1, AccGrantStatusV1, AccPolicyCheckV1, AccPrivacyRedactionV1,
    AccRoleStandingV1, AccToolReferenceV1, AccTracePrivacyPolicyV1, AccTraceReplayV1,
    AccVisibilityPolicyV1, AdlCapabilityContractV1, ACC_SCHEMA_VERSION_V1,
};
use crate::tool_registry::{
    bind_tool_registry_v1, registry_state_fingerprint_v1, wp08_tool_registry_v1_fixture,
    RegisteredToolV1, ToolAdapterCapabilityV1, ToolBindingDecisionV1, ToolBindingRequestV1,
    ToolBindingSourceV1, ToolRegistryV1,
};
use crate::uts::{
    validate_uts_v1, UniversalToolSchemaV1, UtsAuthenticationModeV1,
    UtsAuthenticationRequirementV1, UtsDataSensitivityV1, UtsDeterminismV1, UtsErrorModelV1,
    UtsExecutionEnvironmentKindV1, UtsExecutionEnvironmentV1, UtsExfiltrationRiskV1,
    UtsIdempotenceV1, UtsJsonSchemaFragmentV1, UtsReplaySafetyV1, UtsResourceRequirementV1,
    UtsSideEffectClassV1, UTS_SCHEMA_VERSION_V1,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const WP10_MAX_ARGUMENT_BYTES_V1: usize = 4096;
const WP10_MAX_STRING_BYTES_V1: usize = 1024;

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

fn push_normalization_error(
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

fn evidence(
    stage: UtsAccCompilerEvidenceStageV1,
    detail: impl Into<String>,
) -> UtsAccCompilerEvidenceV1 {
    UtsAccCompilerEvidenceV1 {
        stage,
        detail: detail.into(),
    }
}

fn reject(
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

fn side_effect_label(side_effect: &UtsSideEffectClassV1) -> &'static str {
    match side_effect {
        UtsSideEffectClassV1::Read => "read",
        UtsSideEffectClassV1::LocalWrite => "local_write",
        UtsSideEffectClassV1::ExternalRead => "external_read",
        UtsSideEffectClassV1::ExternalWrite => "external_write",
        UtsSideEffectClassV1::Process => "process",
        UtsSideEffectClassV1::Network => "network",
        UtsSideEffectClassV1::Destructive => "destructive",
        UtsSideEffectClassV1::Exfiltration => "exfiltration",
    }
}

fn environment_label(environment: &UtsExecutionEnvironmentKindV1) -> &'static str {
    match environment {
        UtsExecutionEnvironmentKindV1::Fixture => "fixture",
        UtsExecutionEnvironmentKindV1::DryRun => "dry_run",
        UtsExecutionEnvironmentKindV1::Local => "local",
        UtsExecutionEnvironmentKindV1::ExternalService => "external_service",
        UtsExecutionEnvironmentKindV1::Process => "process",
        UtsExecutionEnvironmentKindV1::Network => "network",
    }
}

fn first_resource(schema: &UniversalToolSchemaV1) -> Option<&UtsResourceRequirementV1> {
    schema.resources.first()
}

fn evidence_digest(value: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(value.as_bytes()))
}

fn proposal_arguments_evidence(arguments: &BTreeMap<String, JsonValue>) -> String {
    let canonical_arguments =
        serde_json::to_string(arguments).expect("proposal arguments should serialize");
    format!(
        "proposal_arguments_redacted count={} digest={}",
        arguments.len(),
        evidence_digest(&canonical_arguments)
    )
}

fn normalized_arguments_evidence(arguments: &BTreeMap<String, JsonValue>) -> String {
    let canonical_arguments =
        serde_json::to_string(arguments).expect("normalized arguments should serialize");
    format!(
        "normalized_arguments_redacted count={} digest={}",
        arguments.len(),
        evidence_digest(&canonical_arguments)
    )
}

fn registry_evidence(registry: &ToolRegistryV1) -> String {
    let fingerprint = registry_state_fingerprint_v1(registry);
    format!("registry_state_digest={}", evidence_digest(&fingerprint))
}

fn schema_properties(schema: &UniversalToolSchemaV1) -> BTreeMap<String, JsonValue> {
    schema
        .input_schema
        .keywords
        .get("properties")
        .and_then(JsonValue::as_object)
        .map(|properties| {
            properties
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect()
        })
        .unwrap_or_default()
}

fn schema_required_fields(schema: &UniversalToolSchemaV1) -> BTreeSet<String> {
    schema
        .input_schema
        .keywords
        .get("required")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
        .filter_map(JsonValue::as_str)
        .map(ToString::to_string)
        .collect()
}

fn schema_allows_additional_fields(schema: &UniversalToolSchemaV1) -> bool {
    schema
        .input_schema
        .keywords
        .get("additionalProperties")
        .and_then(JsonValue::as_bool)
        .unwrap_or(true)
}

fn expected_json_type(property_schema: &JsonValue) -> Option<&str> {
    property_schema
        .as_object()
        .and_then(|schema| schema.get("type"))
        .and_then(JsonValue::as_str)
}

fn value_matches_expected_type(value: &JsonValue, expected_type: &str) -> bool {
    match expected_type {
        "array" => value.is_array(),
        "boolean" => value.is_boolean(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "number" => value.is_number(),
        "object" => value.is_object(),
        "string" => value.is_string(),
        _ => false,
    }
}

fn normalize_value(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::String(value) => JsonValue::String(value.trim().to_string()),
        JsonValue::Array(values) => JsonValue::Array(values.iter().map(normalize_value).collect()),
        JsonValue::Object(values) => JsonValue::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), normalize_value(value)))
                .collect(),
        ),
        _ => value.clone(),
    }
}

fn contains_injection_marker(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
    [
        "<script",
        "{{",
        "}}",
        "$(",
        "`",
        "; rm ",
        "ignore previous instructions",
        "system prompt",
    ]
    .iter()
    .any(|marker| lowered.contains(marker))
}

fn contains_path_traversal(value: &str) -> bool {
    let value = value.trim();
    value.contains("../")
        || value.contains("..\\")
        || value.starts_with("~/")
        || value.starts_with('/')
        || value.as_bytes().get(1).is_some_and(|byte| *byte == b':')
}

fn scan_value_safety(
    field: &str,
    value: &JsonValue,
    errors: &mut Vec<UtsArgumentNormalizationErrorV1>,
) {
    match value {
        JsonValue::String(value) => {
            if value.len() > WP10_MAX_STRING_BYTES_V1 {
                push_normalization_error(
                    errors,
                    UtsArgumentNormalizationErrorCodeV1::OversizedPayload,
                    field,
                    "argument string exceeds the bounded fixture limit",
                );
            }
            if contains_injection_marker(value) {
                push_normalization_error(
                    errors,
                    UtsArgumentNormalizationErrorCodeV1::InjectionString,
                    field,
                    "argument string contains an unsafe control marker",
                );
            }
            if contains_path_traversal(value) {
                push_normalization_error(
                    errors,
                    UtsArgumentNormalizationErrorCodeV1::PathTraversal,
                    field,
                    "argument string contains path traversal or absolute path syntax",
                );
            }
        }
        JsonValue::Array(values) => {
            for value in values {
                scan_value_safety(field, value, errors);
            }
        }
        JsonValue::Object(values) => {
            for (key, value) in values {
                if contains_injection_marker(key) {
                    push_normalization_error(
                        errors,
                        UtsArgumentNormalizationErrorCodeV1::InjectionString,
                        field,
                        "argument object key contains an unsafe control marker",
                    );
                }
                if contains_path_traversal(key) {
                    push_normalization_error(
                        errors,
                        UtsArgumentNormalizationErrorCodeV1::PathTraversal,
                        field,
                        "argument object key contains path traversal or absolute path syntax",
                    );
                }
                scan_value_safety(field, value, errors);
            }
        }
        _ => {}
    }
}

pub fn normalize_tool_proposal_arguments_v1(
    schema: &UniversalToolSchemaV1,
    arguments: &BTreeMap<String, JsonValue>,
) -> Result<BTreeMap<String, JsonValue>, UtsArgumentNormalizationReportV1> {
    let mut errors = Vec::new();
    let properties = schema_properties(schema);
    let required = schema_required_fields(schema);
    let allows_additional = schema_allows_additional_fields(schema);

    let serialized = serde_json::to_string(arguments).expect("proposal arguments should serialize");
    if serialized.len() > WP10_MAX_ARGUMENT_BYTES_V1 {
        push_normalization_error(
            &mut errors,
            UtsArgumentNormalizationErrorCodeV1::OversizedPayload,
            "arguments",
            "argument payload exceeds the bounded fixture limit",
        );
    }

    for (field, property_schema) in &properties {
        if !arguments.contains_key(field)
            && property_schema
                .as_object()
                .is_some_and(|schema| schema.contains_key("default"))
        {
            push_normalization_error(
                &mut errors,
                UtsArgumentNormalizationErrorCodeV1::AmbiguousDefault,
                field,
                "schema default is ambiguous for an omitted model-produced argument",
            );
        }
    }

    for required_field in &required {
        if !arguments.contains_key(required_field)
            && !errors.iter().any(|error| error.field == *required_field)
        {
            push_normalization_error(
                &mut errors,
                UtsArgumentNormalizationErrorCodeV1::MissingRequiredArgument,
                required_field,
                "required argument is absent before policy evaluation",
            );
        }
    }

    if !allows_additional {
        for field in arguments.keys() {
            if !properties.contains_key(field) {
                push_normalization_error(
                    &mut errors,
                    UtsArgumentNormalizationErrorCodeV1::UnexpectedAdditionalField,
                    field,
                    "argument is not declared by the UTS input schema",
                );
            }
        }
    }

    let mut normalized = BTreeMap::new();
    for (field, value) in arguments {
        if let Some(property_schema) = properties.get(field) {
            if let Some(expected_type) = expected_json_type(property_schema) {
                if !value_matches_expected_type(value, expected_type) {
                    push_normalization_error(
                        &mut errors,
                        UtsArgumentNormalizationErrorCodeV1::MalformedValue,
                        field,
                        "argument value does not match the declared JSON type",
                    );
                }
            }
        }
        scan_value_safety(field, value, &mut errors);
        normalized.insert(field.clone(), normalize_value(value));
    }

    if errors.is_empty() {
        Ok(normalized)
    } else {
        Err(UtsArgumentNormalizationReportV1 { errors })
    }
}

fn proposal_token_like(value: &str) -> bool {
    !value.trim().is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.')
        })
}

fn registered_tool<'a>(
    registry: &'a ToolRegistryV1,
    tool_name: &str,
    tool_version: &str,
) -> Option<&'a RegisteredToolV1> {
    registry
        .tools
        .iter()
        .find(|tool| tool.tool_name == tool_name && tool.tool_version == tool_version)
}

fn compile_decision(policy: &UtsAccPolicyContextV1) -> AccDecisionV1 {
    match policy.grant_status {
        AccGrantStatusV1::Delegated => AccDecisionV1::Delegated,
        AccGrantStatusV1::Revoked => AccDecisionV1::Revoked,
        AccGrantStatusV1::Denied => AccDecisionV1::Denied,
        AccGrantStatusV1::Active => AccDecisionV1::Allowed,
    }
}

fn policy_evidence_ref(policy: &UtsAccPolicyContextV1) -> String {
    policy
        .delegation
        .as_ref()
        .map(|delegation| delegation.delegation_id.clone())
        .unwrap_or_else(|| policy.grant_id.clone())
}

fn build_acc(
    input: &UtsAccCompilerInputV1,
    tool: &RegisteredToolV1,
    binding: &crate::tool_registry::ToolBindingV1,
) -> AdlCapabilityContractV1 {
    let schema = &tool.uts;
    let resource = first_resource(schema).expect("validated UTS should include a resource");
    let decision = compile_decision(&input.policy_context);
    let delegation_chain = input
        .policy_context
        .delegation
        .as_ref()
        .map(|delegation| {
            vec![AccDelegationStepV1 {
                delegation_id: delegation.delegation_id.clone(),
                grantor_actor_id: delegation.grantor_actor_id.clone(),
                delegate_actor_id: delegation.delegate_actor_id.clone(),
                grant_id: input.policy_context.grant_id.clone(),
                depth: delegation.depth,
            }]
        })
        .unwrap_or_default();

    AdlCapabilityContractV1 {
        schema_version: ACC_SCHEMA_VERSION_V1.to_string(),
        contract_id: format!("acc.compiler.{}", input.proposal.proposal_id),
        tool: AccToolReferenceV1 {
            tool_name: input.proposal.tool_name.clone(),
            tool_version: input.proposal.tool_version.clone(),
            registry_tool_id: binding.registry_tool_id.clone(),
            adapter_id: binding.adapter_id.clone(),
        },
        actor: AccActorIdentityV1 {
            actor_id: input.policy_context.actor_id.clone(),
            actor_kind: AccActorKindV1::Operator,
            authenticated: input.policy_context.authenticated,
            authority_evidence: vec![
                AccAuthorityEvidenceV1 {
                    evidence_id: format!("credential.{}", input.policy_context.actor_id),
                    kind: AccAuthorityEvidenceKindV1::Credential,
                    issuer: "adl.wp09.fixture".to_string(),
                },
                AccAuthorityEvidenceV1 {
                    evidence_id: "registry.wp08.binding".to_string(),
                    kind: AccAuthorityEvidenceKindV1::RegistryGrant,
                    issuer: "adl.tool_registry.v1".to_string(),
                },
            ],
        },
        authority_grant: AccAuthorityGrantV1 {
            grant_id: input.policy_context.grant_id.clone(),
            grantor_actor_id: input.policy_context.grantor_actor_id.clone(),
            grantee_actor_id: input.policy_context.actor_id.clone(),
            capability_id: binding.capability_id.clone(),
            scope: resource.scope.clone(),
            status: input.policy_context.grant_status.clone(),
            revocation_reason: None,
        },
        role_standing: AccRoleStandingV1 {
            role: input.policy_context.role.clone(),
            standing: input.policy_context.standing.clone(),
        },
        delegation_chain,
        capability: AccCapabilityRequirementV1 {
            capability_id: binding.capability_id.clone(),
            side_effect_class: side_effect_label(&schema.side_effect_class).to_string(),
            resource_type: resource.resource_type.clone(),
            resource_scope: resource.scope.clone(),
        },
        policy_checks: vec![AccPolicyCheckV1 {
            policy_id: "policy.wp09.compiler".to_string(),
            decision: decision.clone(),
            evidence_ref: policy_evidence_ref(&input.policy_context),
        }],
        confirmation: AccConfirmationRequirementV1 {
            required: false,
            confirmed_by_actor_id: None,
            confirmation_id: None,
        },
        freedom_gate: AccFreedomGateRequirementV1 {
            required: false,
            decision: AccFreedomGateDecisionV1::NotRequired,
            event_id: None,
        },
        execution: AccExecutionSemanticsV1 {
            adapter_id: binding.adapter_id.clone(),
            environment: environment_label(&schema.execution_environment.kind).to_string(),
            dry_run: binding.dry_run,
            approved_for_execution: matches!(decision, AccDecisionV1::Allowed)
                && input.policy_context.execution_approved,
        },
        trace_replay: AccTraceReplayV1 {
            trace_id: format!("trace.compiler.{}", input.proposal.proposal_id),
            replay_allowed: input.policy_context.replay_allowed,
            replay_posture: "deterministic_fixture_compiler".to_string(),
            evidence_refs: vec![
                "compiler.validation".to_string(),
                "compiler.normalization".to_string(),
                "compiler.registry_binding".to_string(),
                "compiler.policy".to_string(),
            ],
        },
        privacy_redaction: AccPrivacyRedactionV1 {
            data_sensitivity: format!("{:?}", schema.data_sensitivity).to_ascii_lowercase(),
            visibility: AccVisibilityPolicyV1 {
                actor_view: "compiled ACC request status".to_string(),
                operator_view: "full compiler fixture evidence".to_string(),
                reviewer_view: "redacted compiler evidence and policy result".to_string(),
                public_report_view: "aggregate compiler pass/fail only".to_string(),
                observatory_projection: "redacted compiler governance event".to_string(),
            },
            redaction_rules: vec!["redact_compiler_fixture_payload".to_string()],
            visibility_matrix: acc_v1_visibility_matrix(),
            redaction_examples: acc_v1_redaction_examples(),
            trace_privacy: AccTracePrivacyPolicyV1 {
                exposes_citizen_private_state: false,
                protected_state_refs: vec![
                    "citizen.private_state".to_string(),
                    "operator.private_state".to_string(),
                ],
            },
        },
        failure_policy: AccFailurePolicyV1 {
            failure_code: "compiler_rejection".to_string(),
            message: "Compiler emitted a reviewable ACC or rejection.".to_string(),
            retryable: false,
        },
        decision,
    }
}

pub fn compile_uts_to_acc_v1(input: &UtsAccCompilerInputV1) -> UtsAccCompilerOutcomeV1 {
    let mut evidence_log = vec![evidence(
        UtsAccCompilerEvidenceStageV1::Validation,
        "compiler input received",
    )];

    if !proposal_token_like(&input.proposal.proposal_id)
        || !proposal_token_like(&input.proposal.tool_name)
        || input.proposal.tool_version.trim().is_empty()
        || !proposal_token_like(&input.proposal.adapter_id)
    {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Normalization,
            "proposal identifiers are not normalizable",
        ));
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because proposal normalization failed",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::InvalidProposal,
            "proposal identifiers must be stable repository tokens",
            evidence_log,
        );
    }

    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Normalization,
        proposal_arguments_evidence(&input.proposal.arguments),
    ));

    let binding_request = ToolBindingRequestV1 {
        source: ToolBindingSourceV1::RegistryCompiler,
        tool_name: input.proposal.tool_name.clone(),
        tool_version: input.proposal.tool_version.clone(),
        adapter_id: input.proposal.adapter_id.clone(),
        dry_run_requested: input.proposal.dry_run_requested,
    };
    let binding_outcome = bind_tool_registry_v1(&input.registry, &binding_request);
    if !matches!(binding_outcome.decision, ToolBindingDecisionV1::Bound) {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::RegistryBinding,
            format!("registry rejected: {:?}", binding_outcome.rejection_code),
        ));
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because registry binding failed",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::RegistryBindingRejected,
            "registry binding failed before ACC construction",
            evidence_log,
        );
    }

    let Some(tool) = registered_tool(
        &input.registry,
        &input.proposal.tool_name,
        &input.proposal.tool_version,
    ) else {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::RegistryBinding,
            "registered tool disappeared after binding",
        ));
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because registry binding was inconsistent",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::RegistryBindingRejected,
            "registered tool missing after binding",
            evidence_log,
        );
    };

    if let Err(report) = validate_uts_v1(&tool.uts) {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Validation,
            format!("UTS validation errors: {:?}", report.codes()),
        ));
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because UTS validation failed",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::InvalidUts,
            "UTS failed validation",
            evidence_log,
        );
    }

    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Validation,
        "UTS validation passed",
    ));

    let normalized_arguments =
        match normalize_tool_proposal_arguments_v1(&tool.uts, &input.proposal.arguments) {
            Ok(arguments) => arguments,
            Err(report) => {
                evidence_log.push(evidence(
                    UtsAccCompilerEvidenceStageV1::Normalization,
                    format!("argument_normalization rejected codes={:?}", report.codes()),
                ));
                evidence_log.push(evidence(
                    UtsAccCompilerEvidenceStageV1::Policy,
                    "policy not evaluated because argument normalization failed",
                ));
                return reject(
                    UtsAccCompilerRejectionCodeV1::InvalidProposal,
                    "proposal arguments failed normalization before policy evaluation",
                    evidence_log,
                );
            }
        };
    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Normalization,
        normalized_arguments_evidence(&normalized_arguments),
    ));
    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::RegistryBinding,
        registry_evidence(&input.registry),
    ));

    if input.proposal.ambiguous {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because proposal is ambiguous",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::AmbiguousProposal,
            "ambiguous proposal rejected before ACC construction",
            evidence_log,
        );
    }

    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Policy,
        "policy context evaluated",
    ));

    if tool.uts.resources.len() != 1 {
        return reject(
            UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied,
            "ACC v1 compiler fixture requires exactly one representable resource scope",
            evidence_log,
        );
    }

    if !input.policy_context.authenticated
        || !matches!(
            input.policy_context.grant_status,
            AccGrantStatusV1::Active | AccGrantStatusV1::Delegated
        )
        || !input
            .policy_context
            .allowed_side_effects
            .contains(&tool.uts.side_effect_class)
    {
        return reject(
            UtsAccCompilerRejectionCodeV1::UnsatisfiableAuthority,
            "authority context does not allow this tool proposal",
            evidence_log,
        );
    }

    if tool.uts.resources.iter().any(|resource| {
        !input
            .policy_context
            .allowed_resource_scopes
            .contains(&resource.scope)
    }) {
        return reject(
            UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied,
            "resource scope is not allowed by policy context",
            evidence_log,
        );
    }

    if !input.policy_context.allow_sensitive_data
        && (matches!(
            tool.uts.data_sensitivity,
            UtsDataSensitivityV1::Secret
                | UtsDataSensitivityV1::ProtectedPrompt
                | UtsDataSensitivityV1::PrivateState
        ) || matches!(tool.uts.exfiltration_risk, UtsExfiltrationRiskV1::High))
    {
        return reject(
            UtsAccCompilerRejectionCodeV1::PrivacyConstraintUnsatisfied,
            "privacy context does not allow sensitive or exfiltrating tool proposal",
            evidence_log,
        );
    }

    if !input.policy_context.visibility_constructible {
        return reject(
            UtsAccCompilerRejectionCodeV1::VisibilityConstraintUnsatisfied,
            "visibility matrix could not be constructed safely",
            evidence_log,
        );
    }

    if !input.policy_context.replay_allowed
        || !matches!(tool.uts.replay_safety, UtsReplaySafetyV1::ReplaySafe)
    {
        return reject(
            UtsAccCompilerRejectionCodeV1::ReplayConstraintUnsatisfied,
            "replay constraints are not satisfied",
            evidence_log,
        );
    }

    if !input.policy_context.execution_approved
        && !matches!(
            input.policy_context.grant_status,
            AccGrantStatusV1::Delegated
        )
    {
        return reject(
            UtsAccCompilerRejectionCodeV1::ExecutionConstraintUnsatisfied,
            "execution approval is required for non-delegated ACC construction",
            evidence_log,
        );
    }

    let binding = binding_outcome
        .binding
        .as_ref()
        .expect("bound outcome should carry binding");
    let acc = build_acc(input, tool, binding);
    if let Err(report) = validate_acc_v1(&acc) {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Validation,
            format!("ACC validation errors: {:?}", report.codes()),
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::ExecutionConstraintUnsatisfied,
            "compiled ACC failed validation",
            evidence_log,
        );
    }

    UtsAccCompilerOutcomeV1 {
        decision: UtsAccCompilerDecisionV1::AccEmitted,
        acc: Some(acc),
        rejection: None,
        evidence: evidence_log,
    }
}

fn schema_for_tool(
    name: &str,
    side_effect: UtsSideEffectClassV1,
    resource_scope: &str,
    data_sensitivity: UtsDataSensitivityV1,
    exfiltration_risk: UtsExfiltrationRiskV1,
) -> UniversalToolSchemaV1 {
    UniversalToolSchemaV1 {
        schema_version: UTS_SCHEMA_VERSION_V1.to_string(),
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: format!("Fixture schema for compiler mapping case {name}."),
        input_schema: UtsJsonSchemaFragmentV1 {
            schema_type: "object".to_string(),
            keywords: BTreeMap::from([
                (
                    "properties".to_string(),
                    json!({"fixture_id": {"type": "string"}}),
                ),
                ("required".to_string(), json!(["fixture_id"])),
                ("additionalProperties".to_string(), json!(false)),
            ]),
        },
        output_schema: UtsJsonSchemaFragmentV1 {
            schema_type: "object".to_string(),
            keywords: BTreeMap::from([
                (
                    "properties".to_string(),
                    json!({"content": {"type": "string"}}),
                ),
                ("required".to_string(), json!(["content"])),
                ("additionalProperties".to_string(), json!(false)),
            ]),
        },
        side_effect_class: side_effect,
        determinism: UtsDeterminismV1::Deterministic,
        replay_safety: UtsReplaySafetyV1::ReplaySafe,
        idempotence: UtsIdempotenceV1::Idempotent,
        resources: vec![UtsResourceRequirementV1 {
            resource_type: "fixture".to_string(),
            scope: resource_scope.to_string(),
        }],
        authentication: UtsAuthenticationRequirementV1 {
            mode: UtsAuthenticationModeV1::None,
            required: false,
        },
        data_sensitivity,
        exfiltration_risk,
        execution_environment: UtsExecutionEnvironmentV1 {
            kind: UtsExecutionEnvironmentKindV1::DryRun,
            isolation: "deterministic compiler dry-run fixture only".to_string(),
        },
        errors: vec![UtsErrorModelV1 {
            code: "fixture_not_available".to_string(),
            message: "The requested compiler fixture is not available.".to_string(),
            retryable: false,
        }],
        extensions: BTreeMap::new(),
    }
}

pub fn wp09_compiler_registry_fixture() -> ToolRegistryV1 {
    let mut registry = wp08_tool_registry_v1_fixture();
    for (name, side_effect, scope, sensitivity, exfiltration) in [
        (
            "fixture.local_write",
            UtsSideEffectClassV1::LocalWrite,
            "local-write",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::None,
        ),
        (
            "fixture.destructive",
            UtsSideEffectClassV1::Destructive,
            "destructive-fixture",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::Medium,
        ),
        (
            "fixture.exfiltrate",
            UtsSideEffectClassV1::Exfiltration,
            "protected-prompt",
            UtsDataSensitivityV1::Secret,
            UtsExfiltrationRiskV1::High,
        ),
    ] {
        let adapter_id = format!("adapter.{name}.dry_run");
        registry.tools.push(RegisteredToolV1 {
            registry_tool_id: format!("registry.{name}"),
            tool_name: name.to_string(),
            tool_version: "1.0.0".to_string(),
            active: true,
            uts: schema_for_tool(name, side_effect, scope, sensitivity, exfiltration),
            approved_adapter_ids: vec![adapter_id.clone()],
        });
        registry.adapters.push(ToolAdapterCapabilityV1 {
            adapter_id,
            tool_name: name.to_string(),
            tool_version: "1.0.0".to_string(),
            capability_id: format!("capability.{}", name.replace('_', "-")),
            side_effect_class: side_effect,
            execution_environment: UtsExecutionEnvironmentKindV1::DryRun,
            supports_dry_run: true,
            approved_for_binding: true,
        });
    }
    registry
}

pub fn wp09_policy_context_fixture() -> UtsAccPolicyContextV1 {
    UtsAccPolicyContextV1 {
        actor_id: "actor.operator.alice".to_string(),
        role: "operator".to_string(),
        standing: "active".to_string(),
        authenticated: true,
        grant_id: "grant.compiler.fixture".to_string(),
        grantor_actor_id: "actor.operator.alice".to_string(),
        grant_status: AccGrantStatusV1::Active,
        delegation: None,
        allowed_side_effects: vec![UtsSideEffectClassV1::Read, UtsSideEffectClassV1::LocalWrite],
        allowed_resource_scopes: vec!["local-readonly".to_string(), "local-write".to_string()],
        allow_sensitive_data: true,
        visibility_constructible: true,
        replay_allowed: true,
        execution_approved: true,
    }
}

pub fn wp09_proposal_fixture(tool_name: &str) -> ToolProposalV1 {
    ToolProposalV1 {
        proposal_id: format!("proposal.{}", tool_name.replace('_', "-")),
        tool_name: tool_name.to_string(),
        tool_version: "1.0.0".to_string(),
        adapter_id: format!("adapter.{tool_name}.dry_run"),
        arguments: BTreeMap::from([("fixture_id".to_string(), json!("fixture-a"))]),
        dry_run_requested: true,
        ambiguous: false,
    }
}

pub fn wp09_compiler_input_fixture(tool_name: &str) -> UtsAccCompilerInputV1 {
    let mut proposal = wp09_proposal_fixture(tool_name);
    if tool_name == "fixture.safe_read" {
        proposal.adapter_id = "adapter.fixture.safe_read.dry_run".to_string();
    }
    UtsAccCompilerInputV1 {
        proposal,
        registry: wp09_compiler_registry_fixture(),
        policy_context: wp09_policy_context_fixture(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stages(outcome: &UtsAccCompilerOutcomeV1) -> Vec<UtsAccCompilerEvidenceStageV1> {
        outcome
            .evidence
            .iter()
            .map(|evidence| evidence.stage.clone())
            .collect()
    }

    fn invalid_argument_outcome(arguments: BTreeMap<String, JsonValue>) -> UtsAccCompilerOutcomeV1 {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        input.proposal.arguments = arguments;
        compile_uts_to_acc_v1(&input)
    }

    fn object_argument_outcome(object_argument: JsonValue) -> UtsAccCompilerOutcomeV1 {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        let properties = input.registry.tools[0]
            .uts
            .input_schema
            .keywords
            .get_mut("properties")
            .and_then(JsonValue::as_object_mut)
            .expect("fixture has properties");
        properties.insert("payload".to_string(), json!({"type": "object"}));
        input.proposal.arguments = BTreeMap::from([
            ("fixture_id".to_string(), json!("fixture-a")),
            ("payload".to_string(), object_argument),
        ]);
        compile_uts_to_acc_v1(&input)
    }

    fn assert_invalid_argument_rejection(outcome: &UtsAccCompilerOutcomeV1) {
        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::RejectionEmitted);
        assert_eq!(
            outcome.rejection.as_ref().expect("rejection").code,
            UtsAccCompilerRejectionCodeV1::InvalidProposal
        );
        assert!(
            stages(outcome).contains(&UtsAccCompilerEvidenceStageV1::Policy),
            "unsafe arguments should stop before policy evaluation but record policy status"
        );
    }

    #[test]
    fn wp09_maps_safe_read_to_allowed_acc() {
        let input = wp09_compiler_input_fixture("fixture.safe_read");
        let outcome = compile_uts_to_acc_v1(&input);

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::AccEmitted);
        let acc = outcome.acc.expect("safe read should compile to ACC");
        assert_eq!(acc.decision, AccDecisionV1::Allowed);
        assert_eq!(acc.tool.tool_name, "fixture.safe_read");
        validate_acc_v1(&acc).expect("compiled safe-read ACC should validate");
    }

    #[test]
    fn wp09_maps_delegated_local_write_to_delegated_acc() {
        let mut input = wp09_compiler_input_fixture("fixture.local_write");
        input.policy_context.grant_status = AccGrantStatusV1::Delegated;
        input.policy_context.execution_approved = false;
        input.policy_context.delegation = Some(UtsAccDelegationContextV1 {
            delegation_id: "delegation.compiler.local-write".to_string(),
            grantor_actor_id: "actor.operator.alice".to_string(),
            delegate_actor_id: "actor.operator.alice".to_string(),
            depth: 1,
        });

        let outcome = compile_uts_to_acc_v1(&input);

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::AccEmitted);
        let acc = outcome.acc.expect("delegated local write should compile");
        assert_eq!(acc.decision, AccDecisionV1::Delegated);
        assert!(!acc.execution.approved_for_execution);
        assert_eq!(acc.delegation_chain.len(), 1);
        validate_acc_v1(&acc).expect("compiled delegated ACC should validate");
    }

    #[test]
    fn wp09_rejects_denied_destructive_action() {
        let input = wp09_compiler_input_fixture("fixture.destructive");
        let outcome = compile_uts_to_acc_v1(&input);

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::RejectionEmitted);
        assert_eq!(
            outcome.rejection.expect("rejection").code,
            UtsAccCompilerRejectionCodeV1::UnsatisfiableAuthority
        );
    }

    #[test]
    fn wp09_rejects_denied_exfiltration_for_privacy() {
        let mut input = wp09_compiler_input_fixture("fixture.exfiltrate");
        input.policy_context.allowed_side_effects = vec![UtsSideEffectClassV1::Exfiltration];
        input.policy_context.allowed_resource_scopes = vec!["protected-prompt".to_string()];
        input.policy_context.allow_sensitive_data = false;

        let outcome = compile_uts_to_acc_v1(&input);

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::RejectionEmitted);
        assert_eq!(
            outcome.rejection.expect("rejection").code,
            UtsAccCompilerRejectionCodeV1::PrivacyConstraintUnsatisfied
        );
    }

    #[test]
    fn wp09_rejects_ambiguous_proposal_with_evidence() {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        input.proposal.ambiguous = true;

        let outcome = compile_uts_to_acc_v1(&input);

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::RejectionEmitted);
        assert_eq!(
            outcome.rejection.as_ref().expect("rejection").code,
            UtsAccCompilerRejectionCodeV1::AmbiguousProposal
        );
        let stages = stages(&outcome);
        assert!(stages.contains(&UtsAccCompilerEvidenceStageV1::Validation));
        assert!(stages.contains(&UtsAccCompilerEvidenceStageV1::Normalization));
        assert!(stages.contains(&UtsAccCompilerEvidenceStageV1::RegistryBinding));
        assert!(stages.contains(&UtsAccCompilerEvidenceStageV1::Policy));
        assert!(stages.contains(&UtsAccCompilerEvidenceStageV1::Rejection));
    }

    #[test]
    fn wp09_same_inputs_produce_same_acc_or_rejection() {
        let input = wp09_compiler_input_fixture("fixture.safe_read");
        let first = compile_uts_to_acc_v1(&input);
        let second = compile_uts_to_acc_v1(&input);

        assert_eq!(first, second);

        let mut rejected_input = input.clone();
        rejected_input.policy_context.visibility_constructible = false;
        assert_eq!(
            compile_uts_to_acc_v1(&rejected_input),
            compile_uts_to_acc_v1(&rejected_input)
        );
    }

    #[test]
    fn wp09_rejects_non_normalizable_proposal_before_acc_construction() {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        input.proposal.proposal_id = "proposal.with/local/path".to_string();

        let outcome = compile_uts_to_acc_v1(&input);

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::RejectionEmitted);
        assert_eq!(
            outcome.rejection.as_ref().expect("rejection").code,
            UtsAccCompilerRejectionCodeV1::InvalidProposal
        );
        let stages = stages(&outcome);
        assert!(stages.contains(&UtsAccCompilerEvidenceStageV1::Normalization));
        assert!(stages.contains(&UtsAccCompilerEvidenceStageV1::Policy));
        assert!(stages.contains(&UtsAccCompilerEvidenceStageV1::Rejection));
    }

    #[test]
    fn wp09_redacts_arguments_and_registry_evidence() {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        input.proposal.arguments.insert(
            "fixture_id".to_string(),
            json!("redaction-sensitive-token-123"),
        );

        let outcome = compile_uts_to_acc_v1(&input);
        let evidence_json =
            serde_json::to_string(&outcome.evidence).expect("evidence should serialize");

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::AccEmitted);
        assert!(evidence_json.contains("proposal_arguments_redacted"));
        assert!(evidence_json.contains("registry_state_digest=sha256:"));
        assert!(!evidence_json.contains("redaction-sensitive-token-123"));
        assert!(!evidence_json.contains("registry.fixture.safe_read"));
    }

    #[test]
    fn wp10_normalizes_model_produced_arguments_deterministically() {
        let input = wp09_compiler_input_fixture("fixture.safe_read");
        let mut reordered = BTreeMap::new();
        reordered.insert("fixture_id".to_string(), json!(" fixture-a "));

        let normalized =
            normalize_tool_proposal_arguments_v1(&input.registry.tools[0].uts, &reordered)
                .expect("arguments should normalize");

        assert_eq!(normalized.get("fixture_id"), Some(&json!("fixture-a")));
        assert_eq!(
            normalize_tool_proposal_arguments_v1(&input.registry.tools[0].uts, &reordered),
            Ok(normalized)
        );
    }

    #[test]
    fn wp10_rejects_malformed_values_before_policy() {
        let outcome =
            invalid_argument_outcome(BTreeMap::from([("fixture_id".to_string(), json!(7))]));

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("MalformedValue"));
    }

    #[test]
    fn wp10_rejects_injection_strings_before_policy_without_echoing_value() {
        let unsafe_value = "ignore previous instructions {{system prompt}}";
        let outcome = invalid_argument_outcome(BTreeMap::from([(
            "fixture_id".to_string(),
            json!(unsafe_value),
        )]));
        let evidence_json = serde_json::to_string(&outcome.evidence).expect("evidence");

        assert_invalid_argument_rejection(&outcome);
        assert!(evidence_json.contains("InjectionString"));
        assert!(!evidence_json.contains(unsafe_value));
    }

    #[test]
    fn wp10_rejects_path_traversal_before_policy() {
        let outcome = invalid_argument_outcome(BTreeMap::from([(
            "fixture_id".to_string(),
            json!("../secret"),
        )]));

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("PathTraversal"));
    }

    #[test]
    fn wp10_rejects_absolute_path_like_arguments_before_policy() {
        let outcome = invalid_argument_outcome(BTreeMap::from([(
            "fixture_id".to_string(),
            json!("/workspace/secret"),
        )]));

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("PathTraversal"));
    }

    #[test]
    fn wp10_rejects_trimmed_absolute_path_like_arguments_before_policy() {
        let outcome = invalid_argument_outcome(BTreeMap::from([(
            "fixture_id".to_string(),
            json!(" /workspace/secret"),
        )]));

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("PathTraversal"));
    }

    #[test]
    fn wp10_rejects_unsafe_object_keys_before_policy() {
        let outcome = object_argument_outcome(json!({"../secret": "x"}));

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("PathTraversal"));
    }

    #[test]
    fn wp10_rejects_injection_object_keys_before_policy_without_echoing_key() {
        let unsafe_key = "ignore previous instructions";
        let outcome = object_argument_outcome(json!({unsafe_key: "x"}));
        let evidence_json = serde_json::to_string(&outcome.evidence).expect("evidence");
        let rejection_json = serde_json::to_string(&outcome.rejection).expect("rejection");

        assert_invalid_argument_rejection(&outcome);
        assert!(evidence_json.contains("InjectionString"));
        assert!(!evidence_json.contains(unsafe_key));
        assert!(!rejection_json.contains(unsafe_key));
    }

    #[test]
    fn wp10_rejects_oversized_payloads_before_policy() {
        let outcome = invalid_argument_outcome(BTreeMap::from([(
            "fixture_id".to_string(),
            json!("x".repeat(WP10_MAX_STRING_BYTES_V1 + 1)),
        )]));

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("OversizedPayload"));
    }

    #[test]
    fn wp10_rejects_missing_required_arguments_before_policy() {
        let outcome = invalid_argument_outcome(BTreeMap::new());

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("MissingRequiredArgument"));
    }

    #[test]
    fn wp10_rejects_ambiguous_defaults_before_policy() {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        let properties = input.registry.tools[0]
            .uts
            .input_schema
            .keywords
            .get_mut("properties")
            .and_then(JsonValue::as_object_mut)
            .expect("fixture has properties");
        properties
            .get_mut("fixture_id")
            .and_then(JsonValue::as_object_mut)
            .expect("fixture_id has schema")
            .insert("default".to_string(), json!("fixture-default"));
        input.proposal.arguments.clear();

        let outcome = compile_uts_to_acc_v1(&input);

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("AmbiguousDefault"));
    }

    #[test]
    fn wp10_rejects_omitted_optional_defaults_before_policy() {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        let properties = input.registry.tools[0]
            .uts
            .input_schema
            .keywords
            .get_mut("properties")
            .and_then(JsonValue::as_object_mut)
            .expect("fixture has properties");
        properties.insert(
            "optional_mode".to_string(),
            json!({"type": "string", "default": "implicit"}),
        );

        let outcome = compile_uts_to_acc_v1(&input);

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("AmbiguousDefault"));
    }

    #[test]
    fn wp10_rejects_unexpected_additional_fields_before_policy() {
        let outcome = invalid_argument_outcome(BTreeMap::from([
            ("fixture_id".to_string(), json!("fixture-a")),
            ("extra".to_string(), json!("surprise")),
        ]));

        assert_invalid_argument_rejection(&outcome);
        assert!(serde_json::to_string(&outcome.evidence)
            .expect("evidence")
            .contains("UnexpectedAdditionalField"));
    }

    #[test]
    fn wp10_does_not_echo_secret_like_argument_values_in_rejections() {
        let secret_like = "sk-live-redaction-sensitive-token";
        let outcome = invalid_argument_outcome(BTreeMap::from([
            ("fixture_id".to_string(), json!("fixture-a")),
            ("unexpected_secret".to_string(), json!(secret_like)),
        ]));
        let evidence_json = serde_json::to_string(&outcome.evidence).expect("evidence");
        let rejection_json = serde_json::to_string(&outcome.rejection).expect("rejection");

        assert_invalid_argument_rejection(&outcome);
        assert!(!evidence_json.contains(secret_like));
        assert!(!rejection_json.contains(secret_like));
    }

    #[test]
    fn wp09_checks_every_declared_resource_scope() {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        input.registry.tools[0]
            .uts
            .resources
            .push(UtsResourceRequirementV1 {
                resource_type: "fixture".to_string(),
                scope: "second-denied-scope".to_string(),
            });

        let outcome = compile_uts_to_acc_v1(&input);

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::RejectionEmitted);
        assert_eq!(
            outcome.rejection.as_ref().expect("rejection").code,
            UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied
        );
    }

    #[test]
    fn wp09_rejects_multi_resource_tools_instead_of_truncating_acc_scope() {
        let mut input = wp09_compiler_input_fixture("fixture.safe_read");
        input
            .policy_context
            .allowed_resource_scopes
            .push("second-allowed-scope".to_string());
        input.registry.tools[0]
            .uts
            .resources
            .push(UtsResourceRequirementV1 {
                resource_type: "fixture".to_string(),
                scope: "second-allowed-scope".to_string(),
            });

        let outcome = compile_uts_to_acc_v1(&input);

        assert_eq!(outcome.decision, UtsAccCompilerDecisionV1::RejectionEmitted);
        assert_eq!(
            outcome.rejection.as_ref().expect("rejection").code,
            UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied
        );
    }

    #[test]
    fn wp09_rejection_records_cover_required_constraint_classes() {
        let cases = [
            (
                "authority",
                {
                    let mut input = wp09_compiler_input_fixture("fixture.safe_read");
                    input.policy_context.authenticated = false;
                    input
                },
                UtsAccCompilerRejectionCodeV1::UnsatisfiableAuthority,
            ),
            (
                "resource",
                {
                    let mut input = wp09_compiler_input_fixture("fixture.safe_read");
                    input.policy_context.allowed_resource_scopes = vec!["other".to_string()];
                    input
                },
                UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied,
            ),
            (
                "privacy",
                {
                    let mut input = wp09_compiler_input_fixture("fixture.exfiltrate");
                    input.policy_context.allowed_side_effects =
                        vec![UtsSideEffectClassV1::Exfiltration];
                    input.policy_context.allowed_resource_scopes =
                        vec!["protected-prompt".to_string()];
                    input.policy_context.allow_sensitive_data = false;
                    input
                },
                UtsAccCompilerRejectionCodeV1::PrivacyConstraintUnsatisfied,
            ),
            (
                "visibility",
                {
                    let mut input = wp09_compiler_input_fixture("fixture.safe_read");
                    input.policy_context.visibility_constructible = false;
                    input
                },
                UtsAccCompilerRejectionCodeV1::VisibilityConstraintUnsatisfied,
            ),
            (
                "replay",
                {
                    let mut input = wp09_compiler_input_fixture("fixture.safe_read");
                    input.policy_context.replay_allowed = false;
                    input
                },
                UtsAccCompilerRejectionCodeV1::ReplayConstraintUnsatisfied,
            ),
            (
                "execution",
                {
                    let mut input = wp09_compiler_input_fixture("fixture.safe_read");
                    input.policy_context.execution_approved = false;
                    input
                },
                UtsAccCompilerRejectionCodeV1::ExecutionConstraintUnsatisfied,
            ),
        ];

        for (case_id, input, expected_code) in cases {
            let outcome = compile_uts_to_acc_v1(&input);

            assert_eq!(
                outcome
                    .rejection
                    .as_ref()
                    .unwrap_or_else(|| panic!("{case_id} should reject"))
                    .code,
                expected_code
            );
            assert!(
                stages(&outcome).contains(&UtsAccCompilerEvidenceStageV1::Rejection),
                "{case_id} should emit rejection evidence"
            );
        }
    }
}
