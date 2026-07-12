//! Runtime-v2 CodeFriend/adapter obligation boundary for v0.92 handoff.
//!
//! WP-13 does not implement the full CodeFriend v1 external-repo product path.
//! It makes the pre-v0.92 obligation executable: the MVP/birthday path may
//! consume CodeFriend only as a bounded, evidence-backed proof-planning
//! dependency, while adapter v2 proof packaging remains a v0.95-owned surface.

use super::*;
use std::collections::BTreeSet;

pub const RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_SCHEMA: &str =
    "runtime_v2.codefriend_adapter_obligations.v1";
pub const RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_PATH: &str =
    "docs/milestones/v0.91.7/review/codefriend_adapter_obligations_4756/boundary_packet.json";
pub const RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_TEST_MARKER: &str =
    "runtime_v2_codefriend_adapter_obligations";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CodeFriendAdapterObligationsPacket {
    pub schema_version: String,
    pub boundary_id: String,
    pub milestone: String,
    pub wp: String,
    pub issue: String,
    pub artifact_path: String,
    pub pre_v092_posture: String,
    pub smallest_codefriend_v1_proof: Vec<RuntimeV2CodeFriendProofSurface>,
    pub adapter_v2_dependencies: Vec<RuntimeV2CodeFriendAdapterDependency>,
    pub mvp_birthday_consumption: Vec<String>,
    pub v095_handoff: Vec<String>,
    pub required_promotion_gates: Vec<String>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CodeFriendProofSurface {
    pub surface_id: String,
    pub required_state: String,
    pub evidence_ref: String,
    pub consumed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CodeFriendAdapterDependency {
    pub dependency_id: String,
    pub required_state: String,
    pub owner_milestone: String,
    pub blocks_v092: bool,
    pub consequence: String,
}

impl RuntimeV2CodeFriendAdapterObligationsPacket {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_SCHEMA.to_string(),
            boundary_id: "runtime-v2-codefriend-adapter-obligations-4756".to_string(),
            milestone: "v0.91.7".to_string(),
            wp: "WP-13".to_string(),
            issue: "#4756".to_string(),
            artifact_path: RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_PATH.to_string(),
            pre_v092_posture: "proof_planning_boundary_for_v0_92".to_string(),
            smallest_codefriend_v1_proof: vec![
                proof_surface(
                    "repo-review-packet",
                    "retained evidence-bound packet shape",
                    "docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md",
                    "v0.95 CodeFriend external-repo proof packaging",
                ),
                proof_surface(
                    "specialist-review-lanes",
                    "bounded reviewer roles with synthesis and findings truth",
                    "docs/planning/codefriend/CODEFRIEND_SETUP_PLAN.md",
                    "v0.95 CodeFriend external-repo proof packaging",
                ),
                proof_surface(
                    "redaction-publication-gate",
                    "publication safety check before any customer-facing report",
                    "docs/adr/0025-codefriend-review-packet-product-boundary.md",
                    "v0.95 CodeFriend external-repo proof packaging",
                ),
                proof_surface(
                    "human-readable-report",
                    "customer-grade report generated from retained evidence",
                    "docs/milestones/v0.91.2/review/codefriend_productization/product_report_template.md",
                    "v0.95 CodeFriend external-repo proof packaging",
                ),
            ],
            adapter_v2_dependencies: vec![
                adapter_dependency(
                    "external-repo-input-manifest",
                    "path-safe manifest for target repo, evidence roots, skipped surfaces, and redaction scope",
                    "v0.95",
                    false,
                    "v0.92 may cite the need for this manifest but may not claim external-repo execution readiness.",
                ),
                adapter_dependency(
                    "portable-execution-adapter",
                    "adapter v2 maps CodeFriend packet execution onto ADL lifecycle without raw product authority",
                    "v0.95",
                    false,
                    "v0.92 birthday readiness is not blocked; MVP convergence must prove the adapter before product claims.",
                ),
                adapter_dependency(
                    "retained-proof-artifacts",
                    "stable packet, synthesis, redaction, report, and manifest evidence survive review",
                    "v0.95",
                    false,
                    "v0.92 may preserve the handoff only; retained proof is required before CodeFriend v1 claims.",
                ),
                adapter_dependency(
                    "operator-publication-approval",
                    "operator approval gates any public/customer-facing CodeFriend v1 output",
                    "v0.95",
                    false,
                    "No public CodeFriend claim may be inferred from v0.91.7 planning, v0.92 birthday copy, or this boundary packet.",
                ),
            ],
            mvp_birthday_consumption: vec![
                "v0.92 may state that CodeFriend/adapter obligations are tracked and bounded, not complete.".to_string(),
                "v0.92 may route CodeFriend v1 proof packaging to v0.95 MVP convergence.".to_string(),
                "v0.92 may use CodeFriend as product-roadmap context only when public claims preserve human-review and evidence-boundary language.".to_string(),
                "v0.92 may not depend on CodeFriend external-repo execution for birthday readiness.".to_string(),
            ],
            v095_handoff: vec![
                "consume docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md as the complete CodeFriend v1 build plan".to_string(),
                "prove one external-repo review packet through adapter v2".to_string(),
                "retain manifest, skipped-surface, redaction, synthesis, and report artifacts".to_string(),
                "demonstrate human-review boundary and publication approval".to_string(),
                "map the proof into the v0.95 D4b demo candidate".to_string(),
            ],
            required_promotion_gates: vec![
                "tracked_issue".to_string(),
                "bounded_external_repo_fixture".to_string(),
                "adapter_v2_manifest".to_string(),
                "redaction_publication_review".to_string(),
                "retained_proof_artifact".to_string(),
                "human_review_required".to_string(),
                "operator_approval".to_string(),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_codefriend_adapter_obligations".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary: "WP-13 #4756 proves the Runtime v2 CodeFriend/adapter pre-v0.92 obligation boundary. It identifies the smallest CodeFriend v1 proof surfaces, binds adapter v2 dependencies to v0.95 MVP convergence, and allows v0.92 birthday work to consume only bounded handoff truth, not CodeFriend product readiness or external-repo execution.".to_string(),
            non_claims: vec![
                "codefriend_v1_product_complete".to_string(),
                "adapter_v2_implemented".to_string(),
                "external_repo_execution_proven".to_string(),
                "autonomous_code_review_authority".to_string(),
                "customer_publication_ready".to_string(),
                "v0_92_birthday_blocker".to_string(),
                "product_repo_migration_complete".to_string(),
            ],
        }
    }

    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_SCHEMA,
            "codefriend_adapter_obligations.schema_version",
        )?;
        normalize_id(
            self.boundary_id.clone(),
            "codefriend_adapter_obligations.boundary_id",
        )?;
        require_exact(
            &self.milestone,
            "v0.91.7",
            "codefriend_adapter_obligations.milestone",
        )?;
        require_exact(&self.wp, "WP-13", "codefriend_adapter_obligations.wp")?;
        require_exact(&self.issue, "#4756", "codefriend_adapter_obligations.issue")?;
        require_exact(
            &self.artifact_path,
            RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_PATH,
            "codefriend_adapter_obligations.artifact_path",
        )?;
        validate_relative_path(
            &self.artifact_path,
            "codefriend_adapter_obligations.artifact_path",
        )?;
        require_exact(
            &self.pre_v092_posture,
            "proof_planning_boundary_for_v0_92",
            "codefriend_adapter_obligations.pre_v092_posture",
        )?;
        validate_proof_surfaces(&self.smallest_codefriend_v1_proof)?;
        validate_adapter_dependencies(&self.adapter_v2_dependencies)?;
        validate_consumption(&self.mvp_birthday_consumption)?;
        validate_v095_handoff(&self.v095_handoff)?;
        validate_required_gates(&self.required_promotion_gates)?;
        validate_command_list(&self.validation_commands)?;
        ensure_contains(
            &self.claim_boundary,
            "bounded handoff truth",
            "CodeFriend claim boundary must preserve v0.92 handoff-only posture",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "not CodeFriend product readiness or external-repo execution",
            "CodeFriend claim boundary must deny product/external execution claims",
        )?;
        for required in [
            "codefriend_v1_product_complete",
            "adapter_v2_implemented",
            "external_repo_execution_proven",
            "autonomous_code_review_authority",
            "customer_publication_ready",
            "v0_92_birthday_blocker",
            "product_repo_migration_complete",
        ] {
            ensure_contains_in_list(
                &self.non_claims,
                required,
                "CodeFriend boundary non-claims must include every unsafe claim",
            )?;
        }
        ensure_no_duplicates(
            &self.non_claims,
            "codefriend_adapter_obligations.non_claims",
        )?;
        Ok(())
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical
            .smallest_codefriend_v1_proof
            .sort_by(|a, b| a.surface_id.cmp(&b.surface_id));
        canonical
            .adapter_v2_dependencies
            .sort_by(|a, b| a.dependency_id.cmp(&b.dependency_id));
        canonical.mvp_birthday_consumption.sort();
        canonical.v095_handoff.sort();
        canonical.required_promotion_gates.sort();
        canonical.validation_commands.sort();
        canonical.non_claims.sort();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self.canonicalized()?)
            .context("serialize Runtime v2 CodeFriend/adapter obligations packet")
    }
}

