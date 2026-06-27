use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::Deserialize;
use serde_yaml::{Mapping, Value};

use super::markdown::{markdown_block_field, split_front_matter};
use super::structured_prompt::{validate_sor_text, validate_srp_text};

#[derive(Debug, Default, Deserialize)]
struct FactPacket {
    #[serde(default)]
    review: ReviewFacts,
    #[serde(default)]
    validation: ValidationFacts,
    #[serde(default)]
    integration: IntegrationFacts,
    #[serde(default)]
    release_tail: ReleaseTailFacts,
    #[serde(default)]
    metrics: MetricsFacts,
}

#[derive(Debug, Default, Deserialize)]
struct ReviewFacts {
    findings_status: Option<String>,
    recommended_outcome: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ValidationFacts {
    status: Option<String>,
    #[serde(default)]
    commands: Vec<ValidationCommand>,
}

#[derive(Debug, Deserialize)]
struct ValidationCommand {
    command: String,
    purpose: String,
    result: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct IntegrationFacts {
    #[serde(default)]
    main_repo_paths_updated: Vec<String>,
    worktree_only_paths_remaining: Option<String>,
    integration_state: Option<String>,
    verification_scope: Option<String>,
    integration_method: Option<String>,
    #[serde(default)]
    verification_performed: Vec<String>,
    result: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ReleaseTailFacts {
    watcher_disposition: Option<String>,
    pr_state: Option<String>,
    closeout_state: Option<String>,
    #[serde(default)]
    residual_risks: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
struct MetricsFacts {
    actual_elapsed_seconds: Option<String>,
    actual_total_tokens: Option<String>,
    actual_validation_seconds: Option<String>,
    goal_metrics_data_source: Option<String>,
    goal_metrics_source_ref: Option<String>,
    data_source_confidence: Option<String>,
}

#[derive(Debug, Default)]
struct Args {
    facts: Option<PathBuf>,
    srp: Option<PathBuf>,
    sor: Option<PathBuf>,
    out_srp: Option<PathBuf>,
    out_sor: Option<PathBuf>,
}

pub(super) fn real_srp_sor_update(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|arg| arg.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", srp_sor_update_usage());
        return Ok(());
    }

    let args = parse_args(args)?;
    let facts_path = args
        .facts
        .as_deref()
        .ok_or_else(|| anyhow!("missing required --facts <facts.yaml>"))?;
    let srp_path = args
        .srp
        .as_deref()
        .ok_or_else(|| anyhow!("missing required --srp <srp.md>"))?;
    let sor_path = args
        .sor
        .as_deref()
        .ok_or_else(|| anyhow!("missing required --sor <sor.md>"))?;
    let out_srp = args.out_srp.as_deref().unwrap_or(srp_path);
    let out_sor = args.out_sor.as_deref().unwrap_or(sor_path);

    let facts_text = fs::read_to_string(facts_path)
        .with_context(|| format!("read facts file {}", facts_path.display()))?;
    let facts: FactPacket = serde_yaml::from_str(&facts_text)
        .with_context(|| format!("parse facts file {}", facts_path.display()))?;
    ensure!(
        facts.has_actionable_facts(),
        "facts file contains no SRP/SOR updates"
    );
    facts.validate_consistency()?;

    let srp_text =
        fs::read_to_string(srp_path).with_context(|| format!("read SRP {}", srp_path.display()))?;
    let sor_text =
        fs::read_to_string(sor_path).with_context(|| format!("read SOR {}", sor_path.display()))?;

    let updated_srp = update_srp(&srp_text, &facts).context("update SRP")?;
    let updated_sor = update_sor(&sor_text, &facts).context("update SOR")?;
    assert_requested_facts_landed(&updated_srp, &updated_sor, &facts)?;

    validate_srp_text(&updated_srp).context("updated SRP failed structure validation")?;
    validate_sor_text(&updated_sor, None).context("updated SOR failed structure validation")?;

    let srp_changed = updated_srp != srp_text || out_srp != srp_path;
    let sor_changed = updated_sor != sor_text || out_sor != sor_path;
    if srp_changed {
        write_text(out_srp, &updated_srp)?;
    }
    if sor_changed {
        write_text(out_sor, &updated_sor)?;
    }

    println!("PASS: updated SRP/SOR facts (srp_changed={srp_changed}, sor_changed={sor_changed})");
    Ok(())
}

fn parse_args(raw: &[String]) -> Result<Args> {
    let mut args = Args::default();
    let mut idx = 0;
    while idx < raw.len() {
        match raw[idx].as_str() {
            "--facts" => args.facts = Some(next_path(raw, &mut idx, "--facts")?),
            "--srp" => args.srp = Some(next_path(raw, &mut idx, "--srp")?),
            "--sor" => args.sor = Some(next_path(raw, &mut idx, "--sor")?),
            "--out-srp" => args.out_srp = Some(next_path(raw, &mut idx, "--out-srp")?),
            "--out-sor" => args.out_sor = Some(next_path(raw, &mut idx, "--out-sor")?),
            other => bail!("unknown srp-sor-update argument '{other}'"),
        }
        idx += 1;
    }
    Ok(args)
}

fn srp_sor_update_usage() -> &'static str {
    "adl tooling srp-sor-update --facts <facts.yaml> --srp <srp.md> --sor <sor.md> [--out-srp <srp.md>] [--out-sor <sor.md>]"
}

fn next_path(raw: &[String], idx: &mut usize, flag: &str) -> Result<PathBuf> {
    *idx += 1;
    raw.get(*idx)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("missing value for {flag}"))
}

impl FactPacket {
    fn has_actionable_facts(&self) -> bool {
        self.review.findings_status.is_some()
            || self.review.recommended_outcome.is_some()
            || self.review.notes.is_some()
            || self.validation.status.is_some()
            || !self.validation.commands.is_empty()
            || !self.integration.main_repo_paths_updated.is_empty()
            || self.integration.worktree_only_paths_remaining.is_some()
            || self.integration.integration_state.is_some()
            || self.integration.verification_scope.is_some()
            || self.integration.integration_method.is_some()
            || !self.integration.verification_performed.is_empty()
            || self.integration.result.is_some()
            || self.release_tail.watcher_disposition.is_some()
            || self.release_tail.pr_state.is_some()
            || self.release_tail.closeout_state.is_some()
            || !self.release_tail.residual_risks.is_empty()
            || self.metrics.actual_elapsed_seconds.is_some()
            || self.metrics.actual_total_tokens.is_some()
            || self.metrics.actual_validation_seconds.is_some()
            || self.metrics.goal_metrics_data_source.is_some()
            || self.metrics.goal_metrics_source_ref.is_some()
            || self.metrics.data_source_confidence.is_some()
    }

