use std::path::Path;

use anyhow::{anyhow, Context, Result};

use super::*;

pub const RUNTIME_V2_FOUNDATION_PROOF_PACKET_SCHEMA: &str = "runtime_v2.foundation_proof_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2FoundationArtifactRef {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub schema_version: String,
    pub path: String,
    pub source_wp: String,
    pub proves: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2FoundationProofCheck {
    pub check_id: String,
    pub status: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2FoundationProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub demo_name: String,
    pub classification: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub generated_at_utc: String,
    pub integrated_artifacts: Vec<RuntimeV2FoundationArtifactRef>,
    pub proof_claims: Vec<String>,
    pub non_claims: Vec<String>,
    pub checks: Vec<RuntimeV2FoundationProofCheck>,
    pub reviewer_entrypoint: String,
    pub replay_command: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2FoundationPrototypeArtifacts {
    pub manifold: RuntimeV2ManifoldRoot,
    pub kernel: RuntimeV2KernelLoopArtifacts,
    pub citizens: RuntimeV2CitizenLifecycleArtifacts,
    pub snapshot: RuntimeV2SnapshotAndRehydrationArtifacts,
    pub invariant_violation: RuntimeV2InvariantViolationArtifact,
    pub operator_report: RuntimeV2OperatorControlReport,
    pub security_boundary: RuntimeV2SecurityBoundaryProofPacket,
    pub proof_packet: RuntimeV2FoundationProofPacket,
}

impl RuntimeV2FoundationPrototypeArtifacts {
    pub fn prototype() -> Result<Self> {
        let manifold = runtime_v2_manifold_contract()?;
        let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
        let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
        let snapshot =
            RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)?;
        let invariant_violation =
            RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
                &manifold, &kernel, &citizens,
            )?;
        let operator_report = RuntimeV2OperatorControlReport::prototype(
            &manifold,
            &kernel,
            &citizens,
            &snapshot,
            &invariant_violation,
        )?;
        let security_boundary =
            RuntimeV2SecurityBoundaryProofPacket::refused_resume_without_invariant_prototype(
                &manifold,
                &kernel,
                &invariant_violation,
                &operator_report,
            )?;
        let proof_packet = RuntimeV2FoundationProofPacket::prototype(
            &manifold,
            &kernel,
            &citizens,
            &snapshot,
            &invariant_violation,
            &operator_report,
            &security_boundary,
        )?;

        let artifacts = Self {
            manifold,
            kernel,
            citizens,
            snapshot,
            invariant_violation,
            operator_report,
            security_boundary,
            proof_packet,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.manifold.validate()?;
        self.kernel.validate()?;
        self.citizens.validate()?;
        self.snapshot.validate()?;
        self.invariant_violation.validate()?;
        self.operator_report.validate()?;
        self.security_boundary.validate()?;
        self.proof_packet.validate()?;

        let manifold_id = &self.manifold.manifold_id;
        for (name, actual) in [
            ("kernel", &self.kernel.registry.manifold_id),
            ("citizens", &self.citizens.active_index.manifold_id),
            ("snapshot", &self.snapshot.snapshot.manifold_id),
            ("invariant_violation", &self.invariant_violation.manifold_id),
            ("operator_report", &self.operator_report.manifold_id),
            ("security_boundary", &self.security_boundary.manifold_id),
            ("proof_packet", &self.proof_packet.manifold_id),
        ] {
            if actual != manifold_id {
                return Err(anyhow!(
                    "Runtime v2 foundation artifact '{name}' is bound to manifold '{actual}' instead of '{manifold_id}'"
                ));
            }
        }
        Ok(())
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.validate()?;
        self.manifold
            .write_to_path(root.join(&self.manifold.artifact_path))?;
        self.kernel.write_to_root(root)?;
        self.citizens.write_to_root(root)?;
        self.snapshot.write_to_root(root)?;
        self.invariant_violation.write_to_root(root)?;
        self.operator_report.write_to_root(root)?;
        self.security_boundary.write_to_root(root)?;
        self.proof_packet.write_to_root(root)
    }
}

