use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const DSPARK_SPECULATIVE_DECODING_EVALUATION_SCHEMA_VERSION: &str =
    "dspark_speculative_decoding_evaluation.v1";
pub const DSPARK_SPECULATIVE_DECODING_EVALUATION_PROMPT_VERSION: &str =
    "v0917.provider_sprint.dspark_speculative_decoding.v1";
pub const DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.7/review/provider/DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.json";

#[cfg(test)]
const HOST_PATH_MARKER: &str = "/absolute/host/path/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsparkSourceRecord {
    pub source_id: &'static str,
    pub title: &'static str,
    pub source_ref: &'static str,
    pub observed_date: &'static str,
    pub relevance: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DsparkCandidateDisposition {
    CandidateForBackendProbe,
    BlockedUntilBackendExists,
    RejectCrossFamilyPairing,
    RouteToLiveGpuSmoke,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsparkCandidateRow {
    pub row_id: &'static str,
    pub target_family: &'static str,
    pub draft_family: &'static str,
    pub proposed_models: Vec<&'static str>,
    pub disposition: DsparkCandidateDisposition,
    pub acceptance_condition: &'static str,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeBenchmarkSummary {
    pub mode: &'static str,
    pub runs: u32,
    pub median_elapsed_seconds: f64,
    pub mean_tokens_per_second: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeQwenBenchmarkRecord {
    pub proof_artifact: &'static str,
    pub host_class: &'static str,
    pub runtime: &'static str,
    pub target_model: &'static str,
    pub assistant_model: &'static str,
    pub benchmark_status: &'static str,
    pub summaries: Vec<NativeBenchmarkSummary>,
    pub proves_dspark_backend: bool,
    pub accepted_draft_token_counts_exposed: bool,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossFamilyProbeRecord {
    pub proof_artifact: &'static str,
    pub host_class: &'static str,
    pub runtime: &'static str,
    pub target_model: &'static str,
    pub assistant_model: &'static str,
    pub probe_status: &'static str,
    pub observed_error_class: &'static str,
    pub proves_assisted_generation: bool,
    pub proves_dspark_backend: bool,
    pub accepted_draft_token_counts_exposed: bool,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeVllmAvailabilityProbe {
    pub proof_artifact: &'static str,
    pub host_class: &'static str,
    pub runtime: &'static str,
    pub probe_status: &'static str,
    pub observed_error: &'static str,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VllmBenchmarkSummary {
    pub mode: &'static str,
    pub runs: u32,
    pub median_elapsed_seconds: f64,
    pub mean_tokens_per_second: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VllmSpeculativeCounters {
    pub num_drafts: u32,
    pub num_draft_tokens: u32,
    pub num_accepted_tokens: u32,
    pub num_accepted_tokens_per_pos: Vec<u32>,
    pub draft_acceptance_rate: f64,
    pub mean_acceptance_length_including_bonus: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VllmQwenSpeculativeBenchmarkRecord {
    pub target_only_artifact: &'static str,
    pub speculative_artifact: &'static str,
    pub host_class: &'static str,
    pub runtime: &'static str,
    pub target_model: &'static str,
    pub draft_model: &'static str,
    pub spec_tokens: u32,
    pub benchmark_status: &'static str,
    pub summaries: Vec<VllmBenchmarkSummary>,
    pub measured_speculative_counters: VllmSpeculativeCounters,
    pub proves_vllm_speculative_mode: bool,
    pub proves_dspark_backend: bool,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VllmGemmaSpeculativeProbeRecord {
    pub proof_artifact: &'static str,
    pub host_class: &'static str,
    pub runtime: &'static str,
    pub target_model: &'static str,
    pub draft_model: &'static str,
    pub spec_tokens: u32,
    pub probe_status: &'static str,
    pub observed_error_class: &'static str,
    pub proves_vllm_speculative_mode: bool,
    pub proves_dspark_backend: bool,
    pub accepted_draft_token_counts_exposed: bool,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DsparkEvaluationReport {
    pub schema_version: &'static str,
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub sprint_issue: u32,
    pub source_records: Vec<DsparkSourceRecord>,
    pub candidate_rows: Vec<DsparkCandidateRow>,
    pub native_qwen_benchmarks: Vec<NativeQwenBenchmarkRecord>,
    pub gemma_qwen_cross_family_probes: Vec<CrossFamilyProbeRecord>,
    pub native_vllm_availability_probes: Vec<NativeVllmAvailabilityProbe>,
    pub vllm_qwen_speculative_benchmarks: Vec<VllmQwenSpeculativeBenchmarkRecord>,
    pub vllm_gemma_speculative_probes: Vec<VllmGemmaSpeculativeProbeRecord>,
    pub accepted_for_v0917_provider_sprint: bool,
    pub recommendation: &'static str,
    pub required_next_proof: Vec<&'static str>,
    pub authority_rules: Vec<&'static str>,
    pub non_claims: Vec<&'static str>,
    pub validation_commands: Vec<&'static str>,
}

fn source_records() -> Vec<DsparkSourceRecord> {
    vec![
        DsparkSourceRecord {
            source_id: "arxiv_2607_05147",
            title: "DSpark: Confidence-Scheduled Speculative Decoding with Semi-Autoregressive Generation",
            source_ref: "https://arxiv.org/abs/2607.05147",
            observed_date: "2026-07-07",
            relevance: "Defines DSpark's semi-autoregressive draft and confidence-scheduled verification design; supports evaluating ADL only as a backend capability candidate, not as a prompt-level provider feature.",
        },
        DsparkSourceRecord {
            source_id: "adl_v0912_speculative_decoding_prototype",
            title: "ADL speculative decoding deterministic commit-boundary prototype",
            source_ref: "docs/milestones/v0.91.2/review/speculative_decoding/speculative_decoding_prototype_packet.md",
            observed_date: "2026-07-07",
            relevance: "Existing ADL proof requires target-verified token commit, explicit tokenizer mismatch rejection, and no expansion of tool or side-effect authority.",
        },
    ]
}

fn candidate_rows() -> Vec<DsparkCandidateRow> {
    vec![
        DsparkCandidateRow {
            row_id: "qwen_same_family_candidate",
            target_family: "qwen",
            draft_family: "qwen",
            proposed_models: vec!["qwen/qwen3-coder-next", "qwen/qwen3-6-flash"],
            disposition: DsparkCandidateDisposition::BlockedUntilBackendExists,
            acceptance_condition: "A serving backend must expose DSpark-style draft generation, target verification, accepted-token counts, fallback counts, and tokenizer compatibility for the same Qwen family.",
            reason: "Qwen is plausible as a same-family speculative-decoding candidate, but ADL currently has no live DSpark/Qwen draft-verify backend to prove accepted length or throughput.",
        },
        DsparkCandidateRow {
            row_id: "gemma_same_family_candidate",
            target_family: "gemma",
            draft_family: "gemma",
            proposed_models: vec!["google/gemma-4-31b-it", "gemma4:e4b"],
            disposition: DsparkCandidateDisposition::BlockedUntilBackendExists,
            acceptance_condition: "A serving backend must expose DSpark-style draft generation, target verification, accepted-token counts, fallback counts, and tokenizer compatibility for the same Gemma family.",
            reason: "Gemma is plausible as a same-family local or hosted candidate, but existing ADL Gemma evidence covers model usefulness, not DSpark-style speculative acceptance or throughput.",
        },
        DsparkCandidateRow {
            row_id: "qwen_gemma_cross_family_rejected",
            target_family: "qwen_or_gemma",
            draft_family: "gemma_or_qwen",
            proposed_models: vec!["qwen target with gemma draft", "gemma target with qwen draft"],
            disposition: DsparkCandidateDisposition::RejectCrossFamilyPairing,
            acceptance_condition: "None for v0.91.7; cross-family pairings must not be treated as accepted speculative acceleration evidence.",
            reason: "The ADL speculative-decoding prototype treats tokenizer mismatch as non-proving. Cross-family Qwen/Gemma pairings would widen that risk unless a backend proves tokenizer identity and target-verified commit behavior.",
        },
        DsparkCandidateRow {
            row_id: "deepseek_v4_flash_dspark_live_lane",
            target_family: "deepseek-v4",
            draft_family: "dspark",
            proposed_models: vec!["deepseek-v4-flash-dspark"],
            disposition: DsparkCandidateDisposition::RouteToLiveGpuSmoke,
            acceptance_condition: "Issue #4654 must run the bounded ephemeral 2xH100 AWS smoke, record teardown, and retain provider/model outcome evidence before this row can be accepted.",
            reason: "The external DSpark result is specifically tied to the DeepSeek-V4 serving system; ADL should prove that path in #4654 rather than infer it from Qwen/Gemma candidates.",
        },
    ]
}

fn native_qwen_benchmarks() -> Vec<NativeQwenBenchmarkRecord> {
    vec![NativeQwenBenchmarkRecord {
        proof_artifact: "docs/milestones/v0.91.7/review/provider/NATIVE_QWEN_ASSISTED_GENERATION_BENCHMARK_OFFLINE_4653.json",
        host_class: "nessus_windows_rtx_3090",
        runtime: "native_windows_python_transformers_offline_hf_cache",
        target_model: "Qwen/Qwen2.5-1.5B-Instruct",
        assistant_model: "Qwen/Qwen2.5-0.5B-Instruct",
        benchmark_status: "completed_no_speedup",
        summaries: vec![
            NativeBenchmarkSummary {
                mode: "target_only",
                runs: 15,
                median_elapsed_seconds: 6.020484099979512,
                mean_tokens_per_second: 26.292143901045392,
            },
            NativeBenchmarkSummary {
                mode: "assisted",
                runs: 15,
                median_elapsed_seconds: 7.976211500004865,
                mean_tokens_per_second: 20.018113611735426,
            },
        ],
        proves_dspark_backend: false,
        accepted_draft_token_counts_exposed: false,
        note: "Live native Windows Transformers assisted generation ran on Nessus RTX 3090 with HF_HUB_OFFLINE=1 and TRANSFORMERS_OFFLINE=1, using the existing Hugging Face cache. Assisted generation was slower than target-only for this Qwen2.5 1.5B/0.5B pair and did not expose backend accepted-token or fallback counters, so it is proof of invocation only, not DSpark acceleration.",
    }]
}

fn gemma_qwen_cross_family_probes() -> Vec<CrossFamilyProbeRecord> {
    vec![CrossFamilyProbeRecord {
        proof_artifact:
            "docs/milestones/v0.91.7/review/provider/GEMMA_QWEN_TOKENIZER_PROBE_4653.json",
        host_class: "nessus_windows_rtx_3090",
        runtime: "native_windows_python_transformers",
        target_model: "google/gemma-2-2b-it",
        assistant_model: "Qwen/Qwen2.5-0.5B-Instruct",
        probe_status: "blocked_gated_gemma_repo",
        observed_error_class: "huggingface_hub.errors.GatedRepoError",
        proves_assisted_generation: false,
        proves_dspark_backend: false,
        accepted_draft_token_counts_exposed: false,
        note: "The required Gemma target plus Qwen assistant probe could not load the Gemma Hugging Face tokenizer because google/gemma-2-2b-it is gated. Nessus has local Ollama Gemma models, but that path does not expose assisted-generation invocation or accepted-token/fallback counters, so the Gemma+Qwen cross-family row remains rejected rather than benchmark-accepted.",
    }]
}

fn native_vllm_availability_probes() -> Vec<NativeVllmAvailabilityProbe> {
    vec![NativeVllmAvailabilityProbe {
        proof_artifact:
            "docs/milestones/v0.91.7/review/provider/VLLM_NATIVE_WINDOWS_AVAILABILITY_PROBE_4653.json",
        host_class: "nessus_windows_rtx_3090",
        runtime: "native_windows_python_venv",
        probe_status: "not_available_in_native_windows_venv",
        observed_error: "ERROR: No matching distribution found for vllm",
        note: "The existing Nessus native Windows venv is the reusable benchmark environment for Torch/Transformers and the cached Qwen weights, but pip dry-run resolution for vllm found no installable native Windows wheel. Docker vLLM can initialize Qwen on this host, but that path is WSL/container-detected and is not accepted as native Windows proof for this issue.",
    }]
}

fn vllm_qwen_speculative_benchmarks() -> Vec<VllmQwenSpeculativeBenchmarkRecord> {
    vec![VllmQwenSpeculativeBenchmarkRecord {
        target_only_artifact:
            "docs/milestones/v0.91.7/review/provider/VLLM_QWEN_TARGET_ONLY_BENCHMARK_4653.json",
        speculative_artifact:
            "docs/milestones/v0.91.7/review/provider/VLLM_QWEN_SPECULATIVE_BENCHMARK_4653.json",
        host_class: "nessus_windows_rtx_3090",
        runtime: "vllm_container_offline_hf_cache",
        target_model: "Qwen/Qwen2.5-1.5B-Instruct",
        draft_model: "Qwen/Qwen2.5-0.5B-Instruct",
        spec_tokens: 3,
        benchmark_status: "completed_with_counters_no_speedup",
        summaries: vec![
            VllmBenchmarkSummary {
                mode: "target_only",
                runs: 6,
                median_elapsed_seconds: 0.48294413098483346,
                mean_tokens_per_second: 199.18956234898937,
            },
            VllmBenchmarkSummary {
                mode: "speculative",
                runs: 6,
                median_elapsed_seconds: 1.3170339860080276,
                mean_tokens_per_second: 72.73522446263055,
            },
        ],
        measured_speculative_counters: VllmSpeculativeCounters {
            num_drafts: 204,
            num_draft_tokens: 612,
            num_accepted_tokens: 378,
            num_accepted_tokens_per_pos: vec![156, 120, 102],
            draft_acceptance_rate: 0.6176470588235294,
            mean_acceptance_length_including_bonus: 2.8529411764705883,
        },
        proves_vllm_speculative_mode: true,
        proves_dspark_backend: false,
        note: "vLLM 0.24.0 draft-model speculative decoding ran on Nessus RTX 3090 with offline Hugging Face cache reuse. The target/draft pair exposed real draft and accepted-token counters, but the speculative path was slower than target-only for this measured Qwen2.5 1.5B/0.5B configuration. This is vLLM draft-model speculative evidence, not DSpark confidence-scheduled backend proof.",
    }]
}

fn vllm_gemma_speculative_probes() -> Vec<VllmGemmaSpeculativeProbeRecord> {
    vec![VllmGemmaSpeculativeProbeRecord {
        proof_artifact:
            "docs/milestones/v0.91.7/review/provider/VLLM_GEMMA_SPECULATIVE_SMOKE_4653.json",
        host_class: "nessus_windows_rtx_3090",
        runtime: "vllm_container_hf_token_download_cache",
        target_model: "google/gemma-3-1b-it",
        draft_model: "google/gemma-3-270m-it",
        spec_tokens: 3,
        probe_status: "failed_engine_initialization",
        observed_error_class: "AssertionError: All drafting layers should belong to the same kv cache group",
        proves_vllm_speculative_mode: false,
        proves_dspark_backend: false,
        accepted_draft_token_counts_exposed: false,
        note: "The live Nessus Docker/vLLM Gemma same-family smoke authenticated with the local Hugging Face token, resolved both Gemma3ForCausalLM models, downloaded and loaded target/draft weights, and enabled draft_model speculative configuration. vLLM 0.24.0 then failed during KV-cache initialization before generation with an assertion that all drafting layers should belong to the same KV cache group. This is a useful negative backend-compatibility result, not a Gemma speculative decoding acceptance or acceleration proof.",
    }]
}

pub fn run_dspark_speculative_decoding_evaluation() -> DsparkEvaluationReport {
    DsparkEvaluationReport {
        schema_version: DSPARK_SPECULATIVE_DECODING_EVALUATION_SCHEMA_VERSION,
        prompt_version: DSPARK_SPECULATIVE_DECODING_EVALUATION_PROMPT_VERSION,
        issue_number: 4653,
        sprint_issue: 5027,
        source_records: source_records(),
        candidate_rows: candidate_rows(),
        native_qwen_benchmarks: native_qwen_benchmarks(),
        gemma_qwen_cross_family_probes: gemma_qwen_cross_family_probes(),
        native_vllm_availability_probes: native_vllm_availability_probes(),
        vllm_qwen_speculative_benchmarks: vllm_qwen_speculative_benchmarks(),
        vllm_gemma_speculative_probes: vllm_gemma_speculative_probes(),
        accepted_for_v0917_provider_sprint: false,
        recommendation: "Do not claim Qwen or Gemma DSpark acceleration as accepted in v0.91.7 from planning evidence or the current live vLLM probes. Keep Qwen/Gemma as same-family candidates with measured negative/blocked results, reject cross-family Qwen/Gemma pairings, and route actual DeepSeek-V4 DSpark live proof to #4654.",
        required_next_proof: vec![
            "A same-family Qwen or Gemma backend must expose draft tokens, target verification, accepted-token counts, fallback counts, tokenizer compatibility, latency, and throughput before ADL can accept the row.",
            "A same-family Qwen acceleration claim needs backend accepted-token/fallback counters or a serving stack that exposes equivalent draft acceptance telemetry; native Transformers assisted generation alone is insufficient.",
            "The vLLM Qwen draft-model benchmark proves draft and accepted-token counters for Qwen2.5 1.5B/0.5B, but its measured speculative path is slower than target-only and is not DSpark confidence scheduling.",
            "The vLLM Gemma draft-model smoke reached authenticated model load for google/gemma-3-1b-it plus google/gemma-3-270m-it, but failed engine initialization before generation on vLLM 0.24.0 with a KV-cache grouping assertion.",
            "A Gemma target with Qwen assistant path must prove tokenizer compatibility and target-verified commit behavior in an accessible backend before the cross-family row can move out of rejected status.",
            "A native Windows vLLM claim needs a supported vLLM install/import path or a different accepted serving lane; the existing native venv does not provide vLLM.",
            "Issue #4654 must prove or truthfully block the deepseek-v4-flash-dspark live GPU smoke with Agent Logic AWS account guard and teardown evidence.",
            "The shared provider proof #5026 must consume only rows that have live or accepted blocked dispositions.",
        ],
        authority_rules: vec![
            "Speculative draft tokens remain provisional until target verification accepts them.",
            "Accepted token counts and throughput claims must come from the backend, not prompt-level model text.",
            "Speculative decoding cannot grant tool, mutation, merge, or side-effect authority.",
            "Cross-family tokenizer mismatch is a fail-closed condition unless the backend proves compatibility.",
        ],
        non_claims: vec![
            "does not prove live Qwen DSpark acceleration",
            "does not prove Qwen assisted generation speedup for the measured Qwen2.5 1.5B/0.5B pair",
            "does not prove vLLM Qwen speculative speedup for the measured Qwen2.5 1.5B/0.5B pair",
            "does not prove live Gemma DSpark acceleration",
            "does not prove vLLM Gemma same-family speculative decoding execution",
            "does not prove Gemma target plus Qwen assistant speculative decoding",
            "does not prove native Windows vLLM execution",
            "does not prove DeepSeek-V4 DSpark availability on AWS",
            "does not claim broad speculative decoding support in ADL provider routing",
            "does not replace #4654 live GPU smoke or #5026 shared provider acceptance proof",
        ],
        validation_commands: vec![
            "python3 -m py_compile adl/tools/native_qwen_assisted_generation_benchmark.py",
            "HF_HUB_OFFLINE=1 TRANSFORMERS_OFFLINE=1 C:\\adl-local-artifacts\\issue-4653\\native-venv\\Scripts\\python.exe C:\\adl-local-artifacts\\issue-4653\\native_qwen_assisted_generation_benchmark.py --out C:\\adl-local-artifacts\\issue-4653\\native-qwen-assisted-offline-20260709.json --repeats 3 --prompt-limit 5 --warmup-runs 1 --max-new-tokens 160",
            "C:\\adl-local-artifacts\\issue-4653\\native-venv\\Scripts\\python.exe -m pip install --dry-run --only-binary=:all: vllm",
            "docker run --rm --gpus all -e HF_HUB_OFFLINE=1 -e TRANSFORMERS_OFFLINE=1 -v C:\\Users\\danie\\.cache\\huggingface:/root/.cache/huggingface -v C:\\adl-local-artifacts\\issue-4653\\vllm-cache:/root/.cache/vllm -v C:\\adl-local-artifacts\\issue-4653:/work --entrypoint python3 vllm/vllm-openai:latest /work/vllm_qwen_speculative_decoding_benchmark.py --mode target_only --out /work/vllm-qwen-target-full-20260709.json --max-new-tokens 96 --warmup-runs 1 --repeats 2 --prompt-limit 3 --spec-tokens 3 --gpu-memory-utilization 0.50",
            "docker run --rm --gpus all -e HF_HUB_OFFLINE=1 -e TRANSFORMERS_OFFLINE=1 -v C:\\Users\\danie\\.cache\\huggingface:/root/.cache/huggingface -v C:\\adl-local-artifacts\\issue-4653\\vllm-cache:/root/.cache/vllm -v C:\\adl-local-artifacts\\issue-4653:/work --entrypoint python3 vllm/vllm-openai:latest /work/vllm_qwen_speculative_decoding_benchmark.py --mode speculative --out /work/vllm-qwen-speculative-full-20260709.json --max-new-tokens 96 --warmup-runs 1 --repeats 2 --prompt-limit 3 --spec-tokens 3 --gpu-memory-utilization 0.50",
            "docker run --rm --gpus all -e HF_TOKEN=<redacted> -v C:\\Users\\danie\\.cache\\huggingface:/root/.cache/huggingface -v C:\\adl-local-artifacts\\issue-4653\\vllm-cache:/root/.cache/vllm -v C:\\adl-local-artifacts\\issue-4653:/work --entrypoint python3 vllm/vllm-openai:latest /work/vllm_qwen_speculative_decoding_benchmark.py --mode speculative --target-model google/gemma-3-1b-it --draft-model google/gemma-3-270m-it --model-family gemma --out /work/vllm-gemma-spec-smoke-20260709.json --max-new-tokens 32 --warmup-runs 1 --repeats 1 --prompt-limit 1 --spec-tokens 3 --gpu-memory-utilization 0.50",
            "CARGO_INCREMENTAL=0 cargo test --manifest-path adl/Cargo.toml --lib dspark_speculative_decoding_evaluation -- --nocapture",
            "git diff --check",
        ],
    }
}

pub fn write_dspark_speculative_decoding_evaluation_report(
    output_path: impl AsRef<Path>,
) -> Result<DsparkEvaluationReport> {
    let report = run_dspark_speculative_decoding_evaluation();
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "create DSpark speculative decoding evaluation parent '{}'",
                parent.display()
            )
        })?;
    }
    let json = serde_json::to_string_pretty(&report)
        .context("serialize DSpark speculative decoding evaluation report")?;
    fs::write(output_path, json).with_context(|| {
        format!(
            "write DSpark speculative decoding evaluation report '{}'",
            output_path.display()
        )
    })?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::{
        run_dspark_speculative_decoding_evaluation,
        write_dspark_speculative_decoding_evaluation_report, DsparkCandidateDisposition,
        DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH,
        DSPARK_SPECULATIVE_DECODING_EVALUATION_SCHEMA_VERSION, HOST_PATH_MARKER,
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
    fn dspark_speculative_decoding_evaluation_routes_rows_truthfully() {
        let report = run_dspark_speculative_decoding_evaluation();
        assert!(!report.accepted_for_v0917_provider_sprint);
        let qwen = report
            .candidate_rows
            .iter()
            .find(|row| row.row_id == "qwen_same_family_candidate")
            .expect("qwen row");
        assert_eq!(
            qwen.disposition,
            DsparkCandidateDisposition::BlockedUntilBackendExists
        );
        let cross = report
            .candidate_rows
            .iter()
            .find(|row| row.row_id == "qwen_gemma_cross_family_rejected")
            .expect("cross-family row");
        assert_eq!(
            cross.disposition,
            DsparkCandidateDisposition::RejectCrossFamilyPairing
        );
        let deepseek = report
            .candidate_rows
            .iter()
            .find(|row| row.row_id == "deepseek_v4_flash_dspark_live_lane")
            .expect("deepseek row");
        assert_eq!(
            deepseek.disposition,
            DsparkCandidateDisposition::RouteToLiveGpuSmoke
        );
    }

    #[test]
    fn dspark_speculative_decoding_evaluation_serializes_portably() {
        let first = serde_json::to_string_pretty(&run_dspark_speculative_decoding_evaluation())
            .expect("serialize first report");
        let second = serde_json::to_string_pretty(&run_dspark_speculative_decoding_evaluation())
            .expect("serialize second report");
        assert_eq!(first, second);
        assert!(!first.contains(HOST_PATH_MARKER));
    }

    #[test]
    fn dspark_speculative_decoding_evaluation_writer_emits_expected_json() {
        let path = unique_temp_path("dspark-speculative-decoding-evaluation");
        let report =
            write_dspark_speculative_decoding_evaluation_report(&path).expect("write report");
        let body = fs::read_to_string(&path).expect("read report");
        assert!(body.contains(DSPARK_SPECULATIVE_DECODING_EVALUATION_SCHEMA_VERSION));
        assert_eq!(report.candidate_rows.len(), 4);
        assert_eq!(report.vllm_gemma_speculative_probes.len(), 1);
        fs::remove_file(&path).expect("remove report");
    }

    #[test]
    fn dspark_speculative_decoding_evaluation_artifact_path_is_repo_relative() {
        assert!(
            !Path::new(DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH).is_absolute()
        );
        assert!(!DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH.contains(".."));
    }
}
