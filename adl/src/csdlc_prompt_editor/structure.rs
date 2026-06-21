use super::{
    unresolved_curly_placeholder_offset, unresolved_placeholder_offset, PromptCardForm,
    PromptCardKind, PLACEHOLDERS,
};
use anyhow::{anyhow, bail, ensure, Context, Result};
use markdown::{mdast::Node, ParseOptions};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub(crate) fn validate_rendered_card_structure_from_repo(
    repo_root: &Path,
    card: &PromptCardForm,
    rendered: &str,
) -> Result<()> {
    ensure!(
        unresolved_placeholder_offset(rendered).is_none()
            && unresolved_curly_placeholder_offset(rendered).is_none(),
        "{} rendered card contains unresolved prompt-template placeholder",
        card.key
    );
    let schema = load_structure_schema(repo_root, card)?;
    validate_rendered_card_structure_with_schema(card, rendered, &schema)
}

pub(crate) fn validate_rendered_card_structure(
    card: &PromptCardForm,
    rendered: &str,
) -> Result<()> {
    let schema = build_structure_schema("inline", card)?;
    validate_rendered_card_structure_with_schema(card, rendered, &schema)
}

fn validate_rendered_card_structure_with_schema(
    card: &PromptCardForm,
    rendered: &str,
    schema: &PromptCardStructureSchema,
) -> Result<()> {
    ensure!(
        schema.card_kind == card.key,
        "{} structure schema card_kind mismatch: {}",
        card.key,
        schema.card_kind
    );
    ensure!(
        schema.template_path == card.template_path,
        "{} structure schema template_path mismatch: {}",
        card.key,
        schema.template_path
    );
    let actual = PromptMarkdownStructure::from_text(card.key, rendered, schema)?;
    ensure!(
        actual.frontmatter_keys == schema.frontmatter_keys,
        "{} frontmatter key inventory drifted: expected {:?}, got {:?}",
        card.key,
        schema.frontmatter_keys,
        actual.frontmatter_keys
    );
    ensure!(
        headings_match(&schema.headings, &actual.headings),
        "{} Markdown heading structure drifted: expected {:?}, got {:?}",
        card.key,
        schema.headings,
        actual.headings
    );
    ensure!(
        fenced_blocks_match(&schema.fenced_blocks, &actual.fenced_blocks),
        "{} fenced block structure drifted: expected {:?}, got {:?}",
        card.key,
        schema.fenced_blocks,
        actual.fenced_blocks
    );
    if !locked_lines_match(&schema.locked_lines, &actual.locked_lines) {
        bail!(
            "{} locked template text drifted: {}",
            card.key,
            locked_line_diff(&schema.locked_lines, &actual.locked_lines)
        );
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(super) struct PromptCardStructureSchema {
    schema: String,
    template_set: String,
    card_kind: String,
    template_path: String,
    parser: String,
    editable_sections: Vec<String>,
    scaffold_lines: Vec<String>,
    scaffold_line_prefixes: Vec<String>,
    rendered_value_line_prefixes: Vec<String>,
    frontmatter_keys: Vec<String>,
    headings: Vec<MarkdownHeading>,
    fenced_blocks: Vec<FencedBlockShape>,
    locked_lines: Vec<LockedLine>,
}

#[derive(Debug, PartialEq, Eq)]
struct PromptMarkdownStructure {
    frontmatter_keys: Vec<String>,
    headings: Vec<MarkdownHeading>,
    fenced_blocks: Vec<FencedBlockShape>,
    locked_lines: Vec<LockedLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct MarkdownHeading {
    level: usize,
    text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FencedBlockShape {
    ordinal: usize,
    info: String,
    heading_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LockedLine {
    heading_path: Vec<String>,
    text: String,
}

impl PromptMarkdownStructure {
    fn from_text(kind: &str, text: &str, schema: &PromptCardStructureSchema) -> Result<Self> {
        let (frontmatter_keys, body) = split_optional_frontmatter_keys(text)?;
        let (headings, fenced_blocks) = markdown_ast_structure(kind, body)?;
        let mut locked_lines = Vec::new();
        let mut heading_stack: Vec<(usize, String)> = Vec::new();
        let mut in_fence = false;
        let editable_sections = schema.editable_sections.iter().collect::<BTreeSet<_>>();

        for raw_line in body.lines() {
            let trimmed_start = raw_line.trim_start();
            if trimmed_start.starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            if in_fence {
                continue;
            }

            if let Some(heading) = parse_markdown_heading(raw_line) {
                while heading_stack
                    .last()
                    .is_some_and(|(level, _)| *level >= heading.level)
                {
                    heading_stack.pop();
                }
                if let Some(text) = &heading.text {
                    heading_stack.push((heading.level, text.clone()));
                } else {
                    heading_stack.push((heading.level, "<dynamic-heading>".to_string()));
                }
                continue;
            }

            if heading_stack
                .last()
                .is_some_and(|(_, heading)| editable_sections.contains(heading))
            {
                continue;
            }
            let trimmed = raw_line.trim();
            if trimmed.is_empty() || contains_prompt_placeholder(raw_line) {
                continue;
            }
            if is_template_scaffold_line(trimmed, schema) || is_rendered_value_line(trimmed, schema)
            {
                continue;
            }
            locked_lines.push(LockedLine {
                heading_path: heading_path(&heading_stack),
                text: raw_line.trim_end().to_string(),
            });
        }

        ensure!(
            !in_fence,
            "{kind} rendered card has an unclosed fenced code block"
        );

        Ok(Self {
            frontmatter_keys,
            headings,
            fenced_blocks,
            locked_lines,
        })
    }
}

pub(super) fn build_structure_schema(
    template_set: &str,
    card: &PromptCardForm,
) -> Result<PromptCardStructureSchema> {
    let mut schema = PromptCardStructureSchema {
        schema: "adl.csdlc.prompt_card_structure.v1".to_string(),
        template_set: template_set.to_string(),
        card_kind: card.key.to_string(),
        template_path: card.template_path.clone(),
        parser: "markdown-rs 1.0.0".to_string(),
        editable_sections: dynamic_markdown_sections(card)
            .into_iter()
            .map(str::to_string)
            .collect(),
        scaffold_lines: TEMPLATE_SCAFFOLD_LINES
            .iter()
            .map(|line| line.to_string())
            .collect(),
        scaffold_line_prefixes: TEMPLATE_SCAFFOLD_PREFIXES
            .iter()
            .map(|line| line.to_string())
            .collect(),
        rendered_value_line_prefixes: rendered_value_line_prefixes(card.kind)
            .into_iter()
            .map(str::to_string)
            .collect(),
        frontmatter_keys: Vec::new(),
        headings: Vec::new(),
        fenced_blocks: Vec::new(),
        locked_lines: Vec::new(),
    };
    let structure = PromptMarkdownStructure::from_text(card.key, &card.template, &schema)?;
    schema.frontmatter_keys = structure.frontmatter_keys;
    schema.headings = structure.headings;
    schema.fenced_blocks = structure.fenced_blocks;
    schema.locked_lines = structure.locked_lines;
    Ok(schema)
}

pub(super) fn load_structure_schema(
    repo_root: &Path,
    card: &PromptCardForm,
) -> Result<PromptCardStructureSchema> {
    let path = repo_root.join(&card.structure_schema_path);
    let raw = fs::read_to_string(&path).with_context(|| {
        format!(
            "failed to read prompt-card structure schema {}",
            path.display()
        )
    })?;
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => serde_json::from_str(&raw).with_context(|| {
            format!(
                "failed to parse prompt-card structure schema {}",
                path.display()
            )
        }),
        _ => serde_yaml::from_str(&raw).with_context(|| {
            format!(
                "failed to parse prompt-card structure schema {}",
                path.display()
            )
        }),
    }
}

pub(super) fn default_structure_schema_path(template_set: &str, kind: PromptCardKind) -> String {
    format!(
        "docs/templates/prompts/{template_set}/schemas/{}.structure.json",
        kind.key()
    )
}

fn dynamic_markdown_sections(card: &PromptCardForm) -> BTreeSet<&'static str> {
    let mut sections = PLACEHOLDERS
        .iter()
        .flat_map(|key| dynamic_field_sections(key))
        .collect::<BTreeSet<_>>();
    if card.kind == PromptCardKind::Stp {
        sections.insert("Demo Expectations");
        sections.insert("Notes");
    }
    sections
}

fn dynamic_field_sections(key: &str) -> Vec<&'static str> {
    match key {
        "summary" => vec!["Summary"],
        "goal" => vec!["Goal"],
        "required_outcome" => vec!["Required Outcome"],
        "deliverables" => vec!["Deliverables"],
        "acceptance_criteria" => vec!["Acceptance Criteria"],
        "inputs" => vec!["Inputs"],
        "repo_inputs" => vec!["Repo Inputs"],
        "dependencies" => vec!["Dependencies"],
        "target_files_surfaces" => vec!["Target Files / Surfaces"],
        "validation_plan" => vec!["Validation Plan"],
        "demo_proof_requirements" => vec!["Demo / Proof Requirements", "Demo Expectations"],
        "constraints_policies" => vec!["Constraints / Policies"],
        "system_invariants" => vec!["System Invariants (must remain true)"],
        "reviewer_checklist" => vec!["Reviewer Checklist (machine-readable hints)"],
        "non_goals" => vec!["Non-goals", "Non-goals / Out of scope"],
        "issue_graph_notes" => vec!["Issue-Graph Notes"],
        "notes_risks" => vec!["Notes / Risks", "Notes"],
        "tooling_notes" => vec!["Tooling Notes"],
        "plan_summary" => vec!["Plan Summary", "Validation Planning Summary"],
        "dependencies_inline" => vec!["Proposed Steps"],
        "repo_inputs_inline" => vec!["Proposed Steps"],
        "deliverables_inline" => vec!["Proposed Steps"],
        "acceptance_criteria_inline" => vec!["Proposed Steps"],
        "risks_inline" => vec!["Risks And Edge Cases"],
        "validation_plan_inline" => vec!["Test Strategy"],
        "selected_lanes_inline" => vec!["Selected Validation Lanes"],
        "parallel_groups_inline" => vec!["Parallelization Plan"],
        "validation_commands_inline" => vec!["Validation Commands"],
        "failure_policy" => vec!["Failure Semantics"],
        "notes_risks_inline" => vec!["Notes"],
        "slug" => vec!["Affected Areas"],
        "stp_card" | "sip_card" | "spp_card" | "srp_card" | "sor_card" => {
            vec!["Scope Basis", "Policy Refs"]
        }
        "output_card" => vec!["Outputs"],
        "branch_action" => vec!["Actions taken"],
        _ => Vec::new(),
    }
}

fn split_optional_frontmatter_keys(text: &str) -> Result<(Vec<String>, &str)> {
    let mut lines = text.lines().collect::<Vec<_>>();
    if lines.first().is_none_or(|line| line.trim() != "---") {
        return Ok((Vec::new(), text));
    }
    let close = lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.trim() == "---")
        .map(|(idx, _)| idx)
        .ok_or_else(|| anyhow!("missing YAML frontmatter closer"))?;
    let frontmatter = lines[1..close].join("\n");
    let keys = frontmatter_key_inventory(&frontmatter)?;
    let body_start = text
        .match_indices('\n')
        .nth(close)
        .map(|(idx, _)| idx + 1)
        .unwrap_or(text.len());
    lines.clear();
    Ok((keys, &text[body_start..]))
}

fn frontmatter_key_inventory(frontmatter: &str) -> Result<Vec<String>> {
    let doc: serde_yaml::Value = serde_yaml::from_str(frontmatter)
        .with_context(|| "failed to parse prompt card frontmatter")?;
    let mut keys = Vec::new();
    collect_yaml_key_paths(&doc, "", &mut keys)?;
    keys.sort();
    Ok(keys)
}

fn collect_yaml_key_paths(
    value: &serde_yaml::Value,
    prefix: &str,
    out: &mut Vec<String>,
) -> Result<()> {
    let Some(mapping) = value.as_mapping() else {
        return Ok(());
    };
    for (key, value) in mapping {
        let key = key
            .as_str()
            .ok_or_else(|| anyhow!("frontmatter keys must be strings"))?;
        let path = if prefix.is_empty() {
            key.to_string()
        } else {
            format!("{prefix}.{key}")
        };
        out.push(path.clone());
        collect_yaml_key_paths(value, &path, out)?;
    }
    Ok(())
}

fn parse_markdown_heading(line: &str) -> Option<MarkdownHeading> {
    let trimmed = line.trim_start();
    let marker_len = trimmed.chars().take_while(|ch| *ch == '#').count();
    if !(1..=6).contains(&marker_len) {
        return None;
    }
    let rest = &trimmed[marker_len..];
    if !rest.starts_with(' ') {
        return None;
    }
    Some(MarkdownHeading {
        level: marker_len,
        text: if contains_prompt_placeholder(rest) {
            None
        } else {
            Some(rest.trim().to_string())
        },
    })
}

fn heading_path(stack: &[(usize, String)]) -> Vec<String> {
    stack.iter().map(|(_, text)| text.clone()).collect()
}

fn markdown_ast_structure(
    kind: &str,
    body: &str,
) -> Result<(Vec<MarkdownHeading>, Vec<FencedBlockShape>)> {
    let ast = markdown::to_mdast(body, &ParseOptions::default())
        .map_err(|err| anyhow!("{kind} markdown-rs AST parse failed: {err}"))?;
    let mut headings = Vec::new();
    let mut fenced_blocks = Vec::new();
    let mut heading_stack: Vec<(usize, String)> = Vec::new();
    let mut fence_ordinal = 0usize;
    collect_markdown_ast_structure(
        &ast,
        &mut heading_stack,
        &mut headings,
        &mut fenced_blocks,
        &mut fence_ordinal,
    );
    Ok((headings, fenced_blocks))
}

fn collect_markdown_ast_structure(
    node: &Node,
    heading_stack: &mut Vec<(usize, String)>,
    headings: &mut Vec<MarkdownHeading>,
    fenced_blocks: &mut Vec<FencedBlockShape>,
    fence_ordinal: &mut usize,
) {
    match node {
        Node::Root(root) => {
            for child in &root.children {
                collect_markdown_ast_structure(
                    child,
                    heading_stack,
                    headings,
                    fenced_blocks,
                    fence_ordinal,
                );
            }
        }
        Node::Heading(heading) => {
            let level = usize::from(heading.depth);
            let text = children_plain_text(&heading.children);
            while heading_stack
                .last()
                .is_some_and(|(stack_level, _)| *stack_level >= level)
            {
                heading_stack.pop();
            }
            let heading = MarkdownHeading {
                level,
                text: if text.trim().is_empty() || contains_prompt_placeholder(&text) {
                    None
                } else {
                    Some(text.trim().to_string())
                },
            };
            if let Some(text) = &heading.text {
                heading_stack.push((heading.level, text.clone()));
            } else {
                heading_stack.push((heading.level, "<dynamic-heading>".to_string()));
            }
            headings.push(heading);
        }
        Node::Code(code) => {
            fenced_blocks.push(FencedBlockShape {
                ordinal: *fence_ordinal,
                info: code.lang.clone().unwrap_or_default(),
                heading_path: heading_path(heading_stack),
            });
            *fence_ordinal += 1;
        }
        _ => {
            if let Some(children) = node.children() {
                for child in children {
                    collect_markdown_ast_structure(
                        child,
                        heading_stack,
                        headings,
                        fenced_blocks,
                        fence_ordinal,
                    );
                }
            }
        }
    }
}

fn children_plain_text(children: &[Node]) -> String {
    children.iter().map(node_plain_text).collect::<String>()
}

fn node_plain_text(node: &Node) -> String {
    match node {
        Node::Text(text) => text.value.clone(),
        Node::InlineCode(code) => code.value.clone(),
        Node::InlineMath(math) => math.value.clone(),
        _ => node.children().map_or_else(String::new, |children| {
            children.iter().map(node_plain_text).collect::<String>()
        }),
    }
}

fn headings_match(expected: &[MarkdownHeading], actual: &[MarkdownHeading]) -> bool {
    expected.len() == actual.len()
        && expected.iter().zip(actual).all(|(expected, actual)| {
            expected.level == actual.level
                && expected
                    .text
                    .as_ref()
                    .is_none_or(|expected_text| actual.text.as_ref() == Some(expected_text))
        })
}

fn fenced_blocks_match(expected: &[FencedBlockShape], actual: &[FencedBlockShape]) -> bool {
    expected.len() == actual.len()
        && expected.iter().zip(actual).all(|(expected, actual)| {
            expected.ordinal == actual.ordinal
                && expected.info == actual.info
                && heading_paths_match(&expected.heading_path, &actual.heading_path)
        })
}

fn heading_paths_match(expected: &[String], actual: &[String]) -> bool {
    expected.len() == actual.len()
        && expected
            .iter()
            .zip(actual)
            .all(|(expected, actual)| expected == "<dynamic-heading>" || expected == actual)
}

fn locked_lines_match(expected: &[LockedLine], actual: &[LockedLine]) -> bool {
    expected.len() == actual.len()
        && expected.iter().zip(actual).all(|(expected, actual)| {
            expected.text == actual.text
                && heading_paths_match(&expected.heading_path, &actual.heading_path)
        })
}

fn locked_line_diff(expected: &[LockedLine], actual: &[LockedLine]) -> String {
    let max = expected.len().max(actual.len());
    for idx in 0..max {
        match (expected.get(idx), actual.get(idx)) {
            (Some(expected), Some(actual))
                if expected.text != actual.text
                    || !heading_paths_match(&expected.heading_path, &actual.heading_path) =>
            {
                return format!(
                    "first drift at locked line {idx}: expected {:?}, got {:?}",
                    expected, actual
                );
            }
            (Some(expected), None) => {
                return format!("missing locked line {idx}: expected {:?}", expected);
            }
            (None, Some(actual)) => {
                return format!("unexpected locked line {idx}: got {:?}", actual);
            }
            _ => {}
        }
    }
    "locked line inventory differs".to_string()
}

fn contains_prompt_placeholder(line: &str) -> bool {
    PLACEHOLDERS
        .iter()
        .any(|key| line.contains(&format!("<{key}>")) || line.contains(&format!("{{{{{key}}}}}")))
}

const TEMPLATE_SCAFFOLD_LINES: &[&str] = &[
    "---",
    "```yaml",
    "```",
    "labels:",
    "supersedes: []",
    "duplicates: []",
    "depends_on: []",
    "canonical_files: []",
    "demo_names: []",
    "pr_start:",
    "enabled: true",
    "source_refs:",
    "scope:",
    "files:",
    "components:",
    "out_of_scope:",
    "constraints:",
    "assumptions:",
    "proposed_steps:",
    "codex_plan:",
    "affected_areas:",
    "invariants_to_preserve:",
    "risks_and_edge_cases:",
    "test_strategy:",
    "required_permissions:",
    "stop_conditions:",
    "alternatives_considered:",
    "review_hooks:",
    "review_results:",
];

const TEMPLATE_SCAFFOLD_PREFIXES: &[&str] = &[
    "- kind:",
    "kind:",
    "ref:",
    "- id:",
    "description:",
    "expected_output:",
    "allowed_mode:",
    "- step:",
    "status:",
    "- description:",
    "reason_not_chosen:",
];

const COMMON_RENDERED_VALUE_LINE_PREFIXES: &[&str] = &[
    "Task ID:",
    "Run ID:",
    "Version:",
    "Title:",
    "Branch:",
    "Card Status:",
    "Status:",
    "Generated:",
    "- Actor:",
    "- Model:",
    "- Start Time:",
    "- End Time:",
    "- Issue:",
    "- PR:",
    "- Source Issue Prompt:",
    "- Docs:",
    "- Other:",
    "- Agent:",
    "- Provider:",
    "- Tools allowed:",
    "- Sandbox / approvals:",
    "- Source issue-prompt slug:",
    "- Required outcome type:",
    "- Demo required:",
    "- Local ignored output-card scaffold at",
    "- `bash adl/tools/validate_structured_prompt.sh",
    "Source issue-prompt slug:",
    "Required outcome type:",
    "Demo required:",
    "Canonical Template Source:",
    "Generated from",
    "name:",
    "issue:",
    "task_id:",
    "run_id:",
    "version:",
    "title:",
    "branch:",
    "generated_at:",
    "card_status:",
    "activation_state:",
    "plan_revision:",
    "confidence:",
    "plan_summary:",
    "execution_handoff:",
    "notes:",
];

const PVF_RENDERED_VALUE_LINE_PREFIXES: &[&str] = &[
    "initial_pvf_lane:",
    "planned_pvf_lane:",
    "planned_pvf_lane_source:",
    "lane_registry_path:",
    "lane_registry_template_set:",
    "validation_runtime_class:",
    "validation_resource_profile:",
    "expected_proof_cost:",
    "estimate_elapsed_seconds:",
    "estimate_total_tokens:",
    "estimate_validation_seconds:",
    "planned_validation_seconds:",
    "planned_validation_tokens:",
    "estimate_confidence:",
    "estimate_data_source:",
    "estimate_source_ref:",
    "issue_goal_ref:",
    "sprint_goal_ref:",
    "goal_metrics_rollup_ref:",
    "selected_lanes:",
    "parallel_groups:",
    "validation_commands:",
    "failure_policy:",
    "- Registry path:",
    "- Registry template set:",
    "- Initial PVF lane from issue creation:",
    "- Planned PVF lane for execution:",
    "- Parallel groups:",
    "- Validation runtime class:",
    "- Validation resource profile:",
    "- Issue goal ref:",
    "- Sprint goal ref:",
    "- Goal metrics rollup ref:",
    "- Expected proof cost:",
    "- Planned validation seconds:",
    "- Planned validation tokens:",
    "- Planning lane source:",
    "- Revision rule:",
    "- Estimated elapsed seconds:",
    "- Estimated total tokens:",
    "- Estimated validation seconds:",
    "- Estimate confidence:",
    "- Estimate data source:",
    "- Estimate source ref:",
    "- Initial PVF lane:",
    "- Planned PVF lane:",
    "- Final PVF lane:",
    "- Lane change reason:",
    "- Actual elapsed seconds:",
    "- Actual total tokens:",
    "- Actual validation seconds:",
    "- Goal metrics data source:",
    "- Goal metrics source ref:",
    "- Data-source confidence:",
    "- Estimate error percent:",
    "- Variance analysis required:",
    "- Variance analysis completed:",
    "- Variance category:",
    "- Variance note:",
    "- Validation planning prompt:",
];

fn rendered_value_line_prefixes(kind: PromptCardKind) -> Vec<&'static str> {
    let mut prefixes = COMMON_RENDERED_VALUE_LINE_PREFIXES.to_vec();
    if matches!(
        kind,
        PromptCardKind::Spp | PromptCardKind::Vpp | PromptCardKind::Sor
    ) {
        prefixes.extend_from_slice(PVF_RENDERED_VALUE_LINE_PREFIXES);
    }
    prefixes
}

