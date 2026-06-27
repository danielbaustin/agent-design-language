use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

mod structure;
mod values;
pub(crate) use structure::validate_rendered_card_structure_from_repo;
use structure::{build_structure_schema, default_structure_schema_path, load_structure_schema};
use values::{
    load_values_document, load_values_file, sample_values_document, PromptValuesDocument,
};

const TEMPLATE_REGISTRY: &str = "docs/templates/prompts/current.json";
const VALUES_SCHEMA: &str = "adl.csdlc.prompt_template_values.v1";
const CARD_STATUS_VALUES: &[&str] = &[
    "draft",
    "ready",
    "reviewed",
    "approved",
    "completed",
    "blocked",
    "superseded",
];
const PLACEHOLDERS: &[&str] = &[
    "card_status",
    "issue",
    "issue_padded",
    "task_id",
    "run_id",
    "version",
    "slug",
    "title",
    "branch",
    "issue_url",
    "source_issue_prompt",
    "docs_context",
    "output_card",
    "stp_card",
    "sip_card",
    "spp_card",
    "vpp_card",
    "srp_card",
    "sor_card",
    "wp",
    "required_outcome_type",
    "demo_required",
    "issue_graph_note",
    "summary",
    "goal",
    "required_outcome",
    "deliverables",
    "acceptance_criteria",
    "inputs",
    "repo_inputs",
    "dependencies",
    "target_files_surfaces",
    "validation_plan",
    "demo_proof_requirements",
    "non_goals",
    "issue_graph_notes",
    "notes_risks",
    "tooling_notes",
    "target_files_surfaces_inline",
    "non_goals_inline",
    "plan_summary",
    "dependencies_inline",
    "repo_inputs_inline",
    "deliverables_inline",
    "acceptance_criteria_inline",
    "risks_inline",
    "validation_plan_inline",
    "notes_risks_inline",
    "status",
    "activation_state",
    "timestamp",
    "initial_pvf_lane",
    "planned_pvf_lane",
    "planned_pvf_lane_source",
    "issue_goal_ref",
    "sprint_goal_ref",
    "goal_metrics_rollup_ref",
    "lane_registry_path",
    "lane_registry_template_set",
    "validation_runtime_class",
    "validation_resource_profile",
    "validation_family",
    "validation_size_split",
    "expected_proof_cost",
    "planned_validation_seconds",
    "planned_validation_tokens",
    "selected_lanes_inline",
    "parallel_groups_inline",
    "validation_commands_inline",
    "failure_policy",
    "estimated_elapsed_seconds",
    "estimated_total_tokens",
    "estimated_validation_seconds",
    "expected_runtime_class",
    "estimate_elapsed_seconds",
    "estimate_total_tokens",
    "estimate_validation_seconds",
    "issue_goal_token_budget",
    "variance_threshold_percent",
    "estimate_confidence",
    "estimate_data_source",
    "estimate_source_ref",
    "final_pvf_lane",
    "lane_change_reason",
    "actual_elapsed_seconds",
    "actual_active_work_seconds",
    "actual_total_tokens",
    "actual_validation_seconds",
    "budget_source",
    "actual_pr_wait_seconds",
    "actual_ci_wait_seconds",
    "actual_metrics_data_source",
    "actual_metrics_source_ref",
    "actual_metrics_confidence",
    "estimate_error_percent",
    "completion_state",
    "variance_analysis_required",
    "variance_analysis_completed",
    "variance_category",
    "variance_note",
    "execution_actor",
    "model",
    "provider",
    "start_time",
    "end_time",
    "tracked_implementation_artifacts",
    "additional_proof_artifacts",
    "actions_taken_line_1",
    "actions_taken_line_2",
    "actions_taken_line_3",
    "main_repo_paths_updated",
    "worktree_only_paths_remaining",
    "integration_state",
    "verification_scope",
    "integration_method_used",
    "integration_verification_command",
    "integration_verification_effect",
    "integration_result",
    "validation_command",
    "validation_effect",
    "validation_result",
    "verification_validation_status",
    "verification_check_1",
    "verification_determinism_status",
    "verification_replay_verified",
    "verification_ordering_guarantees_verified",
    "verification_security_privacy_status",
    "verification_secrets_leakage_detected",
    "verification_prompt_or_tool_arg_leakage_detected",
    "verification_absolute_path_leakage_detected",
    "verification_artifacts_status",
    "verification_required_artifacts_present",
    "verification_schema_changes_present",
    "verification_schema_changes_approved",
    "determinism_tests_executed",
    "fixtures_or_scripts_used",
    "replay_verification",
    "ordering_guarantees",
    "artifact_stability_notes",
    "secret_leakage_scan_performed",
    "prompt_tool_arg_redaction_verified",
    "absolute_path_leakage_check",
    "sandbox_policy_invariants_preserved",
    "trace_bundle_paths",
    "run_artifact_root",
    "replay_command",
    "replay_result",
    "primary_proof_surface",
    "required_artifacts_present",
    "artifact_schema_checks",
    "hash_byte_stability_checks",
    "missing_optional_artifacts_rationale",
    "decision_or_deviation_1",
    "decision_or_deviation_2",
    "follow_up_1",
    "follow_up_2",
    "branch_action",
    "findings_status",
    "recommended_outcome",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptCardKind {
    Sip,
    Stp,
    Spp,
    Vpp,
    Srp,
    Sor,
}

impl PromptCardKind {
    pub fn all_for_template_set(template_set: &str) -> Vec<Self> {
        let mut kinds = vec![Self::Sip, Self::Stp, Self::Spp];
        if template_set_supports_vpp(template_set) {
            kinds.push(Self::Vpp);
        }
        kinds.push(Self::Srp);
        kinds.push(Self::Sor);
        kinds
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Sip => "sip",
            Self::Stp => "stp",
            Self::Spp => "spp",
            Self::Vpp => "vpp",
            Self::Srp => "srp",
            Self::Sor => "sor",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Sip => "Structured Issue Prompt",
            Self::Stp => "Structured Task Prompt",
            Self::Spp => "Structured Plan Prompt",
            Self::Vpp => "Structured Validation Planning Prompt",
            Self::Srp => "Structured Review Prompt",
            Self::Sor => "Structured Outcome Record",
        }
    }

    pub fn output_file(self) -> &'static str {
        match self {
            Self::Sip => "sip.md",
            Self::Stp => "stp.md",
            Self::Spp => "spp.md",
            Self::Vpp => "vpp.md",
            Self::Srp => "srp.md",
            Self::Sor => "sor.md",
        }
    }

    pub fn validate_type(self) -> &'static str {
        self.key()
    }

    pub fn parse_key(value: &str) -> Result<Self> {
        match value {
            "sip" => Ok(Self::Sip),
            "stp" => Ok(Self::Stp),
            "spp" => Ok(Self::Spp),
            "vpp" => Ok(Self::Vpp),
            "srp" => Ok(Self::Srp),
            "sor" => Ok(Self::Sor),
            other => bail!("card kind must be one of sip, stp, spp, vpp, srp, sor: {other}"),
        }
    }
}

fn template_set_supports_vpp(template_set: &str) -> bool {
    let mut parts = template_set.split('.');
    let Some(major) = parts.next().and_then(|value| value.parse::<u64>().ok()) else {
        return false;
    };
    let Some(minor) = parts.next().and_then(|value| value.parse::<u64>().ok()) else {
        return false;
    };
    let Some(patch) = parts.next().and_then(|value| value.parse::<u64>().ok()) else {
        return false;
    };
    (major, minor, patch) >= (1, 0, 3)
}

#[derive(Debug, Clone, Serialize)]
pub struct PromptField {
    pub key: &'static str,
    pub label: &'static str,
    pub input: &'static str,
    pub required: bool,
    pub editable: bool,
    pub help: &'static str,
    pub enum_values: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PromptCardForm {
    pub kind: PromptCardKind,
    pub key: &'static str,
    pub label: &'static str,
    pub output_file: &'static str,
    pub template_path: String,
    pub structure_schema_path: String,
    pub fields: Vec<PromptField>,
    pub template: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PromptEditorModel {
    pub schema: &'static str,
    pub template_set: String,
    pub registry_path: &'static str,
    pub card_status_values: &'static [&'static str],
    pub cards: Vec<PromptCardForm>,
}

#[derive(Debug, Deserialize)]
struct Registry {
    csdlc_prompt_template_set: String,
    templates: BTreeMap<String, RegistryTemplate>,
}

#[derive(Debug, Deserialize)]
struct RegistryTemplate {
    path: String,
    structure_schema_path: Option<String>,
}

pub fn load_editor_model(repo_root: &Path) -> Result<PromptEditorModel> {
    load_editor_model_for_template_set(repo_root, None)
}

fn active_prompt_template_set(repo_root: &Path) -> Result<String> {
    let registry_path = repo_root.join(TEMPLATE_REGISTRY);
    let raw = fs::read_to_string(&registry_path)
        .with_context(|| format!("failed to read {}", registry_path.display()))?;
    let registry: Registry = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse {}", registry_path.display()))?;
    Ok(registry.csdlc_prompt_template_set)
}

fn template_set_from_values_file(values_path: &Path) -> Result<String> {
    let raw = fs::read_to_string(values_path)
        .with_context(|| format!("failed to read values file {}", values_path.display()))?;
    let doc: Value = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed to parse values file {}", values_path.display()))?;
    let mapping = doc
        .as_mapping()
        .ok_or_else(|| anyhow!("values file must be a YAML/JSON mapping"))?;
    mapping
        .get(Value::String("template_set".to_string()))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow!("values file must declare template_set"))
}

fn load_editor_model_for_template_set(
    repo_root: &Path,
    template_set_override: Option<&str>,
) -> Result<PromptEditorModel> {
    let registry_path = repo_root.join(TEMPLATE_REGISTRY);
    let raw = fs::read_to_string(&registry_path)
        .with_context(|| format!("failed to read {}", registry_path.display()))?;
    let registry: Registry = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse {}", registry_path.display()))?;
    let template_set = template_set_override.unwrap_or(&registry.csdlc_prompt_template_set);

    let mut cards = Vec::new();
    for kind in PromptCardKind::all_for_template_set(template_set) {
        let (template_path_str, structure_schema_path) =
            if template_set == registry.csdlc_prompt_template_set {
                let template = registry
                    .templates
                    .get(kind.key())
                    .ok_or_else(|| anyhow!("registry missing template for {}", kind.key()))?;
                (
                    template.path.clone(),
                    template
                        .structure_schema_path
                        .clone()
                        .unwrap_or_else(|| default_structure_schema_path(template_set, kind)),
                )
            } else {
                (
                    format!("docs/templates/prompts/{template_set}/{}.md", kind.key()),
                    default_structure_schema_path(template_set, kind),
                )
            };
        let template_path = repo_root.join(&template_path_str);
        let template_text = fs::read_to_string(&template_path)
            .with_context(|| format!("failed to read {}", template_path.display()))?;
        cards.push(PromptCardForm {
            kind,
            key: kind.key(),
            label: kind.label(),
            output_file: kind.output_file(),
            template_path: template_path_str,
            structure_schema_path,
            fields: form_fields(kind),
            template: template_text,
        });
    }

    Ok(PromptEditorModel {
        schema: "adl.csdlc.prompt_editor.model.v1",
        template_set: template_set.to_string(),
        registry_path: TEMPLATE_REGISTRY,
        card_status_values: CARD_STATUS_VALUES,
        cards,
    })
}

pub fn render_sample_card(repo_root: &Path, kind: PromptCardKind) -> Result<String> {
    let model = load_editor_model(repo_root)?;
    let card = model
        .cards
        .iter()
        .find(|card| card.kind == kind)
        .ok_or_else(|| anyhow!("missing card model for {}", kind.key()))?;
    let values = sample_values_for_kind(kind);
    validate_values(card, &values)?;
    let rendered = render_template(&card.template, &values)?;
    validate_rendered_card_structure_from_repo(repo_root, card, &rendered)?;
    Ok(rendered)
}

pub fn render_card_from_values_file(
    repo_root: &Path,
    kind: PromptCardKind,
    values_path: &Path,
) -> Result<String> {
    let template_set = template_set_from_values_file(values_path)?;
    let model = load_editor_model_for_template_set(repo_root, Some(&template_set))?;
    let card = card_model(&model, kind)?;
    let values = load_values_file(card, values_path, &model.template_set)?;
    validate_values(card, &values)?;
    let rendered = render_template(&card.template, &values)?;
    validate_rendered_card_structure_from_repo(repo_root, card, &rendered)?;
    Ok(rendered)
}

pub fn validate_values_file(
    repo_root: &Path,
    kind: PromptCardKind,
    values_path: &Path,
) -> Result<()> {
    let template_set = template_set_from_values_file(values_path)?;
    let model = load_editor_model_for_template_set(repo_root, Some(&template_set))?;
    let card = card_model(&model, kind)?;
    let values = load_values_file(card, values_path, &model.template_set)?;
    validate_values(card, &values)
}

pub fn edit_values_file(
    repo_root: &Path,
    kind: PromptCardKind,
    values_path: &Path,
    updates: &[(String, String)],
    out_path: Option<&Path>,
) -> Result<PathBuf> {
    ensure!(
        !updates.is_empty(),
        "edit-values requires at least one --set field=value update"
    );
    let template_set = template_set_from_values_file(values_path)?;
    let model = load_editor_model_for_template_set(repo_root, Some(&template_set))?;
    let card = card_model(&model, kind)?;
    let mut doc = load_values_document(card, values_path, &model.template_set)?;

    for (key, value) in updates {
        let field = card
            .fields
            .iter()
            .find(|field| field.key == key.as_str())
            .ok_or_else(|| {
                anyhow!(
                    "{}.{} is not a declared prompt-template field",
                    card.key,
                    key
                )
            })?;
        ensure!(
            field.editable,
            "{}.{} is locked; edit only declared values fields",
            card.key,
            key
        );
        doc.values.insert(key.clone(), value.clone());
    }

    let values = doc.merged_values();
    validate_values(card, &values)?;
    let rendered = render_template(&card.template, &values)?;
    validate_rendered_card_structure_from_repo(repo_root, card, &rendered)?;

    let target = out_path.unwrap_or(values_path).to_path_buf();
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&target, doc.to_yaml(card))
        .with_context(|| format!("failed to write edited values file {}", target.display()))?;
    Ok(target)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptCardRoundTripComparison {
    Exact,
    Normalized,
}

