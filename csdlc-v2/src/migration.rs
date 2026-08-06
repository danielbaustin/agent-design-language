use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use markdown::mdast::Node;
use markdown::{to_mdast, ParseOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

use crate::cards::{CardContent, CardKind, InitialCardInput, PlanStep, StepStatus, ValidationLane};
use crate::error::{ErrorCode, Result, V2Error};
use crate::lifecycle::initialize_issue;
use crate::model::{LifecyclePhase, MigrationEvidence};
use crate::{
    edit_issue, BootstrapRequest, CardStatus, EditRequest, PlanningProfile, SemanticOperation,
    Store,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ImportStatus {
    Imported,
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegacyImportRequest {
    pub schema: String,
    pub legacy_root: PathBuf,
    pub output_root: PathBuf,
    pub issue: u64,
    pub repository: String,
    pub title: String,
    pub slug: String,
    pub version: String,
    pub card_paths: BTreeMap<CardKind, String>,
    pub design_path: String,
    pub diagram_path: String,
    pub design_reviewer: String,
    pub actor: String,
    pub planning_profile: PlanningProfile,
    pub validation_lanes: Vec<ValidationLane>,
    pub imported_unix_seconds: u64,
    pub default_cutover_unix_seconds: u64,
    pub legacy_phase: LifecyclePhase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MigrationDiagnostic {
    pub card: Option<CardKind>,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ImportReport {
    pub schema: String,
    pub status: ImportStatus,
    pub issue: u64,
    pub source_digest: Option<String>,
    pub retained_section_count: usize,
    pub diagnostics: Vec<MigrationDiagnostic>,
    pub compatibility_view: Option<String>,
    pub sunset_unix_seconds: u64,
}

pub fn import_legacy(request: LegacyImportRequest) -> Result<ImportReport> {
    validate_request(&request)?;
    let mut authored = BTreeMap::new();
    let mut authored_sources = BTreeMap::new();
    let mut diagnostics = Vec::new();
    let mut source = blake3::Hasher::new();
    let legacy_root = request.legacy_root.canonicalize()?;
    let output_root = request.output_root.canonicalize()?;
    let mut canonical_sources = BTreeSet::new();
    for kind in enum_cards() {
        let Some(relative) = request.card_paths.get(&kind) else {
            diagnostics.push(diag(
                Some(kind),
                "card_missing",
                "all six legacy card paths are required",
            ));
            continue;
        };
        let relative_path = std::path::Path::new(relative);
        if relative_path.is_absolute()
            || !relative_path
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
        {
            diagnostics.push(diag(
                Some(kind),
                "source_path_unsafe",
                "legacy card path must be a clean relative path",
            ));
            continue;
        }
        let path = request.legacy_root.join(relative);
        let canonical = match path.canonicalize() {
            Ok(path) if path.starts_with(&legacy_root) && !path.starts_with(&output_root) => path,
            _ => {
                diagnostics.push(diag(
                    Some(kind),
                    "source_path_outside_legacy_root",
                    "legacy source resolves outside the disjoint legacy root",
                ));
                continue;
            }
        };
        if !canonical_sources.insert(canonical.clone()) {
            diagnostics.push(diag(
                Some(kind),
                "source_path_reused",
                "one legacy file cannot own multiple cards",
            ));
            continue;
        }
        let bytes = match fs::read(&canonical) {
            Ok(bytes) => bytes,
            Err(_) => {
                diagnostics.push(diag(
                    Some(kind),
                    "card_unreadable",
                    &format!("legacy card is unreadable: {relative}"),
                ));
                continue;
            }
        };
        source.update(&(relative.len() as u64).to_le_bytes());
        source.update(relative.as_bytes());
        source.update(&(bytes.len() as u64).to_le_bytes());
        source.update(&bytes);
        let text = match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => {
                diagnostics.push(diag(
                    Some(kind),
                    "card_not_utf8",
                    "legacy Markdown must be UTF-8",
                ));
                continue;
            }
        };
        authored_sources.insert(kind.to_string(), text.clone());
        match parse_sections(&text) {
            Ok(sections) => {
                for required in required_headings(kind) {
                    if !sections.contains_key(*required) {
                        diagnostics.push(diag(
                            Some(kind),
                            "required_heading_missing",
                            &format!("required heading is missing: {required}"),
                        ));
                    }
                }
                authored.insert(kind.to_string(), sections);
            }
            Err(error) => diagnostics.push(diag(Some(kind), "markdown_ambiguous", &error.message)),
        }
    }
    let sunset = request
        .default_cutover_unix_seconds
        .checked_add(30 * 24 * 60 * 60)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "cutover sunset overflow"))?;
    let source_digest = format!("blake3:{}", source.finalize().to_hex());
    if !diagnostics.is_empty() {
        return Ok(ImportReport {
            schema: "csdlc.legacy_import_report.v1".into(),
            status: ImportStatus::Unsupported,
            issue: request.issue,
            source_digest: None,
            retained_section_count: authored.values().map(BTreeMap::len).sum(),
            diagnostics,
            compatibility_view: None,
            sunset_unix_seconds: sunset,
        });
    }
    let initial = match build_initial(&request, &authored) {
        Ok(initial) => initial,
        Err(error) => {
            return Ok(unsupported(
                &request,
                sunset,
                Some(source_digest),
                authored.values().map(BTreeMap::len).sum(),
                vec![diag(None, "typed_values_unrepresentable", &error.message)],
            ));
        }
    };
    let design = request.output_root.join(&request.design_path);
    let diagram = request.output_root.join(&request.diagram_path);
    let safe_output_path = |value: &str| {
        !value.is_empty()
            && std::path::Path::new(value)
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
    };
    if !safe_output_path(&request.design_path)
        || !safe_output_path(&request.diagram_path)
        || !design.is_file()
        || !diagram.is_file()
    {
        return Ok(unsupported(
            &request,
            sunset,
            Some(source_digest),
            authored.values().map(BTreeMap::len).sum(),
            vec![diag(
                None,
                "design_or_diagram_unrepresentable",
                "safe existing output design and diagram files are required before import",
            )],
        ));
    }
    let design_digest = crate::cards::digest(&fs::read(&design)?);
    let diagram_digest = crate::cards::digest(&fs::read(&diagram)?);
    if let Err(error) = crate::cards::initial_cards(
        request.issue,
        &request.repository,
        &request.design_path,
        &design_digest,
        &request.diagram_path,
        &diagram_digest,
        initial.clone(),
    ) {
        return Ok(unsupported(
            &request,
            sunset,
            Some(source_digest),
            authored.values().map(BTreeMap::len).sum(),
            vec![diag(None, "typed_cards_unrepresentable", &error.message)],
        ));
    }
    let store = Store::new(&request.output_root);
    let mut record = initialize_issue(
        &store,
        BootstrapRequest {
            issue: request.issue,
            repository: request.repository.clone(),
            design_path: request.design_path.clone(),
            diagram_path: request.diagram_path.clone(),
            design_reviewer: request.design_reviewer,
            design_approved: true,
            actor: request.actor,
            initial,
        },
    )?;
    let compatibility_path = format!(".csdlc/compat/{}.md", request.issue);
    let compatibility = render_legacy_archive(request.issue, &authored_sources);
    record = store.commit_migration(
        request.issue,
        &record.digest,
        MigrationEvidence {
            schema: "csdlc.migration_evidence.v1".into(),
            imported_unix_seconds: request.imported_unix_seconds,
            sunset_unix_seconds: sunset,
            source_digest: source_digest.clone(),
            authored_sources: authored_sources.clone(),
            authored_sections: authored.clone(),
            compatibility_view: compatibility_path.clone(),
        },
    )?;
    if request.legacy_phase == LifecyclePhase::Ready && record.phase == LifecyclePhase::Initialized
    {
        let _ = edit_issue(
            &store,
            EditRequest {
                issue: request.issue,
                card: CardKind::Sip,
                expected_generation: record.generation,
                expected_digest: record.digest,
                actor: "csdlc-import".into(),
                reason: "normalize supported legacy ready outcome".into(),
                operation: SemanticOperation::AdvancePhase {
                    phase: LifecyclePhase::Ready,
                },
                fail_after_backup: false,
            },
        )?;
    }
    write_compatibility_view_atomic(&store, request.issue, &compatibility)?;
    Ok(ImportReport {
        schema: "csdlc.legacy_import_report.v1".into(),
        status: ImportStatus::Imported,
        issue: request.issue,
        source_digest: Some(source_digest),
        retained_section_count: authored.values().map(BTreeMap::len).sum(),
        diagnostics: Vec::new(),
        compatibility_view: Some(compatibility_path),
        sunset_unix_seconds: sunset,
    })
}

fn validate_request(request: &LegacyImportRequest) -> Result<()> {
    if request.schema != "csdlc.legacy_import_request.v1"
        || request.issue == 0
        || !matches!(
            request.legacy_phase,
            LifecyclePhase::Initialized | LifecyclePhase::Ready
        )
        || request.validation_lanes.is_empty()
        || request.imported_unix_seconds == 0
        || request.default_cutover_unix_seconds == 0
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy import request is invalid or outside the bounded phase set",
        ));
    }
    let legacy = request.legacy_root.canonicalize()?;
    let output = request.output_root.canonicalize()?;
    if legacy == output || legacy.starts_with(&output) || output.starts_with(&legacy) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy and output roots must be canonical and disjoint",
        ));
    }
    let sunset = request
        .default_cutover_unix_seconds
        .checked_add(30 * 24 * 60 * 60)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "cutover sunset overflow"))?;
    if request.imported_unix_seconds > sunset {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy importer expired 30 days after default cutover",
        ));
    }
    Ok(())
}