fn is_template_scaffold_line(trimmed: &str, schema: &PromptCardStructureSchema) -> bool {
    schema.scaffold_lines.iter().any(|line| line == trimmed)
        || schema
            .scaffold_line_prefixes
            .iter()
            .any(|prefix| trimmed.starts_with(prefix))
}

fn is_rendered_value_line(trimmed: &str, schema: &PromptCardStructureSchema) -> bool {
    schema
        .rendered_value_line_prefixes
        .iter()
        .any(|prefix| trimmed.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate dir has repo parent")
            .to_path_buf()
    }

    fn sample_card(kind: PromptCardKind) -> PromptCardForm {
        super::super::load_editor_model(&repo_root())
            .expect("load editor model")
            .cards
            .into_iter()
            .find(|card| card.kind == kind)
            .expect("sample card exists")
    }

    #[test]
    fn structure_helpers_cover_frontmatter_and_heading_parsing() {
        let (keys, body) = split_optional_frontmatter_keys(
            "---\nname: demo\nnested:\n  child: 1\n---\n# Heading\nbody\n",
        )
        .expect("frontmatter should parse");
        assert_eq!(keys, vec!["name", "nested", "nested.child"]);
        assert_eq!(body, "# Heading\nbody\n");

        let (empty_keys, unchanged) =
            split_optional_frontmatter_keys("# Heading\nbody\n").expect("no frontmatter");
        assert!(empty_keys.is_empty());
        assert_eq!(unchanged, "# Heading\nbody\n");

        let err = split_optional_frontmatter_keys("---\nname: missing closer\n")
            .expect_err("missing closer should fail");
        assert!(err.to_string().contains("missing YAML frontmatter closer"));

        let heading = parse_markdown_heading("### Demo Heading").expect("heading");
        assert_eq!(heading.level, 3);
        assert_eq!(heading.text.as_deref(), Some("Demo Heading"));
        let dynamic_heading =
            parse_markdown_heading("## <summary>").expect("dynamic heading still recognized");
        assert!(dynamic_heading.text.is_none());
        assert!(parse_markdown_heading("###NoSpace").is_none());
    }

    #[test]
    fn structure_helpers_cover_schema_and_ast_shape_logic() {
        let card = sample_card(PromptCardKind::Stp);
        let schema = build_structure_schema("test-set", &card).expect("schema");
        assert_eq!(
            default_structure_schema_path("1.0.0", PromptCardKind::Sip),
            "docs/templates/prompts/1.0.0/schemas/sip.structure.json"
        );
        assert!(schema
            .editable_sections
            .contains(&"Acceptance Criteria".to_string()));
        assert!(schema
            .editable_sections
            .contains(&"Demo Expectations".to_string()));

        let (headings, fenced_blocks) =
            markdown_ast_structure("demo", "# Root\n\n## Child\n\n```yaml\nkey: value\n```\n")
                .expect("markdown AST structure");
        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].text.as_deref(), Some("Root"));
        assert_eq!(fenced_blocks.len(), 1);
        assert_eq!(fenced_blocks[0].info, "yaml");
        assert_eq!(
            fenced_blocks[0].heading_path,
            vec!["Root".to_string(), "Child".to_string()]
        );
    }

    #[test]
    fn structure_validation_covers_mismatch_and_locked_line_helpers() {
        let card = sample_card(PromptCardKind::Sip);
        let rendered =
            super::super::render_sample_card(&repo_root(), PromptCardKind::Sip).expect("render");
        validate_rendered_card_structure(&card, &rendered).expect("sample render validates");

        let mut schema = build_structure_schema("inline", &card).expect("schema");
        schema.card_kind = "stp".to_string();
        let err = validate_rendered_card_structure_with_schema(&card, &rendered, &schema)
            .expect_err("card kind mismatch should fail");
        assert!(err
            .to_string()
            .contains("structure schema card_kind mismatch"));

        let expected = vec![LockedLine {
            heading_path: vec!["Root".to_string()],
            text: "expected".to_string(),
        }];
        let actual = vec![LockedLine {
            heading_path: vec!["Root".to_string()],
            text: "actual".to_string(),
        }];
        assert!(!locked_lines_match(&expected, &actual));
        assert!(locked_line_diff(&expected, &actual).contains("first drift"));
        assert!(heading_paths_match(
            &["<dynamic-heading>".to_string()],
            &["Runtime value".to_string()]
        ));
        assert!(fenced_blocks_match(
            &[FencedBlockShape {
                ordinal: 0,
                info: "yaml".to_string(),
                heading_path: vec!["<dynamic-heading>".to_string()],
            }],
            &[FencedBlockShape {
                ordinal: 0,
                info: "yaml".to_string(),
                heading_path: vec!["Actual".to_string()],
            }]
        ));
    }

    #[test]
    fn rendered_value_prefixes_scope_pvf_lane_lines_to_spp_and_sor() {
        let sip_schema = build_structure_schema("inline", &sample_card(PromptCardKind::Sip))
            .expect("sip schema");
        let spp_schema = build_structure_schema("inline", &sample_card(PromptCardKind::Spp))
            .expect("spp schema");
        let sor_schema = build_structure_schema("inline", &sample_card(PromptCardKind::Sor))
            .expect("sor schema");

        assert!(!sip_schema
            .rendered_value_line_prefixes
            .contains(&"initial_pvf_lane:".to_string()));
        assert!(!sip_schema
            .rendered_value_line_prefixes
            .contains(&"- Final PVF lane:".to_string()));
        assert!(spp_schema
            .rendered_value_line_prefixes
            .contains(&"initial_pvf_lane:".to_string()));
        assert!(sor_schema
            .rendered_value_line_prefixes
            .contains(&"- Final PVF lane:".to_string()));
    }
}
