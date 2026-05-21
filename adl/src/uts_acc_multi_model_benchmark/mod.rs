mod evaluation;
mod execution;
mod parsing;
mod runtime;
mod task_fixtures;
#[cfg(test)]
mod tests;
mod types;

pub use execution::{
    render_summary, run_uts_acc_multi_model_benchmark_with_models,
    write_uts_acc_multi_model_benchmark_artifacts,
    write_uts_acc_multi_model_benchmark_report_with_models,
    write_uts_acc_multi_model_benchmark_summary,
};
pub use types::{
    UtsAccBenchmarkClassification, UtsAccBenchmarkConditions, UtsAccBenchmarkModelResult,
    UtsAccBenchmarkScorecard, UtsAccBenchmarkTaskRecord, UtsAccBenchmarkTaskResult,
    UtsAccMultiModelBenchmarkEvidenceStatus, UtsAccMultiModelBenchmarkPromptRecord,
    UtsAccMultiModelBenchmarkReport, UtsAccMultiModelRunStatus, UtsAccMultiModelSelectionSource,
    UTS_ACC_MULTI_MODEL_BENCHMARK_PROMPT_VERSION,
    UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH,
    UTS_ACC_MULTI_MODEL_BENCHMARK_SUMMARY_ARTIFACT_PATH,
};

#[cfg(test)]
pub(crate) use evaluation::{
    appears_refusal, authority_humility, claims_execution, classify_compiler_rejection,
    evaluate_task, scorecard_for,
};
#[cfg(test)]
pub(crate) use parsing::{
    bounded_response_excerpt, first_json_object_body, parse_model_turn_response,
};
#[cfg(test)]
pub(crate) use runtime::{
    model_unavailable_reason, parse_explicit_models, parse_ollama_list_output,
    parse_ollama_ps_output, progress_path, provider_complete_with_retries, provider_id_for_host,
    provider_transport_label, uses_remote_ollama_host,
};
#[cfg(test)]
pub(crate) use task_fixtures::{benchmark_tasks, response_contract, tool_contracts};
#[cfg(test)]
pub(crate) use types::UTS_ACC_MULTI_MODEL_BENCHMARK_SCHEMA_VERSION;

pub fn run_uts_acc_multi_model_benchmark() -> UtsAccMultiModelBenchmarkReport {
    run_uts_acc_multi_model_benchmark_with_models(&[])
}

pub fn write_uts_acc_multi_model_benchmark_report(
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<UtsAccMultiModelBenchmarkReport> {
    write_uts_acc_multi_model_benchmark_report_with_models(path, &[])
}
