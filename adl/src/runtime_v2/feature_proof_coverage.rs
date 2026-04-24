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
            coverage_id: "v0-90-4-feature-proof-coverage-0001".to_string(),
            demo_id: "D13".to_string(),
            milestone: "v0.90.4".to_string(),
            artifact_path: RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH.to_string(),
            coverage_summary:
                "D13 verifies every v0.90.4 contract-market feature claim has a reviewable proof surface, runnable demo, or explicit non-proving boundary before WP-15 convergence."
                    .to_string(),
            entries: feature_proof_entries(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture".to_string(),
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0904/feature-proof-coverage.json".to_string(),
                "git diff --check".to_string(),
            ],
            non_claims: vec![
                "does not implement payment settlement, Lightning, x402, banking, invoicing, tax, or legal contracting".to_string(),
                "does not implement governed tool execution, UTS, ACC, registry binding, or executor authority before v0.90.5".to_string(),
                "does not redefine citizen standing, admission, private-state inspection, continuity, or challenge authority".to_string(),
                "does not implement v0.91 moral governance, emotional civilization, or humor/wellbeing scope".to_string(),
                "does not implement v0.92 identity, capability rebinding, migration, or birthday semantics".to_string(),
                "does not implement full inter-polis economics or open-ended autonomous markets".to_string(),
            ],
            claim_boundary:
                "This packet proves v0.90.4 feature-proof coverage, not new runtime behavior beyond the referenced D1 through D12 evidence surfaces; governed-tool execution remains a non-proving v0.90.5 handoff boundary."
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
        if self.milestone != "v0.90.4" {
            return Err(anyhow!(
                "feature proof coverage must target milestone v0.90.4"
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
            .any(|claim| claim.contains("governed tool execution"))
        {
            return Err(anyhow!(
                "feature proof coverage must preserve the governed-tool non-claim"
            ));
        }
        if !self.claim_boundary.contains("D1 through D12") {
            return Err(anyhow!(
                "feature proof coverage must preserve the D1 through D12 claim boundary"
            ));
        }
        if !self.claim_boundary.contains("v0.90.5") {
            return Err(anyhow!(
                "feature proof coverage must preserve the v0.90.5 governed-tool handoff boundary"
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
            "v0.90.4 consumes the v0.90.3 citizen-state authority surfaces it depends on instead of redefining them",
            "fixture_backed_artifact",
            &["docs/milestones/v0.90.4/ECONOMICS_INHERITANCE_AND_AUTHORITY_AUDIT_v0.90.4.md"],
            &["test -f docs/milestones/v0.90.4/ECONOMICS_INHERITANCE_AND_AUTHORITY_AUDIT_v0.90.4.md"],
            "proving",
        ),
        entry(
            "D2",
            "WP-03",
            "Contract scope, parties, deliverables, process, constraints, evaluation hooks, and trace requirements are explicit and fixture-backed",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.4/features/CONTRACT_AND_BID_SCHEMA.md",
                "adl/tests/fixtures/runtime_v2/contract_market/parent_contract.json",
                "adl/tests/fixtures/runtime_v2/contract_market/contract_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_schema -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D3",
            "WP-04",
            "Bids capture proposal, cost, commitments, exceptions, and trace/signature requirements while preserving governed-tool boundaries",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.4/features/CONTRACT_AND_BID_SCHEMA.md",
                "adl/tests/fixtures/runtime_v2/contract_market/bid_alpha.json",
                "adl/tests/fixtures/runtime_v2/contract_market/bid_bravo.json",
                "adl/tests/fixtures/runtime_v2/contract_market/bid_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_bid_schema -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D4",
            "WP-05",
            "Bid selection is reviewable through mandatory checks, scoring, recommendation, and override evidence without granting tool authority",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.4/features/EVALUATION_AND_TRANSITION_AUTHORITY.md",
                "adl/tests/fixtures/runtime_v2/contract_market/evaluation_selection.json",
                "adl/tests/fixtures/runtime_v2/contract_market/selection_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_evaluation_selection -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D5",
            "WP-06 / WP-07",
            "Contract lifecycle transitions are allowed only by authorized actors and terminal states cannot be silently reopened",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.4/features/EVALUATION_AND_TRANSITION_AUTHORITY.md",
                "adl/tests/fixtures/runtime_v2/contract_market/transition_authority_matrix.json",
                "adl/tests/fixtures/runtime_v2/contract_market/transition_authority_basis.json",
                "adl/tests/fixtures/runtime_v2/contract_market/transition_authority_negative_cases.json",
                "adl/tests/fixtures/runtime_v2/contract_market/contract_lifecycle_state_machine.json",
                "adl/tests/fixtures/runtime_v2/contract_market/contract_lifecycle_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_transition_authority -- --nocapture",
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_lifecycle -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D6",
            "WP-08",
            "External counterparties can participate only through explicit trust, sponsorship, gateway review, revocation, and allowed-action limits",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.4/features/COUNTERPARTY_AND_DELEGATION.md",
                "adl/tests/fixtures/runtime_v2/contract_market/external_counterparty_model.json",
                "adl/tests/fixtures/runtime_v2/contract_market/external_counterparty_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_external_counterparty -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D7",
            "WP-09",
            "Delegation and subcontracting preserve parent accountability, explicit scope, and governed-tool non-authority boundaries",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.4/features/COUNTERPARTY_AND_DELEGATION.md",
                "adl/tests/fixtures/runtime_v2/contract_market/delegation_subcontract.json",
                "adl/tests/fixtures/runtime_v2/contract_market/delegated_output.json",
                "adl/tests/fixtures/runtime_v2/contract_market/parent_integration.json",
                "adl/tests/fixtures/runtime_v2/contract_market/delegation_negative_cases.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_delegation_subcontract -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D8",
            "WP-10",
            "Contract and bid artifacts can declare bounded resource estimates without becoming payment, pricing, or executable tool rails",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.4/features/RESOURCE_STEWARDSHIP_BRIDGE.md",
                "adl/tests/fixtures/runtime_v2/contract_market/resource_stewardship_bridge.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_resource_stewardship_bridge -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D9",
            "WP-11",
            "One coherent fixture packet contains the parent contract, bids, evaluation, delegation, integration, completion, and manifest-linked deliverables",
            "fixture_backed_artifact",
            &[
                "docs/milestones/v0.90.4/features/CONTRACT_MARKET_DEMO_AND_RUNNER.md",
                "adl/tests/fixtures/runtime_v2/contract_market/parent_contract.json",
                "adl/tests/fixtures/runtime_v2/contract_market/bid_alpha.json",
                "adl/tests/fixtures/runtime_v2/contract_market/bid_bravo.json",
                "adl/tests/fixtures/runtime_v2/contract_market/evaluation_selection.json",
                "adl/tests/fixtures/runtime_v2/contract_market/delegation_subcontract.json",
                "adl/tests/fixtures/runtime_v2/contract_market/delegated_output.json",
                "adl/tests/fixtures/runtime_v2/contract_market/parent_integration.json",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo_review_surfaces_are_stable -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D10",
            "WP-12",
            "A deterministic runner validates the fixture packet, emits transition/review artifacts, and refuses unauthorized tool execution",
            "runnable_demo_command",
            &[
                "docs/milestones/v0.90.4/features/CONTRACT_MARKET_DEMO_AND_RUNNER.md",
                "runtime_v2/contract_market/proof_packet.json",
                "runtime_v2/contract_market/operator_report.md",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo -- --nocapture",
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 contract-market-demo --out artifacts/v0904/demo-d12-contract-market",
            ],
            "proving",
        ),
        entry(
            "D11",
            "WP-13",
            "Reviewers can inspect scope, participants, selection, execution, artifacts, trace, validation, caveats, and residual risk through one bounded review surface",
            "test_backed_proof_packet",
            &[
                "docs/milestones/v0.90.4/features/CONTRACT_MARKET_DEMO_AND_RUNNER.md",
                "runtime_v2/contract_market/review_summary_seed.md",
                "runtime_v2/contract_market/operator_report.md",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo_review_surfaces_are_stable -- --nocapture",
            ],
            "proving",
        ),
        entry(
            "D12",
            "WP-14",
            "One bounded contract-market demo proves award, acceptance, delegation, integration, completion, reviewer summary, and negative denial coverage end to end",
            "runnable_demo_command",
            &[
                "docs/milestones/v0.90.4/features/CONTRACT_MARKET_DEMO_AND_RUNNER.md",
                "runtime_v2/contract_market/proof_packet.json",
                "runtime_v2/contract_market/negative_packet.json",
                "runtime_v2/contract_market/operator_report.md",
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo -- --nocapture",
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 contract-market-demo --out artifacts/v0904/demo-d12-contract-market",
            ],
            "proving",
        ),
        entry(
            "D13",
            "WP-14A",
            "Every v0.90.4 feature claim has an explicit proof surface, runnable demo, or explicit non-proving boundary before WP-15 convergence",
            "runnable_demo_command",
            &[
                "docs/milestones/v0.90.4/FEATURE_PROOF_COVERAGE_v0.90.4.md",
                "docs/milestones/v0.90.4/DEMO_MATRIX_v0.90.4.md",
                RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH,
            ],
            &[
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture",
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0904/feature-proof-coverage.json",
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
    if entries.len() != 13 {
        return Err(anyhow!(
            "feature proof coverage must include D1 through D13"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (index, entry) in entries.iter().enumerate() {
        let expected = format!("D{}", index + 1);
        if entry.feature_id != expected {
            return Err(anyhow!(
                "feature proof coverage entries must be ordered D1 through D13"
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