impl PromptCardRoundTripComparison {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Normalized => "normalized",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PromptCardImportReport {
    pub values_path: PathBuf,
    pub normalized_path: Option<PathBuf>,
    pub comparison: PromptCardRoundTripComparison,
    pub unrepresented_required_fields: Vec<String>,
}

pub fn import_values_from_rendered_card_file(
    repo_root: &Path,
    kind: PromptCardKind,
    input_path: &Path,
    out_path: &Path,
    normalized_out_path: Option<&Path>,
    template_set_override: Option<&str>,
) -> Result<PromptCardImportReport> {
    let model = load_editor_model_for_template_set(repo_root, template_set_override)?;
    let card = card_model(&model, kind)?;
    let source = fs::read_to_string(input_path)
        .with_context(|| format!("failed to read rendered card {}", input_path.display()))?;
    let source_template_set = detect_rendered_card_template_set(&source, kind)
        .unwrap_or_else(|| model.template_set.clone());
    let source_model = load_editor_model_for_template_set(repo_root, Some(&source_template_set))?;
    let source_card = card_model(&source_model, kind)?;

    let (doc, unrepresented_required_fields) =
        match validate_rendered_card_structure_from_repo(repo_root, source_card, &source) {
            Ok(()) => import_values_document_from_rendered_card(
                source_card,
                &model.template_set,
                &source,
            )?,
            Err(err) => {
                ensure!(source_template_set != model.template_set, "{err}");
                import_values_document_from_legacy_rendered_card(
                    card,
                    kind,
                    &model.template_set,
                    &source,
                )?
            }
        };
    let values = doc.merged_values();
    validate_values(card, &values)?;

    let rendered = render_template(&card.template, &values)?;
    validate_rendered_card_structure_from_repo(repo_root, card, &rendered)?;

    let comparison = if rendered == source {
        PromptCardRoundTripComparison::Exact
    } else {
        PromptCardRoundTripComparison::Normalized
    };

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(out_path, doc.to_yaml(card)).with_context(|| {
        format!(
            "failed to write imported values file {}",
            out_path.display()
        )
    })?;

    if let Some(normalized_out_path) = normalized_out_path {
        if let Some(parent) = normalized_out_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(normalized_out_path, rendered).with_context(|| {
            format!(
                "failed to write normalized rendered card {}",
                normalized_out_path.display()
            )
        })?;
    }

    Ok(PromptCardImportReport {
        values_path: out_path.to_path_buf(),
        normalized_path: normalized_out_path.map(Path::to_path_buf),
        comparison,
        unrepresented_required_fields,
    })
}

fn import_values_document_from_rendered_card(
    card: &PromptCardForm,
    template_set: &str,
    rendered: &str,
) -> Result<(PromptValuesDocument, Vec<String>)> {
    let mut extracted = match extract_template_values(card, &card.template, rendered) {
        Ok(values) => values,
        Err(err)
            if card.key == "stp"
                && err.to_string().contains(
                    "stp rendered card cannot find post-placeholder literal for title",
                ) =>
        {
            extract_active_stp_values(rendered).with_context(|| err.to_string())?
        }
        Err(err) => return Err(err),
    };
    let unrepresented_required_fields =
        populate_unrepresented_required_import_fields(card, &mut extracted)?;
    Ok(prompt_values_document_from_extracted(
        card,
        template_set,
        extracted,
        unrepresented_required_fields,
    ))
}

fn extract_active_stp_values(rendered: &str) -> Result<BTreeMap<String, String>> {
    let (frontmatter, _) = split_front_matter_local(rendered)?;
    let doc = serde_yaml::from_str::<Value>(&frontmatter)
        .context("stp import requires valid YAML front matter")?;
    let mapping = doc
        .as_mapping()
        .ok_or_else(|| anyhow!("stp front matter must be a mapping"))?;

    let issue = mapping_required_scalar(mapping, "issue_number")?;
    let title = mapping_required_scalar(mapping, "title")?;
    let version = version_from_rendered_title(&title)
        .or_else(|| mapping_required_scalar(mapping, "milestone_sprint").ok())
        .unwrap_or_else(|| "v0.0.0-imported".to_string());
    let required_outcome_type =
        yaml_sequence_summary(mapping.get(Value::String("required_outcome_type".to_string())))
            .unwrap_or_else(|| "combination".to_string());
    let repo_inputs = markdown_section_body_local(rendered, "Repo Inputs")
        .map(|body| body.trim().to_string())
        .filter(|body| !body.is_empty())
        .ok_or_else(|| anyhow!("stp import requires Repo Inputs section"))?;

    let mut values = BTreeMap::new();
    values.insert("issue".to_string(), issue.clone());
    values.insert("version".to_string(), version);
    values.insert(
        "timestamp".to_string(),
        mapping_required_scalar(mapping, "generated_at")
            .unwrap_or_else(|_| "legacy_import_unknown_timestamp".to_string()),
    );
    values.insert(
        "card_status".to_string(),
        mapping_required_scalar(mapping, "card_status").unwrap_or_else(|_| "draft".to_string()),
    );
    values.insert(
        "slug".to_string(),
        mapping_required_scalar(mapping, "slug")
            .unwrap_or_else(|_| "imported-rendered-prompt-card".to_string()),
    );
    values.insert("title".to_string(), title);
    values.insert(
        "wp".to_string(),
        mapping_required_scalar(mapping, "wp").unwrap_or_else(|_| "unassigned".to_string()),
    );
    values.insert("required_outcome_type".to_string(), required_outcome_type);
    values.insert(
        "demo_required".to_string(),
        mapping_required_scalar(mapping, "demo_required").unwrap_or_else(|_| "false".to_string()),
    );
    values.insert(
        "summary".to_string(),
        required_markdown_section(rendered, "Summary")?,
    );
    values.insert(
        "goal".to_string(),
        required_markdown_section(rendered, "Goal")?,
    );
    values.insert(
        "required_outcome".to_string(),
        required_markdown_section(rendered, "Required Outcome")?,
    );
    values.insert(
        "deliverables".to_string(),
        required_markdown_section(rendered, "Deliverables")?,
    );
    values.insert(
        "acceptance_criteria".to_string(),
        required_markdown_section(rendered, "Acceptance Criteria")?,
    );
    values.insert("repo_inputs".to_string(), repo_inputs.clone());
    values.insert(
        "dependencies".to_string(),
        required_markdown_section(rendered, "Dependencies")?,
    );
    values.insert(
        "target_files_surfaces".to_string(),
        required_markdown_section(rendered, "Target Files / Surfaces")?,
    );
    values.insert(
        "validation_plan".to_string(),
        required_markdown_section(rendered, "Validation Plan")?,
    );
    values.insert(
        "demo_proof_requirements".to_string(),
        required_markdown_section(rendered, "Demo Expectations")?,
    );
    values.insert(
        "non_goals".to_string(),
        required_markdown_section(rendered, "Non-goals")?,
    );
    values.insert(
        "issue_graph_notes".to_string(),
        required_markdown_section(rendered, "Issue-Graph Notes")?,
    );
    values.insert(
        "issue_graph_note".to_string(),
        values["issue_graph_notes"]
            .lines()
            .find(|line| !line.trim().is_empty())
            .map(str::trim)
            .unwrap_or("not recorded in rendered card")
            .to_string(),
    );
    values.insert(
        "notes_risks".to_string(),
        required_markdown_section(rendered, "Notes")?,
    );
    values.insert(
        "tooling_notes".to_string(),
        required_markdown_section(rendered, "Tooling Notes")?,
    );
    Ok(values)
}

fn required_markdown_section(rendered: &str, heading: &str) -> Result<String> {
    markdown_section_body_local(rendered, heading)
        .map(|body| body.trim().to_string())
        .filter(|body| !body.is_empty())
        .ok_or_else(|| anyhow!("stp import requires {heading} section"))
}

fn version_from_rendered_title(title: &str) -> Option<String> {
    let start = title.find("[v")?;
    let rest = &title[start + 1..];
    let end = rest.find(']')?;
    Some(rest[..end].to_string())
}

fn import_values_document_from_legacy_rendered_card(
    active_card: &PromptCardForm,
    kind: PromptCardKind,
    template_set: &str,
    rendered: &str,
) -> Result<(PromptValuesDocument, Vec<String>)> {
    let (extracted, unrepresented_required_fields) = match kind {
        PromptCardKind::Spp => extract_legacy_spp_values(rendered)?,
        PromptCardKind::Sor => extract_legacy_sor_values(rendered)?,
        _ => bail!("legacy import is only supported for spp and sor"),
    };
    if kind == PromptCardKind::Sor {
        ensure_legacy_sor_is_representable_in_active_template(&extracted)?;
    }
    Ok(prompt_values_document_from_extracted(
        active_card,
        template_set,
        extracted,
        unrepresented_required_fields,
    ))
}

fn prompt_values_document_from_extracted(
    card: &PromptCardForm,
    template_set: &str,
    extracted: BTreeMap<String, String>,
    unrepresented_required_fields: Vec<String>,
) -> (PromptValuesDocument, Vec<String>) {
    let editable_keys = card
        .fields
        .iter()
        .filter(|field| field.editable)
        .map(|field| field.key)
        .collect::<BTreeSet<_>>();

    let mut system = BTreeMap::new();
    let mut values = BTreeMap::new();
    for (key, value) in extracted {
        if editable_keys.contains(key.as_str()) {
            values.insert(key, value);
        } else {
            system.insert(key, value);
        }
    }

    (
        PromptValuesDocument {
            schema: VALUES_SCHEMA.to_string(),
            template_set: template_set.to_string(),
            card_kind: Some(card.key.to_string()),
            system,
            values,
        },
        unrepresented_required_fields,
    )
}

fn extract_legacy_spp_values(rendered: &str) -> Result<(BTreeMap<String, String>, Vec<String>)> {
    let (frontmatter, _) = split_front_matter_local(rendered)?;
    let doc = serde_yaml::from_str::<Value>(&frontmatter)
        .context("legacy spp import requires valid YAML front matter")?;
    let mapping = doc
        .as_mapping()
        .ok_or_else(|| anyhow!("legacy spp front matter must be a mapping"))?;

    let mut values = BTreeMap::new();
    for key in [
        "issue",
        "task_id",
        "run_id",
        "version",
        "title",
        "branch",
        "card_status",
        "status",
        "activation_state",
        "plan_summary",
        "name",
        "notes",
    ] {
        let value = mapping_required_scalar(mapping, key)?;
        let value = if key == "activation_state" {
            normalize_legacy_spp_activation_state(&value)
        } else {
            value
        };
        values.insert(key.to_string(), value);
    }

    let issue = values
        .get("issue")
        .cloned()
        .ok_or_else(|| anyhow!("legacy spp import requires issue"))?;
    values.insert("issue_padded".to_string(), issue.clone());
    values.insert(
        "slug".to_string(),
        values
            .get("name")
            .and_then(|name| name.strip_suffix("-execution-plan"))
            .unwrap_or("legacy-imported-spp")
            .to_string(),
    );
    values.insert(
        "issue_url".to_string(),
        legacy_source_ref(mapping, "issue")
            .ok_or_else(|| anyhow!("legacy spp import requires source_refs.issue"))?,
    );
    values.insert(
        "source_issue_prompt".to_string(),
        legacy_source_ref(mapping, "source_issue_prompt")
            .ok_or_else(|| anyhow!("legacy spp import requires source_refs.source_issue_prompt"))?,
    );
    values.insert(
        "stp_card".to_string(),
        legacy_source_ref(mapping, "stp")
            .ok_or_else(|| anyhow!("legacy spp import requires source_refs.stp"))?,
    );
    values.insert(
        "sip_card".to_string(),
        legacy_source_ref(mapping, "sip")
            .ok_or_else(|| anyhow!("legacy spp import requires source_refs.sip"))?,
    );

    let scope = mapping
        .get(Value::String("scope".to_string()))
        .and_then(Value::as_mapping)
        .ok_or_else(|| anyhow!("legacy spp import requires scope mapping"))?;
    values.insert(
        "target_files_surfaces_inline".to_string(),
        yaml_sequence_summary(scope.get(Value::String("files".to_string())))
            .unwrap_or_else(|| "not recorded in legacy rendered card".to_string()),
    );
    values.insert(
        "non_goals_inline".to_string(),
        yaml_sequence_summary(scope.get(Value::String("out_of_scope".to_string())))
            .unwrap_or_else(|| "not recorded in legacy rendered card".to_string()),
    );
    values.insert(
        "risks_inline".to_string(),
        yaml_sequence_summary(mapping.get(Value::String("risks_and_edge_cases".to_string())))
            .unwrap_or_else(|| "not recorded in legacy rendered card".to_string()),
    );
    values.insert(
        "validation_plan_inline".to_string(),
        yaml_sequence_summary(mapping.get(Value::String("test_strategy".to_string())))
            .unwrap_or_else(|| "not recorded in legacy rendered card".to_string()),
    );
    values.insert(
        "notes_risks_inline".to_string(),
        values
            .get("notes")
            .cloned()
            .unwrap_or_else(|| "not recorded in legacy rendered card".to_string()),
    );

    let mut defaulted = Vec::new();
    for (key, value) in [
        (
            "dependencies_inline",
            "not recorded in legacy rendered card",
        ),
        ("repo_inputs_inline", "not recorded in legacy rendered card"),
        (
            "deliverables_inline",
            "not recorded in legacy rendered card",
        ),
        (
            "acceptance_criteria_inline",
            "not recorded in legacy rendered card",
        ),
        ("issue_goal_ref", &format!("goal://issues/{issue}")),
        ("sprint_goal_ref", "unknown"),
        ("goal_metrics_rollup_ref", "unknown"),
        ("initial_pvf_lane", "needs_planning_lane_assignment"),
        ("planned_pvf_lane", "needs_planning_lane_assignment"),
        ("planned_pvf_lane_source", "legacy_import_default"),
        ("expected_runtime_class", "unknown"),
        ("estimate_elapsed_seconds", "unknown"),
        ("estimate_total_tokens", "unknown"),
        ("estimate_validation_seconds", "unknown"),
        ("issue_goal_token_budget", "unknown"),
        ("variance_threshold_percent", "10"),
        ("estimate_confidence", "unknown"),
        ("estimate_data_source", "unknown"),
        ("estimate_source_ref", "unknown"),
        ("timestamp", "legacy_import_unknown_timestamp"),
    ] {
        values.insert(key.to_string(), value.to_string());
        defaulted.push(key.to_string());
    }

    Ok((values, defaulted))
}

fn extract_legacy_sor_values(rendered: &str) -> Result<(BTreeMap<String, String>, Vec<String>)> {
    let mut values = BTreeMap::new();
    for (prefix, key) in [
        ("Task ID: ", "task_id"),
        ("Run ID: ", "run_id"),
        ("Version: ", "version"),
        ("Title: ", "title"),
        ("Branch: ", "branch"),
        ("Card Status: ", "card_status"),
        ("Status: ", "status"),
    ] {
        if let Some(value) = rendered
            .lines()
            .find_map(|line| line.strip_prefix(prefix).map(str::trim))
        {
            values.insert(key.to_string(), value.to_string());
        }
    }
    for key in [
        "task_id",
        "run_id",
        "version",
        "title",
        "branch",
        "card_status",
        "status",
    ] {
        ensure!(values.contains_key(key), "legacy sor import requires {key}");
    }
    let slug = rendered
        .lines()
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .ok_or_else(|| anyhow!("legacy sor import requires top-level heading"))?;
    values.insert("slug".to_string(), slug.to_string());
    let issue = extract_issue_number(
        values
            .get("task_id")
            .map(String::as_str)
            .unwrap_or_default(),
    )
    .ok_or_else(|| anyhow!("legacy sor import requires issue-like task_id"))?;
    values.insert("issue".to_string(), issue.clone());
    values.insert("issue_padded".to_string(), issue);
    values.insert(
        "issue_url".to_string(),
        format!(
            "https://github.com/danielbaustin/agent-design-language/issues/{}",
            values["issue"]
        ),
    );
    values.insert(
        "source_issue_prompt".to_string(),
        "not recorded in legacy rendered card".to_string(),
    );
    if let Some(summary) = markdown_section_body_local(rendered, "Summary") {
        let summary = summary.trim();
        if !summary.is_empty() {
            values.insert("summary".to_string(), summary.to_string());
        }
    }
    values.insert(
        "deliverables".to_string(),
        markdown_section_body_local(rendered, "Artifacts produced")
            .map(|body| body.trim().to_string())
            .filter(|body| !body.is_empty())
            .ok_or_else(|| anyhow!("legacy sor import requires Artifacts produced section"))?,
    );
    values.insert(
        "validation_plan".to_string(),
        markdown_section_body_local(rendered, "Validation")
            .map(|body| body.trim().to_string())
            .filter(|body| !body.is_empty())
            .ok_or_else(|| anyhow!("legacy sor import requires Validation section"))?,
    );
    if let Some(vpp_card) = rendered
        .lines()
        .find_map(extract_legacy_sor_validation_planning_prompt)
    {
        values.insert("vpp_card".to_string(), vpp_card);
    }

    let mut defaulted = Vec::new();
    for (key, value) in [
        ("output_card", "not recorded in legacy rendered card"),
        (
            "branch_action",
            "Legacy rendered SOR predates explicit branch_action tracking.",
        ),
        ("initial_pvf_lane", "needs_planning_lane_assignment"),
        ("planned_pvf_lane", "needs_planning_lane_assignment"),
        ("final_pvf_lane", "not_recorded_yet"),
        (
            "lane_change_reason",
            "Legacy rendered SOR predates explicit PVF lane tracking.",
        ),
        ("expected_runtime_class", "unknown"),
        ("estimate_elapsed_seconds", "unknown"),
        ("actual_elapsed_seconds", "unknown"),
        ("actual_active_work_seconds", "unknown"),
        ("estimate_total_tokens", "unknown"),
        ("actual_total_tokens", "unknown"),
        ("estimate_validation_seconds", "unknown"),
        ("actual_validation_seconds", "unknown"),
        ("budget_source", "unknown"),
        ("actual_pr_wait_seconds", "unknown"),
        ("actual_ci_wait_seconds", "unknown"),
        ("actual_metrics_data_source", "unknown"),
        ("actual_metrics_source_ref", "unknown"),
        ("actual_metrics_confidence", "unknown"),
        ("estimate_error_percent", "unknown"),
        ("completion_state", "unknown"),
        ("variance_analysis_required", "not_applicable"),
        ("variance_analysis_completed", "not_applicable"),
        ("variance_category", "not_applicable"),
        (
            "variance_note",
            "Legacy rendered SOR predates explicit variance-analysis tracking.",
        ),
        ("timestamp", "legacy_import_unknown_timestamp"),
    ] {
        values.insert(key.to_string(), value.to_string());
        defaulted.push(key.to_string());
    }

    Ok((values, defaulted))
}

fn ensure_legacy_sor_is_representable_in_active_template(
    values: &BTreeMap<String, String>,
) -> Result<()> {
    let status = values.get("status").map(String::as_str).unwrap_or_default();
    ensure!(
        status == "NOT_STARTED",
        "legacy sor import only supports bootstrap scaffolds; non-bootstrap legacy sor cards are not representable in the active template set"
    );
    ensure!(
        values
            .get("vpp_card")
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false),
        "legacy sor import requires Validation planning prompt line to infer vpp_card"
    );
    Ok(())
}

