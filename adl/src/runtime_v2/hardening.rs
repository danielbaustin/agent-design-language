use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_ADVERSARIAL_RULES_SCHEMA: &str = "runtime_v2.csm_adversarial_rules.v1";
pub const RUNTIME_V2_CSM_ADVERSARIAL_HOOK_SCHEMA: &str =
    "runtime_v2.csm_governed_adversarial_hook.v1";
pub const RUNTIME_V2_CSM_HARDENING_PROBE_SCHEMA: &str = "runtime_v2.csm_hardening_probe.v1";
pub const RUNTIME_V2_CSM_HARDENING_PROOF_SCHEMA: &str = "runtime_v2.csm_hardening_proof_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmAdversarialRules {
    pub schema_version: String,
    pub rules_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub operator_scope: String,
    pub allowed_behaviors: Vec<String>,
    pub forbidden_behaviors: Vec<String>,
    pub required_evidence_refs: Vec<String>,
    pub stop_conditions: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmGovernedAdversarialHookPacket {
    pub schema_version: String,
    pub hook_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub rules_ref: String,
    pub actor: String,
    pub target_surface: String,
    pub scenario: String,
    pub attempted_pressure: String,
    pub expected_safe_outcome: String,
    pub actual_outcome: String,
    pub decision_artifact_ref: String,
    pub containment_artifacts: Vec<String>,
    pub state_mutation_allowed: bool,
    pub operator_review_note: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmHardeningProbePacket {
    pub schema_version: String,
    pub probe_id: String,
    pub demo_id: String,
    pub invariant_id: String,
    pub probe_kind: String,
    pub artifact_path: String,
    pub source_artifact_ref: String,
    pub attempted_failure: String,
    pub expected_detection: String,
    pub actual_result: String,
    pub blocked_before_commit: bool,
    pub preserved_evidence_refs: Vec<String>,
    pub next_action: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmHardeningProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub rules_ref: String,
    pub hook_ref: String,
    pub probe_refs: Vec<String>,
    pub summary: String,
    pub proof_classification: String,
    pub reviewer_entrypoint: String,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmHardeningArtifacts {
    pub rules: RuntimeV2CsmAdversarialRules,
    pub hook: RuntimeV2CsmGovernedAdversarialHookPacket,
    pub duplicate_activation_probe: RuntimeV2CsmHardeningProbePacket,
    pub snapshot_integrity_probe: RuntimeV2CsmHardeningProbePacket,
    pub trace_replay_gap_probe: RuntimeV2CsmHardeningProbePacket,
    pub proof_packet: RuntimeV2CsmHardeningProofPacket,
}

impl RuntimeV2CsmHardeningArtifacts {
    pub fn prototype() -> Result<Self> {
        let invariant_contract = runtime_v2_invariant_and_violation_contract()?;
        let quarantine = runtime_v2_csm_quarantine_contract()?;
        Self::from_contracts(&invariant_contract, &quarantine)
    }

    pub fn from_contracts(
        invariant_contract: &RuntimeV2InvariantAndViolationContractArtifacts,
        quarantine: &RuntimeV2CsmQuarantineArtifacts,
    ) -> Result<Self> {
        invariant_contract.validate()?;
        quarantine.validate()?;
        let manifold_id = quarantine.quarantine_artifact.manifold_id.clone();
        if invariant_contract.invariant_map.manifold_id != manifold_id {
            return Err(anyhow!(
                "CSM hardening inputs must share the same manifold id"
            ));
        }

        let rules = RuntimeV2CsmAdversarialRules {
            schema_version: RUNTIME_V2_CSM_ADVERSARIAL_RULES_SCHEMA.to_string(),
            rules_id: "proto-csm-01-adversarial-rules-0001".to_string(),
            demo_id: "D9".to_string(),
            manifold_id: manifold_id.clone(),
            artifact_path: "runtime_v2/hardening/rules_of_engagement.json".to_string(),
            operator_scope:
                "bounded_review_probe_against_existing_quarantine_and_invariant_artifacts"
                    .to_string(),
            allowed_behaviors: vec![
                "attempt_one_resume_from_quarantined_state".to_string(),
                "inspect_duplicate_activation_fixture".to_string(),
                "inspect_snapshot_integrity_fixture".to_string(),
                "inspect_trace_replay_gap_fixture".to_string(),
            ],
            forbidden_behaviors: vec![
                "mutate_committed_state".to_string(),
                "release_quarantined_state".to_string(),
                "prune_preserved_evidence".to_string(),
                "create_live_fork".to_string(),
            ],
            required_evidence_refs: vec![
                quarantine.quarantine_artifact.artifact_path.clone(),
                quarantine.evidence_preservation.artifact_path.clone(),
                invariant_contract.invariant_map.artifact_path.clone(),
                invariant_contract.violation_schema.artifact_path.clone(),
            ],
            stop_conditions: vec![
                "quarantine_state_would_change_without_operator_review".to_string(),
                "preserved_evidence_would_be_mutated_or_pruned".to_string(),
                "probe_would_require_live_runtime_execution".to_string(),
            ],
            claim_boundary:
                "This rules artifact governs one bounded D9 adversarial hook; it does not implement live Runtime v2 execution, first true Godel-agent birth, a complete red/blue/purple security ecology, or v0.92 identity rebinding."
                    .to_string(),
        };

        let hook = RuntimeV2CsmGovernedAdversarialHookPacket {
            schema_version: RUNTIME_V2_CSM_ADVERSARIAL_HOOK_SCHEMA.to_string(),
            hook_id: "proto-csm-01-governed-adversarial-hook-0001".to_string(),
            demo_id: "D9".to_string(),
            manifold_id: manifold_id.clone(),
            artifact_path: "runtime_v2/hardening/adversarial_hook_packet.json".to_string(),
            rules_ref: rules.artifact_path.clone(),
            actor: "operator_authorized_adversarial_probe".to_string(),
            target_surface: "quarantined_recovery_boundary".to_string(),
            scenario: "attempt_resume_from_quarantined_unsafe_recovery".to_string(),
            attempted_pressure:
                "try_to_convert_wp12_quarantine_artifact_into_active_resume_without_operator_review"
                    .to_string(),
            expected_safe_outcome: "remain_quarantined_and_preserve_evidence".to_string(),
            actual_outcome: "contained_by_quarantine_execution_block".to_string(),
            decision_artifact_ref: quarantine.quarantine_artifact.artifact_path.clone(),
            containment_artifacts: vec![
                quarantine.unsafe_recovery_fixture.artifact_path.clone(),
                quarantine.quarantine_artifact.artifact_path.clone(),
                quarantine.evidence_preservation.artifact_path.clone(),
            ],
            state_mutation_allowed: false,
            operator_review_note:
                "The hook defends the polis by proving adversarial pressure is routed through ordinary quarantine evidence instead of gaining an ungoverned execution path."
                    .to_string(),
            claim_boundary:
                "This hook proves one governed D9 containment path; it does not implement a complete security ecology, live Runtime v2 execution, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        };

        let duplicate_activation_probe = hardening_probe(HardeningProbeSpec {
            probe_id: "duplicate_activation_probe",
            invariant_id: "no_duplicate_active_citizen_instance",
            probe_kind: "duplicate_activation",
            artifact_path: "runtime_v2/hardening/duplicate_activation_probe.json",
            source_artifact_ref: "runtime_v2/citizens/active_index.json",
            attempted_failure: "attempt_second_active_head_for_proto_citizen_alpha",
            expected_detection: "duplicate_active_head_rejected_before_commit",
            actual_result: "duplicate_activation_refused",
            preserved_evidence_refs: vec![
                "runtime_v2/citizens/active_index.json",
                "runtime_v2/invariants/violation-0001.json",
            ],
            next_action: "retain_existing_active_index_and_record_violation",
        })?;
        let snapshot_integrity_probe = hardening_probe(HardeningProbeSpec {
            probe_id: "snapshot_integrity_probe",
            invariant_id: "snapshot_restore_must_validate_before_active_state",
            probe_kind: "snapshot_integrity",
            artifact_path: "runtime_v2/hardening/snapshot_integrity_probe.json",
            source_artifact_ref: "runtime_v2/rehydration_report.json",
            attempted_failure: "attempt_wake_from_snapshot_with_unverified_integrity",
            expected_detection: "wake_refused_until_snapshot_and_invariants_validate",
            actual_result: "snapshot_integrity_refused",
            preserved_evidence_refs: vec![
                "runtime_v2/snapshots/snapshot-0001.json",
                "runtime_v2/rehydration_report.json",
                "runtime_v2/csm_run/wake_continuity_proof.json",
            ],
            next_action: "keep_quarantined_or_paused_until_new_recovery_decision",
        })?;
        let trace_replay_gap_probe = hardening_probe(HardeningProbeSpec {
            probe_id: "trace_replay_gap_probe",
            invariant_id: "trace_sequence_must_advance_monotonically",
            probe_kind: "trace_replay_gap",
            artifact_path: "runtime_v2/hardening/trace_replay_gap_probe.json",
            source_artifact_ref: "runtime_v2/csm_run/first_run_trace.jsonl",
            attempted_failure: "attempt_replay_with_missing_trace_sequence",
            expected_detection: "replay_gap_rejected_and_gap_evidence_preserved",
            actual_result: "trace_replay_gap_refused",
            preserved_evidence_refs: vec![
                "runtime_v2/csm_run/first_run_trace.jsonl",
                "runtime_v2/quarantine/evidence_preservation_artifact.json",
            ],
            next_action: "preserve_trace_gap_and_require_operator_review",
        })?;

        let proof_packet = RuntimeV2CsmHardeningProofPacket {
            schema_version: RUNTIME_V2_CSM_HARDENING_PROOF_SCHEMA.to_string(),
            proof_id: "proto-csm-01-hardening-proof-0001".to_string(),
            demo_id: "D9".to_string(),
            manifold_id,
            artifact_path: "runtime_v2/hardening/hardening_proof_packet.json".to_string(),
            rules_ref: rules.artifact_path.clone(),
            hook_ref: hook.artifact_path.clone(),
            probe_refs: vec![
                duplicate_activation_probe.artifact_path.clone(),
                snapshot_integrity_probe.artifact_path.clone(),
                trace_replay_gap_probe.artifact_path.clone(),
            ],
            summary:
                "D9 proves one governed adversarial hook plus duplicate activation, snapshot integrity, and trace/replay gap hardening probes against existing Runtime v2 evidence."
                    .to_string(),
            proof_classification: "proving".to_string(),
            reviewer_entrypoint:
                "docs/milestones/v0.90.2/features/SECURITY_BOUNDARY_EVIDENCE.md".to_string(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_hardening -- --nocapture"
                    .to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_run_packet_contract -- --nocapture"
                    .to_string(),
                "git diff --check".to_string(),
            ],
            non_claims: vec![
                "does not execute a live CSM run".to_string(),
                "does not claim first true Godel-agent birth".to_string(),
                "does not implement a complete red/blue/purple security ecology".to_string(),
                "does not implement v0.92 identity or migration semantics".to_string(),
            ],
            claim_boundary:
                "This packet proves bounded D9 hardening evidence only; it does not implement live Runtime v2 execution, first true Godel-agent birth, a complete security ecology, or v0.92 identity rebinding."
                    .to_string(),
        };

        let artifacts = Self {
            rules,
            hook,
            duplicate_activation_probe,
            snapshot_integrity_probe,
            trace_replay_gap_probe,
            proof_packet,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.rules.validate()?;
        self.hook.validate_against_rules(&self.rules)?;
        self.duplicate_activation_probe.validate()?;
        self.snapshot_integrity_probe.validate()?;
        self.trace_replay_gap_probe.validate()?;
        self.proof_packet.validate_against_artifacts(self)
    }

    pub fn rules_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.rules).context("serialize Runtime v2 CSM adversarial rules")
    }

    pub fn hook_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.hook).context("serialize Runtime v2 CSM adversarial hook")
    }

    pub fn duplicate_activation_probe_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.duplicate_activation_probe)
            .context("serialize Runtime v2 CSM duplicate activation probe")
    }

    pub fn snapshot_integrity_probe_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.snapshot_integrity_probe)
            .context("serialize Runtime v2 CSM snapshot integrity probe")
    }

    pub fn trace_replay_gap_probe_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.trace_replay_gap_probe)
            .context("serialize Runtime v2 CSM trace replay gap probe")
    }

    pub fn proof_packet_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.proof_packet)
            .context("serialize Runtime v2 CSM hardening proof packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            &self.rules.artifact_path,
            self.rules_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.hook.artifact_path,
            self.hook_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.duplicate_activation_probe.artifact_path,
            self.duplicate_activation_probe_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.snapshot_integrity_probe.artifact_path,
            self.snapshot_integrity_probe_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.trace_replay_gap_probe.artifact_path,
            self.trace_replay_gap_probe_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.proof_packet.artifact_path,
            self.proof_packet_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmAdversarialRules {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_ADVERSARIAL_RULES_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM adversarial rules schema '{}'",
                self.schema_version
            ));
        }
        validate_hardening_demo(&self.demo_id, "csm_hardening.rules.demo_id")?;
        normalize_id(self.rules_id.clone(), "csm_hardening.rules_id")?;
        normalize_id(self.manifold_id.clone(), "csm_hardening.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_hardening.rules_artifact_path")?;
        normalize_id(self.operator_scope.clone(), "csm_hardening.operator_scope")?;
        validate_nonempty_id_list(&self.allowed_behaviors, "csm_hardening.allowed_behaviors")?;
        validate_nonempty_id_list(
            &self.forbidden_behaviors,
            "csm_hardening.forbidden_behaviors",
        )?;
        validate_relative_path_list(
            &self.required_evidence_refs,
            "csm_hardening.required_evidence_refs",
        )?;
        validate_nonempty_id_list(&self.stop_conditions, "csm_hardening.stop_conditions")?;
        for forbidden in [
            "mutate_committed_state",
            "release_quarantined_state",
            "prune_preserved_evidence",
        ] {
            if !self
                .forbidden_behaviors
                .iter()
                .any(|value| value == forbidden)
            {
                return Err(anyhow!("CSM adversarial rules must forbid '{forbidden}'"));
            }
        }
        validate_hardening_boundary(&self.claim_boundary, "csm_hardening.rules_boundary")
    }
}