pub fn runtime_v2_codefriend_adapter_obligations_contract(
) -> Result<RuntimeV2CodeFriendAdapterObligationsPacket> {
    let packet = RuntimeV2CodeFriendAdapterObligationsPacket::prototype();
    packet.validate()?;
    Ok(packet)
}

fn proof_surface(
    surface_id: &str,
    required_state: &str,
    evidence_ref: &str,
    consumed_by: &str,
) -> RuntimeV2CodeFriendProofSurface {
    RuntimeV2CodeFriendProofSurface {
        surface_id: surface_id.to_string(),
        required_state: required_state.to_string(),
        evidence_ref: evidence_ref.to_string(),
        consumed_by: consumed_by.to_string(),
    }
}

fn adapter_dependency(
    dependency_id: &str,
    required_state: &str,
    owner_milestone: &str,
    blocks_v092: bool,
    consequence: &str,
) -> RuntimeV2CodeFriendAdapterDependency {
    RuntimeV2CodeFriendAdapterDependency {
        dependency_id: dependency_id.to_string(),
        required_state: required_state.to_string(),
        owner_milestone: owner_milestone.to_string(),
        blocks_v092,
        consequence: consequence.to_string(),
    }
}

fn validate_proof_surfaces(surfaces: &[RuntimeV2CodeFriendProofSurface]) -> Result<()> {
    let ids = surfaces
        .iter()
        .map(|surface| surface.surface_id.as_str())
        .collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "human-readable-report",
        "redaction-publication-gate",
        "repo-review-packet",
        "specialist-review-lanes",
    ]);
    if surfaces.len() != required.len() || ids != required {
        return Err(anyhow!(
            "codefriend_adapter_obligations.smallest_codefriend_v1_proof must name the exact four proof surfaces"
        ));
    }
    for surface in surfaces {
        validate_relative_path(
            &surface.evidence_ref,
            "codefriend_adapter_obligations.proof_surface.evidence_ref",
        )?;
        if surface.required_state.trim().is_empty()
            || surface.consumed_by != "v0.95 CodeFriend external-repo proof packaging"
        {
            return Err(anyhow!(
                "codefriend_adapter_obligations proof surfaces must have required_state and v0.95 consumption"
            ));
        }
    }
    Ok(())
}

