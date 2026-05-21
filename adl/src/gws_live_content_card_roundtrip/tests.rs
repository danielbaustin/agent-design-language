use super::{
    column_label_to_index, confirm_live_sheet_update, derive_update_range, locate_doc_update_range,
    parse_doc, parse_live_mode_from_env, parse_scope_binding_from_env, parse_sheet,
    parse_sheet_update_confirmation, parse_write_approval_from_env, range_dimensions,
    run_roundtrip_with_runner, run_roundtrip_with_runner_with_approval, split_cell_ref,
    write_gws_live_content_card_roundtrip_report, GwsCommandOutput, GwsCommandRunner,
    GwsRevisionCheckStatus, GwsRoundtripSkipReason, GWS_WRITE_APPROVAL_ENV, HOST_PATH_MARKER,
};
use crate::gws_live_capability_execution_surface::{
    GwsLiveMode, GwsLiveScopeBinding, GWS_DOC_ID_ENV, GWS_DRIVE_FOLDER_ID_ENV, GWS_LIVE_ENABLE_ENV,
    GWS_SHEET_ID_ENV, GWS_SHEET_RANGE_ENV,
};
use crate::gws_live_test_support::{lock_gws_live_test_env, EnvVarGuard};
use crate::rust_native_gws_adapter_boundary::WorkspaceContentStatus;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::collections::VecDeque;
use std::fs;
use std::sync::Mutex;

struct QueueRunner {
    outputs: Mutex<VecDeque<anyhow::Result<GwsCommandOutput>>>,
}

impl QueueRunner {
    fn new(outputs: Vec<anyhow::Result<GwsCommandOutput>>) -> Self {
        Self {
            outputs: Mutex::new(VecDeque::from(outputs)),
        }
    }
}

impl GwsCommandRunner for QueueRunner {
    fn run(&self, _argv: &[String]) -> anyhow::Result<GwsCommandOutput> {
        self.outputs
            .lock()
            .expect("lock outputs")
            .pop_front()
            .unwrap_or_else(|| Err(anyhow::anyhow!("unexpected runner invocation")))
    }
}

fn scope() -> GwsLiveScopeBinding {
    GwsLiveScopeBinding {
        drive_folder_id: "demo-v0912-workspace-cms".to_string(),
        doc_id: "doc-review-packet-demo".to_string(),
        sheet_id: "sheet-content-cards-demo".to_string(),
        sheet_range: "ContentCards!A1:F5".to_string(),
    }
}

fn success_output(stdout: &str) -> anyhow::Result<GwsCommandOutput> {
    Ok(GwsCommandOutput {
        exit_code: 0,
        stdout: stdout.to_string(),
        stderr: String::new(),
    })
}

#[test]
fn gws_live_content_card_roundtrip_env_helpers_cover_aliases_and_scope_binding() {
    let _lock = lock_gws_live_test_env();
    let _mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "dry-run");
    let _drive = EnvVarGuard::set(GWS_DRIVE_FOLDER_ID_ENV, "folder");
    let _doc = EnvVarGuard::set(GWS_DOC_ID_ENV, "doc");
    let _sheet = EnvVarGuard::set(GWS_SHEET_ID_ENV, "sheet");
    let _range = EnvVarGuard::set(GWS_SHEET_RANGE_ENV, "ContentCards!A1:F5");
    let _approval = EnvVarGuard::set(GWS_WRITE_APPROVAL_ENV, "approved");

    assert_eq!(parse_live_mode_from_env(), GwsLiveMode::DryRun);
    assert!(parse_write_approval_from_env());
    let scope = parse_scope_binding_from_env().expect("scope binding");
    assert_eq!(scope.drive_folder_id, "folder");
    assert_eq!(scope.doc_id, "doc");
    assert_eq!(scope.sheet_id, "sheet");
    assert_eq!(scope.sheet_range, "ContentCards!A1:F5");
}