impl RuntimeV2CsmGovernedAdversarialHookPacket {
    pub fn validate_against_rules(&self, rules: &RuntimeV2CsmAdversarialRules) -> Result<()> {
        self.validate()?;
        if self.manifold_id != rules.manifold_id || self.rules_ref != rules.artifact_path {
            return Err(anyhow!(
                "CSM adversarial hook must reference the matching rules artifact"
            ));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_ADVERSARIAL_HOOK_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM adversarial hook schema '{}'",
                self.schema_version
            ));
        }
        validate_hardening_demo(&self.demo_id, "csm_hardening.hook.demo_id")?;
        normalize_id(self.hook_id.clone(), "csm_hardening.hook_id")?;
        normalize_id(self.manifold_id.clone(), "csm_hardening.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_hardening.hook_artifact_path")?;
        validate_relative_path(&self.rules_ref, "csm_hardening.rules_ref")?;
        normalize_id(self.actor.clone(), "csm_hardening.actor")?;
        normalize_id(self.target_surface.clone(), "csm_hardening.target_surface")?;
        normalize_id(self.scenario.clone(), "csm_hardening.scenario")?;
        normalize_id(
            self.attempted_pressure.clone(),
            "csm_hardening.attempted_pressure",
        )?;
        normalize_id(
            self.expected_safe_outcome.clone(),
            "csm_hardening.expected_safe_outcome",
        )?;
        validate_hardening_hook_outcome(&self.actual_outcome)?;
        validate_relative_path(
            &self.decision_artifact_ref,
            "csm_hardening.decision_artifact_ref",
        )?;
        validate_relative_path_list(
            &self.containment_artifacts,
            "csm_hardening.containment_artifacts",
        )?;
        if self.state_mutation_allowed {
            return Err(anyhow!(
                "CSM adversarial hook must not allow state mutation"
            ));
        }
        validate_nonempty_text(
            &self.operator_review_note,
            "csm_hardening.operator_review_note",
        )?;
        validate_hardening_boundary(&self.claim_boundary, "csm_hardening.hook_boundary")
    }
}