fn validate_adapter_dependencies(
    dependencies: &[RuntimeV2CodeFriendAdapterDependency],
) -> Result<()> {
    let ids = dependencies
        .iter()
        .map(|dependency| dependency.dependency_id.as_str())
        .collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "external-repo-input-manifest",
        "operator-publication-approval",
        "portable-execution-adapter",
        "retained-proof-artifacts",
    ]);
    if dependencies.len() != required.len() || ids != required {
        return Err(anyhow!(
            "codefriend_adapter_obligations.adapter_v2_dependencies must name the reviewed v0.95 dependency set"
        ));
    }
    for dependency in dependencies {
        require_exact(
            &dependency.owner_milestone,
            "v0.95",
            "codefriend_adapter_obligations.adapter_dependency.owner_milestone",
        )?;
        if dependency.blocks_v092 {
            return Err(anyhow!(
                "codefriend_adapter_obligations adapter v2 dependencies must not block v0.92 birthday readiness"
            ));
        }
        if dependency.required_state.trim().is_empty() || !dependency.consequence.contains("v0.92")
        {
            return Err(anyhow!(
                "codefriend_adapter_obligations adapter dependencies must record required_state and v0.92 consequence"
            ));
        }
    }
    Ok(())
}

fn validate_consumption(consumption: &[String]) -> Result<()> {
    require_fields(
        consumption,
        &[
            "tracked and bounded, not complete",
            "v0.95 MVP convergence",
            "product-roadmap context only",
            "may not depend on CodeFriend external-repo execution",
        ],
        "codefriend_adapter_obligations.mvp_birthday_consumption",
    )
}

