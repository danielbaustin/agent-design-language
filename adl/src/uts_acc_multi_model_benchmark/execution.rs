use std::fs;
use std::io;
use std::path::Path;

use crate::local_gemma_model_evaluation::default_local_gemma_models;
use crate::model_identity::{
    stable_text_digest_v1, BenchmarkIdentityV1, EvaluatorIdentityV1, LaneIdentityV1, LaneKindV1,
};

use super::evaluation::{evaluate_task, scorecard_for};
use super::runtime::{
    append_progress_line, build_local_ollama_provider, current_ollama_host,
    local_model_identity, local_runtime_busy_reason, model_unavailable_reason,
    provider_complete_with_retries,
    provider_id_for_host, provider_transport_label, resolve_models, skipped_model_result,
};
use super::task_fixtures::{benchmark_tasks, prompt_record};
use super::types::{
    UtsAccBenchmarkConditions, UtsAccBenchmarkModelResult, UtsAccBenchmarkTaskFixture,
    UtsAccMultiModelBenchmarkEvidenceStatus, UtsAccMultiModelBenchmarkReport,
    UtsAccMultiModelRunStatus, UTS_ACC_MULTI_MODEL_BENCHMARK_SCHEMA_VERSION,
    UTS_ACC_MULTI_MODEL_BENCHMARK_RUNNER_VERSION,
};

pub(crate) fn model_result_for(
    model: &str,
    tasks: &[UtsAccBenchmarkTaskFixture],
) -> UtsAccBenchmarkModelResult {
    append_progress_line(&format!("model_start {model} task_count={}", tasks.len()));
    let host = current_ollama_host();
    let conditions = UtsAccBenchmarkConditions {
        provider_id: provider_id_for_host(&host).to_string(),
        model_id: model.to_string(),
        transport: provider_transport_label(&host).to_string(),
        live_model: true,
        notes: format!(
            "Bounded UTS v1.1 + ACC v1.1 model benchmark via {host}; no real tool execution occurs."
        ),
    };
    let provider = match build_local_ollama_provider(model) {
        Ok(provider) => provider,
        Err(error) => {
            append_progress_line(&format!(
                "model_skip {model} provider_build_error={}",
                error
            ));
            return UtsAccBenchmarkModelResult {
                candidate_id: format!("local.{model}"),
                run_status: UtsAccMultiModelRunStatus::Skipped,
                skip_reason: Some(format!("provider unavailable: {error:#}")),
                model_identity: local_model_identity(model),
                conditions,
                scorecard: None,
                cases: Vec::new(),
                failure_notes: vec![
                    "local evaluation could not start because the provider was unavailable"
                        .to_string(),
                ],
            };
        }
    };

    let mut cases = Vec::new();
    append_progress_line(&format!(
        "task_start {model} {}/{} {}",
        1,
        tasks.len(),
        tasks[0].record.id
    ));
    let first_case = evaluate_task(
        &tasks[0],
        provider_complete_with_retries(provider.as_ref(), &tasks[0].prompt),
    );
    append_progress_line(&format!(
        "task_done {model} {}/{} {} {:?} passed={}",
        1,
        tasks.len(),
        tasks[0].record.id,
        first_case.classification,
        first_case.passed
    ));
    cases.push(first_case);
    for (index, task) in tasks.iter().enumerate().skip(1) {
        append_progress_line(&format!(
            "task_start {model} {}/{} {}",
            index + 1,
            tasks.len(),
            task.record.id
        ));
        let case = evaluate_task(
            task,
            provider_complete_with_retries(provider.as_ref(), &task.prompt),
        );
        append_progress_line(&format!(
            "task_done {model} {}/{} {} {:?} passed={}",
            index + 1,
            tasks.len(),
            task.record.id,
            case.classification,
            case.passed
        ));
        cases.push(case);
    }

    let scorecard = scorecard_for(&cases);
    append_progress_line(&format!(
        "model_done {model} passed_count={} total_cases={} supports_governed_tool_use={}",
        scorecard.passed_count, scorecard.total_cases, scorecard.supports_governed_tool_use
    ));
    let failure_notes = cases
        .iter()
        .filter(|case| !case.passed)
        .map(|case| {
            format!(
                "{} -> {:?}: {}",
                case.task_id,
                case.classification,
                case.notes.join("; ")
            )
        })
        .collect::<Vec<_>>();

    UtsAccBenchmarkModelResult {
        candidate_id: format!("local.{model}"),
        run_status: UtsAccMultiModelRunStatus::Evaluated,
        skip_reason: None,
        model_identity: local_model_identity(model),
        conditions,
        scorecard: Some(scorecard),
        cases,
        failure_notes,
    }
}