impl RuntimeV2CsmHardeningProbePacket {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_HARDENING_PROBE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM hardening probe schema '{}'",
                self.schema_version
            ));
        }
        validate_hardening_demo(&self.demo_id, "csm_hardening.probe.demo_id")?;
        normalize_id(self.probe_id.clone(), "csm_hardening.probe_id")?;
        normalize_id(self.invariant_id.clone(), "csm_hardening.invariant_id")?;
        validate_hardening_probe_kind(&self.probe_kind)?;
        validate_relative_path(&self.artifact_path, "csm_hardening.probe_artifact_path")?;
        validate_relative_path(
            &self.source_artifact_ref,
            "csm_hardening.source_artifact_ref",
        )?;
        normalize_id(
            self.attempted_failure.clone(),
            "csm_hardening.attempted_failure",
        )?;
        normalize_id(
            self.expected_detection.clone(),
            "csm_hardening.expected_detection",
        )?;
        validate_hardening_probe_result(&self.actual_result)?;
        if !self.blocked_before_commit {
            return Err(anyhow!(
                "CSM hardening probe must prove blocked_before_commit"
            ));
        }
        validate_relative_path_list(
            &self.preserved_evidence_refs,
            "csm_hardening.preserved_evidence_refs",
        )?;
        normalize_id(self.next_action.clone(), "csm_hardening.next_action")?;
        validate_hardening_boundary(&self.claim_boundary, "csm_hardening.probe_boundary")
    }
}

