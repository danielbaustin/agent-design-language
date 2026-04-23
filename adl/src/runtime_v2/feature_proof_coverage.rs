//! Runtime-v2 feature proof coverage artifacts.
//!
//! Defines explicit proof packets and coverage summaries used to represent
//! feature-level verification outcomes in a replayable format.

use super::*;

pub const RUNTIME_V2_FEATURE_PROOF_COVERAGE_SCHEMA: &str = "runtime_v2.feature_proof_coverage.v2";
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
    pub working_demo_command: String,
    pub primary_evidence_refs: Vec<String>,
    pub validation_refs: Vec<String>,
    pub status: String,
}

impl RuntimeV2FeatureProofCoveragePacket {
    pub fn prototype() -> Result<Self> {
        let packet = Self {
            schema_version: RUNTIME_V2_FEATURE_PROOF_COVERAGE_SCHEMA.to_string(),
            coverage_id: "v0-90-3-feature-proof-coverage-0001".to_string(),
            demo_id: "D13".to_string(),
            milestone: "v0.90.3".to_string(),
            artifact_path: RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH.to_string(),
            coverage_summary:
                "D13 verifies every v0.90.3 citizen-state substrate feature claim has a reviewable proof surface, non-proving boundary, or explicit deferral before WP-15 convergence."
                    .to_string(),
            entries: feature_proof_entries(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture".to_string(),
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0903/feature-proof-coverage.json".to_string(),
                "git diff --check".to_string(),
            ],
            non_claims: vec![
                "does not execute a new unbounded live CSM run".to_string(),
                "does not claim first true Godel-agent birth".to_string(),
                "does not implement v0.91 moral or emotional civilization scope".to_string(),
                "does not implement v0.92 identity/capability rebinding, migration, or birthday record semantics".to_string(),
                "does not implement full citizen economics or contract-market execution".to_string(),
                "does not claim production cloud enclave deployment".to_string(),
                "does not claim production Observatory UI readiness".to_string(),
            ],
            claim_boundary:
                "This packet proves v0.90.3 feature-proof coverage, not new runtime behavior beyond the referenced D1 through D12 evidence surfaces; D14 remains a non-runtime UI architecture boundary."
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
        if self.demo_id != "D13" {
            return Err(anyhow!(
                "feature proof coverage must map to demo matrix row D13"
            ));
        }
        if self.milestone != "v0.90.3" {
            return Err(anyhow!(
                "feature proof coverage must target milestone v0.90.3"
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
        if !self.claim_boundary.contains("D1 through D12") {
            return Err(anyhow!(
                "feature proof coverage must preserve the D1 through D12 claim boundary"
            ));
        }
        if !self.claim_boundary.contains("D14") {
            return Err(anyhow!(
                "feature proof coverage must preserve the D14 non-runtime design boundary"
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
            "v0.90.3 targets actual v0.90.2 citizen, snapshot, wake, quarantine, and Observatory artifacts",
            "fixture_backed_artifact",
            &["docs/milestones/v0.90.3/CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md"],
            &["test -f docs/milestones/v0.90.3/CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md"],
            "proving",
        ),
        entry(
            "D2",
            "WP-03",
            "Authoritative private citizen state is typed and distinct from JSON projection",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/PRIVATE_STATE_FORMAT_DECISION_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/private_state/format_decision.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D3",
            "WP-04",
            "Missing, unknown, revoked, mismatched, regressed, and broken-predecessor private states are rejected",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/SIGNED_PRIVATE_STATE_ENVELOPE_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/private_state/envelope_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_envelope -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_trust_root_matches_golden_fixture -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D4",
            "WP-05",
            "Private checkpoints can be sealed locally without making cloud enclaves mandatory",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/LOCAL_PRIVATE_STATE_SEALING_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/private_state/sealing_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sealing -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D5",
            "WP-06",
            "Current state is accepted only when it matches append-only lineage",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/APPEND_ONLY_LINEAGE_LEDGER_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/private_state/lineage_negative_cases.json",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_lineage -- --nocapture"],
            "proving",
        ),
        entry(
            "D6",
            "WP-07",
            "Admission, snapshot, wake, and quarantine transitions produce explainable continuity evidence",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/CONTINUITY_WITNESSES_AND_RECEIPTS_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/private_state/witness_receipt_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D7",
            "WP-08",
            "Conflicting signed successors for the same sequence are detected and cannot both become active",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/ANTI_EQUIVOCATION_CONFLICT_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/private_state/anti_equivocation_negative_cases.json",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_anti_equivocation -- --nocapture"],
            "proving",
        ),
        entry(
            "D8",
            "WP-09",
            "Ambiguous wake preserves evidence and blocks unsafe activation",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_negative_cases.json",
                "adl/tests/fixtures/runtime_v2/quarantine/quarantine_artifact.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sanctuary -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_quarantine -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D9",
            "WP-10",
            "Operators see continuity status without raw private state",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/observatory/private_state_redaction_policy.json",
                "adl/tests/fixtures/runtime_v2/observatory/private_state_projection_negative_cases.json",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture"],
            "proving",
        ),
        entry(
            "D10",
            "WP-11-WP-12",
            "Guests and service actors cannot silently acquire citizen rights or inspection access, and every sensitive access path emits an auditable event",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md",
                "docs/milestones/v0.90.3/ACCESS_CONTROL_SEMANTICS_v0.90.3.md",
                "adl/tests/fixtures/runtime_v2/standing/standing_policy.json",
                "adl/tests/fixtures/runtime_v2/access_control/authority_matrix.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D11",
            "WP-13",
            "A challenged wake or projection freezes destructive transition and preserves evidence, with threat-model coverage before demo claims widen",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.3/CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md",
            ],
            &["cargo test --manifest-path adl/Cargo.toml runtime_v2_continuity_challenge -- --nocapture"],
            "proving",
        ),
        entry(
            "D12",
            "WP-14",
            "The inhabited CSM Observatory flagship demo ties citizen-state evidence into one bounded reviewer scenario",
            "runnable_demo_command",
            &[
                "docs/milestones/v0.90.3/OBSERVATORY_FLAGSHIP_DEMO_v0.90.3.md",
                "runtime_v2/observatory/flagship_proof_packet.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship -- --nocapture",
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 observatory-flagship-demo --out artifacts/v0903/demo-d12-observatory-flagship",
            ],
            "proving",
        ),
        entry(
            "D13",
            "WP-14A",
            "Every v0.90.3 feature claim has an explicit proof surface, non-proving status, or named deferral before WP-15 convergence",
            "runnable_demo_command",
            &[
                "docs/milestones/v0.90.3/FEATURE_PROOF_COVERAGE_v0.90.3.md",
                "docs/milestones/v0.90.3/DEMO_MATRIX_v0.90.3.md",
                RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH,
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture",
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0903/feature-proof-coverage.json",
            ],
            "proving",
        ),
        entry(
            "D14",
            "WP-14A",
            "The Observatory multimode UI architecture is landed as a design artifact, not as runtime UI proof",
            "documented_non_runtime_design_artifact",
            &[
                "docs/milestones/v0.90.3/OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md",
                "docs/milestones/v0.90.3/assets/csm_observatory_multimode_ui_mockups.png",
            ],
            &[
                "test -f docs/milestones/v0.90.3/OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md",
                "test -f docs/milestones/v0.90.3/assets/csm_observatory_multimode_ui_mockups.png",
            ],
            "non_proving_design_boundary",
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
        working_demo_command: validation_refs.join(" && "),
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
    if entries.len() != 14 {
        return Err(anyhow!(
            "feature proof coverage must include D1 through D14"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (index, entry) in entries.iter().enumerate() {
        let expected = format!("D{}", index + 1);
        if entry.feature_id != expected {
            return Err(anyhow!(
                "feature proof coverage entries must be ordered D1 through D14"
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
        validate_nonempty_text(
            &entry.working_demo_command,
            "feature_proof.entry.working_demo_command",
        )?;
        match entry.coverage_kind.as_str() {
            "runnable_demo_command"
            | "test_backed_proof_packet"
            | "fixture_backed_artifact"
            | "documented_non_runtime_design_artifact" => {}
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
        match entry.status.as_str() {
            "proving" | "non_proving_design_boundary" => {}
            other => {
                return Err(anyhow!(
                    "unsupported feature proof coverage status '{other}'"
                ))
            }
        }
        if entry.coverage_kind == "documented_non_runtime_design_artifact"
            && entry.status != "non_proving_design_boundary"
        {
            return Err(anyhow!(
                "documented design artifacts must preserve a non-proving design-boundary status"
            ));
        }
        if entry.status != "proving"
            && entry.coverage_kind != "documented_non_runtime_design_artifact"
        {
            return Err(anyhow!(
                "non-proving feature proof coverage entries must be documented design artifacts or explicit deferrals"
            ));
        }
    }
    Ok(())
}