fn extract_legacy_sor_validation_planning_prompt(line: &str) -> Option<String> {
    let value = line
        .trim()
        .strip_prefix("- Validation planning prompt:")?
        .trim();
    let value = value
        .strip_prefix('`')
        .and_then(|value| value.strip_suffix('`'))
        .unwrap_or(value)
        .trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn scalar_to_import_string(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}

fn normalize_legacy_spp_activation_state(value: &str) -> String {
    match value {
        "design_time_ready" | "ready_for_execution" => "ready".to_string(),
        "ready_for_execution_binding" | "active" => "approved".to_string(),
        other => other.to_string(),
    }
}

fn mapping_required_scalar(mapping: &serde_yaml::Mapping, key: &str) -> Result<String> {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(scalar_to_import_string)
        .ok_or_else(|| anyhow!("legacy import requires scalar field {key}"))
}

fn legacy_source_ref(mapping: &serde_yaml::Mapping, wanted_kind: &str) -> Option<String> {
    mapping
        .get(Value::String("source_refs".to_string()))
        .and_then(Value::as_sequence)
        .and_then(|items| {
            items.iter().find_map(|item| {
                let item = item.as_mapping()?;
                let kind = item
                    .get(Value::String("kind".to_string()))
                    .and_then(Value::as_str)?;
                if kind != wanted_kind {
                    return None;
                }
                item.get(Value::String("ref".to_string()))
                    .and_then(Value::as_str)
                    .map(str::to_string)
            })
        })
}

fn yaml_sequence_summary(value: Option<&Value>) -> Option<String> {
    value.and_then(Value::as_sequence).map(|items| {
        let values = items
            .iter()
            .filter_map(scalar_to_import_string)
            .collect::<Vec<_>>();
        if values.is_empty() {
            "not recorded in legacy rendered card".to_string()
        } else {
            values.join("; ")
        }
    })
}

fn split_front_matter_local(text: &str) -> Result<(String, String)> {
    let first = text.lines().next().unwrap_or_default();
    ensure!(first.trim() == "---", "missing YAML front matter opener");
    let all = text.lines().collect::<Vec<_>>();
    let close = all
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.trim() == "---")
        .map(|(idx, _)| idx)
        .ok_or_else(|| anyhow!("missing YAML front matter closer"))?;
    let fm = all[1..close].join("\n");
    let body = all[close + 1..].join("\n");
    Ok((fm, body))
}

fn markdown_section_body_local(text: &str, heading: &str) -> Option<String> {
    let mut in_section = false;
    let mut out = Vec::new();
    for line in text.lines() {
        if !in_section {
            if line.trim_end() == format!("## {heading}") {
                in_section = true;
            }
            continue;
        }
        if line.starts_with("## ") {
            break;
        }
        out.push(line.to_string());
    }
    if in_section {
        Some(out.join("\n"))
    } else {
        None
    }
}

fn detect_rendered_card_template_set(rendered: &str, kind: PromptCardKind) -> Option<String> {
    let prefix = "Canonical Template Source: `docs/templates/prompts/";
    let suffix = format!("/{}.md`", kind.key());
    rendered.lines().find_map(|line| {
        let rest = line.trim().strip_prefix(prefix)?;
        let template_set = rest.strip_suffix(&suffix)?;
        if template_set.is_empty() {
            None
        } else {
            Some(template_set.to_string())
        }
    })
}

fn populate_unrepresented_required_import_fields(
    card: &PromptCardForm,
    values: &mut BTreeMap<String, String>,
) -> Result<Vec<String>> {
    let issue = derive_import_issue(values).unwrap_or_else(|| "1".to_string());
    let mut unrepresented = Vec::new();
    for field in &card.fields {
        if !field.required || values.contains_key(field.key) {
            continue;
        }
        values.insert(
            field.key.to_string(),
            unrepresented_import_value(field.key, &issue),
        );
        unrepresented.push(field.key.to_string());
    }
    Ok(unrepresented)
}

fn derive_import_issue(values: &BTreeMap<String, String>) -> Option<String> {
    for key in ["issue", "issue_padded"] {
        if let Some(value) = values.get(key).map(|value| value.trim()) {
            if value.parse::<u32>().is_ok() {
                return Some(value.to_string());
            }
        }
    }
    for key in ["task_id", "run_id", "issue_url", "source_issue_prompt"] {
        if let Some(issue) = values
            .get(key)
            .and_then(|value| extract_issue_number(value))
        {
            return Some(issue);
        }
    }
    None
}

fn extract_issue_number(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut idx = 0usize;
    while idx < bytes.len() {
        if bytes[idx].is_ascii_digit() {
            let start = idx;
            while idx < bytes.len() && bytes[idx].is_ascii_digit() {
                idx += 1;
            }
            return Some(value[start..idx].to_string());
        }
        idx += 1;
    }
    None
}

fn unrepresented_import_value(key: &str, issue: &str) -> String {
    match key {
        "issue" | "issue_padded" => issue.to_string(),
        "task_id" | "run_id" => format!("issue-{issue}"),
        "version" => "v0.0.0-imported".to_string(),
        "slug" => "imported-rendered-prompt-card".to_string(),
        "title" => "Imported rendered prompt card".to_string(),
        "branch" => "not bound yet".to_string(),
        "issue_url" => {
            format!("https://github.com/danielbaustin/agent-design-language/issues/{issue}")
        }
        "source_issue_prompt" => {
            format!(".adl/imported/issue-{issue}-source-issue-prompt.md")
        }
        "card_status" => "draft".to_string(),
        "estimate_elapsed_seconds"
        | "actual_elapsed_seconds"
        | "actual_active_work_seconds"
        | "estimate_total_tokens"
        | "actual_total_tokens"
        | "estimate_validation_seconds"
        | "actual_validation_seconds"
        | "actual_pr_wait_seconds"
        | "actual_ci_wait_seconds"
        | "actual_metrics_data_source"
        | "actual_metrics_source_ref"
        | "actual_metrics_confidence"
        | "estimate_error_percent"
        | "expected_runtime_class"
        | "completion_state" => "unknown".to_string(),
        "variance_analysis_required" | "variance_analysis_completed" | "variance_category" => {
            "not_applicable".to_string()
        }
        _ => "UNREPRESENTED_IN_RENDERED_CARD".to_string(),
    }
}

fn extract_template_values(
    card: &PromptCardForm,
    template: &str,
    rendered: &str,
) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    let mut template_pos = 0usize;
    let mut rendered_pos = 0usize;

    while let Some(token) = next_placeholder_token(template, template_pos) {
        let literal = &template[template_pos..token.start];
        ensure!(
            rendered[rendered_pos..].starts_with(literal),
            "{} rendered card cannot be represented by active template near byte {}",
            card.key,
            rendered_pos
        );
        rendered_pos += literal.len();

        let next_template_pos = token.end;
        let next_literal = next_placeholder_token(template, next_template_pos)
            .map(|next| &template[next_template_pos..next.start])
            .unwrap_or(&template[next_template_pos..]);
        ensure!(
            !next_literal.is_empty(),
            "{} template has adjacent placeholders near byte {}; import would be ambiguous",
            card.key,
            token.start
        );
        let Some(next_literal_offset) = rendered[rendered_pos..].find(next_literal) else {
            bail!(
                "{} rendered card cannot find post-placeholder literal for {} near byte {}",
                card.key,
                token.key,
                rendered_pos
            );
        };
        let value = rendered[rendered_pos..rendered_pos + next_literal_offset].to_string();
        if let Some(existing) = out.get(token.key) {
            ensure!(
                existing == &value,
                "{} repeated placeholder {} resolved to different values",
                card.key,
                token.key
            );
        } else {
            out.insert(token.key.to_string(), value);
        }
        rendered_pos += next_literal_offset;
        template_pos = token.end;
    }

    let tail = &template[template_pos..];
    ensure!(
        rendered[rendered_pos..] == *tail,
        "{} rendered card has trailing content that does not match active template",
        card.key
    );
    Ok(out)
}

#[derive(Debug, Clone, Copy)]
struct PlaceholderToken<'a> {
    key: &'a str,
    start: usize,
    end: usize,
}

fn next_placeholder_token(text: &str, from: usize) -> Option<PlaceholderToken<'_>> {
    let haystack = &text[from..];
    PLACEHOLDERS
        .iter()
        .flat_map(|key| {
            let legacy = format!("<{key}>");
            let curly = format!("{{{{{key}}}}}");
            [
                haystack.find(&legacy).map(|offset| PlaceholderToken {
                    key,
                    start: from + offset,
                    end: from + offset + legacy.len(),
                }),
                haystack.find(&curly).map(|offset| PlaceholderToken {
                    key,
                    start: from + offset,
                    end: from + offset + curly.len(),
                }),
            ]
        })
        .flatten()
        .min_by_key(|token| token.start)
}