impl RuntimeV2CsmHardeningProofPacket {
    pub fn validate_against_artifacts(
        &self,
        artifacts: &RuntimeV2CsmHardeningArtifacts,
    ) -> Result<()> {
        self.validate()?;
        if self.manifold_id != artifacts.rules.manifold_id
            || self.rules_ref != artifacts.rules.artifact_path
        {
            return Err(anyhow!(
                "CSM hardening proof must reference the matching rules artifact"
            ));
        }
        if self.hook_ref != artifacts.hook.artifact_path {
            return Err(anyhow!(
                "CSM hardening proof must reference the matching adversarial hook"
            ));
        }
        let required = [
            artifacts.duplicate_activation_probe.artifact_path.as_str(),
            artifacts.snapshot_integrity_probe.artifact_path.as_str(),
            artifacts.trace_replay_gap_probe.artifact_path.as_str(),
        ];
        for required_ref in required {
            if !self
                .probe_refs
                .iter()
                .any(|probe_ref| probe_ref == required_ref)
            {
                return Err(anyhow!(
                    "CSM hardening proof missing required probe ref '{required_ref}'"
                ));
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_HARDENING_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM hardening proof schema '{}'",
                self.schema_version
            ));
        }
        validate_hardening_demo(&self.demo_id, "csm_hardening.proof.demo_id")?;
        normalize_id(self.proof_id.clone(), "csm_hardening.proof_id")?;
        normalize_id(self.manifold_id.clone(), "csm_hardening.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_hardening.proof_artifact_path")?;
        validate_relative_path(&self.rules_ref, "csm_hardening.proof_rules_ref")?;
        validate_relative_path(&self.hook_ref, "csm_hardening.proof_hook_ref")?;
        validate_relative_path_list(&self.probe_refs, "csm_hardening.proof_probe_refs")?;
        if self.probe_refs.len() != 3 {
            return Err(anyhow!(
                "CSM hardening proof must include exactly three hardening probes"
            ));
        }
        validate_nonempty_text(&self.summary, "csm_hardening.summary")?;
        if self.proof_classification != "proving" {
            return Err(anyhow!(
                "CSM hardening proof must classify the D9 demo as proving"
            ));
        }
        validate_relative_path(
            &self.reviewer_entrypoint,
            "csm_hardening.reviewer_entrypoint",
        )?;
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_csm_hardening"))
        {
            return Err(anyhow!(
                "CSM hardening proof must include the focused hardening validation command"
            ));
        }
        for command in &self.validation_commands {
            validate_nonempty_text(command, "csm_hardening.validation_commands")?;
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("first true Godel-agent birth"))
        {
            return Err(anyhow!(
                "CSM hardening proof must preserve first-birthday non-claim"
            ));
        }
        validate_hardening_boundary(&self.claim_boundary, "csm_hardening.proof_boundary")
    }
}

