use super::support::*;
use super::*;
use crate::cli::tooling_cmd::codex_usage_watch::{
    run_collect_status_text, run_parse_status_text, CodexUsageMode,
};
use serde_json::json;
use serde_json::Value;

#[test]
fn codex_usage_watch_parse_and_watch_emit_json_and_history() {
    let repo = TempRepo::new("codex-usage-watch");
    let status = repo.write_rel(
        ".tmp/tooling_cmd_tests/status.txt",
        "Context: 37% left (161,634 used / 258K)\n5h limit: 4% left (resets 4:04 PM)\n7d limit: 3% left (resets Jun 24)\n",
    );
    let history_root = repo.path().join(".adl/runs/codex_usage_watch");

    real_tooling(&[
        "codex-usage-watch".to_string(),
        "parse".to_string(),
        "--input".to_string(),
        status.to_string_lossy().to_string(),
        "--json".to_string(),
    ])
    .expect("parse dispatch should succeed");

    real_tooling(&[
        "codex-usage-watch".to_string(),
        "watch".to_string(),
        "--input".to_string(),
        status.to_string_lossy().to_string(),
        "--iterations".to_string(),
        "1".to_string(),
        "--interval-seconds".to_string(),
        "0".to_string(),
        "--history-root".to_string(),
        history_root.to_string_lossy().to_string(),
        "--json".to_string(),
    ])
    .expect("watch dispatch should succeed");

    let history_path = history_root.join("history.jsonl");
    let history = fs::read_to_string(&history_path).expect("history jsonl");
    let row: Value =
        serde_json::from_str(history.lines().next().expect("history line")).expect("history row");
    assert_eq!(row["schema_version"], "adl.codex_usage_watch.v1");
    assert_eq!(row["mode"], "emergency");
    assert_eq!(row["parse_ok"], true);
}

#[test]
fn codex_usage_watch_classifies_thresholds_and_token_formats() {
    let cases = [
        (
            "Context: 37% left (161,634 used / 258K)\n5h limit: 45% left (resets 4:04 PM)\n7d limit: 33% left (resets Jun 24)\n",
            CodexUsageMode::Normal,
        ),
        (
            "Context: 19% left (161,634 used / 258K)\n5h limit: 45% left (resets 4:04 PM)\n7d limit: 33% left (resets Jun 24)\n",
            CodexUsageMode::Conserve,
        ),
        (
            "Context: 37% left (1.5K used / 258K)\n5h limit: 0.9% left (resets 4:04 PM)\n7d limit: 33% left (resets Jun 24)\n",
            CodexUsageMode::ResetReady,
        ),
        (
            "Context: 37% left (161,634 used / 258K)\n5h limit: 0.4% left (resets 4:04 PM)\n7d limit: 33% left (resets Jun 24)\n",
            CodexUsageMode::InvokeReset,
        ),
    ];

    for (text, expected_mode) in cases {
        let report = run_parse_status_text(text).expect("status text should parse");
        assert_eq!(report.mode, expected_mode);
        assert_eq!(
            serde_json::to_value(&report.mode).unwrap(),
            json!(report.mode.as_str())
        );
        assert!(report.parse_ok);
    }
}

#[test]
fn codex_usage_watch_parse_text_path_succeeds() {
    real_tooling(&[
        "codex-usage-watch".to_string(),
        "parse".to_string(),
        "--text".to_string(),
        "Context: 37% left (161,634 used / 258K)\n5h limit: 4% left (resets 4:04 PM)\n7d limit: 3% left (resets Jun 24)\n".to_string(),
        "--json".to_string(),
    ])
    .expect("parse --text dispatch should succeed");
}

#[test]
fn codex_usage_watch_collect_accepts_copied_status_panel_text() {
    real_tooling(&[
        "codex-usage-watch".to_string(),
        "collect".to_string(),
        "--text".to_string(),
        "Status\nSession: 019f3d65-2216-7832-804a-26339473b27d\nContext:   58%   left   (109,629 used / 258K)\n5h limit: 93% left (resets 9:45 AM)\n7d limit: 99% left (resets Jul 19)\nClose\n".to_string(),
        "--json".to_string(),
    ])
    .expect("collect --text dispatch should succeed for copied status panel");

    let report = run_collect_status_text(
        "Status\nSession: 019f3d65-2216-7832-804a-26339473b27d\nContext:   58%   left   (109,629 used / 258K)\n5h limit: 93% left (resets 9:45 AM)\n7d limit: 99% left (resets Jul 19)\nClose\n",
    );
    assert_eq!(report.mode, CodexUsageMode::Normal);
    assert!(report.parse_ok);
    assert_eq!(report.context.as_ref().unwrap().used_tokens, Some(109_629));
    assert_eq!(
        report.limit_7d.as_ref().unwrap().resets_at.as_deref(),
        Some("Jul 19")
    );
}

#[test]
fn codex_usage_watch_collect_fails_closed_without_live_input_or_required_limits() {
    let missing_err = real_tooling(&[
        "codex-usage-watch".to_string(),
        "collect".to_string(),
        "--json".to_string(),
    ])
    .expect_err("collect without status text should fail closed");
    assert!(missing_err
        .to_string()
        .contains("Codex status collection input missing"));

    let missing_5h_err = real_tooling(&[
        "codex-usage-watch".to_string(),
        "collect".to_string(),
        "--text".to_string(),
        "Status\nContext: 58% left (109,629 used / 258K)\n7d limit: 99% left (resets Jul 19)\n"
            .to_string(),
    ])
    .expect_err("collect without 5h limit should fail closed");
    assert!(missing_5h_err
        .to_string()
        .contains("missing '5h limit:' line"));
}

#[test]
fn codex_usage_watch_watch_fails_closed_for_missing_or_malformed_input() {
    let repo = TempRepo::new("codex-usage-watch-fail-closed");
    let missing = repo.path().join("missing.txt");
    let malformed = repo.write_rel(".tmp/tooling_cmd_tests/malformed.txt", "broken input\n");
    let history_root = repo.path().join(".adl/runs/codex_usage_watch");

    let missing_err = real_tooling(&[
        "codex-usage-watch".to_string(),
        "watch".to_string(),
        "--input".to_string(),
        missing.to_string_lossy().to_string(),
        "--iterations".to_string(),
        "1".to_string(),
        "--interval-seconds".to_string(),
        "0".to_string(),
        "--history-root".to_string(),
        history_root.to_string_lossy().to_string(),
    ])
    .expect_err("missing input should fail closed");
    assert!(missing_err.to_string().contains("status input missing"));

    let malformed_err = real_tooling(&[
        "codex-usage-watch".to_string(),
        "watch".to_string(),
        "--input".to_string(),
        malformed.to_string_lossy().to_string(),
        "--iterations".to_string(),
        "1".to_string(),
        "--interval-seconds".to_string(),
        "0".to_string(),
        "--history-root".to_string(),
        history_root.to_string_lossy().to_string(),
    ])
    .expect_err("malformed input should fail closed");
    assert!(malformed_err
        .to_string()
        .contains("failed to parse status text"));

    let rows: Vec<Value> = fs::read_to_string(history_root.join("history.jsonl"))
        .expect("history jsonl")
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("json row"))
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["mode"], "usage_unknown");
    assert_eq!(rows[1]["mode"], "usage_unknown");
}
