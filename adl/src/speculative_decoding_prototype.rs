use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const SPECULATIVE_DECODING_PROTOTYPE_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/speculative_decoding/speculative_decoding_prototype_report.json";
pub const SPECULATIVE_DECODING_PROTOTYPE_SCHEMA_VERSION: &str = "speculative_decoding_prototype.v1";
pub const SPECULATIVE_DECODING_PROTOTYPE_PROMPT_VERSION: &str =
    "wp11.speculative_decoding_prototype.v1";
#[cfg(test)]
const HOST_PATH_MARKER: &str = "/Users/daniel/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SpeculativeDecodingPromptRecord {
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub decision_question: &'static str,
    pub boundary_summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeculativeScenarioStatus {
    ProvingWorthContinuing,
    ProvingNotWorthContinuing,
    NonProvingTokenizerMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeculativeWorthinessDecision {
    ContinueForSameFamilyLocalBackends,
    RejectCrossFamilyGenericAcceleration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeculativeTraceStep {
    pub step_index: usize,
    pub proposed_tokens: Vec<String>,
    pub accepted_tokens: Vec<String>,
    pub rejected_token: Option<String>,
    pub authoritative_token: Option<String>,
    pub target_tokens_verified: usize,
    pub fallback_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpeculativeScenarioReport {
    pub scenario_id: &'static str,
    pub workload_class: &'static str,
    pub draft_profile: &'static str,
    pub status: SpeculativeScenarioStatus,
    pub worthwhile: bool,
    pub baseline_total_ms: u64,
    pub speculative_total_ms: u64,
    pub baseline_tokens_per_sec: f64,
    pub speculative_tokens_per_sec: f64,
    pub speedup_tps_ratio: f64,
    pub draft_tokens_proposed: usize,
    pub draft_tokens_accepted: usize,
    pub acceptance_rate: f64,
    pub baseline_target_forward_passes: usize,
    pub speculative_target_forward_passes: usize,
    pub replay_safe: bool,
    pub authority_boundary: &'static str,
    pub evaluation_summary: String,
    pub committed_output: Vec<String>,
    pub trace_steps: Vec<SpeculativeTraceStep>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpeculativeDecodingPrototypeReport {
    pub schema_version: &'static str,
    pub prompt_record: SpeculativeDecodingPromptRecord,
    pub worthwhile_for_adl: bool,
    pub recommendation: SpeculativeWorthinessDecision,
    pub scenarios: Vec<SpeculativeScenarioReport>,
    pub commit_authority_rules: Vec<&'static str>,
    pub fallback_rules: Vec<&'static str>,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<&'static str>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone)]
struct CostModel {
    baseline_target_ms_per_token: u64,
    draft_ms_per_token: u64,
    verification_base_ms: u64,
    verification_ms_per_proposed_token: u64,
}

#[derive(Debug, Clone)]
struct SimulationScenario<'a> {
    scenario_id: &'static str,
    workload_class: &'static str,
    draft_profile: &'static str,
    target_tokens: Vec<&'a str>,
    draft_blocks: Vec<Vec<&'a str>>,
    cost_model: CostModel,
}

fn prompt_record() -> SpeculativeDecodingPromptRecord {
    SpeculativeDecodingPromptRecord {
        prompt_version: SPECULATIVE_DECODING_PROTOTYPE_PROMPT_VERSION,
        issue_number: 3010,
        decision_question:
            "Is speculative decoding worth continuing for ADL workloads under deterministic commit semantics?",
        boundary_summary:
            "WP-11 evaluates bounded draft/verify/commit acceleration posture only; speculative proposal never grants execution authority and Freedom Gate plus ACC remain the side-effect boundary.",
    }
}

fn same_family_code_generation_scenario() -> SimulationScenario<'static> {
    SimulationScenario {
        scenario_id: "same_family_code_generation",
        workload_class: "code_generation",
        draft_profile: "same_family_high_acceptance",
        target_tokens: vec![
            "fn", "add", "(", "a", ",", "b", ")", "{", "a", "+", "b", "}",
        ],
        draft_blocks: vec![
            vec!["fn", "add", "("],
            vec!["a", ",", "b"],
            vec![")", "{", "a"],
            vec!["+", "b", "}"],
        ],
        cost_model: CostModel {
            baseline_target_ms_per_token: 12,
            draft_ms_per_token: 1,
            verification_base_ms: 4,
            verification_ms_per_proposed_token: 3,
        },
    }
}

fn long_form_perfect_scenario() -> SimulationScenario<'static> {
    SimulationScenario {
        scenario_id: "perfect_long_form_generation",
        workload_class: "long_form_generation",
        draft_profile: "perfect_mock_draft",
        target_tokens: vec![
            "bounded",
            "speculative",
            "reasoning",
            "stays",
            "provisional",
            "until",
            "verified",
            "and",
            "committed",
            "by",
            "the",
            "target",
        ],
        draft_blocks: vec![
            vec!["bounded", "speculative", "reasoning"],
            vec!["stays", "provisional", "until"],
            vec!["verified", "and", "committed"],
            vec!["by", "the", "target"],
        ],
        cost_model: CostModel {
            baseline_target_ms_per_token: 12,
            draft_ms_per_token: 1,
            verification_base_ms: 4,
            verification_ms_per_proposed_token: 3,
        },
    }
}

