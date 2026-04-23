//! Runtime-v2 integrated CSM run artifacts and execution recorders.
//!
//! Defines the integrated contract surface used after candidate admission and
//! before steady-state manifold and kernel operations.

use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_INTEGRATED_RUN_PROOF_SCHEMA: &str =
    "runtime_v2.csm_integrated_run_proof_packet.v1";
pub const RUNTIME_V2_CSM_INTEGRATED_RUN_TRANSCRIPT_PATH: &str =
    "runtime_v2/csm_run/integrated_first_run_transcript.jsonl";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmIntegratedRunEvent {
    pub sequence: u32,
    pub stage_id: String,
    pub status: String,
    pub artifact_ref: String,
    pub observatory_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmIntegratedRunProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub run_packet_ref: String,
    pub trace_ref: String,
    pub execution_transcript_ref: String,
    pub observatory_packet_ref: String,
    pub operator_report_ref: String,
    pub recovery_refs: Vec<String>,
    pub quarantine_refs: Vec<String>,
    pub hardening_refs: Vec<String>,
    pub integrated_stage_refs: Vec<String>,
    pub proof_summary: String,
    pub proof_classification: String,
    pub reviewer_entrypoint: String,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmIntegratedRunArtifacts {
    pub run_packet: RuntimeV2CsmRunPacketContract,
    pub invariant_and_violation: RuntimeV2InvariantAndViolationContractArtifacts,
    pub boot_admission: RuntimeV2CsmBootAdmissionArtifacts,
    pub governed_episode: RuntimeV2CsmGovernedEpisodeArtifacts,
    pub freedom_gate: RuntimeV2CsmFreedomGateMediationArtifacts,
    pub invalid_action: RuntimeV2CsmInvalidActionRejectionArtifacts,
    pub wake_continuity: RuntimeV2CsmWakeContinuityArtifacts,
    pub observatory: RuntimeV2CsmObservatoryArtifacts,
    pub recovery: RuntimeV2CsmRecoveryEligibilityArtifacts,
    pub quarantine: RuntimeV2CsmQuarantineArtifacts,
    pub hardening: RuntimeV2CsmHardeningArtifacts,
    pub proof_packet: RuntimeV2CsmIntegratedRunProofPacket,
    pub execution_transcript: Vec<RuntimeV2CsmIntegratedRunEvent>,
}

impl RuntimeV2CsmIntegratedRunArtifacts {
    pub fn prototype() -> Result<Self> {
        let run_packet = runtime_v2_csm_run_packet_contract()?;
        let invariant_and_violation = runtime_v2_invariant_and_violation_contract()?;
        let boot_admission = runtime_v2_csm_boot_admission_contract()?;
        let governed_episode = runtime_v2_csm_governed_episode_contract()?;
        let freedom_gate = runtime_v2_csm_freedom_gate_mediation_contract()?;
        let invalid_action = runtime_v2_csm_invalid_action_rejection_contract()?;
        let wake_continuity = runtime_v2_csm_wake_continuity_contract()?;
        let observatory = runtime_v2_csm_observatory_contract()?;
        let recovery = runtime_v2_csm_recovery_eligibility_contract()?;
        let quarantine = runtime_v2_csm_quarantine_contract()?;
        let hardening = runtime_v2_csm_hardening_contract()?;

        let proof_packet = RuntimeV2CsmIntegratedRunProofPacket {
            schema_version: RUNTIME_V2_CSM_INTEGRATED_RUN_PROOF_SCHEMA.to_string(),
            proof_id: "proto-csm-01-integrated-first-run-proof-0001".to_string(),
            demo_id: "D10".to_string(),
            manifold_id: run_packet.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/integrated_first_run_proof_packet.json"
                .to_string(),
            run_packet_ref: "runtime_v2/csm_run/proto-csm-01-run-packet.json".to_string(),
            trace_ref: governed_episode.first_run_trace_path.clone(),
            execution_transcript_ref: RUNTIME_V2_CSM_INTEGRATED_RUN_TRANSCRIPT_PATH.to_string(),
            observatory_packet_ref: observatory.visibility_packet_path.clone(),
            operator_report_ref: observatory.operator_report_path.clone(),
            recovery_refs: vec![
                recovery.model.artifact_path.clone(),
                recovery.safe_resume_decision.artifact_path.clone(),
                recovery.quarantine_required_decision.artifact_path.clone(),
            ],
            quarantine_refs: vec![
                quarantine.unsafe_recovery_fixture.artifact_path.clone(),
                quarantine.quarantine_artifact.artifact_path.clone(),
                quarantine.evidence_preservation.artifact_path.clone(),
            ],
            hardening_refs: vec![
                hardening.rules.artifact_path.clone(),
                hardening.hook.artifact_path.clone(),
                hardening.duplicate_activation_probe.artifact_path.clone(),
                hardening.snapshot_integrity_probe.artifact_path.clone(),
                hardening.trace_replay_gap_probe.artifact_path.clone(),
                hardening.proof_packet.artifact_path.clone(),
            ],
            integrated_stage_refs: vec![
                run_packet.artifact_path.clone(),
                invariant_and_violation.invariant_map.artifact_path.clone(),
                boot_admission.boot_manifest.artifact_path.clone(),
                governed_episode.first_run_trace_path.clone(),
                freedom_gate.freedom_gate_decision.artifact_path.clone(),
                invalid_action.violation_packet.artifact_path.clone(),
                wake_continuity.wake_continuity_proof.artifact_path.clone(),
                observatory.visibility_packet_path.clone(),
                recovery.safe_resume_decision.artifact_path.clone(),
                quarantine.quarantine_artifact.artifact_path.clone(),
                hardening.proof_packet.artifact_path.clone(),
            ],
            proof_summary:
                "D10 integrates WP-05 through WP-13 into one bounded first CSM run proof packet: boot/admission, governed scheduling, Freedom Gate mediation, invalid-action refusal, snapshot wake continuity, Observatory visibility, recovery/quarantine, and governed hardening."
                    .to_string(),
            proof_classification: "proving".to_string(),
            reviewer_entrypoint:
                "docs/milestones/v0.90.2/CSM_RUN_PACKET_CONTRACT_v0.90.2.md".to_string(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_integrated_run -- --nocapture"
                    .to_string(),
                "adl runtime-v2 integrated-csm-run-demo --out artifacts/v0902/demo-d10-integrated-csm-run"
                    .to_string(),
                "git diff --check".to_string(),
            ],
            non_claims: vec![
                "does not execute an unbounded live CSM run".to_string(),
                "does not claim first true Godel-agent birth".to_string(),
                "does not implement v0.91 moral or emotional civilization scope".to_string(),
                "does not implement v0.92 identity or migration semantics".to_string(),
                "does not implement a complete red/blue/purple security ecology".to_string(),
            ],
            claim_boundary:
                "This packet proves the bounded D10 integrated first-run evidence package; it does not prove first true Godel-agent birth, unbounded live Runtime v2 execution, v0.91 civic substrate, v0.92 identity rebinding, or a complete security ecology."
                    .to_string(),
        };

        let artifacts = Self {
            run_packet,
            invariant_and_violation,
            boot_admission,
            governed_episode,
            freedom_gate,
            invalid_action,
            wake_continuity,
            observatory,
            recovery,
            quarantine,
            hardening,
            proof_packet,
            execution_transcript: integrated_run_transcript(),
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.run_packet.validate()?;
        self.invariant_and_violation.validate()?;
        self.boot_admission.validate()?;
        self.governed_episode.validate()?;
        self.freedom_gate.validate()?;
        self.invalid_action.validate()?;
        self.wake_continuity.validate()?;
        self.observatory.validate()?;
        self.recovery.validate()?;
        self.quarantine.validate()?;
        self.hardening.validate()?;
        validate_integrated_run_transcript(&self.execution_transcript)?;
        self.proof_packet.validate_against_artifacts(self)
    }

    pub fn proof_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.proof_packet)
            .context("serialize Runtime v2 CSM integrated first-run proof packet")
    }

    pub fn observatory_console_markdown(&self) -> Result<String> {
        self.validate()?;
        Ok(format!(
            concat!(
                "# D10 Integrated CSM Run Observatory\n\n",
                "Proof classification: `{}`\n\n",
                "Primary proof packet: `{}`\n\n",
                "Observatory packet: `{}`\n\n",
                "Operator report: `{}`\n\n",
                "{}"
            ),
            self.proof_packet.proof_classification,
            self.proof_packet.artifact_path,
            self.observatory.visibility_packet_path,
            self.observatory.operator_report_path,
            self.observatory.operator_report_markdown
        ))
    }

    pub fn execution_transcript_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::new();
        for event in &self.execution_transcript {
            serde_json::to_writer(&mut out, event).context("serialize integrated CSM run event")?;
            out.push(b'\n');
        }
        Ok(out)
    }

    pub fn execution_summary(&self) -> Result<String> {
        self.validate()?;
        let mut lines = vec![
            "D10 integrated CSM run stage spine:".to_string(),
            format!(
                "- transcript: {}",
                RUNTIME_V2_CSM_INTEGRATED_RUN_TRANSCRIPT_PATH
            ),
        ];
        for event in &self.execution_transcript {
            lines.push(format!(
                "- {:02} {} {} -> {}",
                event.sequence, event.status, event.stage_id, event.artifact_ref
            ));
        }
        Ok(lines.join("\n"))
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.run_packet.write_to_root(root)?;
        self.invariant_and_violation.write_to_root(root)?;
        self.boot_admission.write_to_root(root)?;
        self.governed_episode.write_to_root(root)?;
        self.freedom_gate.write_to_root(root)?;
        self.invalid_action.write_to_root(root)?;
        self.wake_continuity.write_to_root(root)?;
        self.observatory.write_to_root(root)?;
        self.recovery.write_to_root(root)?;
        self.quarantine.write_to_root(root)?;
        self.hardening.write_to_root(root)?;
        write_relative(
            root,
            RUNTIME_V2_CSM_INTEGRATED_RUN_TRANSCRIPT_PATH,
            self.execution_transcript_jsonl_bytes()?,
        )?;
        write_relative(
            root,
            &self.proof_packet.artifact_path,
            self.proof_packet_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmIntegratedRunProofPacket {
    pub fn validate_against_artifacts(
        &self,
        artifacts: &RuntimeV2CsmIntegratedRunArtifacts,
    ) -> Result<()> {
        self.validate()?;
        if self.manifold_id != artifacts.run_packet.manifold_id {
            return Err(anyhow!(
                "integrated CSM proof must share the run packet manifold id"
            ));
        }
        if self.trace_ref != artifacts.governed_episode.first_run_trace_path {
            return Err(anyhow!(
                "integrated CSM proof must reference the first-run trace"
            ));
        }
        if self.execution_transcript_ref != RUNTIME_V2_CSM_INTEGRATED_RUN_TRANSCRIPT_PATH {
            return Err(anyhow!(
                "integrated CSM proof must reference the D10 execution transcript"
            ));
        }
        if self.observatory_packet_ref != artifacts.observatory.visibility_packet_path {
            return Err(anyhow!(
                "integrated CSM proof must reference the observatory packet"
            ));
        }
        if self.operator_report_ref != artifacts.observatory.operator_report_path {
            return Err(anyhow!(
                "integrated CSM proof must reference the operator report"
            ));
        }
        for required in [
            artifacts
                .recovery
                .safe_resume_decision
                .artifact_path
                .as_str(),
            artifacts
                .quarantine
                .quarantine_artifact
                .artifact_path
                .as_str(),
            artifacts.hardening.proof_packet.artifact_path.as_str(),
        ] {
            if !self.contains_artifact_ref(required) {
                return Err(anyhow!(
                    "integrated CSM proof missing required artifact ref '{required}'"
                ));
            }
        }
        if !self
            .hardening_refs
            .iter()
            .any(|value| value == &artifacts.hardening.proof_packet.artifact_path)
        {
            return Err(anyhow!(
                "integrated CSM proof missing hardening_proof_packet in hardening refs"
            ));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_INTEGRATED_RUN_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM integrated proof schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D10" {
            return Err(anyhow!(
                "integrated CSM proof must map to demo matrix row D10"
            ));
        }
        normalize_id(self.proof_id.clone(), "csm_integrated.proof_id")?;
        normalize_id(self.manifold_id.clone(), "csm_integrated.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_integrated.artifact_path")?;
        validate_relative_path(&self.run_packet_ref, "csm_integrated.run_packet_ref")?;
        validate_relative_path(&self.trace_ref, "csm_integrated.trace_ref")?;
        validate_relative_path(
            &self.execution_transcript_ref,
            "csm_integrated.execution_transcript_ref",
        )?;
        validate_relative_path(
            &self.observatory_packet_ref,
            "csm_integrated.observatory_packet_ref",
        )?;
        validate_relative_path(
            &self.operator_report_ref,
            "csm_integrated.operator_report_ref",
        )?;
        validate_relative_path_list(&self.recovery_refs, "csm_integrated.recovery_refs")?;
        validate_relative_path_list(&self.quarantine_refs, "csm_integrated.quarantine_refs")?;
        validate_relative_path_list(&self.hardening_refs, "csm_integrated.hardening_refs")?;
        validate_relative_path_list(
            &self.integrated_stage_refs,
            "csm_integrated.integrated_stage_refs",
        )?;
        if self.integrated_stage_refs.len() < 10 {
            return Err(anyhow!(
                "integrated CSM proof must include WP-05 through WP-13 stage refs"
            ));
        }
        validate_nonempty_text(&self.proof_summary, "csm_integrated.proof_summary")?;
        if self.proof_classification != "proving" {
            return Err(anyhow!(
                "integrated CSM proof must be classified as proving"
            ));
        }
        validate_relative_path(
            &self.reviewer_entrypoint,
            "csm_integrated.reviewer_entrypoint",
        )?;
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_csm_integrated_run"))
        {
            return Err(anyhow!(
                "integrated CSM proof must include the focused validation command"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("integrated-csm-run-demo"))
        {
            return Err(anyhow!(
                "integrated CSM proof must include the runnable demo command"
            ));
        }
        for command in &self.validation_commands {
            validate_nonempty_text(command, "csm_integrated.validation_commands")?;
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("first true Godel-agent birth"))
        {
            return Err(anyhow!(
                "integrated CSM proof must preserve the first-birthday non-claim"
            ));
        }
        if !self
            .claim_boundary
            .contains("bounded D10 integrated first-run evidence package")
        {
            return Err(anyhow!(
                "integrated CSM proof must preserve its bounded D10 claim boundary"
            ));
        }
        Ok(())
    }

    fn contains_artifact_ref(&self, required: &str) -> bool {
        self.recovery_refs.iter().any(|value| value == required)
            || self.quarantine_refs.iter().any(|value| value == required)
            || self.hardening_refs.iter().any(|value| value == required)
            || self
                .integrated_stage_refs
                .iter()
                .any(|value| value == required)
    }
}

fn integrated_run_transcript() -> Vec<RuntimeV2CsmIntegratedRunEvent> {
    [
        (
            1,
            "run_packet_loaded",
            "runtime_v2/csm_run/run_packet_contract.json",
            "Reviewer target and stage spine loaded.",
        ),
        (
            2,
            "boot_admission_validated",
            "runtime_v2/csm_run/boot_manifest.json",
            "Citizens admitted before execution evidence is accepted.",
        ),
        (
            3,
            "governed_episode_projected",
            "runtime_v2/csm_run/first_run_trace.jsonl",
            "Bounded first-run trace reaches event sequence 9.",
        ),
        (
            4,
            "freedom_gate_mediated",
            "runtime_v2/csm_run/freedom_gate_decision.json",
            "Allow/refuse decisions are visible before commit.",
        ),
        (
            5,
            "invalid_action_refused",
            "runtime_v2/csm_run/invalid_action_violation.json",
            "Unsafe mutation is rejected before state commit.",
        ),
        (
            6,
            "wake_continuity_proved",
            "runtime_v2/csm_run/wake_continuity_proof.json",
            "Wake has one active successor head.",
        ),
        (
            7,
            "observatory_rendered",
            "runtime_v2/observatory/operator_report.md",
            "Operator can inspect the run through the Observatory report.",
        ),
        (
            8,
            "recovery_quarantine_checked",
            "runtime_v2/recovery/safe_resume_decision.json",
            "Safe resume and quarantine-required paths are both reviewable.",
        ),
        (
            9,
            "hardening_probes_passed",
            "runtime_v2/hardening/hardening_proof_packet.json",
            "Duplicate activation, snapshot tamper, and trace replay probes are bounded.",
        ),
        (
            10,
            "integrated_proof_emitted",
            "runtime_v2/csm_run/integrated_first_run_proof_packet.json",
            "D10 flagship evidence bundle is ready for review.",
        ),
    ]
    .into_iter()
    .map(
        |(sequence, stage_id, artifact_ref, observatory_note)| RuntimeV2CsmIntegratedRunEvent {
            sequence,
            stage_id: stage_id.to_string(),
            status: "PASS".to_string(),
            artifact_ref: artifact_ref.to_string(),
            observatory_note: observatory_note.to_string(),
        },
    )
    .collect()
}

fn validate_integrated_run_transcript(events: &[RuntimeV2CsmIntegratedRunEvent]) -> Result<()> {
    if events.len() != 10 {
        return Err(anyhow!(
            "integrated CSM run transcript must contain the ten D10 stage events"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (index, event) in events.iter().enumerate() {
        let expected_sequence = (index + 1) as u32;
        if event.sequence != expected_sequence {
            return Err(anyhow!(
                "integrated CSM run transcript sequence must be contiguous"
            ));
        }
        normalize_id(event.stage_id.clone(), "csm_integrated.transcript.stage_id")?;
        if event.status != "PASS" {
            return Err(anyhow!(
                "integrated CSM run transcript events must be passing"
            ));
        }
        validate_relative_path(
            &event.artifact_ref,
            "csm_integrated.transcript.artifact_ref",
        )?;
        validate_nonempty_text(
            &event.observatory_note,
            "csm_integrated.transcript.observatory_note",
        )?;
        if !seen.insert(event.stage_id.clone()) {
            return Err(anyhow!(
                "integrated CSM run transcript stage ids must be unique"
            ));
        }
    }
    Ok(())
}

fn validate_relative_path_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate artifact ref"));
        }
    }
    Ok(())
}
