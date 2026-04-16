use serde::{Deserialize, Serialize};

use crate::delegation_refusal_coordination::DELEGATION_REFUSAL_COORDINATION_SCHEMA;
use crate::provider_substrate::PROVIDER_SUBSTRATE_MANIFEST_SCHEMA;
use crate::skill_composition_model::SKILL_COMPOSITION_MODEL_SCHEMA;

pub const PROVIDER_EXTENSION_PACKAGING_SCHEMA: &str = "provider_extension_packaging.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderExtensionScopeDecision {
    pub decision_id: String,
    pub status: String,
    pub rationale: String,
    pub promoted_surfaces: Vec<String>,
    pub non_promoted_inputs: Vec<String>,
    pub required_docs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCapabilityPackagingBoundary {
    pub owned_capabilities: Vec<String>,
    pub owned_runtime_obligations: Vec<String>,
    pub excluded_security_capabilities: Vec<String>,
    pub defer_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MilestonePackagingConvergence {
    pub milestone: String,
    pub work_package: String,
    pub convergence_rule: String,
    pub accepted_inputs: Vec<String>,
    pub explicitly_deferred_inputs: Vec<String>,
    pub reviewer_questions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderExtensionReviewSurface {
    pub required_questions: Vec<String>,
    pub reviewer_visible_artifacts: Vec<String>,
    pub downstream_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderExtensionPackagingContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub upstream_contracts: Vec<String>,
    pub scope_decision: ProviderExtensionScopeDecision,
    pub capability_boundary: ProviderCapabilityPackagingBoundary,
    pub milestone_packaging: MilestonePackagingConvergence,
    pub review_surface: ProviderExtensionReviewSurface,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl ProviderExtensionPackagingContract {
    pub fn v1() -> Self {
        Self {
            schema_version: PROVIDER_EXTENSION_PACKAGING_SCHEMA.to_string(),
            owned_runtime_surfaces: strings(&[
                "adl::provider_extension_packaging::ProviderExtensionPackagingContract",
                "adl::provider_extension_packaging::ProviderExtensionScopeDecision",
                "adl::provider_substrate::ProviderSubstrateV1",
                "adl::provider_substrate::ProviderInvocationTargetV1",
                "adl identity provider-extension-packaging",
            ]),
            runtime_condition:
                "v0.89.1 packages existing provider capability visibility as milestone evidence while explicitly deferring under-authored provider-security expansion."
                    .to_string(),
            upstream_contracts: strings(&[
                PROVIDER_SUBSTRATE_MANIFEST_SCHEMA,
                SKILL_COMPOSITION_MODEL_SCHEMA,
                DELEGATION_REFUSAL_COORDINATION_SCHEMA,
            ]),
            scope_decision: ProviderExtensionScopeDecision {
                decision_id: "v0.89.1.wp10.provider_extension_scope".to_string(),
                status: "accepted_bounded_package".to_string(),
                rationale:
                    "The milestone already owns provider substrate capability visibility and governed handoff packaging; it does not yet own a complete provider-security capabilities extension."
                        .to_string(),
                promoted_surfaces: strings(&[
                    "provider_substrate_manifest.v1",
                    "provider_substrate_schema_v1_json",
                    "ProviderInvocationTargetV1 capability metadata",
                    "v0.89.1 milestone packaging docs",
                    "provider-extension-packaging identity proof hook",
                ]),
                non_promoted_inputs: strings(&[
                    "PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md",
                    "empty provider/security demo notes",
                    "unvalidated provider trust tier proposals",
                ]),
                required_docs: strings(&[
                    "docs/milestones/v0.89.1/README.md",
                    "docs/milestones/v0.89.1/FEATURE_DOCS_v0.89.1.md",
                    "docs/milestones/v0.89.1/DECISIONS_v0.89.1.md",
                    "docs/milestones/v0.89.1/WBS_v0.89.1.md",
                    "docs/milestones/v0.89.1/SPRINT_v0.89.1.md",
                ]),
            },
            capability_boundary: ProviderCapabilityPackagingBoundary {
                owned_capabilities: strings(&[
                    "tool_calling support mode visibility",
                    "structured_json support mode visibility",
                    "semantic_tool_fallback support mode visibility",
                    "provider vendor and transport normalization",
                    "stable model_ref versus provider_model_id distinction",
                ]),
                owned_runtime_obligations: strings(&[
                    "provider capability claims are explicit data, not prose-only claims",
                    "provider invocation targets carry the capability metadata used for review",
                    "local, HTTP, and in-process provider transports remain distinguishable",
                    "fallback semantics remain visible when native provider capability is absent",
                ]),
                excluded_security_capabilities: strings(&[
                    "provider attestation",
                    "provider trust tier scoring",
                    "network posture enforcement",
                    "secret lifecycle enforcement",
                    "provider sandbox policy",
                    "external provider security demo execution",
                ]),
                defer_rules: strings(&[
                    "under-authored provider-security inputs remain supporting notes until authored and tested",
                    "future provider-security work must introduce its own contract, tests, docs, and proof hook",
                    "v0.89.1 may reference deferred provider-security scope only as carry-forward, not as delivered capability",
                ]),
            },
            milestone_packaging: MilestonePackagingConvergence {
                milestone: "v0.89.1".to_string(),
                work_package: "WP-10".to_string(),
                convergence_rule:
                    "Promote bounded provider capability packaging that is already backed by runtime data; keep broad provider-security extension work out of the release claim."
                        .to_string(),
                accepted_inputs: strings(&[
                    "docs/milestones/v0.89.1/FEATURE_DOCS_v0.89.1.md provider substrate source mapping",
                    "docs/milestones/v0.89.1/DECISIONS_v0.89.1.md D-03/D-04 packaging decisions",
                    "docs/milestones/v0.89.1/WBS_v0.89.1.md WP-10 acceptance mapping",
                    "adl/src/provider_substrate.rs capability metadata",
                ]),
                explicitly_deferred_inputs: strings(&[
                    "provider attestation schema",
                    "provider security posture scoring",
                    "provider network egress policy",
                    "secret manager integration",
                    "provider-security external review demo",
                ]),
                reviewer_questions: strings(&[
                    "Can reviewers distinguish shipped provider capability metadata from deferred provider-security extension scope?",
                    "Do milestone docs point to a concrete proof hook for WP-10?",
                    "Are future provider-security obligations visible without inflating v0.89.1 release claims?",
                ]),
            },
            review_surface: ProviderExtensionReviewSurface {
                required_questions: strings(&[
                    "What provider capability facts are shipped in v0.89.1?",
                    "Which provider-security capabilities are explicitly deferred?",
                    "Which docs bind the packaging decision?",
                    "Which command emits the machine-readable proof packet?",
                ]),
                reviewer_visible_artifacts: strings(&[
                    "adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json",
                    "docs/milestones/v0.89.1/FEATURE_DOCS_v0.89.1.md",
                    "docs/milestones/v0.89.1/DECISIONS_v0.89.1.md",
                    "docs/milestones/v0.89.1/WBS_v0.89.1.md",
                    "adl/src/provider_extension_packaging.rs",
                ]),
                downstream_boundaries: strings(&[
                    "WP-11 may package reviewer demos but should not silently claim deferred provider-security enforcement",
                    "WP-12/WP-13 may review and reconcile milestone claims using this boundary",
                    "a later security-extension issue must own provider attestation, trust scoring, and enforcement if promoted",
                ]),
            },
            proof_fixture_hooks: strings(&[
                "identity_provider_extension_packaging_writes_contract_json",
                "provider_extension_packaging_keeps_security_extension_deferred",
                "provider_extension_packaging_binds_existing_provider_substrate",
            ]),
            proof_hook_command:
                "adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/provider_extension_packaging_v1.json".to_string(),
            scope_boundary:
                "This contract resolves v0.89.1 packaging and provider capability visibility; it does not implement new provider-security enforcement."
                    .to_string(),
        }
    }
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_extension_packaging_keeps_security_extension_deferred() {
        let contract = ProviderExtensionPackagingContract::v1();

        assert!(contract
            .scope_decision
            .non_promoted_inputs
            .iter()
            .any(|input| input == "PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md"));
        assert!(contract
            .capability_boundary
            .excluded_security_capabilities
            .iter()
            .any(|capability| capability == "provider attestation"));
        assert!(contract
            .scope_boundary
            .contains("does not implement new provider-security enforcement"));
    }

    #[test]
    fn provider_extension_packaging_binds_existing_provider_substrate() {
        let contract = ProviderExtensionPackagingContract::v1();

        assert!(contract
            .upstream_contracts
            .iter()
            .any(|schema| schema == PROVIDER_SUBSTRATE_MANIFEST_SCHEMA));
        assert!(contract
            .scope_decision
            .promoted_surfaces
            .iter()
            .any(|surface| surface == "provider_substrate_manifest.v1"));
        assert!(contract
            .capability_boundary
            .owned_capabilities
            .iter()
            .any(|capability| capability == "structured_json support mode visibility"));
    }

    #[test]
    fn provider_extension_packaging_exposes_reviewer_proof_hook() {
        let contract = ProviderExtensionPackagingContract::v1();

        assert_eq!(
            contract.proof_hook_command,
            "adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json"
        );
        assert!(contract
            .review_surface
            .reviewer_visible_artifacts
            .iter()
            .any(|artifact| artifact.contains("provider_extension_packaging_v1.json")));
        assert!(contract
            .milestone_packaging
            .reviewer_questions
            .iter()
            .any(|question| question.contains("deferred provider-security extension")));
    }
}
