use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use swarm::execute::materialize_inputs;

mod helpers;
use helpers::EnvVarGuard;

fn tmp_dir(prefix: &str) -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    p.push(format!("swarm-{prefix}-{nanos}"));
    fs::create_dir_all(&p).unwrap();
    p
}

fn write_file(dir: &Path, rel: &str, contents: &[u8]) -> std::path::PathBuf {
    let path = dir.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, contents).unwrap();
    path
}

#[test]
fn materialize_inputs_leaves_non_file_values_unchanged() {
    let base = tmp_dir("mat-unchanged");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "hello".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "hello");
}

#[test]
fn materialize_inputs_reads_relative_path_against_base_dir() {
    let base = tmp_dir("mat-rel");
    write_file(&base, "docs/doc_1.txt", b"abc");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/doc_1.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "abc");
}

#[test]
fn materialize_inputs_accepts_quoted_paths() {
    let base = tmp_dir("mat-quoted");
    write_file(&base, "docs/doc_1.txt", b"abc");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:'docs/doc_1.txt'".to_string());
    inputs.insert("doc_2".to_string(), "@file:\"docs/doc_1.txt\"".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "abc");
    assert_eq!(out["doc_2"], "abc");
}

#[test]
fn materialize_inputs_rejects_empty_path() {
    let base = tmp_dir("mat-emptypath");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:   ".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("empty path"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_errors_on_missing_file() {
    let base = tmp_dir("mat-missing");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/nope.txt".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("failed to stat input file"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_normalizes_windows_newlines() {
    let base = tmp_dir("mat-newlines");
    write_file(&base, "docs/doc_1.txt", b"line1\r\nline2\r\n");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/doc_1.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "line1\nline2\n");
}

#[test]
fn materialize_inputs_rejects_non_utf8() {
    let base = tmp_dir("mat-nonutf8");
    // Invalid UTF-8 byte sequence
    write_file(&base, "docs/bad.bin", &[0xff, 0xfe, 0xfd]);

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/bad.bin".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("not valid UTF-8"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_enforces_max_size() {
    let base = tmp_dir("mat-maxsize");
    // MAX is 512 KiB; create 513 KiB
    let big = vec![b'a'; 513 * 1024];
    write_file(&base, "docs/big.txt", &big);

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/big.txt".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("too large"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

fn prepend_path(bin_dir: &Path) -> String {
    let old_path = std::env::var("PATH").ok();
    let mut new_path = bin_dir.to_string_lossy().to_string();
    if let Some(old) = &old_path {
        new_path.push(':');
        new_path.push_str(old);
    }
    new_path
}

fn write_mock_ollama(dir: &Path, behavior: MockOllamaBehavior) -> std::path::PathBuf {
    let bin = dir.join("ollama");

    // Simple shell mock:
    // - expects: ollama run <model>
    // - reads stdin (prompt) and prints a canned response
    // - can be configured to fail and emit stderr
    let script = match behavior {
        MockOllamaBehavior::Success => {
            r#"#!/bin/sh
set -eu
# Args: run <model>
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
# read stdin but ignore content (still exercises piping)
cat >/dev/null
echo "• mock summary bullet one"
echo "• mock summary bullet two"
exit 0
"#
        }
        MockOllamaBehavior::Fail => {
            r#"#!/bin/sh
set -eu
cat >/dev/null
echo "mock ollama failure: boom" 1>&2
exit 42
"#
        }
    };

    fs::write(&bin, script.as_bytes()).unwrap();

    // chmod +x
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&bin).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin, perms).unwrap();
    }

    bin
}

#[derive(Clone, Copy)]
enum MockOllamaBehavior {
    Success,
    Fail,
}

fn run_swarm(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_swarm");
    Command::new(exe).args(args).output().unwrap()
}