    fn validate_consistency(&self) -> Result<()> {
        let findings = self
            .review
            .findings_status
            .as_deref()
            .map(normalize_findings_status)
            .transpose()?;
        let outcome = self
            .review
            .recommended_outcome
            .as_deref()
            .map(normalize_recommended_outcome)
            .transpose()?;
        if matches!(findings.as_deref(), Some("findings_present"))
            && matches!(outcome.as_deref(), Some("pass"))
        {
            bail!(
                "contradictory review facts: recommended_outcome pass with findings_status findings_present"
            );
        }

        let validation_status = self
            .validation
            .status
            .as_deref()
            .map(normalize_status_token);
        if matches!(validation_status.as_deref(), Some("PASS"))
            && self
                .validation
                .commands
                .iter()
                .any(|command| command.result.as_deref().is_some_and(is_failure_status))
        {
            bail!("contradictory validation facts: validation.status PASS with a failing command result");
        }

        if self
            .release_tail
            .closeout_state
            .as_deref()
            .is_some_and(is_terminal_closeout_state)
        {
            ensure!(
                matches!(findings.as_deref(), Some("no_findings"))
                    && matches!(outcome.as_deref(), Some("pass")),
                "terminal closeout_state requires review.findings_status no_findings and review.recommended_outcome pass"
            );
            ensure!(
                matches!(validation_status.as_deref(), Some("PASS")),
                "terminal closeout_state requires validation.status PASS"
            );
            ensure!(
                self.integration
                    .integration_state
                    .as_deref()
                    .is_some_and(is_terminal_integration_state),
                "terminal closeout_state requires terminal integration.integration_state"
            );
            if !self
                .integration
                .integration_state
                .as_deref()
                .is_some_and(is_no_pr_terminal_integration_state)
            {
                ensure!(
                    self.release_tail
                        .pr_state
                        .as_deref()
                        .is_some_and(is_terminal_pr_state),
                    "terminal closeout_state requires terminal release_tail.pr_state unless integration.integration_state is closed_no_pr"
                );
            }
        }

        Ok(())
    }
}

fn update_srp(text: &str, facts: &FactPacket) -> Result<String> {
    let (frontmatter, body) = split_front_matter(text)?;
    let mut yaml: Value = serde_yaml::from_str(&frontmatter)?;
    let mapping = yaml
        .as_mapping_mut()
        .ok_or_else(|| anyhow!("SRP front matter must be a YAML mapping"))?;

    if facts.review.findings_status.is_some() || facts.review.recommended_outcome.is_some() {
        let review_results_key = Value::String("review_results".to_string());
        if !mapping.contains_key(&review_results_key) {
            mapping.insert(review_results_key.clone(), Value::Mapping(Mapping::new()));
        }
        let review_results = mapping
            .get_mut(&review_results_key)
            .and_then(Value::as_mapping_mut)
            .ok_or_else(|| anyhow!("SRP review_results must be a YAML mapping"))?;
        if let Some(value) = facts.review.findings_status.as_deref() {
            insert_string(
                review_results,
                "findings_status",
                &normalize_findings_status(value)?,
            );
        }
        if let Some(value) = facts.review.recommended_outcome.as_deref() {
            insert_string(
                review_results,
                "recommended_outcome",
                &normalize_recommended_outcome(value)?,
            );
        }
    }
    if let Some(value) = facts.review.notes.as_deref() {
        insert_string(mapping, "notes", value);
    }
    if facts.review.findings_status.is_some() || facts.review.recommended_outcome.is_some() {
        let key = Value::String("review_results_exception".to_string());
        mapping.remove(&key);
    }

    let frontmatter = serde_yaml::to_string(&yaml)?.trim_end().to_string();
    Ok(format!("---\n{frontmatter}\n---\n{body}\n"))
}

fn normalize_findings_status(value: &str) -> Result<String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "no_findings" | "no-findings" | "none" | "resolved" | "clean" | "pass" => {
            Ok("no_findings".to_string())
        }
        "findings_present" | "findings-present" | "findings" | "open" | "unresolved"
        | "blocked" => Ok("findings_present".to_string()),
        other => bail!(
            "unsupported review.findings_status '{other}' (expected no_findings or findings_present)"
        ),
    }
}