fn validate_v095_handoff(handoff: &[String]) -> Result<()> {
    require_fields(
        handoff,
        &[
            "external-repo review packet through adapter v2",
            "complete CodeFriend v1 build plan",
            "manifest, skipped-surface, redaction, synthesis, and report artifacts",
            "human-review boundary and publication approval",
            "v0.95 D4b demo candidate",
        ],
        "codefriend_adapter_obligations.v095_handoff",
    )
}

fn validate_required_gates(gates: &[String]) -> Result<()> {
    let actual = gates.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let required = BTreeSet::from([
        "adapter_v2_manifest",
        "bounded_external_repo_fixture",
        "human_review_required",
        "operator_approval",
        "redaction_publication_review",
        "retained_proof_artifact",
        "tracked_issue",
    ]);
    if gates.len() != required.len() || actual != required {
        return Err(anyhow!(
            "codefriend_adapter_obligations.required_promotion_gates must retain the full gate set"
        ));
    }
    Ok(())
}

fn validate_command_list(commands: &[String]) -> Result<()> {
    require_fields(
        commands,
        &[
            RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_TEST_MARKER,
            "git diff --check",
        ],
        "codefriend_adapter_obligations.validation_commands",
    )?;
    for command in commands {
        if command.trim().is_empty() || command.contains("/Users/") {
            return Err(anyhow!(
                "codefriend_adapter_obligations.validation_commands must be non-empty and path-safe"
            ));
        }
    }
    Ok(())
}

fn require_fields(values: &[String], required: &[&str], field_name: &str) -> Result<()> {
    for needle in required {
        ensure_contains_in_list(values, needle, field_name)?;
    }
    ensure_no_duplicates(values, field_name)?;
    Ok(())
}

fn ensure_no_duplicates(values: &[String], field_name: &str) -> Result<()> {
    let unique = values.iter().collect::<BTreeSet<_>>();
    if unique.len() != values.len() {
        return Err(anyhow!("{field_name} must not contain duplicate entries"));
    }
    Ok(())
}

fn ensure_contains(value: &str, needle: &str, message: &str) -> Result<()> {
    if value.contains(needle) {
        Ok(())
    } else {
        Err(anyhow!("{message}: missing '{needle}'"))
    }
}

fn ensure_contains_in_list(values: &[String], needle: &str, message: &str) -> Result<()> {
    if values.iter().any(|value| value.contains(needle)) {
        Ok(())
    } else {
        Err(anyhow!("{message}: missing '{needle}'"))
    }
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must be '{expected}', got '{actual}'"))
    }
}