fn parse_sections(text: &str) -> Result<BTreeMap<String, String>> {
    let ast = to_mdast(text, &ParseOptions::gfm())
        .map_err(|message| V2Error::new(ErrorCode::InvalidInput, message.to_string()))?;
    let children = ast
        .children()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "mdast root is absent"))?;
    let mut anchors = Vec::new();
    for child in children {
        if let Node::Heading(heading) = child {
            if heading.depth == 2 {
                let position = child.position().ok_or_else(|| {
                    V2Error::new(ErrorCode::InvalidInput, "heading source position is absent")
                })?;
                anchors.push((
                    child.to_string().trim().to_owned(),
                    position.start.offset,
                    position.end.offset,
                ));
            }
        }
    }
    let mut seen = BTreeSet::new();
    if anchors.is_empty()
        || anchors
            .iter()
            .any(|(name, _, _)| name.is_empty() || !seen.insert(name.clone()))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "level-two headings are missing, empty, or duplicated",
        ));
    }
    let mut sections = BTreeMap::new();
    for (index, (name, _, end)) in anchors.iter().enumerate() {
        let next = anchors
            .get(index + 1)
            .map_or(text.len(), |(_, start, _)| *start);
        let raw = text
            .get(*end..next)
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "heading offsets are invalid"))?;
        if raw.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                format!("authored section is empty: {name}"),
            ));
        }
        sections.insert(name.clone(), raw.to_owned());
    }
    Ok(sections)
}

