use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::{env, fs, path::Path, process::Command};

use crate::gws_live_capability_execution_surface::{
    GwsCapabilityResult, GwsLiveDocRecord, GwsLiveMode, GwsLiveScopeBinding, GWS_DOC_ID_ENV,
    GWS_DRIVE_FOLDER_ID_ENV, GWS_LIVE_ENABLE_ENV, GWS_SHEET_ID_ENV, GWS_SHEET_RANGE_ENV,
};
use crate::rust_native_gws_adapter_boundary::{
    apply_workspace_content_card_update, load_workspace_cms_fixture,
    prepare_workspace_promotion_packet, preview_workspace_content_card_update,
    read_workspace_content_cards, WorkspaceContentCardUpdateApplyResult,
    WorkspaceContentCardUpdatePreview, WorkspacePromotionPacket, WorkspaceReadiness,
};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GwsCommandOutput {
    pub(super) exit_code: i32,
    pub(super) stdout: String,
    pub(super) stderr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GwsSheetUpdateConfirmation {
    pub(super) updated_range: String,
    pub(super) updated_rows: Option<u64>,
    pub(super) updated_columns: Option<u64>,
    pub(super) updated_cells: Option<u64>,
}

pub(super) trait GwsCommandRunner {
    fn run(&self, argv: &[String]) -> Result<GwsCommandOutput>;
}

struct SystemGwsCommandRunner;

impl GwsCommandRunner for SystemGwsCommandRunner {
    fn run(&self, argv: &[String]) -> Result<GwsCommandOutput> {
        let output = Command::new("gws")
            .args(argv)
            .output()
            .with_context(|| format!("run `gws {}`", argv.join(" ")))?;
        Ok(GwsCommandOutput {
            exit_code: output.status.code().unwrap_or(1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

fn prompt_record() -> GwsLiveRoundtripPromptRecord {
    GwsLiveRoundtripPromptRecord {
        prompt_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_PROMPT_VERSION,
        issue_number: 3093,
        depends_on_issue_number: 3091,
        summary:
            "WP-3093 extends the bounded live Workspace bridge into one governed content-card mutation and promotion-packet roundtrip with revision-anchor enforcement and explicit Git/PR stop boundaries.",
    }
}

pub(super) fn parse_live_mode_from_env() -> GwsLiveMode {
    match env::var(GWS_LIVE_ENABLE_ENV)
        .unwrap_or_else(|_| "disabled".to_string())
        .to_ascii_lowercase()
        .as_str()
    {
        "dry_run" | "dry-run" => GwsLiveMode::DryRun,
        "execute" | "live" | "enabled" => GwsLiveMode::Execute,
        _ => GwsLiveMode::Disabled,
    }
}

pub(super) fn parse_scope_binding_from_env() -> Option<GwsLiveScopeBinding> {
    let drive_folder_id = env::var(GWS_DRIVE_FOLDER_ID_ENV).ok()?;
    let doc_id = env::var(GWS_DOC_ID_ENV).ok()?;
    let sheet_id = env::var(GWS_SHEET_ID_ENV).ok()?;
    let sheet_range = env::var(GWS_SHEET_RANGE_ENV).ok()?;
    Some(GwsLiveScopeBinding {
        drive_folder_id,
        doc_id,
        sheet_id,
        sheet_range,
    })
}

pub(super) fn parse_write_approval_from_env() -> bool {
    matches!(
        env::var(GWS_WRITE_APPROVAL_ENV)
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "approve" | "approved"
    )
}

fn write_approval_record(
    live_mode: GwsLiveMode,
    approval_present: bool,
) -> GwsLiveWriteApprovalRecord {
    GwsLiveWriteApprovalRecord {
        approval_required: true,
        approval_checked: matches!(live_mode, GwsLiveMode::Execute),
        approval_present,
        approval_env_var: GWS_WRITE_APPROVAL_ENV,
    }
}

fn docs_get_args(scope: &GwsLiveScopeBinding) -> Vec<String> {
    vec![
        "docs".to_string(),
        "documents".to_string(),
        "get".to_string(),
        "--params".to_string(),
        serde_json::json!({ "documentId": scope.doc_id }).to_string(),
    ]
}

fn sheets_values_get_args(scope: &GwsLiveScopeBinding) -> Vec<String> {
    vec![
        "sheets".to_string(),
        "spreadsheets".to_string(),
        "values".to_string(),
        "get".to_string(),
        "--params".to_string(),
        serde_json::json!({
            "spreadsheetId": scope.sheet_id,
            "range": scope.sheet_range
        })
        .to_string(),
    ]
}

fn sheets_values_update_args(
    scope: &GwsLiveScopeBinding,
    update_range: &str,
    preview: &WorkspaceContentCardUpdatePreview,
    expected_revision_anchor: &str,
) -> Vec<String> {
    vec![
        "sheets".to_string(),
        "spreadsheets".to_string(),
        "values".to_string(),
        "update".to_string(),
        "--params".to_string(),
        serde_json::json!({
            "spreadsheetId": scope.sheet_id,
            "range": update_range,
            "valueInputOption": "USER_ENTERED"
        })
        .to_string(),
        "--data".to_string(),
        serde_json::json!({
            "values": [[
                preview.doc_id,
                "promotion_packet_prepared",
                format!("issue://{}", preview.issue_number),
                preview.next_promotion_pr.clone().unwrap_or_default(),
                expected_revision_anchor,
                "bounded-live-roundtrip"
            ]]
        })
        .to_string(),
    ]
}

fn classify_command_failure(output: &GwsCommandOutput) -> GwsRoundtripSkipReason {
    let body = format!("{}\n{}", output.stdout, output.stderr).to_ascii_lowercase();
    if body.contains("scope")
        || body.contains("permission")
        || body.contains("forbidden")
        || body.contains("insufficient")
    {
        GwsRoundtripSkipReason::MissingScopes
    } else if body.contains("credential")
        || body.contains("login")
        || body.contains("oauth")
        || body.contains("token")
        || body.contains("unauth")
        || body.contains("auth")
    {
        GwsRoundtripSkipReason::MissingAuth
    } else {
        GwsRoundtripSkipReason::GwsUnavailable
    }
}

fn classify_runner_error(error: &anyhow::Error) -> GwsRoundtripSkipReason {
    let body = error.to_string().to_ascii_lowercase();
    if body.contains("scope")
        || body.contains("permission")
        || body.contains("forbidden")
        || body.contains("insufficient")
    {
        GwsRoundtripSkipReason::MissingScopes
    } else if body.contains("credential")
        || body.contains("login")
        || body.contains("oauth")
        || body.contains("token")
        || body.contains("unauth")
        || body.contains("auth")
    {
        GwsRoundtripSkipReason::MissingAuth
    } else {
        GwsRoundtripSkipReason::GwsUnavailable
    }
}

pub(super) fn parse_doc(stdout: &str) -> Result<GwsLiveDocRecord> {
    let value: Value = serde_json::from_str(stdout).context("parse docs get json")?;
    Ok(GwsLiveDocRecord {
        document_id: value
            .get("documentId")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("docs get missing documentId"))?
            .to_string(),
        title: value
            .get("title")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("docs get missing title"))?
            .to_string(),
        revision_id: value
            .get("revisionId")
            .and_then(Value::as_str)
            .map(ToString::to_string),
    })
}

pub(super) fn parse_sheet(
    stdout: &str,
    scope: &GwsLiveScopeBinding,
) -> Result<GwsLiveContentCardSheetRecord> {
    let value: Value = serde_json::from_str(stdout).context("parse sheets values get json")?;
    let values = value
        .get("values")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("sheets values get missing values array"))?;
    let parsed_values: Vec<Vec<String>> = values
        .iter()
        .map(|row: &Value| {
            row.as_array()
                .map(|cells: &Vec<Value>| {
                    cells
                        .iter()
                        .filter_map(Value::as_str)
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();
    let header_row = values
        .first()
        .and_then(Value::as_array)
        .map(|row: &Vec<Value>| {
            row.iter()
                .filter_map(Value::as_str)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(GwsLiveContentCardSheetRecord {
        spreadsheet_id: scope.sheet_id.clone(),
        range: value
            .get("range")
            .and_then(Value::as_str)
            .unwrap_or(scope.sheet_range.as_str())
            .to_string(),
        row_count: values.len(),
        header_row,
        values: parsed_values,
    })
}

pub(super) fn parse_sheet_update_confirmation(stdout: &str) -> Result<GwsSheetUpdateConfirmation> {
    let value: Value = serde_json::from_str(stdout).context("parse sheets values update json")?;
    Ok(GwsSheetUpdateConfirmation {
        updated_range: value
            .get("updatedRange")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("sheets values update missing updatedRange"))?
            .to_string(),
        updated_rows: value.get("updatedRows").and_then(Value::as_u64),
        updated_columns: value.get("updatedColumns").and_then(Value::as_u64),
        updated_cells: value.get("updatedCells").and_then(Value::as_u64),
    })
}

pub(super) fn range_dimensions(range: &str) -> Option<(u64, u64)> {
    let (_, cells) = range.split_once('!')?;
    let (start, end) = cells.split_once(':')?;
    let (start_col, start_row) = split_cell_ref(start);
    let (end_col, end_row) = split_cell_ref(end);
    Some((
        u64::from(end_row?.saturating_sub(start_row?).saturating_add(1)),
        column_label_to_index(&end_col?)?
            .saturating_sub(column_label_to_index(&start_col?)?)
            .saturating_add(1),
    ))
}

pub(super) fn column_label_to_index(label: &str) -> Option<u64> {
    if label.is_empty() {
        return None;
    }
    let mut value = 0u64;
    for ch in label.chars() {
        if !ch.is_ascii_alphabetic() {
            return None;
        }
        let digit = u64::from(ch.to_ascii_uppercase() as u8 - b'A' + 1);
        value = value.checked_mul(26)?.checked_add(digit)?;
    }
    Some(value)
}

pub(super) fn confirm_live_sheet_update(
    stdout: &str,
    expected_range: &str,
) -> Result<GwsSheetUpdateConfirmation> {
    let confirmation = parse_sheet_update_confirmation(stdout)?;
    if confirmation.updated_range != expected_range {
        anyhow::bail!(
            "sheets values update confirmed wrong range: expected '{}' got '{}'",
            expected_range,
            confirmation.updated_range
        );
    }
    let (expected_rows, expected_columns) =
        range_dimensions(expected_range).ok_or_else(|| anyhow!("invalid expected update range"))?;
    if let Some(updated_rows) = confirmation.updated_rows {
        if updated_rows == 0 || updated_rows != expected_rows {
            anyhow::bail!(
                "sheets values update confirmed wrong row count: expected {} got {}",
                expected_rows,
                updated_rows
            );
        }
    }
    if let Some(updated_columns) = confirmation.updated_columns {
        if updated_columns == 0 || updated_columns != expected_columns {
            anyhow::bail!(
                "sheets values update confirmed wrong column count: expected {} got {}",
                expected_columns,
                updated_columns
            );
        }
    }
    if let Some(updated_cells) = confirmation.updated_cells {
        let expected_cells = expected_rows.saturating_mul(expected_columns);
        if updated_cells == 0 || updated_cells != expected_cells {
            anyhow::bail!(
                "sheets values update confirmed wrong cell count: expected {} got {}",
                expected_cells,
                updated_cells
            );
        }
    }
    Ok(confirmation)
}

fn skipped_trace(
    capability_name: &'static str,
    argv: Vec<String>,
    mode: GwsLiveMode,
    reason: GwsRoundtripSkipReason,
    summary: impl Into<String>,
) -> GwsLiveMutationCommandTraceRecord {
    GwsLiveMutationCommandTraceRecord {
        capability_name,
        argv,
        mode,
        result: GwsCapabilityResult::Skipped,
        skipped_reason: Some(reason),
        exit_code: None,
        summary: summary.into(),
    }
}

fn proving_trace(
    capability_name: &'static str,
    argv: Vec<String>,
    mode: GwsLiveMode,
    exit_code: i32,
    summary: impl Into<String>,
) -> GwsLiveMutationCommandTraceRecord {
    GwsLiveMutationCommandTraceRecord {
        capability_name,
        argv,
        mode,
        result: GwsCapabilityResult::Proving,
        skipped_reason: None,
        exit_code: Some(exit_code),
        summary: summary.into(),
    }
}

pub(super) fn derive_update_range(sheet_range: &str) -> String {
    let (sheet_name, cells) = match sheet_range.split_once('!') {
        Some(parts) => parts,
        None => return sheet_range.to_string(),
    };
    let (start, end) = match cells.split_once(':') {
        Some(parts) => parts,
        None => return sheet_range.to_string(),
    };
    let (start_col, start_row) = split_cell_ref(start);
    let (end_col, _end_row) = split_cell_ref(end);
    match (start_col, start_row, end_col) {
        (Some(start_col), Some(start_row), Some(end_col)) => {
            let row = start_row.saturating_add(1);
            format!("{sheet_name}!{start_col}{row}:{end_col}{row}")
        }
        _ => sheet_range.to_string(),
    }
}

pub(super) fn locate_doc_update_range(
    sheet: &GwsLiveContentCardSheetRecord,
    doc_id: &str,
) -> Option<String> {
    let (sheet_name, cells) = sheet.range.split_once('!')?;
    let (start, end) = cells.split_once(':')?;
    let (start_col, start_row) = split_cell_ref(start);
    let (end_col, _end_row) = split_cell_ref(end);
    let (start_col, start_row, end_col) = (start_col?, start_row?, end_col?);
    let row_index = sheet
        .values
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, row)| row.first().map(|value| value == doc_id).unwrap_or(false))
        .map(|(index, _)| index)?;
    let row_number = start_row.saturating_add(u32::try_from(row_index).ok()?);
    Some(format!(
        "{sheet_name}!{start_col}{row_number}:{end_col}{row_number}"
    ))
}

pub(super) fn split_cell_ref(cell: &str) -> (Option<String>, Option<u32>) {
    let col: String = cell
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect();
    let row: String = cell
        .chars()
        .skip_while(|c| c.is_ascii_alphabetic())
        .collect();
    let col = if col.is_empty() { None } else { Some(col) };
    let row = row.parse::<u32>().ok();
    (col, row)
}

fn choose_ready_doc_id() -> Result<String> {
    let fixture = load_workspace_cms_fixture()?;
    read_workspace_content_cards(&fixture)
        .into_iter()
        .find(|card| card.readiness == WorkspaceReadiness::BoundedPromotionPacketReady)
        .map(|card| card.doc_id)
        .ok_or_else(|| anyhow!("fixture should contain one ready-for-promotion card"))
}

fn build_preview_and_handoff() -> Result<(
    WorkspaceContentCardUpdatePreview,
    WorkspacePromotionPacket,
    String,
)> {
    let fixture = load_workspace_cms_fixture()?;
    let doc_id = choose_ready_doc_id()?;
    let mut preview = preview_workspace_content_card_update(&fixture, &doc_id)?;
    preview.issue_number = 3093;
    preview.next_promotion_pr = Some("pending://issue-3093/gws-live-roundtrip".to_string());
    let promotion_packet = prepare_workspace_promotion_packet(&fixture, &doc_id)?;
    let expected_revision_anchor = promotion_packet.revision_anchor.clone();
    Ok((preview, promotion_packet, expected_revision_anchor))
}

fn default_apply_outcome(
    preview: WorkspaceContentCardUpdatePreview,
    update_range: String,
    skipped_reason: Option<GwsRoundtripSkipReason>,
) -> GwsLiveApplyOutcomeRecord {
    GwsLiveApplyOutcomeRecord {
        result: GwsCapabilityResult::Skipped,
        skipped_reason,
        update_range,
        preview: preview.clone(),
        apply_result: WorkspaceContentCardUpdateApplyResult {
            doc_id: preview.doc_id.clone(),
            applied_to_fixture: false,
            resulting_status: preview.previous_status.clone(),
            resulting_promotion_pr: preview.previous_promotion_pr.clone(),
            persisted_to_live_workspace: false,
        },
    }
}

fn promotion_handoff_record(
    packet: &WorkspacePromotionPacket,
    revision_anchor: &str,
) -> GwsPromotionPacketHandoffRecord {
    GwsPromotionPacketHandoffRecord {
        doc_id: packet.doc_id.clone(),
        title: packet.title.clone(),
        target_repo_path: packet.target_repo_path.clone(),
        workspace_revision_anchor: revision_anchor.to_string(),
        issue_route: "issue://3093".to_string(),
        pr_route: "pr://pending".to_string(),
        canonical_authority: packet.canonical_authority,
        stop_boundary: packet.stop_boundary,
        tracked_packet_consistent: packet.tracked_packet_consistent,
    }
}

pub(super) fn run_roundtrip_with_runner_with_approval(
    live_mode: GwsLiveMode,
    scope_binding: Option<GwsLiveScopeBinding>,
    write_approval_present: bool,
    runner: &dyn GwsCommandRunner,
) -> Result<GwsLiveContentCardRoundtripReport> {
    let (preview, packet, expected_revision_anchor) = build_preview_and_handoff()?;
    let expected_doc_id = preview.doc_id.clone();
    let update_range = scope_binding
        .as_ref()
        .map(|scope| derive_update_range(&scope.sheet_range))
        .unwrap_or_else(|| "unbound".to_string());
    let mut apply_outcome = default_apply_outcome(preview.clone(), update_range.clone(), None);
    let mut traces = Vec::new();
    let promotion_handoff = promotion_handoff_record(&packet, &expected_revision_anchor);
    let write_approval = write_approval_record(live_mode.clone(), write_approval_present);

    let report = match live_mode {
        GwsLiveMode::Disabled => GwsLiveContentCardRoundtripReport {
            schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
            prompt_record: prompt_record(),
            live_mode,
            live_scope_binding: scope_binding,
            write_approval: write_approval.clone(),
            expected_content_card_doc_id: expected_doc_id,
            content_card_sheet_preview: None,
            revision_anchor: GwsRevisionAnchorRecord {
                expected_revision_anchor: expected_revision_anchor.to_string(),
                live_revision_anchor: None,
                check_status: GwsRevisionCheckStatus::Skipped,
            },
            live_doc_snapshot: None,
            apply_outcome: {
                apply_outcome.skipped_reason = Some(GwsRoundtripSkipReason::LiveModeDisabled);
                apply_outcome
            },
            promotion_packet_handoff: promotion_handoff,
            command_traces: traces,
            roundtrip_result: GwsCapabilityResult::Skipped,
            roundtrip_skipped_reason: Some(GwsRoundtripSkipReason::LiveModeDisabled),
            non_claims: default_non_claims(),
        },
        GwsLiveMode::DryRun => {
            let scope = match scope_binding {
                Some(scope) => scope,
                None => {
                    return Ok(GwsLiveContentCardRoundtripReport {
                        schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                        prompt_record: prompt_record(),
                        live_mode,
                        live_scope_binding: None,
                        write_approval: write_approval.clone(),
                        expected_content_card_doc_id: expected_doc_id,
                        content_card_sheet_preview: None,
                        revision_anchor: GwsRevisionAnchorRecord {
                            expected_revision_anchor: expected_revision_anchor.to_string(),
                            live_revision_anchor: None,
                            check_status: GwsRevisionCheckStatus::Skipped,
                        },
                        live_doc_snapshot: None,
                        apply_outcome: {
                            apply_outcome.skipped_reason =
                                Some(GwsRoundtripSkipReason::MissingScopeBinding);
                            apply_outcome
                        },
                        promotion_packet_handoff: promotion_handoff,
                        command_traces: traces,
                        roundtrip_result: GwsCapabilityResult::Skipped,
                        roundtrip_skipped_reason: Some(GwsRoundtripSkipReason::MissingScopeBinding),
                        non_claims: default_non_claims(),
                    });
                }
            };
            traces.push(skipped_trace(
                "gws.docs.read_snapshot",
                docs_get_args(&scope),
                GwsLiveMode::DryRun,
                GwsRoundtripSkipReason::DryRunOnly,
                "Dry-run posture records the bounded document snapshot command plan without executing a live read.",
            ));
            traces.push(skipped_trace(
                "gws.sheets.read_content_cards",
                sheets_values_get_args(&scope),
                GwsLiveMode::DryRun,
                GwsRoundtripSkipReason::DryRunOnly,
                "Dry-run posture records the bounded content-card read command plan without executing a live sheet read.",
            ));
            traces.push(skipped_trace(
                "gws.sheets.write_content_cards",
                sheets_values_update_args(&scope, &update_range, &preview, &expected_revision_anchor),
                GwsLiveMode::DryRun,
                GwsRoundtripSkipReason::DryRunOnly,
                "Dry-run posture records the bounded content-card write command plan without executing a live Workspace mutation.",
            ));
            GwsLiveContentCardRoundtripReport {
                schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                prompt_record: prompt_record(),
                live_mode,
                live_scope_binding: Some(scope),
                write_approval: write_approval.clone(),
                expected_content_card_doc_id: expected_doc_id,
                content_card_sheet_preview: None,
                revision_anchor: GwsRevisionAnchorRecord {
                    expected_revision_anchor: expected_revision_anchor.to_string(),
                    live_revision_anchor: None,
                    check_status: GwsRevisionCheckStatus::Skipped,
                },
                live_doc_snapshot: None,
                apply_outcome: {
                    apply_outcome.skipped_reason = Some(GwsRoundtripSkipReason::DryRunOnly);
                    apply_outcome
                },
                promotion_packet_handoff: promotion_handoff,
                command_traces: traces,
                roundtrip_result: GwsCapabilityResult::Skipped,
                roundtrip_skipped_reason: Some(GwsRoundtripSkipReason::DryRunOnly),
                non_claims: default_non_claims(),
            }
        }
        GwsLiveMode::Execute => {
            let scope = match scope_binding {
                Some(scope) => scope,
                None => {
                    return Ok(GwsLiveContentCardRoundtripReport {
                        schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                        prompt_record: prompt_record(),
                        live_mode,
                        live_scope_binding: None,
                        write_approval: write_approval.clone(),
                        expected_content_card_doc_id: expected_doc_id,
                        content_card_sheet_preview: None,
                        revision_anchor: GwsRevisionAnchorRecord {
                            expected_revision_anchor: expected_revision_anchor.to_string(),
                            live_revision_anchor: None,
                            check_status: GwsRevisionCheckStatus::Skipped,
                        },
                        live_doc_snapshot: None,
                        apply_outcome: {
                            apply_outcome.skipped_reason =
                                Some(GwsRoundtripSkipReason::MissingScopeBinding);
                            apply_outcome
                        },
                        promotion_packet_handoff: promotion_handoff,
                        command_traces: traces,
                        roundtrip_result: GwsCapabilityResult::Skipped,
                        roundtrip_skipped_reason: Some(GwsRoundtripSkipReason::MissingScopeBinding),
                        non_claims: default_non_claims(),
                    });
                }
            };

            let doc_args = docs_get_args(&scope);
            let doc_output = match runner.run(&doc_args) {
                Ok(output) => output,
                Err(error) => {
                    let reason = classify_runner_error(&error);
                    traces.push(skipped_trace(
                        "gws.docs.read_snapshot",
                        doc_args,
                        GwsLiveMode::Execute,
                        reason.clone(),
                        "Document snapshot read could not start through `gws`, so the live roundtrip is recorded as skipped instead of failing generically.",
                    ));
                    return Ok(GwsLiveContentCardRoundtripReport {
                        schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                        prompt_record: prompt_record(),
                        live_mode,
                        live_scope_binding: Some(scope),
                        write_approval: write_approval.clone(),
                        expected_content_card_doc_id: expected_doc_id,
                        content_card_sheet_preview: None,
                        revision_anchor: GwsRevisionAnchorRecord {
                            expected_revision_anchor: expected_revision_anchor.to_string(),
                            live_revision_anchor: None,
                            check_status: GwsRevisionCheckStatus::Skipped,
                        },
                        live_doc_snapshot: None,
                        apply_outcome: {
                            apply_outcome.skipped_reason = Some(reason.clone());
                            apply_outcome
                        },
                        promotion_packet_handoff: promotion_handoff,
                        command_traces: traces,
                        roundtrip_result: GwsCapabilityResult::Skipped,
                        roundtrip_skipped_reason: Some(reason),
                        non_claims: default_non_claims(),
                    });
                }
            };
            if doc_output.exit_code != 0 {
                let reason = classify_command_failure(&doc_output);
                traces.push(skipped_trace(
                    "gws.docs.read_snapshot",
                    doc_args,
                    GwsLiveMode::Execute,
                    reason.clone(),
                    "Document snapshot read did not complete because the live `gws` request lacked working auth or scope.",
                ));
                return Ok(GwsLiveContentCardRoundtripReport {
                    schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                    prompt_record: prompt_record(),
                    live_mode,
                    live_scope_binding: Some(scope),
                    write_approval: write_approval.clone(),
                    expected_content_card_doc_id: expected_doc_id,
                    content_card_sheet_preview: None,
                    revision_anchor: GwsRevisionAnchorRecord {
                        expected_revision_anchor: expected_revision_anchor.to_string(),
                        live_revision_anchor: None,
                        check_status: GwsRevisionCheckStatus::Skipped,
                    },
                    live_doc_snapshot: None,
                    apply_outcome: {
                        apply_outcome.skipped_reason = Some(reason.clone());
                        apply_outcome
                    },
                    promotion_packet_handoff: promotion_handoff,
                    command_traces: traces,
                    roundtrip_result: GwsCapabilityResult::Skipped,
                    roundtrip_skipped_reason: Some(reason),
                    non_claims: default_non_claims(),
                });
            }
            let live_doc = parse_doc(&doc_output.stdout)?;
            let live_revision_anchor = live_doc.revision_id.clone();
            traces.push(proving_trace(
                "gws.docs.read_snapshot",
                doc_args,
                GwsLiveMode::Execute,
                doc_output.exit_code,
                format!(
                    "Read one bounded live document snapshot for '{}' at revision '{}'.",
                    live_doc.title,
                    live_revision_anchor
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string())
                ),
            ));

            let sheet_args = sheets_values_get_args(&scope);
            let sheet_output = match runner.run(&sheet_args) {
                Ok(output) => output,
                Err(error) => {
                    let reason = classify_runner_error(&error);
                    traces.push(skipped_trace(
                        "gws.sheets.read_content_cards",
                        sheet_args,
                        GwsLiveMode::Execute,
                        reason.clone(),
                        "Content-card sheet read could not start through `gws`, so the live roundtrip is recorded as skipped instead of failing generically.",
                    ));
                    return Ok(GwsLiveContentCardRoundtripReport {
                        schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                        prompt_record: prompt_record(),
                        live_mode,
                        live_scope_binding: Some(scope),
                        write_approval: write_approval.clone(),
                        expected_content_card_doc_id: expected_doc_id,
                        content_card_sheet_preview: None,
                        revision_anchor: GwsRevisionAnchorRecord {
                            expected_revision_anchor: expected_revision_anchor.to_string(),
                            live_revision_anchor,
                            check_status: GwsRevisionCheckStatus::Skipped,
                        },
                        live_doc_snapshot: Some(live_doc),
                        apply_outcome: {
                            apply_outcome.skipped_reason = Some(reason.clone());
                            apply_outcome
                        },
                        promotion_packet_handoff: promotion_handoff,
                        command_traces: traces,
                        roundtrip_result: GwsCapabilityResult::Skipped,
                        roundtrip_skipped_reason: Some(reason),
                        non_claims: default_non_claims(),
                    });
                }
            };
            if sheet_output.exit_code != 0 {
                let reason = classify_command_failure(&sheet_output);
                traces.push(skipped_trace(
                    "gws.sheets.read_content_cards",
                    sheet_args,
                    GwsLiveMode::Execute,
                    reason.clone(),
                    "Content-card sheet read did not complete because the live `gws` request lacked working auth or scope.",
                ));
                return Ok(GwsLiveContentCardRoundtripReport {
                    schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                    prompt_record: prompt_record(),
                    live_mode,
                    live_scope_binding: Some(scope),
                    write_approval: write_approval.clone(),
                    expected_content_card_doc_id: expected_doc_id,
                    content_card_sheet_preview: None,
                    revision_anchor: GwsRevisionAnchorRecord {
                        expected_revision_anchor: expected_revision_anchor.to_string(),
                        live_revision_anchor,
                        check_status: GwsRevisionCheckStatus::Skipped,
                    },
                    live_doc_snapshot: Some(live_doc),
                    apply_outcome: {
                        apply_outcome.skipped_reason = Some(reason.clone());
                        apply_outcome
                    },
                    promotion_packet_handoff: promotion_handoff,
                    command_traces: traces,
                    roundtrip_result: GwsCapabilityResult::Skipped,
                    roundtrip_skipped_reason: Some(reason),
                    non_claims: default_non_claims(),
                });
            }
            let sheet_preview = parse_sheet(&sheet_output.stdout, &scope)?;
            traces.push(proving_trace(
                "gws.sheets.read_content_cards",
                sheet_args,
                GwsLiveMode::Execute,
                sheet_output.exit_code,
                format!(
                    "Read one bounded live content-card sheet range with {} rows.",
                    sheet_preview.row_count
                ),
            ));

            if live_doc.document_id != expected_doc_id {
                let update_args = sheets_values_update_args(
                    &scope,
                    &update_range,
                    &preview,
                    &expected_revision_anchor,
                );
                traces.push(skipped_trace(
                    "gws.sheets.write_content_cards",
                    update_args,
                    GwsLiveMode::Execute,
                    GwsRoundtripSkipReason::RevisionMismatch,
                    "The live document binding did not match the expected content-card doc id, so the bounded write path was stopped before mutation.",
                ));
                return Ok(GwsLiveContentCardRoundtripReport {
                    schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                    prompt_record: prompt_record(),
                    live_mode,
                    live_scope_binding: Some(scope),
                    write_approval: write_approval.clone(),
                    expected_content_card_doc_id: expected_doc_id,
                    content_card_sheet_preview: Some(sheet_preview),
                    revision_anchor: GwsRevisionAnchorRecord {
                        expected_revision_anchor: expected_revision_anchor.to_string(),
                        live_revision_anchor,
                        check_status: GwsRevisionCheckStatus::Mismatched,
                    },
                    live_doc_snapshot: Some(live_doc),
                    apply_outcome: {
                        apply_outcome.skipped_reason =
                            Some(GwsRoundtripSkipReason::RevisionMismatch);
                        apply_outcome
                    },
                    promotion_packet_handoff: promotion_handoff,
                    command_traces: traces,
                    roundtrip_result: GwsCapabilityResult::Skipped,
                    roundtrip_skipped_reason: Some(GwsRoundtripSkipReason::RevisionMismatch),
                    non_claims: default_non_claims(),
                });
            }

            let revision_status =
                if live_revision_anchor.as_deref() == Some(&expected_revision_anchor) {
                    GwsRevisionCheckStatus::Matched
                } else {
                    GwsRevisionCheckStatus::Mismatched
                };

            if revision_status == GwsRevisionCheckStatus::Mismatched {
                let update_args = sheets_values_update_args(
                    &scope,
                    &update_range,
                    &preview,
                    &expected_revision_anchor,
                );
                traces.push(skipped_trace(
                    "gws.sheets.write_content_cards",
                    update_args,
                    GwsLiveMode::Execute,
                    GwsRoundtripSkipReason::RevisionMismatch,
                    "The live revision anchor did not match the tracked promotion packet input, so the bounded write path was stopped before mutation.",
                ));
                return Ok(GwsLiveContentCardRoundtripReport {
                    schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                    prompt_record: prompt_record(),
                    live_mode,
                    live_scope_binding: Some(scope),
                    write_approval: write_approval.clone(),
                    expected_content_card_doc_id: expected_doc_id,
                    content_card_sheet_preview: Some(sheet_preview),
                    revision_anchor: GwsRevisionAnchorRecord {
                        expected_revision_anchor: expected_revision_anchor.to_string(),
                        live_revision_anchor,
                        check_status: GwsRevisionCheckStatus::Mismatched,
                    },
                    live_doc_snapshot: Some(live_doc),
                    apply_outcome: {
                        apply_outcome.skipped_reason =
                            Some(GwsRoundtripSkipReason::RevisionMismatch);
                        apply_outcome
                    },
                    promotion_packet_handoff: promotion_handoff,
                    command_traces: traces,
                    roundtrip_result: GwsCapabilityResult::Skipped,
                    roundtrip_skipped_reason: Some(GwsRoundtripSkipReason::RevisionMismatch),
                    non_claims: default_non_claims(),
                });
            }

            let update_range = match locate_doc_update_range(&sheet_preview, &expected_doc_id) {
                Some(range) => range,
                None => {
                    let fallback_args = sheets_values_update_args(
                        &scope,
                        &update_range,
                        &preview,
                        &expected_revision_anchor,
                    );
                    traces.push(skipped_trace(
                        "gws.sheets.write_content_cards",
                        fallback_args,
                        GwsLiveMode::Execute,
                        GwsRoundtripSkipReason::TargetContentCardMissing,
                        "The bounded content-card row could not be located in the live sheet read, so the write path was stopped before mutation.",
                    ));
                    return Ok(GwsLiveContentCardRoundtripReport {
                        schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                        prompt_record: prompt_record(),
                        live_mode,
                        live_scope_binding: Some(scope),
                        write_approval: write_approval.clone(),
                        expected_content_card_doc_id: expected_doc_id,
                        content_card_sheet_preview: Some(sheet_preview),
                        revision_anchor: GwsRevisionAnchorRecord {
                            expected_revision_anchor: expected_revision_anchor.to_string(),
                            live_revision_anchor,
                            check_status: GwsRevisionCheckStatus::Matched,
                        },
                        live_doc_snapshot: Some(live_doc),
                        apply_outcome: {
                            apply_outcome.skipped_reason =
                                Some(GwsRoundtripSkipReason::TargetContentCardMissing);
                            apply_outcome
                        },
                        promotion_packet_handoff: promotion_handoff,
                        command_traces: traces,
                        roundtrip_result: GwsCapabilityResult::Skipped,
                        roundtrip_skipped_reason: Some(
                            GwsRoundtripSkipReason::TargetContentCardMissing,
                        ),
                        non_claims: default_non_claims(),
                    });
                }
            };

            if !write_approval_present {
                let skipped_update_args = sheets_values_update_args(
                    &scope,
                    &update_range,
                    &preview,
                    &expected_revision_anchor,
                );
                traces.push(skipped_trace(
                    "gws.sheets.write_content_cards",
                    skipped_update_args,
                    GwsLiveMode::Execute,
                    GwsRoundtripSkipReason::MissingWriteApproval,
                    format!(
                        "Execute mode alone does not authorize live Workspace mutation; set {} explicitly before the bounded write path may proceed.",
                        GWS_WRITE_APPROVAL_ENV
                    ),
                ));
                return Ok(GwsLiveContentCardRoundtripReport {
                    schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                    prompt_record: prompt_record(),
                    live_mode,
                    live_scope_binding: Some(scope),
                    write_approval: write_approval.clone(),
                    expected_content_card_doc_id: expected_doc_id,
                    content_card_sheet_preview: Some(sheet_preview),
                    revision_anchor: GwsRevisionAnchorRecord {
                        expected_revision_anchor: expected_revision_anchor.to_string(),
                        live_revision_anchor,
                        check_status: GwsRevisionCheckStatus::Matched,
                    },
                    live_doc_snapshot: Some(live_doc),
                    apply_outcome: {
                        apply_outcome.skipped_reason =
                            Some(GwsRoundtripSkipReason::MissingWriteApproval);
                        apply_outcome
                    },
                    promotion_packet_handoff: promotion_handoff,
                    command_traces: traces,
                    roundtrip_result: GwsCapabilityResult::Skipped,
                    roundtrip_skipped_reason: Some(GwsRoundtripSkipReason::MissingWriteApproval),
                    non_claims: default_non_claims(),
                });
            }

            let update_args = sheets_values_update_args(
                &scope,
                &update_range,
                &preview,
                &expected_revision_anchor,
            );
            let update_output = match runner.run(&update_args) {
                Ok(output) => output,
                Err(error) => {
                    let reason = classify_runner_error(&error);
                    traces.push(skipped_trace(
                        "gws.sheets.write_content_cards",
                        update_args,
                        GwsLiveMode::Execute,
                        reason.clone(),
                        "The bounded content-card write could not start through `gws`, so the live roundtrip is recorded as skipped instead of failing generically.",
                    ));
                    return Ok(GwsLiveContentCardRoundtripReport {
                        schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                        prompt_record: prompt_record(),
                        live_mode,
                        live_scope_binding: Some(scope),
                        write_approval: write_approval.clone(),
                        expected_content_card_doc_id: expected_doc_id,
                        content_card_sheet_preview: Some(sheet_preview),
                        revision_anchor: GwsRevisionAnchorRecord {
                            expected_revision_anchor: expected_revision_anchor.to_string(),
                            live_revision_anchor,
                            check_status: GwsRevisionCheckStatus::Matched,
                        },
                        live_doc_snapshot: Some(live_doc),
                        apply_outcome: {
                            apply_outcome.skipped_reason = Some(reason.clone());
                            apply_outcome
                        },
                        promotion_packet_handoff: promotion_handoff,
                        command_traces: traces,
                        roundtrip_result: GwsCapabilityResult::Skipped,
                        roundtrip_skipped_reason: Some(reason),
                        non_claims: default_non_claims(),
                    });
                }
            };
            if update_output.exit_code != 0 {
                let reason = classify_command_failure(&update_output);
                traces.push(skipped_trace(
                    "gws.sheets.write_content_cards",
                    update_args,
                    GwsLiveMode::Execute,
                    reason.clone(),
                    "The bounded content-card write did not complete because the live `gws` request lacked working auth or scope.",
                ));
                return Ok(GwsLiveContentCardRoundtripReport {
                    schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                    prompt_record: prompt_record(),
                    live_mode,
                    live_scope_binding: Some(scope),
                    write_approval: write_approval.clone(),
                    expected_content_card_doc_id: expected_doc_id,
                    content_card_sheet_preview: Some(sheet_preview),
                    revision_anchor: GwsRevisionAnchorRecord {
                        expected_revision_anchor: expected_revision_anchor.to_string(),
                        live_revision_anchor,
                        check_status: GwsRevisionCheckStatus::Matched,
                    },
                    live_doc_snapshot: Some(live_doc),
                    apply_outcome: {
                        apply_outcome.skipped_reason = Some(reason.clone());
                        apply_outcome
                    },
                    promotion_packet_handoff: promotion_handoff,
                    command_traces: traces,
                    roundtrip_result: GwsCapabilityResult::Skipped,
                    roundtrip_skipped_reason: Some(reason),
                    non_claims: default_non_claims(),
                });
            }
            let update_confirmation = match confirm_live_sheet_update(
                &update_output.stdout,
                &update_range,
            ) {
                Ok(confirmation) => confirmation,
                Err(error) => {
                    let reason = GwsRoundtripSkipReason::GwsUnavailable;
                    traces.push(skipped_trace(
                        "gws.sheets.write_content_cards",
                        update_args,
                        GwsLiveMode::Execute,
                        reason.clone(),
                        format!(
                            "The bounded content-card write returned exit code 0 but did not confirm the expected live Sheets mutation: {error}"
                        ),
                    ));
                    return Ok(GwsLiveContentCardRoundtripReport {
                        schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                        prompt_record: prompt_record(),
                        live_mode,
                        live_scope_binding: Some(scope),
                        write_approval: write_approval.clone(),
                        expected_content_card_doc_id: expected_doc_id,
                        content_card_sheet_preview: Some(sheet_preview),
                        revision_anchor: GwsRevisionAnchorRecord {
                            expected_revision_anchor: expected_revision_anchor.to_string(),
                            live_revision_anchor,
                            check_status: GwsRevisionCheckStatus::Matched,
                        },
                        live_doc_snapshot: Some(live_doc),
                        apply_outcome: {
                            apply_outcome.skipped_reason = Some(reason.clone());
                            apply_outcome
                        },
                        promotion_packet_handoff: promotion_handoff,
                        command_traces: traces,
                        roundtrip_result: GwsCapabilityResult::Skipped,
                        roundtrip_skipped_reason: Some(reason),
                        non_claims: default_non_claims(),
                    });
                }
            };
            let fixture = load_workspace_cms_fixture()?;
            let mut apply_result =
                apply_workspace_content_card_update(&fixture, &preview)?.apply_result;
            apply_result.persisted_to_live_workspace = true;
            traces.push(proving_trace(
                "gws.sheets.write_content_cards",
                update_args,
                GwsLiveMode::Execute,
                update_output.exit_code,
                format!(
                    "Applied one bounded live content-card mutation after preview and revision-anchor match; confirmed persisted range {}.",
                    update_confirmation.updated_range
                ),
            ));
            GwsLiveContentCardRoundtripReport {
                schema_version: GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION,
                prompt_record: prompt_record(),
                live_mode,
                live_scope_binding: Some(scope),
                write_approval,
                expected_content_card_doc_id: expected_doc_id,
                content_card_sheet_preview: Some(sheet_preview),
                revision_anchor: GwsRevisionAnchorRecord {
                    expected_revision_anchor: expected_revision_anchor.to_string(),
                    live_revision_anchor: live_revision_anchor.clone(),
                    check_status: GwsRevisionCheckStatus::Matched,
                },
                live_doc_snapshot: Some(live_doc),
                apply_outcome: GwsLiveApplyOutcomeRecord {
                    result: GwsCapabilityResult::Proving,
                    skipped_reason: None,
                    update_range,
                    preview,
                    apply_result,
                },
                promotion_packet_handoff: promotion_handoff_record(
                    &packet,
                    live_revision_anchor
                        .as_deref()
                        .unwrap_or(&expected_revision_anchor),
                ),
                command_traces: traces,
                roundtrip_result: GwsCapabilityResult::Proving,
                roundtrip_skipped_reason: None,
                non_claims: default_non_claims(),
            }
        }
    };

    Ok(report)
}

fn default_non_claims() -> Vec<&'static str> {
    vec![
        "This surface does not make Google Workspace canonical repo truth.",
        "This surface does not authorize direct tracked repository mutation from Workspace state.",
        "This surface does not treat execute mode alone as approval for live Workspace writes.",
        "This surface does not create bidirectional Git and Workspace sync.",
    ]
}

pub(super) fn run_roundtrip_with_runner(
    live_mode: GwsLiveMode,
    scope_binding: Option<GwsLiveScopeBinding>,
    runner: &dyn GwsCommandRunner,
) -> Result<GwsLiveContentCardRoundtripReport> {
    let write_approval_present = match live_mode {
        GwsLiveMode::Execute => parse_write_approval_from_env(),
        _ => false,
    };
    run_roundtrip_with_runner_with_approval(
        live_mode,
        scope_binding,
        write_approval_present,
        runner,
    )
}

pub fn run_gws_live_content_card_roundtrip_report() -> Result<GwsLiveContentCardRoundtripReport> {
    let runner = SystemGwsCommandRunner;
    run_roundtrip_with_runner(
        parse_live_mode_from_env(),
        parse_scope_binding_from_env(),
        &runner,
    )
}

pub fn write_gws_live_content_card_roundtrip_report(
    report_path: impl AsRef<Path>,
) -> Result<GwsLiveContentCardRoundtripReport> {
    let report = run_gws_live_content_card_roundtrip_report()?;
    let report_path = report_path.as_ref();
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("create parent directories for '{}'", report_path.display())
        })?;
    }
    fs::write(report_path, serde_json::to_string_pretty(&report)?)
        .with_context(|| format!("write '{}'", report_path.display()))?;
    Ok(report)
}