fn normalize_recommended_outcome(value: &str) -> Result<String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "pass" | "passed" | "approve" | "approved" => Ok("pass".to_string()),
        "block" | "blocked" | "fail" | "failed" => Ok("block".to_string()),
        "needs_followup" | "needs-followup" | "followup" | "follow-up" => {
            Ok("needs_followup".to_string())
        }
        other => bail!(
            "unsupported review.recommended_outcome '{other}' (expected pass, block, or needs_followup)"
        ),
    }
}

fn insert_string(mapping: &mut Mapping, key: &str, value: &str) {
    mapping.insert(
        Value::String(key.to_string()),
        Value::String(value.to_string()),
    );
}

fn update_sor(text: &str, facts: &FactPacket) -> Result<String> {
    let mut out = text.to_string();
    out = replace_metric_fields(&out, &facts.metrics);
    out = replace_integration_fields(&out, &facts.integration);
    out = replace_release_tail_fields(&out, &facts.release_tail);
    out = replace_validation_section(&out, &facts.validation);
    out = replace_verification_summary_validation(&out, &facts.validation)?;
    Ok(out)
}

fn assert_requested_facts_landed(srp_text: &str, sor_text: &str, facts: &FactPacket) -> Result<()> {
    let (frontmatter, _) = split_front_matter(srp_text)?;
    let yaml: Value = serde_yaml::from_str(&frontmatter)?;
    let mapping = yaml
        .as_mapping()
        .ok_or_else(|| anyhow!("updated SRP front matter must be a YAML mapping"))?;
    if facts.review.findings_status.is_some() || facts.review.recommended_outcome.is_some() {
        let review_results = mapping
            .get(Value::String("review_results".to_string()))
            .and_then(Value::as_mapping)
            .ok_or_else(|| anyhow!("requested SRP review facts did not land in review_results"))?;
        if let Some(value) = facts.review.findings_status.as_deref() {
            ensure_yaml_field(
                review_results,
                "findings_status",
                &normalize_findings_status(value)?,
            )?;
        }
        if let Some(value) = facts.review.recommended_outcome.as_deref() {
            ensure_yaml_field(
                review_results,
                "recommended_outcome",
                &normalize_recommended_outcome(value)?,
            )?;
        }
    }
    if let Some(value) = facts.review.notes.as_deref() {
        ensure_yaml_field(mapping, "notes", value)?;
    }

    for (block, label, value) in [
        (
            "Issue Metrics Truth",
            "Actual elapsed seconds",
            facts.metrics.actual_elapsed_seconds.as_deref(),
        ),
        (
            "Issue Metrics Truth",
            "Actual total tokens",
            facts.metrics.actual_total_tokens.as_deref(),
        ),
        (
            "Issue Metrics Truth",
            "Actual validation seconds",
            facts.metrics.actual_validation_seconds.as_deref(),
        ),
        (
            "Issue Metrics Truth",
            "Goal metrics data source",
            facts.metrics.goal_metrics_data_source.as_deref(),
        ),
        (
            "Issue Metrics Truth",
            "Goal metrics source ref",
            facts.metrics.goal_metrics_source_ref.as_deref(),
        ),
        (
            "Issue Metrics Truth",
            "Data-source confidence",
            facts.metrics.data_source_confidence.as_deref(),
        ),
        (
            "Main Repo Integration (REQUIRED)",
            "Worktree-only paths remaining",
            facts.integration.worktree_only_paths_remaining.as_deref(),
        ),
        (
            "Main Repo Integration (REQUIRED)",
            "Integration state",
            facts.integration.integration_state.as_deref(),
        ),
        (
            "Main Repo Integration (REQUIRED)",
            "Verification scope",
            facts.integration.verification_scope.as_deref(),
        ),
        (
            "Main Repo Integration (REQUIRED)",
            "Integration method used",
            facts.integration.integration_method.as_deref(),
        ),
        (
            "Main Repo Integration (REQUIRED)",
            "Result",
            facts.integration.result.as_deref(),
        ),
        (
            "Main Repo Integration (REQUIRED)",
            "Watcher disposition",
            facts.release_tail.watcher_disposition.as_deref(),
        ),
        (
            "Main Repo Integration (REQUIRED)",
            "PR state",
            facts.release_tail.pr_state.as_deref(),
        ),
        (
            "Main Repo Integration (REQUIRED)",
            "Closeout state",
            facts.release_tail.closeout_state.as_deref(),
        ),
    ] {
        if let Some(value) = value {
            let observed = markdown_block_field(sor_text, block, label).unwrap_or_default();
            ensure!(
                observed == value,
                "requested SOR fact {block}.{label} did not land; expected '{value}', observed '{observed}'"
            );
        }
    }

    for path in &facts.integration.main_repo_paths_updated {
        ensure!(
            sor_text.contains(&format!("  - `{}`", escape_backticks(path))),
            "requested SOR main-repo path did not land: {path}"
        );
    }
    for check in &facts.integration.verification_performed {
        ensure!(
            sor_text.contains(&format!("  - `{}`", escape_backticks(check))),
            "requested SOR verification item did not land: {check}"
        );
    }
    for command in &facts.validation.commands {
        ensure!(
            sor_text.contains(&format!("`{}`", escape_backticks(&command.command))),
            "requested validation command did not land: {}",
            command.command
        );
        ensure!(
            sor_text.contains(&format!("\"{}\"", yaml_double_quoted(&command.command))),
            "requested validation command did not land in Verification Summary: {}",
            command.command
        );
    }
    if let Some(status) = facts.validation.status.as_deref() {
        ensure!(
            sor_text.contains(&format!("    status: {}", normalize_status_token(status))),
            "requested validation status did not land in Verification Summary: {}",
            normalize_status_token(status)
        );
    }
    for risk in &facts.release_tail.residual_risks {
        ensure!(
            sor_text.contains(&format!("- {}", escape_backticks(risk))),
            "requested residual risk did not land: {risk}"
        );
    }

    Ok(())
}

