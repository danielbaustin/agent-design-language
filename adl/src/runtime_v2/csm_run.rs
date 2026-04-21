use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_RUN_PACKET_CONTRACT_SCHEMA: &str = "runtime_v2.csm_run_packet_contract.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRunArtifactRequirement {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub path: String,
    pub owner_wp: String,
    pub required_by_wp: String,
    pub must_exist_before_live_run: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRunStage {
    pub stage_id: String,
    pub sequence: u64,
    pub owner_wp: String,
    pub entry_artifact_ref: String,
    pub exit_artifact_ref: String,
    pub required_invariants: Vec<String>,
    pub status_before_wp: String,
    pub proof_obligation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRunReviewTarget {
    pub reviewer_entrypoint: String,
    pub required_artifacts: Vec<String>,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRunPacketContract {
    pub schema_version: String,
    pub contract_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub generated_at_utc: String,
    pub source_refs: Vec<String>,
    pub artifact_requirements: Vec<RuntimeV2CsmRunArtifactRequirement>,
    pub stages: Vec<RuntimeV2CsmRunStage>,
    pub review_target: RuntimeV2CsmRunReviewTarget,
    pub downstream_consumers: Vec<String>,
    pub claim_boundary: String,
}

impl RuntimeV2CsmRunPacketContract {
    pub fn prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
    ) -> Result<Self> {
        manifold.validate()?;
        kernel.validate()?;
        citizens.validate()?;

        let packet = Self {
            schema_version: RUNTIME_V2_CSM_RUN_PACKET_CONTRACT_SCHEMA.to_string(),
            contract_id: "runtime-v2-csm-run-packet-contract-0001".to_string(),
            demo_id: "D2".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
            generated_at_utc: "2026-04-20T00:00:00Z".to_string(),
            source_refs: vec![
                "docs/milestones/v0.90.2/WBS_v0.90.2.md".to_string(),
                "docs/milestones/v0.90.2/DEMO_MATRIX_v0.90.2.md".to_string(),
                "docs/milestones/v0.90.2/RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md"
                    .to_string(),
                manifold.artifact_path.clone(),
                kernel.registry.registry_path.clone(),
                kernel.state.service_state_path.clone(),
                citizens.active_index.index_path.clone(),
                citizens.pending_index.index_path.clone(),
            ],
            artifact_requirements: vec![
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "run_packet_contract".to_string(),
                    artifact_kind: "contract".to_string(),
                    path: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    owner_wp: "WP-03".to_string(),
                    required_by_wp: "WP-04".to_string(),
                    must_exist_before_live_run: true,
                    purpose: "stable contract for the first bounded CSM run packet".to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "run_packet_fixture".to_string(),
                    artifact_kind: "fixture".to_string(),
                    path: "runtime_v2/csm_run/proto-csm-01-run-packet.json".to_string(),
                    owner_wp: "WP-03".to_string(),
                    required_by_wp: "WP-05".to_string(),
                    must_exist_before_live_run: true,
                    purpose: "fixture definition for the first live proto-csm-01 run".to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "invariant_map".to_string(),
                    artifact_kind: "invariant_map".to_string(),
                    path: "runtime_v2/invariants/csm_run_invariant_map.json".to_string(),
                    owner_wp: "WP-04".to_string(),
                    required_by_wp: "WP-05".to_string(),
                    must_exist_before_live_run: true,
                    purpose: "expanded invariant map before runtime work widens".to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "violation_schema".to_string(),
                    artifact_kind: "violation_schema".to_string(),
                    path: "runtime_v2/violations/violation_artifact_schema.json".to_string(),
                    owner_wp: "WP-04".to_string(),
                    required_by_wp: "WP-08".to_string(),
                    must_exist_before_live_run: true,
                    purpose: "stable negative-path artifact shape for invalid action rejection"
                        .to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "boot_manifest".to_string(),
                    artifact_kind: "runtime_artifact".to_string(),
                    path: "runtime_v2/csm_run/boot_manifest.json".to_string(),
                    owner_wp: "WP-05".to_string(),
                    required_by_wp: "WP-14".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "live manifold boot evidence for the integrated proof".to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "first_run_trace".to_string(),
                    artifact_kind: "trace".to_string(),
                    path: "runtime_v2/csm_run/first_run_trace.jsonl".to_string(),
                    owner_wp: "WP-06".to_string(),
                    required_by_wp: "WP-14".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "ordered trace spine for scheduling, mediation, and rejection"
                        .to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "resource_pressure_fixture".to_string(),
                    artifact_kind: "runtime_artifact".to_string(),
                    path: "runtime_v2/csm_run/resource_pressure_fixture.json".to_string(),
                    owner_wp: "WP-06".to_string(),
                    required_by_wp: "WP-14".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "bounded resource-pressure input for the governed episode scheduler"
                        .to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "scheduling_decision".to_string(),
                    artifact_kind: "runtime_artifact".to_string(),
                    path: "runtime_v2/csm_run/scheduling_decision.json".to_string(),
                    owner_wp: "WP-06".to_string(),
                    required_by_wp: "WP-07".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "reviewable scheduler choice before Freedom Gate mediation"
                        .to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "citizen_action_fixture".to_string(),
                    artifact_kind: "runtime_artifact".to_string(),
                    path: "runtime_v2/csm_run/citizen_action_fixture.json".to_string(),
                    owner_wp: "WP-07".to_string(),
                    required_by_wp: "WP-08".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "scheduled non-trivial citizen action routed through the Freedom Gate"
                        .to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "freedom_gate_decision".to_string(),
                    artifact_kind: "runtime_artifact".to_string(),
                    path: "runtime_v2/csm_run/freedom_gate_decision.json".to_string(),
                    owner_wp: "WP-07".to_string(),
                    required_by_wp: "WP-08".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "bounded Freedom Gate mediation decision for the scheduled action"
                        .to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "invalid_action_fixture".to_string(),
                    artifact_kind: "runtime_artifact".to_string(),
                    path: "runtime_v2/csm_run/invalid_action_fixture.json".to_string(),
                    owner_wp: "WP-08".to_string(),
                    required_by_wp: "WP-11".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "invalid action input that must be rejected before commit"
                        .to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "invalid_action_violation".to_string(),
                    artifact_kind: "violation_packet".to_string(),
                    path: "runtime_v2/csm_run/invalid_action_violation.json".to_string(),
                    owner_wp: "WP-08".to_string(),
                    required_by_wp: "WP-11".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "stable violation packet proving invalid action rejection without side effects"
                        .to_string(),
                },
                RuntimeV2CsmRunArtifactRequirement {
                    artifact_id: "observatory_packet".to_string(),
                    artifact_kind: "observatory".to_string(),
                    path: "runtime_v2/observatory/visibility_packet.json".to_string(),
                    owner_wp: "WP-10".to_string(),
                    required_by_wp: "WP-14".to_string(),
                    must_exist_before_live_run: false,
                    purpose: "operator-visible projection of the completed bounded run"
                        .to_string(),
                },
            ],
            stages: vec![
                RuntimeV2CsmRunStage {
                    stage_id: "contract_and_fixture".to_string(),
                    sequence: 1,
                    owner_wp: "WP-03".to_string(),
                    entry_artifact_ref: manifold.artifact_path.clone(),
                    exit_artifact_ref: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    required_invariants: vec![
                        "single_active_manifold_instance".to_string(),
                        "no_duplicate_active_citizen_instance".to_string(),
                    ],
                    status_before_wp: "landed_by_wp03".to_string(),
                    proof_obligation:
                        "D2 has a code-backed packet contract before runtime execution widens"
                            .to_string(),
                },
                RuntimeV2CsmRunStage {
                    stage_id: "invariant_and_violation_contract".to_string(),
                    sequence: 2,
                    owner_wp: "WP-04".to_string(),
                    entry_artifact_ref: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    exit_artifact_ref: "runtime_v2/invariants/csm_run_invariant_map.json"
                        .to_string(),
                    required_invariants: vec![
                        "trace_sequence_must_advance_monotonically".to_string(),
                        "snapshot_restore_must_validate_before_active_state".to_string(),
                    ],
                    status_before_wp: "landed_by_wp04".to_string(),
                    proof_obligation:
                        "D2 gains invariant map and violation schema before WP-05 starts"
                            .to_string(),
                },
                RuntimeV2CsmRunStage {
                    stage_id: "boot_and_admission".to_string(),
                    sequence: 3,
                    owner_wp: "WP-05".to_string(),
                    entry_artifact_ref: "runtime_v2/csm_run/proto-csm-01-run-packet.json"
                        .to_string(),
                    exit_artifact_ref: "runtime_v2/csm_run/boot_manifest.json".to_string(),
                    required_invariants: vec![
                        "single_active_manifold_instance".to_string(),
                        "no_duplicate_active_citizen_instance".to_string(),
                    ],
                    status_before_wp: "landed_by_wp05".to_string(),
                    proof_obligation:
                        "proto-csm-01 boots and admits workers only after contract gates pass"
                            .to_string(),
                },
                RuntimeV2CsmRunStage {
                    stage_id: "governed_episode_and_rejection".to_string(),
                    sequence: 4,
                    owner_wp: "WP-06-WP-08".to_string(),
                    entry_artifact_ref: "runtime_v2/csm_run/boot_manifest.json".to_string(),
                    exit_artifact_ref: "runtime_v2/csm_run/first_run_trace.jsonl".to_string(),
                    required_invariants: vec![
                        "trace_sequence_must_advance_monotonically".to_string(),
                        "invalid_action_must_be_refused_before_commit".to_string(),
                    ],
                    status_before_wp: "wp08_invalid_action_rejection_landed".to_string(),
                    proof_obligation:
                        "one resource-pressure episode and one invalid action rejection share the same trace spine"
                            .to_string(),
                },
                RuntimeV2CsmRunStage {
                    stage_id: "snapshot_wake_and_observatory".to_string(),
                    sequence: 5,
                    owner_wp: "WP-09-WP-10".to_string(),
                    entry_artifact_ref: "runtime_v2/csm_run/first_run_trace.jsonl".to_string(),
                    exit_artifact_ref: "runtime_v2/observatory/visibility_packet.json".to_string(),
                    required_invariants: vec![
                        "snapshot_restore_must_validate_before_active_state".to_string(),
                        "no_duplicate_active_citizen_instance".to_string(),
                    ],
                    status_before_wp: "planned".to_string(),
                    proof_obligation:
                        "wake continuity and operator visibility are tied to the same bounded run"
                            .to_string(),
                },
            ],
            review_target: RuntimeV2CsmRunReviewTarget {
                reviewer_entrypoint:
                    "docs/milestones/v0.90.2/CSM_RUN_PACKET_CONTRACT_v0.90.2.md".to_string(),
                required_artifacts: vec![
                    "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    "runtime_v2/csm_run/proto-csm-01-run-packet.json".to_string(),
                    "runtime_v2/invariants/csm_run_invariant_map.json".to_string(),
                    "runtime_v2/violations/violation_artifact_schema.json".to_string(),
                    "runtime_v2/csm_run/resource_pressure_fixture.json".to_string(),
                    "runtime_v2/csm_run/scheduling_decision.json".to_string(),
                    "runtime_v2/csm_run/citizen_action_fixture.json".to_string(),
                    "runtime_v2/csm_run/freedom_gate_decision.json".to_string(),
                    "runtime_v2/csm_run/invalid_action_fixture.json".to_string(),
                    "runtime_v2/csm_run/invalid_action_violation.json".to_string(),
                    "runtime_v2/csm_run/first_run_trace.jsonl".to_string(),
                ],
                validation_commands: vec![
                    "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_run_packet_contract -- --nocapture".to_string(),
                    "cargo test --manifest-path adl/Cargo.toml runtime_v2_invariant_and_violation_contract -- --nocapture".to_string(),
                    "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_boot_admission -- --nocapture".to_string(),
                    "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_governed_episode -- --nocapture".to_string(),
                    "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_freedom_gate_mediation -- --nocapture".to_string(),
                    "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_invalid_action_rejection -- --nocapture".to_string(),
                    "git diff --check".to_string(),
                ],
                non_claims: vec![
                    "does not execute a live CSM run".to_string(),
                    "does not claim first true Godel-agent birth".to_string(),
                    "does not implement v0.91 moral or emotional civilization scope"
                        .to_string(),
                    "does not implement v0.92 identity or migration semantics".to_string(),
                ],
            },
            downstream_consumers: vec![
                "WP-04 invariant and violation artifact contract".to_string(),
                "WP-05 manifold boot and citizen admission".to_string(),
                "WP-10 Observatory packet and operator report integration".to_string(),
                "WP-14 integrated first CSM run demo".to_string(),
            ],
            claim_boundary:
                "This contract fixes the first bounded CSM run packet shape and review target; it is not a live Runtime v2 execution artifact."
                    .to_string(),
        };

        packet.validate()?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_RUN_PACKET_CONTRACT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM run packet contract schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D2" {
            return Err(anyhow!(
                "Runtime v2 CSM run packet contract must map to demo matrix row D2"
            ));
        }
        normalize_id(self.contract_id.clone(), "csm_run.contract_id")?;
        normalize_id(self.manifold_id.clone(), "csm_run.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_run.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "csm_run.generated_at_utc")?;
        validate_csm_run_source_refs(&self.source_refs)?;
        validate_csm_run_artifact_requirements(&self.artifact_requirements)?;
        validate_csm_run_stages(&self.stages)?;
        self.review_target.validate()?;
        if self.downstream_consumers.len() < 3 {
            return Err(anyhow!(
                "CSM run packet contract must name downstream consumers"
            ));
        }
        for consumer in &self.downstream_consumers {
            validate_nonempty_text(consumer, "csm_run.downstream_consumers")?;
        }
        if !self
            .claim_boundary
            .contains("not a live Runtime v2 execution artifact")
        {
            return Err(anyhow!(
                "CSM run packet contract must preserve its non-live claim boundary"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM run packet contract")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmRunReviewTarget {
    pub fn validate(&self) -> Result<()> {
        validate_relative_path(&self.reviewer_entrypoint, "csm_run.review_target")?;
        if self.required_artifacts.len() < 4 {
            return Err(anyhow!(
                "CSM run review target must name WP-03 and WP-04 required artifacts"
            ));
        }
        for artifact in &self.required_artifacts {
            validate_relative_path(artifact, "csm_run.review_target.required_artifacts")?;
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_csm_run_packet_contract"))
        {
            return Err(anyhow!(
                "CSM run review target must include the focused contract validation command"
            ));
        }
        for command in &self.validation_commands {
            validate_nonempty_text(command, "csm_run.review_target.validation_commands")?;
        }
        if self.non_claims.len() < 3 {
            return Err(anyhow!("CSM run review target must name non-claims"));
        }
        for non_claim in &self.non_claims {
            validate_nonempty_text(non_claim, "csm_run.review_target.non_claims")?;
        }
        Ok(())
    }
}

fn validate_csm_run_source_refs(source_refs: &[String]) -> Result<()> {
    if source_refs.len() < 5 {
        return Err(anyhow!("CSM run packet contract source_refs is too small"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for source_ref in source_refs {
        validate_relative_path(source_ref, "csm_run.source_refs")?;
        if !seen.insert(source_ref.clone()) {
            return Err(anyhow!(
                "CSM run packet contract contains duplicate source_ref"
            ));
        }
    }
    Ok(())
}

fn validate_csm_run_artifact_requirements(
    artifacts: &[RuntimeV2CsmRunArtifactRequirement],
) -> Result<()> {
    let mut seen = std::collections::BTreeSet::new();
    for artifact in artifacts {
        normalize_id(artifact.artifact_id.clone(), "csm_run.artifact_id")?;
        validate_nonempty_text(&artifact.artifact_kind, "csm_run.artifact_kind")?;
        validate_relative_path(&artifact.path, "csm_run.artifact_path")?;
        validate_nonempty_text(&artifact.owner_wp, "csm_run.owner_wp")?;
        validate_nonempty_text(&artifact.required_by_wp, "csm_run.required_by_wp")?;
        validate_nonempty_text(&artifact.purpose, "csm_run.purpose")?;
        if !seen.insert(artifact.artifact_id.clone()) {
            return Err(anyhow!(
                "CSM run packet contract contains duplicate artifact id '{}'",
                artifact.artifact_id
            ));
        }
    }
    let required_ids = [
        "run_packet_contract",
        "run_packet_fixture",
        "invariant_map",
        "violation_schema",
        "boot_manifest",
        "first_run_trace",
        "resource_pressure_fixture",
        "scheduling_decision",
        "citizen_action_fixture",
        "freedom_gate_decision",
        "invalid_action_fixture",
        "invalid_action_violation",
        "observatory_packet",
    ];
    for required_id in required_ids {
        if !seen.contains(required_id) {
            return Err(anyhow!(
                "CSM run packet contract missing required artifact '{required_id}'"
            ));
        }
    }
    if artifacts.len() < 13 {
        return Err(anyhow!(
            "CSM run packet contract must define the first-run artifact set"
        ));
    }
    Ok(())
}

fn validate_csm_run_stages(stages: &[RuntimeV2CsmRunStage]) -> Result<()> {
    if stages.len() < 5 {
        return Err(anyhow!(
            "CSM run packet contract must define the first-run stage sequence"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (index, stage) in stages.iter().enumerate() {
        normalize_id(stage.stage_id.clone(), "csm_run.stage_id")?;
        if stage.sequence != index as u64 + 1 {
            return Err(anyhow!(
                "CSM run packet contract stages must preserve contiguous sequence order"
            ));
        }
        validate_nonempty_text(&stage.owner_wp, "csm_run.stage_owner_wp")?;
        validate_relative_path(
            &stage.entry_artifact_ref,
            "csm_run.stage_entry_artifact_ref",
        )?;
        validate_relative_path(&stage.exit_artifact_ref, "csm_run.stage_exit_artifact_ref")?;
        if stage.required_invariants.is_empty() {
            return Err(anyhow!(
                "CSM run packet contract stages must name required invariants"
            ));
        }
        for invariant in &stage.required_invariants {
            normalize_id(invariant.clone(), "csm_run.stage_required_invariants")?;
        }
        validate_nonempty_text(&stage.status_before_wp, "csm_run.stage_status_before_wp")?;
        validate_nonempty_text(&stage.proof_obligation, "csm_run.stage_proof_obligation")?;
        if !seen.insert(stage.stage_id.clone()) {
            return Err(anyhow!(
                "CSM run packet contract contains duplicate stage id '{}'",
                stage.stage_id
            ));
        }
    }
    Ok(())
}
