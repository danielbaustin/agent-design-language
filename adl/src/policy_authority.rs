use crate::acc::AccGrantStatusV1;
use crate::uts::{UtsDataSensitivityV1, UtsExecutionEnvironmentKindV1};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAuthorityDecisionV1 {
    Allowed,
    Denied,
    Deferred,
    Challenged,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAuthorityEvidenceKindV1 {
    ContextValidation,
    RoleStanding,
    Grant,
    Delegation,
    Environment,
    Sensitivity,
    Resource,
    Adapter,
    Decision,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyAuthorityEvidenceRecordV1 {
    pub kind: PolicyAuthorityEvidenceKindV1,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyAuthorityContextV1 {
    pub actor_id: String,
    pub role: String,
    pub standing: String,
    pub grant_id: String,
    pub grant_status: AccGrantStatusV1,
    pub delegation_depth: u8,
    pub environment: UtsExecutionEnvironmentKindV1,
    pub sensitivity: UtsDataSensitivityV1,
    pub resource_scope: String,
    pub adapter_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyAuthorityConstraintsV1 {
    pub allowed_roles: Vec<String>,
    pub allowed_standings: Vec<String>,
    pub max_delegation_depth: u8,
    pub allowed_environments: Vec<UtsExecutionEnvironmentKindV1>,
    pub allowed_sensitivities: Vec<UtsDataSensitivityV1>,
    pub allowed_resource_scopes: Vec<String>,
    pub allowed_adapter_ids: Vec<String>,
    pub defer_for_environment_approval: bool,
    pub require_human_challenge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PolicyAuthorityEvaluationInputV1 {
    #[serde(default)]
    pub context: Option<PolicyAuthorityContextV1>,
    pub constraints: PolicyAuthorityConstraintsV1,
    #[serde(default)]
    pub model_confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyAuthorityEvaluationV1 {
    pub decision: PolicyAuthorityDecisionV1,
    pub evidence: Vec<PolicyAuthorityEvidenceRecordV1>,
}

fn evidence(
    kind: PolicyAuthorityEvidenceKindV1,
    detail: impl Into<String>,
) -> PolicyAuthorityEvidenceRecordV1 {
    PolicyAuthorityEvidenceRecordV1 {
        kind,
        detail: detail.into(),
    }
}

fn final_decision(
    decision: PolicyAuthorityDecisionV1,
    mut evidence_log: Vec<PolicyAuthorityEvidenceRecordV1>,
) -> PolicyAuthorityEvaluationV1 {
    evidence_log.push(evidence(
        PolicyAuthorityEvidenceKindV1::Decision,
        format!("{decision:?}"),
    ));
    PolicyAuthorityEvaluationV1 {
        decision,
        evidence: evidence_log,
    }
}

fn token_like(value: &str) -> bool {
    !value.trim().is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.')
        })
}

pub fn evaluate_policy_authority_v1(
    input: &PolicyAuthorityEvaluationInputV1,
) -> PolicyAuthorityEvaluationV1 {
    let mut evidence_log = vec![evidence(
        PolicyAuthorityEvidenceKindV1::ContextValidation,
        "policy authority evaluation started",
    )];

    let Some(context) = input.context.as_ref() else {
        evidence_log.push(evidence(
            PolicyAuthorityEvidenceKindV1::ContextValidation,
            "missing policy context",
        ));
        return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
    };

    if !token_like(&context.actor_id)
        || !token_like(&context.grant_id)
        || !token_like(&context.resource_scope)
        || !token_like(&context.adapter_id)
    {
        evidence_log.push(evidence(
            PolicyAuthorityEvidenceKindV1::ContextValidation,
            "policy context contains non-normalized identifiers",
        ));
        return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
    }

    evidence_log.push(evidence(
        PolicyAuthorityEvidenceKindV1::RoleStanding,
        "role and standing evaluated",
    ));
    if !input.constraints.allowed_roles.contains(&context.role)
        || !input
            .constraints
            .allowed_standings
            .contains(&context.standing)
    {
        return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
    }

    evidence_log.push(evidence(
        PolicyAuthorityEvidenceKindV1::Grant,
        "grant status evaluated",
    ));
    match context.grant_status {
        AccGrantStatusV1::Revoked => {
            return final_decision(PolicyAuthorityDecisionV1::Revoked, evidence_log);
        }
        AccGrantStatusV1::Denied => {
            return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
        }
        AccGrantStatusV1::Active | AccGrantStatusV1::Delegated => {}
    }

    evidence_log.push(evidence(
        PolicyAuthorityEvidenceKindV1::Delegation,
        "delegation depth evaluated",
    ));
    if context.delegation_depth > input.constraints.max_delegation_depth {
        return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
    }

    evidence_log.push(evidence(
        PolicyAuthorityEvidenceKindV1::Environment,
        "environment constraints evaluated",
    ));
    if !input
        .constraints
        .allowed_environments
        .contains(&context.environment)
    {
        return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
    }

    evidence_log.push(evidence(
        PolicyAuthorityEvidenceKindV1::Sensitivity,
        "sensitivity constraints evaluated",
    ));
    if !input
        .constraints
        .allowed_sensitivities
        .contains(&context.sensitivity)
    {
        return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
    }

    evidence_log.push(evidence(
        PolicyAuthorityEvidenceKindV1::Resource,
        "resource constraints evaluated",
    ));
    if !input
        .constraints
        .allowed_resource_scopes
        .contains(&context.resource_scope)
    {
        return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
    }

    evidence_log.push(evidence(
        PolicyAuthorityEvidenceKindV1::Adapter,
        "adapter constraints evaluated",
    ));
    if !input
        .constraints
        .allowed_adapter_ids
        .contains(&context.adapter_id)
    {
        return final_decision(PolicyAuthorityDecisionV1::Denied, evidence_log);
    }

    if input.constraints.defer_for_environment_approval {
        return final_decision(PolicyAuthorityDecisionV1::Deferred, evidence_log);
    }

    if input.constraints.require_human_challenge {
        return final_decision(PolicyAuthorityDecisionV1::Challenged, evidence_log);
    }

    final_decision(PolicyAuthorityDecisionV1::Allowed, evidence_log)
}

pub fn wp11_policy_context_fixture() -> PolicyAuthorityContextV1 {
    PolicyAuthorityContextV1 {
        actor_id: "actor.operator.alice".to_string(),
        role: "operator".to_string(),
        standing: "active".to_string(),
        grant_id: "grant.policy.fixture".to_string(),
        grant_status: AccGrantStatusV1::Active,
        delegation_depth: 0,
        environment: UtsExecutionEnvironmentKindV1::DryRun,
        sensitivity: UtsDataSensitivityV1::Internal,
        resource_scope: "local-readonly".to_string(),
        adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
    }
}

pub fn wp11_policy_constraints_fixture() -> PolicyAuthorityConstraintsV1 {
    PolicyAuthorityConstraintsV1 {
        allowed_roles: vec!["operator".to_string()],
        allowed_standings: vec!["active".to_string()],
        max_delegation_depth: 1,
        allowed_environments: vec![UtsExecutionEnvironmentKindV1::DryRun],
        allowed_sensitivities: vec![UtsDataSensitivityV1::Internal],
        allowed_resource_scopes: vec!["local-readonly".to_string()],
        allowed_adapter_ids: vec!["adapter.fixture.safe_read.dry_run".to_string()],
        defer_for_environment_approval: false,
        require_human_challenge: false,
    }
}

pub fn wp11_policy_input_fixture() -> PolicyAuthorityEvaluationInputV1 {
    PolicyAuthorityEvaluationInputV1 {
        context: Some(wp11_policy_context_fixture()),
        constraints: wp11_policy_constraints_fixture(),
        model_confidence: Some(0.99),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(input: PolicyAuthorityEvaluationInputV1) -> PolicyAuthorityEvaluationV1 {
        evaluate_policy_authority_v1(&input)
    }

    #[test]
    fn wp11_allows_valid_policy_context() {
        let outcome = evaluate(wp11_policy_input_fixture());

        assert_eq!(outcome.decision, PolicyAuthorityDecisionV1::Allowed);
        assert!(outcome
            .evidence
            .iter()
            .any(|entry| entry.kind == PolicyAuthorityEvidenceKindV1::Decision));
    }

    #[test]
    fn wp11_denies_disallowed_role() {
        let mut input = wp11_policy_input_fixture();
        input.context.as_mut().expect("context").role = "viewer".to_string();

        assert_eq!(evaluate(input).decision, PolicyAuthorityDecisionV1::Denied);
    }

    #[test]
    fn wp11_defers_when_environment_approval_is_required() {
        let mut input = wp11_policy_input_fixture();
        input.constraints.defer_for_environment_approval = true;

        assert_eq!(
            evaluate(input).decision,
            PolicyAuthorityDecisionV1::Deferred
        );
    }

    #[test]
    fn wp11_deferral_cannot_bypass_hard_policy_constraints() {
        let mut disallowed_environment = wp11_policy_input_fixture();
        disallowed_environment
            .constraints
            .defer_for_environment_approval = true;
        disallowed_environment
            .context
            .as_mut()
            .expect("context")
            .environment = UtsExecutionEnvironmentKindV1::Network;
        assert_eq!(
            evaluate(disallowed_environment).decision,
            PolicyAuthorityDecisionV1::Denied
        );

        let mut disallowed_sensitivity = wp11_policy_input_fixture();
        disallowed_sensitivity
            .constraints
            .defer_for_environment_approval = true;
        disallowed_sensitivity
            .context
            .as_mut()
            .expect("context")
            .sensitivity = UtsDataSensitivityV1::Secret;
        assert_eq!(
            evaluate(disallowed_sensitivity).decision,
            PolicyAuthorityDecisionV1::Denied
        );

        let mut disallowed_resource = wp11_policy_input_fixture();
        disallowed_resource
            .constraints
            .defer_for_environment_approval = true;
        disallowed_resource
            .context
            .as_mut()
            .expect("context")
            .resource_scope = "other-scope".to_string();
        assert_eq!(
            evaluate(disallowed_resource).decision,
            PolicyAuthorityDecisionV1::Denied
        );

        let mut disallowed_adapter = wp11_policy_input_fixture();
        disallowed_adapter
            .constraints
            .defer_for_environment_approval = true;
        disallowed_adapter
            .context
            .as_mut()
            .expect("context")
            .adapter_id = "adapter.fixture.other.dry_run".to_string();
        assert_eq!(
            evaluate(disallowed_adapter).decision,
            PolicyAuthorityDecisionV1::Denied
        );
    }

    #[test]
    fn wp11_challenges_when_human_challenge_is_required() {
        let mut input = wp11_policy_input_fixture();
        input.constraints.require_human_challenge = true;

        assert_eq!(
            evaluate(input).decision,
            PolicyAuthorityDecisionV1::Challenged
        );
    }

    #[test]
    fn wp11_revokes_revoked_grant() {
        let mut input = wp11_policy_input_fixture();
        input.context.as_mut().expect("context").grant_status = AccGrantStatusV1::Revoked;

        assert_eq!(evaluate(input).decision, PolicyAuthorityDecisionV1::Revoked);
    }

    #[test]
    fn wp11_missing_policy_context_fails_closed() {
        let mut input = wp11_policy_input_fixture();
        input.context = None;

        let outcome = evaluate(input);

        assert_eq!(outcome.decision, PolicyAuthorityDecisionV1::Denied);
        assert!(outcome
            .evidence
            .iter()
            .any(|entry| entry.detail == "missing policy context"));
    }

    #[test]
    fn wp11_model_confidence_does_not_affect_authority() {
        let mut low_confidence = wp11_policy_input_fixture();
        low_confidence.model_confidence = Some(0.01);
        let mut high_confidence = wp11_policy_input_fixture();
        high_confidence.model_confidence = Some(0.99);

        assert_eq!(evaluate(low_confidence), evaluate(high_confidence));
    }

    #[test]
    fn wp11_denies_bad_delegation_resource_sensitivity_and_adapter_constraints() {
        let mut delegated = wp11_policy_input_fixture();
        delegated
            .context
            .as_mut()
            .expect("context")
            .delegation_depth = 9;
        assert_eq!(
            evaluate(delegated).decision,
            PolicyAuthorityDecisionV1::Denied
        );

        let mut sensitive = wp11_policy_input_fixture();
        sensitive.context.as_mut().expect("context").sensitivity = UtsDataSensitivityV1::Secret;
        assert_eq!(
            evaluate(sensitive).decision,
            PolicyAuthorityDecisionV1::Denied
        );

        let mut resource = wp11_policy_input_fixture();
        resource.context.as_mut().expect("context").resource_scope = "other-scope".to_string();
        assert_eq!(
            evaluate(resource).decision,
            PolicyAuthorityDecisionV1::Denied
        );

        let mut adapter = wp11_policy_input_fixture();
        adapter.context.as_mut().expect("context").adapter_id =
            "adapter.fixture.other.dry_run".to_string();
        assert_eq!(
            evaluate(adapter).decision,
            PolicyAuthorityDecisionV1::Denied
        );
    }
}