impl RuntimeV2FoundationProofPacket {
    pub fn prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
        snapshot: &RuntimeV2SnapshotAndRehydrationArtifacts,
        invariant_violation: &RuntimeV2InvariantViolationArtifact,
        operator_report: &RuntimeV2OperatorControlReport,
        security_boundary: &RuntimeV2SecurityBoundaryProofPacket,
    ) -> Result<Self> {
        let active_record = citizens
            .records
            .iter()
            .find(|record| record.lifecycle_state == "active")
            .ok_or_else(|| anyhow!("foundation prototype requires one active citizen record"))?;
        let pending_record = citizens
            .records
            .iter()
            .find(|record| record.lifecycle_state != "active")
            .ok_or_else(|| anyhow!("foundation prototype requires one pending citizen record"))?;

        let packet = Self {
            schema_version: RUNTIME_V2_FOUNDATION_PROOF_PACKET_SCHEMA.to_string(),
            proof_id: "runtime-v2-foundation-proof-0001".to_string(),
            demo_id: "D7".to_string(),
            demo_name: "runtime_v2_foundation_prototype".to_string(),
            classification: "proving".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/proof_packet.json".to_string(),
            generated_at_utc: "not_started".to_string(),
            integrated_artifacts: vec![
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "manifold_root".to_string(),
                    artifact_kind: "manifold".to_string(),
                    schema_version: manifold.schema_version.clone(),
                    path: manifold.artifact_path.clone(),
                    source_wp: "WP-05".to_string(),
                    proves: "bounded persistent manifold root exists".to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "kernel_service_registry".to_string(),
                    artifact_kind: "kernel_registry".to_string(),
                    schema_version: kernel.registry.schema_version.clone(),
                    path: kernel.registry.registry_path.clone(),
                    source_wp: "WP-06".to_string(),
                    proves: "kernel services are registered in deterministic activation order"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "kernel_service_state".to_string(),
                    artifact_kind: "kernel_state".to_string(),
                    schema_version: kernel.state.schema_version.clone(),
                    path: kernel.state.service_state_path.clone(),
                    source_wp: "WP-06".to_string(),
                    proves: "bounded kernel loop state is reviewable".to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "active_citizen_record".to_string(),
                    artifact_kind: "citizen_record".to_string(),
                    schema_version: active_record.schema_version.clone(),
                    path: active_record.record_path.clone(),
                    source_wp: "WP-07".to_string(),
                    proves: "one provisional citizen can be active without claiming true birth"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "pending_citizen_record".to_string(),
                    artifact_kind: "citizen_record".to_string(),
                    schema_version: pending_record.schema_version.clone(),
                    path: pending_record.record_path.clone(),
                    source_wp: "WP-07".to_string(),
                    proves: "one provisional citizen can remain pending without duplicate activation"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "snapshot_manifest".to_string(),
                    artifact_kind: "snapshot".to_string(),
                    schema_version: snapshot.snapshot.schema_version.clone(),
                    path: snapshot.snapshot.snapshot_path.clone(),
                    source_wp: "WP-08".to_string(),
                    proves: "manifold state can be snapshotted with invariant status"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "rehydration_report".to_string(),
                    artifact_kind: "rehydration".to_string(),
                    schema_version: snapshot.rehydration_report.schema_version.clone(),
                    path: snapshot.rehydration_report.report_path.clone(),
                    source_wp: "WP-08".to_string(),
                    proves: "wake can be refused when invariant preconditions are not satisfied"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "invariant_violation".to_string(),
                    artifact_kind: "invariant_violation".to_string(),
                    schema_version: invariant_violation.schema_version.clone(),
                    path: invariant_violation.artifact_path.clone(),
                    source_wp: "WP-09".to_string(),
                    proves: "illegal duplicate active citizen transition is rejected before commit"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "operator_control_report".to_string(),
                    artifact_kind: "operator_control".to_string(),
                    schema_version: operator_report.schema_version.clone(),
                    path: operator_report.artifact_path.clone(),
                    source_wp: "WP-10".to_string(),
                    proves:
                        "inspect, pause, resume, snapshot, and termination commands are bounded"
                            .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "security_boundary_proof".to_string(),
                    artifact_kind: "security_boundary".to_string(),
                    schema_version: security_boundary.schema_version.clone(),
                    path: security_boundary.artifact_path.clone(),
                    source_wp: "WP-11".to_string(),
                    proves: "invalid resume is refused through operator and scheduler policy"
                        .to_string(),
                },
            ],
            proof_claims: vec![
                "Runtime v2 foundation prototype can be inspected end to end".to_string(),
                "All WP-05 through WP-11 proof surfaces are bound to one manifold id"
                    .to_string(),
                "The prototype records both happy-path continuity handles and blocked invalid action evidence"
                    .to_string(),
            ],
            non_claims: vec![
                "does not prove first true Godel-agent birth".to_string(),
                "does not prove full moral, emotional, or polis governance".to_string(),
                "does not prove live scheduling, cross-machine migration, or full security ecology"
                    .to_string(),
            ],
            checks: vec![
                RuntimeV2FoundationProofCheck {
                    check_id: "same_manifold_id".to_string(),
                    status: "pass".to_string(),
                    evidence_ref: manifold.artifact_path.clone(),
                },
                RuntimeV2FoundationProofCheck {
                    check_id: "one_active_one_pending_citizen".to_string(),
                    status: "pass".to_string(),
                    evidence_ref: citizens.active_index.index_path.clone(),
                },
                RuntimeV2FoundationProofCheck {
                    check_id: "snapshot_rehydration_linked".to_string(),
                    status: "pass".to_string(),
                    evidence_ref: snapshot.rehydration_report.report_path.clone(),
                },
                RuntimeV2FoundationProofCheck {
                    check_id: "invalid_action_refused".to_string(),
                    status: "pass".to_string(),
                    evidence_ref: security_boundary.artifact_path.clone(),
                },
            ],
            reviewer_entrypoint:
                "cargo run --manifest-path adl/Cargo.toml -- demo demo-l-v0901-runtime-v2-foundation --run --out artifacts/v0901 --no-open"
                    .to_string(),
            replay_command:
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 foundation-demo --out artifacts/v0901/demo-l-v0901-runtime-v2-foundation"
                    .to_string(),
        };
        packet.validate()?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_FOUNDATION_PROOF_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 foundation proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "foundation.proof_id")?;
        if self.demo_id != "D7" {
            return Err(anyhow!(
                "Runtime v2 foundation demo must map to demo matrix row D7"
            ));
        }
        if self.demo_name != "runtime_v2_foundation_prototype" {
            return Err(anyhow!(
                "Runtime v2 foundation demo name must remain runtime_v2_foundation_prototype"
            ));
        }
        if self.classification != "proving" {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet must classify the demo as proving"
            ));
        }
        normalize_id(self.manifold_id.clone(), "foundation.manifold_id")?;
        validate_relative_path(&self.artifact_path, "foundation.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "foundation.generated_at_utc")?;
        validate_foundation_artifact_refs(&self.integrated_artifacts)?;
        validate_foundation_checks(&self.checks)?;
        if self.proof_claims.len() < 3 || self.non_claims.len() < 3 {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet must include explicit claims and non-claims"
            ));
        }
        validate_nonempty_text(&self.reviewer_entrypoint, "foundation.reviewer_entrypoint")?;
        validate_nonempty_text(&self.replay_command, "foundation.replay_command")?;
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 foundation proof packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