#[test]
fn gws_live_content_card_roundtrip_env_helpers_cover_execute_disabled_and_missing_scope() {
    let _lock = lock_gws_live_test_env();
    let _mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "enabled");
    let _drive = EnvVarGuard::remove(GWS_DRIVE_FOLDER_ID_ENV);
    let _doc = EnvVarGuard::remove(GWS_DOC_ID_ENV);
    let _sheet = EnvVarGuard::remove(GWS_SHEET_ID_ENV);
    let _range = EnvVarGuard::remove(GWS_SHEET_RANGE_ENV);
    let _approval = EnvVarGuard::remove(GWS_WRITE_APPROVAL_ENV);

    assert_eq!(parse_live_mode_from_env(), GwsLiveMode::Execute);
    assert_eq!(parse_scope_binding_from_env(), None);
    assert!(!parse_write_approval_from_env());

    let _mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "not-a-mode");
    assert_eq!(parse_live_mode_from_env(), GwsLiveMode::Disabled);
}

#[test]
fn gws_live_content_card_roundtrip_disabled_and_missing_scope_paths_are_skipped() {
    let disabled = run_roundtrip_with_runner(
        GwsLiveMode::Disabled,
        Some(scope()),
        &QueueRunner::new(vec![]),
    )
    .expect("run disabled report");
    assert_eq!(
        disabled.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::LiveModeDisabled)
    );
    assert!(disabled.command_traces.is_empty());

    let dry_run_missing_scope =
        run_roundtrip_with_runner(GwsLiveMode::DryRun, None, &QueueRunner::new(vec![]))
            .expect("run dry-run missing-scope report");
    assert_eq!(
        dry_run_missing_scope.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingScopeBinding)
    );

    let execute_missing_scope =
        run_roundtrip_with_runner(GwsLiveMode::Execute, None, &QueueRunner::new(vec![]))
            .expect("run execute missing-scope report");
    assert_eq!(
        execute_missing_scope.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingScopeBinding)
    );
}