fn adl_card_generation_mixed_scenario() -> SimulationScenario<'static> {
    SimulationScenario {
        scenario_id: "adl_card_generation_mixed_quality",
        workload_class: "adl_card_generation",
        draft_profile: "same_family_mixed_acceptance",
        target_tokens: vec![
            "goal",
            "required",
            "outcome",
            "acceptance",
            "criteria",
            "validation",
            "plan",
            "proof",
            "boundary",
        ],
        draft_blocks: vec![
            vec!["goal", "required", "outcome"],
            vec!["acceptance", "criterion", "validation"],
            vec!["plan", "proof", "boundary"],
        ],
        cost_model: CostModel {
            baseline_target_ms_per_token: 12,
            draft_ms_per_token: 1,
            verification_base_ms: 4,
            verification_ms_per_proposed_token: 3,
        },
    }
}

fn poor_draft_short_chat_scenario() -> SimulationScenario<'static> {
    SimulationScenario {
        scenario_id: "poor_draft_short_chat",
        workload_class: "short_chat",
        draft_profile: "poor_mock_draft",
        target_tokens: vec!["safe", "answers", "stay", "reviewable", "and", "bounded"],
        draft_blocks: vec![
            vec!["unsafe", "answers"],
            vec!["stops", "here"],
            vec!["drift", "again"],
            vec!["wrong", "token"],
            vec!["and", "bounded"],
        ],
        cost_model: CostModel {
            baseline_target_ms_per_token: 12,
            draft_ms_per_token: 1,
            verification_base_ms: 5,
            verification_ms_per_proposed_token: 4,
        },
    }
}

fn tokenizer_mismatch_scenario() -> SpeculativeScenarioReport {
    SpeculativeScenarioReport {
        scenario_id: "cross_family_tokenizer_mismatch",
        workload_class: "cross_family_evaluation",
        draft_profile: "tokenizer_mismatch",
        status: SpeculativeScenarioStatus::NonProvingTokenizerMismatch,
        worthwhile: false,
        baseline_total_ms: 0,
        speculative_total_ms: 0,
        baseline_tokens_per_sec: 0.0,
        speculative_tokens_per_sec: 0.0,
        speedup_tps_ratio: 0.0,
        draft_tokens_proposed: 0,
        draft_tokens_accepted: 0,
        acceptance_rate: 0.0,
        baseline_target_forward_passes: 0,
        speculative_target_forward_passes: 0,
        replay_safe: false,
        authority_boundary:
            "Tokenizer mismatch fails before any accepted speculative token can claim target-authoritative commit.",
        evaluation_summary:
            "Cross-family or tokenizer-mismatch pairings are non-proving for WP-11. ADL should reject them before treating speculative acceleration as worth continuing."
                .to_string(),
        committed_output: Vec::new(),
        trace_steps: Vec::new(),
    }
}