#[test]
fn run_executes_example_with_mock_ollama_and_prints_step_output() {
    let base = tmp_dir("exec-run-mock-ollama");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out = run_swarm(&["examples/adl-0.1.yaml", "--run"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("--- step: summarize_relevant_docs ---"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("mock summary bullet one"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_with_trace_emits_trace_header_and_events() {
    let base = tmp_dir("exec-run-trace-mock-ollama");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out = run_swarm(&["examples/adl-0.1.yaml", "--run", "--trace"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("TRACE run_id=") && stdout.contains("workflow_id="),
        "stdout was:\n{stdout}"
    );
    assert!(stdout.contains("StepStarted"), "stdout was:\n{stdout}");
    assert!(stdout.contains("PromptAssembled"), "stdout was:\n{stdout}");
    assert!(stdout.contains("StepFinished"), "stdout was:\n{stdout}");
}

#[test]
fn run_rejects_concurrent_workflows_in_v0_1() {
    // Even though we expect to fail before executing the provider, install a mock
    // `ollama` to keep the test hermetic if execution order changes.
    let base = tmp_dir("exec-reject-concurrent");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    // Minimal doc that would otherwise run, but uses a concurrent workflow.
    let yaml = r#"
version: "0.1"

providers:
  local:
    type: "ollama"
    config:
      model: "phi4-mini"

agents:
  a1:
    provider: "local"
    model: "phi4-mini"

tasks:
  t1:
    prompt:
      user: "Summarize: {{text}}"

run:
  name: "reject-concurrent"
  workflow:
    kind: "concurrent"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
"#;

    let tmp_yaml = base.join("concurrent.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure for concurrent workflow, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr).to_lowercase();
    assert!(
        stderr.contains("concurrent"),
        "stderr should mention concurrent workflows; stderr was:\n{stderr}"
    );
    assert!(
        stderr.contains("does not support")
            || stderr.contains("not supported")
            || stderr.contains("not implemented"),
        "stderr should explain concurrency is unsupported in v0.1; stderr was:\n{stderr}"
    );
    assert!(
        stderr.contains("sequential") || stderr.contains("single workflow"),
        "stderr should suggest sequential/single workflow; stderr was:\n{stderr}"
    );
    assert!(
        stderr.contains("upgrade") || stderr.contains("v0.3"),
        "stderr should mention upgrading for concurrency; stderr was:\n{stderr}"
    );
}

#[test]
fn run_reports_error_when_materialized_doc_is_missing() {
    let base = tmp_dir("exec-missing-doc");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    // The example workflow may reference docs via either:
    // - @file:examples/docs/*.txt
    // - @file:docs/*.txt
    // We populate both locations under `base` so the test stays aligned with the
    // evolving example format, and then break exactly one path.
    let names = ["doc_1.txt", "doc_2.txt", "doc_3.txt"];

    fs::create_dir_all(base.join("examples/docs")).unwrap();
    fs::create_dir_all(base.join("docs")).unwrap();

    for name in names {
        let src = Path::new("examples/docs").join(name);
        fs::copy(&src, base.join("examples/docs").join(name)).unwrap();
        fs::copy(&src, base.join("docs").join(name)).unwrap();
    }

    // Copy the example yaml and break one file input path.
    let yaml_src = fs::read_to_string("examples/adl-0.1.yaml").unwrap();
    // Break doc_1 for either path style.
    let yaml_broken = yaml_src
        .replace(
            "@file:examples/docs/doc_1.txt",
            "@file:examples/docs/DOES_NOT_EXIST.txt",
        )
        .replace("@file:docs/doc_1.txt", "@file:docs/DOES_NOT_EXIST.txt");

    let tmp_yaml = base.join("adl-broken.yaml");
    fs::write(&tmp_yaml, yaml_broken.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("failed to materialize inputs")
            || stderr.contains("failed to stat input file"),
        "stderr was:\n{stderr}"
    );
    assert!(stderr.contains("doc_1"), "stderr was:\n{stderr}");
}

#[test]
fn run_surfaces_provider_failure_stderr() {
    let base = tmp_dir("exec-provider-failure");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Fail);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out = run_swarm(&["examples/adl-0.1.yaml", "--run"]);
    assert!(
        !out.status.success(),
        "expected failure, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("ollama run failed") || stderr.contains("mock ollama failure"),
        "stderr was:\n{stderr}"
    );
}
