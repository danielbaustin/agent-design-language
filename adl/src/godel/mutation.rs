//! Mutation artifact contracts and canonical mutation records.
use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

use super::hypothesis::HypothesisCandidate;

pub const CANONICAL_MUTATION_SCHEMA_NAME: &str = "mutation";
pub const CANONICAL_MUTATION_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationProposal {
    pub id: String,
    pub hypothesis_id: String,
    pub target_surface: String,
    pub bounded_change: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationPlan {
    pub selected_hypothesis_id: String,
    pub proposals: Vec<MutationProposal>,
    pub ordering_rule: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalMutation {
    pub schema_name: String,
    pub schema_version: u32,
    pub mutation_id: String,
    pub experiment_id: String,
    pub hypothesis_id: String,
    pub mutation_type: String,
    pub bounded_scope: Vec<String>,
    pub operations: Vec<CanonicalMutationOperation>,
    pub constraints: CanonicalMutationConstraints,
    pub comparison: CanonicalMutationComparison,
    pub safety: CanonicalMutationSafety,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<CanonicalMutationEvidenceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_plan_ref: Option<CanonicalMutationEvaluationPlanRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CanonicalMutationMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalMutationOperation {
    pub op_id: String,
    pub action: String,
    pub target_surface: String,
    pub target_pointer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_old_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMutationConstraints {
    pub max_operations: u32,
    pub policy_gate_required: bool,
    pub sandbox_required: bool,
    pub allow_create_new_paths: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMutationComparison {
    pub canonical_fingerprint: String,
    pub ordering_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMutationSafety {
    pub allowed_surfaces: Vec<String>,
    pub prohibited_surfaces: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMutationEvidenceRef {
    pub evidence_id: String,
    pub schema_name: String,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMutationEvaluationPlanRef {
    pub plan_id: String,
    pub schema_name: String,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMutationMetadata {
    pub tags: Vec<String>,
    pub created_by: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for MutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_MUTATION_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_MUTATION_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_MUTATION_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for MutationError {}

pub fn propose_mutations(run_id: &str, hypotheses: &[HypothesisCandidate]) -> MutationPlan {
    let mut sorted_hypotheses = hypotheses.to_vec();
    sorted_hypotheses.sort_by(|a, b| a.id.cmp(&b.id));

    let mut proposals = Vec::with_capacity(sorted_hypotheses.len());
    for (idx, hypothesis) in sorted_hypotheses.iter().enumerate() {
        let target_surface = match hypothesis.failure_code.as_str() {
            "tool_failure" => "tool-invocation-config",
            "policy_denied" => "delegation-policy-input",
            "verification_failed" => "verification-gate-input",
            _ => "workflow-step-config",
        };
        proposals.push(MutationProposal {
            id: format!("mut:{run_id}:{}:{idx:02}", hypothesis.failure_code),
            hypothesis_id: hypothesis.id.clone(),
            target_surface: target_surface.to_string(),
            bounded_change: format!(
                "Apply deterministic bounded adjustment for failure_code={} based on hypothesis_id={}.",
                hypothesis.failure_code, hypothesis.id
            ),
        });
    }
    proposals.sort_by(|a, b| a.id.cmp(&b.id));

    let selected_hypothesis_id = sorted_hypotheses
        .first()
        .map(|h| h.id.clone())
        .unwrap_or_else(|| format!("hyp:{run_id}:none:00"));

    MutationPlan {
        selected_hypothesis_id,
        proposals,
        ordering_rule: "proposal order sorted lexicographically by mutation_id".to_string(),
    }
}

pub fn propose_mutation(run_id: &str, hypothesis: &HypothesisCandidate) -> MutationProposal {
    let plan = propose_mutations(run_id, std::slice::from_ref(hypothesis));
    plan.proposals
        .into_iter()
        .next()
        .expect("mutation pipeline must produce at least one proposal")
}

pub fn build_canonical_mutation(
    run_id: &str,
    workflow_id: &str,
    failure_code: &str,
    hypothesis: &HypothesisCandidate,
    proposal: &MutationProposal,
) -> Result<CanonicalMutation, MutationError> {
    if run_id.trim().is_empty() || workflow_id.trim().is_empty() || failure_code.trim().is_empty() {
        return Err(MutationError::Invalid(
            "run_id, workflow_id, and failure_code must be non-empty".to_string(),
        ));
    }

    let descriptor = descriptor_for_proposal(proposal, failure_code);
    let mut bounded_scope = descriptor.bounded_scope;
    bounded_scope.sort();
    bounded_scope.dedup();

    let mut allowed_surfaces = descriptor.allowed_surfaces;
    allowed_surfaces.sort();
    allowed_surfaces.dedup();

    let mut prohibited_surfaces = descriptor.prohibited_surfaces;
    prohibited_surfaces.sort();
    prohibited_surfaces.dedup();

    let operations = vec![CanonicalMutationOperation {
        op_id: descriptor.op_id.to_string(),
        action: descriptor.action.to_string(),
        target_surface: descriptor.target_surface.to_string(),
        target_pointer: descriptor.target_pointer.to_string(),
        value: Some(descriptor.value),
        expected_old_value: descriptor.expected_old_value,
    }];

    let mut tags = vec![
        "deterministic".to_string(),
        "mutation".to_string(),
        "v0.8".to_string(),
        descriptor.tag.to_string(),
    ];
    tags.sort();
    tags.dedup();

    let mutation_id = format!("mut_{}", sanitize_identifier(&proposal.id));
    let experiment_id = format!(
        "exp_{}_{}",
        sanitize_identifier(run_id),
        sanitize_identifier(workflow_id)
    );
    let hypothesis_id = format!("hyp_{}", sanitize_identifier(&hypothesis.id));
    let evidence_id = format!(
        "ev_{}_{}",
        sanitize_identifier(run_id),
        sanitize_identifier(failure_code)
    );
    let plan_id = format!(
        "plan_{}_{}",
        sanitize_identifier(run_id),
        sanitize_identifier(failure_code)
    );

    let fingerprint = canonical_fingerprint(&CanonicalFingerprintInput {
        mutation_id: &mutation_id,
        experiment_id: &experiment_id,
        hypothesis_id: &hypothesis_id,
        mutation_type: descriptor.mutation_type,
        bounded_scope: &bounded_scope,
        operations: &operations,
        allowed_surfaces: &allowed_surfaces,
        prohibited_surfaces: &prohibited_surfaces,
    })?;

    let mutation = CanonicalMutation {
        schema_name: CANONICAL_MUTATION_SCHEMA_NAME.to_string(),
        schema_version: CANONICAL_MUTATION_SCHEMA_VERSION,
        mutation_id,
        experiment_id,
        hypothesis_id,
        mutation_type: descriptor.mutation_type.to_string(),
        bounded_scope,
        operations,
        constraints: CanonicalMutationConstraints {
            max_operations: 1,
            policy_gate_required: true,
            sandbox_required: true,
            allow_create_new_paths: false,
        },
        comparison: CanonicalMutationComparison {
            canonical_fingerprint: fingerprint,
            ordering_key: descriptor.ordering_key.to_string(),
        },
        safety: CanonicalMutationSafety {
            allowed_surfaces,
            prohibited_surfaces,
        },
        evidence_ref: Some(CanonicalMutationEvidenceRef {
            evidence_id,
            schema_name: "canonical_evidence_view".to_string(),
            schema_version: 1,
        }),
        evaluation_plan_ref: Some(CanonicalMutationEvaluationPlanRef {
            plan_id,
            schema_name: "evaluation_plan".to_string(),
            schema_version: 1,
        }),
        notes: Some(proposal.bounded_change.clone()),
        metadata: Some(CanonicalMutationMetadata {
            tags,
            created_by: "godel.candidate.generator".to_string(),
        }),
    };

    validate_canonical_mutation(&mutation)?;
    Ok(mutation)
}

pub fn persist_canonical_mutation(
    runs_root: &Path,
    run_id: &str,
    mutation: &CanonicalMutation,
) -> Result<PathBuf, MutationError> {
    validate_canonical_mutation(mutation)?;
    let rel_path = PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join("mutation.v1.json");
    let out_dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&out_dir)
        .map_err(|err| MutationError::Io(format!("create dir failed: {err}")))?;
    let json = serde_json::to_string_pretty(mutation)
        .map_err(|err| MutationError::Serialize(err.to_string()))?;
    fs::write(out_dir.join("mutation.v1.json"), json)
        .map_err(|err| MutationError::Io(format!("write failed: {err}")))?;
    Ok(rel_path)
}

pub fn load_canonical_mutation(path: &Path) -> Result<CanonicalMutation, MutationError> {
    let raw =
        fs::read_to_string(path).map_err(|err| MutationError::Io(format!("read failed: {err}")))?;
    let parsed: CanonicalMutation = serde_json::from_str(&raw)
        .map_err(|err| MutationError::Invalid(format!("parse failed: {err}")))?;
    validate_canonical_mutation(&parsed)?;
    Ok(parsed)
}

pub fn validate_canonical_mutation(mutation: &CanonicalMutation) -> Result<(), MutationError> {
    let repo_root = repo_root_from_manifest()?;
    let schema_path = repo_root
        .join("adl-spec")
        .join("schemas")
        .join("v0.8")
        .join("mutation.v1.json");
    let schema_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&schema_path)
            .map_err(|err| MutationError::Io(format!("read schema failed: {err}")))?,
    )
    .map_err(|err| MutationError::Invalid(format!("parse schema failed: {err}")))?;
    let compiled = JSONSchema::options()
        .compile(&schema_json)
        .map_err(|err| MutationError::Invalid(format!("compile schema failed: {err}")))?;
    let value =
        serde_json::to_value(mutation).map_err(|err| MutationError::Serialize(err.to_string()))?;
    if let Err(errors) = compiled.validate(&value) {
        let first = errors
            .into_iter()
            .next()
            .map(|err| err.to_string())
            .unwrap_or_else(|| "unknown schema validation failure".to_string());
        return Err(MutationError::Invalid(format!(
            "canonical schema validation failed: {first}"
        )));
    }

    require_sorted_unique(
        &mutation.bounded_scope,
        "bounded_scope must be sorted and unique",
    )?;
    require_sorted_unique(
        &mutation.safety.allowed_surfaces,
        "allowed_surfaces must be sorted and unique",
    )?;
    require_sorted_unique(
        &mutation.safety.prohibited_surfaces,
        "prohibited_surfaces must be sorted and unique",
    )?;
    if let Some(metadata) = &mutation.metadata {
        require_sorted_unique(&metadata.tags, "metadata.tags must be sorted and unique")?;
    }

    let op_ids = mutation
        .operations
        .iter()
        .map(|op| op.op_id.as_str())
        .collect::<Vec<_>>();
    let mut sorted_ids = op_ids.clone();
    sorted_ids.sort();
    sorted_ids.dedup();
    if op_ids != sorted_ids {
        return Err(MutationError::Invalid(
            "operations must be sorted lexicographically by op_id and unique".to_string(),
        ));
    }

    if let Some(notes) = &mutation.notes {
        reject_unsafe_text(notes, "notes")?;
    }
    for scope in &mutation.bounded_scope {
        reject_unsafe_text(scope, "bounded_scope")?;
    }

    Ok(())
}

fn require_sorted_unique(values: &[String], msg: &str) -> Result<(), MutationError> {
    let mut sorted = values.to_vec();
    sorted.sort();
    sorted.dedup();
    if values != sorted {
        return Err(MutationError::Invalid(msg.to_string()));
    }
    Ok(())
}

fn reject_unsafe_text(value: &str, field: &str) -> Result<(), MutationError> {
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(MutationError::Invalid(format!(
            "{field} must not contain absolute or traversal paths"
        )));
    }
    if value.len() > 1 && value.as_bytes()[1] == b':' {
        return Err(MutationError::Invalid(format!(
            "{field} must not contain drive-prefixed paths"
        )));
    }
    Ok(())
}

struct CanonicalFingerprintInput<'a> {
    mutation_id: &'a str,
    experiment_id: &'a str,
    hypothesis_id: &'a str,
    mutation_type: &'a str,
    bounded_scope: &'a [String],
    operations: &'a [CanonicalMutationOperation],
    allowed_surfaces: &'a [String],
    prohibited_surfaces: &'a [String],
}

fn canonical_fingerprint(input: &CanonicalFingerprintInput<'_>) -> Result<String, MutationError> {
    let payload = json!({
        "mutation_id": input.mutation_id,
        "experiment_id": input.experiment_id,
        "hypothesis_id": input.hypothesis_id,
        "mutation_type": input.mutation_type,
        "bounded_scope": input.bounded_scope,
        "operations": input.operations,
        "allowed_surfaces": input.allowed_surfaces,
        "prohibited_surfaces": input.prohibited_surfaces
    });
    let bytes =
        serde_json::to_vec(&payload).map_err(|err| MutationError::Serialize(err.to_string()))?;
    let digest = Sha256::digest(bytes);
    Ok(format!("sha256:{digest:x}"))
}

struct MutationDescriptor {
    mutation_type: &'static str,
    bounded_scope: Vec<String>,
    op_id: &'static str,
    action: &'static str,
    target_surface: &'static str,
    target_pointer: &'static str,
    value: serde_json::Value,
    expected_old_value: Option<serde_json::Value>,
    ordering_key: &'static str,
    allowed_surfaces: Vec<String>,
    prohibited_surfaces: Vec<String>,
    tag: &'static str,
}

fn descriptor_for_proposal(proposal: &MutationProposal, failure_code: &str) -> MutationDescriptor {
    match proposal.target_surface.as_str() {
        "tool-invocation-config" => MutationDescriptor {
            mutation_type: "overlay_update",
            bounded_scope: vec!["overlay:run.retry_policy".to_string()],
            op_id: "op_set_retry_max_attempts",
            action: "set",
            target_surface: "workflow_overlay",
            target_pointer: "/run/retry_policy/max_attempts",
            value: json!(2),
            expected_old_value: Some(json!(1)),
            ordering_key: "workflow_overlay.retry_policy.max_attempts",
            allowed_surfaces: vec![
                "evaluation_plan".to_string(),
                "workflow_overlay".to_string(),
            ],
            prohibited_surfaces: vec![
                "artifact_validation_strictness".to_string(),
                "security_envelope".to_string(),
                "signing_trust_policy".to_string(),
            ],
            tag: "retry-policy",
        },
        "delegation-policy-input" => MutationDescriptor {
            mutation_type: "guardrail_adjustment",
            bounded_scope: vec!["delegation_policy:approval_path".to_string()],
            op_id: "op_set_delegation_action",
            action: "set",
            target_surface: "delegation_policy",
            target_pointer: "/rules/0/action",
            value: json!("allow_with_approval"),
            expected_old_value: Some(json!("deny")),
            ordering_key: "delegation_policy.rules.0.action",
            allowed_surfaces: vec!["delegation_policy".to_string()],
            prohibited_surfaces: vec![
                "artifact_validation_strictness".to_string(),
                "security_envelope".to_string(),
                "signing_trust_policy".to_string(),
            ],
            tag: "delegation",
        },
        "verification-gate-input" => MutationDescriptor {
            mutation_type: "evaluation_adjustment",
            bounded_scope: vec!["evaluation_plan:checks".to_string()],
            op_id: "op_set_check_strict",
            action: "set",
            target_surface: "evaluation_plan",
            target_pointer: "/checks/0/strict",
            value: json!(false),
            expected_old_value: Some(json!(true)),
            ordering_key: "evaluation_plan.checks.0.strict",
            allowed_surfaces: vec!["evaluation_plan".to_string()],
            prohibited_surfaces: vec![
                "artifact_validation_strictness".to_string(),
                "security_envelope".to_string(),
                "signing_trust_policy".to_string(),
            ],
            tag: "verification",
        },
        _ => MutationDescriptor {
            mutation_type: "overlay_update",
            bounded_scope: vec!["workflow_overlay:step_timeout".to_string()],
            op_id: "op_set_step_timeout_ms",
            action: "set",
            target_surface: "workflow_overlay",
            target_pointer: "/workflow/steps/0/timeout_ms",
            value: json!(5000),
            expected_old_value: Some(json!(3000)),
            ordering_key: if failure_code == "timeout" {
                "workflow_overlay.step.timeout_ms.timeout"
            } else {
                "workflow_overlay.step.timeout_ms"
            },
            allowed_surfaces: vec!["workflow_overlay".to_string()],
            prohibited_surfaces: vec![
                "artifact_validation_strictness".to_string(),
                "security_envelope".to_string(),
                "signing_trust_policy".to_string(),
            ],
            tag: "workflow-step",
        },
    }
}

fn sanitize_identifier(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        "mutation".to_string()
    } else {
        trimmed.chars().take(96).collect()
    }
}

fn repo_root_from_manifest() -> Result<PathBuf, MutationError> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let Some(repo_root) = manifest.parent() else {
        return Err(MutationError::Invalid(
            "unable to derive repository root from CARGO_MANIFEST_DIR".to_string(),
        ));
    };
    Ok(repo_root.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_hypothesis() -> HypothesisCandidate {
        HypothesisCandidate {
            id: "hyp:run-1:tool_failure:00".to_string(),
            statement: "retrying reduces flaky failures".to_string(),
            failure_code: "tool_failure".to_string(),
            evidence_refs: vec!["runs/run-1/run_status.json".to_string()],
        }
    }

    fn fixture_proposal() -> MutationProposal {
        MutationProposal {
            id: "mut:run-1:tool_failure:00".to_string(),
            hypothesis_id: "hyp:run-1:tool_failure:00".to_string(),
            target_surface: "tool-invocation-config".to_string(),
            bounded_change: "Apply deterministic bounded retry adjustment.".to_string(),
        }
    }

    fn test_tmp_dir(label: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("adl-godel-mutation-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("mkdir test tmp");
        root
    }

    #[test]
    fn propose_mutation_is_deterministic() {
        let hypothesis = fixture_hypothesis();
        let left = propose_mutation("run-1", &hypothesis);
        let right = propose_mutation("run-1", &hypothesis);
        assert_eq!(left, right);
    }

    #[test]
    fn propose_mutations_orders_by_hypothesis_id_deterministically() {
        let hypotheses = vec![
            HypothesisCandidate {
                id: "hyp:run-1:tool_failure:01".to_string(),
                statement: "secondary".to_string(),
                failure_code: "tool_failure".to_string(),
                evidence_refs: vec![],
            },
            fixture_hypothesis(),
        ];

        let plan = propose_mutations("run-1", &hypotheses);
        assert_eq!(plan.selected_hypothesis_id, "hyp:run-1:tool_failure:00");
        assert_eq!(plan.proposals.len(), 2);
        assert!(plan.proposals[0].id < plan.proposals[1].id);
        assert_eq!(plan.proposals[0].target_surface, "tool-invocation-config");
    }

    #[test]
    fn canonical_mutation_round_trip_validates_against_schema() {
        let tmp = test_tmp_dir("round-trip");
        let mutation = build_canonical_mutation(
            "run-1",
            "wf-godel-loop",
            "tool_failure",
            &fixture_hypothesis(),
            &fixture_proposal(),
        )
        .expect("build mutation");
        let rel = persist_canonical_mutation(&tmp, "run-1", &mutation).expect("persist mutation");
        assert_eq!(rel, PathBuf::from("runs/run-1/godel/mutation.v1.json"));
        let loaded = load_canonical_mutation(&tmp.join("run-1/godel/mutation.v1.json"))
            .expect("load mutation");
        assert_eq!(loaded.schema_name, "mutation");
        assert_eq!(loaded.operations.len(), 1);
        assert_eq!(loaded.operations[0].target_surface, "workflow_overlay");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn canonical_mutation_rejects_unsorted_operation_ids_and_unsafe_notes() {
        let mut mutation = build_canonical_mutation(
            "run-1",
            "wf-godel-loop",
            "tool_failure",
            &fixture_hypothesis(),
            &fixture_proposal(),
        )
        .expect("build mutation");
        mutation.operations = vec![
            CanonicalMutationOperation {
                op_id: "op_z_last".to_string(),
                action: "set".to_string(),
                target_surface: "workflow_overlay".to_string(),
                target_pointer: "/run/retry_policy/backoff_ms".to_string(),
                value: Some(json!(100)),
                expected_old_value: Some(json!(0)),
            },
            CanonicalMutationOperation {
                op_id: "op_a_first".to_string(),
                action: "set".to_string(),
                target_surface: "workflow_overlay".to_string(),
                target_pointer: "/run/retry_policy/max_attempts".to_string(),
                value: Some(json!(2)),
                expected_old_value: Some(json!(1)),
            },
        ];
        let err = validate_canonical_mutation(&mutation).expect_err("unsorted ops must fail");
        assert!(err.to_string().contains("operations must be sorted"));

        mutation.operations.sort_by(|a, b| a.op_id.cmp(&b.op_id));
        mutation.notes = Some("/Users/daniel/secret".to_string());
        let err = validate_canonical_mutation(&mutation).expect_err("unsafe notes must fail");
        assert!(err.to_string().contains("notes must not contain absolute"));
    }
}