fn build_initial(
    request: &LegacyImportRequest,
    authored: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<InitialCardInput> {
    let get = |kind: CardKind, heading: &str| -> Result<String> {
        authored
            .get(&kind.to_string())
            .and_then(|sections| sections.get(heading))
            .cloned()
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::InvalidInput,
                    format!("{kind} {heading} is absent"),
                )
            })
    };
    let optional = |kind: CardKind, heading: &str| -> Option<String> {
        authored
            .get(&kind.to_string())
            .and_then(|sections| sections.get(heading))
            .cloned()
    };
    let acceptance_criteria = list(&get(CardKind::Stp, "Acceptance Criteria")?);
    let acceptance_ids: Vec<_> = (1..=acceptance_criteria.len())
        .map(|index| format!("AC-{index}"))
        .collect();
    let steps: Vec<_> = list(&get(CardKind::Spp, "Plan")?)
        .into_iter()
        .enumerate()
        .map(|(index, action)| PlanStep {
            id: format!("imported-{}", index + 1),
            action,
            acceptance_ids: acceptance_ids.clone(),
            status: StepStatus::Pending,
        })
        .collect();
    if steps.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy Plan has no unambiguous list items",
        ));
    }
    Ok(InitialCardInput {
        title: request.title.clone(),
        slug: request.slug.clone(),
        version: request.version.clone(),
        goal: plain(&get(CardKind::Sip, "Goal")?),
        required_outcome: plain(&get(CardKind::Stp, "Required Outcome")?),
        declared_scope: list(&get(CardKind::Sip, "Scope")?),
        authority_boundary: list(&get(CardKind::Sip, "Authority")?),
        operator_constraints: optional(CardKind::Sip, "Operator Constraints")
            .map(|value| list(&value))
            .filter(|values| !values.is_empty())
            .unwrap_or_else(|| vec!["none".into()]),
        task_boundary: plain(&get(CardKind::Stp, "Summary")?),
        deliverables: list(&get(CardKind::Stp, "Deliverables")?),
        acceptance_criteria,
        dependencies: vec!["imported legacy observation".into()],
        repo_inputs: vec!["one-way imported authored cards".into()],
        non_goals: vec!["legacy implementation reuse".into()],
        plan_summary: plain(&get(CardKind::Spp, "Plan")?),
        steps,
        affected_areas: vec![format!(".csdlc/issues/{}", request.issue)],
        invariants: list(&get(CardKind::Spp, "Invariants")?),
        risks: list(&get(CardKind::Spp, "Risks")?),
        planning_profile: request.planning_profile,
        stop_conditions: list(&get(CardKind::Spp, "Stop Conditions")?),
        validation_lanes: request
            .validation_lanes
            .iter()
            .cloned()
            .map(|mut lane| {
                lane.acceptance_ids = acceptance_ids.clone();
                lane
            })
            .collect(),
        failure_policy: plain(&get(CardKind::Vpp, "Failure Policy")?),
        review_prompts: list(&get(CardKind::Srp, "Prompts")?),
        review_scope: plain(&get(CardKind::Srp, "Review Scope")?),
    })
}

