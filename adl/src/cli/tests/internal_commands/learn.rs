use super::super::*;

#[test]
fn real_learn_validates_subcommand_and_export_args() {
    let err = real_learn(&[]).expect_err("missing subcommand");
    assert!(err.to_string().contains("supported: export"));

    let err = real_learn(&["unknown".to_string()]).expect_err("unknown subcommand");
    assert!(err.to_string().contains("unknown learn subcommand"));

    let err = real_learn_export(&[
        "--format".to_string(),
        "csv".to_string(),
        "--out".to_string(),
        "/tmp/out".to_string(),
    ])
    .expect_err("unsupported format");
    assert!(err.to_string().contains("unsupported learn export format"));

    let err =
        real_learn_export(&["--format".to_string(), "jsonl".to_string()]).expect_err("missing out");
    assert!(err.to_string().contains("requires --out"));

    let err =
        real_learn_export(&["--bogus".to_string(), "x".to_string()]).expect_err("unknown arg");
    assert!(err.to_string().contains("unknown learn export arg"));
}

#[test]
fn cli_internal_learn_export_writes_jsonl() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-learn-{now}"));
    let runs_dir = base.join("runs");
    std::fs::create_dir_all(&runs_dir).expect("create runs dir");
    let out = base.join("learning.jsonl");
    real_learn_export(&[
        "--format".to_string(),
        "jsonl".to_string(),
        "--runs-dir".to_string(),
        runs_dir.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
    ])
    .expect("learn export");
    assert!(out.exists(), "learn export should emit output file");
    let tool_result = base.join("learning.jsonl.tool_result.v1.json");
    assert!(
        tool_result.exists(),
        "learn export should emit tool_result sidecar"
    );
    let tool_result_json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&tool_result).expect("read tool_result"))
            .expect("parse tool_result");
    assert_eq!(
        tool_result_json
            .get("schema_version")
            .and_then(|v| v.as_str()),
        Some("tool_result.v1")
    );
    assert_eq!(
        tool_result_json.get("tool_name").and_then(|v| v.as_str()),
        Some("adl.learn.export")
    );
    assert_eq!(
        tool_result_json.get("status").and_then(|v| v.as_str()),
        Some("success")
    );
    let _ = std::fs::remove_dir_all(base);
}