fn simulate_speculative_scenario(
    scenario: &SimulationScenario<'_>,
) -> Result<SpeculativeScenarioReport> {
    if scenario.target_tokens.is_empty() {
        return Err(anyhow!("target token sequence must not be empty"));
    }
    if scenario.draft_blocks.is_empty() {
        return Err(anyhow!("draft block sequence must not be empty"));
    }

    let baseline_target_forward_passes = scenario.target_tokens.len();
    let baseline_total_ms =
        scenario.cost_model.baseline_target_ms_per_token * baseline_target_forward_passes as u64;
    let baseline_tokens_per_sec = round_ratio(
        scenario.target_tokens.len() as f64 * 1000.0,
        baseline_total_ms as f64,
    );

    let mut committed_output = Vec::new();
    let mut trace_steps = Vec::new();
    let mut draft_tokens_proposed = 0usize;
    let mut draft_tokens_accepted = 0usize;
    let mut speculative_target_forward_passes = 0usize;
    let mut speculative_total_ms = 0u64;
    let mut cursor = 0usize;

    for (step_index, block) in scenario.draft_blocks.iter().enumerate() {
        if cursor >= scenario.target_tokens.len() {
            break;
        }
        if block.is_empty() {
            return Err(anyhow!("draft block {} must not be empty", step_index + 1));
        }
        draft_tokens_proposed += block.len();
        speculative_total_ms += block.len() as u64 * scenario.cost_model.draft_ms_per_token;
        speculative_total_ms += scenario.cost_model.verification_base_ms
            + block.len() as u64 * scenario.cost_model.verification_ms_per_proposed_token;
        speculative_target_forward_passes += 1;

        let mut accepted_tokens = Vec::new();
        let mut rejected_token = None;
        let mut authoritative_token = None;
        let mut fallback_reason = None;

        for (candidate_index, candidate) in block.iter().enumerate() {
            let target_index = cursor + candidate_index;
            if target_index >= scenario.target_tokens.len() {
                break;
            }
            let expected = scenario.target_tokens[target_index];
            if *candidate == expected {
                accepted_tokens.push(expected.to_string());
                continue;
            }

            rejected_token = Some((*candidate).to_string());
            authoritative_token = Some(expected.to_string());
            fallback_reason = Some(
                "target verifier rejected speculative token and supplied authoritative fallback"
                    .to_string(),
            );
            break;
        }

        draft_tokens_accepted += accepted_tokens.len();
        committed_output.extend(accepted_tokens.iter().cloned());
        cursor += accepted_tokens.len();

        if let Some(authoritative) = authoritative_token.clone() {
            committed_output.push(authoritative);
            cursor += 1;
        }

        trace_steps.push(SpeculativeTraceStep {
            step_index: step_index + 1,
            proposed_tokens: block.iter().map(|token| (*token).to_string()).collect(),
            accepted_tokens,
            rejected_token,
            authoritative_token,
            target_tokens_verified: block.len(),
            fallback_reason,
        });
    }

    if cursor < scenario.target_tokens.len() {
        for token in scenario.target_tokens.iter().skip(cursor) {
            committed_output.push((*token).to_string());
        }
        speculative_total_ms += (scenario.target_tokens.len() - cursor) as u64
            * scenario.cost_model.baseline_target_ms_per_token;
        speculative_target_forward_passes += scenario.target_tokens.len() - cursor;
        trace_steps.push(SpeculativeTraceStep {
            step_index: trace_steps.len() + 1,
            proposed_tokens: Vec::new(),
            accepted_tokens: Vec::new(),
            rejected_token: None,
            authoritative_token: None,
            target_tokens_verified: 0,
            fallback_reason: Some(
                "draft budget exhausted; baseline target-only fallback finished the sequence"
                    .to_string(),
            ),
        });
    }

    let target_output: Vec<String> = scenario
        .target_tokens
        .iter()
        .map(|token| (*token).to_string())
        .collect();
    let replay_safe = committed_output == target_output;
    if !replay_safe {
        return Err(anyhow!(
            "speculative scenario '{}' failed deterministic committed-output equivalence",
            scenario.scenario_id
        ));
    }

    let speculative_tokens_per_sec = round_ratio(
        scenario.target_tokens.len() as f64 * 1000.0,
        speculative_total_ms as f64,
    );
    let speedup_tps_ratio = round_ratio(speculative_tokens_per_sec, baseline_tokens_per_sec);
    let acceptance_rate = round_ratio(draft_tokens_accepted as f64, draft_tokens_proposed as f64);
    let worthwhile = speedup_tps_ratio >= 1.25;
    let status = if worthwhile {
        SpeculativeScenarioStatus::ProvingWorthContinuing
    } else {
        SpeculativeScenarioStatus::ProvingNotWorthContinuing
    };
    let evaluation_summary = if worthwhile {
        format!(
            "{} shows a {:.2}x throughput improvement with acceptance rate {:.2}; worthwhile for bounded same-family ADL evaluation without changing commit authority.",
            scenario.scenario_id, speedup_tps_ratio, acceptance_rate
        )
    } else {
        format!(
            "{} shows only a {:.2}x throughput ratio with acceptance rate {:.2}; draft overhead overwhelms savings and this configuration is not worth continuing.",
            scenario.scenario_id, speedup_tps_ratio, acceptance_rate
        )
    };

    Ok(SpeculativeScenarioReport {
        scenario_id: scenario.scenario_id,
        workload_class: scenario.workload_class,
        draft_profile: scenario.draft_profile,
        status,
        worthwhile,
        baseline_total_ms,
        speculative_total_ms,
        baseline_tokens_per_sec,
        speculative_tokens_per_sec,
        speedup_tps_ratio,
        draft_tokens_proposed,
        draft_tokens_accepted,
        acceptance_rate,
        baseline_target_forward_passes,
        speculative_target_forward_passes,
        replay_safe,
        authority_boundary:
            "Speculative proposal accelerates only token suggestion. Authoritative commit remains target-verified, and Freedom Gate plus ACC remain the only side-effect boundary.",
        evaluation_summary,
        committed_output,
        trace_steps,
    })
}