pub fn validate_rendered_card_structure_file(
    repo_root: &Path,
    kind: PromptCardKind,
    rendered_path: &Path,
) -> Result<()> {
    validate_rendered_card_structure_file_for_template_set(repo_root, kind, rendered_path, None)
}

pub fn validate_rendered_card_structure_file_for_template_set(
    repo_root: &Path,
    kind: PromptCardKind,
    rendered_path: &Path,
    template_set_override: Option<&str>,
) -> Result<()> {
    let model = load_editor_model_for_template_set(repo_root, template_set_override)?;
    let card = card_model(&model, kind)?;
    let rendered = fs::read_to_string(rendered_path)
        .with_context(|| format!("failed to read rendered card {}", rendered_path.display()))?;
    validate_rendered_card_structure_from_repo(repo_root, card, &rendered)
}

pub fn validate_rendered_card_structure(card: &PromptCardForm, rendered: &str) -> Result<()> {
    structure::validate_rendered_card_structure(card, rendered)
}

pub fn write_all_structure_schemas(repo_root: &Path, out_dir: &Path) -> Result<()> {
    write_all_structure_schemas_for_template_set(repo_root, out_dir, None)
}

pub fn write_all_structure_schemas_for_template_set(
    repo_root: &Path,
    out_dir: &Path,
    template_set_override: Option<&str>,
) -> Result<()> {
    let model = load_editor_model_for_template_set(repo_root, template_set_override)?;
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;
    for card in &model.cards {
        let schema = build_structure_schema(&model.template_set, card)?;
        let text = serde_json::to_string_pretty(&schema)?;
        fs::write(
            out_dir.join(format!("{}.structure.json", card.key)),
            format!("{text}\n"),
        )
        .with_context(|| format!("failed to write {} structure schema", card.key))?;
    }
    Ok(())
}

pub fn validate_structure_schema_files(repo_root: &Path) -> Result<()> {
    validate_structure_schema_files_for_template_set(repo_root, None)
}

pub fn validate_structure_schema_files_for_template_set(
    repo_root: &Path,
    template_set_override: Option<&str>,
) -> Result<()> {
    let model = load_editor_model_for_template_set(repo_root, template_set_override)?;
    for card in &model.cards {
        let expected = build_structure_schema(&model.template_set, card)?;
        let actual = load_structure_schema(repo_root, card)?;
        ensure!(
            actual == expected,
            "{} structure schema does not match active template extraction",
            card.key
        );
    }
    Ok(())
}

pub fn render_all_cards_from_values_dir(
    repo_root: &Path,
    values_dir: &Path,
    out_dir: &Path,
    template_set_override: Option<&str>,
) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;
    let template_set = template_set_override
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| {
            active_prompt_template_set(repo_root).unwrap_or_else(|_| "1.0.2".to_string())
        });
    for kind in PromptCardKind::all_for_template_set(&template_set) {
        let values_path = values_dir.join(format!("{}.values.yaml", kind.key()));
        if template_set_override.is_some() {
            let values_template_set = template_set_from_values_file(&values_path)?;
            ensure!(
                values_template_set == template_set,
                "values file {} declares template_set {}; expected {}",
                values_path.display(),
                values_template_set,
                template_set
            );
        }
        let text = render_card_from_values_file(repo_root, kind, &values_path)?;
        fs::write(out_dir.join(kind.output_file()), text)
            .with_context(|| format!("failed to write {}", kind.output_file()))?;
    }
    Ok(())
}

fn card_model(model: &PromptEditorModel, kind: PromptCardKind) -> Result<&PromptCardForm> {
    model
        .cards
        .iter()
        .find(|card| card.kind == kind)
        .ok_or_else(|| anyhow!("missing card model for {}", kind.key()))
}

pub fn render_all_sample_cards(repo_root: &Path, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;
    let template_set = active_prompt_template_set(repo_root)?;
    for kind in PromptCardKind::all_for_template_set(&template_set) {
        let text = render_sample_card(repo_root, kind)?;
        fs::write(out_dir.join(kind.output_file()), text)
            .with_context(|| format!("failed to write {}", kind.output_file()))?;
    }
    Ok(())
}

pub fn write_all_sample_values(repo_root: &Path, out_dir: &Path) -> Result<()> {
    write_all_sample_values_for_template_set(repo_root, out_dir, None)
}

pub fn write_all_sample_values_for_template_set(
    repo_root: &Path,
    out_dir: &Path,
    template_set_override: Option<&str>,
) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;
    let model = load_editor_model_for_template_set(repo_root, template_set_override)?;
    for kind in PromptCardKind::all_for_template_set(&model.template_set) {
        let text = sample_values_document(kind, &model.template_set);
        fs::write(out_dir.join(format!("{}.values.yaml", kind.key())), text)
            .with_context(|| format!("failed to write {} values", kind.key()))?;
    }
    Ok(())
}

pub fn write_editor_model_js(repo_root: &Path, out_path: &Path) -> Result<()> {
    let model = load_editor_model(repo_root)?;
    let json = serde_json::to_string_pretty(&model)?;
    let js = format!(
        "// Generated by `adl tooling csdlc-prompt-editor --emit-model-js`.\nwindow.CSDLC_PROMPT_EDITOR_MODEL = {};\n",
        json
    );
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(out_path, js).with_context(|| format!("failed to write {}", out_path.display()))
}

pub fn validate_values(card: &PromptCardForm, values: &BTreeMap<String, String>) -> Result<()> {
    for field in &card.fields {
        let value = values
            .get(field.key)
            .map(|value| value.trim())
            .unwrap_or_default();
        ensure!(
            !field.required || !value.is_empty(),
            "{}.{} is required",
            card.key,
            field.key
        );
        if !field.enum_values.is_empty() && !value.is_empty() {
            ensure!(
                field.enum_values.iter().any(|allowed| allowed == &value),
                "{}.{} must be one of: {}",
                card.key,
                field.key,
                field.enum_values.join(", ")
            );
        }
    }

    let issue = values.get("issue").map(String::as_str).unwrap_or_default();
    ensure!(
        issue.parse::<u32>().is_ok(),
        "{}.issue must be a positive integer",
        card.key
    );
    let version = values
        .get("version")
        .map(String::as_str)
        .unwrap_or_default();
    ensure!(
        version.starts_with('v') && version.chars().any(|ch| ch == '.'),
        "{}.version must look like v0.91.3",
        card.key
    );
    let card_status = values
        .get("card_status")
        .map(String::as_str)
        .unwrap_or_default();
    ensure!(
        CARD_STATUS_VALUES.contains(&card_status),
        "{}.card_status must be one of: {}",
        card.key,
        CARD_STATUS_VALUES.join(", ")
    );
    match card.kind {
        PromptCardKind::Spp => validate_spp_values(card, values)?,
        PromptCardKind::Vpp => validate_vpp_values(card, values)?,
        PromptCardKind::Sor => validate_sor_values(card, values)?,
        _ => {}
    }
    Ok(())
}

fn validate_unknown_or_positive_int_value(field: &str, actual: &str) -> Result<()> {
    if actual == "unknown" {
        return Ok(());
    }
    actual.parse::<u64>().map_err(|_| {
        anyhow!("{field} must be `unknown` or a positive integer; actual: {actual}")
    })?;
    ensure!(
        actual != "0",
        "{field} must be greater than zero when recorded"
    );
    Ok(())
}

fn validate_unknown_or_nonnegative_int_value(field: &str, actual: &str) -> Result<()> {
    if actual == "unknown" {
        return Ok(());
    }
    actual.parse::<u64>().map_err(|_| {
        anyhow!("{field} must be `unknown` or a non-negative integer; actual: {actual}")
    })?;
    Ok(())
}

fn validate_reference_or_unknown_value(field: &str, actual: &str) -> Result<()> {
    if actual == "unknown" {
        return Ok(());
    }
    ensure!(
        valid_reference_value(actual),
        "{field} must be `unknown` or a repo-relative reference/URL; actual: {actual}"
    );
    Ok(())
}

fn valid_reference_value(value: &str) -> bool {
    value.starts_with("http://")
        || value.starts_with("https://")
        || value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '/' | '-'))
}

fn validate_spp_values(card: &PromptCardForm, values: &BTreeMap<String, String>) -> Result<()> {
    for key in [
        "estimate_elapsed_seconds",
        "estimate_total_tokens",
        "estimate_validation_seconds",
        "issue_goal_token_budget",
        "variance_threshold_percent",
    ] {
        if let Some(value) = values.get(key) {
            validate_unknown_or_positive_int_value(&format!("{}.{}", card.key, key), value)?;
        }
    }
    if let Some(value) = values.get("estimate_data_source") {
        let source_ref = values
            .get("estimate_source_ref")
            .map(String::as_str)
            .unwrap_or("unknown");
        validate_reference_or_unknown_value("spp.estimate_source_ref", source_ref)?;
        if value == "derived_sprint_state" {
            ensure!(
                source_ref != "unknown",
                "spp.estimate_source_ref is required when spp.estimate_data_source is `derived_sprint_state`"
            );
        }
        if value == "unknown" {
            ensure!(
                source_ref == "unknown",
                "spp.estimate_source_ref must remain `unknown` when spp.estimate_data_source is `unknown`"
            );
        }
    }
    if let Some(value) = values.get("estimate_confidence") {
        if value != "unknown" {
            let source = values
                .get("estimate_data_source")
                .map(String::as_str)
                .unwrap_or("unknown");
            ensure!(
                source != "unknown",
                "spp.estimate_confidence cannot be set when spp.estimate_data_source is `unknown`"
            );
        }
    }
    Ok(())
}

fn validate_vpp_values(card: &PromptCardForm, values: &BTreeMap<String, String>) -> Result<()> {
    for key in ["planned_validation_seconds", "planned_validation_tokens"] {
        if let Some(value) = values.get(key) {
            validate_unknown_or_positive_int_value(&format!("{}.{}", card.key, key), value)?;
        }
    }
    Ok(())
}

fn validate_sor_values(card: &PromptCardForm, values: &BTreeMap<String, String>) -> Result<()> {
    for key in [
        "estimate_elapsed_seconds",
        "actual_elapsed_seconds",
        "actual_active_work_seconds",
        "estimate_total_tokens",
        "actual_total_tokens",
        "estimate_validation_seconds",
        "actual_validation_seconds",
        "actual_pr_wait_seconds",
        "actual_ci_wait_seconds",
    ] {
        if let Some(value) = values.get(key) {
            validate_unknown_or_positive_int_value(&format!("{}.{}", card.key, key), value)?;
        }
    }
    if let Some(value) = values.get("estimate_error_percent") {
        validate_unknown_or_nonnegative_int_value("sor.estimate_error_percent", value)?;
    }
    if let Some(value) = values.get("actual_metrics_data_source") {
        let source_ref = values
            .get("actual_metrics_source_ref")
            .map(String::as_str)
            .unwrap_or("unknown");
        validate_reference_or_unknown_value("sor.actual_metrics_source_ref", source_ref)?;
        if value == "codex_goal_tool" || value == "derived_sprint_state" {
            ensure!(
                source_ref != "unknown",
                "sor.actual_metrics_source_ref is required when sor.actual_metrics_data_source is `{value}`"
            );
        }
        if value == "unknown" {
            ensure!(
                source_ref == "unknown",
                "sor.actual_metrics_source_ref must remain `unknown` when sor.actual_metrics_data_source is `unknown`"
            );
        }
    }
    if let Some(value) = values.get("actual_metrics_confidence") {
        if value != "unknown" {
            let source = values
                .get("actual_metrics_data_source")
                .map(String::as_str)
                .unwrap_or("unknown");
            ensure!(
                source != "unknown",
                "sor.actual_metrics_confidence cannot be set when sor.actual_metrics_data_source is `unknown`"
            );
        }
    }
    let variance_required = values
        .get("variance_analysis_required")
        .map(String::as_str)
        .unwrap_or("not_applicable");
    let variance_completed = values
        .get("variance_analysis_completed")
        .map(String::as_str)
        .unwrap_or("not_applicable");
    let variance_category = values
        .get("variance_category")
        .map(String::as_str)
        .unwrap_or("not_applicable");
    let variance_note = values
        .get("variance_note")
        .map(String::as_str)
        .unwrap_or("");
    let any_known_pair_exceeds_threshold = [
        metric_pair_exceeds_variance_threshold(
            values
                .get("estimate_elapsed_seconds")
                .map(String::as_str)
                .unwrap_or("unknown"),
            values
                .get("actual_elapsed_seconds")
                .map(String::as_str)
                .unwrap_or("unknown"),
        ),
        metric_pair_exceeds_variance_threshold(
            values
                .get("estimate_total_tokens")
                .map(String::as_str)
                .unwrap_or("unknown"),
            values
                .get("actual_total_tokens")
                .map(String::as_str)
                .unwrap_or("unknown"),
        ),
        metric_pair_exceeds_variance_threshold(
            values
                .get("estimate_validation_seconds")
                .map(String::as_str)
                .unwrap_or("unknown"),
            values
                .get("actual_validation_seconds")
                .map(String::as_str)
                .unwrap_or("unknown"),
        ),
    ]
    .into_iter()
    .flatten()
    .any(|exceeds| exceeds);
    if any_known_pair_exceeds_threshold {
        ensure!(
            variance_required == "yes",
            "sor.variance_analysis_required must be `yes` when any estimated/actual metric pair differs by more than 10 percent"
        );
    }
    if variance_required == "yes" {
        ensure!(
            variance_completed != "not_applicable",
            "sor.variance_analysis_completed cannot be `not_applicable` when sor.variance_analysis_required is `yes`"
        );
        ensure!(
            variance_category != "not_applicable",
            "sor.variance_category cannot be `not_applicable` when sor.variance_analysis_required is `yes`"
        );
        ensure!(
            !variance_note.trim().is_empty() && variance_note != "not_applicable",
            "sor.variance_note must be non-empty when sor.variance_analysis_required is `yes`"
        );
    } else {
        ensure!(
            variance_completed != "yes",
            "sor.variance_analysis_completed cannot be `yes` when sor.variance_analysis_required is not `yes`"
        );
        ensure!(
            variance_category == "not_applicable",
            "sor.variance_category must remain `not_applicable` when sor.variance_analysis_required is not `yes`"
        );
    }
    Ok(())
}

