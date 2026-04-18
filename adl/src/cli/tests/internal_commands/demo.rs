use super::super::*;

#[test]
fn cli_internal_demo_print_plan_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--print-plan".to_string()])
        .expect("known demo should succeed");
}

#[test]
fn cli_internal_demo_trace_only_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--trace".to_string()])
        .expect("trace-only dry run should succeed");
}

#[test]
fn cli_internal_demo_run_no_open_path_succeeds() {
    let out_dir = unique_temp_dir("adl-demo-no-open");
    real_demo(&[
        "demo-b-one-command".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("demo run with explicit no-open should succeed");
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_internal_demo_help_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--help".to_string()])
        .expect("help path should succeed");
}

#[test]
fn cli_internal_demo_defaults_to_run_when_no_mode_flag_is_given() {
    let out_dir = unique_temp_dir("adl-demo-default-run");
    real_demo(&[
        "demo-a-say-mcp".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("default demo invocation should run");
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_internal_demo_run_trace_and_out_path_succeeds() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir = std::env::temp_dir().join(format!("adl-demo-out-{now}"));
    real_demo(&[
        "demo-a-say-mcp".to_string(),
        "--run".to_string(),
        "--trace".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("demo run with trace and explicit out dir should succeed");
    assert!(out_dir.join("demo-a-say-mcp").exists());
    let _ = std::fs::remove_dir_all(out_dir);
}
