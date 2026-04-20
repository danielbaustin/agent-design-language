use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_RUN_INVARIANT_MAP_SCHEMA: &str = "runtime_v2.csm_run_invariant_map.v1";
pub const RUNTIME_V2_VIOLATION_ARTIFACT_SCHEMA_CONTRACT: &str =
    "runtime_v2.violation_artifact_schema_contract.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2InvariantCoverageEntry {
    pub invariant_id: String,
    pub invariant_class: String,
    pub owner_service_id: String,
    pub enforcement_mode: String,
    pub stage_ref: String,
    pub required_before_wp: String,
    pub evidence_refs: Vec<String>,
    pub positive_fixture_ref: String,
    pub negative_fixture_ref: String,
    pub coverage_status: String,
    pub proof_obligation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmRunInvariantMap {
    pub schema_version: String,
    pub map_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub generated_at_utc: String,
    pub source_refs: Vec<String>,
    pub coverage_entries: Vec<RuntimeV2InvariantCoverageEntry>,
    pub required_before_live_run: bool,
    pub gap_policy: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ViolationArtifactFieldSpec {
    pub field_name: String,
    pub requirement: String,
    pub validation_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ViolationArtifactSchemaContract {
    pub schema_version: String,
    pub contract_id: String,
    pub artifact_schema_version: String,
    pub artifact_path: String,
    pub generated_at_utc: String,
    pub source_refs: Vec<String>,
    pub required_fields: Vec<RuntimeV2ViolationArtifactFieldSpec>,
    pub required_decision_values: Vec<String>,
    pub positive_fixture_ref: String,
    pub negative_fixture_ref: String,
    pub reviewer_entrypoint: String,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2InvariantAndViolationContractArtifacts {
    pub invariant_map: RuntimeV2CsmRunInvariantMap,
    pub violation_schema: RuntimeV2ViolationArtifactSchemaContract,
}

impl RuntimeV2InvariantAndViolationContractArtifacts {
    pub fn prototype() -> Result<Self> {
        let manifold = runtime_v2_manifold_contract()?;
        let csm_run = runtime_v2_csm_run_packet_contract()?;
        let violation = runtime_v2_invariant_violation_contract()?;
        Self::from_contracts(&manifold, &csm_run, &violation)
    }

    pub fn from_contracts(
        manifold: &RuntimeV2ManifoldRoot,
        csm_run: &RuntimeV2CsmRunPacketContract,
        violation: &RuntimeV2InvariantViolationArtifact,
    ) -> Result<Self> {
        manifold.validate()?;
        csm_run.validate()?;
        violation.validate()?;
        if csm_run.manifold_id != manifold.manifold_id
            || violation.manifold_id != manifold.manifold_id
        {
            return Err(anyhow!(
                "invariant contract inputs must share the same manifold id"
            ));
        }

        let invariant_map = RuntimeV2CsmRunInvariantMap {
            schema_version: RUNTIME_V2_CSM_RUN_INVARIANT_MAP_SCHEMA.to_string(),
            map_id: "proto-csm-01-invariant-map-0001".to_string(),
            demo_id: "D2".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/invariants/csm_run_invariant_map.json".to_string(),
            generated_at_utc: "2026-04-20T00:00:00Z".to_string(),
            source_refs: vec![
                "docs/milestones/v0.90.2/WBS_v0.90.2.md".to_string(),
                "docs/milestones/v0.90.2/DEMO_MATRIX_v0.90.2.md".to_string(),
                "docs/milestones/v0.90.2/features/INVARIANT_EXPANSION_AND_COVERAGE.md"
                    .to_string(),
                csm_run.artifact_path.clone(),
                violation.artifact_path.clone(),
                manifold.invariant_policy_refs.policy_path.clone(),
            ],
            coverage_entries: vec![
                RuntimeV2InvariantCoverageEntry {
                    invariant_id: "single_active_manifold_instance".to_string(),
                    invariant_class: "manifold_integrity".to_string(),
                    owner_service_id: "kernel_runtime".to_string(),
                    enforcement_mode: "fail_closed_before_activation".to_string(),
                    stage_ref: "contract_and_fixture".to_string(),
                    required_before_wp: "WP-05".to_string(),
                    evidence_refs: vec![
                        manifold.artifact_path.clone(),
                        csm_run.artifact_path.clone(),
                    ],
                    positive_fixture_ref: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    negative_fixture_ref: violation.artifact_path.clone(),
                    coverage_status: "contracted".to_string(),
                    proof_obligation:
                        "only one proto-csm-01 manifold head may be accepted for the run"
                            .to_string(),
                },
                RuntimeV2InvariantCoverageEntry {
                    invariant_id: "no_duplicate_active_citizen_instance".to_string(),
                    invariant_class: "citizen_continuity".to_string(),
                    owner_service_id: "invariant_checker".to_string(),
                    enforcement_mode: "fail_closed_before_activation".to_string(),
                    stage_ref: "boot_and_admission".to_string(),
                    required_before_wp: "WP-05".to_string(),
                    evidence_refs: vec![
                        "runtime_v2/citizens/active_index.json".to_string(),
                        violation.artifact_path.clone(),
                    ],
                    positive_fixture_ref: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    negative_fixture_ref: violation.artifact_path.clone(),
                    coverage_status: "negative_fixture_backed".to_string(),
                    proof_obligation:
                        "duplicate active citizen activation must be refused before commit"
                            .to_string(),
                },
                RuntimeV2InvariantCoverageEntry {
                    invariant_id: "trace_sequence_must_advance_monotonically".to_string(),
                    invariant_class: "temporal_ordering".to_string(),
                    owner_service_id: "trace_writer".to_string(),
                    enforcement_mode: "fail_closed_before_activation".to_string(),
                    stage_ref: "governed_episode_and_rejection".to_string(),
                    required_before_wp: "WP-06".to_string(),
                    evidence_refs: vec![
                        "runtime_v2/kernel/service_loop.jsonl".to_string(),
                        "runtime_v2/csm_run/first_run_trace.jsonl".to_string(),
                    ],
                    positive_fixture_ref: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    negative_fixture_ref: violation.artifact_path.clone(),
                    coverage_status: "contracted_for_wp06".to_string(),
                    proof_obligation:
                        "later trace events must be contiguous and may not rewrite prior order"
                            .to_string(),
                },
                RuntimeV2InvariantCoverageEntry {
                    invariant_id: "invalid_action_must_be_refused_before_commit".to_string(),
                    invariant_class: "security_boundary_enforcement".to_string(),
                    owner_service_id: "operator_control_interface".to_string(),
                    enforcement_mode: "fail_closed_before_activation".to_string(),
                    stage_ref: "governed_episode_and_rejection".to_string(),
                    required_before_wp: "WP-08".to_string(),
                    evidence_refs: vec![
                        violation.artifact_path.clone(),
                        "runtime_v2/security_boundary/proof_packet.json".to_string(),
                    ],
                    positive_fixture_ref: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    negative_fixture_ref: violation.artifact_path.clone(),
                    coverage_status: "negative_fixture_backed".to_string(),
                    proof_obligation:
                        "invalid actions must emit stable refusal evidence without state mutation"
                            .to_string(),
                },
                RuntimeV2InvariantCoverageEntry {
                    invariant_id: "snapshot_restore_must_validate_before_active_state".to_string(),
                    invariant_class: "recovery_eligibility".to_string(),
                    owner_service_id: "snapshot_service".to_string(),
                    enforcement_mode: "fail_closed_before_activation".to_string(),
                    stage_ref: "snapshot_wake_and_observatory".to_string(),
                    required_before_wp: "WP-09".to_string(),
                    evidence_refs: vec![
                        "runtime_v2/snapshots/snapshot-0001.json".to_string(),
                        "runtime_v2/rehydration_report.json".to_string(),
                    ],
                    positive_fixture_ref: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
                    negative_fixture_ref: violation.artifact_path.clone(),
                    coverage_status: "contracted_for_wp09".to_string(),
                    proof_obligation:
                        "wake must prove invariant checks before re-entering active state"
                            .to_string(),
                },
            ],
            required_before_live_run: true,
            gap_policy:
                "missing or ambiguous invariant coverage blocks WP-05 boot and later live-run claims"
                    .to_string(),
            claim_boundary:
                "This map contracts invariant coverage for D2; it is not evidence that a live Runtime v2 run has executed."
                    .to_string(),
        };

        let violation_schema = RuntimeV2ViolationArtifactSchemaContract {
            schema_version: RUNTIME_V2_VIOLATION_ARTIFACT_SCHEMA_CONTRACT.to_string(),
            contract_id: "runtime-v2-violation-artifact-schema-0001".to_string(),
            artifact_schema_version: RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA.to_string(),
            artifact_path: "runtime_v2/violations/violation_artifact_schema.json".to_string(),
            generated_at_utc: "2026-04-20T00:00:00Z".to_string(),
            source_refs: vec![
                "docs/milestones/v0.90.2/features/VIOLATION_ARTIFACT_CONTRACT.md".to_string(),
                "docs/milestones/v0.90.2/WBS_v0.90.2.md".to_string(),
                csm_run.artifact_path.clone(),
                violation.artifact_path.clone(),
            ],
            required_fields: vec![
                field_spec("schema_version", "stable version for violation artifacts", "must equal runtime_v2.invariant_violation.v1"),
                field_spec("violation_id", "stable identifier for the rejected transition", "must be a non-path id"),
                field_spec("manifold_id", "manifold lineage affected by the attempted transition", "must match proto-csm-01 for D2"),
                field_spec("invariant_id", "invariant that caused refusal", "must be a non-path id"),
                field_spec("policy_enforcement_mode", "policy mode used for the decision", "must be fail_closed_before_activation or report_only"),
                field_spec("attempted_transition", "actor, action, state, and source artifact for the attempt", "must name stable ids and repository-relative refs"),
                field_spec("evaluated_refs", "artifacts checked before refusing the transition", "must be non-empty and unique"),
                field_spec("affected_citizens", "citizen ids affected by the attempted transition", "must be non-empty and unique"),
                field_spec("refusal_reason", "human-reviewable reason for the refusal", "must be non-empty"),
                field_spec("source_error", "validator error or policy failure that caused refusal", "must be non-empty"),
                field_spec("result", "resulting state, before-commit block proof, recovery action, and trace ref", "blocked_before_commit must be true"),
            ],
            required_decision_values: vec![
                "transition_refused_state_unchanged".to_string(),
                "blocked_before_commit".to_string(),
                "retain_existing_active_index_and_record_violation".to_string(),
            ],
            positive_fixture_ref: "runtime_v2/csm_run/run_packet_contract.json".to_string(),
            negative_fixture_ref: violation.artifact_path.clone(),
            reviewer_entrypoint:
                "docs/milestones/v0.90.2/features/VIOLATION_ARTIFACT_CONTRACT.md".to_string(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_invariant_and_violation_contract -- --nocapture".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "This schema contract fixes violation artifact shape; it does not execute WP-08 invalid-action flow."
                    .to_string(),
        };

        let artifacts = Self {
            invariant_map,
            violation_schema,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.invariant_map.validate()?;
        self.violation_schema.validate()?;
        if self.invariant_map.demo_id != "D2" {
            return Err(anyhow!(
                "invariant and violation contract must remain bound to D2"
            ));
        }
        if self.violation_schema.negative_fixture_ref
            != self.invariant_map.coverage_entries[1].negative_fixture_ref
        {
            return Err(anyhow!(
                "invariant map and violation schema must share the negative fixture"
            ));
        }
        Ok(())
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.invariant_map.write_to_root(root.as_ref())?;
        self.violation_schema.write_to_root(root.as_ref())
    }
}

impl RuntimeV2CsmRunInvariantMap {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_RUN_INVARIANT_MAP_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM run invariant map schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D2" {
            return Err(anyhow!("CSM run invariant map must map to D2"));
        }
        normalize_id(self.map_id.clone(), "invariant_map.map_id")?;
        normalize_id(self.manifold_id.clone(), "invariant_map.manifold_id")?;
        validate_relative_path(&self.artifact_path, "invariant_map.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "invariant_map.generated_at_utc")?;
        validate_relative_refs(&self.source_refs, "invariant_map.source_refs")?;
        validate_invariant_coverage_entries(&self.coverage_entries)?;
        if !self.required_before_live_run {
            return Err(anyhow!(
                "CSM run invariant map must be required before live run"
            ));
        }
        validate_nonempty_text(&self.gap_policy, "invariant_map.gap_policy")?;
        if !self
            .claim_boundary
            .contains("not evidence that a live Runtime v2 run has executed")
        {
            return Err(anyhow!(
                "CSM run invariant map must preserve its non-live claim boundary"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM run invariant map")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2InvariantCoverageEntry {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.invariant_id.clone(), "invariant_map.invariant_id")?;
        normalize_id(
            self.invariant_class.clone(),
            "invariant_map.invariant_class",
        )?;
        normalize_id(
            self.owner_service_id.clone(),
            "invariant_map.owner_service_id",
        )?;
        match self.enforcement_mode.as_str() {
            "fail_closed_before_activation" | "report_only" => {}
            other => {
                return Err(anyhow!(
                    "unsupported invariant_map.enforcement_mode '{other}'"
                ))
            }
        }
        normalize_id(self.stage_ref.clone(), "invariant_map.stage_ref")?;
        validate_nonempty_text(&self.required_before_wp, "invariant_map.required_before_wp")?;
        validate_relative_refs(&self.evidence_refs, "invariant_map.evidence_refs")?;
        validate_relative_path(
            &self.positive_fixture_ref,
            "invariant_map.positive_fixture_ref",
        )?;
        validate_relative_path(
            &self.negative_fixture_ref,
            "invariant_map.negative_fixture_ref",
        )?;
        match self.coverage_status.as_str() {
            "contracted"
            | "negative_fixture_backed"
            | "contracted_for_wp06"
            | "contracted_for_wp09" => {}
            other => {
                return Err(anyhow!(
                    "unsupported invariant_map.coverage_status '{other}'"
                ))
            }
        }
        validate_nonempty_text(&self.proof_obligation, "invariant_map.proof_obligation")
    }
}

impl RuntimeV2ViolationArtifactSchemaContract {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_VIOLATION_ARTIFACT_SCHEMA_CONTRACT {
            return Err(anyhow!(
                "unsupported Runtime v2 violation artifact schema contract '{}'",
                self.schema_version
            ));
        }
        if self.artifact_schema_version != RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA {
            return Err(anyhow!(
                "violation schema contract must describe the invariant violation artifact schema"
            ));
        }
        normalize_id(self.contract_id.clone(), "violation_schema.contract_id")?;
        validate_relative_path(&self.artifact_path, "violation_schema.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "violation_schema.generated_at_utc")?;
        validate_relative_refs(&self.source_refs, "violation_schema.source_refs")?;
        validate_violation_schema_fields(&self.required_fields)?;
        validate_decision_values(&self.required_decision_values)?;
        validate_relative_path(
            &self.positive_fixture_ref,
            "violation_schema.positive_fixture_ref",
        )?;
        validate_relative_path(
            &self.negative_fixture_ref,
            "violation_schema.negative_fixture_ref",
        )?;
        validate_relative_path(
            &self.reviewer_entrypoint,
            "violation_schema.reviewer_entrypoint",
        )?;
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_invariant_and_violation_contract"))
        {
            return Err(anyhow!(
                "violation schema contract must include focused WP-04 validation"
            ));
        }
        for command in &self.validation_commands {
            validate_nonempty_text(command, "violation_schema.validation_commands")?;
        }
        if !self.claim_boundary.contains("does not execute WP-08") {
            return Err(anyhow!(
                "violation schema contract must preserve its WP-08 non-claim"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self)
            .context("serialize Runtime v2 violation artifact schema contract")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2ViolationArtifactFieldSpec {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.field_name.clone(), "violation_schema.field_name")?;
        validate_nonempty_text(&self.requirement, "violation_schema.field_requirement")?;
        validate_nonempty_text(
            &self.validation_rule,
            "violation_schema.field_validation_rule",
        )
    }
}

fn field_spec(
    field_name: &str,
    requirement: &str,
    validation_rule: &str,
) -> RuntimeV2ViolationArtifactFieldSpec {
    RuntimeV2ViolationArtifactFieldSpec {
        field_name: field_name.to_string(),
        requirement: requirement.to_string(),
        validation_rule: validation_rule.to_string(),
    }
}

fn validate_relative_refs(refs: &[String], field: &str) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for reference in refs {
        validate_relative_path(reference, field)?;
        if !seen.insert(reference.clone()) {
            return Err(anyhow!("{field} contains duplicate reference"));
        }
    }
    Ok(())
}

fn validate_invariant_coverage_entries(entries: &[RuntimeV2InvariantCoverageEntry]) -> Result<()> {
    let required_ids = [
        "single_active_manifold_instance",
        "no_duplicate_active_citizen_instance",
        "trace_sequence_must_advance_monotonically",
        "invalid_action_must_be_refused_before_commit",
        "snapshot_restore_must_validate_before_active_state",
    ];
    if entries.len() != required_ids.len() {
        return Err(anyhow!(
            "CSM run invariant map must cover the D2 invariant set exactly"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for entry in entries {
        entry.validate()?;
        if !seen.insert(entry.invariant_id.clone()) {
            return Err(anyhow!(
                "CSM run invariant map contains duplicate invariant '{}'",
                entry.invariant_id
            ));
        }
    }
    for required_id in required_ids {
        if !seen.contains(required_id) {
            return Err(anyhow!(
                "CSM run invariant map missing required invariant '{required_id}'"
            ));
        }
    }
    Ok(())
}

fn validate_violation_schema_fields(fields: &[RuntimeV2ViolationArtifactFieldSpec]) -> Result<()> {
    let required_fields = [
        "schema_version",
        "violation_id",
        "manifold_id",
        "invariant_id",
        "policy_enforcement_mode",
        "attempted_transition",
        "evaluated_refs",
        "affected_citizens",
        "refusal_reason",
        "source_error",
        "result",
    ];
    if fields.len() != required_fields.len() {
        return Err(anyhow!(
            "violation schema contract must define every required field"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for field in fields {
        field.validate()?;
        if !seen.insert(field.field_name.clone()) {
            return Err(anyhow!(
                "violation schema contract contains duplicate field '{}'",
                field.field_name
            ));
        }
    }
    for required_field in required_fields {
        if !seen.contains(required_field) {
            return Err(anyhow!(
                "violation schema contract missing required field '{required_field}'"
            ));
        }
    }
    Ok(())
}

fn validate_decision_values(values: &[String]) -> Result<()> {
    let required_values = [
        "transition_refused_state_unchanged",
        "blocked_before_commit",
        "retain_existing_active_index_and_record_violation",
    ];
    let mut seen = std::collections::BTreeSet::new();
    for value in values {
        normalize_id(value.clone(), "violation_schema.required_decision_values")?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!(
                "violation schema contract contains duplicate decision value"
            ));
        }
    }
    for required_value in required_values {
        if !seen.contains(required_value) {
            return Err(anyhow!(
                "violation schema contract missing decision value '{required_value}'"
            ));
        }
    }
    Ok(())
}