fn metric_pair_exceeds_variance_threshold(estimated: &str, actual: &str) -> Option<bool> {
    if estimated == "unknown" || actual == "unknown" {
        return None;
    }
    let estimated = estimated.parse::<u64>().ok()?;
    let actual = actual.parse::<u64>().ok()?;
    let diff = estimated.abs_diff(actual);
    Some(diff.saturating_mul(100) > estimated.saturating_mul(10))
}

pub fn render_template(template: &str, values: &BTreeMap<String, String>) -> Result<String> {
    let mut rendered = template.to_string();
    for key in PLACEHOLDERS {
        let legacy = format!("<{key}>");
        let curly = format!("{{{{{key}}}}}");
        if rendered.contains(&legacy) || rendered.contains(&curly) {
            let value = values
                .get(*key)
                .ok_or_else(|| anyhow!("missing template value: {key}"))?;
            rendered = rendered.replace(&legacy, value);
            rendered = rendered.replace(&curly, value);
        }
    }
    if let Some(idx) = unresolved_placeholder_offset(&rendered) {
        bail!("rendered card contains unresolved prompt-template placeholder near byte {idx}");
    }
    if let Some(idx) = unresolved_curly_placeholder_offset(&rendered) {
        bail!("rendered card contains unresolved prompt-template placeholder near byte {idx}");
    }
    Ok(rendered)
}

pub fn sample_values() -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    let issue = "1374";
    let slug = "csdlc-prompt-editor-sample";
    let version = "v0.91.3";
    let task_id = format!("issue-{issue}");
    for (key, value) in [
        ("card_status", "ready"),
        ("issue", issue),
        ("issue_padded", issue),
        ("task_id", &task_id),
        ("run_id", &task_id),
        ("version", version),
        ("slug", slug),
        ("title", "[v0.91.3][tools] C-SDLC prompt editor sample"),
        ("branch", "codex/1374-csdlc-prompt-editor-sample"),
        (
            "issue_url",
            "https://github.com/danielbaustin/agent-design-language/issues/1374",
        ),
        (
            "source_issue_prompt",
            ".adl/v0.91.3/bodies/issue-1374-csdlc-prompt-editor-sample.md",
        ),
        ("docs_context", "docs/tooling/csdlc-prompt-editor/README.md"),
        (
            "output_card",
            ".adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/sor.md",
        ),
        (
            "stp_card",
            ".adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/stp.md",
        ),
        (
            "sip_card",
            ".adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/sip.md",
        ),
        (
            "spp_card",
            ".adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/spp.md",
        ),
        (
            "vpp_card",
            ".adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/vpp.md",
        ),
        (
            "srp_card",
            ".adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/srp.md",
        ),
        (
            "sor_card",
            ".adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/sor.md",
        ),
        ("wp", "tools"),
        ("required_outcome_type", "combination"),
        ("demo_required", "true"),
        (
            "issue_graph_note",
            "Sample generated by the C-SDLC prompt editor.",
        ),
        ("summary", "Sample C-SDLC prompt editor card."),
        (
            "goal",
            "Demonstrate deterministic form-driven prompt-card generation.",
        ),
        (
            "required_outcome",
            "Generate validator-clean sample cards for the staged C-SDLC lifecycle.",
        ),
        (
            "deliverables",
            "- Rust-owned form metadata\n- Local browser editor\n- Validator-clean samples",
        ),
        (
            "acceptance_criteria",
            "- All generated samples validate\n- No unresolved placeholders remain",
        ),
        (
            "inputs",
            "- Active SemVer prompt-template registry\n- Human-entered issue metadata",
        ),
        (
            "repo_inputs",
            "- docs/templates/prompts/current.json\n- docs/templates/prompts/1.0.2/",
        ),
        ("dependencies", "- #3286 SemVer prompt templates"),
        (
            "target_files_surfaces",
            "- docs/tooling/csdlc-prompt-editor/\n- adl/src/csdlc_prompt_editor.rs",
        ),
        (
            "validation_plan",
            "- Run focused prompt-editor tests\n- Validate generated samples",
        ),
        (
            "demo_proof_requirements",
            "- Open the local editor page\n- Generate sample Markdown",
        ),
        ("non_goals", "- Full Jira replacement\n- Cloud persistence"),
        ("issue_graph_notes", "- This sample is local proof only."),
        (
            "notes_risks",
            "- Keep browser UI subordinate to Rust model truth.",
        ),
        (
            "tooling_notes",
            "- Generated from the active SemVer prompt template set.",
        ),
        (
            "target_files_surfaces_inline",
            "docs/tooling/csdlc-prompt-editor/ and adl/src/csdlc_prompt_editor.rs",
        ),
        (
            "non_goals_inline",
            "Full Jira replacement or cloud persistence.",
        ),
        (
            "plan_summary",
            "Render sample cards from Rust-owned form data and SemVer templates.",
        ),
        (
            "dependencies_inline",
            "#3286 SemVer prompt-template substrate.",
        ),
        (
            "repo_inputs_inline",
            "docs/templates/prompts/current.json and docs/templates/prompts/1.0.2/.",
        ),
        (
            "deliverables_inline",
            "Rust form model, local editor, validator-clean samples.",
        ),
        (
            "acceptance_criteria_inline",
            "All generated samples validate with structured-prompt validators.",
        ),
        (
            "risks_inline",
            "Browser UI could drift if JavaScript becomes semantic authority.",
        ),
        (
            "validation_plan_inline",
            "Run prompt-editor unit tests and generated-card validators.",
        ),
        (
            "notes_risks_inline",
            "Use Rust-generated metadata as the browser model.",
        ),
        ("status", "NOT_STARTED"),
        ("activation_state", "draft"),
        ("timestamp", "2026-05-23T00:00:00Z"),
        ("initial_pvf_lane", "prompt_template"),
        ("planned_pvf_lane", "prompt_template"),
        ("planned_pvf_lane_source", "matched_initial_issue_lane"),
        ("issue_goal_ref", "goal://issues/1374"),
        ("sprint_goal_ref", "goal://sprints/v0.91.3-sample"),
        (
            "goal_metrics_rollup_ref",
            ".adl/reviews/sample-goal-rollup.jsonl",
        ),
        (
            "lane_registry_path",
            "adl/config/validation_lane_selector.v0.91.6.json",
        ),
        ("lane_registry_template_set", "v0.91.6"),
        ("validation_runtime_class", "docs_only"),
        ("validation_resource_profile", "small"),
        ("validation_family", "prompt_template"),
        ("validation_size_split", "not_applicable"),
        ("expected_proof_cost", "low"),
        ("planned_validation_seconds", "unknown"),
        ("planned_validation_tokens", "unknown"),
        ("selected_lanes_inline", "docs_diff_check"),
        ("parallel_groups_inline", "docs_only"),
        (
            "validation_commands_inline",
            "bash adl/tools/test_prompt_template_workflow_integration.sh 1.0.3",
        ),
        (
            "failure_policy",
            "Do not collapse blocked, skipped, failed, or pending validation states into PASS.",
        ),
        ("estimated_elapsed_seconds", "unknown"),
        ("estimated_total_tokens", "unknown"),
        ("estimated_validation_seconds", "unknown"),
        ("expected_runtime_class", "unknown"),
        ("estimate_elapsed_seconds", "unknown"),
        ("estimate_total_tokens", "unknown"),
        ("estimate_validation_seconds", "unknown"),
        ("issue_goal_token_budget", "unknown"),
        ("variance_threshold_percent", "10"),
        ("estimate_confidence", "unknown"),
        ("estimate_data_source", "unknown"),
        ("estimate_source_ref", "unknown"),
        ("final_pvf_lane", "not_recorded_yet"),
        (
            "lane_change_reason",
            "No lane change recorded in the generated sample.",
        ),
        ("actual_elapsed_seconds", "unknown"),
        ("actual_active_work_seconds", "unknown"),
        ("actual_total_tokens", "unknown"),
        ("actual_validation_seconds", "unknown"),
        ("budget_source", "unknown"),
        ("actual_pr_wait_seconds", "unknown"),
        ("actual_ci_wait_seconds", "unknown"),
        ("actual_metrics_data_source", "unknown"),
        ("actual_metrics_source_ref", "unknown"),
        ("actual_metrics_confidence", "unknown"),
        ("estimate_error_percent", "unknown"),
        ("completion_state", "unknown"),
        ("variance_analysis_required", "not_applicable"),
        ("variance_analysis_completed", "not_applicable"),
        ("variance_category", "not_applicable"),
        (
            "variance_note",
            "No variance analysis is required in the generated sample.",
        ),
        ("execution_actor", "issue-wave bootstrap"),
        ("model", "not_applicable"),
        ("provider", "not_applicable"),
        ("start_time", "2026-05-23T00:00:00Z"),
        ("end_time", "2026-05-23T00:00:00Z"),
        (
            "tracked_implementation_artifacts",
            "not_applicable until execution begins",
        ),
        (
            "additional_proof_artifacts",
            "not_applicable until execution begins",
        ),
        (
            "actions_taken_line_1",
            "Opened the local issue bundle and wrote a truthful pre-run output scaffold.",
        ),
        (
            "actions_taken_line_2",
            "Preserved pre-run branch truth in generated sample content.",
        ),
        (
            "actions_taken_line_3",
            "Deferred implementation, proof capture, and release integration to the execution lifecycle and PR publication.",
        ),
        ("main_repo_paths_updated", "none"),
        (
            "worktree_only_paths_remaining",
            "no tracked implementation artifacts exist yet; execution-time proof surfaces will be established during implementation and PR publication",
        ),
        ("integration_state", "worktree_only"),
        ("verification_scope", "main_repo"),
        (
            "integration_method_used",
            "local ignored card-bundle scaffold write under the active checkout; tracked implementation artifacts do not exist yet",
        ),
        (
            "integration_verification_command",
            "`bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/sor.md`",
        ),
        (
            "integration_verification_effect",
            "Verified bootstrap SOR contract compliance for the local pre-run scaffold.",
        ),
        ("integration_result", "PASS"),
        (
            "validation_command",
            "`bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/sor.md`",
        ),
        (
            "validation_effect",
            "Verified bootstrap SOR contract compliance for the local output scaffold.",
        ),
        ("validation_result", "PASS"),
        ("verification_validation_status", "PASS"),
        (
            "verification_check_1",
            "bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/v0.91.3/tasks/issue-1374__csdlc-prompt-editor-sample/sor.md",
        ),
        ("verification_determinism_status", "NOT_RUN"),
        ("verification_replay_verified", "unknown"),
        ("verification_ordering_guarantees_verified", "unknown"),
        ("verification_security_privacy_status", "PARTIAL"),
        ("verification_secrets_leakage_detected", "false"),
        (
            "verification_prompt_or_tool_arg_leakage_detected",
            "false",
        ),
        ("verification_absolute_path_leakage_detected", "false"),
        ("verification_artifacts_status", "PASS"),
        ("verification_required_artifacts_present", "true"),
        ("verification_schema_changes_present", "false"),
        ("verification_schema_changes_approved", "not_applicable"),
        (
            "determinism_tests_executed",
            "not_run; bootstrap scaffold creation has not been replay-verified for this issue yet.",
        ),
        (
            "fixtures_or_scripts_used",
            "`adl/tools/pr.sh` issue-wave opening flow.",
        ),
        (
            "replay_verification",
            "not yet verified for this specific issue record.",
        ),
        (
            "ordering_guarantees",
            "not_applicable for a single-card bootstrap write.",
        ),
        (
            "artifact_stability_notes",
            "repository-relative paths only; execution-time proof artifacts are not expected yet.",
        ),
        (
            "secret_leakage_scan_performed",
            "limited content review only; no secrets were intentionally recorded in the scaffold.",
        ),
        (
            "prompt_tool_arg_redaction_verified",
            "not_applicable for bootstrap scaffold generation.",
        ),
        (
            "absolute_path_leakage_check",
            "repository-relative paths only in the scaffold.",
        ),
        (
            "sandbox_policy_invariants_preserved",
            "yes; local ignored issue-record path only.",
        ),
        ("trace_bundle_paths", "not_applicable until execution begins"),
        ("run_artifact_root", "not_applicable until execution begins"),
        ("replay_command", "not_run"),
        ("replay_result", "NOT_RUN"),
        (
            "primary_proof_surface",
            "this local pre-run SOR scaffold and its bootstrap validation result",
        ),
        (
            "required_artifacts_present",
            "local output card scaffold only; tracked implementation artifacts are not expected yet",
        ),
        ("artifact_schema_checks", "bootstrap SOR validator passed"),
        ("hash_byte_stability_checks", "not_run"),
        (
            "missing_optional_artifacts_rationale",
            "execution proofs, demos, and tracked outputs are intentionally absent before implementation begins",
        ),
        (
            "decision_or_deviation_1",
            "Issue-wave opening emits a truthful pre-run SOR scaffold instead of leaving raw template residue for later cleanup.",
        ),
        (
            "decision_or_deviation_2",
            "Integration state remains `worktree_only` until execution creates tracked artifacts or opens a PR.",
        ),
        (
            "follow_up_1",
            "Update this record during execution with actual actions, validations, proof surfaces, and integration truth.",
        ),
        (
            "follow_up_2",
            "Normalize this record to `pr_open`, `merged`, or `closed_no_pr` during finish/closeout as appropriate.",
        ),
        (
            "branch_action",
            "Preserved pre-run branch truth in generated sample content.",
        ),
        ("findings_status", "not_run"),
        ("recommended_outcome", "not_run"),
    ] {
        values.insert(key.to_string(), value.to_string());
    }
    values
}

