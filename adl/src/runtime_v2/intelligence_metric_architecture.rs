//! Runtime-v2 intelligence metric architecture packet for v0.91.1.
//!
//! This packet defines evidence-bound intelligence metric surfaces over the
//! landed capability and Theory-of-Mind packets without collapsing into
//! reputation, productivity scoring, or universal-intelligence claims.

use super::*;
use crate::capability_aptitude_testing::{
    build_capability_aptitude_artifact_bundle, CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT,
};
use std::fs;
use std::path::Path;

pub const RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_SCHEMA: &str =
    "runtime_v2.intelligence_metric_architecture_packet.v1";
pub const RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_PATH: &str =
    "runtime_v2/intelligence/intelligence_metric_architecture.json";
pub const RUNTIME_V2_INTELLIGENCE_METRIC_REPORT_ROOT: &str =
    "docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2IntelligenceEvidenceSurface {
    pub surface_kind: String,
    pub artifact_ref: String,
    pub evidence_role: String,
    pub metric_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2IntelligenceMetricDimension {
    pub dimension_id: String,
    pub display_name: String,
    pub evidence_refs: Vec<String>,
    pub aggregation_rule: String,
    pub interpretation_boundary: String,
    pub prohibited_uses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CognitiveCompressionCostBoundary {
    pub metric_id: String,
    pub evidence_refs: Vec<String>,
    pub cost_axes: Vec<String>,
    pub bounded_interpretation: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2IntelligenceFixtureReportRef {
    pub artifact_ref: String,
    pub proving_surface: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2IntelligenceMetricArchitecturePacket {
    pub schema_version: String,
    pub metric_architecture_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub capability_dependency_ref: String,
    pub theory_of_mind_dependency_ref: String,
    pub evidence_requirements: Vec<String>,
    pub evidence_surfaces: Vec<RuntimeV2IntelligenceEvidenceSurface>,
    pub metric_dimensions: Vec<RuntimeV2IntelligenceMetricDimension>,
    pub cognitive_compression_cost: RuntimeV2CognitiveCompressionCostBoundary,
    pub fixture_reports: Vec<RuntimeV2IntelligenceFixtureReportRef>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

impl RuntimeV2IntelligenceMetricArchitecturePacket {
    pub fn prototype() -> Result<Self> {
        let tom = runtime_v2_theory_of_mind_foundation_contract()?;
        let capability = build_capability_aptitude_artifact_bundle();

        let packet = Self {
            schema_version: RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_SCHEMA.to_string(),
            metric_architecture_id: "intelligence-metric-architecture-v0-91-1-wp-10".to_string(),
            milestone: "v0.91.1".to_string(),
            wp: "WP-10".to_string(),
            artifact_path: RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_PATH.to_string(),
            source_feature_doc:
                "docs/milestones/v0.91.1/features/INTELLIGENCE_METRIC_ARCHITECTURE.md"
                    .to_string(),
            capability_dependency_ref: CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT.to_string(),
            theory_of_mind_dependency_ref: tom.artifact_path.clone(),
            evidence_requirements: vec![
                "Metrics must derive from explicit traces or test artifacts.".to_string(),
                "Capability evidence, uncertainty, and review context must stay visible."
                    .to_string(),
                "Metrics must not collapse into reputation, productivity punishment, or identity replacement.".to_string(),
            ],
            evidence_surfaces: evidence_surfaces(&tom, &capability),
            metric_dimensions: metric_dimensions(&tom, &capability),
            cognitive_compression_cost: cognitive_compression_cost_boundary(),
            fixture_reports: fixture_reports(),
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_intelligence_metric_architecture -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml capability_aptitude_testing -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_theory_of_mind_foundation -- --nocapture".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "WP-10 proves one bounded evidence-bound intelligence metric architecture over landed capability and Theory-of-Mind artifacts. It makes limitations explicit, preserves uncertainty and review context, and does not claim universal intelligence, production fitness, or punitive productivity ranking."
                    .to_string(),
            non_claims: vec![
                "does not prove universal intelligence".to_string(),
                "does not create a public leaderboard or reputation score".to_string(),
                "does not convert intelligence metrics into productivity punishment or policy authority".to_string(),
                "does not replace v0.92 identity or birthday readiness work".to_string(),
            ],
        };
        packet.validate_against(&tom)?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_SCHEMA {
            return Err(anyhow!(
                "unsupported intelligence metric architecture schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.metric_architecture_id.clone(),
            "intelligence_metric.metric_architecture_id",
        )?;
        if self.milestone != "v0.91.1" {
            return Err(anyhow!(
                "intelligence metric architecture must target milestone v0.91.1"
            ));
        }
        if self.wp != "WP-10" {
            return Err(anyhow!(
                "intelligence metric architecture must remain bound to WP-10"
            ));
        }
        validate_relative_path(&self.artifact_path, "intelligence_metric.artifact_path")?;
        if self.source_feature_doc
            != "docs/milestones/v0.91.1/features/INTELLIGENCE_METRIC_ARCHITECTURE.md"
        {
            return Err(anyhow!(
                "intelligence metric architecture must point at the v0.91.1 feature doc"
            ));
        }
        validate_relative_path(
            &self.source_feature_doc,
            "intelligence_metric.source_feature_doc",
        )?;
        if self.capability_dependency_ref != CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT {
            return Err(anyhow!(
                "intelligence metric architecture must depend on the landed capability harness artifact root"
            ));
        }
        if self.theory_of_mind_dependency_ref != RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_PATH {
            return Err(anyhow!(
                "intelligence metric architecture must depend on the landed theory-of-mind packet"
            ));
        }
        validate_requirement_list(
            &self.evidence_requirements,
            "intelligence_metric.evidence_requirements",
        )?;
        ensure_required_substring(
            &self.evidence_requirements,
            "explicit traces or test artifacts",
            "intelligence metric architecture must require explicit evidence",
        )?;
        ensure_required_substring(
            &self.evidence_requirements,
            "reputation",
            "intelligence metric architecture must forbid reputation collapse",
        )?;
        validate_evidence_surfaces(&self.evidence_surfaces)?;
        validate_metric_dimensions(&self.metric_dimensions)?;
        validate_cognitive_compression_cost(&self.cognitive_compression_cost)?;
        validate_fixture_reports(&self.fixture_reports)?;
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_intelligence_metric_architecture"))
        {
            return Err(anyhow!(
                "intelligence metric architecture must include its focused validation command"
            ));
        }
        if !self
            .claim_boundary
            .contains("does not claim universal intelligence")
        {
            return Err(anyhow!(
                "intelligence metric architecture must preserve the universal-intelligence boundary"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("public leaderboard"))
        {
            return Err(anyhow!(
                "intelligence metric architecture must preserve the no-leaderboard non-claim"
            ));
        }
        Ok(())
    }

    pub fn validate_against(&self, tom: &RuntimeV2TheoryOfMindFoundationPacket) -> Result<()> {
        self.validate()?;
        tom.validate()?;
        if self.theory_of_mind_dependency_ref != tom.artifact_path {
            return Err(anyhow!(
                "intelligence metric architecture ToM dependency drifted from the landed packet"
            ));
        }
        let capability_bundle = build_capability_aptitude_artifact_bundle();
        let expected_surfaces = evidence_surfaces(tom, &capability_bundle);
        if self.evidence_surfaces != expected_surfaces {
            return Err(anyhow!(
                "intelligence metric architecture evidence surfaces must stay aligned with the landed capability and ToM artifacts"
            ));
        }
        for dimension in &self.metric_dimensions {
            for evidence_ref in &dimension.evidence_refs {
                if !evidence_ref_allowed(&expected_surfaces, &capability_bundle, evidence_ref) {
                    return Err(anyhow!(
                        "metric dimension '{}' references unknown evidence '{}'",
                        dimension.dimension_id,
                        evidence_ref
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create parent directory '{}'", parent.display()))?;
        }
        fs::write(path, self.pretty_json_bytes()?).with_context(|| {
            format!(
                "write intelligence metric architecture to '{}'",
                path.display()
            )
        })
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.write_to_path(
            root.as_ref()
                .join(RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_PATH),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2IntelligenceMetricFixtureBundle {
    pub scorecard_json: String,
    pub final_report_md: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RuntimeV2IntelligenceMetricFixtureScorecard {
    schema_version: String,
    metric_architecture_ref: String,
    dimensions: Vec<RuntimeV2IntelligenceMetricFixtureDimension>,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RuntimeV2IntelligenceMetricFixtureDimension {
    dimension_id: String,
    level: String,
    evidence_refs: Vec<String>,
    interpretation_boundary: String,
}

pub fn build_intelligence_metric_fixture_bundle() -> RuntimeV2IntelligenceMetricFixtureBundle {
    let scorecard_json = serde_json::to_string_pretty(&RuntimeV2IntelligenceMetricFixtureScorecard {
        schema_version: "runtime_v2.intelligence_metric_report.v1".to_string(),
        metric_architecture_ref: RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_PATH.to_string(),
        dimensions: vec![
            RuntimeV2IntelligenceMetricFixtureDimension {
                dimension_id: "contracted_capability_evidence".to_string(),
                level: "bounded_positive".to_string(),
                evidence_refs: vec![
                    "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/scorecard.json".to_string(),
                    "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/raw_outputs/contract_following.json".to_string(),
                ],
                interpretation_boundary:
                    "Shows bounded capability evidence only; not universal intelligence."
                        .to_string(),
            },
            RuntimeV2IntelligenceMetricFixtureDimension {
                dimension_id: "uncertainty_preservation".to_string(),
                level: "bounded_positive".to_string(),
                evidence_refs: vec![
                    "adl/tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json"
                        .to_string(),
                ],
                interpretation_boundary:
                    "Shows explicit uncertainty handling remains visible in intelligence interpretation."
                        .to_string(),
            },
            RuntimeV2IntelligenceMetricFixtureDimension {
                dimension_id: "cognitive_compression_cost".to_string(),
                level: "exploratory".to_string(),
                evidence_refs: vec![
                    "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/test_manifest.json".to_string(),
                    "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/run_manifest.json".to_string(),
                ],
                interpretation_boundary:
                    "Exploratory cost framing only; not an optimization mandate or punitive productivity metric."
                        .to_string(),
            },
        ],
        non_claims: vec![
            "No public leaderboard row.".to_string(),
            "No production certification.".to_string(),
            "No universal intelligence verdict.".to_string(),
        ],
    })
    .expect("serialize intelligence scorecard");

    let final_report_md = [
        "# Intelligence Metric Fixture Report",
        "",
        "This WP-10 slice defines intelligence metrics as evidence-bound interpretation over landed capability and Theory-of-Mind artifacts.",
        "",
        "## What The Metric Does Prove",
        "",
        "- it can summarize bounded capability evidence from explicit fixture artifacts",
        "- it can preserve uncertainty and review context from Theory-of-Mind evidence",
        "- it can expose an exploratory Cognitive Compression Cost boundary without hiding its limits",
        "",
        "## What The Metric Does Not Prove",
        "",
        "- universal intelligence",
        "- identity, birthday, or personhood completion",
        "- punitive productivity judgments",
        "- reputation or leaderboard rank",
        "",
        "## Publication Boundary",
        "",
        "Internal architecture and fixture report only. This artifact must not be presented as a public ranking surface.",
    ]
    .join("\n");

    RuntimeV2IntelligenceMetricFixtureBundle {
        scorecard_json,
        final_report_md,
    }
}

pub fn write_intelligence_metric_fixture_bundle(root: &Path) -> Result<()> {
    let bundle = build_intelligence_metric_fixture_bundle();
    fs::create_dir_all(root).with_context(|| {
        format!(
            "create intelligence metric fixture root '{}'",
            root.display()
        )
    })?;
    fs::write(root.join("scorecard.json"), bundle.scorecard_json).with_context(|| {
        format!(
            "write intelligence metric scorecard under '{}'",
            root.display()
        )
    })?;
    fs::write(root.join("final_report.md"), bundle.final_report_md).with_context(|| {
        format!(
            "write intelligence metric report under '{}'",
            root.display()
        )
    })?;
    Ok(())
}

fn evidence_ref_allowed(
    expected_surfaces: &[RuntimeV2IntelligenceEvidenceSurface],
    capability_bundle: &crate::capability_aptitude_testing::CapabilityAptitudeArtifactBundle,
    evidence_ref: &str,
) -> bool {
    expected_surfaces
        .iter()
        .any(|surface| surface.artifact_ref == evidence_ref)
        || allowed_capability_artifact_refs(capability_bundle).contains(evidence_ref)
}

fn allowed_capability_artifact_refs(
    capability_bundle: &crate::capability_aptitude_testing::CapabilityAptitudeArtifactBundle,
) -> std::collections::BTreeSet<String> {
    let mut refs = std::collections::BTreeSet::from([
        format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/subject_manifest.json"),
        format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/test_manifest.json"),
        format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/run_manifest.json"),
        format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/scorecard.json"),
        format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/final_report.md"),
        format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/evaluator_notes.md"),
        format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/redaction_report.md"),
    ]);
    refs.extend(
        capability_bundle
            .raw_outputs
            .keys()
            .map(|name| format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/raw_outputs/{name}")),
    );
    refs
}

fn evidence_surfaces(
    tom: &RuntimeV2TheoryOfMindFoundationPacket,
    capability: &crate::capability_aptitude_testing::CapabilityAptitudeArtifactBundle,
) -> Vec<RuntimeV2IntelligenceEvidenceSurface> {
    vec![
        RuntimeV2IntelligenceEvidenceSurface {
            surface_kind: "capability_scorecard".to_string(),
            artifact_ref: format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/scorecard.json"),
            evidence_role: "bounded_capability_summary".to_string(),
            metric_boundary: "No leaderboard or reputation collapse.".to_string(),
        },
        RuntimeV2IntelligenceEvidenceSurface {
            surface_kind: "capability_test_manifest".to_string(),
            artifact_ref: format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/test_manifest.json"),
            evidence_role: "test_family_scope_and_limits".to_string(),
            metric_boundary: "Metric interpretation must respect fixture family scope.".to_string(),
        },
        RuntimeV2IntelligenceEvidenceSurface {
            surface_kind: "theory_of_mind_packet".to_string(),
            artifact_ref: tom.artifact_path.clone(),
            evidence_role: "uncertainty_and_review_context".to_string(),
            metric_boundary: "Intelligence metric cannot erase uncertainty or privacy boundaries."
                .to_string(),
        },
        RuntimeV2IntelligenceEvidenceSurface {
            surface_kind: "theory_of_mind_fixture".to_string(),
            artifact_ref:
                "adl/tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json"
                    .to_string(),
            evidence_role: "stable ToM evidence surface".to_string(),
            metric_boundary: "Metric derives from explicit fixture evidence only.".to_string(),
        },
        RuntimeV2IntelligenceEvidenceSurface {
            surface_kind: "capability_subject_manifest".to_string(),
            artifact_ref: format!(
                "{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/subject_manifest.json"
            ),
            evidence_role: "subject_and_runtime_constraints".to_string(),
            metric_boundary: format!(
                "Capability harness schema '{}' remains a dependency, not a rank source.",
                capability.test_manifest.schema_version
            ),
        },
    ]
}

fn metric_dimensions(
    tom: &RuntimeV2TheoryOfMindFoundationPacket,
    _capability: &crate::capability_aptitude_testing::CapabilityAptitudeArtifactBundle,
) -> Vec<RuntimeV2IntelligenceMetricDimension> {
    vec![
        RuntimeV2IntelligenceMetricDimension {
            dimension_id: "contracted_capability_evidence".to_string(),
            display_name: "Contracted Capability Evidence".to_string(),
            evidence_refs: vec![
                format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/scorecard.json"),
                format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/raw_outputs/contract_following.json"),
            ],
            aggregation_rule: "derive only from explicit fixture scorecards and raw output summaries".to_string(),
            interpretation_boundary: "Shows bounded capability evidence, not universal intelligence.".to_string(),
            prohibited_uses: vec![
                "public_ranking".to_string(),
                "reputation_scoring".to_string(),
            ],
        },
        RuntimeV2IntelligenceMetricDimension {
            dimension_id: "uncertainty_preservation".to_string(),
            display_name: "Uncertainty Preservation".to_string(),
            evidence_refs: vec![
                tom.artifact_path.clone(),
                "adl/tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json".to_string(),
            ],
            aggregation_rule: "verify intelligence summaries preserve explicit unknown, correction, and privacy-restricted states".to_string(),
            interpretation_boundary: "A high signal here means uncertainty stayed visible, not that hidden mental state was known.".to_string(),
            prohibited_uses: vec![
                "policy_override".to_string(),
                "mind_reading_claims".to_string(),
            ],
        },
        RuntimeV2IntelligenceMetricDimension {
            dimension_id: "cognitive_compression_cost".to_string(),
            display_name: "Cognitive Compression Cost".to_string(),
            evidence_refs: vec![
                format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/test_manifest.json"),
                format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/run_manifest.json"),
            ],
            aggregation_rule: "estimate explanatory burden from fixture-family count, evidence diversity, and review caveat load only".to_string(),
            interpretation_boundary: "Exploratory architecture boundary only; not an optimization mandate or labor metric.".to_string(),
            prohibited_uses: vec![
                "productivity_punishment".to_string(),
                "single_scalar_intelligence".to_string(),
            ],
        },
    ]
}

fn cognitive_compression_cost_boundary() -> RuntimeV2CognitiveCompressionCostBoundary {
    RuntimeV2CognitiveCompressionCostBoundary {
        metric_id: "cognitive_compression_cost".to_string(),
        evidence_refs: vec![
            format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/test_manifest.json"),
            format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/run_manifest.json"),
            format!("{CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT}/scorecard.json"),
        ],
        cost_axes: vec![
            "evidence_diversity".to_string(),
            "review_caveat_load".to_string(),
            "fixture_family_span".to_string(),
        ],
        bounded_interpretation:
            "Cognitive Compression Cost is an exploratory boundary for how much explicit evidence and caveat structure an intelligence summary must compress. It is not a productivity score, morale score, or universal intelligence scalar."
                .to_string(),
        non_claims: vec![
            "not a productivity metric".to_string(),
            "not a replacement for human review".to_string(),
            "not a universal intelligence scalar".to_string(),
        ],
    }
}

fn fixture_reports() -> Vec<RuntimeV2IntelligenceFixtureReportRef> {
    vec![
        RuntimeV2IntelligenceFixtureReportRef {
            artifact_ref: format!("{RUNTIME_V2_INTELLIGENCE_METRIC_REPORT_ROOT}/scorecard.json"),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_intelligence_metric_architecture -- --nocapture"
                    .to_string(),
            summary: "Machine-readable bounded intelligence metric scorecard.".to_string(),
        },
        RuntimeV2IntelligenceFixtureReportRef {
            artifact_ref: format!("{RUNTIME_V2_INTELLIGENCE_METRIC_REPORT_ROOT}/final_report.md"),
            proving_surface:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_intelligence_metric_architecture -- --nocapture"
                    .to_string(),
            summary: "Human-readable fixture report describing what the metric does and does not prove.".to_string(),
        },
    ]
}

fn validate_evidence_surfaces(surfaces: &[RuntimeV2IntelligenceEvidenceSurface]) -> Result<()> {
    if surfaces.len() != 5 {
        return Err(anyhow!(
            "intelligence metric architecture must cover exactly five evidence surfaces"
        ));
    }
    let mut kinds = std::collections::BTreeSet::new();
    for surface in surfaces {
        validate_nonempty_text(&surface.surface_kind, "intelligence_metric.surface_kind")?;
        validate_relative_path(&surface.artifact_ref, "intelligence_metric.artifact_ref")?;
        validate_nonempty_text(&surface.evidence_role, "intelligence_metric.evidence_role")?;
        validate_nonempty_text(
            &surface.metric_boundary,
            "intelligence_metric.metric_boundary",
        )?;
        kinds.insert(surface.surface_kind.as_str());
    }
    for required in [
        "capability_scorecard",
        "capability_test_manifest",
        "theory_of_mind_packet",
        "theory_of_mind_fixture",
        "capability_subject_manifest",
    ] {
        if !kinds.contains(required) {
            return Err(anyhow!(
                "intelligence metric architecture missing required evidence surface '{required}'"
            ));
        }
    }
    Ok(())
}

fn validate_metric_dimensions(dimensions: &[RuntimeV2IntelligenceMetricDimension]) -> Result<()> {
    if dimensions.len() != 3 {
        return Err(anyhow!(
            "intelligence metric architecture must define exactly three metric dimensions"
        ));
    }
    let mut ids = std::collections::BTreeSet::new();
    for dimension in dimensions {
        normalize_id(
            dimension.dimension_id.clone(),
            "intelligence_metric.dimension_id",
        )?;
        validate_nonempty_text(&dimension.display_name, "intelligence_metric.display_name")?;
        if dimension.evidence_refs.is_empty() {
            return Err(anyhow!(
                "intelligence metric dimension '{}' must cite evidence refs",
                dimension.dimension_id
            ));
        }
        for evidence_ref in &dimension.evidence_refs {
            validate_relative_path(evidence_ref, "intelligence_metric.dimension_evidence_ref")?;
        }
        validate_nonempty_text(
            &dimension.aggregation_rule,
            "intelligence_metric.aggregation_rule",
        )?;
        validate_nonempty_text(
            &dimension.interpretation_boundary,
            "intelligence_metric.interpretation_boundary",
        )?;
        if dimension.prohibited_uses.is_empty() {
            return Err(anyhow!(
                "intelligence metric dimension '{}' must declare prohibited uses",
                dimension.dimension_id
            ));
        }
        ids.insert(dimension.dimension_id.as_str());
    }
    for required in [
        "contracted_capability_evidence",
        "uncertainty_preservation",
        "cognitive_compression_cost",
    ] {
        if !ids.contains(required) {
            return Err(anyhow!(
                "intelligence metric architecture missing required dimension '{required}'"
            ));
        }
    }
    Ok(())
}

fn validate_cognitive_compression_cost(
    boundary: &RuntimeV2CognitiveCompressionCostBoundary,
) -> Result<()> {
    if boundary.metric_id != "cognitive_compression_cost" {
        return Err(anyhow!(
            "intelligence metric architecture must keep the CCC boundary id stable"
        ));
    }
    if boundary.evidence_refs.len() != 3 {
        return Err(anyhow!(
            "cognitive compression cost boundary must cite exactly three evidence refs"
        ));
    }
    for evidence_ref in &boundary.evidence_refs {
        validate_relative_path(evidence_ref, "intelligence_metric.ccc_evidence_ref")?;
    }
    if boundary.cost_axes
        != [
            "evidence_diversity",
            "review_caveat_load",
            "fixture_family_span",
        ]
    {
        return Err(anyhow!(
            "cognitive compression cost boundary must preserve the bounded cost axes"
        ));
    }
    if !boundary
        .bounded_interpretation
        .contains("not a productivity score")
    {
        return Err(anyhow!(
            "cognitive compression cost boundary must reject productivity scoring"
        ));
    }
    if !boundary
        .non_claims
        .iter()
        .any(|claim| claim.contains("universal intelligence scalar"))
    {
        return Err(anyhow!(
            "cognitive compression cost boundary must reject universal scalar claims"
        ));
    }
    Ok(())
}

fn validate_fixture_reports(reports: &[RuntimeV2IntelligenceFixtureReportRef]) -> Result<()> {
    if reports.len() != 2 {
        return Err(anyhow!(
            "intelligence metric architecture must expose exactly two fixture report artifacts"
        ));
    }
    let mut refs = std::collections::BTreeSet::new();
    for report in reports {
        validate_relative_path(
            &report.artifact_ref,
            "intelligence_metric.fixture_report_ref",
        )?;
        validate_nonempty_text(
            &report.proving_surface,
            "intelligence_metric.fixture_report_surface",
        )?;
        validate_nonempty_text(
            &report.summary,
            "intelligence_metric.fixture_report_summary",
        )?;
        refs.insert(report.artifact_ref.as_str());
    }
    for required in [
        "docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/scorecard.json",
        "docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/final_report.md",
    ] {
        if !refs.contains(required) {
            return Err(anyhow!(
                "intelligence metric architecture missing required fixture report '{}'",
                required
            ));
        }
    }
    Ok(())
}

fn validate_requirement_list(values: &[String], label: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, label)?;
    }
    Ok(())
}

fn ensure_required_substring(values: &[String], needle: &str, err: &str) -> Result<()> {
    if values.iter().any(|value| value.contains(needle)) {
        Ok(())
    } else {
        Err(anyhow!(err.to_string()))
    }
}
