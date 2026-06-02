use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

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
    "timestamp",
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
    Srp,
    Sor,
}

impl PromptCardKind {
    pub fn all() -> [Self; 5] {
        [Self::Sip, Self::Stp, Self::Spp, Self::Srp, Self::Sor]
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Sip => "sip",
            Self::Stp => "stp",
            Self::Spp => "spp",
            Self::Srp => "srp",
            Self::Sor => "sor",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Sip => "Structured Issue Prompt",
            Self::Stp => "Structured Task Prompt",
            Self::Spp => "Structured Plan Prompt",
            Self::Srp => "Structured Review Prompt",
            Self::Sor => "Structured Outcome Record",
        }
    }

    pub fn output_file(self) -> &'static str {
        match self {
            Self::Sip => "sip.md",
            Self::Stp => "stp.md",
            Self::Spp => "spp.md",
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
            "srp" => Ok(Self::Srp),
            "sor" => Ok(Self::Sor),
            other => bail!("card kind must be one of sip, stp, spp, srp, sor: {other}"),
        }
    }
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
}

pub fn load_editor_model(repo_root: &Path) -> Result<PromptEditorModel> {
    let registry_path = repo_root.join(TEMPLATE_REGISTRY);
    let raw = fs::read_to_string(&registry_path)
        .with_context(|| format!("failed to read {}", registry_path.display()))?;
    let registry: Registry = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse {}", registry_path.display()))?;

    let mut cards = Vec::new();
    for kind in PromptCardKind::all() {
        let template = registry
            .templates
            .get(kind.key())
            .ok_or_else(|| anyhow!("registry missing template for {}", kind.key()))?;
        let template_path = repo_root.join(&template.path);
        let template_text = fs::read_to_string(&template_path)
            .with_context(|| format!("failed to read {}", template_path.display()))?;
        cards.push(PromptCardForm {
            kind,
            key: kind.key(),
            label: kind.label(),
            output_file: kind.output_file(),
            template_path: template.path.clone(),
            fields: form_fields(kind),
            template: template_text,
        });
    }

    Ok(PromptEditorModel {
        schema: "adl.csdlc.prompt_editor.model.v1",
        template_set: registry.csdlc_prompt_template_set,
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
    let values = sample_values();
    validate_values(card, &values)?;
    render_template(&card.template, &values)
}

pub fn render_card_from_values_file(
    repo_root: &Path,
    kind: PromptCardKind,
    values_path: &Path,
) -> Result<String> {
    let model = load_editor_model(repo_root)?;
    let card = card_model(&model, kind)?;
    let values = load_values_file(card, values_path, &model.template_set)?;
    validate_values(card, &values)?;
    render_template(&card.template, &values)
}

pub fn validate_values_file(
    repo_root: &Path,
    kind: PromptCardKind,
    values_path: &Path,
) -> Result<()> {
    let model = load_editor_model(repo_root)?;
    let card = card_model(&model, kind)?;
    let values = load_values_file(card, values_path, &model.template_set)?;
    validate_values(card, &values)
}

pub fn render_all_cards_from_values_dir(
    repo_root: &Path,
    values_dir: &Path,
    out_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;
    for kind in PromptCardKind::all() {
        let values_path = values_dir.join(format!("{}.values.yaml", kind.key()));
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
    for kind in PromptCardKind::all() {
        let text = render_sample_card(repo_root, kind)?;
        fs::write(out_dir.join(kind.output_file()), text)
            .with_context(|| format!("failed to write {}", kind.output_file()))?;
    }
    Ok(())
}

pub fn write_all_sample_values(out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;
    for kind in PromptCardKind::all() {
        let text = sample_values_document(kind);
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
    Ok(())
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

fn load_values_file(
    card: &PromptCardForm,
    path: &Path,
    expected_template_set: &str,
) -> Result<BTreeMap<String, String>> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read values file {}", path.display()))?;
    let doc: serde_yaml::Value = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed to parse values file {}", path.display()))?;
    let mapping = doc
        .as_mapping()
        .ok_or_else(|| anyhow!("values file must be a YAML/JSON mapping"))?;

    let schema = mapping_string_value(mapping, "schema")?
        .ok_or_else(|| anyhow!("values file must declare schema"))?;
    ensure!(
        schema == VALUES_SCHEMA,
        "values schema must be {VALUES_SCHEMA}: got {schema}"
    );
    let template_set = mapping_string_value(mapping, "template_set")?
        .ok_or_else(|| anyhow!("values file must declare template_set"))?;
    ensure!(
        template_set == expected_template_set,
        "values template_set must match active template set: expected {}, got {}",
        expected_template_set,
        template_set
    );
    if let Some(card_kind) = mapping_string_value(mapping, "card_kind")? {
        ensure!(
            card_kind == card.key,
            "values card_kind must match requested kind: expected {}, got {}",
            card.key,
            card_kind
        );
    }

    let mut values = BTreeMap::new();
    if let Some(system) = mapping.get(serde_yaml::Value::String("system".to_string())) {
        collect_values_section(card, system, false, &mut values, "system")?;
    }
    if let Some(editable) = mapping.get(serde_yaml::Value::String("values".to_string())) {
        collect_values_section(card, editable, true, &mut values, "values")?;
    }

    ensure!(
        !values.is_empty(),
        "values file must contain system and/or values mappings"
    );
    Ok(values)
}

fn collect_values_section(
    card: &PromptCardForm,
    section: &serde_yaml::Value,
    ordinary_values: bool,
    out: &mut BTreeMap<String, String>,
    section_name: &str,
) -> Result<()> {
    let mapping = section
        .as_mapping()
        .ok_or_else(|| anyhow!("{section_name} must be a mapping"))?;
    for (key, value) in mapping {
        let key = key
            .as_str()
            .ok_or_else(|| anyhow!("{section_name} keys must be strings"))?;
        let field = card.fields.iter().find(|field| field.key == key);
        let known_placeholder = PLACEHOLDERS.contains(&key);
        ensure!(
            field.is_some() || known_placeholder,
            "{}.{} is not a supported field for {}",
            section_name,
            key,
            card.key
        );
        if ordinary_values {
            ensure!(
                field.is_some_and(|field| field.editable),
                "values.{} is locked; supply it through system fields",
                key
            );
        } else if let Some(field) = field {
            ensure!(
                !field.editable,
                "system.{} is editable; supply it through values fields",
                key
            );
        }
        let value = scalar_to_string(value)
            .ok_or_else(|| anyhow!("{}.{} must be a scalar value", section_name, key))?;
        out.insert(key.to_string(), value);
    }
    Ok(())
}

fn scalar_to_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(value) => Some(value.clone()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        serde_yaml::Value::Null => Some(String::new()),
        _ => None,
    }
}

fn mapping_string_value(mapping: &serde_yaml::Mapping, key: &str) -> Result<Option<String>> {
    let Some(value) = mapping.get(serde_yaml::Value::String(key.to_string())) else {
        return Ok(None);
    };
    let value = value
        .as_str()
        .ok_or_else(|| anyhow!("{key} must be a string"))?;
    Ok(Some(value.to_string()))
}

fn sample_values_document(kind: PromptCardKind) -> String {
    let values = sample_values();
    let editable_keys = form_fields(kind)
        .iter()
        .filter(|field| field.editable)
        .map(|field| field.key)
        .collect::<BTreeSet<_>>();
    let system_keys = PLACEHOLDERS
        .iter()
        .copied()
        .filter(|key| !editable_keys.contains(key))
        .collect::<BTreeSet<_>>();

    let mut out = String::new();
    out.push_str("schema: adl.csdlc.prompt_template_values.v1\n");
    out.push_str("template_set: 1.0.0\n");
    out.push_str(&format!("card_kind: {}\n", kind.key()));
    out.push_str("system:\n");
    for key in system_keys {
        let value = values.get(key).map(String::as_str).unwrap_or_default();
        out.push_str(&format!("  {}: {}\n", key, yaml_quote(value)));
    }
    out.push_str("values:\n");
    for key in editable_keys {
        let value = values.get(key).map(String::as_str).unwrap_or_default();
        out.push_str(&format!("  {}: {}\n", key, yaml_quote(value)));
    }
    out
}

fn yaml_quote(value: &str) -> String {
    if value.contains('\n') {
        let mut out = String::from("|-\n");
        for line in value.lines() {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
        return out.trim_end().to_string();
    }
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\t', "\\t")
    )
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
            "Generate validator-clean sample cards for all five C-SDLC phases.",
        ),
        (
            "deliverables",
            "- Rust-owned form metadata\n- Local browser editor\n- Validator-clean samples",
        ),
        (
            "acceptance_criteria",
            "- All five samples validate\n- No unresolved placeholders remain",
        ),
        (
            "inputs",
            "- Active SemVer prompt-template registry\n- Human-entered issue metadata",
        ),
        (
            "repo_inputs",
            "- docs/templates/prompts/current.json\n- docs/templates/prompts/1.0.0/",
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
            "docs/templates/prompts/current.json and docs/templates/prompts/1.0.0/.",
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
        ("timestamp", "2026-05-23T00:00:00Z"),
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

    #[test]
    fn editor_model_covers_all_five_cards() {
        let model = load_editor_model(&repo_root()).expect("model");
        assert_eq!(model.template_set, "1.0.0");
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
        assert_eq!(model.cards.len(), 5);
        assert!(model
            .cards
            .iter()
            .any(|card| card.kind == PromptCardKind::Srp));
        assert!(model.cards.iter().all(|card| card
            .template_path
            .starts_with("docs/templates/prompts/1.0.0/")));
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
        for kind in PromptCardKind::all() {
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
        fs::write(&good_path, sample_values_document(PromptCardKind::Sip)).expect("good values");
        let good =
            load_values_file(sip, &good_path, &model.template_set).expect("load good values");
        assert_eq!(good.get("card_status").map(String::as_str), Some("ready"));
        assert_eq!(good.get("issue").map(String::as_str), Some("1374"));
        assert_eq!(
            good.get("required_outcome_type").map(String::as_str),
            Some("combination")
        );

        let bad_locked = tmp.join("bad-locked.values.yaml");
        fs::write(
            &bad_locked,
            r#"schema: adl.csdlc.prompt_template_values.v1
template_set: "1.0.0"
card_kind: sip
values:
  issue: "1374"
"#,
        )
        .expect("bad locked values");
        let err = load_values_file(sip, &bad_locked, &model.template_set)
            .expect_err("locked value should fail");
        assert!(err.to_string().contains("values.issue is locked"));

        let bad_editable = tmp.join("bad-editable.values.yaml");
        fs::write(
            &bad_editable,
            r#"schema: adl.csdlc.prompt_template_values.v1
template_set: "1.0.0"
card_kind: sip
system:
  goal: "Wrong section"
"#,
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
}