#[test]
fn gws_live_content_card_roundtrip_dry_run_records_preview_without_live_mutation() {
    let report = run_roundtrip_with_runner(
        GwsLiveMode::DryRun,
        Some(scope()),
        &QueueRunner::new(vec![]),
    )
    .expect("run dry-run report");

    assert_eq!(
        report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::DryRunOnly)
    );
    assert_eq!(
        report.apply_outcome.skipped_reason,
        Some(GwsRoundtripSkipReason::DryRunOnly)
    );
    assert_eq!(report.apply_outcome.preview.issue_number, 3093);
    assert_eq!(
        report.apply_outcome.preview.next_promotion_pr.as_deref(),
        Some("pending://issue-3093/gws-live-roundtrip")
    );
    assert_eq!(report.command_traces.len(), 3);
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_classifies_doc_step_failures() {
    let doc_runner_error = QueueRunner::new(vec![Err(anyhow::anyhow!(
        "oauth login required before docs read"
    ))]);
    let doc_runner_error_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &doc_runner_error,
    )
    .expect("run doc runner error report");
    assert_eq!(
        doc_runner_error_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingAuth)
    );

    let doc_scope_failure = QueueRunner::new(vec![Ok(GwsCommandOutput {
        exit_code: 7,
        stdout: String::new(),
        stderr: "insufficient scope for docs get".to_string(),
    })]);
    let doc_scope_failure_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &doc_scope_failure,
    )
    .expect("run doc scope failure report");
    assert_eq!(
        doc_scope_failure_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingScopes)
    );

    let doc_unavailable = QueueRunner::new(vec![Err(anyhow::anyhow!("gws binary missing"))]);
    let doc_unavailable_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &doc_unavailable,
    )
    .expect("run doc unavailable report");
    assert_eq!(
        doc_unavailable_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::GwsUnavailable)
    );
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_classifies_sheet_step_failures() {
    let sheet_runner_error = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        Err(anyhow::anyhow!("oauth token expired for sheets read")),
    ]);
    let sheet_runner_error_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &sheet_runner_error,
    )
    .expect("run sheet runner error report");
    assert_eq!(
        sheet_runner_error_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingAuth)
    );

    let sheet_scope_failure = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        Ok(GwsCommandOutput {
            exit_code: 9,
            stdout: String::new(),
            stderr: "permission denied by sheet scope".to_string(),
        }),
    ]);
    let sheet_scope_failure_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &sheet_scope_failure,
    )
    .expect("run sheet scope failure report");
    assert_eq!(
        sheet_scope_failure_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingScopes)
    );
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_stops_on_revision_mismatch() {
    let runner = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-99"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
    ]);

    let report =
        run_roundtrip_with_runner_with_approval(GwsLiveMode::Execute, Some(scope()), true, &runner)
            .expect("run mismatch report");

    assert_eq!(
        report.revision_anchor.check_status,
        GwsRevisionCheckStatus::Mismatched
    );
    assert_eq!(
        report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::RevisionMismatch)
    );
    assert_eq!(
        report.apply_outcome.skipped_reason,
        Some(GwsRoundtripSkipReason::RevisionMismatch)
    );
    assert_eq!(report.command_traces.len(), 3);
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_applies_when_revision_matches() {
    let runner = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["another-doc","blocked"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
        success_output(r#"{"updatedRange":"ContentCards!A3:F3"}"#),
    ]);

    let report =
        run_roundtrip_with_runner_with_approval(GwsLiveMode::Execute, Some(scope()), true, &runner)
            .expect("run successful report");

    assert_eq!(
        report.revision_anchor.check_status,
        GwsRevisionCheckStatus::Matched
    );
    assert_eq!(report.roundtrip_skipped_reason, None);
    assert!(report.write_approval.approval_present);
    assert!(
        report
            .apply_outcome
            .apply_result
            .persisted_to_live_workspace
    );
    assert_eq!(
        report.apply_outcome.apply_result.resulting_status,
        WorkspaceContentStatus::PromotionPacketPrepared
    );
    assert_eq!(report.apply_outcome.update_range, "ContentCards!A3:F3");
    assert_eq!(report.command_traces.len(), 3);
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_requires_confirmed_update_response() {
    let wrong_range_runner = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["another-doc","blocked"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
        success_output(
            r#"{"updatedRange":"ContentCards!A2:F2","updatedRows":1,"updatedColumns":6,"updatedCells":6}"#,
        ),
    ]);
    let wrong_range_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &wrong_range_runner,
    )
    .expect("run wrong-range report");
    assert_eq!(
        wrong_range_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::GwsUnavailable)
    );
    assert!(
        !wrong_range_report
            .apply_outcome
            .apply_result
            .persisted_to_live_workspace
    );

    let zero_cells_runner = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["another-doc","blocked"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
        success_output(
            r#"{"updatedRange":"ContentCards!A3:F3","updatedRows":1,"updatedColumns":6,"updatedCells":0}"#,
        ),
    ]);
    let zero_cells_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &zero_cells_runner,
    )
    .expect("run zero-cells report");
    assert_eq!(
        zero_cells_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::GwsUnavailable)
    );
    assert!(
        !zero_cells_report
            .apply_outcome
            .apply_result
            .persisted_to_live_workspace
    );
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_requires_explicit_write_approval() {
    let runner = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
    ]);

    let report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        false,
        &runner,
    )
    .expect("run approval-missing report");

    assert_eq!(
        report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingWriteApproval)
    );
    assert_eq!(
        report.apply_outcome.skipped_reason,
        Some(GwsRoundtripSkipReason::MissingWriteApproval)
    );
    assert!(report.write_approval.approval_checked);
    assert!(!report.write_approval.approval_present);
    assert_eq!(report.command_traces.len(), 3);
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_stops_when_target_row_missing() {
    let runner = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["another-doc","blocked"]]}"#,
        ),
    ]);

    let report =
        run_roundtrip_with_runner_with_approval(GwsLiveMode::Execute, Some(scope()), true, &runner)
            .expect("run missing-row report");
    assert_eq!(
        report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::TargetContentCardMissing)
    );
    assert_eq!(
        report.apply_outcome.skipped_reason,
        Some(GwsRoundtripSkipReason::TargetContentCardMissing)
    );
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_classifies_update_step_failures() {
    let update_runner_error = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
        Err(anyhow::anyhow!("oauth token expired for sheets update")),
    ]);
    let update_runner_error_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &update_runner_error,
    )
    .expect("run update runner error report");
    assert_eq!(
        update_runner_error_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingAuth)
    );

    let update_scope_failure = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
        Ok(GwsCommandOutput {
            exit_code: 13,
            stdout: String::new(),
            stderr: "forbidden: missing sheet scope".to_string(),
        }),
    ]);
    let update_scope_failure_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &update_scope_failure,
    )
    .expect("run update scope failure report");
    assert_eq!(
        update_scope_failure_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingScopes)
    );
}

