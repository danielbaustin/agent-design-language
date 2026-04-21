use super::*;

pub const RUNTIME_V2_FEATURE_PROOF_COVERAGE_SCHEMA: &str = "runtime_v2.feature_proof_coverage.v1";
pub const RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH: &str =
    "runtime_v2/feature_proof_coverage/feature_proof_coverage.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2FeatureProofCoveragePacket {
    pub schema_version: String,
    pub coverage_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub artifact_path: String,
    pub coverage_summary: String,
    pub entries: Vec<RuntimeV2FeatureProofCoverageEntry>,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2FeatureProofCoverageEntry {
    pub feature_id: String,
    pub wp: String,
    pub claim: String,
    pub coverage_kind: String,
    pub primary_evidence_refs: Vec<String>,
    pub validation_refs: Vec<String>,
    pub status: String,
}

impl RuntimeV2FeatureProofCoveragePacket {
    pub fn prototype() -> Result<Self> {
        let packet = Self {
            schema_version: RUNTIME_V2_FEATURE_PROOF_COVERAGE_SCHEMA.to_string(),
            coverage_id: "v0-90-2-feature-proof-coverage-0001".to_string(),
            demo_id: "D11".to_string(),
            milestone: "v0.90.2".to_string(),
            artifact_path: RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH.to_string(),
            coverage_summary:
                "D11 verifies every v0.90.2 Runtime v2 / CSM feature claim has a reviewable proof surface before WP-15 convergence."
                    .to_string(),
            entries: feature_proof_entries(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture".to_string(),
                "adl runtime-v2 feature-proof-coverage --out artifacts/v0902/feature-proof-coverage.json".to_string(),
                "git diff --check".to_string(),
            ],
            non_claims: vec![
                "does not execute a new unbounded live CSM run".to_string(),
                "does not claim first true Godel-agent birth".to_string(),
                "does not implement v0.91 moral or emotional civilization scope".to_string(),
                "does not implement v0.92 identity or migration semantics".to_string(),
                "does not implement a complete red/blue/purple security ecology".to_string(),
            ],
            claim_boundary:
                "This packet proves v0.90.2 feature-proof coverage, not new runtime behavior beyond the referenced D1 through D10 evidence surfaces."
                    .to_string(),
        };
        packet.validate()?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_FEATURE_PROOF_COVERAGE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 feature proof coverage schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D11" {
            return Err(anyhow!(
                "feature proof coverage must map to demo matrix row D11"
            ));
        }
        if self.milestone != "v0.90.2" {
            return Err(anyhow!(
                "feature proof coverage must target milestone v0.90.2"
            ));
        }
        normalize_id(self.coverage_id.clone(), "feature_proof.coverage_id")?;
        validate_relative_path(&self.artifact_path, "feature_proof.artifact_path")?;
        validate_nonempty_text(&self.coverage_summary, "feature_proof.coverage_summary")?;
        validate_feature_entries(&self.entries)?;
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_feature_proof_coverage"))
        {
            return Err(anyhow!(
                "feature proof coverage must include the focused validation command"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("feature-proof-coverage"))
        {
            return Err(anyhow!(
                "feature proof coverage must include the runnable coverage command"
            ));
        }
        for command in &self.validation_commands {
            validate_nonempty_text(command, "feature_proof.validation_commands")?;
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("first true Godel-agent birth"))
        {
            return Err(anyhow!(
                "feature proof coverage must preserve the first-birthday non-claim"
            ));
        }
        if !self.claim_boundary.contains("D1 through D10") {
            return Err(anyhow!(
                "feature proof coverage must preserve the D1 through D10 claim boundary"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self)
            .context("serialize Runtime v2 feature proof coverage packet")
    }

    pub fn write_to_path(&self, output_path: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let output_path = output_path.as_ref();
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create Runtime v2 feature proof coverage parent {}",
                    parent.display()
                )
            })?;
        }
        std::fs::write(output_path, self.pretty_json_bytes()?).with_context(|| {
            format!(
                "failed to write Runtime v2 feature proof coverage packet to {}",
                output_path.display()
            )
        })
    }
}