fn round_ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        return 0.0;
    }
    ((numerator / denominator) * 100.0).round() / 100.0
}

pub fn run_speculative_decoding_prototype_report() -> SpeculativeDecodingPrototypeReport {
    let mut scenarios = vec![
        simulate_speculative_scenario(&same_family_code_generation_scenario())
            .expect("same-family code generation scenario"),
        simulate_speculative_scenario(&long_form_perfect_scenario())
            .expect("perfect long-form scenario"),
        simulate_speculative_scenario(&adl_card_generation_mixed_scenario())
            .expect("mixed ADL card scenario"),
        simulate_speculative_scenario(&poor_draft_short_chat_scenario())
            .expect("poor draft short chat scenario"),
        tokenizer_mismatch_scenario(),
    ];
    scenarios.sort_by_key(|scenario| scenario.scenario_id);

    let worthwhile_for_adl = scenarios.iter().any(|scenario| {
        scenario.status == SpeculativeScenarioStatus::ProvingWorthContinuing
            && scenario.workload_class != "cross_family_evaluation"
    });

    SpeculativeDecodingPrototypeReport {
        schema_version: SPECULATIVE_DECODING_PROTOTYPE_SCHEMA_VERSION,
        prompt_record: prompt_record(),
        worthwhile_for_adl,
        recommendation: SpeculativeWorthinessDecision::ContinueForSameFamilyLocalBackends,
        scenarios,
        commit_authority_rules: vec![
            "Speculative draft tokens are provisional until target verification accepts them.",
            "Rejected speculative tokens are discarded and replaced only by target-authoritative fallback tokens.",
            "Freedom Gate and ACC remain the authority boundary for any non-token side effects.",
        ],
        fallback_rules: vec![
            "Tokenizer mismatch must fail before speculative acceptance claims.",
            "Poor acceptance configurations remain valid negative evidence and should not be smuggled into positive worthiness claims.",
            "If speculative efficiency collapses, baseline target-only generation remains the truthful fallback.",
        ],
        validation_commands: vec![
            "cargo test --manifest-path adl/Cargo.toml speculative_decoding_prototype -- --nocapture".to_string(),
            "cargo test --manifest-path adl/Cargo.toml demo_v0912_speculative_decoding_prototype -- --nocapture".to_string(),
            "cargo run --manifest-path adl/Cargo.toml --bin demo_v0912_speculative_decoding_prototype".to_string(),
            "git diff --check".to_string(),
        ],
        non_claims: vec![
            "does not prove production backend speedups on live provider infrastructure",
            "does not grant tool or side-effect execution authority through speculative proposal",
            "does not claim cross-family tokenizer-mismatch pairings are worthwhile",
            "does not replace replay, audit, Freedom Gate, or ACC boundaries with throughput claims",
        ],
        claim_boundary:
            "WP-11 proves a bounded speculative draft/verify/commit prototype and a worthiness decision for ADL-style workloads. It shows same-family local acceleration can be worth continuing when acceptance is high, shows poor-draft and tokenizer-mismatch failure cases explicitly, and does not claim production serving gains or execution authority expansion."
                .to_string(),
    }
}