#[test]
fn gws_live_content_card_roundtrip_execute_mode_stops_when_live_doc_binding_mismatches() {
    let runner = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"wrong-doc","title":"Wrong Doc","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
    ]);

    let report =
        run_roundtrip_with_runner_with_approval(GwsLiveMode::Execute, Some(scope()), true, &runner)
            .expect("run provenance mismatch report");

    assert_eq!(
        report.revision_anchor.check_status,
        GwsRevisionCheckStatus::Mismatched
    );
    assert_eq!(
        report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::RevisionMismatch)
    );
    assert_eq!(
        report.apply_outcome.skipped_reason,
        Some(GwsRoundtripSkipReason::RevisionMismatch)
    );
}

#[test]
fn gws_live_content_card_roundtrip_report_writer_emits_portable_json() {
    let _lock = lock_gws_live_test_env();
    let _mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "dry_run");
    let _drive = EnvVarGuard::set(GWS_DRIVE_FOLDER_ID_ENV, "demo-v0912-workspace-cms");
    let _doc = EnvVarGuard::set(GWS_DOC_ID_ENV, "doc-review-packet-demo");
    let _sheet = EnvVarGuard::set(GWS_SHEET_ID_ENV, "sheet-content-cards-demo");
    let _range = EnvVarGuard::set(GWS_SHEET_RANGE_ENV, "ContentCards!A1:F5");
    let _approval = EnvVarGuard::set(GWS_WRITE_APPROVAL_ENV, "approved");

    let report_path = std::env::temp_dir().join("gws-live-content-card-roundtrip-report.json");
    let report = write_gws_live_content_card_roundtrip_report(&report_path).expect("write report");
    let body = fs::read_to_string(&report_path).expect("read report");
    assert!(body.contains("gws_live_content_card_roundtrip.v1"));
    assert!(!body.contains(HOST_PATH_MARKER));
    assert_eq!(report.apply_outcome.preview.issue_number, 3093);
    assert!(!report.write_approval.approval_checked);
    fs::remove_file(&report_path).expect("remove report");
}