fn list(raw: &str) -> Vec<String> {
    raw.lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("- ")
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
        })
        .collect()
}
fn plain(raw: &str) -> String {
    let items = list(raw);
    if items.is_empty() {
        raw.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        items.join("; ")
    }
}
fn diag(card: Option<CardKind>, code: &str, message: &str) -> MigrationDiagnostic {
    MigrationDiagnostic {
        card,
        code: code.into(),
        message: message.into(),
    }
}

fn unsupported(
    request: &LegacyImportRequest,
    sunset_unix_seconds: u64,
    source_digest: Option<String>,
    retained_section_count: usize,
    diagnostics: Vec<MigrationDiagnostic>,
) -> ImportReport {
    ImportReport {
        schema: "csdlc.legacy_import_report.v1".into(),
        status: ImportStatus::Unsupported,
        issue: request.issue,
        source_digest,
        retained_section_count,
        diagnostics,
        compatibility_view: None,
        sunset_unix_seconds,
    }
}
fn enum_cards() -> [CardKind; 6] {
    [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ]
}
fn required_headings(kind: CardKind) -> &'static [&'static str] {
    match kind {
        CardKind::Sip => &["Goal", "Scope", "Authority"],
        CardKind::Stp => &[
            "Summary",
            "Required Outcome",
            "Deliverables",
            "Acceptance Criteria",
        ],
        CardKind::Spp => &["Plan", "Invariants", "Risks", "Stop Conditions"],
        CardKind::Vpp => &["Validation", "Failure Policy"],
        CardKind::Srp => &["Review Scope", "Prompts"],
        CardKind::Sor => &[
            "Summary",
            "Artifacts",
            "Execution",
            "Integration",
            "Publication",
            "Closeout",
            "Follow Ups",
        ],
    }
}
fn render_legacy_archive(issue: u64, authored: &BTreeMap<String, String>) -> String {
    let mut out = format!("# Generated compatibility view for issue {issue}\n\nGenerated from canonical migration evidence. Do not edit.\n");
    for (card, source) in authored {
        out.push_str(&format!("\n<!-- BEGIN exact legacy source: {card} -->\n"));
        out.push_str(source);
        if !source.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(&format!("<!-- END exact legacy source: {card} -->\n"));
    }
    out
}

