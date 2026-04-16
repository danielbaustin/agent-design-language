use serde::{Deserialize, Serialize};

pub const ADVERSARIAL_EXECUTION_RUNNER_SCHEMA: &str = "adversarial_execution_runner.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialRunnerEntrypointContract {
    pub command_shape: String,
    pub required_inputs: Vec<String>,
    pub limit_semantics: Vec<String>,
    pub non_authoritative_until: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialRunnerStageContract {
    pub stage_id: String,
    pub role: String,
    pub purpose: String,
    pub required_inputs: Vec<String>,
    pub required_outputs: Vec<String>,
    pub blocked_in_postures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialRunnerPosturePolicyContract {
    pub posture_profiles: Vec<String>,
    pub freedom_gate_requirements: Vec<String>,
    pub enforcement_rules: Vec<String>,
    pub prohibited_shortcuts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialEvidenceCaptureContract {
    pub artifact_families: Vec<String>,
    pub trace_requirements: Vec<String>,
    pub linkage_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialRunnerReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub required_visibility: Vec<String>,
    pub downstream_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialExecutionRunnerContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub entrypoint: AdversarialRunnerEntrypointContract,
    pub canonical_stages: Vec<AdversarialRunnerStageContract>,
    pub posture_policy: AdversarialRunnerPosturePolicyContract,
    pub evidence_capture: AdversarialEvidenceCaptureContract,
    pub review_surface: AdversarialRunnerReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl AdversarialExecutionRunnerContract {
    pub fn v1() -> Self {
        Self {
            schema_version: ADVERSARIAL_EXECUTION_RUNNER_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::adversarial_execution_runner::AdversarialExecutionRunnerContract"
                    .to_string(),
                "adl::adversarial_execution_runner::AdversarialRunnerStageContract".to_string(),
                "adl::adversarial_execution_runner::AdversarialRunnerPosturePolicyContract"
                    .to_string(),
                "adl::adversarial_execution_runner::AdversarialEvidenceCaptureContract"
                    .to_string(),
                "adl identity adversarial-runner".to_string(),
            ],
            runtime_condition:
                "ADL adversarial execution must be orchestrated as a bounded staged runner with explicit target, posture, role attribution, evidence capture, and defer points."
                    .to_string(),
            entrypoint: AdversarialRunnerEntrypointContract {
                command_shape:
                    "planned: adl adversarial run --target <target_ref> --posture <profile> --goal <string> --limit <n|time> --output <path>"
                        .to_string(),
                required_inputs: vec![
                    "target_ref".to_string(),
                    "posture_profile".to_string(),
                    "goal_or_focus".to_string(),
                    "bounded_limit".to_string(),
                    "artifact_root".to_string(),
                ],
                limit_semantics: vec![
                    "iteration count or duration limit must be declared before execution"
                        .to_string(),
                    "runner stages must emit an explicit defer record instead of silently exceeding limits"
                        .to_string(),
                ],
                non_authoritative_until: vec![
                    "actual `adl adversarial run` CLI dispatch is intentionally deferred beyond this contract"
                        .to_string(),
                    "exploit and replay artifact schemas are owned by WP-05".to_string(),
                ],
            },
            canonical_stages: vec![
                AdversarialRunnerStageContract {
                    stage_id: "load_target".to_string(),
                    role: "purple".to_string(),
                    purpose: "resolve the bounded target reference and declare the review scope"
                        .to_string(),
                    required_inputs: vec!["target_ref".to_string()],
                    required_outputs: vec!["target scope record".to_string()],
                    blocked_in_postures: vec![],
                },
                AdversarialRunnerStageContract {
                    stage_id: "declare_posture".to_string(),
                    role: "purple".to_string(),
                    purpose: "make posture, goal, and limit visible before any adversarial action"
                        .to_string(),
                    required_inputs: vec![
                        "posture_profile".to_string(),
                        "goal_or_focus".to_string(),
                        "bounded_limit".to_string(),
                    ],
                    required_outputs: vec!["posture declaration".to_string()],
                    blocked_in_postures: vec![],
                },
                AdversarialRunnerStageContract {
                    stage_id: "enumerate_surfaces".to_string(),
                    role: "red".to_string(),
                    purpose: "enumerate bounded target surfaces without attempting exploitation"
                        .to_string(),
                    required_inputs: vec!["target scope record".to_string()],
                    required_outputs: vec!["attack-surface inventory".to_string()],
                    blocked_in_postures: vec![],
                },
                AdversarialRunnerStageContract {
                    stage_id: "generate_hypothesis".to_string(),
                    role: "red".to_string(),
                    purpose: "produce one attributable exploit hypothesis for review".to_string(),
                    required_inputs: vec!["attack-surface inventory".to_string()],
                    required_outputs: vec!["exploit hypothesis draft".to_string()],
                    blocked_in_postures: vec![],
                },
                AdversarialRunnerStageContract {
                    stage_id: "attempt_bounded_exploit".to_string(),
                    role: "red".to_string(),
                    purpose: "attempt the declared exploit path only when posture allows it"
                        .to_string(),
                    required_inputs: vec![
                        "exploit hypothesis draft".to_string(),
                        "posture declaration".to_string(),
                    ],
                    required_outputs: vec!["exploit evidence or blocked-action record".to_string()],
                    blocked_in_postures: vec!["audit".to_string()],
                },
                AdversarialRunnerStageContract {
                    stage_id: "evaluate_risk".to_string(),
                    role: "blue".to_string(),
                    purpose: "interpret exploit evidence and decide mitigation posture".to_string(),
                    required_inputs: vec!["exploit evidence or blocked-action record".to_string()],
                    required_outputs: vec!["risk evaluation".to_string()],
                    blocked_in_postures: vec![],
                },
                AdversarialRunnerStageContract {
                    stage_id: "decide_mitigation".to_string(),
                    role: "blue".to_string(),
                    purpose: "produce a mitigation or containment decision without hiding uncertainty"
                        .to_string(),
                    required_inputs: vec!["risk evaluation".to_string()],
                    required_outputs: vec!["mitigation decision".to_string()],
                    blocked_in_postures: vec![],
                },
                AdversarialRunnerStageContract {
                    stage_id: "capture_replay_decision".to_string(),
                    role: "purple".to_string(),
                    purpose: "record whether replay is strict, bounded, or deferred".to_string(),
                    required_inputs: vec![
                        "exploit evidence or blocked-action record".to_string(),
                        "mitigation decision".to_string(),
                    ],
                    required_outputs: vec!["replay decision".to_string()],
                    blocked_in_postures: vec![],
                },
                AdversarialRunnerStageContract {
                    stage_id: "emit_runner_packet".to_string(),
                    role: "purple".to_string(),
                    purpose: "link target, posture, role stages, evidence, and defers into one review packet"
                        .to_string(),
                    required_inputs: vec![
                        "posture declaration".to_string(),
                        "risk evaluation".to_string(),
                        "mitigation decision".to_string(),
                        "replay decision".to_string(),
                    ],
                    required_outputs: vec!["adversarial runner review packet".to_string()],
                    blocked_in_postures: vec![],
                },
            ],
            posture_policy: AdversarialRunnerPosturePolicyContract {
                posture_profiles: vec![
                    "audit".to_string(),
                    "validation".to_string(),
                    "hardening".to_string(),
                    "internal_contest".to_string(),
                ],
                freedom_gate_requirements: vec![
                    "target scope must be declared before red execution".to_string(),
                    "posture must be visible before exploit attempts".to_string(),
                    "mutation-capable stages require explicit posture permission".to_string(),
                ],
                enforcement_rules: vec![
                    "audit posture blocks exploit attempts".to_string(),
                    "no-mutation posture blocks remediation application".to_string(),
                    "limit exhaustion produces an explicit defer record".to_string(),
                ],
                prohibited_shortcuts: vec![
                    "unbounded adversarial loops".to_string(),
                    "exploit attempts without posture declaration".to_string(),
                    "evidence capture without role attribution".to_string(),
                    "silent replay omission".to_string(),
                ],
            },
            evidence_capture: AdversarialEvidenceCaptureContract {
                artifact_families: vec![
                    "target scope record".to_string(),
                    "posture declaration".to_string(),
                    "attack-surface inventory".to_string(),
                    "exploit hypothesis draft".to_string(),
                    "exploit evidence or blocked-action record".to_string(),
                    "risk evaluation".to_string(),
                    "mitigation decision".to_string(),
                    "replay decision".to_string(),
                    "adversarial runner review packet".to_string(),
                ],
                trace_requirements: vec![
                    "every stage must record role, stage_id, posture, and target_ref"
                        .to_string(),
                    "every produced artifact must link back to the stage that produced it"
                        .to_string(),
                    "blocked stages must be preserved as first-class evidence".to_string(),
                ],
                linkage_rules: vec![
                    "mitigation decisions must cite exploit evidence or a blocked-action record"
                        .to_string(),
                    "replay decisions must cite both exploit evidence and mitigation decision when both exist"
                        .to_string(),
                    "runner packet must preserve canonical stage order for review".to_string(),
                ],
            },
            review_surface: AdversarialRunnerReviewSurfaceContract {
                required_questions: vec![
                    "what target, posture, goal, and limit governed the run".to_string(),
                    "which red, blue, and purple stages executed or were blocked".to_string(),
                    "what evidence was captured and how is it linked across stages".to_string(),
                    "which replay or schema details are explicitly deferred downstream".to_string(),
                ],
                required_visibility: vec![
                    "target scope and posture declaration".to_string(),
                    "canonical stage order and role attribution".to_string(),
                    "evidence capture and blocked-action records".to_string(),
                    "defer boundaries for exploit/replay schemas".to_string(),
                ],
                downstream_boundaries: vec![
                    "canonical exploit artifact schemas remain WP-05".to_string(),
                    "replay manifests and strict replay execution remain WP-05".to_string(),
                    "continuous verification and self-attack loops remain WP-06".to_string(),
                    "flagship adversarial demo entry points remain WP-07".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "adl::adversarial_execution_runner::AdversarialExecutionRunnerContract::v1"
                    .to_string(),
                "adl identity adversarial-runner --out .adl/state/adversarial_execution_runner_v1.json".to_string(),
            ],
            proof_hook_command:
                "adl identity adversarial-runner --out .adl/state/adversarial_execution_runner_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/adversarial_execution_runner_v1.json".to_string(),
            scope_boundary:
                "bounded runner orchestration and evidence-capture contract only; concrete exploit schemas, replay manifests, continuous self-attack scheduling, and flagship demos remain downstream milestone work."
                    .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AdversarialExecutionRunnerContract;

    #[test]
    fn adversarial_execution_runner_contract_is_staged_bounded_and_reviewable() {
        let contract = AdversarialExecutionRunnerContract::v1();

        assert_eq!(contract.schema_version, "adversarial_execution_runner.v1");
        assert!(contract
            .owned_runtime_surfaces
            .iter()
            .any(|surface| surface == "adl identity adversarial-runner"));
        assert_eq!(
            contract
                .canonical_stages
                .first()
                .map(|stage| stage.stage_id.as_str()),
            Some("load_target")
        );
        assert_eq!(
            contract
                .canonical_stages
                .last()
                .map(|stage| stage.stage_id.as_str()),
            Some("emit_runner_packet")
        );
        assert!(contract.canonical_stages.iter().any(|stage| {
            stage.stage_id == "attempt_bounded_exploit"
                && stage
                    .blocked_in_postures
                    .iter()
                    .any(|posture| posture == "audit")
        }));
        assert!(contract
            .posture_policy
            .enforcement_rules
            .iter()
            .any(|rule| rule == "limit exhaustion produces an explicit defer record"));
        assert!(contract
            .evidence_capture
            .linkage_rules
            .iter()
            .any(|rule| rule.contains("mitigation decisions must cite exploit evidence")));
        assert!(contract
            .review_surface
            .downstream_boundaries
            .iter()
            .any(|boundary| boundary.contains("WP-05")));
        assert!(contract
            .scope_boundary
            .contains("downstream milestone work"));
    }
}