struct HardeningProbeSpec {
    probe_id: &'static str,
    invariant_id: &'static str,
    probe_kind: &'static str,
    artifact_path: &'static str,
    source_artifact_ref: &'static str,
    attempted_failure: &'static str,
    expected_detection: &'static str,
    actual_result: &'static str,
    preserved_evidence_refs: Vec<&'static str>,
    next_action: &'static str,
}

fn hardening_probe(spec: HardeningProbeSpec) -> Result<RuntimeV2CsmHardeningProbePacket> {
    let probe = RuntimeV2CsmHardeningProbePacket {
        schema_version: RUNTIME_V2_CSM_HARDENING_PROBE_SCHEMA.to_string(),
        probe_id: format!("proto-csm-01-{}-0001", spec.probe_id),
        demo_id: "D9".to_string(),
        invariant_id: spec.invariant_id.to_string(),
        probe_kind: spec.probe_kind.to_string(),
        artifact_path: spec.artifact_path.to_string(),
        source_artifact_ref: spec.source_artifact_ref.to_string(),
        attempted_failure: spec.attempted_failure.to_string(),
        expected_detection: spec.expected_detection.to_string(),
        actual_result: spec.actual_result.to_string(),
        blocked_before_commit: true,
        preserved_evidence_refs: spec
            .preserved_evidence_refs
            .into_iter()
            .map(str::to_string)
            .collect(),
        next_action: spec.next_action.to_string(),
        claim_boundary:
            "This probe proves one bounded D9 negative path; it does not implement live Runtime v2 execution, first true Godel-agent birth, a complete security ecology, or v0.92 identity rebinding."
                .to_string(),
    };
    probe.validate()?;
    Ok(probe)
}