fn sample_values_for_kind(kind: PromptCardKind) -> BTreeMap<String, String> {
    let mut values = sample_values();
    if kind == PromptCardKind::Spp {
        values.insert("status".to_string(), "draft".to_string());
        values.insert("activation_state".to_string(), "draft".to_string());
    }
    if kind == PromptCardKind::Vpp {
        values.insert("status".to_string(), "draft".to_string());
    }
    values
}

fn unresolved_placeholder_offset(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut idx = 0usize;
    while idx < bytes.len() {
        if bytes[idx] != b'<' {
            idx += 1;
            continue;
        }
        let start = idx;
        idx += 1;
        if idx >= bytes.len() || !bytes[idx].is_ascii_lowercase() {
            continue;
        }
        while idx < bytes.len()
            && (bytes[idx].is_ascii_lowercase()
                || bytes[idx].is_ascii_digit()
                || bytes[idx] == b'_')
        {
            idx += 1;
        }
        if idx < bytes.len() && bytes[idx] == b'>' {
            return Some(start);
        }
    }
    None
}

fn unresolved_curly_placeholder_offset(text: &str) -> Option<usize> {
    text.find("{{")
}

fn form_fields(kind: PromptCardKind) -> Vec<PromptField> {
    let mut fields = read_only_common_fields();
    match kind {
        PromptCardKind::Sip => {
            fields.push(read_only_textarea(
                "docs_context",
                "Docs Context",
                false,
                "System-supplied repo documentation context.",
            ));
            fields.push(textarea(
                "goal",
                "Goal",
                true,
                "Issue intent in operator-readable form.",
            ));
            fields.push(textarea(
                "required_outcome",
                "Required Outcome",
                true,
                "Concrete output expected from the issue.",
            ));
            fields.push(textarea(
                "acceptance_criteria",
                "Acceptance Criteria",
                true,
                "Reviewable conditions that prove the issue is complete.",
            ));
            fields.push(textarea(
                "inputs",
                "Inputs",
                true,
                "Issue-local source inputs and context surfaces.",
            ));
            fields.push(textarea(
                "target_files_surfaces",
                "Target Files / Surfaces",
                true,
                "Repo surfaces the issue is expected to touch.",
            ));
            fields.push(read_only_select(
                "required_outcome_type",
                "Required Outcome Type",
                true,
                "System-classified primary outcome type.",
                &["code", "docs", "tests", "demo", "combination"],
            ));
            fields.push(read_only_select(
                "demo_required",
                "Demo Required",
                true,
                "System-classified demo proof requirement.",
                &["true", "false"],
            ));
            fields.push(read_only_textarea(
                "validation_plan",
                "Validation Plan",
                true,
                "System-supplied baseline proof commands.",
            ));
            fields.push(textarea(
                "demo_proof_requirements",
                "Demo / Proof Requirements",
                true,
                "Human-visible demo or proof expectations.",
            ));
            fields.push(textarea(
                "non_goals",
                "Non-goals",
                true,
                "Explicit out-of-scope boundaries.",
            ));
            fields.push(textarea(
                "notes_risks",
                "Notes / Risks",
                true,
                "Known risks, constraints, or reviewer cautions.",
            ));
        }
        PromptCardKind::Stp => {
            fields.push(read_only_text(
                "wp",
                "WP / Area",
                true,
                "System-supplied milestone work-package or area label.",
            ));
            fields.push(textarea("summary", "Summary", true, "Short issue summary."));
            fields.push(textarea("goal", "Goal", true, "Concrete task intent."));
            fields.push(textarea(
                "required_outcome",
                "Required Outcome",
                true,
                "The outcome the task must produce.",
            ));
            fields.push(read_only_select(
                "required_outcome_type",
                "Required Outcome Type",
                true,
                "System-classified primary outcome type.",
                &["code", "docs", "tests", "demo", "combination"],
            ));
            fields.push(textarea(
                "deliverables",
                "Deliverables",
                true,
                "Concrete tracked outputs.",
            ));
            fields.push(textarea(
                "acceptance_criteria",
                "Acceptance Criteria",
                true,
                "Reviewable completion criteria.",
            ));
            fields.push(textarea(
                "repo_inputs",
                "Repo Inputs",
                true,
                "Named repo inputs.",
            ));
            fields.push(textarea(
                "dependencies",
                "Dependencies",
                true,
                "Issue or milestone dependencies.",
            ));
            fields.push(textarea(
                "target_files_surfaces",
                "Target Files / Surfaces",
                true,
                "Repo surfaces expected to change or be reviewed.",
            ));
            fields.push(textarea(
                "validation_plan",
                "Validation Plan",
                true,
                "Focused proof commands and review checks.",
            ));
            fields.push(read_only_select(
                "demo_required",
                "Demo Required",
                true,
                "System-classified demo proof requirement.",
                &["true", "false"],
            ));
            fields.push(textarea(
                "demo_proof_requirements",
                "Demo Expectations",
                true,
                "Human-visible demo or proof expectations.",
            ));
            fields.push(textarea(
                "non_goals",
                "Non-goals",
                true,
                "Explicit boundaries.",
            ));
            fields.push(textarea(
                "issue_graph_notes",
                "Issue-Graph Notes",
                true,
                "Issue graph routing, dependencies, or sequencing notes.",
            ));
            fields.push(textarea(
                "notes_risks",
                "Notes / Risks",
                true,
                "Known risks, constraints, or reviewer cautions.",
            ));
            fields.push(textarea(
                "tooling_notes",
                "Tooling Notes",
                true,
                "Tooling expectations or automation notes.",
            ));
        }
        PromptCardKind::Spp => {
            fields.push(select(
                "card_status",
                "Card Status",
                true,
                "SPP lifecycle card status.",
                CARD_STATUS_VALUES,
            ));
            fields.push(select(
                "status",
                "Status",
                true,
                "SPP frontmatter lifecycle status.",
                CARD_STATUS_VALUES,
            ));
            fields.push(select(
                "activation_state",
                "Activation State",
                true,
                "SPP execution-readiness lifecycle state.",
                CARD_STATUS_VALUES,
            ));
            fields.push(read_only_text(
                "initial_pvf_lane",
                "Initial PVF Lane",
                true,
                "System-supplied PVF lane from issue creation/bootstrap.",
            ));
            fields.push(text(
                "planned_pvf_lane",
                "Planned PVF Lane",
                true,
                "Planning-confirmed or planning-revised PVF lane.",
            ));
            fields.push(text(
                "planned_pvf_lane_source",
                "Planned PVF Lane Source",
                true,
                "Why the planned PVF lane was kept or changed during planning.",
            ));
            fields.push(text(
                "expected_runtime_class",
                "Expected Runtime Class",
                true,
                "Qualitative runtime posture for the issue, or `unknown`.",
            ));
            fields.push(text(
                "estimate_elapsed_seconds",
                "Estimated Elapsed Seconds",
                true,
                "Expected issue elapsed time in seconds, or `unknown`.",
            ));
            fields.push(text(
                "estimate_total_tokens",
                "Estimated Total Tokens",
                true,
                "Expected token consumption, or `unknown`.",
            ));
            fields.push(text(
                "estimate_validation_seconds",
                "Estimated Validation Seconds",
                true,
                "Expected validation time in seconds, or `unknown`.",
            ));
            fields.push(text(
                "issue_goal_token_budget",
                "Issue Goal Token Budget",
                true,
                "Bounded issue-goal token budget, or `unknown`.",
            ));
            fields.push(text(
                "variance_threshold_percent",
                "Variance Threshold Percent",
                true,
                "Percent threshold that requires variance analysis.",
            ));
            fields.push(select(
                "estimate_confidence",
                "Estimate Confidence",
                true,
                "Confidence in the planning estimates.",
                &["low", "medium", "high", "unknown"],
            ));
            fields.push(select(
                "estimate_data_source",
                "Estimate Data Source",
                true,
                "Where the estimate truth came from.",
                &["manual_entry", "derived_sprint_state", "unknown"],
            ));
            fields.push(text(
                "estimate_source_ref",
                "Estimate Source Ref",
                true,
                "Reference backing the estimate, or `unknown`.",
            ));
            fields.push(text(
                "issue_goal_ref",
                "Issue Goal Ref",
                true,
                "Issue-bound goal reference for metrics rollup.",
            ));
            fields.push(text(
                "sprint_goal_ref",
                "Sprint Goal Ref",
                true,
                "Sprint-bound goal reference for metrics rollup.",
            ));
            fields.push(text(
                "goal_metrics_rollup_ref",
                "Goal Metrics Rollup Ref",
                true,
                "Goal metrics rollup artifact or `unknown`.",
            ));
            fields.push(read_only_textarea(
                "stp_card",
                "STP Card",
                true,
                "System-supplied Structured Task Prompt path.",
            ));
            fields.push(read_only_textarea(
                "sip_card",
                "SIP Card",
                true,
                "System-supplied Structured Issue Prompt path.",
            ));
            fields.push(textarea(
                "plan_summary",
                "Plan Summary",
                true,
                "Issue-local operative plan.",
            ));
            fields.push(textarea(
                "dependencies_inline",
                "Dependencies",
                true,
                "Dependency truth before execution.",
            ));
            fields.push(textarea(
                "repo_inputs_inline",
                "Repo Inputs",
                true,
                "Repo inputs to inspect before editing.",
            ));
            fields.push(textarea(
                "target_files_surfaces_inline",
                "Target Files / Surfaces",
                true,
                "Issue-local surfaces the operative plan may touch.",
            ));
            fields.push(textarea(
                "deliverables_inline",
                "Deliverables",
                true,
                "Bounded deliverables for the execution plan.",
            ));
            fields.push(textarea(
                "acceptance_criteria_inline",
                "Acceptance Criteria",
                true,
                "Proof gates before proceeding.",
            ));
            fields.push(textarea(
                "risks_inline",
                "Risks",
                true,
                "Replan triggers and edge cases.",
            ));
            fields.push(textarea(
                "validation_plan_inline",
                "Test Strategy",
                true,
                "Focused proof gates for this plan.",
            ));
            fields.push(textarea(
                "non_goals_inline",
                "Out Of Scope",
                true,
                "Boundaries the operative plan must preserve.",
            ));
            fields.push(textarea(
                "notes_risks_inline",
                "Notes",
                true,
                "Issue-local notes for execution handoff.",
            ));
        }
        PromptCardKind::Vpp => {
            fields.push(select(
                "card_status",
                "Card Status",
                true,
                "VPP lifecycle card status.",
                CARD_STATUS_VALUES,
            ));
            fields.push(select(
                "status",
                "Status",
                true,
                "VPP lifecycle status.",
                &["draft", "ready", "reviewed", "approved"],
            ));
            fields.push(read_only_text(
                "initial_pvf_lane",
                "Initial PVF Lane",
                true,
                "System-supplied PVF lane from issue creation/bootstrap.",
            ));
            fields.push(text(
                "planned_pvf_lane",
                "Planned PVF Lane",
                true,
                "Validation-planning confirmed PVF lane.",
            ));
            fields.push(text(
                "lane_registry_path",
                "Lane Registry Path",
                true,
                "Registry backing the validation lane selection.",
            ));
            fields.push(text(
                "lane_registry_template_set",
                "Lane Registry Template Set",
                true,
                "Version label for the lane registry contract.",
            ));
            fields.push(text(
                "validation_runtime_class",
                "Validation Runtime Class",
                true,
                "Validation runtime class for this issue.",
            ));
            fields.push(text(
                "validation_resource_profile",
                "Validation Resource Profile",
                true,
                "Resource profile expected for this validation plan.",
            ));
            fields.push(text(
                "validation_family",
                "Validation Family",
                true,
                "Primary validation family for this issue, or `unknown`.",
            ));
            fields.push(select(
                "validation_size_split",
                "Validation Size Split",
                true,
                "Whether the plan expects small, large, mixed, or no validation split.",
                &[
                    "small_only",
                    "large_only",
                    "mixed",
                    "not_applicable",
                    "unknown",
                ],
            ));
            fields.push(text(
                "expected_proof_cost",
                "Expected Proof Cost",
                true,
                "Expected proof-cost classification.",
            ));
            fields.push(text(
                "planned_validation_seconds",
                "Planned Validation Seconds",
                true,
                "Expected validation elapsed time in seconds, or `unknown`.",
            ));
            fields.push(text(
                "planned_validation_tokens",
                "Planned Validation Tokens",
                true,
                "Expected validation token use, or `unknown`.",
            ));
            fields.push(text(
                "issue_goal_ref",
                "Issue Goal Ref",
                true,
                "Issue-bound goal reference for metrics rollup.",
            ));
            fields.push(text(
                "sprint_goal_ref",
                "Sprint Goal Ref",
                true,
                "Sprint-bound goal reference for rollup.",
            ));
            fields.push(text(
                "goal_metrics_rollup_ref",
                "Goal Metrics Rollup Ref",
                true,
                "Goal metrics rollup artifact or `unknown`.",
            ));
            fields.push(read_only_textarea(
                "stp_card",
                "STP Card",
                true,
                "System-supplied STP path.",
            ));
            fields.push(read_only_textarea(
                "sip_card",
                "SIP Card",
                true,
                "System-supplied SIP path.",
            ));
            fields.push(read_only_textarea(
                "spp_card",
                "SPP Card",
                true,
                "System-supplied SPP path.",
            ));
            fields.push(textarea(
                "plan_summary",
                "Validation Planning Summary",
                true,
                "Summary of the validation-planning intent.",
            ));
            fields.push(textarea(
                "selected_lanes_inline",
                "Selected Validation Lanes",
                true,
                "Selected validation lanes for this issue.",
            ));
            fields.push(textarea(
                "parallel_groups_inline",
                "Parallelization Plan",
                true,
                "Parallel groups or `none`.",
            ));
            fields.push(textarea(
                "validation_commands_inline",
                "Validation Commands",
                true,
                "Commands expected to prove the issue outcome.",
            ));
            fields.push(textarea(
                "failure_policy",
                "Failure Semantics",
                true,
                "How blocked, skipped, failed, and pending states must be represented.",
            ));
            fields.push(textarea(
                "notes_risks_inline",
                "Notes",
                true,
                "Validation-planning notes and edge cases.",
            ));
        }
        PromptCardKind::Srp => {
            fields.push(read_only_textarea(
                "stp_card",
                "STP Card",
                true,
                "System-supplied STP path.",
            ));
            fields.push(read_only_textarea(
                "sip_card",
                "SIP Card",
                true,
                "System-supplied SIP path.",
            ));
            fields.push(read_only_textarea(
                "spp_card",
                "SPP Card",
                true,
                "System-supplied SPP path.",
            ));
            fields.push(read_only_textarea(
                "vpp_card",
                "VPP Card",
                true,
                "System-supplied VPP path.",
            ));
            fields.push(read_only_textarea(
                "sor_card",
                "SOR Card",
                true,
                "System-supplied SOR path.",
            ));
            fields.push(textarea(
                "notes_risks",
                "Review Notes",
                true,
                "Review constraints and residual risks.",
            ));
            fields.push(select(
                "findings_status",
                "Findings Status",
                true,
                "Machine-readable final review findings status.",
                &["not_run", "findings_present", "no_findings"],
            ));
            fields.push(select(
                "recommended_outcome",
                "Recommended Outcome",
                true,
                "Machine-readable final review outcome.",
                &["not_run", "pass", "block", "needs_followup"],
            ));
        }
        PromptCardKind::Sor => {
            fields.push(read_only_textarea(
                "output_card",
                "Output Card",
                true,
                "System-supplied SOR output path.",
            ));
            fields.push(read_only_textarea(
                "branch_action",
                "Branch Action",
                true,
                "System-supplied branch-binding action record.",
            ));
            fields.push(select(
                "status",
                "Status",
                true,
                "Execution status.",
                &["NOT_STARTED", "IN_PROGRESS", "DONE", "FAILED"],
            ));
            fields.push(read_only_text(
                "initial_pvf_lane",
                "Initial PVF Lane",
                true,
                "System-supplied PVF lane from issue creation/bootstrap.",
            ));
            fields.push(read_only_text(
                "planned_pvf_lane",
                "Planned PVF Lane",
                true,
                "System-supplied planned PVF lane from SPP truth.",
            ));
            fields.push(text(
                "final_pvf_lane",
                "Final PVF Lane",
                true,
                "Final execution PVF lane, or `not_recorded_yet` during bootstrap.",
            ));
            fields.push(textarea(
                "lane_change_reason",
                "Lane Change Reason",
                true,
                "Reason the final PVF lane changed or stayed the same.",
            ));
            fields.push(text(
                "expected_runtime_class",
                "Expected Runtime Class",
                true,
                "Qualitative runtime posture planned for the issue, or `unknown`.",
            ));
            fields.push(text(
                "estimate_elapsed_seconds",
                "Estimated Elapsed Seconds",
                true,
                "Estimated issue elapsed time in seconds, or `unknown`.",
            ));
            fields.push(text(
                "actual_elapsed_seconds",
                "Actual Elapsed Seconds",
                true,
                "Measured issue elapsed time in seconds, or `unknown`.",
            ));
            fields.push(text(
                "actual_active_work_seconds",
                "Actual Active Work Seconds",
                true,
                "Measured active implementation time in seconds, or `unknown`.",
            ));
            fields.push(text(
                "estimate_total_tokens",
                "Estimated Total Tokens",
                true,
                "Estimated token consumption, or `unknown`.",
            ));
            fields.push(text(
                "actual_total_tokens",
                "Actual Total Tokens",
                true,
                "Measured token consumption, or `unknown`.",
            ));
            fields.push(text(
                "estimate_validation_seconds",
                "Estimated Validation Seconds",
                true,
                "Estimated validation time in seconds, or `unknown`.",
            ));
            fields.push(text(
                "actual_validation_seconds",
                "Actual Validation Seconds",
                true,
                "Measured validation time in seconds, or `unknown`.",
            ));
            fields.push(select(
                "budget_source",
                "Budget Source",
                true,
                "Where the execution-time budget truth came from.",
                &[
                    "issue_goal_budget",
                    "sprint_rollup",
                    "manual_entry",
                    "not_applicable",
                    "unknown",
                ],
            ));
            fields.push(text(
                "actual_pr_wait_seconds",
                "Actual PR Wait Seconds",
                true,
                "Measured PR wait time in seconds, or `unknown`.",
            ));
            fields.push(text(
                "actual_ci_wait_seconds",
                "Actual CI Wait Seconds",
                true,
                "Measured CI wait time in seconds, or `unknown`.",
            ));
            fields.push(select(
                "actual_metrics_data_source",
                "Actual Metrics Data Source",
                true,
                "Where the actual issue metrics came from.",
                &[
                    "codex_goal_tool",
                    "manual_entry",
                    "derived_sprint_state",
                    "unknown",
                ],
            ));
            fields.push(text(
                "actual_metrics_source_ref",
                "Actual Metrics Source Ref",
                true,
                "Reference backing the actual metrics, or `unknown`.",
            ));
            fields.push(select(
                "actual_metrics_confidence",
                "Actual Metrics Confidence",
                true,
                "Confidence in the actual metric values.",
                &["low", "medium", "high", "unknown"],
            ));
            fields.push(text(
                "estimate_error_percent",
                "Estimate Error Percent",
                true,
                "Difference between estimated and actual elapsed time, or `unknown`.",
            ));
            fields.push(select(
                "completion_state",
                "Completion State",
                true,
                "Truthful issue-local completion state.",
                &[
                    "completed",
                    "completed_with_follow_on",
                    "blocked",
                    "failed",
                    "deferred",
                    "cancelled",
                    "unknown",
                ],
            ));
            fields.push(select(
                "variance_analysis_required",
                "Variance Analysis Required",
                true,
                "Whether any known estimate/actual metric pair exceeded the 10 percent threshold.",
                &["not_applicable", "no", "yes"],
            ));
            fields.push(select(
                "variance_analysis_completed",
                "Variance Analysis Completed",
                true,
                "Whether the required variance analysis was completed.",
                &["not_applicable", "no", "yes"],
            ));
            fields.push(select(
                "variance_category",
                "Variance Category",
                true,
                "Primary category for a required variance analysis.",
                &[
                    "not_applicable",
                    "validation_misclassification",
                    "pr_wait",
                    "merge_conflict",
                    "tool_failure",
                    "unclear_scope",
                    "model_drift",
                    "human_wait",
                    "external_api_latency",
                    "overestimated_scope",
                ],
            ));
            fields.push(textarea(
                "variance_note",
                "Variance Note",
                true,
                "Short variance-analysis note when the threshold was exceeded.",
            ));
            fields.push(textarea(
                "summary",
                "Summary",
                true,
                "Outcome truth summary.",
            ));
            fields.push(textarea(
                "deliverables",
                "Artifacts Produced",
                true,
                "Produced artifacts or not-applicable rationale.",
            ));
            fields.push(textarea(
                "validation_plan",
                "Validation",
                true,
                "Exact validation commands and results.",
            ));
        }
    }
    fields
}