pub fn generate_compatibility_view(store: &Store, issue: u64) -> Result<String> {
    let record = store.load_record(issue)?;
    let migration = record
        .migration
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "issue has no migration evidence"))?;
    Ok(render_legacy_archive(issue, &migration.authored_sources))
}

pub fn write_compatibility_view_atomic(store: &Store, issue: u64, view: &str) -> Result<String> {
    let directory = store.root().join(".csdlc/compat");
    fs::create_dir_all(&directory)?;
    let target = directory.join(format!("{issue}.md"));
    let temporary = directory.join(format!(".{issue}.tmp"));
    fs::write(&temporary, view)?;
    fs::rename(&temporary, &target)?;
    Ok(format!(".csdlc/compat/{issue}.md"))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct NormalizedOutcome {
    pub schema: String,
    pub issue: u64,
    pub phase: LifecyclePhase,
    pub card_statuses: BTreeMap<CardKind, CardStatus>,
    pub review_result: String,
    pub integration_state: String,
    pub publication_state: String,
    pub merge_state: String,
    pub closeout_state: String,
    pub doctor_status: String,
    pub doctor_findings: Vec<String>,
}

impl NormalizedOutcome {
    pub fn from_v2(store: &Store, issue: u64) -> Result<Self> {
        let record = store.load_record(issue)?;
        let cards = store.load_cards(issue)?;
        let doctor = crate::diagnose(store, issue);
        let mut doctor_findings: Vec<_> = doctor
            .findings
            .into_iter()
            .map(|finding| finding.code)
            .collect();
        doctor_findings.sort();
        let statuses = cards
            .iter()
            .map(|(kind, values)| (*kind, values.status))
            .collect();
        let srp = match &cards[&CardKind::Srp].content {
            CardContent::Srp(value) => value,
            _ => unreachable!(),
        };
        let sor = match &cards[&CardKind::Sor].content {
            CardContent::Sor(value) => value,
            _ => unreachable!(),
        };
        Ok(Self {
            schema: "csdlc.normalized_outcome.v1".into(),
            issue,
            phase: record.phase,
            card_statuses: statuses,
            review_result: srp.review_result.to_string(),
            integration_state: sor.integration_state.to_string(),
            publication_state: sor.publication_state.to_string(),
            merge_state: sor.merge_state.to_string(),
            closeout_state: sor.closeout_state.to_string(),
            doctor_status: doctor.status.to_string(),
            doctor_findings,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ShadowComparison {
    pub schema: String,
    pub equivalent: bool,
    pub differences: Vec<String>,
}

pub fn compare_shadow(legacy: &NormalizedOutcome, v2: &NormalizedOutcome) -> ShadowComparison {
    let mut differences = Vec::new();
    if legacy.issue != v2.issue {
        differences.push("issue".into());
    }
    if legacy.phase != v2.phase {
        differences.push("phase".into());
    }
    if legacy.card_statuses != v2.card_statuses {
        differences.push("card_statuses".into());
    }
    if legacy.review_result != v2.review_result {
        differences.push("review_result".into());
    }
    if legacy.integration_state != v2.integration_state {
        differences.push("integration_state".into());
    }
    if legacy.publication_state != v2.publication_state {
        differences.push("publication_state".into());
    }
    if legacy.merge_state != v2.merge_state {
        differences.push("merge_state".into());
    }
    if legacy.closeout_state != v2.closeout_state {
        differences.push("closeout_state".into());
    }
    if legacy.doctor_status != v2.doctor_status {
        differences.push("doctor_status".into());
    }
    if legacy.doctor_findings != v2.doctor_findings {
        differences.push("doctor_findings".into());
    }
    ShadowComparison {
        schema: "csdlc.shadow_comparison.v1".into(),
        equivalent: differences.is_empty(),
        differences,
    }
}