#[test]
fn gws_live_content_card_roundtrip_parsers_and_range_helpers_behave() {
    let doc = parse_doc(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        )
        .expect("parse doc");
    assert_eq!(doc.revision_id.as_deref(), Some("workspace-revision-42"));

    let sheet = parse_sheet(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
            &scope(),
        )
        .expect("parse sheet");
    assert_eq!(sheet.row_count, 2);
    assert_eq!(
        sheet.header_row,
        vec!["doc_id".to_string(), "status".to_string()]
    );
    assert_eq!(
        locate_doc_update_range(&sheet, "doc-review-packet-demo").as_deref(),
        Some("ContentCards!A2:F2")
    );

    assert_eq!(
        derive_update_range("ContentCards!A1:F5"),
        "ContentCards!A2:F2"
    );
    let confirmation = parse_sheet_update_confirmation(
            r#"{"updatedRange":"ContentCards!A3:F3","updatedRows":1,"updatedColumns":6,"updatedCells":6}"#,
        )
        .expect("parse update confirmation");
    assert_eq!(confirmation.updated_range, "ContentCards!A3:F3");
    assert_eq!(range_dimensions("ContentCards!A3:F3"), Some((1, 6)));
    assert_eq!(column_label_to_index("A"), Some(1));
    assert_eq!(column_label_to_index("F"), Some(6));
    confirm_live_sheet_update(
            r#"{"updatedRange":"ContentCards!A3:F3","updatedRows":1,"updatedColumns":6,"updatedCells":6}"#,
            "ContentCards!A3:F3",
        )
        .expect("confirm good update");
    assert!(confirm_live_sheet_update(
            r#"{"updatedRange":"ContentCards!A2:F2","updatedRows":1,"updatedColumns":6,"updatedCells":6}"#,
            "ContentCards!A3:F3",
        )
        .is_err());
    assert_eq!(
        derive_update_range("ContentCards!malformed"),
        "ContentCards!malformed"
    );
    assert_eq!(split_cell_ref("A12"), (Some("A".to_string()), Some(12)));
    assert_eq!(split_cell_ref("12"), (None, Some(12)));
    assert_eq!(split_cell_ref("A"), (Some("A".to_string()), None));
}

#[test]
fn gws_live_content_card_roundtrip_env_parser_edges() {
    let _lock = lock_gws_live_test_env();
    let _dry_run_mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "live");
    assert_eq!(parse_live_mode_from_env(), GwsLiveMode::Execute);
    let _execute_mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "execute");
    assert_eq!(parse_live_mode_from_env(), GwsLiveMode::Execute);
    let _enabled_mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "disabled");
    assert_eq!(parse_live_mode_from_env(), GwsLiveMode::Disabled);

    let _scope_drive = EnvVarGuard::set(GWS_DRIVE_FOLDER_ID_ENV, "folder");
    let _scope_doc = EnvVarGuard::set(GWS_DOC_ID_ENV, "doc");
    let _scope_sheet = EnvVarGuard::set(GWS_SHEET_ID_ENV, "sheet");
    let _scope_range = EnvVarGuard::set(GWS_SHEET_RANGE_ENV, "ContentCards!A1:F5");
    assert_eq!(parse_scope_binding_from_env().is_none(), false);

    let _scope_missing_doc = EnvVarGuard::remove(GWS_DOC_ID_ENV);
    assert!(parse_scope_binding_from_env().is_none());

    let _write_approval_1 = EnvVarGuard::set(GWS_WRITE_APPROVAL_ENV, "1");
    assert!(parse_write_approval_from_env());
    let _write_approval_yes = EnvVarGuard::set(GWS_WRITE_APPROVAL_ENV, "yes");
    assert!(parse_write_approval_from_env());
    let _write_approval_bad = EnvVarGuard::set(GWS_WRITE_APPROVAL_ENV, "maybe");
    assert!(!parse_write_approval_from_env());
}

#[test]
fn gws_live_content_card_roundtrip_parse_and_classification_branches() {
    let doc_bad_json = parse_doc("{ invalid json");
    assert!(doc_bad_json.is_err());
    let doc_missing_document_id = parse_doc(r#"{"title":"Missing ID","revisionId":"r"}"#);
    assert!(doc_missing_document_id.is_err());

    let sheet_missing_values = parse_sheet(r#"{"range":"ContentCards!A1:F5"}"#, &scope());
    assert!(sheet_missing_values.is_err());
    let confirmation_missing_range = parse_sheet_update_confirmation(r#"{"updatedRows":1}"#);
    assert!(confirmation_missing_range.is_err());

    let permission_error = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        Ok(GwsCommandOutput {
            exit_code: 1,
            stdout: String::new(),
            stderr: "forbidden access to sheets data".to_string(),
        }),
    ]);
    let doc_scope_error_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &permission_error,
    )
    .expect("run doc scope error report");
    assert_eq!(
        doc_scope_error_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingScopes)
    );

    let auth_error = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
        Err(anyhow::anyhow!("oauth token revoked")),
    ]);
    let auth_error_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &auth_error,
    )
    .expect("run doc auth error report");
    assert_eq!(
        auth_error_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingAuth)
    );

    let unavailable_error = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        Ok(GwsCommandOutput {
            exit_code: 1,
            stdout: String::new(),
            stderr: "unexpected transport drop".to_string(),
        }),
    ]);
    let unavailable_error_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &unavailable_error,
    )
    .expect("run doc unavailable report");
    assert_eq!(
        unavailable_error_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::GwsUnavailable)
    );

    let runner_unknown = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(
            r#"{"range":"ContentCards!A1:F5","values":[["doc_id","status"],["doc-review-packet-demo","ready_for_repo_promotion"]]}"#,
        ),
        Err(anyhow::anyhow!("temporary transport issue")),
    ]);
    let runner_unknown_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &runner_unknown,
    )
    .expect("run opaque runner error report");
    assert_eq!(
        runner_unknown_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::GwsUnavailable)
    );
}