fn read_only_common_fields() -> Vec<PromptField> {
    vec![
        read_only_text("issue", "Issue Number", true, "GitHub issue number."),
        read_only_text(
            "version",
            "Milestone Version",
            true,
            "Milestone version such as v0.91.3.",
        ),
        read_only_text("slug", "Slug", true, "Normalized issue slug."),
        read_only_text("title", "Title", true, "Issue title."),
        read_only_text(
            "branch",
            "Branch",
            true,
            "Execution branch or `not bound yet` during pre-run.",
        ),
        read_only_text(
            "issue_url",
            "Issue URL",
            true,
            "Canonical GitHub issue URL.",
        ),
        read_only_text(
            "source_issue_prompt",
            "Source Issue Prompt",
            true,
            "Repo-relative source issue prompt.",
        ),
    ]
}

fn read_only_text(
    key: &'static str,
    label: &'static str,
    required: bool,
    help: &'static str,
) -> PromptField {
    PromptField {
        key,
        label,
        input: "text",
        required,
        editable: false,
        help,
        enum_values: Vec::new(),
    }
}

fn textarea(
    key: &'static str,
    label: &'static str,
    required: bool,
    help: &'static str,
) -> PromptField {
    PromptField {
        key,
        label,
        input: "textarea",
        required,
        editable: true,
        help,
        enum_values: Vec::new(),
    }
}

fn text(key: &'static str, label: &'static str, required: bool, help: &'static str) -> PromptField {
    PromptField {
        key,
        label,
        input: "text",
        required,
        editable: true,
        help,
        enum_values: Vec::new(),
    }
}

fn read_only_textarea(
    key: &'static str,
    label: &'static str,
    required: bool,
    help: &'static str,
) -> PromptField {
    PromptField {
        key,
        label,
        input: "textarea",
        required,
        editable: false,
        help,
        enum_values: Vec::new(),
    }
}