pub fn write_speculative_decoding_prototype_report(
    output_path: impl AsRef<Path>,
) -> Result<SpeculativeDecodingPrototypeReport> {
    let report = run_speculative_decoding_prototype_report();
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "create speculative decoding prototype report parent '{}'",
                parent.display()
            )
        })?;
    }
    let json = serde_json::to_string_pretty(&report)
        .context("serialize speculative decoding prototype report")?;
    fs::write(output_path, json).with_context(|| {
        format!(
            "write speculative decoding prototype report '{}'",
            output_path.display()
        )
    })?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::{
        run_speculative_decoding_prototype_report, same_family_code_generation_scenario,
        simulate_speculative_scenario, tokenizer_mismatch_scenario,
        write_speculative_decoding_prototype_report, SpeculativeScenarioStatus, HOST_PATH_MARKER,
        SPECULATIVE_DECODING_PROTOTYPE_REPORT_ARTIFACT_PATH,
        SPECULATIVE_DECODING_PROTOTYPE_SCHEMA_VERSION,
    };
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn speculative_decoding_prototype_perfect_same_family_scenario_is_replay_safe_and_faster() {
        let report = simulate_speculative_scenario(&same_family_code_generation_scenario())
            .expect("same-family scenario");
        assert_eq!(
            report.status,
            SpeculativeScenarioStatus::ProvingWorthContinuing
        );
        assert!(report.replay_safe);
        assert!(report.worthwhile);
        assert!(report.speedup_tps_ratio > 1.25);
        assert_eq!(
            report.committed_output,
            vec!["fn", "add", "(", "a", ",", "b", ")", "{", "a", "+", "b", "}"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn speculative_decoding_prototype_records_negative_worthiness_case() {
        let report = run_speculative_decoding_prototype_report();
        let poor = report
            .scenarios
            .iter()
            .find(|scenario| scenario.scenario_id == "poor_draft_short_chat")
            .expect("poor draft scenario");
        assert_eq!(
            poor.status,
            SpeculativeScenarioStatus::ProvingNotWorthContinuing
        );
        assert!(!poor.worthwhile);
        assert!(poor.speedup_tps_ratio < 1.25);
    }

    #[test]
    fn speculative_decoding_prototype_records_tokenizer_mismatch_as_non_proving() {
        let mismatch = tokenizer_mismatch_scenario();
        assert_eq!(
            mismatch.status,
            SpeculativeScenarioStatus::NonProvingTokenizerMismatch
        );
        assert!(!mismatch.replay_safe);
        assert!(!mismatch.worthwhile);
        assert!(mismatch.trace_steps.is_empty());
    }

    #[test]
    fn speculative_decoding_prototype_serializes_portably_without_host_paths() {
        let first = serde_json::to_string_pretty(&run_speculative_decoding_prototype_report())
            .expect("serialize report");
        let second = serde_json::to_string_pretty(&run_speculative_decoding_prototype_report())
            .expect("serialize report again");
        assert_eq!(first, second);
        assert!(!first.contains(HOST_PATH_MARKER));
    }

    #[test]
    fn speculative_decoding_prototype_report_writer_emits_expected_json() {
        let report_path = unique_temp_path("speculative-decoding-prototype");
        let report = write_speculative_decoding_prototype_report(&report_path)
            .expect("write speculative decoding report");
        let body = fs::read_to_string(&report_path).expect("read report");
        assert!(body.contains(SPECULATIVE_DECODING_PROTOTYPE_SCHEMA_VERSION));
        assert!(body.contains(report.prompt_record.prompt_version));
        fs::remove_file(&report_path).expect("remove report");
    }

    #[test]
    fn speculative_decoding_prototype_artifact_path_is_repo_relative_and_bounded() {
        assert!(!Path::new(SPECULATIVE_DECODING_PROTOTYPE_REPORT_ARTIFACT_PATH).is_absolute());
        assert!(!SPECULATIVE_DECODING_PROTOTYPE_REPORT_ARTIFACT_PATH.contains(".."));
    }

    #[test]
    fn speculative_decoding_prototype_tracked_report_matches_generated_report() {
        let tracked_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(SPECULATIVE_DECODING_PROTOTYPE_REPORT_ARTIFACT_PATH);
        let tracked = fs::read_to_string(&tracked_path).expect("tracked report should exist");
        let generated = serde_json::to_string_pretty(&run_speculative_decoding_prototype_report())
            .expect("serialize generated report");
        assert_eq!(tracked, generated);
    }
}