fn ensure_yaml_field(mapping: &Mapping, key: &str, expected: &str) -> Result<()> {
    let observed = mapping
        .get(Value::String(key.to_string()))
        .and_then(Value::as_str)
        .unwrap_or_default();
    ensure!(
        observed == expected,
        "requested SRP fact {key} did not land; expected '{expected}', observed '{observed}'"
    );
    Ok(())
}

fn replace_metric_fields(text: &str, metrics: &MetricsFacts) -> String {
    let mut out = text.to_string();
    for (label, value) in [
        (
            "Actual elapsed seconds",
            metrics.actual_elapsed_seconds.as_deref(),
        ),
        (
            "Actual total tokens",
            metrics.actual_total_tokens.as_deref(),
        ),
        (
            "Actual validation seconds",
            metrics.actual_validation_seconds.as_deref(),
        ),
        (
            "Goal metrics data source",
            metrics.goal_metrics_data_source.as_deref(),
        ),
        (
            "Goal metrics source ref",
            metrics.goal_metrics_source_ref.as_deref(),
        ),
        (
            "Data-source confidence",
            metrics.data_source_confidence.as_deref(),
        ),
    ] {
        if let Some(value) = value {
            out = replace_dash_field(&out, label, &format!("`{value}`"));
        }
    }
    out
}