fn evaluator_identity() -> EvaluatorIdentityV1 {
    EvaluatorIdentityV1 {
        evaluator_ref: "uts_acc_multi_model_evaluator".to_string(),
        evaluator_version: "1.0.0".to_string(),
        prompt_contract_version: super::UTS_ACC_MULTI_MODEL_BENCHMARK_PROMPT_VERSION.to_string(),
        classifier_version: "uts_acc_classification.v1".to_string(),
    }
}

fn lane_identities() -> Vec<LaneIdentityV1> {
    vec![LaneIdentityV1 {
        lane_ref: "uts_acc_governed.v1".to_string(),
        lane_kind: LaneKindV1::UtsAccGoverned,
        contract_version: "uts.v1.1+acc.v1.1".to_string(),
    }]
}

fn benchmark_identity(
    tasks: &[UtsAccBenchmarkTaskFixture],
    selected_models: &[String],
) -> BenchmarkIdentityV1 {
    let task_parts = tasks
        .iter()
        .flat_map(|task| {
            [
                task.record.id,
                task.record.scenario,
                task.record.expected_behavior,
            ]
        })
        .collect::<Vec<_>>();
    let model_parts = selected_models.iter().map(String::as_str).collect::<Vec<_>>();
    BenchmarkIdentityV1 {
        benchmark_ref: "uts_acc_multi_model_benchmark".to_string(),
        benchmark_version: UTS_ACC_MULTI_MODEL_BENCHMARK_SCHEMA_VERSION.to_string(),
        task_panel_digest: stable_text_digest_v1(&task_parts),
        model_panel_digest: stable_text_digest_v1(&model_parts),
        runner_version: UTS_ACC_MULTI_MODEL_BENCHMARK_RUNNER_VERSION.to_string(),
        contract_lock_digest: stable_text_digest_v1(&[
            super::UTS_ACC_MULTI_MODEL_BENCHMARK_PROMPT_VERSION,
            "uts.v1.1",
            "acc.v1.1",
        ]),
    }
}

pub fn run_uts_acc_multi_model_benchmark_with_models(
    explicit_models: &[String],
) -> UtsAccMultiModelBenchmarkReport {
    let tasks = benchmark_tasks();
    let (selection_source, selected_models) =
        resolve_models(explicit_models, default_local_gemma_models);
    let local_busy_reason = local_runtime_busy_reason(&selected_models);
    let model_results = selected_models
        .iter()
        .map(|model| match &local_busy_reason {
            Some(reason) => skipped_model_result(
                model,
                reason.clone(),
                "local evaluation could not start because the local Ollama runtime was busy",
            ),
            None => match model_unavailable_reason(model) {
                Some(reason) => skipped_model_result(
                    model,
                    reason,
                    "local evaluation could not start because the selected model was unavailable",
                ),
                None => model_result_for(model, &tasks),
            },
        })
        .collect::<Vec<_>>();
    let evaluated_count = model_results
        .iter()
        .filter(|model| model.run_status == UtsAccMultiModelRunStatus::Evaluated)
        .count();
    let usable_model_count = model_results
        .iter()
        .filter_map(|model| model.scorecard.as_ref())
        .filter(|scorecard| scorecard.supports_governed_tool_use)
        .count();
    let evidence_status = if selected_models.is_empty() {
        UtsAccMultiModelBenchmarkEvidenceStatus::BlockedNoLocalModels
    } else if usable_model_count == 0 {
        UtsAccMultiModelBenchmarkEvidenceStatus::NonProvingNoPassingModels
    } else {
        UtsAccMultiModelBenchmarkEvidenceStatus::ProvingAtLeastOneModelPassed
    };

    UtsAccMultiModelBenchmarkReport {
        schema_version: UTS_ACC_MULTI_MODEL_BENCHMARK_SCHEMA_VERSION,
        prompt_record: prompt_record(),
        benchmark_identity: benchmark_identity(&tasks, &selected_models),
        evaluator_identity: evaluator_identity(),
        lane_identities: lane_identities(),
        evidence_status,
        selection_source,
        selected_models,
        task_count: tasks.len(),
        candidate_count: model_results.len(),
        evaluated_count,
        usable_model_count,
        tasks: tasks.into_iter().map(|task| task.record).collect(),
        models: model_results,
        non_claims: vec![
            "This harness scores proposal discipline under UTS v1.1 + ACC v1.1; it does not grant execution authority.",
            "A valid usable proposal still requires governed runtime mediation before any real side effect.",
            "No hosted-provider ranking, latency benchmark, or quality ranking is claimed by this bounded local run.",
        ],
    }
}