#[test]
fn gws_live_content_card_roundtrip_exec_classifies_runner_and_command_errors() {
    let doc_command_auth_failure = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        Ok(GwsCommandOutput {
            exit_code: 1,
            stdout: String::new(),
            stderr: "oauth token expired".to_string(),
        }),
    ]);
    let doc_command_auth_failure_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &doc_command_auth_failure,
    )
    .expect("run doc command auth failure report");
    assert_eq!(
        doc_command_auth_failure_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingAuth)
    );

    let doc_runner_scope_failure = QueueRunner::new(vec![Err(anyhow::anyhow!(
        "insufficient scope for docs read"
    ))]);
    let doc_runner_scope_failure_report = run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &doc_runner_scope_failure,
    )
    .expect("run doc runner scope failure report");
    assert_eq!(
        doc_runner_scope_failure_report.roundtrip_skipped_reason,
        Some(GwsRoundtripSkipReason::MissingScopes)
    );
}

#[test]
fn gws_live_content_card_roundtrip_range_helpers_cover_invalid_inputs() {
    assert_eq!(range_dimensions("Sheet"), None);
    assert_eq!(range_dimensions("Sheet!A1"), None);
    assert_eq!(range_dimensions("Sheet!A1:B"), None);
    assert_eq!(range_dimensions("Sheet!1:2"), None);
    assert_eq!(column_label_to_index(""), None);
    assert_eq!(column_label_to_index("12"), None);
    assert_eq!(column_label_to_index("A-"), None);

    assert_eq!(derive_update_range("ContentCards:A1:F5"), "ContentCards:A1:F5");
    assert_eq!(
        derive_update_range("ContentCards!A1:F"),
        "ContentCards!A2:F2"
    );
    assert_eq!(derive_update_range("ContentCards!A:F5"), "ContentCards!A:F5");
}

#[test]
fn gws_live_content_card_roundtrip_live_update_confirmation_validation() {
    assert!(confirm_live_sheet_update(
        r#"{"updatedRange":"ContentCards!A3:F3","updatedRows":0,"updatedColumns":6,"updatedCells":6}"#,
        "ContentCards!A3:F3",
    )
    .is_err());
    assert!(confirm_live_sheet_update(
        r#"{"updatedRange":"ContentCards!A3:F3","updatedRows":2,"updatedColumns":6,"updatedCells":6}"#,
        "ContentCards!A3:F3",
    )
    .is_err());
    assert!(confirm_live_sheet_update(
        r#"{"updatedRange":"ContentCards!A3:F3","updatedRows":1,"updatedColumns":0,"updatedCells":6}"#,
        "ContentCards!A3:F3",
    )
    .is_err());
    assert!(confirm_live_sheet_update(
        r#"{"updatedRange":"ContentCards!A3:F3","updatedRows":1,"updatedColumns":6,"updatedCells":1}"#,
        "ContentCards!A3:F3",
    )
    .is_err());
}

