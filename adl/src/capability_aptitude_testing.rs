use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub const CAPABILITY_APTITUDE_TESTING_SCHEMA_VERSION: &str = "capability_aptitude_testing.v1";
pub const CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT: &str =
    "docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilitySubjectManifest {
    pub subject_id: String,
    pub subject_type: String,
    pub model: String,
    pub provider: String,
    pub model_version_or_tag: String,
    pub runtime_environment: String,
    pub tool_access: Vec<String>,
    pub skill_scaffolding: Vec<String>,
    pub context_limits: String,
    pub privacy_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityTestManifestRecord {
    pub test_family: String,
    pub fixture_id: String,
    pub fixture_version: String,
    pub task_contract: String,
    pub expected_evidence: Vec<String>,
    pub trap_cases: Vec<String>,
    pub scoring_dimensions: Vec<String>,
    pub publication_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityTestManifestPacket {
    pub schema_version: String,
    pub manifests: Vec<CapabilityTestManifestRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRunManifest {
    pub run_id: String,
    pub started_at: String,
    pub completed_at: String,
    pub operator: String,
    pub subject_id: String,
    pub test_manifest: String,
    pub output_paths: Vec<String>,
    pub validation_commands: Vec<String>,
    pub evaluator: String,
    pub rerun_of: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityScoreBand {
    Excellent,
    Strong,
    Adequate,
    Weak,
    Unsuitable,
    NotTested,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDimensionScore {
    pub dimension: String,
    pub band: CapabilityScoreBand,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityFamilyScorecard {
    pub subject_id: String,
    pub test_family: String,
    pub overall_band: CapabilityScoreBand,
    pub confidence: String,
    pub dimension_scores: Vec<CapabilityDimensionScore>,
    pub strengths: Vec<String>,
    pub failure_modes: Vec<String>,
    pub false_positive_notes: Vec<String>,
    pub false_negative_notes: Vec<String>,
    pub repair_burden: String,
    pub recommended_use: Vec<String>,
    pub discouraged_use: Vec<String>,
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityScorecardPacket {
    pub schema_version: String,
    pub evaluation_subject: String,
    pub family_scorecards: Vec<CapabilityFamilyScorecard>,
    pub limitations: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRawOutputRecord {
    pub test_family: String,
    pub fixture_id: String,
    pub observed_output_summary: String,
    pub validator_result: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityAptitudeArtifactBundle {
    pub subject_manifest: CapabilitySubjectManifest,
    pub test_manifest: CapabilityTestManifestPacket,
    pub run_manifest: CapabilityRunManifest,
    pub scorecard: CapabilityScorecardPacket,
    pub evaluator_notes: String,
    pub final_report: String,
    pub redaction_report: String,
    pub raw_outputs: BTreeMap<String, CapabilityRawOutputRecord>,
}

pub fn build_capability_aptitude_artifact_bundle() -> CapabilityAptitudeArtifactBundle {
    let subject_manifest = CapabilitySubjectManifest {
        subject_id: "fixture_subject.adl_evaluation_slice".to_string(),
        subject_type: "fixture_mode_harness".to_string(),
        model: "not_applicable".to_string(),
        provider: "deterministic_fixture".to_string(),
        model_version_or_tag: "wp09-fixture-v1".to_string(),
        runtime_environment: "repo_local_fixture_mode".to_string(),
        tool_access: vec![
            "repo_shell".to_string(),
            "structured_prompt_validators".to_string(),
            "focused_tests".to_string(),
        ],
        skill_scaffolding: vec![
            "workflow-conductor".to_string(),
            "sprint-conductor".to_string(),
            "review-subagent-policy".to_string(),
        ],
        context_limits: "bounded_issue_local_fixture_inputs".to_string(),
        privacy_mode: "redacted_fixture_only".to_string(),
    };

    let test_manifest = CapabilityTestManifestPacket {
        schema_version: CAPABILITY_APTITUDE_TESTING_SCHEMA_VERSION.to_string(),
        manifests: vec![
            CapabilityTestManifestRecord {
                test_family: "contract_following".to_string(),
                fixture_id: "wp09-contract-following-fixture".to_string(),
                fixture_version: "v1".to_string(),
                task_contract: "Correct or classify bounded card-state drift without inventing progress.".to_string(),
                expected_evidence: vec![
                    "structured card normalization".to_string(),
                    "blocked-state honesty".to_string(),
                    "scope restraint".to_string(),
                ],
                trap_cases: vec![
                    "dirty root checkout temptation".to_string(),
                    "merged-claim without PR".to_string(),
                ],
                scoring_dimensions: vec![
                    "structural_compliance".to_string(),
                    "instruction_fidelity".to_string(),
                    "scope_control".to_string(),
                ],
                publication_status: "internal_only".to_string(),
            },
            CapabilityTestManifestRecord {
                test_family: "review_aptitude".to_string(),
                fixture_id: "wp09-review-aptitude-fixture".to_string(),
                fixture_version: "v1".to_string(),
                task_contract: "Produce findings-first review output with evidence and restrained false positives.".to_string(),
                expected_evidence: vec![
                    "severity calibration".to_string(),
                    "evidence backed findings".to_string(),
                    "non-finding restraint".to_string(),
                ],
                trap_cases: vec![
                    "intentional false trail".to_string(),
                    "clean file should emit no findings".to_string(),
                ],
                scoring_dimensions: vec![
                    "true_positive_detection".to_string(),
                    "false_positive_restraint".to_string(),
                    "remediation_usefulness".to_string(),
                ],
                publication_status: "internal_only".to_string(),
            },
            CapabilityTestManifestRecord {
                test_family: "planning_aptitude".to_string(),
                fixture_id: "wp09-planning-aptitude-fixture".to_string(),
                fixture_version: "v1".to_string(),
                task_contract: "Decompose bounded work into sequenced execution without widening scope.".to_string(),
                expected_evidence: vec![
                    "dependency recognition".to_string(),
                    "non-goal clarity".to_string(),
                    "validation planning".to_string(),
                ],
                trap_cases: vec![
                    "backlog-only request should not create public issues".to_string(),
                    "v0.92 scope should remain deferred".to_string(),
                ],
                scoring_dimensions: vec![
                    "decomposition_quality".to_string(),
                    "milestone_fit".to_string(),
                    "promotion_defer_judgment".to_string(),
                ],
                publication_status: "internal_only".to_string(),
            },
        ],
    };

    let scorecard = CapabilityScorecardPacket {
        schema_version: CAPABILITY_APTITUDE_TESTING_SCHEMA_VERSION.to_string(),
        evaluation_subject: subject_manifest.subject_id.clone(),
        family_scorecards: vec![
            CapabilityFamilyScorecard {
                subject_id: subject_manifest.subject_id.clone(),
                test_family: "contract_following".to_string(),
                overall_band: CapabilityScoreBand::Strong,
                confidence: "fixture_high".to_string(),
                dimension_scores: vec![
                    CapabilityDimensionScore {
                        dimension: "structural_compliance".to_string(),
                        band: CapabilityScoreBand::Strong,
                        note: "Fixture path preserves branch/worktree truth without hidden edits."
                            .to_string(),
                    },
                    CapabilityDimensionScore {
                        dimension: "instruction_fidelity".to_string(),
                        band: CapabilityScoreBand::Strong,
                        note: "Bounded contracts remain explicit and reviewable.".to_string(),
                    },
                    CapabilityDimensionScore {
                        dimension: "scope_control".to_string(),
                        band: CapabilityScoreBand::Adequate,
                        note: "Execution still requires human review to prevent issue-overhead drift."
                            .to_string(),
                    },
                ],
                strengths: vec![
                    "Preserves issue-local workflow constraints.".to_string(),
                    "Keeps blocker/refusal paths explicit.".to_string(),
                ],
                failure_modes: vec![
                    "Workflow wrappers may hang without surfacing a refusal.".to_string(),
                ],
                false_positive_notes: vec![
                    "No universal trust claim is made from fixture-mode execution.".to_string(),
                ],
                false_negative_notes: vec![
                    "Live operator friction is only partially modeled in fixture mode.".to_string(),
                ],
                repair_burden: "bounded manual cleanup when wrappers drift".to_string(),
                recommended_use: vec![
                    "Early lifecycle contract checks".to_string(),
                    "Sprint readiness gating".to_string(),
                ],
                discouraged_use: vec![
                    "Universal model ranking".to_string(),
                    "Autonomous release approval".to_string(),
                ],
                caveats: vec![
                    "Fixture mode measures bounded contract behavior, not broad intelligence."
                        .to_string(),
                ],
            },
            CapabilityFamilyScorecard {
                subject_id: subject_manifest.subject_id.clone(),
                test_family: "review_aptitude".to_string(),
                overall_band: CapabilityScoreBand::Strong,
                confidence: "fixture_medium".to_string(),
                dimension_scores: vec![
                    CapabilityDimensionScore {
                        dimension: "true_positive_detection".to_string(),
                        band: CapabilityScoreBand::Strong,
                        note: "Findings-first review packet shape is preserved.".to_string(),
                    },
                    CapabilityDimensionScore {
                        dimension: "false_positive_restraint".to_string(),
                        band: CapabilityScoreBand::Adequate,
                        note: "Still depends on explicit review policy and human merge review."
                            .to_string(),
                    },
                    CapabilityDimensionScore {
                        dimension: "remediation_usefulness".to_string(),
                        band: CapabilityScoreBand::Strong,
                        note: "Issue-local remediation direction remains concrete and bounded."
                            .to_string(),
                    },
                ],
                strengths: vec![
                    "Can express evidence-backed findings cleanly.".to_string(),
                    "Supports bounded subagent review loops.".to_string(),
                ],
                failure_modes: vec![
                    "Review value drops when lifecycle truth surfaces drift.".to_string(),
                ],
                false_positive_notes: vec![
                    "Fixture avoids grading prose flair as review quality.".to_string(),
                ],
                false_negative_notes: vec![
                    "Does not yet model full repo-wide review breadth.".to_string(),
                ],
                repair_burden: "moderate when stale cards or review packets must be normalized"
                    .to_string(),
                recommended_use: vec![
                    "Pre-PR review packet comparisons".to_string(),
                    "Evidence-backed review skill trials".to_string(),
                ],
                discouraged_use: vec![
                    "Security certification".to_string(),
                    "Customer-facing leaderboard claims".to_string(),
                ],
                caveats: vec![
                    "This slice proves reportability and scoring shape, not complete review coverage."
                        .to_string(),
                ],
            },
            CapabilityFamilyScorecard {
                subject_id: subject_manifest.subject_id.clone(),
                test_family: "planning_aptitude".to_string(),
                overall_band: CapabilityScoreBand::Adequate,
                confidence: "fixture_medium".to_string(),
                dimension_scores: vec![
                    CapabilityDimensionScore {
                        dimension: "decomposition_quality".to_string(),
                        band: CapabilityScoreBand::Strong,
                        note: "Sprint/issue sequencing is explicit and reusable.".to_string(),
                    },
                    CapabilityDimensionScore {
                        dimension: "milestone_fit".to_string(),
                        band: CapabilityScoreBand::Adequate,
                        note: "Milestone discipline still needs active truth checks.".to_string(),
                    },
                    CapabilityDimensionScore {
                        dimension: "promotion_defer_judgment".to_string(),
                        band: CapabilityScoreBand::Adequate,
                        note: "Tooling issues must still be surfaced separately to avoid sprint drag."
                            .to_string(),
                    },
                ],
                strengths: vec![
                    "Captures dependencies and non-goals in durable cards.".to_string(),
                    "Supports a reportable, reviewable planning trail.".to_string(),
                ],
                failure_modes: vec![
                    "Workflow instability can consume sprint time before issue execution starts."
                        .to_string(),
                ],
                false_positive_notes: vec![
                    "Does not confuse planning polish with successful execution.".to_string(),
                ],
                false_negative_notes: vec![
                    "Cannot yet score long-horizon delivery outcomes automatically.".to_string(),
                ],
                repair_burden: "moderate when sprint state or closeout truth drifts".to_string(),
                recommended_use: vec![
                    "Sprint planning comparisons".to_string(),
                    "Issue-wave readiness trials".to_string(),
                ],
                discouraged_use: vec![
                    "Universal capability ranking".to_string(),
                    "Standalone product marketing claims".to_string(),
                ],
                caveats: vec![
                    "Planning aptitude here is bounded to ADL workflow execution, not general strategy."
                        .to_string(),
                ],
            },
        ],
        limitations: vec![
            "Fixture mode does not compare real provider/model runs yet.".to_string(),
            "WP-10 still owns intelligence-specific metric architecture.".to_string(),
            "No reputation or leaderboard output is produced in this slice.".to_string(),
        ],
        non_claims: vec![
            "Does not prove universal intelligence.".to_string(),
            "Does not certify production fitness.".to_string(),
            "Does not publish a public leaderboard row.".to_string(),
        ],
    };

    let raw_outputs = BTreeMap::from([
        (
            "contract_following.json".to_string(),
            CapabilityRawOutputRecord {
                test_family: "contract_following".to_string(),
                fixture_id: "wp09-contract-following-fixture".to_string(),
                observed_output_summary:
                    "Subject classified bounded lifecycle drift and preserved blocker truth."
                        .to_string(),
                validator_result: "PASS".to_string(),
                notes: vec![
                    "No root-checkout mutation was accepted.".to_string(),
                    "Scope remained issue-local.".to_string(),
                ],
            },
        ),
        (
            "review_aptitude.json".to_string(),
            CapabilityRawOutputRecord {
                test_family: "review_aptitude".to_string(),
                fixture_id: "wp09-review-aptitude-fixture".to_string(),
                observed_output_summary:
                    "Subject emitted evidence-backed findings-first review output with bounded caveats."
                        .to_string(),
                validator_result: "PASS".to_string(),
                notes: vec![
                    "Severity remained visible.".to_string(),
                    "No invented files or unsupported claims.".to_string(),
                ],
            },
        ),
        (
            "planning_aptitude.json".to_string(),
            CapabilityRawOutputRecord {
                test_family: "planning_aptitude".to_string(),
                fixture_id: "wp09-planning-aptitude-fixture".to_string(),
                observed_output_summary:
                    "Subject sequenced bounded work, validation, and non-goals without widening to v0.92."
                        .to_string(),
                validator_result: "PASS".to_string(),
                notes: vec![
                    "Dependencies remained explicit.".to_string(),
                    "Public-issue creation was not treated as a backlog default.".to_string(),
                ],
            },
        ),
    ]);

    let run_manifest = CapabilityRunManifest {
        run_id: "wp09-capability-aptitude-fixture-run-v1".to_string(),
        started_at: "2026-05-09T00:00:00Z".to_string(),
        completed_at: "2026-05-09T00:00:00Z".to_string(),
        operator: "codex".to_string(),
        subject_id: subject_manifest.subject_id.clone(),
        test_manifest: "test_manifest.json".to_string(),
        output_paths: vec![
            "subject_manifest.json".to_string(),
            "test_manifest.json".to_string(),
            "run_manifest.json".to_string(),
            "scorecard.json".to_string(),
            "final_report.md".to_string(),
            "evaluator_notes.md".to_string(),
            "redaction_report.md".to_string(),
            "raw_outputs/contract_following.json".to_string(),
            "raw_outputs/review_aptitude.json".to_string(),
            "raw_outputs/planning_aptitude.json".to_string(),
        ],
        validation_commands: vec![
            "cargo test --manifest-path adl/Cargo.toml capability_aptitude_testing -- --nocapture"
                .to_string(),
            "cargo test --manifest-path adl/Cargo.toml demo_v0911_capability_aptitude_testing -- --nocapture"
                .to_string(),
            "git diff --check".to_string(),
        ],
        evaluator: "adl.wp09.fixture_harness".to_string(),
        rerun_of: None,
    };

    let evaluator_notes = [
        "# Evaluator Notes",
        "",
        "- This fixture-mode slice proves the report packet shape and deterministic harness surface.",
        "- It does not compare live subjects yet.",
        "- It distinguishes capability evidence from rank, reputation, or benchmark theater.",
    ]
    .join("\n");

    let final_report = [
        "# Executive Summary",
        "",
        "This WP-09 slice delivers the first executable capability/aptitude harness artifact for ADL.",
        "It covers contract following, review aptitude, and planning aptitude in deterministic fixture mode and emits a report packet with explicit limitations.",
        "",
        "## Task Family",
        "",
        "Capability and aptitude fixture families for ADL workflow work.",
        "",
        "## Subjects Tested",
        "",
        "- fixture_subject.adl_evaluation_slice (`deterministic_fixture` / `wp09-fixture-v1`)",
        "",
        "## Test Fixture Summary",
        "",
        "- contract_following",
        "- review_aptitude",
        "- planning_aptitude",
        "",
        "## Scorecard Table",
        "",
        "| Family | Band | Confidence |",
        "| --- | --- | --- |",
        "| contract_following | Strong | fixture_high |",
        "| review_aptitude | Strong | fixture_medium |",
        "| planning_aptitude | Adequate | fixture_medium |",
        "",
        "## Findings By Subject",
        "",
        "The fixture subject preserves workflow constraints, review structure, and bounded planning shape, but still requires human oversight when workflow wrappers drift or closeout truth diverges.",
        "",
        "## Strengths",
        "",
        "- preserves issue-local workflow constraints",
        "- supports findings-first review output",
        "- keeps dependencies and non-goals explicit in planning",
        "",
        "## Failure Modes",
        "",
        "- wrapper hangs can consume sprint time before issue execution starts",
        "- stale card truth reduces review and planning signal quality",
        "",
        "## Repair Burden",
        "",
        "Repair burden is bounded but real: manual cleanup is still needed when wrapper behavior or sprint-state truth drifts.",
        "",
        "## Recommended Use",
        "",
        "- internal sprint readiness checks",
        "- pre-PR workflow and review trials",
        "",
        "## Discouraged Use",
        "",
        "- universal intelligence ranking",
        "- public reputation scoreboard",
        "",
        "## Caveats",
        "",
        "- fixture mode only",
        "- no live provider/model comparison yet",
        "- WP-10 still owns intelligence-specific metric architecture",
        "",
        "## Evidence Appendix",
        "",
        "- subject_manifest.json",
        "- test_manifest.json",
        "- scorecard.json",
        "- raw_outputs/*.json",
        "",
        "## Publication Boundary",
        "",
        "Internal only. This slice must not be presented as a public leaderboard or a universal model score.",
    ]
    .join("\n");

    let redaction_report = [
        "# Redaction Report",
        "",
        "- publication_status: internal_only",
        "- private prompts: none included",
        "- customer code: none included",
        "- absolute host paths: none included",
        "- raw outputs limited to fixture summaries and validator notes",
    ]
    .join("\n");

    CapabilityAptitudeArtifactBundle {
        subject_manifest,
        test_manifest,
        run_manifest,
        scorecard,
        evaluator_notes,
        final_report,
        redaction_report,
        raw_outputs,
    }
}

pub fn write_capability_aptitude_artifact_bundle(
    root: &Path,
) -> Result<CapabilityAptitudeArtifactBundle> {
    let bundle = build_capability_aptitude_artifact_bundle();
    fs::create_dir_all(root.join("raw_outputs")).with_context(|| {
        format!(
            "create capability harness artifact root '{}'",
            root.display()
        )
    })?;

    write_pretty_json(root.join("subject_manifest.json"), &bundle.subject_manifest)?;
    write_pretty_json(root.join("test_manifest.json"), &bundle.test_manifest)?;
    write_pretty_json(root.join("run_manifest.json"), &bundle.run_manifest)?;
    write_pretty_json(root.join("scorecard.json"), &bundle.scorecard)?;
    fs::write(root.join("evaluator_notes.md"), &bundle.evaluator_notes)
        .with_context(|| format!("write evaluator notes under '{}'", root.display()))?;
    fs::write(root.join("final_report.md"), &bundle.final_report)
        .with_context(|| format!("write final report under '{}'", root.display()))?;
    fs::write(root.join("redaction_report.md"), &bundle.redaction_report)
        .with_context(|| format!("write redaction report under '{}'", root.display()))?;
    for (file_name, raw_output) in &bundle.raw_outputs {
        write_pretty_json(root.join("raw_outputs").join(file_name), raw_output)?;
    }
    Ok(bundle)
}

fn write_pretty_json<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    let path = path.as_ref();
    let body = serde_json::to_string_pretty(value)
        .with_context(|| format!("serialize json artifact '{}'", path.display()))?;
    fs::write(path, body).with_context(|| format!("write json artifact '{}'", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        build_capability_aptitude_artifact_bundle, write_capability_aptitude_artifact_bundle,
        CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT, CAPABILITY_APTITUDE_TESTING_SCHEMA_VERSION,
    };
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("{prefix}-{nanos}"));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn collect_files(root: &Path) -> BTreeMap<String, String> {
        fn walk(base: &Path, current: &Path, out: &mut BTreeMap<String, String>) {
            let mut entries = fs::read_dir(current)
                .expect("read dir")
                .map(|entry| entry.expect("entry").path())
                .collect::<Vec<_>>();
            entries.sort();
            for path in entries {
                if path.is_dir() {
                    walk(base, &path, out);
                } else {
                    let rel = path
                        .strip_prefix(base)
                        .expect("relative path")
                        .to_string_lossy()
                        .to_string();
                    out.insert(rel, fs::read_to_string(&path).expect("read file"));
                }
            }
        }
        let mut out = BTreeMap::new();
        walk(root, root, &mut out);
        out
    }

    #[test]
    fn capability_aptitude_testing_bundle_covers_three_families_in_order() {
        let bundle = build_capability_aptitude_artifact_bundle();
        let families = bundle
            .test_manifest
            .manifests
            .iter()
            .map(|manifest| manifest.test_family.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            families,
            vec!["contract_following", "review_aptitude", "planning_aptitude"]
        );
    }

    #[test]
    fn capability_aptitude_testing_bundle_is_deterministic_and_portable() {
        let first =
            serde_json::to_string_pretty(&build_capability_aptitude_artifact_bundle().scorecard)
                .expect("serialize scorecard");
        let second =
            serde_json::to_string_pretty(&build_capability_aptitude_artifact_bundle().scorecard)
                .expect("serialize scorecard twice");
        assert_eq!(first, second);
        assert!(!first.contains("/Users/"));
        assert!(first.contains(CAPABILITY_APTITUDE_TESTING_SCHEMA_VERSION));
    }

    #[test]
    fn capability_aptitude_testing_writer_emits_required_bundle_files() {
        let temp = unique_temp_dir("capability-aptitude-bundle");
        let out = temp.join("bundle");
        write_capability_aptitude_artifact_bundle(&out).expect("write bundle");

        for relative in [
            "subject_manifest.json",
            "test_manifest.json",
            "run_manifest.json",
            "scorecard.json",
            "evaluator_notes.md",
            "final_report.md",
            "redaction_report.md",
            "raw_outputs/contract_following.json",
            "raw_outputs/review_aptitude.json",
            "raw_outputs/planning_aptitude.json",
        ] {
            assert!(out.join(relative).exists(), "missing {}", relative);
        }

        let report = fs::read_to_string(out.join("final_report.md")).expect("read report");
        for section in [
            "# Executive Summary",
            "## Scorecard Table",
            "## Publication Boundary",
        ] {
            assert!(report.contains(section), "missing report section {section}");
        }
    }

    #[test]
    fn capability_aptitude_testing_artifact_root_is_repo_relative() {
        assert!(CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT.starts_with("docs/"));
        assert!(!CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT.starts_with('/'));
    }

    #[test]
    fn capability_aptitude_testing_tracked_bundle_matches_generated_bundle() {
        let temp = unique_temp_dir("capability-aptitude-tracked");
        let generated = temp.join("bundle");
        write_capability_aptitude_artifact_bundle(&generated).expect("write generated bundle");

        let tracked_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root parent")
            .join(CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT);
        let tracked_files = collect_files(&tracked_root);
        let generated_files = collect_files(&generated);
        assert_eq!(tracked_files, generated_files);
    }
}