fn validate_hardening_demo(value: &str, field: &str) -> Result<()> {
    if value != "D9" {
        return Err(anyhow!("{field} must map to demo matrix row D9"));
    }
    Ok(())
}

fn validate_nonempty_id_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for value in values {
        normalize_id(value.clone(), field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate value '{value}'"));
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

fn validate_hardening_hook_outcome(value: &str) -> Result<()> {
    match value {
        "contained_by_quarantine_execution_block" | "refused_before_commit" => Ok(()),
        other => Err(anyhow!(
            "unsupported CSM adversarial hook outcome '{other}'"
        )),
    }
}

fn validate_hardening_probe_kind(value: &str) -> Result<()> {
    match value {
        "duplicate_activation" | "snapshot_integrity" | "trace_replay_gap" => Ok(()),
        other => Err(anyhow!("unsupported CSM hardening probe kind '{other}'")),
    }
}

fn validate_hardening_probe_result(value: &str) -> Result<()> {
    match value {
        "duplicate_activation_refused"
        | "snapshot_integrity_refused"
        | "trace_replay_gap_refused" => Ok(()),
        other => Err(anyhow!("unsupported CSM hardening probe result '{other}'")),
    }
}

fn validate_hardening_boundary(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    for required in [
        "does not implement",
        "live Runtime v2 execution",
        "first true Godel-agent birth",
        "v0.92 identity rebinding",
    ] {
        if !value.contains(required) {
            return Err(anyhow!(
                "CSM hardening {field} must preserve non-claim '{required}'"
            ));
        }
    }
    Ok(())
}