#[test]
fn gws_live_content_card_roundtrip_run_roundtrip_records_parse_failures() {
    let malformed_doc_runner = QueueRunner::new(vec![success_output(r#"{ invalid json"#)]);
    assert!(run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &malformed_doc_runner,
    )
    .is_err());

    let malformed_sheet_runner = QueueRunner::new(vec![
        success_output(
            r#"{"documentId":"doc-review-packet-demo","title":"CodeFriend Review Packet Draft","revisionId":"workspace-revision-42"}"#,
        ),
        success_output(r#"{ invalid json"#),
    ]);
    assert!(run_roundtrip_with_runner_with_approval(
        GwsLiveMode::Execute,
        Some(scope()),
        true,
        &malformed_sheet_runner,
    )
    .is_err());
}

#[cfg(unix)]
#[test]
fn gws_live_content_card_roundtrip_report_writer_creates_parent_dirs_with_system_runner() {
    let _lock = lock_gws_live_test_env();
    let root = std::env::temp_dir().join(format!("adl-gws-live-{}", std::process::id()));
    fs::create_dir_all(&root).expect("create root");
    let counter_path = root.join("gws-call-count");
    fs::write(&counter_path, "0").expect("write call counter");

    let script = root.join("gws");
    fs::write(
        &script,
        format!(
            "#!/bin/sh\ncount_file=\"{}\"\ncount=$(cat \"$count_file\")\ncount=$((count + 1))\necho \"$count\" > \"$count_file\"\ncase \"$count\" in\n1) echo '{{\"documentId\":\"doc-review-packet-demo\",\"title\":\"CodeFriend Review Packet Draft\",\"revisionId\":\"workspace-revision-42\"}}' ;;\n2) echo '{{\"range\":\"ContentCards!A1:F5\",\"values\":[[\"doc_id\",\"status\"],[\"doc-review-packet-demo\",\"ready_for_repo_promotion\"]]}}' ;;\n3) echo '{{\"updatedRange\":\"ContentCards!A2:F2\",\"updatedRows\":1,\"updatedColumns\":6,\"updatedCells\":6}}' ;;\n*) echo '{{\"updatedRange\":\"ContentCards!A2:F2\",\"updatedRows\":1,\"updatedColumns\":6,\"updatedCells\":6}}' ;;\nesac\nexit 0\n",
            counter_path.display()
        ),
    )
    .expect("write gws shim");
    let mut permissions = fs::metadata(&script).expect("script metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&script, permissions).expect("chmod gws shim");

    let original_path = std::env::var("PATH").unwrap_or_default();
    let _path = EnvVarGuard::set("PATH", format!("{}:{}", root.display(), original_path));
    let _mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "execute");
    let _drive = EnvVarGuard::set(GWS_DRIVE_FOLDER_ID_ENV, "demo-v0912-workspace-cms");
    let _doc = EnvVarGuard::set(GWS_DOC_ID_ENV, "doc-review-packet-demo");
    let _sheet = EnvVarGuard::set(GWS_SHEET_ID_ENV, "sheet-content-cards-demo");
    let _range = EnvVarGuard::set(GWS_SHEET_RANGE_ENV, "ContentCards!A1:F5");
    let _approval = EnvVarGuard::set(GWS_WRITE_APPROVAL_ENV, "approved");

    let report_path = root.join("nested").join("report").join("gws-live.json");
    let report = write_gws_live_content_card_roundtrip_report(&report_path)
        .expect("write gws live content card roundtrip report");

    assert_eq!(report.roundtrip_skipped_reason, None);
    assert!(report.apply_outcome.apply_result.persisted_to_live_workspace);
    assert_eq!(report.write_approval.approval_checked, true);
}

#[test]
fn gws_live_content_card_roundtrip_locate_update_range_fails_for_malformed_sheet() {
    let sheet = parse_sheet(
        r#"{"range":"malformed","values":[["doc_id"],["doc-review-packet-demo"]]}"#,
        &scope(),
    )
    .expect("parse sheet");
    assert_eq!(
        locate_doc_update_range(&sheet, "doc-review-packet-demo"),
        None
    );
}