fn feature_proof_entries() -> Vec<RuntimeV2FeatureProofCoverageEntry> {
    vec![
        entry(
            "D1",
            "WP-02",
            "v0.90.2 inherits the actual v0.90.1 Runtime v2 substrate before CSM work widens",
            "fixture_backed_artifact",
            &["docs/milestones/v0.90.2/RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md"],
            &["test -f docs/milestones/v0.90.2/RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md"],
            "proving",
        ),
        entry(
            "D2",
            "WP-03-WP-04",
            "The first CSM run has a stable packet, invariant map, and violation artifact contract",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.2/CSM_RUN_PACKET_CONTRACT_v0.90.2.md",
                "adl/tests/fixtures/runtime_v2/csm_run/run_packet_contract.json",
                "adl/tests/fixtures/runtime_v2/invariants/csm_run_invariant_map.json",
                "adl/tests/fixtures/runtime_v2/violations/violation_artifact_schema.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_run_packet_contract -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_invariant_and_violation_contract -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D3",
            "WP-05",
            "proto-csm-01 boots and admits two worker citizens with traceable identity handles",
            "test_backed_proof_packet",
            &[
                "adl/tests/fixtures/runtime_v2/csm_run/boot_manifest.json",
                "adl/tests/fixtures/runtime_v2/csm_run/citizen_roster.json",
                "adl/tests/fixtures/runtime_v2/csm_run/boot_admission_trace.jsonl",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_boot_admission -- --nocapture"],
            "proving",
        ),
        entry(
            "D4",
            "WP-06-WP-07",
            "A governed episode runs under resource pressure and Freedom Gate mediation",
            "test_backed_proof_packet",
            &[
                "adl/tests/fixtures/runtime_v2/csm_run/scheduling_decision.json",
                "adl/tests/fixtures/runtime_v2/csm_run/freedom_gate_decision.json",
                "adl/tests/fixtures/runtime_v2/csm_run/first_run_trace.jsonl",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_governed_episode -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_freedom_gate_mediation -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D5",
            "WP-08",
            "An invalid action is rejected before commit through the normal policy path",
            "test_backed_proof_packet",
            &[
                "adl/tests/fixtures/runtime_v2/csm_run/invalid_action_fixture.json",
                "adl/tests/fixtures/runtime_v2/csm_run/invalid_action_violation.json",
                "adl/tests/fixtures/runtime_v2/csm_run/first_run_trace.jsonl",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_invalid_action_rejection -- --nocapture"],
            "proving",
        ),
        entry(
            "D6",
            "WP-09",
            "Snapshot, rehydrate, and wake preserve continuity without duplicate activation",
            "test_backed_proof_packet",
            &[
                "adl/tests/fixtures/runtime_v2/snapshots/snapshot-0001.json",
                "adl/tests/fixtures/runtime_v2/rehydration_report.json",
                "adl/tests/fixtures/runtime_v2/csm_run/wake_continuity_proof.json",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_wake_continuity -- --nocapture"],
            "proving",
        ),
        entry(
            "D7",
            "WP-10",
            "The CSM Observatory shows the first bounded run through packet and operator report surfaces",
            "test_backed_proof_packet",
            &[
                "adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json",
                "adl/tests/fixtures/runtime_v2/observatory/operator_report.md",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_observatory -- --nocapture"],
            "proving",
        ),
        entry(
            "D8",
            "WP-11-WP-12",
            "Runtime distinguishes safe resume from quarantine-required unsafe recovery",
            "test_backed_proof_packet",
            &[
                "adl/tests/fixtures/runtime_v2/recovery/eligibility_model.json",
                "adl/tests/fixtures/runtime_v2/recovery/safe_resume_decision.json",
                "adl/tests/fixtures/runtime_v2/quarantine/quarantine_artifact.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_recovery_eligibility -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_quarantine -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D9",
            "WP-13",
            "The governed adversarial hook and hardening probes fail closed under explicit rules",
            "test_backed_proof_packet",
            &[
                "adl/tests/fixtures/runtime_v2/hardening/adversarial_hook_packet.json",
                "adl/tests/fixtures/runtime_v2/hardening/hardening_proof_packet.json",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_hardening -- --nocapture"],
            "proving",
        ),
        entry(
            "D10",
            "WP-14",
            "The flagship command runs the bounded CSM stage spine and prints the Observatory report",
            "runnable_demo_command",
            &[
                "adl/tests/fixtures/runtime_v2/csm_run/integrated_first_run_proof_packet.json",
                "docs/milestones/v0.90.2/DEMO_MATRIX_v0.90.2.md",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_integrated_run -- --nocapture",
                "adl runtime-v2 integrated-csm-run-demo --out artifacts/v0902/demo-d10-integrated-csm-run",
            ],
            "proving",
        ),
        entry(
            "D11",
            "WP-14A",
            "Every v0.90.2 feature claim has an explicit proof surface before WP-15 convergence",
            "runnable_demo_command",
            &[
                "docs/milestones/v0.90.2/FEATURE_PROOF_COVERAGE_v0.90.2.md",
                RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH,
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture",
                "adl runtime-v2 feature-proof-coverage --out artifacts/v0902/feature-proof-coverage.json",
            ],
            "proving",
        ),
    ]
}

fn entry(
    feature_id: &str,
    wp: &str,
    claim: &str,
    coverage_kind: &str,
    primary_evidence_refs: &[&str],
    validation_refs: &[&str],
    status: &str,
) -> RuntimeV2FeatureProofCoverageEntry {
    RuntimeV2FeatureProofCoverageEntry {
        feature_id: feature_id.to_string(),
        wp: wp.to_string(),
        claim: claim.to_string(),
        coverage_kind: coverage_kind.to_string(),
        primary_evidence_refs: primary_evidence_refs
            .iter()
            .map(|value| value.to_string())
            .collect(),
        validation_refs: validation_refs
            .iter()
            .map(|value| value.to_string())
            .collect(),
        status: status.to_string(),
    }
}

fn validate_feature_entries(entries: &[RuntimeV2FeatureProofCoverageEntry]) -> Result<()> {
    if entries.len() != 11 {
        return Err(anyhow!(
            "feature proof coverage must include D1 through D11"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (index, entry) in entries.iter().enumerate() {
        let expected = format!("D{}", index + 1);
        if entry.feature_id != expected {
            return Err(anyhow!(
                "feature proof coverage entries must be ordered D1 through D11"
            ));
        }
        if !seen.insert(entry.feature_id.clone()) {
            return Err(anyhow!(
                "feature proof coverage contains duplicate feature id"
            ));
        }
        normalize_id(entry.feature_id.clone(), "feature_proof.entry.feature_id")?;
        validate_nonempty_text(&entry.wp, "feature_proof.entry.wp")?;
        validate_nonempty_text(&entry.claim, "feature_proof.entry.claim")?;
        match entry.coverage_kind.as_str() {
            "runnable_demo_command" | "test_backed_proof_packet" | "fixture_backed_artifact" => {}
            other => return Err(anyhow!("unsupported feature proof coverage kind '{other}'")),
        }
        if entry.primary_evidence_refs.is_empty() {
            return Err(anyhow!(
                "feature proof coverage entry must include evidence refs"
            ));
        }
        for value in &entry.primary_evidence_refs {
            validate_relative_path(value, "feature_proof.entry.primary_evidence_refs")?;
        }
        if entry.validation_refs.is_empty() {
            return Err(anyhow!(
                "feature proof coverage entry must include validation refs"
            ));
        }
        for value in &entry.validation_refs {
            validate_nonempty_text(value, "feature_proof.entry.validation_refs")?;
        }
        if entry.status != "proving" {
            return Err(anyhow!(
                "feature proof coverage entries must be proving or explicitly deferred"
            ));
        }
    }
    Ok(())
}
