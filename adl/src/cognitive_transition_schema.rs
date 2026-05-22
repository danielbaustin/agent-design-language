use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::path::Path;

pub const COGNITIVE_TRANSITION_MANIFEST_SCHEMA: &str = "cognitive_transition_manifest.v1";

pub const COGNITIVE_TRANSITION_LIFECYCLE_STATES: &[&str] = &[
    "planned",
    "bound",
    "in_progress",
    "review_ready",
    "reviewed",
    "merge_ready",
    "merged",
    "closed_out",
    "blocked",
    "superseded",
];

pub const COGNITIVE_TRANSITION_SEED_REQUIRED_ROLES: &[&str] =
    &["operator", "lifecycle_router", "implementation_owner"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct CognitiveTransitionActorRoleRef {
    pub actor_id: String,
    pub role: String,
    pub responsibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct CognitiveTransitionCardPaths {
    pub sip_rel_path: String,
    pub stp_rel_path: String,
    pub spp_rel_path: String,
    pub srp_rel_path: String,
    pub sor_rel_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct CognitiveTransitionManifestV1 {
    pub schema_version: String,
    pub transition_id: String,
    pub issue_number: u64,
    pub issue_url: String,
    pub milestone_version: String,
    pub branch_ref: String,
    pub worktree_ref: String,
    pub lifecycle_state: String,
    pub actor_roles: Vec<CognitiveTransitionActorRoleRef>,
    pub cards: CognitiveTransitionCardPaths,
    pub dag_rel_path: String,
    pub shard_plan_rel_path: Option<String>,
    pub evidence_bundle_rel_path: Option<String>,
    pub merge_readiness_gate_rel_path: Option<String>,
    pub obsmem_handoff_rel_path: Option<String>,
    pub trace_proof_rel_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CognitiveTransitionSchemaError {
    pub code: &'static str,
    pub field: &'static str,
    pub message: String,
}

impl CognitiveTransitionSchemaError {
    fn new(code: &'static str, field: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            field,
            message: message.into(),
        }
    }
}

pub fn cognitive_transition_manifest_v1_schema_json() -> JsonValue {
    serde_json::to_value(schema_for!(CognitiveTransitionManifestV1))
        .expect("cognitive transition schema should serialize")
}

pub fn validate_cognitive_transition_manifest_v1(
    manifest: &CognitiveTransitionManifestV1,
) -> Result<(), CognitiveTransitionSchemaError> {
    if manifest.schema_version != COGNITIVE_TRANSITION_MANIFEST_SCHEMA {
        return Err(CognitiveTransitionSchemaError::new(
            "unsupported_schema_version",
            "schema_version",
            format!("schema_version must be {COGNITIVE_TRANSITION_MANIFEST_SCHEMA}"),
        ));
    }
    if manifest.transition_id.trim().is_empty() {
        return Err(CognitiveTransitionSchemaError::new(
            "missing_transition_id",
            "transition_id",
            "transition_id must be non-empty",
        ));
    }
    if !manifest.issue_url.starts_with("https://github.com/") {
        return Err(CognitiveTransitionSchemaError::new(
            "invalid_issue_url",
            "issue_url",
            "issue_url must be a GitHub HTTPS URL",
        ));
    }
    if manifest.branch_ref.trim().is_empty() || manifest.branch_ref == "main" {
        return Err(CognitiveTransitionSchemaError::new(
            "invalid_branch_ref",
            "branch_ref",
            "branch_ref must identify the transition branch, not main",
        ));
    }
    if !COGNITIVE_TRANSITION_LIFECYCLE_STATES
        .iter()
        .any(|state| *state == manifest.lifecycle_state)
    {
        return Err(CognitiveTransitionSchemaError::new(
            "unsupported_lifecycle_state",
            "lifecycle_state",
            format!(
                "lifecycle_state must be one of {}",
                COGNITIVE_TRANSITION_LIFECYCLE_STATES.join(", ")
            ),
        ));
    }

    validate_repo_relative("worktree_ref", &manifest.worktree_ref)?;
    validate_repo_relative("cards.sip_rel_path", &manifest.cards.sip_rel_path)?;
    validate_repo_relative("cards.stp_rel_path", &manifest.cards.stp_rel_path)?;
    validate_repo_relative("cards.spp_rel_path", &manifest.cards.spp_rel_path)?;
    validate_repo_relative("cards.srp_rel_path", &manifest.cards.srp_rel_path)?;
    validate_repo_relative("cards.sor_rel_path", &manifest.cards.sor_rel_path)?;
    validate_repo_relative("dag_rel_path", &manifest.dag_rel_path)?;

    if let Some(path) = &manifest.shard_plan_rel_path {
        validate_repo_relative("shard_plan_rel_path", path)?;
    }
    if let Some(path) = &manifest.evidence_bundle_rel_path {
        validate_repo_relative("evidence_bundle_rel_path", path)?;
    }
    if let Some(path) = &manifest.merge_readiness_gate_rel_path {
        validate_repo_relative("merge_readiness_gate_rel_path", path)?;
    }
    if let Some(path) = &manifest.obsmem_handoff_rel_path {
        validate_repo_relative("obsmem_handoff_rel_path", path)?;
    }
    for trace_path in &manifest.trace_proof_rel_paths {
        validate_repo_relative("trace_proof_rel_paths", trace_path)?;
    }

    if manifest.actor_roles.is_empty() {
        return Err(CognitiveTransitionSchemaError::new(
            "missing_actor_roles",
            "actor_roles",
            "actor_roles must include the seed transition participants",
        ));
    }
    for role in COGNITIVE_TRANSITION_SEED_REQUIRED_ROLES {
        if !manifest.actor_roles.iter().any(|entry| entry.role == *role) {
            return Err(CognitiveTransitionSchemaError::new(
                "missing_required_actor_role",
                "actor_roles",
                format!("actor_roles must include required seed role `{role}`"),
            ));
        }
    }

    Ok(())
}

pub fn wp02_cognitive_transition_manifest_valid_fixture() -> CognitiveTransitionManifestV1 {
    CognitiveTransitionManifestV1 {
        schema_version: COGNITIVE_TRANSITION_MANIFEST_SCHEMA.to_string(),
        transition_id: "cts.v0_91_3.issue_3200.ct_demo_001".to_string(),
        issue_number: 3200,
        issue_url: "https://github.com/danielbaustin/agent-design-language/issues/3200".to_string(),
        milestone_version: "v0.91.3".to_string(),
        branch_ref: "codex/3200-v0-91-3-wp-02-cognitive-transition-schema".to_string(),
        worktree_ref: ".worktrees/adl-wp-3200".to_string(),
        lifecycle_state: "planned".to_string(),
        actor_roles: vec![
            CognitiveTransitionActorRoleRef {
                actor_id: "human.operator".to_string(),
                role: "operator".to_string(),
                responsibility: "approves scope and sprint progression".to_string(),
            },
            CognitiveTransitionActorRoleRef {
                actor_id: "adl.workflow_conductor".to_string(),
                role: "lifecycle_router".to_string(),
                responsibility: "routes lifecycle stage changes through bounded skills".to_string(),
            },
            CognitiveTransitionActorRoleRef {
                actor_id: "codex.issue_3200".to_string(),
                role: "implementation_owner".to_string(),
                responsibility: "owns the bounded WP-02 implementation slice".to_string(),
            },
        ],
        cards: CognitiveTransitionCardPaths {
            sip_rel_path:
                ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sip.md"
                    .to_string(),
            stp_rel_path:
                ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/stp.md"
                    .to_string(),
            spp_rel_path:
                ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/spp.md"
                    .to_string(),
            srp_rel_path:
                ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/srp.md"
                    .to_string(),
            sor_rel_path:
                ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sor.md"
                    .to_string(),
        },
        dag_rel_path: "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_transition_dag.md"
            .to_string(),
        shard_plan_rel_path: Some(
            "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md".to_string(),
        ),
        evidence_bundle_rel_path: Some(
            "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_evidence_bundle.md"
                .to_string(),
        ),
        merge_readiness_gate_rel_path: Some(
            "docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md".to_string(),
        ),
        obsmem_handoff_rel_path: Some(
            "docs/milestones/v0.91.3/review/obsmem/ct_demo_001_handoff.md".to_string(),
        ),
        trace_proof_rel_paths: vec![
            "workflow/c-sdlc/v0.91.3/trace/ct_demo_001_trace_manifest.json".to_string(),
        ],
    }
}

fn validate_repo_relative(
    field: &'static str,
    candidate: &str,
) -> Result<(), CognitiveTransitionSchemaError> {
    if candidate.trim().is_empty() {
        return Err(CognitiveTransitionSchemaError::new(
            "missing_repo_relative_path",
            field,
            format!("{field} must be non-empty"),
        ));
    }
    let path = Path::new(candidate);
    if path.is_absolute() || candidate.starts_with("../") {
        return Err(CognitiveTransitionSchemaError::new(
            "non_repo_relative_path",
            field,
            format!("{field} must be repository-relative"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const WP04_TRANSITION_DAG_PACKET: &str = include_str!(
        "../../docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_transition_dag.md"
    );
    const WP04_SHARD_PLAN_PACKET: &str = include_str!(
        "../../docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md"
    );

    #[test]
    fn cognitive_transition_manifest_valid_fixture_passes_validation() {
        let manifest = wp02_cognitive_transition_manifest_valid_fixture();

        validate_cognitive_transition_manifest_v1(&manifest)
            .expect("valid fixture should pass validation");
    }

    #[test]
    fn cognitive_transition_manifest_schema_json_exposes_expected_properties() {
        let schema = cognitive_transition_manifest_v1_schema_json();
        let properties = schema
            .get("properties")
            .and_then(|props| props.as_object())
            .or_else(|| {
                schema
                    .get("definitions")
                    .and_then(|defs| defs.get("CognitiveTransitionManifestV1"))
                    .and_then(|entry| entry.get("properties"))
                    .and_then(|props| props.as_object())
            })
            .expect("schema properties should exist");

        for key in [
            "schema_version",
            "transition_id",
            "issue_number",
            "issue_url",
            "branch_ref",
            "worktree_ref",
            "lifecycle_state",
            "actor_roles",
            "cards",
            "dag_rel_path",
        ] {
            assert!(
                properties.contains_key(key),
                "schema missing property {key}"
            );
        }
    }

    #[test]
    fn cognitive_transition_manifest_rejects_unknown_schema_version() {
        let mut manifest = wp02_cognitive_transition_manifest_valid_fixture();
        manifest.schema_version = "cognitive_transition_manifest.v2".to_string();

        let err = validate_cognitive_transition_manifest_v1(&manifest)
            .expect_err("unknown schema version should fail");
        assert_eq!(err.code, "unsupported_schema_version");
        assert_eq!(err.field, "schema_version");
    }

    #[test]
    fn cognitive_transition_manifest_rejects_absolute_paths() {
        let mut manifest = wp02_cognitive_transition_manifest_valid_fixture();
        manifest.cards.sip_rel_path = "/tmp/not-repo-relative/sip.md".to_string();

        let err = validate_cognitive_transition_manifest_v1(&manifest)
            .expect_err("absolute path should fail");
        assert_eq!(err.code, "non_repo_relative_path");
        assert_eq!(err.field, "cards.sip_rel_path");
    }

    #[test]
    fn cognitive_transition_manifest_requires_seed_actor_roles() {
        let mut manifest = wp02_cognitive_transition_manifest_valid_fixture();
        manifest
            .actor_roles
            .retain(|entry| entry.role != "implementation_owner");

        let err = validate_cognitive_transition_manifest_v1(&manifest)
            .expect_err("missing seed role should fail");
        assert_eq!(err.code, "missing_required_actor_role");
        assert_eq!(err.field, "actor_roles");
    }

    #[test]
    fn cognitive_transition_manifest_rejects_unknown_lifecycle_state() {
        let mut manifest = wp02_cognitive_transition_manifest_valid_fixture();
        manifest.lifecycle_state = "nebulous".to_string();

        let err = validate_cognitive_transition_manifest_v1(&manifest)
            .expect_err("unknown lifecycle state should fail");
        assert_eq!(err.code, "unsupported_lifecycle_state");
        assert_eq!(err.field, "lifecycle_state");
    }

    #[test]
    fn cognitive_transition_manifest_fixture_points_at_wp04_transition_packet() {
        let manifest = wp02_cognitive_transition_manifest_valid_fixture();

        assert_eq!(
            manifest.dag_rel_path,
            "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_transition_dag.md"
        );
        assert_eq!(
            manifest.shard_plan_rel_path.as_deref(),
            Some("docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md")
        );

        for snippet in [
            "## Serial Nodes",
            "## Shard Nodes",
            "## Barrier Nodes",
            "barrier.review_barrier",
            "barrier.merge_readiness_barrier",
            "barrier.closeout_barrier",
            "coordination latency",
            "implementation time",
        ] {
            assert!(
                WP04_TRANSITION_DAG_PACKET.contains(snippet),
                "transition DAG packet missing `{snippet}`"
            );
        }

        for snippet in [
            "## Shards",
            "## Interface Freeze Rules",
            "## Handoff Contracts",
            "## Barrier Contracts",
            "## Coordination Metrics Split",
        ] {
            assert!(
                WP04_SHARD_PLAN_PACKET.contains(snippet),
                "shard plan packet missing `{snippet}`"
            );
        }
    }
}
