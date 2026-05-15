use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

pub const RUST_NATIVE_GWS_ADAPTER_BOUNDARY_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/google_workspace_cms_bridge/rust_native_gws_adapter_boundary_report.json";
pub const WORKSPACE_CMS_SNAPSHOT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_cms_snapshot.json";
pub const WORKSPACE_PROMOTION_PACKET_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_promotion_packet.md";
pub const RUST_NATIVE_GWS_ADAPTER_BOUNDARY_SCHEMA_VERSION: &str =
    "rust_native_gws_adapter_boundary.v1";
pub const RUST_NATIVE_GWS_ADAPTER_BOUNDARY_PROMPT_VERSION: &str =
    "wp09.rust_native_gws_adapter_boundary.v3";
#[cfg(test)]
const HOST_PATH_MARKER: &str = "/Users/daniel/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RustNativeGwsAdapterBoundaryPromptRecord {
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub depends_on_issue_number: u32,
    pub boundary_summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceFixtureMode {
    Fixture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceAuthority {
    DraftOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceContentType {
    Doc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceContentStatus {
    ReadyForRepoPromotion,
    Blocked,
    PromotionPacketPrepared,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceReadiness {
    BoundedPromotionPacketReady,
    BlockedRevisionMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceExecutionMode {
    FixtureBacked,
    FixtureBackedOnly,
    LiveGated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceOperationResult {
    Proving,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCmsRootRef {
    pub display_name: String,
    pub folder_ref: String,
    pub authority: WorkspaceAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceContentCard {
    pub doc_id: String,
    pub title: String,
    pub source_url: String,
    pub workspace_folder: String,
    pub content_type: WorkspaceContentType,
    pub status: WorkspaceContentStatus,
    pub owner: String,
    pub target_repo_path: String,
    pub readiness: WorkspaceReadiness,
    pub blockers: Vec<String>,
    pub last_reviewed: String,
    pub github_issue: Option<u32>,
    pub promotion_pr: Option<String>,
    pub revision_anchor: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCmsFixture {
    pub schema_version: String,
    pub mode: WorkspaceFixtureMode,
    pub cms_root: WorkspaceCmsRootRef,
    pub content_cards: Vec<WorkspaceContentCard>,
    pub live_gated_capabilities: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceTypedContractRecord {
    pub contract_name: &'static str,
    pub purpose: &'static str,
    pub carries_repo_truth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceDocSnapshot {
    pub doc_id: String,
    pub title: String,
    pub source_url: String,
    pub revision_anchor: String,
    pub readiness: WorkspaceReadiness,
    pub status: WorkspaceContentStatus,
    pub note_excerpt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspacePromotionPacket {
    pub doc_id: String,
    pub title: String,
    pub target_repo_path: String,
    pub revision_anchor: String,
    pub canonical_authority: &'static str,
    pub tracked_packet_consistent: bool,
    pub required_checks: Vec<&'static str>,
    pub stop_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceContentCardUpdatePreview {
    pub doc_id: String,
    pub previous_status: WorkspaceContentStatus,
    pub next_status: WorkspaceContentStatus,
    pub previous_promotion_pr: Option<String>,
    pub next_promotion_pr: Option<String>,
    pub issue_number: u32,
    pub live_write_requires_operator_approval: bool,
    pub can_apply_to_fixture: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceContentCardUpdateApplyResult {
    pub doc_id: String,
    pub applied_to_fixture: bool,
    pub resulting_status: WorkspaceContentStatus,
    pub resulting_promotion_pr: Option<String>,
    pub persisted_to_live_workspace: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceCmsUpdateOutcome {
    pub updated_fixture: WorkspaceCmsFixture,
    pub apply_result: WorkspaceContentCardUpdateApplyResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceCapabilityRecord {
    pub capability_name: &'static str,
    pub operation: &'static str,
    pub execution_mode: WorkspaceExecutionMode,
    pub result: WorkspaceOperationResult,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceCredentialBoundaryRecord {
    pub secrets_required_for_fixture_validation: bool,
    pub read_scope_summary: Vec<&'static str>,
    pub write_scope_summary: Vec<&'static str>,
    pub prohibited_default_behaviors: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceAdapterDecision {
    ImplementFixtureFirstCmsBoundary,
    DeferLiveWritesUntilGovernedToolsV2,
    NoGo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceAdapterRecommendationRecord {
    pub decision: WorkspaceAdapterDecision,
    pub summary: &'static str,
    pub rationale: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RustNativeGwsAdapterBoundaryReport {
    pub schema_version: &'static str,
    pub prompt_record: RustNativeGwsAdapterBoundaryPromptRecord,
    pub snapshot_artifact_path: &'static str,
    pub promotion_packet_artifact_path: &'static str,
    pub cms_root: WorkspaceCmsRootRef,
    pub content_cards: Vec<WorkspaceContentCard>,
    pub typed_contracts: Vec<WorkspaceTypedContractRecord>,
    pub doc_snapshot_demo: WorkspaceDocSnapshot,
    pub promotion_packet_demo: WorkspacePromotionPacket,
    pub content_card_update_preview: WorkspaceContentCardUpdatePreview,
    pub fixture_apply_result: WorkspaceContentCardUpdateApplyResult,
    pub capability_results: Vec<WorkspaceCapabilityRecord>,
    pub credential_boundary: WorkspaceCredentialBoundaryRecord,
    pub recommendation: WorkspaceAdapterRecommendationRecord,
    pub non_claims: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrackedPromotionPacketContract {
    title: String,
    revision_anchor: String,
    target_repo_path: String,
}

fn prompt_record() -> RustNativeGwsAdapterBoundaryPromptRecord {
    RustNativeGwsAdapterBoundaryPromptRecord {
        prompt_version: RUST_NATIVE_GWS_ADAPTER_BOUNDARY_PROMPT_VERSION,
        issue_number: 3008,
        depends_on_issue_number: 3007,
        boundary_summary:
            "WP-09 exposes callable fixture-first native Workspace CMS operations for inventory, snapshot, promotion, preview, and bounded apply flows while keeping live authenticated writes gated.",
    }
}

fn tracked_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(relative)
}

pub fn load_workspace_cms_fixture() -> Result<WorkspaceCmsFixture> {
    let body = fs::read_to_string(tracked_path(WORKSPACE_CMS_SNAPSHOT_ARTIFACT_PATH))
        .with_context(|| format!("read '{}'", WORKSPACE_CMS_SNAPSHOT_ARTIFACT_PATH))?;
    serde_json::from_str(&body)
        .with_context(|| format!("parse '{}'", WORKSPACE_CMS_SNAPSHOT_ARTIFACT_PATH))
}

fn read_tracked_promotion_packet() -> Result<String> {
    fs::read_to_string(tracked_path(WORKSPACE_PROMOTION_PACKET_ARTIFACT_PATH))
        .with_context(|| format!("read '{}'", WORKSPACE_PROMOTION_PACKET_ARTIFACT_PATH))
}

fn parse_backticked_value(line: &str) -> Option<String> {
    let start = line.find('`')?;
    let end = line[start + 1..].find('`')? + start + 1;
    Some(line[start + 1..end].to_string())
}

fn parse_tracked_promotion_packet_contract(body: &str) -> Result<TrackedPromotionPacketContract> {
    let mut title = None;
    let mut revision_anchor = None;
    let mut target_repo_path = None;
    let mut expect_target_path = false;

    for raw_line in body.lines() {
        let line = raw_line.trim();
        if line.starts_with("- Title:") {
            title = parse_backticked_value(line);
        } else if line.starts_with("- Revision anchor:") {
            revision_anchor = parse_backticked_value(line);
        } else if line.starts_with("- Target repo path:") {
            expect_target_path = true;
        } else if expect_target_path && !line.is_empty() {
            target_repo_path = parse_backticked_value(line);
            expect_target_path = false;
        }
    }

    Ok(TrackedPromotionPacketContract {
        title: title.ok_or_else(|| anyhow!("missing promotion packet title"))?,
        revision_anchor: revision_anchor
            .ok_or_else(|| anyhow!("missing promotion packet revision anchor"))?,
        target_repo_path: target_repo_path
            .ok_or_else(|| anyhow!("missing promotion packet target repo path"))?,
    })
}

pub fn inventory_workspace_cms_root(fixture: &WorkspaceCmsFixture) -> WorkspaceCmsRootRef {
    fixture.cms_root.clone()
}

pub fn read_workspace_content_cards(fixture: &WorkspaceCmsFixture) -> Vec<WorkspaceContentCard> {
    fixture.content_cards.clone()
}

fn find_card<'a>(
    fixture: &'a WorkspaceCmsFixture,
    doc_id: &str,
) -> Result<&'a WorkspaceContentCard> {
    fixture
        .content_cards
        .iter()
        .find(|card| card.doc_id == doc_id)
        .ok_or_else(|| anyhow!("workspace content card not found: {doc_id}"))
}

fn ready_card(fixture: &WorkspaceCmsFixture) -> Result<&WorkspaceContentCard> {
    fixture
        .content_cards
        .iter()
        .find(|card| card.readiness == WorkspaceReadiness::BoundedPromotionPacketReady)
        .ok_or_else(|| anyhow!("fixture should contain one ready-for-promotion card"))
}

pub fn read_workspace_doc_snapshot(
    fixture: &WorkspaceCmsFixture,
    doc_id: &str,
) -> Result<WorkspaceDocSnapshot> {
    let card = find_card(fixture, doc_id)?;
    Ok(WorkspaceDocSnapshot {
        doc_id: card.doc_id.clone(),
        title: card.title.clone(),
        source_url: card.source_url.clone(),
        revision_anchor: card.revision_anchor.clone(),
        readiness: card.readiness.clone(),
        status: card.status.clone(),
        note_excerpt: card.notes.clone(),
    })
}

pub fn prepare_workspace_promotion_packet(
    fixture: &WorkspaceCmsFixture,
    doc_id: &str,
) -> Result<WorkspacePromotionPacket> {
    let card = find_card(fixture, doc_id)?;
    let tracked_contract =
        parse_tracked_promotion_packet_contract(&read_tracked_promotion_packet()?)?;
    Ok(WorkspacePromotionPacket {
        doc_id: card.doc_id.clone(),
        title: card.title.clone(),
        target_repo_path: card.target_repo_path.clone(),
        revision_anchor: card.revision_anchor.clone(),
        canonical_authority: "Git + PR review + merge history",
        tracked_packet_consistent: tracked_contract.title == card.title
            && tracked_contract.revision_anchor == card.revision_anchor
            && tracked_contract.target_repo_path == card.target_repo_path,
        required_checks: vec![
            "confirm the Workspace source is still at the recorded revision anchor",
            "confirm the target repo path is correct and still in scope",
            "confirm promotion remains issue-backed and PR-reviewed",
        ],
        stop_boundary: "stop before editing tracked repository files directly from Workspace state",
    })
}

pub fn preview_workspace_content_card_update(
    fixture: &WorkspaceCmsFixture,
    doc_id: &str,
) -> Result<WorkspaceContentCardUpdatePreview> {
    let card = find_card(fixture, doc_id)?;
    Ok(WorkspaceContentCardUpdatePreview {
        doc_id: card.doc_id.clone(),
        previous_status: card.status.clone(),
        next_status: WorkspaceContentStatus::PromotionPacketPrepared,
        previous_promotion_pr: card.promotion_pr.clone(),
        next_promotion_pr: Some("pending://issue-3008/native-cms-boundary".to_string()),
        issue_number: 3008,
        live_write_requires_operator_approval: true,
        can_apply_to_fixture: true,
    })
}

pub fn apply_workspace_content_card_update(
    fixture: &WorkspaceCmsFixture,
    preview: &WorkspaceContentCardUpdatePreview,
) -> Result<WorkspaceCmsUpdateOutcome> {
    let mut updated_fixture = fixture.clone();
    let mut updated = false;

    for card in &mut updated_fixture.content_cards {
        if card.doc_id == preview.doc_id {
            card.status = preview.next_status.clone();
            card.promotion_pr = preview.next_promotion_pr.clone();
            updated = true;
            break;
        }
    }

    if !updated {
        bail!(
            "workspace content card not found for apply: {}",
            preview.doc_id
        );
    }

    Ok(WorkspaceCmsUpdateOutcome {
        updated_fixture,
        apply_result: WorkspaceContentCardUpdateApplyResult {
            doc_id: preview.doc_id.clone(),
            applied_to_fixture: true,
            resulting_status: preview.next_status.clone(),
            resulting_promotion_pr: preview.next_promotion_pr.clone(),
            persisted_to_live_workspace: false,
        },
    })
}

fn typed_contracts() -> Vec<WorkspaceTypedContractRecord> {
    vec![
        WorkspaceTypedContractRecord {
            contract_name: "WorkspaceCmsRootRef",
            purpose: "name one bounded Drive-rooted CMS inventory surface",
            carries_repo_truth: false,
        },
        WorkspaceTypedContractRecord {
            contract_name: "WorkspaceContentCard",
            purpose: "normalize one draft lifecycle row for operator review and promotion",
            carries_repo_truth: false,
        },
        WorkspaceTypedContractRecord {
            contract_name: "WorkspaceDocSnapshot",
            purpose: "capture one bounded reviewable document snapshot without exporting live secrets",
            carries_repo_truth: false,
        },
        WorkspaceTypedContractRecord {
            contract_name: "WorkspacePromotionPacket",
            purpose: "prepare one Git-backed promotion handoff without directly mutating tracked files",
            carries_repo_truth: false,
        },
        WorkspaceTypedContractRecord {
            contract_name: "WorkspaceContentCardUpdatePreview",
            purpose: "preview one content-card state transition before any live write",
            carries_repo_truth: false,
        },
        WorkspaceTypedContractRecord {
            contract_name: "WorkspaceCmsUpdateOutcome",
            purpose: "apply one bounded fixture-backed content-card update while keeping live writes gated",
            carries_repo_truth: false,
        },
    ]
}

fn capability_results(
    fixture: &WorkspaceCmsFixture,
    doc_snapshot: &WorkspaceDocSnapshot,
    promotion_packet: &WorkspacePromotionPacket,
    preview: &WorkspaceContentCardUpdatePreview,
    apply_result: &WorkspaceContentCardUpdateApplyResult,
) -> Vec<WorkspaceCapabilityRecord> {
    vec![
        WorkspaceCapabilityRecord {
            capability_name: "workspace.drive.list_folder",
            operation: "folder_inventory",
            execution_mode: WorkspaceExecutionMode::FixtureBacked,
            result: WorkspaceOperationResult::Proving,
            summary: format!(
                "Inventoried '{}' with {} bounded content cards.",
                fixture.cms_root.folder_ref,
                fixture.content_cards.len()
            ),
        },
        WorkspaceCapabilityRecord {
            capability_name: "workspace.docs.read_snapshot",
            operation: "read_doc_snapshot",
            execution_mode: WorkspaceExecutionMode::FixtureBacked,
            result: WorkspaceOperationResult::Proving,
            summary: format!(
                "Read callable fixture snapshot for '{}' at revision '{}'.",
                doc_snapshot.title, doc_snapshot.revision_anchor
            ),
        },
        WorkspaceCapabilityRecord {
            capability_name: "workspace.sheets.read_content_cards",
            operation: "read_content_cards",
            execution_mode: WorkspaceExecutionMode::FixtureBacked,
            result: WorkspaceOperationResult::Proving,
            summary:
                "Loaded the tracked content-card set through the native fixture API without requiring live Workspace access."
                    .to_string(),
        },
        WorkspaceCapabilityRecord {
            capability_name: "workspace.promotion.prepare_packet",
            operation: "prepare_promotion_packet",
            execution_mode: WorkspaceExecutionMode::FixtureBacked,
            result: WorkspaceOperationResult::Proving,
            summary: format!(
                "Prepared a Git-backed promotion handoff for '{}' with exact tracked packet consistency={}.",
                promotion_packet.title, promotion_packet.tracked_packet_consistent
            ),
        },
        WorkspaceCapabilityRecord {
            capability_name: "workspace.sheets.preview_content_card_update",
            operation: "preview_content_card_update",
            execution_mode: WorkspaceExecutionMode::FixtureBacked,
            result: WorkspaceOperationResult::Proving,
            summary: format!(
                "Previewed callable status transition '{:?}' -> '{:?}'.",
                preview.previous_status, preview.next_status
            ),
        },
        WorkspaceCapabilityRecord {
            capability_name: "workspace.sheets.apply_content_card_update",
            operation: "apply_content_card_update",
            execution_mode: WorkspaceExecutionMode::FixtureBackedOnly,
            result: WorkspaceOperationResult::Proving,
            summary: format!(
                "Applied callable fixture-only update for '{}' with live persistence={}.",
                apply_result.doc_id, apply_result.persisted_to_live_workspace
            ),
        },
        WorkspaceCapabilityRecord {
            capability_name: "workspace.sheets.apply_content_card_update",
            operation: "live_write_boundary",
            execution_mode: WorkspaceExecutionMode::LiveGated,
            result: WorkspaceOperationResult::Skipped,
            summary:
                "Live Workspace writes remain gated behind explicit operator approval and governed-tools semantics."
                    .to_string(),
        },
    ]
}

fn credential_boundary() -> WorkspaceCredentialBoundaryRecord {
    WorkspaceCredentialBoundaryRecord {
        secrets_required_for_fixture_validation: false,
        read_scope_summary: vec![
            "bounded Drive metadata read scope",
            "single-document snapshot read scope",
            "single-range content-card read scope",
        ],
        write_scope_summary: vec![
            "single-range content-card write scope only after preview and operator approval",
            "no ambient broad Workspace authority",
        ],
        prohibited_default_behaviors: vec![
            "No live Workspace write during fixture validation.",
            "No direct tracked repo edits from Workspace state.",
            "No private document body export into public artifacts by default.",
        ],
    }
}

fn recommendation() -> WorkspaceAdapterRecommendationRecord {
    WorkspaceAdapterRecommendationRecord {
        decision: WorkspaceAdapterDecision::ImplementFixtureFirstCmsBoundary,
        summary:
            "Implement the callable typed fixture-first CMS boundary now, and keep live secret-bearing Workspace writes deferred behind governed-tools semantics.",
        rationale: vec![
            "WP-09 now exposes callable native operations for folder inventory, document snapshot, content-card reads, promotion packet preparation, preview, and bounded fixture apply.",
            "This gives ADL a bounded CMS capability for draft lifecycle management without promoting Workspace to canonical truth.",
            "Live external writes remain the only intentionally deferred part of the CMS capability surface.",
        ],
    }
}

pub fn run_rust_native_gws_adapter_boundary_report() -> RustNativeGwsAdapterBoundaryReport {
    let fixture = load_workspace_cms_fixture().expect("tracked Workspace CMS fixture should load");
    let ready =
        ready_card(&fixture).expect("tracked Workspace CMS fixture should have a ready card");
    let doc_snapshot =
        read_workspace_doc_snapshot(&fixture, &ready.doc_id).expect("read ready doc snapshot");
    let promotion_packet = prepare_workspace_promotion_packet(&fixture, &ready.doc_id)
        .expect("prepare tracked promotion packet");
    let preview =
        preview_workspace_content_card_update(&fixture, &ready.doc_id).expect("preview update");
    let update_outcome =
        apply_workspace_content_card_update(&fixture, &preview).expect("apply fixture update");
    let capability_results = capability_results(
        &fixture,
        &doc_snapshot,
        &promotion_packet,
        &preview,
        &update_outcome.apply_result,
    );

    RustNativeGwsAdapterBoundaryReport {
        schema_version: RUST_NATIVE_GWS_ADAPTER_BOUNDARY_SCHEMA_VERSION,
        prompt_record: prompt_record(),
        snapshot_artifact_path: WORKSPACE_CMS_SNAPSHOT_ARTIFACT_PATH,
        promotion_packet_artifact_path: WORKSPACE_PROMOTION_PACKET_ARTIFACT_PATH,
        cms_root: inventory_workspace_cms_root(&fixture),
        content_cards: read_workspace_content_cards(&fixture),
        typed_contracts: typed_contracts(),
        doc_snapshot_demo: doc_snapshot,
        promotion_packet_demo: promotion_packet,
        content_card_update_preview: preview,
        fixture_apply_result: update_outcome.apply_result,
        capability_results,
        credential_boundary: credential_boundary(),
        recommendation: recommendation(),
        non_claims: vec![
            "This report does not prove live authenticated Google Workspace execution.",
            "This report does not make Google Workspace canonical repo truth.",
            "This report does not authorize direct tracked repository mutation from Workspace state.",
        ],
    }
}

pub fn write_rust_native_gws_adapter_boundary_report(
    path: impl AsRef<Path>,
) -> Result<RustNativeGwsAdapterBoundaryReport> {
    let report = run_rust_native_gws_adapter_boundary_report();
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent directories for '{}'", path.display()))?;
    }
    let body = serde_json::to_string_pretty(&report)?;
    fs::write(path, body).with_context(|| format!("write '{}'", path.display()))?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::{
        apply_workspace_content_card_update, load_workspace_cms_fixture,
        parse_tracked_promotion_packet_contract, prepare_workspace_promotion_packet,
        preview_workspace_content_card_update, read_workspace_content_cards,
        read_workspace_doc_snapshot, run_rust_native_gws_adapter_boundary_report,
        write_rust_native_gws_adapter_boundary_report, WorkspaceAdapterDecision,
        WorkspaceContentStatus, WorkspaceReadiness, HOST_PATH_MARKER,
        RUST_NATIVE_GWS_ADAPTER_BOUNDARY_REPORT_ARTIFACT_PATH,
        RUST_NATIVE_GWS_ADAPTER_BOUNDARY_SCHEMA_VERSION,
    };
    use std::fs;
    use std::io;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn rust_native_gws_adapter_boundary_exposes_callable_fixture_first_cms_operations() {
        let fixture = load_workspace_cms_fixture().expect("load fixture");
        assert_eq!(read_workspace_content_cards(&fixture).len(), 2);
        let ready = fixture
            .content_cards
            .iter()
            .find(|card| card.readiness == WorkspaceReadiness::BoundedPromotionPacketReady)
            .expect("ready card");
        let snapshot = read_workspace_doc_snapshot(&fixture, &ready.doc_id).expect("read snapshot");
        let promotion =
            prepare_workspace_promotion_packet(&fixture, &ready.doc_id).expect("prepare packet");
        let preview =
            preview_workspace_content_card_update(&fixture, &ready.doc_id).expect("preview update");
        let outcome =
            apply_workspace_content_card_update(&fixture, &preview).expect("apply update");

        assert_eq!(snapshot.doc_id, ready.doc_id);
        assert!(promotion.tracked_packet_consistent);
        assert_eq!(
            preview.next_status,
            WorkspaceContentStatus::PromotionPacketPrepared
        );
        assert_eq!(
            outcome
                .updated_fixture
                .content_cards
                .iter()
                .find(|card| card.doc_id == ready.doc_id)
                .expect("updated card")
                .status,
            WorkspaceContentStatus::PromotionPacketPrepared
        );
    }

    #[test]
    fn rust_native_gws_adapter_boundary_uses_strong_tracked_packet_contract() {
        let parsed = parse_tracked_promotion_packet_contract(
            "# Workspace Promotion Packet Fixture\n\n- Title: `CodeFriend Review Packet Draft`\n- Revision anchor: `workspace-revision-42`\n\n- Target repo path:\n  `docs/milestones/v0.91.2/review/codefriend_productization/review_packet_workflow_package.md`\n",
        )
        .expect("parse packet contract");
        assert_eq!(parsed.title, "CodeFriend Review Packet Draft");
        assert_eq!(parsed.revision_anchor, "workspace-revision-42");
        assert_eq!(
            parsed.target_repo_path,
            "docs/milestones/v0.91.2/review/codefriend_productization/review_packet_workflow_package.md"
        );
    }

    #[test]
    fn rust_native_gws_adapter_boundary_packet_contract_rejects_missing_fields() {
        let error = parse_tracked_promotion_packet_contract("- Title: `Only title`")
            .expect_err("missing fields should fail");
        assert!(error.to_string().contains("missing promotion packet"));
    }

    #[test]
    fn rust_native_gws_adapter_boundary_report_preserves_typed_stateful_contracts() {
        let report = run_rust_native_gws_adapter_boundary_report();
        assert_eq!(
            report.recommendation.decision,
            WorkspaceAdapterDecision::ImplementFixtureFirstCmsBoundary
        );
        assert_eq!(
            report.doc_snapshot_demo.status,
            WorkspaceContentStatus::ReadyForRepoPromotion
        );
        assert_eq!(
            report.content_card_update_preview.previous_status,
            WorkspaceContentStatus::ReadyForRepoPromotion
        );
        assert_eq!(
            report.fixture_apply_result.resulting_status,
            WorkspaceContentStatus::PromotionPacketPrepared
        );
        assert!(report
            .capability_results
            .iter()
            .any(|record| record.operation == "live_write_boundary"));
    }

    #[test]
    fn rust_native_gws_adapter_boundary_serializes_deterministically_without_host_paths() {
        let first = serde_json::to_string_pretty(&run_rust_native_gws_adapter_boundary_report())
            .expect("serialize first report");
        let second = serde_json::to_string_pretty(&run_rust_native_gws_adapter_boundary_report())
            .expect("serialize second report");
        assert_eq!(first, second);
        assert!(!first.contains(HOST_PATH_MARKER));
    }

    #[test]
    fn rust_native_gws_adapter_boundary_report_writer_emits_portable_json() {
        let report_path = unique_temp_path("rust-native-gws-adapter-boundary");
        let report = write_rust_native_gws_adapter_boundary_report(&report_path)
            .expect("write adapter boundary report");
        let body = fs::read_to_string(&report_path).expect("read adapter boundary report");
        assert!(body.contains(RUST_NATIVE_GWS_ADAPTER_BOUNDARY_SCHEMA_VERSION));
        assert!(body.contains("implement_fixture_first_cms_boundary"));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&body).expect("valid json"),
            serde_json::to_value(report).expect("serialize report")
        );
        fs::remove_file(&report_path).expect("remove report");
    }

    #[test]
    fn rust_native_gws_adapter_boundary_artifact_path_is_repo_relative_and_bounded() {
        assert!(RUST_NATIVE_GWS_ADAPTER_BOUNDARY_REPORT_ARTIFACT_PATH.starts_with("docs/"));
        assert!(!RUST_NATIVE_GWS_ADAPTER_BOUNDARY_REPORT_ARTIFACT_PATH.starts_with('/'));
        assert!(!RUST_NATIVE_GWS_ADAPTER_BOUNDARY_REPORT_ARTIFACT_PATH.contains(".."));
    }

    #[test]
    fn rust_native_gws_adapter_boundary_tracked_report_matches_generated_report() {
        let tracked_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(RUST_NATIVE_GWS_ADAPTER_BOUNDARY_REPORT_ARTIFACT_PATH);
        let tracked = fs::read_to_string(tracked_path).expect("read tracked report");
        let generated =
            serde_json::to_string_pretty(&run_rust_native_gws_adapter_boundary_report())
                .expect("serialize generated report");
        assert_eq!(tracked.trim(), generated.trim());
    }

    #[test]
    fn rust_native_gws_adapter_boundary_writer_adds_context_on_failure() {
        let dir = std::env::temp_dir().join(format!(
            "rust-native-gws-adapter-boundary-dir-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be valid")
                .as_nanos()
        ));
        fs::create_dir_all(&dir).expect("create temp dir");
        let error = write_rust_native_gws_adapter_boundary_report(&dir)
            .expect_err("directory path should fail");
        let message = error.to_string();
        assert!(message.contains("write"));
        assert!(
            message.contains(&dir.display().to_string())
                || matches!(
                    error.root_cause().downcast_ref::<io::Error>(),
                    Some(io_err) if io_err.kind() == io::ErrorKind::IsADirectory
                )
        );
        fs::remove_dir_all(&dir).expect("remove temp dir");
    }
}