fn replace_integration_fields(text: &str, integration: &IntegrationFacts) -> String {
    replace_section_body(text, "Main Repo Integration (REQUIRED)", |section_text| {
        replace_integration_fields_in_section(section_text, integration)
    })
}

fn replace_integration_fields_in_section(text: &str, integration: &IntegrationFacts) -> String {
    let mut out = text.to_string();
    if !integration.main_repo_paths_updated.is_empty() {
        out = replace_indented_list_field(
            &out,
            "Main-repo paths updated",
            &integration.main_repo_paths_updated,
        );
    }
    for (label, value) in [
        (
            "Worktree-only paths remaining",
            integration.worktree_only_paths_remaining.as_deref(),
        ),
        (
            "Integration state",
            integration.integration_state.as_deref(),
        ),
        (
            "Verification scope",
            integration.verification_scope.as_deref(),
        ),
        (
            "Integration method used",
            integration.integration_method.as_deref(),
        ),
        ("Result", integration.result.as_deref()),
    ] {
        if let Some(value) = value {
            out = replace_dash_field(&out, label, value);
        }
    }
    if !integration.verification_performed.is_empty() {
        out = replace_indented_list_field(
            &out,
            "Verification performed",
            &integration.verification_performed,
        );
    }
    out
}

fn replace_release_tail_fields(text: &str, release_tail: &ReleaseTailFacts) -> String {
    let mut out = replace_section_body(text, "Main Repo Integration (REQUIRED)", |section_text| {
        replace_release_tail_fields_in_integration_section(section_text, release_tail)
    });
    if !release_tail.residual_risks.is_empty() {
        out = replace_followups_with_residual_risks(&out, &release_tail.residual_risks);
    }
    out
}

fn replace_release_tail_fields_in_integration_section(
    text: &str,
    release_tail: &ReleaseTailFacts,
) -> String {
    let mut out = text.to_string();
    for (label, value) in [
        (
            "Watcher disposition",
            release_tail.watcher_disposition.as_deref(),
        ),
        ("PR state", release_tail.pr_state.as_deref()),
        ("Closeout state", release_tail.closeout_state.as_deref()),
    ] {
        if let Some(value) = value {
            out = upsert_dash_field(&out, label, value, Some("Result"));
        }
    }
    out
}

fn replace_followups_with_residual_risks(text: &str, residual_risks: &[String]) -> String {
    replace_section_body(text, "Follow-ups / Deferred work", |section_text| {
        upsert_residual_risk_block(section_text, residual_risks)
    })
}

