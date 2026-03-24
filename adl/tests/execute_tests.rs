use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use ::adl::execute::{materialize_inputs, MATERIALIZE_INPUT_MAX_FILE_BYTES};

mod helpers;
use helpers::{unique_test_temp_dir, EnvVarGuard};

fn tmp_dir(prefix: &str) -> std::path::PathBuf {
    unique_test_temp_dir(prefix)
}

fn write_file(dir: &Path, rel: &str, contents: &[u8]) -> std::path::PathBuf {
    let path = dir.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, contents).unwrap();
    path
}

fn reserve_local_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

fn start_swarm_remote_server() -> String {
    let port = reserve_local_port();
    let bind_addr = format!("127.0.0.1:{port}");
    thread::spawn({
        let bind_addr = bind_addr.clone();
        move || {
            let _ = ::adl::remote_exec::run_server(&bind_addr);
        }
    });
    thread::sleep(Duration::from_millis(120));
    format!("http://{bind_addr}")
}

fn start_raw_http_server(raw_response: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind raw http test server");
    let bind_addr = listener
        .local_addr()
        .expect("raw http listener local addr")
        .to_string();
    thread::spawn({
        let listener = listener;
        move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0_u8; 2048];
                let _ = stream.read(&mut buf);
                let _ = stream.write_all(raw_response.as_bytes());
                let _ = stream.flush();
            }
        }
    });
    thread::sleep(Duration::from_millis(80));
    format!("http://{bind_addr}")
}

fn start_fixed_http_provider_server(max_requests: usize, output: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind fixed http test server");
    let bind_addr = listener
        .local_addr()
        .expect("fixed http listener local addr")
        .to_string();
    thread::spawn({
        let listener = listener;
        move || {
            for _ in 0..max_requests {
                let (mut stream, _) = listener.accept().expect("accept fixed http request");
                let mut buf = [0_u8; 4096];
                let _ = stream.read(&mut buf);
                let body = format!(r#"{{"output":"{output}"}}"#);
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes());
                let _ = stream.flush();
            }
        }
    });
    thread::sleep(Duration::from_millis(80));
    format!("http://{bind_addr}")
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
        MockOllamaBehavior::EchoModel => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
model="${2:-}"
if [ -z "${model}" ]; then
  echo "mock ollama: expected model arg2" 1>&2
  exit 2
fi
cat >/dev/null
echo "MODEL=${model}"
exit 0
"#
        }
        MockOllamaBehavior::EchoPrompt => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
cat
exit 0
"#
        }
        MockOllamaBehavior::SleepEchoPrompt => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
sleep 1
cat
exit 0
"#
        }
        MockOllamaBehavior::FailOnce => {
            r#"#!/bin/sh
set -eu
state_file="${0}.state"
if [ ! -f "${state_file}" ]; then
  echo "mock ollama first attempt failure" 1>&2
  touch "${state_file}"
  exit 42
fi
cat >/dev/null
echo "MOCK_RECOVERED"
exit 0
"#
        }
        MockOllamaBehavior::FailOnToken => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
prompt="$(cat)"
if printf "%s" "${prompt}" | grep -q "FAIL_THIS_STEP"; then
  echo "mock ollama forced fail token seen" 1>&2
  exit 41
fi
echo "MOCK_CONTINUE_OK"
exit 0
"#
        }
        MockOllamaBehavior::StreamThenFailOnToken => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
prompt="$(cat)"
if printf "%s" "${prompt}" | grep -q "FAIL_THIS_STEP"; then
  printf "PARTIAL_FAIL_CHUNK\n"
  echo "mock ollama forced fail token seen after chunk" 1>&2
  exit 41
fi
echo "MOCK_STREAM_OK"
exit 0
"#
        }
        MockOllamaBehavior::SleepTrackConcurrency => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
cat >/dev/null
sleep 1
echo "MOCK_SLEEP_OK"
exit 0
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
    EchoModel,
    EchoPrompt,
    SleepEchoPrompt,
    FailOnce,
    FailOnToken,
    StreamThenFailOnToken,
    SleepTrackConcurrency,
}

fn run_swarm(args: &[&str]) -> std::process::Output {
    Command::new(resolve_adl_exe())
        .env("ADL_ALLOW_UNSIGNED", "1")
        .args(args)
        .output()
        .unwrap()
}

fn run_swarm_in_dir(cwd: &Path, args: &[&str]) -> std::process::Output {
    Command::new(resolve_adl_exe())
        .current_dir(cwd)
        .env("ADL_ALLOW_UNSIGNED", "1")
        .args(args)
        .output()
        .unwrap()
}

fn resolve_adl_exe() -> std::path::PathBuf {
    let raw = std::env::var("CARGO_BIN_EXE_adl")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_adl").to_string());
    let path = std::path::PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}

fn repo_runs_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .join(".adl")
        .join("runs")
}

fn run_artifact_paths(
    run_id: &str,
) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
    let run_dir = repo_runs_dir().join(run_id);
    (
        run_dir.join("run.json"),
        run_dir.join("steps.json"),
        run_dir.join("run_summary.json"),
    )
}

fn pause_state_path(run_id: &str) -> std::path::PathBuf {
    repo_runs_dir().join(run_id).join("pause_state.json")
}

fn trace_started_step_ids(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            let marker = "StepStarted step=";
            let (_, tail) = line.split_once(marker)?;
            Some(tail.split_whitespace().next()?.to_string())
        })
        .collect()
}

fn trace_chunk_step_ids(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            let marker = "StepOutputChunk step=";
            let (_, tail) = line.split_once(marker)?;
            Some(tail.split_whitespace().next()?.to_string())
        })
        .collect()
}

fn delegation_error_code(stderr: &str) -> Option<&str> {
    stderr
        .lines()
        .find_map(|line| line.strip_prefix("Error: "))
        .and_then(|msg| msg.split(": ").next())
}

#[path = "execute_tests/concurrency_patterns.rs"]
mod concurrency_patterns;
#[path = "execute_tests/delegation_resume.rs"]
mod delegation_resume;
#[path = "execute_tests/materialize_inputs.rs"]
mod materialize_inputs;
#[path = "execute_tests/run_flows.rs"]
mod run_flows;
#[path = "execute_tests/runtime_artifacts.rs"]
mod runtime_artifacts;