fn validate_foundation_artifact_refs(artifacts: &[RuntimeV2FoundationArtifactRef]) -> Result<()> {
    if artifacts.len() < 10 {
        return Err(anyhow!(
            "Runtime v2 foundation proof packet must integrate at least ten artifact refs"
        ));
    }
    let mut seen_ids = std::collections::BTreeSet::new();
    let mut seen_wps = std::collections::BTreeSet::new();
    for artifact in artifacts {
        normalize_id(artifact.artifact_id.clone(), "foundation.artifact_id")?;
        validate_foundation_artifact_kind(&artifact.artifact_kind)?;
        validate_relative_path(&artifact.path, "foundation.artifact_path")?;
        validate_nonempty_text(&artifact.schema_version, "foundation.artifact_schema")?;
        validate_nonempty_text(&artifact.source_wp, "foundation.source_wp")?;
        validate_nonempty_text(&artifact.proves, "foundation.proves")?;
        if !seen_ids.insert(artifact.artifact_id.clone()) {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet contains duplicate artifact id '{}'",
                artifact.artifact_id
            ));
        }
        seen_wps.insert(artifact.source_wp.clone());
    }

    for required_wp in [
        "WP-05", "WP-06", "WP-07", "WP-08", "WP-09", "WP-10", "WP-11",
    ] {
        if !seen_wps.contains(required_wp) {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet missing required source {required_wp}"
            ));
        }
    }
    Ok(())
}

fn validate_foundation_artifact_kind(value: &str) -> Result<()> {
    match value {
        "manifold"
        | "kernel_registry"
        | "kernel_state"
        | "citizen_record"
        | "snapshot"
        | "rehydration"
        | "invariant_violation"
        | "operator_control"
        | "security_boundary" => Ok(()),
        other => Err(anyhow!("unsupported foundation artifact kind '{other}'")),
    }
}

fn validate_foundation_checks(checks: &[RuntimeV2FoundationProofCheck]) -> Result<()> {
    let mut seen = std::collections::BTreeSet::new();
    for check in checks {
        normalize_id(check.check_id.clone(), "foundation.check_id")?;
        if check.status != "pass" {
            return Err(anyhow!(
                "Runtime v2 foundation proof check '{}' must pass",
                check.check_id
            ));
        }
        validate_relative_path(&check.evidence_ref, "foundation.check_evidence_ref")?;
        if !seen.insert(check.check_id.clone()) {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet contains duplicate check '{}'",
                check.check_id
            ));
        }
    }
    for required_check in [
        "same_manifold_id",
        "one_active_one_pending_citizen",
        "snapshot_rehydration_linked",
        "invalid_action_refused",
    ] {
        if !seen.contains(required_check) {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet missing required check {required_check}"
            ));
        }
    }
    Ok(())
}