fn upsert_residual_risk_block(text: &str, residual_risks: &[String]) -> String {
    let mut out = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim_start() == "- Residual risks:" {
            while let Some(next) = lines.peek() {
                let trimmed = next.trim_start();
                let indent_len = next.len() - trimmed.len();
                if indent_len == 0 && trimmed.starts_with("- ") {
                    break;
                }
                lines.next();
            }
        } else {
            out.push(line.to_string());
        }
    }
    if !out.is_empty() && out.last().is_some_and(|line| !line.trim().is_empty()) {
        out.push(String::new());
    }
    out.push("- Residual risks:".to_string());
    for risk in residual_risks {
        out.push(format!("  - {}", escape_backticks(risk)));
    }
    out.join("\n") + trailing_newline(text)
}

fn replace_validation_section(text: &str, validation: &ValidationFacts) -> String {
    if validation.commands.is_empty() && validation.status.is_none() {
        return text.to_string();
    }

    let mut replacement = String::from("## Validation\n");
    if validation.commands.is_empty() {
        replacement.push_str("- Validation commands and their purpose:\n  - `not_recorded` - no command facts supplied\n");
    } else {
        replacement.push_str("- Validation commands and their purpose:\n");
        for command in &validation.commands {
            replacement.push_str(&format!(
                "  - `{}` - {}\n",
                escape_backticks(&command.command),
                command.purpose
            ));
        }
    }
    replacement.push_str("- Results:\n");
    if validation.commands.is_empty() {
        replacement.push_str(&format!(
            "  - {}\n",
            validation.status.as_deref().unwrap_or("not_recorded")
        ));
    } else {
        for command in &validation.commands {
            replacement.push_str(&format!(
                "  - `{}`: {}\n",
                escape_backticks(&command.command),
                command.result.as_deref().unwrap_or("not_recorded")
            ));
        }
    }

    replace_section(text, "Validation", &replacement)
}

fn replace_verification_summary_validation(
    text: &str,
    validation: &ValidationFacts,
) -> Result<String> {
    if validation.commands.is_empty() && validation.status.is_none() {
        return Ok(text.to_string());
    }
    let Some(start) = text.find("verification_summary:\n") else {
        return Ok(text.to_string());
    };
    let tail = &text[start..];
    let Some(validation_start_rel) = tail.find("  validation:\n") else {
        return Ok(text.to_string());
    };
    let validation_start = start + validation_start_rel;
    let after_validation = &text[validation_start + "  validation:\n".len()..];
    let next_section_rel = after_validation.find("\n  determinism:").ok_or_else(|| {
        anyhow!("Verification Summary validation block missing determinism boundary")
    })?;
    let block_end = validation_start + "  validation:\n".len() + next_section_rel;

    let mut block = String::from("  validation:\n");
    block.push_str(&format!(
        "    status: {}\n",
        validation
            .status
            .as_deref()
            .map(normalize_status_token)
            .unwrap_or_else(|| "not_recorded".to_string())
    ));
    block.push_str("    checks_run:\n");
    if validation.commands.is_empty() {
        block.push_str("      - \"not_recorded\"\n");
    } else {
        for command in &validation.commands {
            block.push_str(&format!(
                "      - \"{}\"\n",
                yaml_double_quoted(&command.command)
            ));
        }
    }

    let mut out = String::with_capacity(text.len() + block.len());
    out.push_str(&text[..validation_start]);
    out.push_str(&block);
    out.push_str(&text[block_end..]);
    Ok(out)
}