fn select(
    key: &'static str,
    label: &'static str,
    required: bool,
    help: &'static str,
    enum_values: &[&'static str],
) -> PromptField {
    PromptField {
        key,
        label,
        input: "select",
        required,
        editable: true,
        help,
        enum_values: enum_values.to_vec(),
    }
}

fn read_only_select(
    key: &'static str,
    label: &'static str,
    required: bool,
    help: &'static str,
    enum_values: &[&'static str],
) -> PromptField {
    PromptField {
        key,
        label,
        input: "select",
        required,
        editable: false,
        help,
        enum_values: enum_values.to_vec(),
    }
}

pub fn repo_root_from_arg(path: Option<PathBuf>) -> Result<PathBuf> {
    let root = path.unwrap_or(std::env::current_dir()?);
    if root.join("adl/Cargo.toml").is_file() {
        return Ok(root);
    }
    if root.join("Cargo.toml").is_file()
        && root
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "adl")
    {
        if let Some(parent) = root.parent() {
            if parent.join("adl/Cargo.toml").is_file() {
                return Ok(parent.to_path_buf());
            }
        }
    }
    ensure!(
        root.join("adl/Cargo.toml").is_file(),
        "repo root must contain adl/Cargo.toml: {}",
        root.display()
    );
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root")
            .to_path_buf()
    }

    fn active_template_set() -> String {
        load_editor_model(&repo_root()).expect("model").template_set
    }

    #[test]
    fn editor_model_covers_all_five_cards() {
        let model = load_editor_model(&repo_root()).expect("model");
        let active = active_template_set();
        assert_eq!(model.template_set, active);
        assert_eq!(
            model.card_status_values,
            [
                "draft",
                "ready",
                "reviewed",
                "approved",
                "completed",
                "blocked",
                "superseded"
            ]
        );
        assert_eq!(
            model.cards.len(),
            PromptCardKind::all_for_template_set(&active).len()
        );
        assert_eq!(
            model
                .cards
                .iter()
                .any(|card| card.kind == PromptCardKind::Vpp),
            template_set_supports_vpp(&active)
        );
        assert!(model
            .cards
            .iter()
            .any(|card| card.kind == PromptCardKind::Srp));
        assert!(model.cards.iter().all(|card| card
            .template_path
            .starts_with(&format!("docs/templates/prompts/{active}/"))));
    }

    #[test]
    fn staged_template_set_includes_vpp_card() {
        let model = load_editor_model_for_template_set(&repo_root(), Some("1.0.3")).expect("model");
        assert_eq!(model.template_set, "1.0.3");
        assert_eq!(model.cards.len(), 6);
        assert!(model
            .cards
            .iter()
            .any(|card| card.kind == PromptCardKind::Vpp));
    }

    #[test]
    fn import_values_accepts_rendered_cards_from_prior_template_sets() {
        let repo_root = repo_root();
        let tmp = std::env::temp_dir().join(format!(
            "adl-prompt-import-legacy-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&tmp).expect("tmp");

        let kind = PromptCardKind::Spp;
        let fixture_rel =
            "docs/tooling/csdlc-prompt-editor/repair_examples/spp_repaired_issue_plan.md";
        let legacy_rendered =
            fs::read_to_string(repo_root.join(fixture_rel)).expect("legacy fixture");

        let input = tmp.join(format!("legacy-{}.md", kind.key()));
        let imported = tmp.join(format!("legacy-{}.values.yaml", kind.key()));
        let normalized = tmp.join(format!("normalized-{}.md", kind.key()));
        fs::write(&input, legacy_rendered).expect("write legacy rendered card");

        let report = import_values_from_rendered_card_file(
            &repo_root,
            kind,
            &input,
            &imported,
            Some(&normalized),
            None,
        )
        .expect("import should bridge legacy rendered card");
        assert_eq!(report.comparison, PromptCardRoundTripComparison::Normalized);

        let imported_text = fs::read_to_string(&imported).expect("imported values");
        let active = active_template_set();
        assert!(imported_text.contains(&format!("template_set: \"{active}\"")));
        let normalized_text = fs::read_to_string(&normalized).expect("normalized rendered");
        assert!(normalized_text.contains(&format!(
            "Canonical Template Source: `docs/templates/prompts/{}/{}.md`",
            active,
            kind.key()
        )));
    }

    #[test]
    fn import_values_rejects_unrepresentable_legacy_sor_cards() {
        let repo_root = repo_root();
        let tmp = std::env::temp_dir().join(format!(
            "adl-prompt-import-legacy-sor-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&tmp).expect("tmp");

        let input = tmp.join("legacy-sor.md");
        let imported = tmp.join("legacy-sor.values.yaml");
        let legacy_rendered = fs::read_to_string(
            repo_root
                .join("docs/tooling/csdlc-prompt-editor/repair_examples/sor_repaired_pr_open.md"),
        )
        .expect("legacy sor fixture");
        fs::write(&input, legacy_rendered).expect("write legacy sor rendered card");

        let err = import_values_from_rendered_card_file(
            &repo_root,
            PromptCardKind::Sor,
            &input,
            &imported,
            None,
            None,
        )
        .expect_err("non-bootstrap legacy sor should fail closed");
        assert!(err
            .to_string()
            .contains("legacy sor import only supports bootstrap scaffolds"));
    }

    #[test]
    fn legacy_sor_import_infers_vpp_card_from_validation_planning_prompt() {
        let rendered = r#"# infer-vpp-card-when-importing-older-sor-cards

Task ID: issue-4584
Run ID: issue-4584
Version: v0.91.6
Title: [v0.91.6][tools][templates] Infer vpp_card when importing older SOR cards
Branch: codex/4584-infer-vpp-card-when-importing-older-sor-cards
Card Status: draft
Status: NOT_STARTED

## Summary

Pre-run output scaffold initialized during issue-wave opening.

## Issue Metrics Truth
- Validation planning prompt: `.adl/v0.91.6/tasks/issue-4584__infer-vpp-card-when-importing-older-sor-cards/vpp.md`

## Artifacts produced
- Local ignored output-card scaffold.

## Validation
- `bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/v0.91.6/tasks/issue-4584__infer-vpp-card-when-importing-older-sor-cards/sor.md`
  Verified bootstrap SOR contract compliance.
"#;

        let (values, _) = extract_legacy_sor_values(rendered).expect("legacy sor values");
        ensure_legacy_sor_is_representable_in_active_template(&values)
            .expect("bootstrap legacy sor with vpp card should be representable");
        assert_eq!(
            values.get("vpp_card").map(String::as_str),
            Some(".adl/v0.91.6/tasks/issue-4584__infer-vpp-card-when-importing-older-sor-cards/vpp.md")
        );
    }

    #[test]
    fn legacy_sor_import_requires_vpp_card_for_bootstrap_scaffolds() {
        let rendered = r#"# infer-vpp-card-when-importing-older-sor-cards

Task ID: issue-4584
Run ID: issue-4584
Version: v0.91.6
Title: [v0.91.6][tools][templates] Infer vpp_card when importing older SOR cards
Branch: codex/4584-infer-vpp-card-when-importing-older-sor-cards
Card Status: draft
Status: NOT_STARTED

## Summary

Pre-run output scaffold initialized during issue-wave opening.

## Artifacts produced
- Local ignored output-card scaffold.

## Validation
- `bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/v0.91.6/tasks/issue-4584__infer-vpp-card-when-importing-older-sor-cards/sor.md`
  Verified bootstrap SOR contract compliance.
"#;

        let (values, _) = extract_legacy_sor_values(rendered).expect("legacy sor values");
        let err = ensure_legacy_sor_is_representable_in_active_template(&values)
            .expect_err("bootstrap legacy sor without vpp card should fail clearly");
        assert!(err.to_string().contains(
            "legacy sor import requires Validation planning prompt line to infer vpp_card"
        ));
    }

    #[test]
    fn import_values_rejects_spoofed_legacy_template_cards_with_missing_truth() {
        let repo_root = repo_root();
        let tmp = std::env::temp_dir().join(format!(
            "adl-prompt-import-legacy-spoof-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&tmp).expect("tmp");
        let input = tmp.join("spoofed-legacy-spp.md");
        let imported = tmp.join("spoofed-legacy-spp.values.yaml");
        let spoofed = r#"---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "spoofed-legacy-execution-plan"
issue: 4278
task_id: "issue-4278"
run_id: "issue-4278"
version: "v0.91.6"
title: "Spoofed legacy card"
branch: "codex/4278-spoofed"
card_status: "approved"
status: "reviewed"
activation_state: "ready_for_execution_binding"
plan_summary: "Malformed legacy card."
---

Canonical Template Source: `docs/templates/prompts/1.0.0/spp.md`

# Structured Plan Prompt

## Plan Summary

Malformed legacy card.
"#;
        fs::write(&input, spoofed).expect("write spoofed legacy rendered card");
        let err = import_values_from_rendered_card_file(
            &repo_root,
            PromptCardKind::Spp,
            &input,
            &imported,
            None,
            None,
        )
        .expect_err("spoofed legacy card should fail closed");
        assert!(err
            .to_string()
            .contains("legacy import requires scalar field"));
    }

    #[test]
    fn sor_actual_metrics_data_source_editor_enum_matches_validator_sources() {
        let model = load_editor_model(&repo_root()).expect("model");
        let sor = model
            .cards
            .iter()
            .find(|card| card.kind == PromptCardKind::Sor)
            .expect("sor model");
        let mut values = sample_values();
        values.insert(
            "actual_metrics_data_source".to_string(),
            "codex_goal_tool".to_string(),
        );
        values.insert(
            "actual_metrics_source_ref".to_string(),
            ".adl/goal-metrics/issue-4278.json".to_string(),
        );
        values.insert(
            "actual_metrics_confidence".to_string(),
            "medium".to_string(),
        );
        validate_values(sor, &values).expect("editor should accept codex_goal_tool");
    }

    #[test]
    fn invalid_enum_values_are_rejected() {
        let model = load_editor_model(&repo_root()).expect("model");
        let sor = model
            .cards
            .iter()
            .find(|card| card.kind == PromptCardKind::Sor)
            .expect("sor model");
        let mut values = sample_values();
        values.insert("status".to_string(), "almost_done".to_string());
        let err = validate_values(sor, &values).expect_err("invalid enum should fail");
        assert!(err.to_string().contains("status must be one of"));
    }

    #[test]
    fn sample_rendering_has_no_unresolved_placeholders() {
        let active = active_template_set();
        for kind in PromptCardKind::all_for_template_set(&active) {
            let text = render_sample_card(&repo_root(), kind).expect("sample render");
            assert!(
                unresolved_placeholder_offset(&text).is_none(),
                "{} sample should not have unresolved placeholders",
                kind.key()
            );
            if kind == PromptCardKind::Srp {
                assert!(text.contains("# Structured Review Prompt"));
                assert!(!text.contains("# Structured Review Policy"));
            }
        }
    }

    #[test]
    fn import_values_from_rendered_cards_round_trips_all_card_kinds() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-prompt-values-import-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&tmp).expect("tmp");

        let active = active_template_set();
        for kind in PromptCardKind::all_for_template_set(&active) {
            let rendered = render_sample_card(&repo_root(), kind).expect("sample render");
            let input = tmp.join(format!("{}.md", kind.key()));
            let values = tmp.join(format!("{}.imported.values.yaml", kind.key()));
            let normalized = tmp.join(format!("{}.normalized.md", kind.key()));
            fs::write(&input, &rendered).expect("rendered card");

            let report = import_values_from_rendered_card_file(
                &repo_root(),
                kind,
                &input,
                &values,
                Some(&normalized),
                None,
            )
            .expect("rendered card should import");
            assert_eq!(report.comparison, PromptCardRoundTripComparison::Exact);

            validate_values_file(&repo_root(), kind, &values).expect("imported values valid");
            let rerendered = render_card_from_values_file(&repo_root(), kind, &values)
                .expect("imported values render");
            assert_eq!(
                rerendered,
                rendered,
                "{} should round-trip exactly",
                kind.key()
            );
            assert_eq!(
                fs::read_to_string(&normalized).expect("normalized card"),
                rendered
            );
        }
    }

    #[test]
    fn import_values_fails_closed_for_locked_template_drift_on_all_card_kinds() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-prompt-values-import-drift-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&tmp).expect("tmp");

        let active = active_template_set();
        for kind in PromptCardKind::all_for_template_set(&active) {
            let rendered = render_sample_card(&repo_root(), kind).expect("sample render");
            let drifted = rendered.replace(
                "Canonical Template Source:",
                "Canonical Template Source Drift:",
            );
            let input = tmp.join(format!("{}.drift.md", kind.key()));
            let values = tmp.join(format!("{}.drift.values.yaml", kind.key()));
            fs::write(&input, drifted).expect("drifted card");

            let err = import_values_from_rendered_card_file(
                &repo_root(),
                kind,
                &input,
                &values,
                None,
                None,
            )
            .expect_err("locked template drift should fail");
            assert!(
                err.to_string().contains("locked template text drifted")
                    || err
                        .to_string()
                        .contains("cannot be represented by active template"),
                "{} should fail closed for locked prose drift: {err}",
                kind.key()
            );
        }
    }

    #[test]
    fn values_files_split_locked_system_fields_from_editable_values() {
        let model = load_editor_model(&repo_root()).expect("model");
        let sip = model
            .cards
            .iter()
            .find(|card| card.kind == PromptCardKind::Sip)
            .expect("sip model");
        let tmp = std::env::temp_dir().join(format!(
            "adl-prompt-values-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&tmp).expect("tmp");

        let good_path = tmp.join("sip.values.yaml");
        fs::write(
            &good_path,
            sample_values_document(PromptCardKind::Sip, &active_template_set()),
        )
        .expect("good values");
        let good =
            load_values_file(sip, &good_path, &model.template_set).expect("load good values");
        assert_eq!(good.get("card_status").map(String::as_str), Some("ready"));
        assert_eq!(good.get("issue").map(String::as_str), Some("1374"));
        assert_eq!(
            good.get("required_outcome_type").map(String::as_str),
            Some("combination")
        );

        let bad_locked = tmp.join("bad-locked.values.yaml");
        let active_set = active_template_set();
        fs::write(
            &bad_locked,
            format!(
                r#"schema: adl.csdlc.prompt_template_values.v1
template_set: "{active_set}"
card_kind: sip
values:
  issue: "1374"
"#
            ),
        )
        .expect("bad locked values");
        let err = load_values_file(sip, &bad_locked, &model.template_set)
            .expect_err("locked value should fail");
        assert!(err.to_string().contains("values.issue is locked"));

        let bad_editable = tmp.join("bad-editable.values.yaml");
        fs::write(
            &bad_editable,
            format!(
                r#"schema: adl.csdlc.prompt_template_values.v1
template_set: "{active_set}"
card_kind: sip
system:
  goal: "Wrong section"
"#
            ),
        )
        .expect("bad editable values");
        let err = load_values_file(sip, &bad_editable, &model.template_set)
            .expect_err("editable system should fail");
        assert!(err.to_string().contains("system.goal is editable"));

        let bad_template_set = tmp.join("bad-template-set.values.yaml");
        fs::write(
            &bad_template_set,
            r#"schema: adl.csdlc.prompt_template_values.v1
template_set: "9.9.9"
card_kind: sip
system:
  issue: "1374"
"#,
        )
        .expect("bad template set values");
        let err = load_values_file(sip, &bad_template_set, &model.template_set)
            .expect_err("template set drift should fail");
        assert!(err
            .to_string()
            .contains("template_set must match active template set"));
    }

    #[test]
    fn edit_values_file_updates_declared_editable_fields_for_all_card_kinds() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-prompt-values-edit-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&tmp).expect("tmp");

        for (kind, field, value) in [
            (
                PromptCardKind::Sip,
                "goal",
                "Exercise the field-level editor for SIP.",
            ),
            (
                PromptCardKind::Stp,
                "summary",
                "Exercise the field-level editor for STP.",
            ),
            (
                PromptCardKind::Spp,
                "plan_summary",
                "Exercise the field-level editor for SPP.",
            ),
            (
                PromptCardKind::Srp,
                "notes_risks",
                "Exercise the field-level editor for SRP.",
            ),
            (PromptCardKind::Sor, "status", "DONE"),
        ] {
            let input = tmp.join(format!("{}.values.yaml", kind.key()));
            let output = tmp.join(format!("{}.edited.values.yaml", kind.key()));
            fs::write(&input, sample_values_document(kind, &active_template_set()))
                .expect("sample values");

            edit_values_file(
                &repo_root(),
                kind,
                &input,
                &[(field.to_string(), value.to_string())],
                Some(&output),
            )
            .expect("editable field should update");

            let model = load_editor_model(&repo_root()).expect("model");
            let card = card_model(&model, kind).expect("card");
            let edited = load_values_file(card, &output, &model.template_set)
                .expect("edited values should load");
            assert_eq!(edited.get(field).map(String::as_str), Some(value));
            render_card_from_values_file(&repo_root(), kind, &output)
                .expect("edited values should render and validate structure");
        }

        let input = tmp.join("stp-multiline.values.yaml");
        let output = tmp.join("stp-multiline.edited.values.yaml");
        let multiline = "First line.\nSecond line.";
        fs::write(
            &input,
            sample_values_document(PromptCardKind::Stp, &active_template_set()),
        )
        .expect("sample values");
        edit_values_file(
            &repo_root(),
            PromptCardKind::Stp,
            &input,
            &[("summary".to_string(), multiline.to_string())],
            Some(&output),
        )
        .expect("multiline editable value should update");
        let model = load_editor_model(&repo_root()).expect("model");
        let stp = card_model(&model, PromptCardKind::Stp).expect("stp");
        let edited =
            load_values_file(stp, &output, &model.template_set).expect("load multiline values");
        assert_eq!(edited.get("summary").map(String::as_str), Some(multiline));
    }

    #[test]
    fn edit_values_file_fails_closed_for_locked_unknown_and_invalid_fields() {
        let tmp = std::env::temp_dir().join(format!(
            "adl-prompt-values-edit-fail-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&tmp).expect("tmp");
        let sip = tmp.join("sip.values.yaml");
        fs::write(
            &sip,
            sample_values_document(PromptCardKind::Sip, &active_template_set()),
        )
        .expect("sip values");

        let locked = edit_values_file(
            &repo_root(),
            PromptCardKind::Sip,
            &sip,
            &[("issue".to_string(), "9999".to_string())],
            Some(&tmp.join("locked.values.yaml")),
        )
        .expect_err("locked field edit should fail");
        assert!(locked.to_string().contains("sip.issue is locked"));

        let unknown = edit_values_file(
            &repo_root(),
            PromptCardKind::Sip,
            &sip,
            &[("unknown_field".to_string(), "nope".to_string())],
            Some(&tmp.join("unknown.values.yaml")),
        )
        .expect_err("unknown field edit should fail");
        assert!(unknown
            .to_string()
            .contains("sip.unknown_field is not a declared prompt-template field"));

        let sor = tmp.join("sor.values.yaml");
        fs::write(
            &sor,
            sample_values_document(PromptCardKind::Sor, &active_template_set()),
        )
        .expect("sor values");
        let invalid_enum = edit_values_file(
            &repo_root(),
            PromptCardKind::Sor,
            &sor,
            &[("status".to_string(), "ALMOST_DONE".to_string())],
            Some(&tmp.join("invalid-enum.values.yaml")),
        )
        .expect_err("invalid enum edit should fail");
        assert!(invalid_enum
            .to_string()
            .contains("sor.status must be one of"));
    }
}