pub fn render_summary(report: &UtsAccMultiModelBenchmarkReport) -> String {
    let mut lines = vec![
        "# UTS v1.1 + ACC v1.1 Local Small Model Benchmark Summary".to_string(),
        String::new(),
        format!(
            "- prompt version: `{}`",
            report.prompt_record.prompt_version
        ),
        format!("- evidence status: `{:?}`", report.evidence_status),
        format!("- selection source: `{:?}`", report.selection_source),
        format!(
            "- selected models: {}",
            if report.selected_models.is_empty() {
                "none".to_string()
            } else {
                report.selected_models.join(", ")
            }
        ),
        format!("- evaluated models: `{}`", report.evaluated_count),
        format!(
            "- models passing the full bounded governed-tool-use panel: `{}`",
            report.usable_model_count
        ),
        String::new(),
    ];

    if report.selected_models.is_empty() {
        lines.push(
            "This run is non-proving. No local models were discovered, so the benchmark could not demonstrate live governed tool proposals yet."
                .to_string(),
        );
        lines.push(String::new());
    }

    for model in &report.models {
        lines.push(format!("## {}", model.candidate_id));
        lines.push(String::new());
        lines.push(format!("- run status: `{:?}`", model.run_status));
        lines.push(format!(
            "- identity strength: `{:?}`",
            model.model_identity.identity_strength
        ));
        if let Some(digest) = &model.model_identity.resolved_digest {
            lines.push(format!("- resolved digest: `{digest}`"));
        }
        if let Some(reason) = &model.skip_reason {
            lines.push(format!("- skip reason: {}", reason));
        }
        if let Some(scorecard) = &model.scorecard {
            lines.push(format!(
                "- supports governed tool use: `{}`",
                scorecard.supports_governed_tool_use
            ));
            lines.push(format!(
                "- valid usable cases: `{}` / `{}`",
                scorecard.valid_usable_count, scorecard.total_cases
            ));
            lines.push(format!(
                "- refused risky cases: `{}`",
                scorecard.refused_count
            ));
            lines.push(format!(
                "- UTS-valid but ACC-invalid cases: `{}`",
                scorecard.uts_valid_but_acc_invalid_count
            ));
            lines.push(format!("- unusable cases: `{}`", scorecard.unusable_count));
        }
        for case in &model.cases {
            lines.push(format!(
                "- `{}` -> `{:?}`{}",
                case.task_id,
                case.classification,
                if case.notes.is_empty() {
                    String::new()
                } else {
                    format!(": {}", case.notes.join("; "))
                }
            ));
        }
        lines.push(String::new());
    }

    lines.join("\n") + "\n"
}

pub fn write_uts_acc_multi_model_benchmark_report_with_models(
    path: impl AsRef<Path>,
    models: &[String],
) -> io::Result<UtsAccMultiModelBenchmarkReport> {
    let report = run_uts_acc_multi_model_benchmark_with_models(models);
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_string_pretty(&report).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("serialize uts+acc multi-model benchmark report: {error}"),
        )
    })?;
    fs::write(path, body + "\n")?;
    Ok(report)
}

pub fn write_uts_acc_multi_model_benchmark_summary(
    path: impl AsRef<Path>,
    report: &UtsAccMultiModelBenchmarkReport,
) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, render_summary(report))
}

pub fn write_uts_acc_multi_model_benchmark_artifacts(
    report_path: impl AsRef<Path>,
    summary_path: impl AsRef<Path>,
    models: &[String],
) -> io::Result<UtsAccMultiModelBenchmarkReport> {
    let report = write_uts_acc_multi_model_benchmark_report_with_models(report_path, models)?;
    write_uts_acc_multi_model_benchmark_summary(summary_path, &report)?;
    Ok(report)
}