fn replace_dash_field(text: &str, label: &str, value: &str) -> String {
    let prefix = format!("- {label}:");
    text.lines()
        .map(|line| {
            if line.trim_start().starts_with(&prefix) {
                let indent_len = line.len() - line.trim_start().len();
                format!("{}- {label}: {value}", " ".repeat(indent_len))
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + trailing_newline(text)
}

fn upsert_dash_field(text: &str, label: &str, value: &str, after_label: Option<&str>) -> String {
    let prefix = format!("- {label}:");
    if text
        .lines()
        .any(|line| line.trim_start().starts_with(&prefix))
    {
        return replace_dash_field(text, label, value);
    }

    let after_prefix = after_label.map(|label| format!("- {label}:"));
    let mut out = Vec::new();
    let mut inserted = false;
    for line in text.lines() {
        out.push(line.to_string());
        if !inserted
            && after_prefix
                .as_ref()
                .is_some_and(|prefix| line.trim_start().starts_with(prefix))
        {
            let indent_len = line.len() - line.trim_start().len();
            out.push(format!("{}- {label}: {value}", " ".repeat(indent_len)));
            inserted = true;
        }
    }
    if !inserted {
        out.push(format!("- {label}: {value}"));
    }
    out.join("\n") + trailing_newline(text)
}

fn replace_indented_list_field(text: &str, label: &str, values: &[String]) -> String {
    let marker = format!("- {label}:");
    let mut out = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim_start().starts_with(&marker) {
            let indent_len = line.len() - line.trim_start().len();
            let indent = " ".repeat(indent_len);
            out.push(format!("{indent}- {label}:"));
            for value in values {
                out.push(format!("{indent}  - `{}`", escape_backticks(value)));
            }
            while let Some(next) = lines.peek() {
                let trimmed = next.trim_start();
                if trimmed.starts_with("- ") && !trimmed.starts_with("- `") {
                    break;
                }
                if next.starts_with("## ") {
                    break;
                }
                if trimmed.is_empty() {
                    out.push(lines.next().unwrap().to_string());
                    break;
                }
                let next_indent = next.len() - trimmed.len();
                if next_indent <= indent_len {
                    break;
                }
                lines.next();
            }
        } else {
            out.push(line.to_string());
        }
    }
    out.join("\n") + trailing_newline(text)
}

fn replace_section(text: &str, heading: &str, replacement: &str) -> String {
    let section = format!("## {heading}");
    let mut out = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim_end() == section {
            out.extend(replacement.trim_end().lines().map(str::to_string));
            while let Some(next) = lines.peek() {
                if next.starts_with("## ") {
                    break;
                }
                lines.next();
            }
        } else {
            out.push(line.to_string());
        }
    }
    out.join("\n") + trailing_newline(text)
}

fn replace_section_body(text: &str, heading: &str, replace: impl FnOnce(&str) -> String) -> String {
    let section = format!("## {heading}");
    let mut before = String::new();
    let mut body = String::new();
    let mut after = String::new();
    let mut state = 0;

    for line in text.lines() {
        if state == 0 {
            before.push_str(line);
            before.push('\n');
            if line.trim_end() == section {
                state = 1;
            }
            continue;
        }
        if state == 1 && line.starts_with("## ") {
            state = 2;
        }
        if state == 1 {
            body.push_str(line);
            body.push('\n');
        } else {
            after.push_str(line);
            after.push('\n');
        }
    }

    if state == 0 {
        return text.to_string();
    }

    let mut out = before;
    out.push_str(replace(&body).trim_end());
    out.push('\n');
    out.push_str(&after);
    if !text.ends_with('\n') && out.ends_with('\n') {
        out.pop();
    }
    out
}

fn write_text(path: &Path, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, text).with_context(|| format!("write {}", path.display()))
}

fn escape_backticks(value: &str) -> String {
    value.replace('`', "\\`")
}

fn yaml_double_quoted(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn normalize_status_token(value: &str) -> String {
    match value.trim().to_ascii_lowercase().as_str() {
        "pass" | "passed" | "success" | "succeeded" | "green" => "PASS".to_string(),
        "fail" | "failed" | "failure" | "red" => "FAIL".to_string(),
        "skip" | "skipped" => "SKIPPED".to_string(),
        "not_run" | "not-run" | "not recorded" | "not_recorded" => "not_recorded".to_string(),
        other => other.to_string(),
    }
}

fn is_failure_status(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "fail" | "failed" | "failure" | "red" | "blocked"
    )
}

fn is_terminal_closeout_state(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "closed" | "complete" | "completed" | "closeout_complete" | "merged_and_closed"
    )
}

fn is_terminal_integration_state(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "merged" | "main_repo" | "main_repo_integrated" | "closed" | "closed_no_pr" | "complete"
    )
}

fn is_no_pr_terminal_integration_state(value: &str) -> bool {
    value.trim().eq_ignore_ascii_case("closed_no_pr")
}

fn is_terminal_pr_state(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "merged" | "closed_merged" | "merged_and_closed" | "closed"
    )
}

fn trailing_newline(text: &str) -> &'static str {
    if text.ends_with('\n') {
        "\n"
    } else {
        ""
    }
}
