use serde::{Deserialize, Serialize};

pub const CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA: &str =
    "continuous_verification_self_attack.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuousVerificationCadenceContract {
    pub supported_modes: Vec<String>,
    pub trigger_requirements: Vec<String>,
    pub scheduling_guarantees: Vec<String>,
    pub noise_controls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerificationSurfaceSelectionContract {
    pub eligible_surface_types: Vec<String>,
    pub selection_requirements: Vec<String>,
    pub posture_constraints: Vec<String>,
    pub prohibited_targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuousVerificationStageContract {
    pub stage_id: String,
    pub role: String,
    pub purpose: String,
    pub required_inputs: Vec<String>,
    pub required_outputs: Vec<String>,
    pub blocking_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelfAttackLayerContract {
    pub layer_id: String,
    pub purpose: String,
    pub required_artifacts: Vec<String>,
    pub policy_bounds: Vec<String>,
    pub evidence_expectations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuousVerificationArtifactPackageContract {
    pub required_artifacts: Vec<String>,
    pub lifecycle_linkage_rules: Vec<String>,
    pub replay_requirements: Vec<String>,
    pub promotion_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuousVerificationPolicyContract {
    pub posture_profiles: Vec<String>,
    pub freedom_gate_rules: Vec<String>,
    pub mutation_rules: Vec<String>,
    pub limit_rules: Vec<String>,
    pub prohibited_shortcuts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuousVerificationReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub required_visibility: Vec<String>,
    pub downstream_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuousVerificationSelfAttackContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub upstream_contracts: Vec<String>,
    pub cadence: ContinuousVerificationCadenceContract,
    pub surface_selection: VerificationSurfaceSelectionContract,
    pub lifecycle: Vec<ContinuousVerificationStageContract>,
    pub self_attack_layers: Vec<SelfAttackLayerContract>,
    pub artifact_package: ContinuousVerificationArtifactPackageContract,
    pub policy: ContinuousVerificationPolicyContract,
    pub review_surface: ContinuousVerificationReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

impl ContinuousVerificationSelfAttackContract {
    pub fn v1() -> Self {
        Self {
            schema_version: CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA.to_string(),
            owned_runtime_surfaces: strings(&[
                "adl::continuous_verification_self_attack::ContinuousVerificationSelfAttackContract",
                "adl::continuous_verification_self_attack::ContinuousVerificationStageContract",
                "adl::continuous_verification_self_attack::SelfAttackLayerContract",
                "adl::continuous_verification_self_attack::ContinuousVerificationPolicyContract",
                "adl identity continuous-verification",
            ]),
            runtime_condition:
                "ADL adversarial verification must run as a bounded, posture-governed, replay-linked loop rather than as occasional narrative red-team prose."
                    .to_string(),
            upstream_contracts: strings(&[
                "adversarial_runtime_model.v1",
                "red_blue_agent_architecture.v1",
                "adversarial_execution_runner.v1",
                "exploit_artifact_replay.v1",
            ]),
            cadence: ContinuousVerificationCadenceContract {
                supported_modes: strings(&[
                    "on_demand",
                    "scheduled",
                    "change_triggered",
                    "continuous_bounded",
                ]),
                trigger_requirements: strings(&[
                    "each run declares trigger source before target selection",
                    "change-triggered runs cite the changed surface or dependency",
                    "scheduled runs declare cadence and stop condition",
                    "continuous-bounded runs declare iteration, time, or budget limits",
                ]),
                scheduling_guarantees: strings(&[
                    "identical inputs preserve lifecycle order",
                    "limit exhaustion emits an explicit defer artifact",
                    "cadence never grants additional target or mutation authority",
                ]),
                noise_controls: strings(&[
                    "hypothesis generation must be prioritized by posture, target criticality, and prior evidence",
                    "duplicate findings link to prior exploit knowledge instead of creating uncorrelated churn",
                    "low-confidence speculation remains a hypothesis artifact rather than proof",
                ]),
            },
            surface_selection: VerificationSurfaceSelectionContract {
                eligible_surface_types: strings(&[
                    "input parser",
                    "workflow state transition",
                    "policy boundary",
                    "identity or permission check",
                    "artifact validation surface",
                    "trace or replay integrity surface",
                    "provider or tool integration boundary",
                ]),
                selection_requirements: strings(&[
                    "target_ref is required before adversarial hypothesis generation",
                    "security_posture_ref is required before any exploit attempt",
                    "authorization basis is recorded in the verification plan artifact",
                    "selected surfaces are trace-visible and reviewable",
                ]),
                posture_constraints: strings(&[
                    "audit posture may enumerate and hypothesize but blocks exploit attempts",
                    "validation posture may attempt bounded non-mutating exploit paths",
                    "hardening posture may prioritize mitigation and replay validation",
                    "internal_contest posture may increase exploration only inside declared internal targets",
                ]),
                prohibited_targets: strings(&[
                    "arbitrary external systems",
                    "live production targets without explicit authorization",
                    "targets lacking a stable target_ref",
                    "targets whose posture forbids the requested action",
                ]),
            },
            lifecycle: vec![
                ContinuousVerificationStageContract {
                    stage_id: "select_verification_surface".to_string(),
                    role: "purple".to_string(),
                    purpose: "choose an eligible bounded surface and record why it is in scope"
                        .to_string(),
                    required_inputs: strings(&["trigger source", "posture profile"]),
                    required_outputs: strings(&["VerificationPlanArtifact"]),
                    blocking_rules: strings(&[
                        "missing target_ref blocks red execution",
                        "missing authorization basis blocks self-attack",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "declare_cadence_and_posture".to_string(),
                    role: "purple".to_string(),
                    purpose: "make cadence, posture, goal, and limits visible before the loop proceeds"
                        .to_string(),
                    required_inputs: strings(&[
                        "VerificationPlanArtifact",
                        "bounded_limit",
                        "security_posture_ref",
                    ]),
                    required_outputs: strings(&["posture and cadence declaration"]),
                    blocking_rules: strings(&[
                        "continuous mode without limit produces explicit defer",
                        "posture mismatch blocks the requested stage",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "enumerate_attack_surface".to_string(),
                    role: "red".to_string(),
                    purpose: "list bounded attack surfaces without yet claiming exploitability"
                        .to_string(),
                    required_inputs: strings(&["VerificationPlanArtifact"]),
                    required_outputs: strings(&["attack-surface inventory"]),
                    blocking_rules: strings(&[
                        "empty eligible surface set records no-op evidence instead of fabricating findings",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "generate_exploit_hypothesis".to_string(),
                    role: "red".to_string(),
                    purpose: "produce attributable exploit hypotheses with preconditions and uncertainty"
                        .to_string(),
                    required_inputs: strings(&["attack-surface inventory"]),
                    required_outputs: strings(&["ExploitHypothesisArtifact"]),
                    blocking_rules: strings(&[
                        "hypothesis cannot omit expected unsafe outcome",
                        "hypothesis cannot omit required preconditions",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "analyze_preconditions".to_string(),
                    role: "red".to_string(),
                    purpose: "decide whether the hypothesis is reachable under current target and posture"
                        .to_string(),
                    required_inputs: strings(&["ExploitHypothesisArtifact", "posture declaration"]),
                    required_outputs: strings(&["precondition analysis"]),
                    blocking_rules: strings(&[
                        "missing preconditions create a blocked-action record",
                        "unmet policy requirements block exploit attempt",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "attempt_bounded_exploit".to_string(),
                    role: "red".to_string(),
                    purpose: "attempt the declared path only when preconditions and posture permit it"
                        .to_string(),
                    required_inputs: strings(&["precondition analysis", "ExploitHypothesisArtifact"]),
                    required_outputs: strings(&["ExploitEvidenceArtifact or blocked-action record"]),
                    blocking_rules: strings(&[
                        "audit posture blocks this stage",
                        "mutation requires explicit mutation authority",
                        "limit exhaustion produces explicit defer",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "capture_evidence".to_string(),
                    role: "purple".to_string(),
                    purpose: "capture observed result, trace anchors, and uncertainty as durable evidence"
                        .to_string(),
                    required_inputs: strings(&["exploit attempt result or blocked-action record"]),
                    required_outputs: strings(&["ExploitEvidenceArtifact"]),
                    blocking_rules: strings(&[
                        "narrative-only result is rejected",
                        "missing trace_refs require explicit limitation",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "classify_result".to_string(),
                    role: "blue".to_string(),
                    purpose: "classify severity, reproducibility, and mitigation complexity without overwriting evidence"
                        .to_string(),
                    required_inputs: strings(&["ExploitEvidenceArtifact"]),
                    required_outputs: strings(&["ExploitClassificationArtifact"]),
                    blocking_rules: strings(&[
                        "ambiguous evidence may defer classification but cannot claim proof",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "package_replay".to_string(),
                    role: "purple".to_string(),
                    purpose: "define deterministic, bounded-variance, or best-effort replay requirements"
                        .to_string(),
                    required_inputs: strings(&["ExploitEvidenceArtifact", "ExploitClassificationArtifact"]),
                    required_outputs: strings(&["AdversarialReplayManifest"]),
                    blocking_rules: strings(&[
                        "non-replayable evidence must disclose limitations",
                        "deterministic replay requires stable inputs and expected outcome",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "link_mitigation".to_string(),
                    role: "blue".to_string(),
                    purpose: "link the defensive response or explicit defer to exploit evidence"
                        .to_string(),
                    required_inputs: strings(&["ExploitEvidenceArtifact", "AdversarialReplayManifest"]),
                    required_outputs: strings(&["MitigationLinkageArtifact"]),
                    blocking_rules: strings(&[
                        "mitigation cannot be claimed without evidence linkage",
                        "auto-application requires explicit posture permission",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "validate_replay".to_string(),
                    role: "blue".to_string(),
                    purpose: "validate mitigation or residual risk against replay expectations"
                        .to_string(),
                    required_inputs: strings(&["MitigationLinkageArtifact", "AdversarialReplayManifest"]),
                    required_outputs: strings(&["ReplayValidationArtifact"]),
                    blocking_rules: strings(&[
                        "patch-without-proof records residual risk",
                        "replay omission requires explicit defer reason",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "promote_learning".to_string(),
                    role: "purple".to_string(),
                    purpose: "turn verified exploit knowledge into durable future proof surfaces"
                        .to_string(),
                    required_inputs: strings(&["ReplayValidationArtifact", "MitigationLinkageArtifact"]),
                    required_outputs: strings(&["RegressionPromotionArtifact"]),
                    blocking_rules: strings(&[
                        "promotion cannot omit evidence and mitigation provenance",
                        "deferred promotion must include a defer reason",
                    ]),
                },
                ContinuousVerificationStageContract {
                    stage_id: "emit_verification_packet".to_string(),
                    role: "purple".to_string(),
                    purpose: "emit the ordered review packet for the full verification or self-attack loop"
                        .to_string(),
                    required_inputs: strings(&[
                        "VerificationPlanArtifact",
                        "ExploitEvidenceArtifact",
                        "ReplayValidationArtifact",
                        "RegressionPromotionArtifact",
                    ]),
                    required_outputs: strings(&["SelfAttackReviewPacket"]),
                    blocking_rules: strings(&[
                        "packet must preserve executed, blocked, and deferred stages",
                    ]),
                },
            ],
            self_attack_layers: vec![
                SelfAttackLayerContract {
                    layer_id: "target_selection".to_string(),
                    purpose: "prevent indiscriminate self-probing by declaring authorized targets"
                        .to_string(),
                    required_artifacts: strings(&["VerificationPlanArtifact"]),
                    policy_bounds: strings(&[
                        "target allowlist required",
                        "scope and authorization are review-visible",
                    ]),
                    evidence_expectations: strings(&["target identity survives into trace and artifacts"]),
                },
                SelfAttackLayerContract {
                    layer_id: "adversarial_hypothesis".to_string(),
                    purpose: "generate candidate failure paths with attribution and uncertainty"
                        .to_string(),
                    required_artifacts: strings(&["ExploitHypothesisArtifact"]),
                    policy_bounds: strings(&[
                        "hypotheses remain non-executing until posture permits action",
                    ]),
                    evidence_expectations: strings(&[
                        "preconditions, unsafe outcome, confidence, and uncertainty are explicit",
                    ]),
                },
                SelfAttackLayerContract {
                    layer_id: "bounded_exploit".to_string(),
                    purpose: "convert suspicion into evidence without exceeding declared posture"
                        .to_string(),
                    required_artifacts: strings(&["ExploitEvidenceArtifact"]),
                    policy_bounds: strings(&[
                        "exploit attempts require posture permission",
                        "mutation authority is explicit and narrow",
                    ]),
                    evidence_expectations: strings(&[
                        "success, failure, ambiguity, and policy blocking are distinct outcomes",
                    ]),
                },
                SelfAttackLayerContract {
                    layer_id: "defensive_response".to_string(),
                    purpose: "choose mitigation, containment, or defer while preserving residual risk"
                        .to_string(),
                    required_artifacts: strings(&["MitigationLinkageArtifact"]),
                    policy_bounds: strings(&[
                        "auto-remediation is not implied by mitigation planning",
                    ]),
                    evidence_expectations: strings(&[
                        "defensive claims cite exploit evidence and accepted tradeoffs",
                    ]),
                },
                SelfAttackLayerContract {
                    layer_id: "replay_validation".to_string(),
                    purpose: "prove or limit defensive success using replay expectations".to_string(),
                    required_artifacts: strings(&["AdversarialReplayManifest", "ReplayValidationArtifact"]),
                    policy_bounds: strings(&[
                        "non-replayable cases remain explicit rather than hidden",
                    ]),
                    evidence_expectations: strings(&[
                        "validation result records success, failure, uncertainty, or defer",
                    ]),
                },
                SelfAttackLayerContract {
                    layer_id: "learning_promotion".to_string(),
                    purpose: "make verified adversarial knowledge cumulative instead of episodic"
                        .to_string(),
                    required_artifacts: strings(&["RegressionPromotionArtifact"]),
                    policy_bounds: strings(&[
                        "promotion target must be named before claiming durable learning",
                    ]),
                    evidence_expectations: strings(&[
                        "promotion links exploit, mitigation, replay outcome, and future proof surface",
                    ]),
                },
            ],
            artifact_package: ContinuousVerificationArtifactPackageContract {
                required_artifacts: strings(&[
                    "VerificationPlanArtifact",
                    "ExploitHypothesisArtifact",
                    "ExploitEvidenceArtifact",
                    "ExploitClassificationArtifact",
                    "AdversarialReplayManifest",
                    "MitigationLinkageArtifact",
                    "ReplayValidationArtifact",
                    "RegressionPromotionArtifact",
                    "SelfAttackReviewPacket",
                ]),
                lifecycle_linkage_rules: strings(&[
                    "VerificationPlanArtifact -> ExploitHypothesisArtifact via target_ref and security_posture_ref",
                    "ExploitHypothesisArtifact -> ExploitEvidenceArtifact via hypothesis_ref",
                    "ExploitEvidenceArtifact -> AdversarialReplayManifest via evidence_ref",
                    "ExploitEvidenceArtifact -> MitigationLinkageArtifact via evidence_ref",
                    "MitigationLinkageArtifact -> ReplayValidationArtifact via mitigation_ref and replay_id",
                    "ReplayValidationArtifact -> RegressionPromotionArtifact via validation_result_ref",
                ]),
                replay_requirements: strings(&[
                    "each proven exploit must produce replay requirements or a non-replayable limitation",
                    "post-mitigation validation must cite the replay manifest when replay is available",
                    "replay result preserves unsafe_state_reached before and after mitigation",
                ]),
                promotion_requirements: strings(&[
                    "promotion target names a regression test, replay suite, hardening rule, provider warning, or posture rule",
                    "promotion records why the knowledge should be permanent",
                    "deferred promotion records owner, reason, and expected future surface",
                ]),
            },
            policy: ContinuousVerificationPolicyContract {
                posture_profiles: strings(&[
                    "audit",
                    "validation",
                    "hardening",
                    "internal_contest",
                ]),
                freedom_gate_rules: strings(&[
                    "target scope must be declared before self-attack",
                    "posture must be declared before exploit attempts",
                    "allowed exploit classes are limited by the active posture",
                    "human review requirement is explicit when posture or mutation authority demands it",
                ]),
                mutation_rules: strings(&[
                    "default posture is no live mutation",
                    "mutation requires explicit posture_permission and mutation_boundary",
                    "auto-remediation may emit a mitigation artifact but does not apply changes unless separately authorized",
                ]),
                limit_rules: strings(&[
                    "continuous loops require iteration, duration, or budget limits",
                    "limit exhaustion records defer status instead of continuing silently",
                    "scheduler cadence cannot widen target scope or exploit authority",
                ]),
                prohibited_shortcuts: strings(&[
                    "unbounded continuous verification",
                    "self-attack without target allowlist",
                    "exploit attempt without posture declaration",
                    "narrative-only exploit or mitigation claims",
                    "patch-without-proof when replay is available",
                    "promotion without evidence and mitigation provenance",
                ]),
            },
            review_surface: ContinuousVerificationReviewSurfaceContract {
                required_questions: strings(&[
                    "what triggered the verification loop and which cadence governed it",
                    "which target and posture bounded self-attack behavior",
                    "which lifecycle stages executed, blocked, or deferred",
                    "what exploit evidence, replay expectation, mitigation link, and promotion decision resulted",
                    "what residual risk remains explicit",
                ]),
                required_visibility: strings(&[
                    "cadence and trigger source",
                    "target_ref, security_posture_ref, and authorization basis",
                    "ordered lifecycle stage results",
                    "artifact linkage from hypothesis through promotion",
                    "defer, limitation, and residual-risk records",
                ]),
                downstream_boundaries: strings(&[
                    "flagship adversarial demo wiring remains WP-07",
                    "operational skill composition remains WP-08",
                    "provider extension and packaging convergence remain WP-10",
                    "full continuous scheduler and exploit runner CLI remain future executable tooling beyond this contract",
                ]),
            },
            proof_fixture_hooks: strings(&[
                "adl::continuous_verification_self_attack::ContinuousVerificationSelfAttackContract::v1",
                "adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json",
            ]),
            proof_hook_command:
                "adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json"
                    .to_string(),
            proof_hook_output_path:
                ".adl/state/continuous_verification_self_attack_v1.json".to_string(),
            scope_boundary:
                "bounded continuous-verification and self-attack contract only; flagship demos, operational skill composition, and full scheduler or exploit runner execution remain future executable tooling beyond this contract."
                    .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContinuousVerificationSelfAttackContract;

    #[test]
    fn continuous_verification_contract_covers_cadence_lifecycle_and_artifacts() {
        let contract = ContinuousVerificationSelfAttackContract::v1();

        assert_eq!(
            contract.schema_version,
            "continuous_verification_self_attack.v1"
        );
        assert!(contract
            .owned_runtime_surfaces
            .iter()
            .any(|surface| surface == "adl identity continuous-verification"));
        assert_eq!(
            contract
                .cadence
                .supported_modes
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec![
                "on_demand",
                "scheduled",
                "change_triggered",
                "continuous_bounded"
            ]
        );
        assert_eq!(
            contract
                .lifecycle
                .first()
                .map(|stage| stage.stage_id.as_str()),
            Some("select_verification_surface")
        );
        assert_eq!(
            contract
                .lifecycle
                .last()
                .map(|stage| stage.stage_id.as_str()),
            Some("emit_verification_packet")
        );
        assert!(contract
            .lifecycle
            .iter()
            .any(|stage| stage.stage_id == "attempt_bounded_exploit"
                && stage
                    .blocking_rules
                    .iter()
                    .any(|rule| rule == "audit posture blocks this stage")));
        assert!(contract
            .artifact_package
            .required_artifacts
            .iter()
            .any(|artifact| artifact == "ReplayValidationArtifact"));
        assert!(contract
            .artifact_package
            .promotion_requirements
            .iter()
            .any(|rule| rule.contains("regression test")));
    }

    #[test]
    fn self_attack_contract_is_policy_bounded_and_reviewable() {
        let contract = ContinuousVerificationSelfAttackContract::v1();

        assert_eq!(
            contract.upstream_contracts,
            vec![
                "adversarial_runtime_model.v1",
                "red_blue_agent_architecture.v1",
                "adversarial_execution_runner.v1",
                "exploit_artifact_replay.v1",
            ]
        );
        assert_eq!(
            contract
                .self_attack_layers
                .iter()
                .map(|layer| layer.layer_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "target_selection",
                "adversarial_hypothesis",
                "bounded_exploit",
                "defensive_response",
                "replay_validation",
                "learning_promotion",
            ]
        );
        assert!(contract
            .surface_selection
            .prohibited_targets
            .iter()
            .any(|target| target == "arbitrary external systems"));
        assert!(contract
            .policy
            .prohibited_shortcuts
            .iter()
            .any(|shortcut| shortcut == "self-attack without target allowlist"));
        assert!(contract
            .policy
            .mutation_rules
            .iter()
            .any(|rule| rule.contains("does not apply changes unless separately authorized")));
        assert!(contract
            .review_surface
            .downstream_boundaries
            .iter()
            .any(|boundary| boundary.contains("WP-07")));
        assert!(contract
            .scope_boundary
            .contains("future executable tooling"));
    }
}
