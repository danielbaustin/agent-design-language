use serde::{Deserialize, Serialize};

use crate::adversarial_execution_runner::ADVERSARIAL_EXECUTION_RUNNER_SCHEMA;
use crate::adversarial_runtime::ADVERSARIAL_RUNTIME_MODEL_SCHEMA;
use crate::continuous_verification_self_attack::CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA;
use crate::delegation_refusal_coordination::DELEGATION_REFUSAL_COORDINATION_SCHEMA;
use crate::exploit_artifact_replay::EXPLOIT_ARTIFACT_REPLAY_SCHEMA;
use crate::operational_skills_substrate::OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA;
use crate::provider_extension_packaging::PROVIDER_EXTENSION_PACKAGING_SCHEMA;
use crate::red_blue_agent_architecture::RED_BLUE_AGENT_ARCHITECTURE_SCHEMA;
use crate::skill_composition_model::SKILL_COMPOSITION_MODEL_SCHEMA;

pub const DEMO_PROOF_ENTRY_POINTS_SCHEMA: &str = "demo_proof_entry_points.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DemoProofEntryPointRow {
    pub demo_id: String,
    pub title: String,
    pub work_packages: Vec<String>,
    pub entry_commands: Vec<String>,
    pub primary_proof_surfaces: Vec<String>,
    pub proof_role: String,
    pub status: String,
    pub determinism_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DemoProofRunbookStep {
    pub step_id: String,
    pub purpose: String,
    pub command: String,
    pub expected_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DemoProofDeferredSurface {
    pub surface: String,
    pub owner: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DemoProofPackage {
    pub package_id: String,
    pub milestone: String,
    pub work_package: String,
    pub package_role: String,
    pub rows: Vec<DemoProofEntryPointRow>,
    pub copy_paste_runbook: Vec<DemoProofRunbookStep>,
    pub deferred_surfaces: Vec<DemoProofDeferredSurface>,
    pub review_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DemoProofEntryPointsContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub upstream_contracts: Vec<String>,
    pub package: DemoProofPackage,
    pub reviewer_questions: Vec<String>,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl DemoProofEntryPointsContract {
    pub fn v1() -> Self {
        Self {
            schema_version: DEMO_PROOF_ENTRY_POINTS_SCHEMA.to_string(),
            owned_runtime_surfaces: strings(&[
                "adl::demo_proof_entry_points::DemoProofEntryPointsContract",
                "adl::demo_proof_entry_points::DemoProofPackage",
                "docs/milestones/v0.89.1/DEMO_MATRIX_v0.89.1.md",
                "adl identity demo-proof-entry-points",
            ]),
            runtime_condition:
                "v0.89.1 exposes reviewer-facing proof entry points for adversarial runtime, exploit replay, continuous verification, skill governance, and bounded integration demo packets."
                    .to_string(),
            upstream_contracts: strings(&[
                ADVERSARIAL_RUNTIME_MODEL_SCHEMA,
                RED_BLUE_AGENT_ARCHITECTURE_SCHEMA,
                ADVERSARIAL_EXECUTION_RUNNER_SCHEMA,
                EXPLOIT_ARTIFACT_REPLAY_SCHEMA,
                CONTINUOUS_VERIFICATION_SELF_ATTACK_SCHEMA,
                OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA,
                SKILL_COMPOSITION_MODEL_SCHEMA,
                DELEGATION_REFUSAL_COORDINATION_SCHEMA,
                PROVIDER_EXTENSION_PACKAGING_SCHEMA,
            ]),
            package: DemoProofPackage {
                package_id: "v0.89.1.wp11.demo_proof_entry_points".to_string(),
                milestone: "v0.89.1".to_string(),
                work_package: "WP-11 / WP-13".to_string(),
                package_role:
                    "Copy/paste reviewer entry-point package for the milestone demo matrix and WP-13 integration packet."
                        .to_string(),
                rows: demo_rows(),
                copy_paste_runbook: runbook_steps(),
                deferred_surfaces: deferred_surfaces(),
                review_boundaries: strings(&[
                    "The package names commands and proof surfaces; it does not execute every heavyweight demo during normal identity-contract validation.",
                    "D8 is landed as a bounded integration demo with a fixture-backed validation command.",
                    "D9 is landed as a bounded manuscript workflow packet without claiming final arXiv submission.",
                    "Provider-security attestation, trust scoring, sandbox policy, and external provider-security demos remain out of v0.89.1 scope unless later work explicitly promotes them.",
                ]),
            },
            reviewer_questions: strings(&[
                "Which command should a reviewer run for each v0.89.1 demo row?",
                "Which rows are already landed, partial, ready, or still planned?",
                "Which proof surfaces are deterministic contract packets versus heavyweight demo runs?",
                "Which later release or provider-security surfaces are intentionally deferred?",
            ]),
            proof_fixture_hooks: strings(&[
                "demo_proof_entry_points_exposes_copy_paste_runtime_and_replay_commands",
                "demo_proof_entry_points_exposes_wp13_integration_demos",
                "identity_demo_proof_entry_points_writes_contract_json",
            ]),
            proof_hook_command:
                "adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/demo_proof_entry_points_v1.json".to_string(),
            scope_boundary:
                "This contract lands WP-11 demo scaffolding plus WP-13 integration rows; it does not replace WP-12 review, later quality gates, release review, or release closeout."
                    .to_string(),
        }
    }
}

fn demo_rows() -> Vec<DemoProofEntryPointRow> {
    vec![
        DemoProofEntryPointRow {
            demo_id: "D1".to_string(),
            title: "Adversarial runtime walkthrough".to_string(),
            work_packages: strings(&["WP-02", "WP-03", "WP-04"]),
            entry_commands: strings(&[
                "adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json",
                "adl identity red-blue-architecture --out .adl/state/red_blue_agent_architecture_v1.json",
                "adl identity adversarial-runner --out .adl/state/adversarial_execution_runner_v1.json",
            ]),
            primary_proof_surfaces: strings(&[
                ".adl/state/adversarial_runtime_model_v1.json",
                ".adl/state/red_blue_agent_architecture_v1.json",
                ".adl/state/adversarial_execution_runner_v1.json",
            ]),
            proof_role:
                "Shows contested runtime assumptions, red/blue/purple role boundaries, and bounded adversarial execution stages."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "Identity contract packets are deterministic; runner execution semantics remain bounded by the contract."
                    .to_string(),
        },
        DemoProofEntryPointRow {
            demo_id: "D2".to_string(),
            title: "Exploit artifact and replay proof".to_string(),
            work_packages: strings(&["WP-05"]),
            entry_commands: strings(&[
                "adl identity exploit-replay --out .adl/state/exploit_artifact_replay_v1.json",
            ]),
            primary_proof_surfaces: strings(&[".adl/state/exploit_artifact_replay_v1.json"]),
            proof_role:
                "Shows exploit artifact family, replay preconditions, expected outcome, and deterministic or bounded-variance replay declaration."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "Replay mode is declared in the contract packet rather than inferred from prose."
                    .to_string(),
        },
        DemoProofEntryPointRow {
            demo_id: "D3".to_string(),
            title: "Continuous verification loop".to_string(),
            work_packages: strings(&["WP-06"]),
            entry_commands: strings(&[
                "adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json",
            ]),
            primary_proof_surfaces: strings(&[
                ".adl/state/continuous_verification_self_attack_v1.json",
            ]),
            proof_role:
                "Shows repeated falsification pressure, exploit hypothesis generation, evidence capture, replay, mitigation, and promotion linkage."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "Repeated bounded inputs preserve lifecycle shape and proof packet structure."
                    .to_string(),
        },
        DemoProofEntryPointRow {
            demo_id: "D4".to_string(),
            title: "Self-attack scenario packet".to_string(),
            work_packages: strings(&["WP-06"]),
            entry_commands: strings(&[
                "adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json",
            ]),
            primary_proof_surfaces: strings(&[
                ".adl/state/continuous_verification_self_attack_v1.json",
            ]),
            proof_role:
                "Shows bounded self-attack layer rules, target/posture scope, evidence requirements, replay linkage, and learning-promotion boundaries."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "Scenario structure remains posture-bounded and replay-legible."
                    .to_string(),
        },
        DemoProofEntryPointRow {
            demo_id: "D5".to_string(),
            title: "Flagship adversarial demo".to_string(),
            work_packages: strings(&["WP-07"]),
            entry_commands: strings(&[
                "adl demo demo-h-v0891-adversarial-self-attack --run --trace --out .adl/reports/adversarial-demo --no-open",
            ]),
            primary_proof_surfaces: strings(&[
                ".adl/reports/adversarial-demo/demo-h-v0891-adversarial-self-attack/review_packet.json",
            ]),
            proof_role:
                "Shows exploit, replay, mitigation, post-fix replay, and regression-promotion in one safe local packet."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "The flagship local demo compares the same bounded request before and after mitigation."
                    .to_string(),
        },
        DemoProofEntryPointRow {
            demo_id: "D6".to_string(),
            title: "Operational skills substrate integration".to_string(),
            work_packages: strings(&["WP-08", "WP-09"]),
            entry_commands: strings(&[
                "adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json",
                "adl identity skill-composition --out .adl/state/skill_composition_model_v1.json",
                "adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json",
            ]),
            primary_proof_surfaces: strings(&[
                ".adl/state/operational_skills_substrate_v1.json",
                ".adl/state/skill_composition_model_v1.json",
                ".adl/state/delegation_refusal_coordination_v1.json",
            ]),
            proof_role:
                "Shows explicit skill invocation, composition, refusal, approval-gate, and coordination governance surfaces."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "Governance outcome taxonomy is deterministic even when future node outputs are stochastic."
                    .to_string(),
        },
        DemoProofEntryPointRow {
            demo_id: "D7".to_string(),
            title: "Reviewer-facing security proof package".to_string(),
            work_packages: strings(&["WP-10", "WP-11", "WP-12", "WP-13"]),
            entry_commands: strings(&[
                "adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json",
                "adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json",
            ]),
            primary_proof_surfaces: strings(&[
                ".adl/state/provider_extension_packaging_v1.json",
                ".adl/state/demo_proof_entry_points_v1.json",
                "artifacts/v0891/wp13_demo_integration/integration_manifest.json",
            ]),
            proof_role:
                "Bundles milestone proof commands, carry-forward boundaries, D8 delight demo, and D9 manuscript workflow packet into one machine-readable reviewer package."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "WP-11 identity packets and the WP-13 integration packet are deterministic; heavyweight child demos remain replayable through their own test commands."
                    .to_string(),
        },
        DemoProofEntryPointRow {
            demo_id: "D8".to_string(),
            title: "Five-Agent Hey Jude MIDI demo".to_string(),
            work_packages: strings(&["WP-08", "WP-09", "WP-10", "WP-13"]),
            entry_commands: strings(&["bash adl/tools/demo_v0891_five_agent_hey_jude.sh"]),
            primary_proof_surfaces: strings(&[
                "artifacts/v0891/five_agent_hey_jude/performance_manifest.json",
                "artifacts/v0891/five_agent_hey_jude/midi_event_log.json",
                "artifacts/v0891/five_agent_hey_jude/provider_participation_summary.json",
                "artifacts/v0891/five_agent_hey_jude/runtime/runs/v0-89-1-five-agent-hey-jude-midi-demo/run_summary.json",
            ]),
            proof_role:
                "Shows one human Layer 8 participant plus four provider voices coordinating through one bounded ADL runtime packet and MIDI cue layer."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "The fixture-backed MVAVE Chocolate event stream preserves cue order and validates the same artifact schema on each run."
                    .to_string(),
        },
        DemoProofEntryPointRow {
            demo_id: "D9".to_string(),
            title: "ArXiv manuscript workflow packet".to_string(),
            work_packages: strings(&["WP-08", "WP-13"]),
            entry_commands: strings(&["bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh"]),
            primary_proof_surfaces: strings(&[
                "artifacts/v0891/arxiv_manuscript_workflow/demo_manifest.json",
                "artifacts/v0891/arxiv_manuscript_workflow/source_packets/source_packet_manifest.json",
                "artifacts/v0891/arxiv_manuscript_workflow/manuscript_status/three_paper_status.json",
                "artifacts/v0891/arxiv_manuscript_workflow/review/review_gates.json",
            ]),
            proof_role:
                "Shows bounded manuscript workflow scaffolding, source packets, review gates, and three-paper status packet shape."
                    .to_string(),
            status: "LANDED".to_string(),
            determinism_note:
                "Packet generation is deterministic and WP-13 preserves the no-submission boundary while making the three-paper status packet reviewable."
                    .to_string(),
        },
    ]
}

fn runbook_steps() -> Vec<DemoProofRunbookStep> {
    vec![
        DemoProofRunbookStep {
            step_id: "runtime-and-roles".to_string(),
            purpose: "Materialize D1 adversarial runtime, role architecture, and runner contract packets."
                .to_string(),
            command:
                "adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json && adl identity red-blue-architecture --out .adl/state/red_blue_agent_architecture_v1.json && adl identity adversarial-runner --out .adl/state/adversarial_execution_runner_v1.json"
                    .to_string(),
            expected_output:
                ".adl/state/adversarial_runtime_model_v1.json, .adl/state/red_blue_agent_architecture_v1.json, .adl/state/adversarial_execution_runner_v1.json"
                    .to_string(),
        },
        DemoProofRunbookStep {
            step_id: "replay-contract".to_string(),
            purpose: "Materialize D2 exploit replay proof packet.".to_string(),
            command:
                "adl identity exploit-replay --out .adl/state/exploit_artifact_replay_v1.json"
                    .to_string(),
            expected_output: ".adl/state/exploit_artifact_replay_v1.json".to_string(),
        },
        DemoProofRunbookStep {
            step_id: "continuous-verification".to_string(),
            purpose: "Materialize D3/D4 continuous verification and self-attack packet."
                .to_string(),
            command:
                "adl identity continuous-verification --out .adl/state/continuous_verification_self_attack_v1.json"
                    .to_string(),
            expected_output: ".adl/state/continuous_verification_self_attack_v1.json".to_string(),
        },
        DemoProofRunbookStep {
            step_id: "flagship-demo".to_string(),
            purpose: "Run D5 flagship adversarial proof demo when heavyweight demo execution is desired."
                .to_string(),
            command:
                "adl demo demo-h-v0891-adversarial-self-attack --run --trace --out .adl/reports/adversarial-demo --no-open"
                    .to_string(),
            expected_output:
                ".adl/reports/adversarial-demo/demo-h-v0891-adversarial-self-attack/review_packet.json"
                    .to_string(),
        },
        DemoProofRunbookStep {
            step_id: "skills-and-governance".to_string(),
            purpose: "Materialize D6 operational skills, composition, and governance packets."
                .to_string(),
            command:
                "adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json && adl identity skill-composition --out .adl/state/skill_composition_model_v1.json && adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json"
                    .to_string(),
            expected_output:
                ".adl/state/operational_skills_substrate_v1.json, .adl/state/skill_composition_model_v1.json, .adl/state/delegation_refusal_coordination_v1.json"
                    .to_string(),
        },
        DemoProofRunbookStep {
            step_id: "reviewer-package".to_string(),
            purpose: "Materialize D7 reviewer-facing package boundaries.".to_string(),
            command:
                "adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json && adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json"
                    .to_string(),
            expected_output:
                ".adl/state/provider_extension_packaging_v1.json, .adl/state/demo_proof_entry_points_v1.json"
                    .to_string(),
        },
        DemoProofRunbookStep {
            step_id: "wp13-integration-packet".to_string(),
            purpose: "Materialize the WP-13 integration package for D7, D8, and D9.".to_string(),
            command: "bash adl/tools/demo_v0891_wp13_demo_integration.sh".to_string(),
            expected_output: "artifacts/v0891/wp13_demo_integration/integration_manifest.json"
                .to_string(),
        },
        DemoProofRunbookStep {
            step_id: "five-agent-midi-demo".to_string(),
            purpose: "Run D8 five-agent Hey Jude MIDI demo when integration demo execution is desired."
                .to_string(),
            command: "bash adl/tools/demo_v0891_five_agent_hey_jude.sh".to_string(),
            expected_output:
                "artifacts/v0891/five_agent_hey_jude/performance_manifest.json".to_string(),
        },
        DemoProofRunbookStep {
            step_id: "arxiv-manuscript-workflow".to_string(),
            purpose: "Run D9 bounded three-paper manuscript workflow packet.".to_string(),
            command: "bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh".to_string(),
            expected_output:
                "artifacts/v0891/arxiv_manuscript_workflow/demo_manifest.json".to_string(),
        },
    ]
}

fn deferred_surfaces() -> Vec<DemoProofDeferredSurface> {
    vec![
        DemoProofDeferredSurface {
            surface: "formal release review and remediation outcomes".to_string(),
            owner: "WP-14 through WP-20".to_string(),
            reason:
                "WP-11 provides demo scaffolding only; later closeout packages own quality gates, review, remediation, and release ceremony."
                    .to_string(),
        },
    ]
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_proof_entry_points_exposes_copy_paste_runtime_and_replay_commands() {
        let contract = DemoProofEntryPointsContract::v1();

        let d1 = contract
            .package
            .rows
            .iter()
            .find(|row| row.demo_id == "D1")
            .expect("D1 row");
        assert_eq!(d1.status, "LANDED");
        assert!(d1
            .entry_commands
            .iter()
            .any(|command| command.contains("adl identity adversarial-runtime")));
        assert!(d1
            .entry_commands
            .iter()
            .any(|command| command.contains("adl identity adversarial-runner")));

        let d2 = contract
            .package
            .rows
            .iter()
            .find(|row| row.demo_id == "D2")
            .expect("D2 row");
        assert_eq!(d2.status, "LANDED");
        assert!(d2
            .primary_proof_surfaces
            .iter()
            .any(|surface| surface == ".adl/state/exploit_artifact_replay_v1.json"));
    }

    #[test]
    fn demo_proof_entry_points_exposes_wp13_integration_demos() {
        let contract = DemoProofEntryPointsContract::v1();

        let d7 = contract
            .package
            .rows
            .iter()
            .find(|row| row.demo_id == "D7")
            .expect("D7 row");
        assert_eq!(d7.status, "LANDED");
        assert!(d7
            .primary_proof_surfaces
            .iter()
            .any(|surface| surface
                == "artifacts/v0891/wp13_demo_integration/integration_manifest.json"));

        let d8 = contract
            .package
            .rows
            .iter()
            .find(|row| row.demo_id == "D8")
            .expect("D8 row");
        assert_eq!(d8.status, "LANDED");
        assert!(d8
            .entry_commands
            .iter()
            .any(|command| command == "bash adl/tools/demo_v0891_five_agent_hey_jude.sh"));
        assert!(d8
            .primary_proof_surfaces
            .iter()
            .any(|surface| surface.ends_with("performance_manifest.json")));

        let d9 = contract
            .package
            .rows
            .iter()
            .find(|row| row.demo_id == "D9")
            .expect("D9 row");
        assert_eq!(d9.status, "LANDED");
        assert!(d9
            .primary_proof_surfaces
            .iter()
            .any(|surface| surface.ends_with("three_paper_status.json")));
        assert!(contract
            .package
            .copy_paste_runbook
            .iter()
            .any(|step| step.step_id == "wp13-integration-packet"));
        assert!(contract.scope_boundary.contains("does not replace WP-12"));
    }

    #[test]
    fn demo_proof_entry_points_binds_upstream_contract_schemas() {
        let contract = DemoProofEntryPointsContract::v1();

        assert!(contract
            .upstream_contracts
            .iter()
            .any(|schema| schema == ADVERSARIAL_RUNTIME_MODEL_SCHEMA));
        assert!(contract
            .upstream_contracts
            .iter()
            .any(|schema| schema == EXPLOIT_ARTIFACT_REPLAY_SCHEMA));
        assert_eq!(
            contract.proof_hook_command,
            "adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json"
        );
    }
}
