use super::*;
use base64::Engine;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::io::Write;

const CSM_COVERAGE_STARTUP_ATTEMPTS: &str = "80";
const CSM_CONTROL_PLANE_FIRST_REQUEST_TIMEOUT_SECS: u64 = 120;
const CSM_DISK_READY_ENV: [(&str, &str); 2] = [
    ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
    ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
];

fn spawn_loopback_control_plane() -> (
    String,
    std::sync::mpsc::Receiver<String>,
    std::thread::JoinHandle<()>,
) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback control plane");
    listener
        .set_nonblocking(true)
        .expect("set control plane nonblocking");
    let addr = listener.local_addr().expect("control plane addr");
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let started = std::time::Instant::now();
        let mut last_request_at: Option<std::time::Instant> = None;
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_nonblocking(false)
                        .expect("set control plane stream blocking");
                    stream
                        .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                        .expect("set control plane read timeout");
                    let request = read_http_request(&mut stream);
                    tx.send(request).expect("retain control plane request");
                    stream
                        .write_all(
                            b"HTTP/1.1 200 OK\r\nx-request-id: shutdown-live-receipt-1\r\ncontent-length: 2\r\n\r\nOK",
                        )
                        .expect("write control plane response");
                    last_request_at = Some(std::time::Instant::now());
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    let idle_done = last_request_at
                        .map(|instant| instant.elapsed() > std::time::Duration::from_secs(2))
                        .unwrap_or(false);
                    if idle_done
                        || started.elapsed()
                            > std::time::Duration::from_secs(
                                CSM_CONTROL_PLANE_FIRST_REQUEST_TIMEOUT_SECS,
                            )
                    {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
                Err(err) => panic!("control plane accept failed: {err}"),
            }
        }
    });
    (format!("http://{addr}/runtime-notices"), rx, handle)
}

fn spawn_loopback_otlp_collector() -> (
    String,
    std::sync::mpsc::Receiver<String>,
    std::sync::mpsc::Sender<()>,
    std::thread::JoinHandle<()>,
) {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback otlp collector");
    listener
        .set_nonblocking(true)
        .expect("set collector nonblocking");
    let addr = listener.local_addr().expect("collector addr");
    let (tx, rx) = std::sync::mpsc::channel();
    let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let started = std::time::Instant::now();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_nonblocking(false)
                        .expect("set collector stream blocking");
                    stream
                        .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                        .expect("set collector stream read timeout");
                    let body = read_http_body(&mut stream);
                    tx.send(body).expect("send otlp body");
                    stream
                        .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\n\r\nOK")
                        .expect("write collector response");
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    match shutdown_rx.try_recv() {
                        Ok(()) | Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
                        Err(std::sync::mpsc::TryRecvError::Empty) => {}
                    }
                    if started.elapsed() > std::time::Duration::from_secs(60) {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
                Err(err) => panic!("collector accept failed: {err}"),
            }
        }
    });
    (format!("http://{addr}/v1/traces"), rx, shutdown_tx, handle)
}

fn read_http_body(stream: &mut std::net::TcpStream) -> String {
    let request = read_http_request(stream);
    request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body.to_string())
        .unwrap_or_default()
}

fn read_http_request(stream: &mut std::net::TcpStream) -> String {
    use std::io::Read;
    let mut buf = Vec::new();
    let mut temp = [0_u8; 4096];
    loop {
        let n = stream.read(&mut temp).expect("read request");
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&temp[..n]);
        if let Some(header_end) = find_header_end(&buf) {
            let content_length = content_length(&buf[..header_end]).unwrap_or(0);
            if buf.len() >= header_end + 4 + content_length {
                break;
            }
        }
    }
    String::from_utf8_lossy(&buf).to_string()
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|window| window == b"\r\n\r\n")
}

fn content_length(headers: &[u8]) -> Option<usize> {
    String::from_utf8_lossy(headers).lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("content-length")
            .then(|| value.trim().parse::<usize>().ok())
            .flatten()
    })
}

fn read_http_response_body(stream: &mut std::net::TcpStream) -> String {
    use std::io::Read;
    let mut buf = Vec::new();
    let mut temp = [0_u8; 4096];
    loop {
        let n = match stream.read(&mut temp) {
            Ok(n) => n,
            Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => break,
            Err(err) => panic!("read response: {err}"),
        };
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&temp[..n]);
        if let Some(header_end) = find_header_end(&buf) {
            let content_length = content_length(&buf[..header_end]).unwrap_or(0);
            if buf.len() >= header_end + 4 + content_length {
                break;
            }
        }
    }
    if let Some(header_end) = find_header_end(&buf) {
        String::from_utf8_lossy(&buf[header_end + 4..]).to_string()
    } else {
        String::new()
    }
}

fn read_text_or_missing(path: &std::path::Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| format!("<missing {}: {err}>", path.display()))
}

fn assert_text_contains(haystack: &str, needle: &str, label: &str) {
    assert!(
        haystack.contains(needle),
        "expected {label} to contain {needle:?}, actual:\n{haystack}"
    );
}

fn run_csm_with_unconfigured_cloud_notice_env(args: &[&str]) -> std::process::Output {
    let mut command = std::process::Command::new(resolve_csm_exe());
    command.args(args);
    for key in [
        "ADL_AWS_SIGNAL_MODE",
        "ADL_AWS_SIGNAL_APPROVED",
        "ADL_AWS_REGION",
        "ADL_AWS_PROFILE",
        "AWS_PROFILE",
        "ADL_AWS_HEARTBEAT_TARGET",
        "ADL_AWS_HEARTBEAT_LOG_GROUP",
        "ADL_AWS_HEARTBEAT_LOG_STREAM",
        "ADL_AWS_SNS_TOPIC_ARN",
        "ADL_CSM_NOTICE_CONTROL_PLANE_MODE",
        "ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED",
        "ADL_CSM_NOTICE_CONTROL_PLANE_TARGET",
        "ADL_CSM_NOTICE_CONTROL_PLANE_URL",
        "ADL_CSM_NOTICE_LAMBDA_FUNCTION",
        "ADL_CSM_NOTICE_EVENT_BUS",
    ] {
        command.env_remove(key);
    }
    command.output().expect("run csm binary")
}

fn http_get_json(addr: &str, path: &str) -> serde_json::Value {
    let started = std::time::Instant::now();
    loop {
        match std::net::TcpStream::connect(addr) {
            Ok(mut stream) => {
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                    .expect("set API read timeout");
                write!(
                    stream,
                    "GET {path} HTTP/1.1\r\nhost: {addr}\r\nconnection: close\r\n\r\n"
                )
                .expect("write API request");
                stream.flush().expect("flush API request");
                let body = read_http_response_body(&mut stream);
                if body.trim().is_empty() && started.elapsed() < std::time::Duration::from_secs(5) {
                    std::thread::sleep(std::time::Duration::from_millis(25));
                    continue;
                }
                return serde_json::from_str(&body).unwrap_or_else(|err| {
                    panic!("parse API response for {path}: {err}; body:\n{body}")
                });
            }
            Err(err) if started.elapsed() < std::time::Duration::from_secs(5) => {
                let _ = err;
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            Err(err) => panic!("connect to CSM API {addr} for {path}: {err}"),
        }
    }
}

fn http_get_json_authenticated(
    addr: &str,
    state_root: &std::path::Path,
    path: &str,
) -> serde_json::Value {
    let store =
        adl_runtime::runtime_api_auth::RuntimeApiCredentialStore::for_state_root(state_root);
    let started = std::time::Instant::now();
    loop {
        match std::net::TcpStream::connect(addr) {
            Ok(mut stream) => {
                let response = store.with_bearer_token(|token| {
                    stream
                        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                        .expect("set API read timeout");
                    write!(
                        stream,
                        "GET {path} HTTP/1.1\r\nhost: {addr}\r\nauthorization: Bearer {token}\r\nconnection: close\r\n\r\n"
                    )
                    .expect("write authenticated API request");
                    stream.flush().expect("flush authenticated API request");
                    read_http_response_body(&mut stream)
                });
                match response {
                    Ok(body)
                        if !body.trim().is_empty()
                            || started.elapsed() >= std::time::Duration::from_secs(5) =>
                    {
                        return serde_json::from_str(&body).unwrap_or_else(|err| {
                            panic!(
                                "parse authenticated API response for {path}: {err}; body:\n{body}"
                            )
                        });
                    }
                    Ok(_) | Err(_) if started.elapsed() < std::time::Duration::from_secs(5) => {
                        std::thread::sleep(std::time::Duration::from_millis(25));
                    }
                    Ok(_) => panic!("authenticated API response for {path} was empty"),
                    Err(err) => panic!("load runtime API credential for {path}: {err}"),
                }
            }
            Err(err) if started.elapsed() < std::time::Duration::from_secs(5) => {
                let _ = err;
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            Err(err) => panic!("connect to authenticated CSM API {addr} for {path}: {err}"),
        }
    }
}

fn reserve_csm_test_port(label: &str) -> (std::net::TcpListener, String) {
    let mut ports: Vec<u16> = (19950..=19999).filter(|port| *port != 19997).collect();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    label.hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    let offset = (hasher.finish() as usize) % ports.len();
    ports.rotate_left(offset);
    for port in ports {
        let addr = format!("127.0.0.1:{port}");
        if let Ok(listener) = std::net::TcpListener::bind(&addr) {
            return (listener, addr);
        }
    }
    panic!("could not bind one governed CSM test port for {label} in 19950-19999");
}

fn reserve_ephemeral_csm_test_port(label: &str) -> (std::net::TcpListener, String) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap_or_else(|err| panic!("reserve ephemeral CSM test port for {label}: {err}"));
    let addr = listener
        .local_addr()
        .unwrap_or_else(|err| panic!("read ephemeral CSM test port for {label}: {err}"))
        .to_string();
    (listener, addr)
}

fn request_governed_stop_and_wait(spec: &std::path::Path, child: &mut std::process::Child) {
    let stop = run_csm(&[
        "governed-stop",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--reason",
        "test cleanup requested governed runtime stop",
        "--operator",
        "cli-smoke",
        "--authorization",
        "test-governed-stop",
        "--intent",
        "recoverability_drill",
        "--requested-at",
        "2026-07-07T16:00:00Z",
        "--json",
    ]);
    assert!(
        stop.status.success(),
        "governed stop stderr:\n{}",
        String::from_utf8_lossy(&stop.stderr)
    );
    wait_for_governed_shutdown_child(child);
}

fn wait_for_governed_shutdown_child(child: &mut std::process::Child) {
    let started = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(CSM_CONTROL_PLANE_FIRST_REQUEST_TIMEOUT_SECS);
    loop {
        if child
            .try_wait()
            .expect("check governed CSM daemon child")
            .is_some()
        {
            break;
        }
        if started.elapsed() > timeout {
            let _ = child.kill();
            panic!("CSM daemon did not exit after governed stop request");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn write_shutdown_probe_spec(root: &std::path::Path, agent_id: &str) -> std::path::PathBuf {
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        format!(
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: {agent_id}
display_name: Shutdown Probe
state_root: state
workflow:
  kind: demo_adapter
  name: shutdown_probe
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/{agent_id}
  write_policy: append_only
"#
        ),
    )
    .expect("write shutdown probe spec");
    spec
}

fn wait_for_shutdown_probe_running(root: &std::path::Path) {
    let status_path = root.join("state/daemon_status.json");
    let started = std::time::Instant::now();
    loop {
        let running = fs::read(&status_path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
            .and_then(|status| {
                status
                    .get("state")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_string)
            })
            .as_deref()
            == Some("running");
        if running {
            return;
        }
        assert!(
            started.elapsed() < std::time::Duration::from_secs(20),
            "shutdown probe daemon did not reach running state"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn otlp_attr_string<'a>(attrs: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    attrs.as_array()?.iter().find_map(|attr| {
        (attr.get("key")?.as_str()? == key)
            .then(|| attr.get("value")?.get("stringValue")?.as_str())
            .flatten()
    })
}

fn copy_dir_all(source: &std::path::Path, dest: &std::path::Path) {
    fs::create_dir_all(dest).expect("create copied bundle dir");
    for entry in fs::read_dir(source).expect("read source dir") {
        let entry = entry.expect("read source entry");
        let child_source = entry.path();
        let child_dest = dest.join(entry.file_name());
        if child_source.is_dir() {
            copy_dir_all(&child_source, &child_dest);
        } else {
            fs::copy(&child_source, &child_dest).expect("copy bundle file");
        }
    }
}

fn assert_stage_failure_with_env<F>(
    bundle: &std::path::Path,
    bad_bundle: &std::path::Path,
    out_dir: &std::path::Path,
    env: &[(&str, &str)],
    mutate: F,
    expected_stderr: &str,
) where
    F: FnOnce(&std::path::Path),
{
    copy_dir_all(bundle, bad_bundle);
    mutate(bad_bundle);
    let out = run_csm_with_env(
        &[
            "continuity",
            "stage",
            "--bundle",
            bad_bundle.to_str().expect("utf8 bad bundle"),
            "--out",
            out_dir.to_str().expect("utf8 stage dir"),
            "--target-host",
            "ec2-staging",
            "--json",
        ],
        env,
    );
    assert!(
        !out.status.success(),
        "expected stage failure containing {expected_stderr}, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains(expected_stderr),
        "expected stderr to contain {expected_stderr}, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn custody_public_key_from_private_key(private_key_b64: &str) -> String {
    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(private_key_b64.as_bytes())
        .expect("decode fixture custody private key");
    let signing = p256::ecdsa::SigningKey::from_slice(&key_bytes)
        .expect("fixture custody private key must be valid P-256");
    base64::engine::general_purpose::STANDARD
        .encode(signing.verifying_key().to_encoded_point(false).as_bytes())
}

fn sign_custody_value_with_private_key(
    custody: &mut serde_json::Value,
    private_key_b64: &str,
    key_id: &str,
) {
    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(private_key_b64.as_bytes())
        .expect("decode attacker custody private key");
    let signing = p256::ecdsa::SigningKey::from_slice(&key_bytes)
        .expect("attacker custody private key must be valid P-256");
    custody["signature"] = serde_json::Value::Null;
    let canonical = canonical_sorted_json_bytes(custody);
    let digest = Sha256::digest(&canonical);
    let signature: p256::ecdsa::Signature =
        p256::ecdsa::signature::Signer::sign(&signing, &canonical);
    custody["signature"] = serde_json::json!({
        "schema": "adl.csm.polis_artifact_custody_signature.v1",
        "alg": "ecdsa-p256-sha256",
        "key_id": key_id,
        "public_key_b64": base64::engine::general_purpose::STANDARD
            .encode(signing.verifying_key().to_encoded_point(false).as_bytes()),
        "sig_b64": base64::engine::general_purpose::STANDARD
            .encode(signature.to_der().as_bytes()),
        "signed_payload": {
            "canonical_json_profile": "adl.csm.polis_custody.canonical_json.sorted_serde_json.v1",
            "excluded_fields": ["signature"],
            "payload_sha256": format!("sha256:{digest:x}")
        }
    });
}

fn canonical_sorted_json_bytes(value: &serde_json::Value) -> Vec<u8> {
    let mut sorted = value.clone();
    sort_json_value(&mut sorted);
    serde_json::to_vec(&sorted).expect("serialize canonical sorted JSON")
}

fn sort_json_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            let mut sorted = BTreeMap::new();
            for (key, mut child) in std::mem::take(map) {
                sort_json_value(&mut child);
                sorted.insert(key, child);
            }
            for (key, child) in sorted {
                map.insert(key, child);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                sort_json_value(item);
            }
        }
        _ => {}
    }
}

#[test]
fn agent_run_writes_bounded_cycles_and_status() {
    let root = unique_test_temp_dir("agent-smoke");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: smoke-agent
display_name: Smoke Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let disk_ready_env = [
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ];
    let out = run_adl_with_env(
        &[
            "agent",
            "run",
            "--spec",
            spec_str,
            "--max-cycles",
            "3",
            "--no-sleep",
            "--json",
        ],
        &disk_ready_env,
    );
    assert!(
        out.status.success(),
        "expected agent run success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"state\": \"completed\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"completed_cycle_count\": 3"),
        "stdout:\n{stdout}"
    );
    assert!(root.join("state/status.json").exists());
    assert!(root.join("state/agent_spec.locked.json").exists());
    assert!(root.join("state/continuity.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());
    assert!(root.join("state/cycle_ledger.jsonl").exists());
    assert!(root.join("state/provider_binding_history.jsonl").exists());
    assert!(root.join("state/memory_index.json").exists());
    for cycle_id in ["cycle-000001", "cycle-000002", "cycle-000003"] {
        let cycle_dir = root.join("state/cycles").join(cycle_id);
        for artifact in [
            "cycle_manifest.json",
            "observations.json",
            "decision_request.json",
            "decision_result.json",
            "run_ref.json",
            "memory_writes.jsonl",
            "guardrail_report.json",
            "cycle_summary.md",
        ] {
            assert!(
                cycle_dir.join(artifact).exists(),
                "missing {artifact} for {cycle_id}"
            );
        }
    }
    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 3);
    let continuity =
        fs::read_to_string(root.join("state/continuity.json")).expect("read continuity");
    assert!(continuity.contains(r#""continuity_kind": "pre_v0_92_handle""#));
    assert!(continuity.contains(r#""latest_cycle_id": "cycle-000003""#));

    let human_status = run_adl(&["agent", "status", "--spec", spec_str]);
    assert!(
        human_status.status.success(),
        "expected agent status success, stderr:\n{}",
        String::from_utf8_lossy(&human_status.stderr)
    );
    let human_stdout = String::from_utf8_lossy(&human_status.stdout);
    assert!(human_stdout.contains("agent: smoke-agent"));
    assert!(human_stdout.contains("state: completed"));

    let status = run_adl(&["agent", "status", "--spec", spec_str, "--json"]);
    assert!(
        status.status.success(),
        "expected agent status success, stderr:\n{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let status_stdout = String::from_utf8_lossy(&status.stdout);
    assert!(
        status_stdout.contains("\"state\": \"completed\""),
        "stdout:\n{status_stdout}"
    );

    let inspect = run_adl(&["agent", "inspect", "--spec", spec_str, "--json"]);
    assert!(
        inspect.status.success(),
        "expected agent inspect success, stderr:\n{}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    let inspect_stdout = String::from_utf8_lossy(&inspect.stdout);
    assert!(
        inspect_stdout.contains("\"schema\": \"adl.long_lived_agent_inspection_packet.v1\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout.contains("\"manifest\": \"cycles/cycle-000003/cycle_manifest.json\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout
            .contains("\"guardrail_report\": \"cycles/cycle-000003/guardrail_report.json\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout.contains("\"path\": \"continuity_checkpoint.json\""),
        "stdout:\n{inspect_stdout}"
    );

    let human_inspect = run_adl(&["agent", "inspect", "--spec", spec_str]);
    assert!(
        human_inspect.status.success(),
        "expected human agent inspect success, stderr:\n{}",
        String::from_utf8_lossy(&human_inspect.stderr)
    );
    let human_inspect_stdout = String::from_utf8_lossy(&human_inspect.stdout);
    assert!(human_inspect_stdout.contains("agent: smoke-agent"));
    assert!(human_inspect_stdout.contains("cycle: cycle-000003 success"));
    assert!(human_inspect_stdout.contains("proof: pass"));
}

#[test]
fn agent_restart_restores_checkpoint_and_reuses_next_cycle_id_without_duplicates() {
    let root = unique_test_temp_dir("agent-restart");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: restart-agent
display_name: Restart Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let disk_ready_env = [
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ];
    let first = run_adl_with_env(
        &[
            "agent",
            "run",
            "--spec",
            spec_str,
            "--max-cycles",
            "2",
            "--no-sleep",
            "--json",
        ],
        &disk_ready_env,
    );
    assert!(
        first.status.success(),
        "expected first run success, stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );

    fs::remove_file(root.join("state/status.json")).expect("remove status to force restore");

    let restored = run_adl_with_env(
        &["agent", "status", "--spec", spec_str, "--json"],
        &disk_ready_env,
    );
    assert!(
        restored.status.success(),
        "expected restored status success, stderr:\n{}",
        String::from_utf8_lossy(&restored.stderr)
    );
    let restored_stdout = String::from_utf8_lossy(&restored.stdout);
    assert!(
        restored_stdout.contains("\"last_cycle_id\": \"cycle-000002\""),
        "stdout:\n{restored_stdout}"
    );

    let replay_manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/continuity_replay_manifest.json"))
            .expect("read continuity replay manifest"),
    )
    .expect("parse continuity replay manifest");
    assert_eq!(
        replay_manifest["expected_resume"]["next_cycle_id"],
        "cycle-000003"
    );

    let second = run_adl_with_env(
        &[
            "agent",
            "run",
            "--spec",
            spec_str,
            "--max-cycles",
            "1",
            "--no-sleep",
            "--json",
        ],
        &disk_ready_env,
    );
    assert!(
        second.status.success(),
        "expected resumed run success, stderr:\n{}",
        String::from_utf8_lossy(&second.stderr)
    );

    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 3, "ledger:\n{ledger}");
    assert!(ledger.contains("\"cycle_id\":\"cycle-000001\""));
    assert!(ledger.contains("\"cycle_id\":\"cycle-000002\""));
    assert!(ledger.contains("\"cycle_id\":\"cycle-000003\""));

    let checkpoint: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/continuity_checkpoint.json"))
            .expect("read continuity checkpoint"),
    )
    .expect("parse continuity checkpoint");
    assert_eq!(checkpoint["latest_cycle_id"], "cycle-000003");
}

#[test]
fn csm_daemon_writes_status_checkpoints_and_otel_observability() {
    let root = unique_test_temp_dir("csm-daemon");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-agent
display_name: Daemon Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/daemon-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let spec_str = spec.to_str().expect("utf8 path");
    let log_str = observability_log.to_str().expect("utf8 log path");
    let otel_log_str = otel_log.to_str().expect("utf8 otel log path");
    let otel_status_str = otel_status.to_str().expect("utf8 otel status path");
    let disk_ready_env = [
        ("ADL_OBSERVABILITY_STDERR", "0"),
        ("ADL_OBSERVABILITY_LOG", log_str),
        ("ADL_OBSERVABILITY_HEARTBEAT_MS", "25"),
        ("ADL_OTEL_LOG", otel_log_str),
        ("ADL_OTEL_STATUS", otel_status_str),
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ];
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--test-supervisor-failure-after-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &disk_ready_env,
    );
    assert!(
        out.status.success(),
        "expected daemon success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"schema\": \"adl.long_lived_agent_daemon_status.v1\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"state\": \"completed\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"restart_policy\": \"always\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"service_mode\": \"bounded_test_only\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"bounded_test_mode\": true"),
        "stdout:\n{stdout}"
    );
    assert!(root.join("state/daemon_status.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(
        daemon_status["unsupported_permanence_claims"][0],
        "not_os_boot_persistent"
    );
    assert_eq!(daemon_status["restart_policy"], "always");
    assert_eq!(daemon_status["service_mode"], "bounded_test_only");
    assert_eq!(daemon_status["bounded_test_mode"], true);
    assert_eq!(
        daemon_status["runtime_capabilities"]["supervisor"]["restart_policy"],
        "always"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["supervisor"]["service_mode"],
        "permanent"
    );
    assert_eq!(daemon_status["trace_id"], "agent.daemon-agent.daemon");

    let operator_events = read_text_or_missing(&root.join("state/operator_events.jsonl"));
    assert_text_contains(
        &operator_events,
        "\"event\":\"daemon_started\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"event\":\"child_spawn\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"event\":\"checkpoint_write\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"trace_id\":\"agent.daemon-agent.daemon\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"restart_policy\":\"always\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"service_mode\":\"bounded_test_only\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"cycle_count_lifetime_boundary\":\"not_applicable\"",
        "operator events",
    );
    assert_text_contains(&operator_events, "\"otel\"", "operator events");

    let observability = read_text_or_missing(&observability_log);
    assert_text_contains(&observability, "command=csm", "observability log");
    assert_text_contains(&observability, "stage=csm_daemon", "observability log");
    assert_text_contains(&observability, "stage=daemon_started", "observability log");
    assert_text_contains(
        &observability,
        "stage=checkpoint_write",
        "observability log",
    );
    assert_text_contains(
        &observability,
        "otel_service_name=csm-runtime-daemon",
        "observability log",
    );
    assert_text_contains(
        &observability,
        "trace_id=agent.daemon-agent.daemon",
        "observability log",
    );

    let otel_events = read_text_or_missing(&otel_log);
    let otel_event_lines = otel_events
        .lines()
        .filter(|line| line.contains("\"schema\":\"adl.otel.event.v1\""))
        .count();
    assert!(
        otel_event_lines >= 4,
        "expected at least four retained OTel JSONL events, got {otel_event_lines}; events:\n{otel_events}\nobservability:\n{observability}\noperator_events:\n{operator_events}"
    );
    assert_text_contains(
        &otel_events,
        "\"schema\":\"adl.otel.event.v1\"",
        "OTel JSONL",
    );
    assert_text_contains(
        &otel_events,
        "\"name\":\"csm.daemon_started\"",
        "OTel JSONL",
    );
    assert_text_contains(&otel_events, "\"name\":\"csm.csm_daemon\"", "OTel JSONL");
    assert_text_contains(
        &otel_events,
        "\"trace_id\":\"agent.daemon-agent.daemon\"",
        "OTel JSONL",
    );
    assert_text_contains(
        &otel_events,
        "\"service.name\":\"csm-runtime-daemon\"",
        "OTel JSONL",
    );

    let otel_status: serde_json::Value =
        serde_json::from_str(&read_text_or_missing(&otel_status)).expect("parse otel status");
    assert_eq!(otel_status["schema"], "adl.otel.monitor_status.v1");
    let status_event_count = otel_status["event_count"].as_u64().expect("event count");
    assert!(
        status_event_count >= 4,
        "expected OTel monitor status to observe at least four events, got {status_event_count}; status:\n{}\nevents:\n{otel_events}",
        serde_json::to_string_pretty(&otel_status).expect("render otel status")
    );
    assert_eq!(
        otel_status["last_trace_id"],
        "agent.daemon-agent.daemon",
        "status:\n{}\nevents:\n{otel_events}",
        serde_json::to_string_pretty(&otel_status).expect("render otel status")
    );
}

#[test]
fn csm_runtime_api_serves_status_health_ready_metrics_and_events() {
    let root = unique_test_temp_dir("csm-runtime-api");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: api-agent
display_name: API Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp07_api_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/api-agent
  write_policy: append_only
"#,
    )
    .expect("write API agent spec");

    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let spec_str = spec.to_str().expect("utf8 spec path");
    let (daemon_api_probe, daemon_api_bind) =
        reserve_csm_test_port("runtime API smoke bounded daemon");
    drop(daemon_api_probe);
    let daemon = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--api-bind",
            &daemon_api_bind,
            "--test-supervisor-failure-after-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
            ("ADL_OBSERVABILITY_HEARTBEAT_MS", "25"),
            (
                "ADL_OTEL_LOG",
                otel_log.to_str().expect("utf8 otel log path"),
            ),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("utf8 otel status path"),
            ),
            ("ADL_CSM_DISK_FLOOR_BYTES", "1"),
        ],
    );
    assert!(
        daemon.status.success(),
        "expected daemon success, stderr:\n{}",
        String::from_utf8_lossy(&daemon.stderr)
    );
    let backpressure_dir = root.join("backpressure-proof");
    let backpressure = run_csm_with_env(
        &[
            "backpressure",
            "prove",
            "--spec",
            spec_str,
            "--out",
            backpressure_dir.to_str().expect("utf8 backpressure dir"),
            "--profile",
            "soak2",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
        ],
    );
    assert!(
        backpressure.status.success(),
        "expected backpressure proof success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&backpressure.stdout),
        String::from_utf8_lossy(&backpressure.stderr)
    );
    let backpressure_stdout: serde_json::Value =
        serde_json::from_slice(&backpressure.stdout).expect("parse backpressure stdout");
    assert_eq!(
        backpressure_stdout["schema"],
        "adl.csm.backpressure_command_result.v1"
    );
    assert_eq!(backpressure_stdout["status"], "passed");
    let backpressure_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(backpressure_dir.join("backpressure_report.json"))
            .expect("read backpressure report"),
    )
    .expect("parse backpressure report");
    assert_eq!(
        backpressure_report["schema"],
        "adl.csm.backpressure_report.v1"
    );
    assert_eq!(
        backpressure_report["summary"]["required_state_silently_dropped"],
        false
    );
    assert_eq!(
        backpressure_report["typed_channel_runtime_proof"]["status"],
        "passed"
    );
    assert_eq!(
        backpressure_report["typed_channel_runtime_proof"]["channel_count"],
        7
    );
    assert_eq!(
        backpressure_report["typed_channel_runtime_proof"]["required_state_silently_dropped"],
        false
    );
    let channel_observations = backpressure_report["typed_channel_runtime_proof"]["observations"]
        .as_array()
        .expect("live channel observations");
    assert!(channel_observations.iter().any(|observation| {
        observation["receipt"]["outcome"] == "durably_spooled"
            && observation["snapshot_before_publish_ack"]["durable_spool_depth"] == 1
    }));
    let cloud_observation = channel_observations
        .iter()
        .find(|observation| observation["channel"] == "cloud_bridge_to_aws_routes")
        .expect("cloud bridge observation");
    assert_eq!(cloud_observation["receipt"]["cursor_may_advance"], false);
    assert_eq!(
        cloud_observation["snapshot_before_publish_ack"]["durable_spool_depth"],
        1
    );
    assert_eq!(
        cloud_observation["publish_ack_status"],
        "waiting_for_live_transport_receipt"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["action"],
        "safe_fail_serialize"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["status"],
        "verified"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["artifact_schema"],
        "adl.csm.safe_fail_bundle.v1"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["agent_outcome_state"],
        "sleeping"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["recoverability_class"],
        "recoverable_sleeping"
    );
    let proved_surfaces: std::collections::BTreeSet<_> = backpressure_report["proof_cases"]
        .as_array()
        .expect("proof cases")
        .iter()
        .map(|case| case["surface"].as_str().expect("proof surface"))
        .collect();
    for expected_surface in [
        "runtime_loop",
        "event_export",
        "checkpoint_write",
        "snapshot_diff",
        "dag_execution",
        "provider_call",
        "cloud_hook",
        "continuity_serialization",
    ] {
        assert!(
            proved_surfaces.contains(expected_surface),
            "missing proof case for {expected_surface}"
        );
    }
    assert!(root.join("state/csm_backpressure_state.json").exists());
    let safe_fail_bundle = root.join("state/safe_fail_bundle.json");
    let safe_fail_bundle_backup = root.join("state/safe_fail_bundle.json.bak");
    fs::rename(&safe_fail_bundle, &safe_fail_bundle_backup).expect("hide safe-fail bundle");
    let missing_bundle = run_csm(&[
        "backpressure",
        "prove",
        "--spec",
        spec_str,
        "--out",
        root.join("missing-bundle-backpressure")
            .to_str()
            .expect("utf8 missing bundle backpressure"),
        "--profile",
        "local",
        "--json",
    ]);
    fs::rename(&safe_fail_bundle_backup, &safe_fail_bundle).expect("restore safe-fail bundle");
    assert!(
        !missing_bundle.status.success(),
        "expected missing safe-fail bundle rejection"
    );
    assert!(String::from_utf8_lossy(&missing_bundle.stderr)
        .contains("missing required safe-fail bundle"));
    let bad_profile = run_csm(&[
        "backpressure",
        "prove",
        "--spec",
        spec_str,
        "--out",
        root.join("bad-backpressure")
            .to_str()
            .expect("utf8 bad backpressure"),
        "--profile",
        "unbounded",
        "--json",
    ]);
    assert!(
        !bad_profile.status.success(),
        "expected unsupported backpressure profile rejection"
    );
    assert!(String::from_utf8_lossy(&bad_profile.stderr)
        .contains("unsupported csm backpressure profile"));

    let (control_plane_url, control_plane_requests, control_plane) = spawn_loopback_control_plane();
    let (probe, addr) = reserve_csm_test_port("runtime API smoke");
    drop(probe);
    let mut child = runtime_test_command(resolve_csm_exe())
        .args([
            "daemon",
            "--spec",
            spec_str,
            "--api-bind",
            &addr,
            "--checkpoint-interval-secs",
            "1",
            "--interval-secs",
            "1",
        ])
        .env(
            "ADL_OTEL_STATUS",
            otel_status.to_str().expect("utf8 otel status path"),
        )
        .env(
            "ADL_OTEL_LOG",
            otel_log.to_str().expect("utf8 otel log path"),
        )
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live")
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1")
        .env("ADL_CSM_NOTICE_REQUIRED_CHANNEL", "control_plane")
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https")
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_URL", &control_plane_url)
        .env("ADL_CSM_DISK_FLOOR_BYTES", "1")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn csm daemon with embedded API");

    let api_state_root = root.join("state");
    let mut status = http_get_json_authenticated(&addr, &api_state_root, "/status");
    let status_wait_started = std::time::Instant::now();
    while status["daemon_liveness"]["state"] != "running"
        || status["typed_channels"]["last_event"] != "cycle_observability_record"
    {
        if status_wait_started.elapsed() > std::time::Duration::from_secs(20) {
            panic!(
                "embedded CSM API did not report ready running daemon state:\n{}",
                serde_json::to_string_pretty(&status).expect("serialize status")
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        status = http_get_json_authenticated(&addr, &api_state_root, "/status");
    }
    let health = http_get_json_authenticated(&addr, &api_state_root, "/health");
    let ready = http_get_json_authenticated(&addr, &api_state_root, "/ready");
    let metrics = http_get_json_authenticated(&addr, &api_state_root, "/metrics");
    let events = http_get_json_authenticated(&addr, &api_state_root, "/events");
    let shepherd = http_get_json_authenticated(&addr, &api_state_root, "/shepherd");

    request_governed_stop_and_wait(&spec, &mut child);
    control_plane.join().expect("join loopback control plane");
    let published_notices: Vec<_> = control_plane_requests.try_iter().collect();
    assert!(
        !published_notices.is_empty(),
        "governed shutdown must publish at least one live control-plane notice"
    );
    assert!(published_notices.iter().all(|request| {
        request
            .lines()
            .any(|line| line.to_ascii_lowercase().starts_with("idempotency-key:"))
    }));
    assert!(published_notices
        .iter()
        .any(|request| request.contains("\"notice_kind\":\"graceful_shutdown\"")));

    let shutdown_state: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("state/csm_shutdown_state.json")).expect("shutdown state"),
    )
    .expect("parse shutdown state");
    let shutdown_disposition: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("state/csm_shutdown_disposition.json")).expect("shutdown disposition"),
    )
    .expect("parse shutdown disposition");
    let observed_phases: Vec<_> = shutdown_state["steps"]
        .as_array()
        .expect("shutdown steps")
        .iter()
        .map(|step| step["phase"].as_str().expect("shutdown phase"))
        .collect();
    assert_eq!(
        observed_phases,
        [
            "quiesce_admission",
            "drain_work",
            "flush_continuity",
            "close_lifelog",
            "drain_observability",
            "final_cloud_notices",
            "join_components",
            "retain_disposition",
        ]
    );
    assert_eq!(shutdown_state["status"], "shutdown_complete");
    assert_eq!(shutdown_state["admission_quiesced"], true);
    assert_eq!(shutdown_disposition["status"], "retained");
    assert_eq!(shutdown_disposition["final_state"], "governed_stopped");
    assert_eq!(shutdown_disposition["publishable"], true);
    assert_eq!(shutdown_disposition["blocked_count"], 0);
    let cloud_notice_step = shutdown_disposition["steps"]
        .as_array()
        .expect("disposition steps")
        .iter()
        .find(|step| step["phase"] == "final_cloud_notices")
        .expect("final cloud notice step");
    assert_eq!(cloud_notice_step["outcome"], "completed");
    assert_eq!(
        cloud_notice_step["detail"]["notice"]["typed_channel_delivery"]["provider_receipt_id"],
        "shutdown-live-receipt-1"
    );
    assert!(shutdown_disposition["steps"]
        .as_array()
        .expect("disposition steps")
        .iter()
        .any(|step| step["phase"] == "join_components"));

    assert_eq!(status["schema"], "adl.csm.runtime_api.status.v1");
    assert_eq!(
        status["networking"]["listeners"][0]["default_bind"],
        "127.0.0.1:19997"
    );
    assert_eq!(
        status["pooling_plan"]["decision_summary"],
        "CSM runtime pooling uses the deadpool crate for governed bounded resource-slot mechanics; protocol-specific clients may still perform native reuse inside checked-out deadpool slots."
    );
    assert_eq!(status["pooling_plan"]["pool_crate"], "deadpool");
    assert_eq!(
        status["pooling_plan"]["pool_backend"],
        "deadpool::unmanaged"
    );
    assert_eq!(
        status["connection_pool_status"]["schema"],
        "adl.csm.connection_pool_status.v1"
    );
    assert_eq!(status["connection_pool_status"]["status"], "configured");
    assert_eq!(
        status["connection_pool_status"]["roles"][0]["pool_backend"],
        "deadpool::unmanaged"
    );
    assert_eq!(status["runtime_owner"], "csm");
    assert_eq!(status["agent_instance_id"], "api-agent");
    assert_eq!(status["status"], "degraded");
    assert_eq!(status["ready"], "not_ready");
    assert_eq!(status["daemon_liveness"]["state"], "running");
    assert_eq!(
        status["daemon_liveness"]["supervisor_pid_liveness"],
        "live_pid"
    );
    assert_eq!(
        status["backpressure"]["schema"],
        "adl.csm.backpressure_state.v1"
    );
    assert_eq!(status["typed_channels"]["status"], "ready");
    assert_eq!(
        status["typed_channels"]["schema"],
        "adl.csm.typed_channel_state.v1"
    );
    assert_eq!(status["typed_channels"]["summary"]["channel_count"], 7);
    assert_eq!(
        status["typed_channels"]["last_event"],
        "cycle_observability_record"
    );
    assert_eq!(status["scheduler"]["status"], "integrated");
    assert_eq!(status["chronosense"]["status"], "integrated");
    assert_eq!(status["aee_resilience"]["status"], "integrated");
    assert_eq!(
        status["polis_shepherd_agent"]["component"],
        "polis_shepherd_agent"
    );
    assert_eq!(
        status["polis_shepherd_agent"]["capability"]["model_policy"]["candidate"]["model"],
        "gemma4:12b-mlx"
    );
    assert_eq!(
        status["polis_shepherd_agent"]["decision"]["authority"],
        "advisory"
    );
    assert_eq!(shepherd["schema"], "adl.csm.runtime_api.shepherd.v1");
    assert_eq!(
        shepherd["component"]["model_policy"]["defaulting_rule"],
        "gemma4:12b-mlx_not_default_until_shepherd_eval_passes"
    );
    assert_eq!(
        status["otel"]["status"]["schema"],
        "adl.otel.monitor_status.v1"
    );
    assert_eq!(health["schema"], "adl.csm.runtime_api.health.v1");
    assert_eq!(health["status"], "degraded");
    assert_eq!(ready["schema"], "adl.csm.runtime_api.ready.v1");
    assert_eq!(ready["ready"], "not_ready");
    assert!(ready["blocking_reasons"]
        .as_array()
        .expect("ready blockers")
        .contains(&serde_json::json!("curiosity_engine_not_ready")));
    assert_eq!(status["reasoning_runtime"]["status"], "serialized");
    assert_eq!(status["reasoning_runtime"]["value"]["health"], "ready");
    assert_eq!(metrics["schema"], "adl.csm.runtime_api.metrics.v1");
    assert_eq!(metrics["gauges"]["backpressure_queue_depth"], 12);
    assert_eq!(metrics["gauges"]["backpressure_lag_ms"], 3100);
    assert_eq!(metrics["gauges"]["backpressure_deferred_count"], 23);
    assert_eq!(metrics["gauges"]["backpressure_shed_count"], 7);
    assert_eq!(metrics["gauges"]["typed_channel_count"], 7);
    assert_eq!(metrics["gauges"]["typed_channel_queue_depth"], 0);
    assert_eq!(metrics["gauges"]["typed_channel_durable_spool_depth"], 0);
    assert_eq!(metrics["gauges"]["typed_channel_blocked_count"], 0);
    assert_eq!(metrics["gauges"]["typed_channel_throttled_count"], 0);
    assert_eq!(metrics["gauges"]["typed_channel_shed_count"], 0);
    assert_eq!(
        metrics["states"]["backpressure_health"],
        "capacity_degraded"
    );
    assert_eq!(
        metrics["states"]["backpressure_safe_fail_action"],
        "safe_fail_serialize"
    );
    assert_eq!(metrics["states"]["typed_channel_readiness"], "ready");
    assert!(
        matches!(
            metrics["states"]["agent_state"].as_str(),
            Some("idle" | "running_cycle" | "completed")
        ),
        "unexpected metrics state: {}",
        metrics["states"]["agent_state"]
    );
    assert_eq!(events["schema"], "adl.csm.runtime_api.events.v1");
    assert!(events["events"]["entries"]
        .as_array()
        .expect("events array")
        .iter()
        .any(|event| event["event"] == "daemon_started"));
    let observability = fs::read_to_string(&observability_log).expect("read observability log");
    assert!(observability.contains("stage=backpressure_policy"));
    assert!(observability.contains("safe_fail_action=safe_fail_serialize"));

    for response in [&status, &health, &ready, &metrics, &events] {
        let raw = serde_json::to_string(response).expect("serialize API response");
        assert!(!raw.contains("/Users/"), "leaked host path:\n{raw}");
        assert!(
            !raw.contains("Authorization:"),
            "leaked auth header:\n{raw}"
        );
        assert!(!raw.contains("Bearer "), "leaked bearer token:\n{raw}");
    }
}

#[test]
fn csm_governed_shutdown_retains_continuity_and_publish_failures_without_false_success() {
    let continuity_root = unique_test_temp_dir("csm-shutdown-continuity-failure");
    let continuity_spec = write_shutdown_probe_spec(&continuity_root, "continuity-failure-agent");
    let (control_plane_url, _requests, control_plane) = spawn_loopback_control_plane();
    let mut continuity_child = std::process::Command::new(resolve_csm_exe())
        .args([
            "daemon",
            "--spec",
            continuity_spec.to_str().expect("utf8 continuity spec"),
            "--checkpoint-interval-secs",
            "1",
            "--interval-secs",
            "1",
        ])
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live")
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1")
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https")
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_URL", &control_plane_url)
        .env("ADL_CSM_TEST_SHUTDOWN_CONTINUITY_FAILURE", "1")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn continuity-failure daemon");
    wait_for_shutdown_probe_running(&continuity_root);
    request_governed_stop_and_wait(&continuity_spec, &mut continuity_child);
    control_plane.join().expect("join continuity control plane");
    let continuity_disposition: serde_json::Value = serde_json::from_slice(
        &fs::read(continuity_root.join("state/csm_shutdown_disposition.json"))
            .expect("continuity failure disposition"),
    )
    .expect("parse continuity failure disposition");
    assert_eq!(continuity_disposition["publishable"], false);
    let continuity_step = continuity_disposition["steps"]
        .as_array()
        .expect("continuity steps")
        .iter()
        .find(|step| step["phase"] == "flush_continuity")
        .expect("continuity phase");
    assert_eq!(continuity_step["outcome"], "blocked");
    assert!(continuity_step["detail"]["continuity_flush_error"].is_string());

    let publish_root = unique_test_temp_dir("csm-shutdown-publish-blocked");
    let publish_spec = write_shutdown_probe_spec(&publish_root, "publish-blocked-agent");
    let mut publish_child = std::process::Command::new(resolve_csm_exe())
        .args([
            "daemon",
            "--spec",
            publish_spec.to_str().expect("utf8 publish spec"),
            "--checkpoint-interval-secs",
            "1",
            "--interval-secs",
            "1",
        ])
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live")
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1")
        .env("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https")
        .env(
            "ADL_CSM_NOTICE_CONTROL_PLANE_URL",
            "http://127.0.0.1:9/unreachable",
        )
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn publish-blocked daemon");
    wait_for_shutdown_probe_running(&publish_root);
    request_governed_stop_and_wait(&publish_spec, &mut publish_child);
    let publish_disposition: serde_json::Value = serde_json::from_slice(
        &fs::read(publish_root.join("state/csm_shutdown_disposition.json"))
            .expect("publish-blocked disposition"),
    )
    .expect("parse publish-blocked disposition");
    assert_eq!(publish_disposition["publishable"], false);
    assert_eq!(publish_disposition["blocked_count"], 1);
    let cloud_step = publish_disposition["steps"]
        .as_array()
        .expect("publish steps")
        .iter()
        .find(|step| step["phase"] == "final_cloud_notices")
        .expect("cloud notice phase");
    assert_eq!(cloud_step["outcome"], "blocked");
    assert_eq!(
        cloud_step["detail"]["notice"]["typed_channel_delivery"]["cursor_advanced"],
        false
    );
}

#[test]
fn csm_continuity_capsule_captures_stages_and_rejects_unsafe_bundles() {
    let root = unique_test_temp_dir("csm-continuity-capsule");
    let (api_probe, api_bind) = reserve_ephemeral_csm_test_port("continuity capsule API");
    let custody_p256_signing_private_key = "CQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQk=";
    let custody_trusted_public_key =
        custody_public_key_from_private_key(custody_p256_signing_private_key);
    let custody_signing_key_id = "test-csm-custody-key";
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: continuity-agent
display_name: Continuity Agent
state_root: state
workflow:
  kind: demo_adapter
  name: continuity_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/continuity-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    drop(api_probe);

    let observability_log = root.join("daemon-observability.log");
    let otel_log = root.join("daemon-otel.jsonl");
    let otel_status = root.join("daemon-otel-status.json");
    let (otel_endpoint, _captured_otel, shutdown_otel_collector, otel_collector) =
        spawn_loopback_otlp_collector();
    let daemon = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--test-supervisor-failure-after-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--api-bind",
            &api_bind,
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
            ("ADL_OBSERVABILITY_HEARTBEAT_MS", "25"),
            (
                "ADL_OTEL_LOG",
                otel_log.to_str().expect("utf8 otel log path"),
            ),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("utf8 otel status path"),
            ),
            CSM_DISK_READY_ENV[0],
            CSM_DISK_READY_ENV[1],
            ("ADL_OTEL_EXPORTER_OTLP_ENDPOINT", otel_endpoint.as_str()),
            ("ADL_OTEL_EXPORTER_TIMEOUT_MS", "2000"),
        ],
    );
    shutdown_otel_collector
        .send(())
        .expect("signal continuity OTLP collector shutdown");
    otel_collector
        .join()
        .expect("join continuity OTLP collector");
    assert!(
        daemon.status.success(),
        "expected daemon success, stderr:\n{}",
        String::from_utf8_lossy(&daemon.stderr)
    );

    let bundle = root.join("continuity-capsule");
    let capture = run_csm_with_env(
        &[
            "continuity",
            "capture",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--out",
            bundle.to_str().expect("utf8 bundle"),
            "--source-host",
            "wuji",
            "--target-host",
            "ec2-staging",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
            (
                "ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64",
                custody_p256_signing_private_key,
            ),
            ("ADL_CSM_CUSTODY_SIGNING_KEY_ID", custody_signing_key_id),
            (
                "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64",
                custody_trusted_public_key.as_str(),
            ),
        ],
    );
    assert!(
        capture.status.success(),
        "expected continuity capture success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&capture.stdout),
        String::from_utf8_lossy(&capture.stderr)
    );
    let capture_stdout: serde_json::Value =
        serde_json::from_slice(&capture.stdout).expect("parse capture stdout");
    assert_eq!(
        capture_stdout["schema"],
        "adl.csm.continuity_capsule_command_result.v1"
    );
    assert_eq!(capture_stdout["operation"], "capture");
    assert_eq!(capture_stdout["status"], "captured");

    let manifest_path = bundle.join("continuity_capsule_manifest.json");
    let manifest_text = fs::read_to_string(&manifest_path).expect("read continuity manifest");
    assert!(
        !manifest_text.contains(root.to_str().expect("root utf8")),
        "continuity capsule manifest leaked host path:\n{manifest_text}"
    );
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_text).expect("parse continuity manifest");
    assert_eq!(manifest["schema"], "adl.csm.continuity_capsule.v1");
    assert_eq!(manifest["format_version"], "csm.continuity-capsule.v1");
    assert_eq!(manifest["runtime_owner"], "csm");
    assert_eq!(manifest["source_host"], "wuji");
    assert_eq!(manifest["target_host"], "ec2-staging");
    assert_eq!(
        manifest["rebind_policy"]["aws"]["default_profile"],
        "agent-logic-admin"
    );
    assert!(manifest["artifacts"]
        .as_array()
        .expect("artifact array")
        .iter()
        .any(|artifact| artifact["role"] == "continuity_checkpoint"));
    let binary_segments = manifest["binary_segments"]
        .as_array()
        .expect("binary segment array");
    let checkpoint_segment = binary_segments
        .iter()
        .find(|segment| segment["role"] == "continuity_checkpoint_snapshot")
        .expect("continuity checkpoint binary segment");
    assert_eq!(checkpoint_segment["schema"], "adl.csm.snapshot_segment.v1");
    assert_eq!(
        checkpoint_segment["format_version"],
        "csm.snapshot-segment.v1"
    );
    assert_eq!(
        checkpoint_segment["source_ref"],
        "continuity_checkpoint.json"
    );
    assert!(checkpoint_segment["hash_address"]
        .as_str()
        .expect("hash address")
        .starts_with("sha256:"));
    assert!(bundle
        .join(
            checkpoint_segment["segment_ref"]
                .as_str()
                .expect("segment ref")
        )
        .exists());
    assert_eq!(manifest["custody_manifest_ref"], "custody_manifest.json");
    let custody_path = bundle.join("custody_manifest.json");
    let custody_text = fs::read_to_string(&custody_path).expect("read custody manifest");
    assert!(
        !custody_text.contains(root.to_str().expect("root utf8")),
        "custody manifest leaked host path:\n{custody_text}"
    );
    let custody: serde_json::Value =
        serde_json::from_str(&custody_text).expect("parse custody manifest");
    assert_eq!(
        custody["schema"],
        "adl.csm.polis_artifact_custody_manifest.v1"
    );
    assert_eq!(custody["format_version"], "csm.polis-custody.v1");
    assert_eq!(
        custody["capsule_manifest_ref"],
        "continuity_capsule_manifest.json"
    );
    assert!(custody["capsule_instance_id"]
        .as_str()
        .expect("capsule instance id")
        .starts_with("sha256:"));
    assert_eq!(
        custody["signature"]["schema"],
        "adl.csm.polis_artifact_custody_signature.v1"
    );
    assert_eq!(custody["signature"]["alg"], "ecdsa-p256-sha256");
    assert_eq!(custody["signature"]["key_id"], custody_signing_key_id);
    assert_eq!(
        custody["signature"]["public_key_b64"],
        custody_trusted_public_key
    );
    assert!(
        custody["signature"]["public_key_b64"]
            .as_str()
            .expect("public key")
            .len()
            > 40
    );
    assert!(
        custody["signature"]["sig_b64"]
            .as_str()
            .expect("signature")
            .len()
            > 80
    );
    assert_eq!(
        custody["signature"]["signed_payload"]["canonical_json_profile"],
        "adl.csm.polis_custody.canonical_json.sorted_serde_json.v1"
    );
    assert_eq!(
        custody["signature"]["signed_payload"]["excluded_fields"][0],
        "signature"
    );
    assert!(custody["signature"]["signed_payload"]["payload_sha256"]
        .as_str()
        .expect("signed payload sha")
        .starts_with("sha256:"));
    let custody_artifacts = custody["artifacts"].as_array().expect("custody artifacts");
    assert!(custody_artifacts.iter().any(|artifact| {
        artifact["artifact_id"] == "continuity-capsule-manifest"
            && artifact["storage_location"] == "continuity_capsule_manifest.json"
            && artifact["parent"].is_null()
    }));
    assert!(custody_artifacts.iter().any(|artifact| {
        artifact["artifact_id"] == "artifact:continuity_checkpoint:continuity_checkpoint.json"
            && artifact["storage_location"] == "state/continuity_checkpoint.json"
            && artifact["parent"]["artifact_id"] == "continuity-capsule-manifest"
    }));
    assert!(custody_artifacts.iter().any(|artifact| {
        artifact["artifact_id"]
            == "binary-segment:continuity_checkpoint_snapshot:continuity_checkpoint.json"
            && artifact["storage_location"] == "segments/continuity_checkpoint.snapshot.segment"
            && artifact["parent"]["artifact_id"] == "continuity-capsule-manifest"
    }));
    assert!(bundle.join("state/continuity_checkpoint.json").exists());
    assert!(bundle.join("state/operator_events.jsonl").exists());
    let custody_trust_env = [(
        "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64",
        custody_trusted_public_key.as_str(),
    )];

    let staged = root.join("ec2-staged");
    let stage = run_csm_with_env(
        &[
            "continuity",
            "stage",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            staged.to_str().expect("utf8 staged"),
            "--target-host",
            "ec2-staging",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
            (
                "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64",
                custody_trusted_public_key.as_str(),
            ),
        ],
    );
    assert!(
        stage.status.success(),
        "expected continuity stage success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&stage.stdout),
        String::from_utf8_lossy(&stage.stderr)
    );
    let stage_stdout: serde_json::Value =
        serde_json::from_slice(&stage.stdout).expect("parse stage stdout");
    assert_eq!(stage_stdout["operation"], "stage");
    assert_eq!(stage_stdout["status"], "staged");
    let stage_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(staged.join("stage_report.json")).expect("read stage report"),
    )
    .expect("parse stage report");
    assert_eq!(
        stage_report["schema"],
        "adl.csm.continuity_capsule_stage_report.v1"
    );
    assert_eq!(stage_report["status"], "staged");
    assert_eq!(
        stage_report["custody_manifest_ref"],
        "custody_manifest.json"
    );
    assert!(staged
        .join("staged_state/continuity_checkpoint.json")
        .exists());

    let restored = root.join("restored-runtime");
    let restore = run_csm_with_env(
        &[
            "continuity",
            "restore",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            restored.to_str().expect("utf8 restored"),
            "--target-host",
            "ec2-staging",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
            (
                "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64",
                custody_trusted_public_key.as_str(),
            ),
        ],
    );
    assert!(
        restore.status.success(),
        "expected continuity restore success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&restore.stdout),
        String::from_utf8_lossy(&restore.stderr)
    );
    let restore_stdout: serde_json::Value =
        serde_json::from_slice(&restore.stdout).expect("parse restore stdout");
    assert_eq!(restore_stdout["operation"], "restore");
    assert_eq!(restore_stdout["status"], "restored");
    let restore_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(restored.join("restore_report.json")).expect("read restore report"),
    )
    .expect("parse restore report");
    assert_eq!(
        restore_report["schema"],
        "adl.csm.continuity_capsule_restore_report.v1"
    );
    assert_eq!(restore_report["status"], "restored");
    assert_eq!(
        restore_report["custody_manifest_ref"],
        "custody_manifest.json"
    );
    assert!(restored.join("agent.yaml").exists());
    assert!(restored.join("state/continuity_checkpoint.json").exists());

    let restored_daemon = run_csm(&[
        "daemon",
        "--spec",
        restored
            .join("agent.yaml")
            .to_str()
            .expect("utf8 restored spec"),
        "--test-supervisor-failure-after-restarts",
        "1",
        "--checkpoint-interval-secs",
        "1",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        restored_daemon.status.success(),
        "expected restored daemon fire-up success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&restored_daemon.stdout),
        String::from_utf8_lossy(&restored_daemon.stderr)
    );

    let ec2_blocked = root.join("ec2-blocked");
    let ec2_stage = run_csm_with_env(
        &[
            "continuity",
            "stage",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            ec2_blocked.to_str().expect("utf8 ec2 blocked"),
            "--target-host",
            "ec2",
            "--json",
        ],
        &custody_trust_env,
    );
    assert!(
        ec2_stage.status.success(),
        "expected bounded EC2 stage packet success, stderr:\n{}",
        String::from_utf8_lossy(&ec2_stage.stderr)
    );
    let ec2_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(ec2_blocked.join("stage_report.json")).expect("read ec2 report"),
    )
    .expect("parse ec2 report");
    assert_eq!(ec2_report["status"], "blocked");
    assert_eq!(ec2_report["rebind_policy"]["target_host"], "ec2");
    assert_eq!(
        ec2_report["blockers"][0]["required_profile"],
        "agent-logic-admin"
    );

    let drill_dir = root.join("fire-drill");
    let drill = run_csm_with_env(
        &[
            "continuity",
            "drill",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            drill_dir.to_str().expect("utf8 drill"),
            "--target-host",
            "local",
            "--cadence",
            "daily",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
            (
                "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64",
                custody_trusted_public_key.as_str(),
            ),
        ],
    );
    assert!(
        drill.status.success(),
        "expected continuity drill success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&drill.stdout),
        String::from_utf8_lossy(&drill.stderr)
    );
    let drill_stdout: serde_json::Value =
        serde_json::from_slice(&drill.stdout).expect("parse drill stdout");
    assert_eq!(drill_stdout["operation"], "fire_drill");
    assert_eq!(
        drill_stdout["status"],
        "passed",
        "drill stdout:\n{}\ndrill stderr:\n{}",
        String::from_utf8_lossy(&drill.stdout),
        String::from_utf8_lossy(&drill.stderr)
    );
    assert_eq!(drill_stdout["report_ref"], "fire_drill_report.json");
    let drill_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(drill_dir.join("fire_drill_report.json")).expect("read drill report"),
    )
    .expect("parse drill report");
    assert_eq!(
        drill_report["schema"],
        "adl.csm.continuity_fire_drill_report.v1"
    );
    assert_eq!(
        drill_report["status"],
        "passed",
        "drill report:\n{}",
        serde_json::to_string_pretty(&drill_report).expect("format drill report")
    );
    assert_eq!(drill_report["source_bundle_ref"], "../continuity-capsule");
    assert_eq!(
        drill_report["manifest_ref"],
        "../continuity-capsule/continuity_capsule_manifest.json"
    );
    assert_eq!(drill_report["cadence_policy"]["selected"], "daily");
    assert_eq!(drill_report["safety"]["mutates_live_runtime_state"], false);
    assert_eq!(drill_report["stage"]["status"], "staged");
    assert_eq!(drill_report["restore"]["status"], "restored");
    assert!(drill_dir
        .join("restored-runtime/state/continuity_checkpoint.json")
        .exists());
    let negative_cases = drill_report["negative_cases"]
        .as_array()
        .expect("negative cases");
    assert_eq!(negative_cases.len(), 4);
    assert!(negative_cases
        .iter()
        .all(|case| case["status"] == "failed_as_expected"));
    assert!(negative_cases
        .iter()
        .any(|case| case["case_id"] == "missing_custody_manifest"));
    assert!(negative_cases
        .iter()
        .any(|case| case["case_id"] == "wrong_custody_parent"));
    assert_eq!(
        drill_report["rto_rpo_measurement"]["rpo_scope"],
        "selected_continuity_capsule_point_in_time"
    );
    let overlapping_drill = run_csm_with_env(
        &[
            "continuity",
            "drill",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            bundle.join("nested-drill").to_str().expect("utf8 nested"),
            "--target-host",
            "local",
            "--json",
        ],
        &custody_trust_env,
    );
    assert!(
        !overlapping_drill.status.success(),
        "expected overlapping drill output rejection"
    );
    assert!(String::from_utf8_lossy(&overlapping_drill.stderr)
        .contains("must be disjoint from --bundle"));
    assert!(
        bundle.join("continuity_capsule_manifest.json").exists(),
        "overlapping drill rejection must preserve source bundle"
    );

    let observability =
        fs::read_to_string(&observability_log).expect("read continuity observability");
    assert!(observability.contains("stage=continuity_capsule_capture"));
    assert!(observability.contains("stage=continuity_capsule_stage"));
    assert!(observability.contains("stage=continuity_capsule_restore"));
    assert!(observability.contains("stage=continuity_fire_drill"));
    assert!(observability.contains("otel_service_name=csm-runtime-daemon"));

    assert_stage_failure_with_env(
        &bundle,
        &root.join("bad-version"),
        &root.join("bad-version-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read bad manifest"))
                    .expect("parse bad manifest");
            value["format_version"] = serde_json::json!("csm.continuity-capsule.v0");
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize bad manifest"),
            )
            .expect("write bad manifest");
        },
        "unsupported continuity capsule format version",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("missing-file"),
        &root.join("missing-file-stage"),
        &custody_trust_env,
        |bad| {
            fs::remove_file(bad.join("state/status.json")).expect("remove staged status");
        },
        "custody retained artifact missing",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("missing-custody-manifest"),
        &root.join("missing-custody-manifest-stage"),
        &custody_trust_env,
        |bad| {
            fs::remove_file(bad.join("custody_manifest.json")).expect("remove custody manifest");
        },
        "custody manifest missing bundle artifact",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("custody-modified-artifact"),
        &root.join("custody-modified-artifact-stage"),
        &custody_trust_env,
        |bad| {
            fs::write(
                bad.join("state/continuity_checkpoint.json"),
                "{\"schema\":\"tampered\"}\n",
            )
            .expect("tamper retained artifact");
        },
        "payload hash does not match source file",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("custody-wrong-parent"),
        &root.join("custody-wrong-parent-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("custody_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read custody"))
                    .expect("parse custody");
            value["artifacts"][1]["parent"]["sha256"] = serde_json::json!(
                "0000000000000000000000000000000000000000000000000000000000000000"
            );
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize custody"),
            )
            .expect("write custody");
        },
        "signed payload digest mismatch",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("custody-replay-guard"),
        &root.join("custody-replay-guard-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("custody_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read custody"))
                    .expect("parse custody");
            value["capsule_instance_id"] = serde_json::json!(
                "sha256:1111111111111111111111111111111111111111111111111111111111111111"
            );
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize custody"),
            )
            .expect("write custody");
        },
        "signed payload digest mismatch",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("custody-redacted-fields"),
        &root.join("custody-redacted-fields-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("custody_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read custody"))
                    .expect("parse custody");
            value["artifacts"][1]["redaction"]["redacted_fields"] = serde_json::json!(["sha256"]);
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize custody"),
            )
            .expect("write custody");
        },
        "signed payload digest mismatch",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("custody-signature-tamper"),
        &root.join("custody-signature-tamper-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("custody_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read custody"))
                    .expect("parse custody");
            value["signature"]["sig_b64"] = serde_json::json!("not-a-valid-signature");
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize custody"),
            )
            .expect("write custody");
        },
        "invalid base64 custody signature",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("custody-attacker-resigned"),
        &root.join("custody-attacker-resigned-stage"),
        &custody_trust_env,
        |bad| {
            let status_path = bad.join("state/status.json");
            fs::write(&status_path, "{\"schema\":\"attacker\"}\n")
                .expect("tamper retained status artifact");
            let tampered_status_sha = {
                let bytes = fs::read(&status_path).expect("read tampered status");
                format!("{:x}", Sha256::digest(&bytes))
            };
            let manifest_path = bad.join("continuity_capsule_manifest.json");
            let mut manifest: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&manifest_path).expect("read manifest"))
                    .expect("parse manifest");
            for artifact in manifest["artifacts"]
                .as_array_mut()
                .expect("manifest artifacts")
            {
                if artifact["bundle_ref"] == "state/status.json" {
                    artifact["sha256"] = serde_json::json!(tampered_status_sha);
                    artifact["bytes"] = serde_json::json!(fs::metadata(&status_path)
                        .expect("tampered status metadata")
                        .len());
                }
            }
            fs::write(
                &manifest_path,
                serde_json::to_vec_pretty(&manifest).expect("serialize manifest"),
            )
            .expect("write manifest");

            let custody_path = bad.join("custody_manifest.json");
            let mut custody: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&custody_path).expect("read custody"))
                    .expect("parse custody");
            for artifact in custody["artifacts"]
                .as_array_mut()
                .expect("custody artifacts")
            {
                if artifact["storage_location"] == "state/status.json" {
                    artifact["sha256"] = serde_json::json!(tampered_status_sha);
                    artifact["bytes"] = serde_json::json!(fs::metadata(&status_path)
                        .expect("tampered status metadata")
                        .len());
                }
            }
            sign_custody_value_with_private_key(
                &mut custody,
                "CAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAg=",
                "attacker-key",
            );
            fs::write(
                &custody_path,
                serde_json::to_vec_pretty(&custody).expect("serialize attacker custody"),
            )
            .expect("write attacker custody");
        },
        "public key does not match trusted verification key",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("truncated-manifest"),
        &root.join("truncated-manifest-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read manifest"))
                    .expect("parse manifest");
            value["artifacts"] = serde_json::json!([{
                "role": "recoverable_status",
                "source_ref": "status.json",
                "bundle_ref": "state/status.json",
                "sha256": value["artifacts"][1]["sha256"].clone(),
                "bytes": value["artifacts"][1]["bytes"].clone(),
                "required": true
            }]);
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize manifest"),
            )
            .expect("write manifest");
        },
        "missing required artifact role",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("path-leak"),
        &root.join("path-leak-stage"),
        &custody_trust_env,
        |bad| {
            fs::write(
                bad.join("state/status.json"),
                format!("{{\"path\":\"{}\"}}", bad.display()),
            )
            .expect("write path leak");
        },
        "host-private absolute path",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("linux-path-leak"),
        &root.join("linux-path-leak-stage"),
        &custody_trust_env,
        |bad| {
            fs::write(
                bad.join("state/status.json"),
                "{\"path\":\"/home/runner/work/agent-design-language\"}\n",
            )
            .expect("write linux path leak");
        },
        "host-private absolute path",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("credential-leak"),
        &root.join("credential-leak-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read manifest"))
                    .expect("parse manifest");
            value["api_key"] = serde_json::json!("not-exportable");
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize manifest"),
            )
            .expect("write manifest");
        },
        "credential-like key",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("missing-binary-segment-manifest-entry"),
        &root.join("missing-binary-segment-manifest-entry-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read manifest"))
                    .expect("parse manifest");
            value["binary_segments"] = serde_json::json!([]);
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize manifest"),
            )
            .expect("write manifest");
        },
        "missing required binary segment role",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("divergent-binary-segment-payload"),
        &root.join("divergent-binary-segment-payload-stage"),
        &custody_trust_env,
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read manifest"))
                    .expect("parse manifest");
            let segment = value["binary_segments"][0]
                .as_object_mut()
                .expect("binary segment object");
            segment.insert(
                "payload_sha256".to_string(),
                serde_json::json!(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                ),
            );
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize manifest"),
            )
            .expect("write manifest");
        },
        "payload hash does not match retained artifact",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("corrupted-segment"),
        &root.join("corrupted-segment-stage"),
        &custody_trust_env,
        |bad| {
            fs::write(
                bad.join("segments/continuity_checkpoint.snapshot.segment"),
                b"not-a-valid-segment",
            )
            .expect("corrupt binary segment");
        },
        "hash mismatch",
    );
    assert_stage_failure_with_env(
        &bundle,
        &root.join("corrupted-manifest"),
        &root.join("corrupted-manifest-stage"),
        &custody_trust_env,
        |bad| {
            fs::write(bad.join("continuity_capsule_manifest.json"), b"{").expect("corrupt");
        },
        "failed parsing",
    );

    let unsupported_stage = root.join("unsupported-stage");
    let unsupported = run_csm_with_env(
        &[
            "continuity",
            "stage",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            unsupported_stage.to_str().expect("utf8 stage"),
            "--target-host",
            "mars",
            "--json",
        ],
        &custody_trust_env,
    );
    assert!(
        !unsupported.status.success(),
        "expected unsupported target failure"
    );
    assert!(String::from_utf8_lossy(&unsupported.stderr)
        .contains("unsupported continuity capsule target host"));
}

#[test]
fn csm_credential_policy_proves_break_glass_and_negative_cases_without_secrets() {
    let root = unique_test_temp_dir("csm-credential-policy");
    let observability_log = root.join("credential-observability.log");
    let proof_dir = root.join("credential-proof");
    let out = run_csm_with_env_without_aws_credentials(
        &[
            "credential-policy",
            "prove",
            "--out",
            proof_dir.to_str().expect("utf8 proof dir"),
            "--run-id",
            "wp12-4920-smoke",
            "--operator",
            "operator-alice",
            "--requested-at",
            "2026-07-10T00:00:00Z",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
        ],
    );
    assert!(
        out.status.success(),
        "expected credential policy proof success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("parse credential policy stdout");
    assert_eq!(stdout["schema"], "adl.csm.credential_policy_proof.v1");
    assert_eq!(stdout["status"], "passed");
    assert_eq!(stdout["redaction"]["secret_values_retained"], false);
    assert!(stdout["inventory_classes"]
        .as_array()
        .expect("inventory classes")
        .iter()
        .all(|class| class["secret_values_retained"] == false));
    for expected in [
        "missing_credential",
        "expired_credential",
        "denied_break_glass",
        "stale_binding",
    ] {
        assert!(
            stdout["negative_cases"]
                .as_array()
                .expect("negative cases")
                .iter()
                .any(|case| case["name"] == expected && case["secret_material_retained"] == false),
            "missing negative case {expected}: {stdout}"
        );
    }

    let summary_text = fs::read_to_string(proof_dir.join("credential_policy_summary.json"))
        .expect("read credential policy summary");
    let events_text = fs::read_to_string(proof_dir.join("credential_lifecycle_events.jsonl"))
        .expect("read credential lifecycle events");
    let observability = fs::read_to_string(&observability_log).expect("read observability log");
    for text in [&summary_text, &events_text, &observability] {
        assert!(
            !text.contains("operator-alice"),
            "operator identity leaked: {text}"
        );
        assert!(
            !text.contains("PRIVATE KEY"),
            "secret marker leaked: {text}"
        );
        assert!(!text.contains("token="), "token marker leaked: {text}");
        assert!(!text.contains("/Users/"), "host path leaked: {text}");
    }
    assert!(events_text.contains("credential_access_denied"));
    assert!(events_text.contains("break_glass_denied"));
    assert!(events_text.contains("break_glass_revoked"));
    assert!(observability.contains("stage=break_glass_denied"));
    assert!(observability.contains("stage=credential_access_denied"));
    assert!(observability.contains("credential_material=not_retained"));
}

#[test]
fn csm_daemon_exports_otlp_http_json_to_loopback_collector() {
    let root = unique_test_temp_dir("csm-daemon-otlp");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-otlp-agent
display_name: Daemon OTLP Agent
state_root: state
workflow:
  kind: demo_adapter
  name: otlp_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/daemon-otlp-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let (endpoint, captured, shutdown_collector, collector) = spawn_loopback_otlp_collector();
    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--test-supervisor-failure-after-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
            ("ADL_OTEL_LOG", otel_log.to_str().expect("utf8 otel path")),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("utf8 otel status path"),
            ),
            ("ADL_OTEL_EXPORTER_OTLP_ENDPOINT", endpoint.as_str()),
            ("ADL_OTEL_EXPORTER_TIMEOUT_MS", "2000"),
        ],
    );
    let daemon_success = out.status.success();
    let daemon_stderr = String::from_utf8_lossy(&out.stderr).to_string();
    shutdown_collector
        .send(())
        .expect("signal otlp collector shutdown");
    collector.join().expect("collector joined");
    assert!(
        daemon_success,
        "expected daemon OTLP success, stderr:\n{}",
        daemon_stderr
    );
    let exported = captured.try_iter().collect::<Vec<_>>();
    let otel_text = fs::read_to_string(&otel_log).expect("read otel log");
    let mut span_names = std::collections::BTreeSet::new();
    let mut service_names = std::collections::BTreeSet::new();
    let mut trace_id_lengths = std::collections::BTreeSet::new();
    for body in &exported {
        let payload: serde_json::Value = serde_json::from_str(body).expect("parse otlp payload");
        for resource_span in payload["resourceSpans"].as_array().expect("resource spans") {
            if let Some(service_name) =
                otlp_attr_string(&resource_span["resource"]["attributes"], "service.name")
            {
                service_names.insert(service_name.to_string());
            }
            for scope_span in resource_span["scopeSpans"].as_array().expect("scope spans") {
                for span in scope_span["spans"].as_array().expect("spans") {
                    span_names.insert(span["name"].as_str().expect("span name").to_string());
                    trace_id_lengths.insert(span["traceId"].as_str().expect("trace id").len());
                    assert_eq!(span["kind"], 1);
                    assert!(span["startTimeUnixNano"].as_str().is_some());
                    assert!(span["endTimeUnixNano"].as_str().is_some());
                    assert_eq!(span["spanId"].as_str().expect("span id").len(), 16);
                }
            }
        }
    }
    assert!(service_names.contains("csm-runtime-daemon"));
    assert!(
        span_names.contains("csm.daemon_started") || otel_text.contains("csm.daemon_started"),
        "daemon_started span missing from exported payloads and retained otel log"
    );
    assert!(
        span_names.contains("csm.checkpoint_write") || otel_text.contains("csm.checkpoint_write"),
        "checkpoint_write span missing from exported payloads and retained otel log"
    );
    assert!(trace_id_lengths.contains(&32));
    let exported_text = exported.join("\n");
    assert!(!exported_text.contains("adl.otlp_http_json.export.v1"));
    assert!(!exported_text.contains(root.to_str().expect("root utf8")));

    let status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&otel_status).expect("read otel status"))
            .expect("parse otel status");
    assert_eq!(status["schema"], "adl.otel.monitor_status.v1");
    assert_eq!(status["exporter"]["schema"], "adl.otel.exporter_status.v1");
    assert_eq!(status["exporter"]["protocol"], "otlp_http_json");
    let exporter_status = status["exporter"]["status"].as_str().unwrap_or_default();
    assert!(
        exporter_status == "success" || (exporter_status == "failed" && !exported.is_empty()),
        "unexpected exporter status after captured loopback payloads: {status}"
    );
    assert_eq!(status["exporter"]["endpoint"], "<configured>");
}

#[test]
fn csm_daemon_executes_adl_workflow_dag_with_aee_runtime_trace() {
    let root = unique_test_temp_dir("csm-adl-workflow");
    let spec = root.join("agent.yaml");
    let workflow = fixture_path("examples/v0-3-scheduler-max-concurrency.adl.yaml");
    fs::write(
        &spec,
        format!(
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: csm-dag-agent
display_name: CSM DAG Agent
state_root: state
workflow:
  kind: adl_workflow
  name: scheduler_max_concurrency
  path: {}
  run_args: {{}}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/csm-dag-agent
  write_policy: append_only
"#,
            workflow.display()
        ),
    )
    .expect("write agent spec");

    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let spec_str = spec.to_str().expect("utf8 path");
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--test-supervisor-failure-after-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
            ("ADL_OTEL_LOG", otel_log.to_str().expect("utf8 otel path")),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("utf8 otel status path"),
            ),
            ("ADL_OLLAMA_BIN", mock.to_str().expect("utf8 mock path")),
        ],
    );
    assert!(
        out.status.success(),
        "expected csm DAG runtime success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let cycle_root = root.join("state/cycles/cycle-000001");
    let run_status_path = cycle_root.join("csm_adl_run_status.json");
    assert!(run_status_path.exists());
    assert!(cycle_root.join("adl_runtime").exists());
    let run_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_status_path).expect("read run status"))
            .expect("parse run status");
    assert_eq!(run_status["schema"], "adl.csm.adl_workflow_run_status.v1");
    assert_eq!(run_status["runtime_owner"], "csm");
    assert_eq!(run_status["adl_role"], "tooling_control_plane");
    assert_eq!(run_status["status"], "success");
    assert_eq!(run_status["step_count"], 4);
    assert_eq!(run_status["scheduler_policy"]["max_concurrency"], 2);
    assert_eq!(run_status["scheduler_policy"]["source"], "run_default");
    assert_eq!(run_status["records"][0]["step_id"], "fork.a");
    assert_eq!(run_status["records"][0]["status"], "success");
    assert!(run_status["trace_events"]
        .as_array()
        .expect("trace events array")
        .iter()
        .any(|event| event
            .as_str()
            .expect("trace event")
            .contains("SchedulerPolicy max_concurrency=2 source=run_default")));
    assert!(
        run_status["trace_events"]
            .as_array()
            .expect("trace events array")
            .iter()
            .any(|event| event
                .as_str()
                .expect("trace event")
                .contains("RuntimeResilienceDecision")),
        "expected retained AEE/runtime resilience trace: {run_status}"
    );
    assert_eq!(
        run_status["aee_resilience_trace"],
        "retained_in_trace_events"
    );

    let run_ref: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(cycle_root.join("run_ref.json")).unwrap())
            .expect("parse run ref");
    assert_eq!(run_ref["run_status_ref"], "csm_adl_run_status.json");
    assert!(run_ref["execution_note"]
        .as_str()
        .expect("execution note")
        .contains("CSM executed the configured ADL DAG"));

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(
        daemon_status["runtime_capabilities"]["chronosense"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["aee"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["scheduler_watcher"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["resilience_middleware"]["status"],
        "integrated"
    );
}

#[test]
fn csm_owns_daemon_and_adl_agent_daemon_is_removed() {
    let help = run_csm(&["--help"]);
    assert!(
        help.status.success(),
        "expected csm help success, stderr:\n{}",
        String::from_utf8_lossy(&help.stderr)
    );
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("csm daemon --spec"));
    assert!(help_stdout.contains("dedicated runtime owner binary"));

    let removed_agent = run_adl(&["agent", "daemon", "--help"]);
    assert!(
        !removed_agent.status.success(),
        "expected adl agent daemon removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_agent.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_agent.stderr);
    assert!(
        stderr.contains("unknown agent subcommand 'daemon'"),
        "stderr:\n{stderr}"
    );

    let removed_adl_csm = run_adl(&["csm", "daemon", "--help"]);
    assert!(
        !removed_adl_csm.status.success(),
        "expected adl csm daemon removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_adl_csm.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_adl_csm.stderr);
    assert!(
        stderr.contains("csm daemon is owned by the standalone csm runtime binary"),
        "stderr:\n{stderr}"
    );

    let removed_adl_csm_service = run_adl(&["csm", "service", "install", "--help"]);
    assert!(
        !removed_adl_csm_service.status.success(),
        "expected adl csm service removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_adl_csm_service.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_adl_csm_service.stderr);
    assert!(
        stderr.contains("csm service is owned by the standalone csm runtime binary"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csmctl_is_modular_runtime_control_plane_not_runtime_loop_owner() {
    let help = run_csmctl(&["--help"]);
    assert!(
        help.status.success(),
        "expected csmctl help success, stderr:\n{}",
        String::from_utf8_lossy(&help.stderr)
    );
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("csmctl runtime service"));
    assert!(help_stdout.contains("csmctl diagnostics process status"));
    assert!(help_stdout.contains("csmctl cloud aws-signal"));
    assert!(help_stdout.contains("csm is the runtime owner"));
    assert!(help_stdout.contains("adl remains ADL language"));
    assert!(!help_stdout.contains("adl pr run"));

    let service_help = run_csmctl(&["runtime", "service", "--help"]);
    assert!(
        service_help.status.success(),
        "expected csmctl service help success, stderr:\n{}",
        String::from_utf8_lossy(&service_help.stderr)
    );
    assert!(String::from_utf8_lossy(&service_help.stdout).contains("csm service install"));

    let daemon = run_csmctl(&["runtime", "daemon", "--help"]);
    assert!(
        !daemon.status.success(),
        "expected csmctl daemon execution rejection, stdout:\n{}",
        String::from_utf8_lossy(&daemon.stdout)
    );
    let stderr = String::from_utf8_lossy(&daemon.stderr);
    assert!(
        stderr.contains("csmctl does not execute the runtime daemon loop"),
        "stderr:\n{stderr}"
    );

    let status = run_csmctl(&["status", "--pid", &std::process::id().to_string(), "--json"]);
    assert!(
        status.status.success(),
        "expected csmctl status success, stderr:\n{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let status_json: serde_json::Value =
        serde_json::from_slice(&status.stdout).expect("parse csmctl status json");
    assert_eq!(status_json["schema"], "adl.process_status.v1");
    assert_eq!(status_json["check"], "pid");
    assert_eq!(status_json["broad_process_scan"], false);
}

#[test]
fn csm_service_install_writes_launchd_envelope_without_adl_runtime_owner() {
    let root = unique_test_temp_dir("csm-service-install");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-install-agent
display_name: Service Install Agent
state_root: runtime-state
workflow:
  kind: demo_adapter
  name: service_install_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-install-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let out = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "launchd",
        "--label",
        "com.agentlogic.csm.test-install",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--otlp-endpoint",
        "http://127.0.0.1:4318/v1/traces",
        "--otlp-timeout-ms",
        "750",
        "--json",
    ]);
    assert!(
        out.status.success(),
        "expected service install success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_manifest.json")).expect("manifest"),
    )
    .expect("parse manifest");
    assert_eq!(manifest["schema"], "adl.csm.service_manifest.v1");
    assert_eq!(manifest["runtime_owner"], "csm");
    assert_eq!(manifest["restart_policy"], "always");
    assert_eq!(manifest["service_mode"], "permanent");
    assert_eq!(manifest["manager"], "launchd");
    assert_eq!(manifest["checkpoint_interval_secs"], 1);
    assert_eq!(
        manifest["network_registry"]["listeners"][0]["default_bind"],
        "127.0.0.1:19997"
    );
    assert_eq!(
        manifest["connection_pooling_plan"]["schema"],
        "adl.csm.pooling_plan.v1"
    );
    assert_eq!(
        manifest["connection_pool_status"]["schema"],
        "adl.csm.connection_pool_status.v1"
    );
    assert_eq!(
        manifest["connection_pool_status"]["pool_backend"],
        "deadpool::unmanaged"
    );
    assert_eq!(manifest["otlp_endpoint"], "http://127.0.0.1:4318/v1/traces");
    assert_eq!(manifest["otlp_timeout_ms"], 750);
    assert!(manifest["daemon_status"]
        .as_str()
        .expect("daemon status path")
        .ends_with("runtime-state/daemon_status.json"));
    assert!(manifest["continuity_checkpoint"]
        .as_str()
        .expect("checkpoint path")
        .ends_with("runtime-state/continuity_checkpoint.json"));
    assert!(manifest["unsupported_permanence_claims"]
        .as_array()
        .expect("nonclaims")
        .iter()
        .any(|value| value == "host_reboot_survival_not_proven"));

    let plist = fs::read_to_string(service_root.join("csm.launchd.plist")).expect("plist");
    assert!(plist.contains("<key>KeepAlive</key>"));
    assert!(plist.contains("<string>daemon</string>"));
    assert!(plist.contains("<string>--api-bind</string>"));
    assert!(plist.contains("<string>127.0.0.1:19997</string>"));
    assert!(plist.contains("ADL_OTEL_STATUS"));
    assert!(plist.contains("ADL_OTEL_EXPORTER_OTLP_ENDPOINT"));
    assert!(plist.contains("http://127.0.0.1:4318/v1/traces"));
    assert!(plist.contains("ADL_OTEL_EXPORTER_TIMEOUT_MS"));
    assert!(!plist.contains("adl agent daemon"));

    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_status.json")).expect("service status"),
    )
    .expect("parse service status");
    assert_eq!(status["schema"], "adl.csm.service_status.v1");
    assert_eq!(status["restart_policy"], "always");
    assert_eq!(status["service_mode"], "permanent");
    assert_eq!(status["service_state"], "installed");
    assert_eq!(status["broad_process_scan"], false);
    assert_eq!(status["uses_ps"], false);
    assert_eq!(
        status["network_registry"]["active_listener"]["listener_role"],
        "main_runtime_api"
    );
    assert_eq!(
        status["network_registry"]["active_listener"]["bind_addr"],
        "127.0.0.1:19997"
    );
    assert_eq!(
        status["connection_pooling_plan"]["roles"][0]["decision"],
        "use_deadpool_for_governed_client_slot_capacity"
    );
    assert_eq!(status["connection_pooling_plan"]["pool_crate"], "deadpool");
    assert_eq!(
        status["connection_pooling_plan"]["pool_backend"],
        "deadpool::unmanaged"
    );
    assert_eq!(
        status["connection_pool_status"]["schema"],
        "adl.csm.connection_pool_status.v1"
    );
    assert_eq!(
        status["connection_pool_status"]["roles"][0]["pool_crate"],
        "deadpool"
    );
    assert_eq!(status["otlp_exporter_configured"], true);
    assert_eq!(status["otlp_endpoint_ref"], "<configured>");
    assert_eq!(
        status["startup_classification"],
        "startup_missing_pid_metadata"
    );
    assert_eq!(status["first_daemon_record_observed"], false);
    assert_eq!(status["continuity_checkpoint_observed"], false);
    assert_eq!(status["cycle_ledger_observed"], false);
    assert!(status["startup_ledger_ref"]
        .as_str()
        .expect("startup ledger ref")
        .ends_with("logs/startup_ledger.jsonl"));
}

#[test]
fn csm_service_status_reports_invalid_manifest_api_bind_truthfully() {
    let root = unique_test_temp_dir("csm-service-invalid-api-bind");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-invalid-bind-agent
display_name: Service Invalid Bind Agent
state_root: runtime-state
workflow:
  kind: demo_adapter
  name: service_invalid_bind_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-invalid-bind-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "launchd",
        "--label",
        "com.agentlogic.csm.invalid-bind",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );

    let manifest_path = service_root.join("service_manifest.json");
    let mut manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&manifest_path).expect("manifest"))
            .expect("parse manifest");
    manifest["api_bind"] = serde_json::Value::String("127.0.0.1:20000".to_string());
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).expect("serialize manifest"),
    )
    .expect("write mutated manifest");

    let status = run_csm(&[
        "service",
        "status",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        status.status.success(),
        "status stderr:\n{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let service_status: serde_json::Value =
        serde_json::from_slice(&status.stdout).expect("parse service status stdout");
    let active_listener = &service_status["network_registry"]["active_listener"];
    assert_eq!(active_listener["status"], "invalid");
    assert_eq!(active_listener["bind_addr"], "127.0.0.1:20000");
    assert!(active_listener["error"]
        .as_str()
        .expect("invalid bind error")
        .contains("outside reserved local CSM port range"));
}

#[test]
fn csm_service_install_classifies_local_and_no_sleep_modes_truthfully() {
    let root = unique_test_temp_dir("csm-service-mode-truth");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-mode-agent
display_name: Service Mode Agent
state_root: runtime-state
workflow:
  kind: demo_adapter
  name: service_mode_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-mode-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let csm_bin = resolve_csm_exe();
    let (api_probe, api_bind) = reserve_csm_test_port("service mode manifest");
    drop(api_probe);

    let local_root = root.join("local-service");
    let local = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        local_root.to_str().expect("utf8 local service root"),
        "--manager",
        "local",
        "--api-bind",
        &api_bind,
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--json",
    ]);
    assert!(
        local.status.success(),
        "expected local service install success, stderr:\n{}",
        String::from_utf8_lossy(&local.stderr)
    );
    let local_manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(local_root.join("service_manifest.json")).expect("local manifest"),
    )
    .expect("parse local manifest");
    assert_eq!(local_manifest["restart_policy"], "always");
    assert_eq!(local_manifest["service_mode"], "rust_supervisor");
    let mut legacy_local_manifest = local_manifest.clone();
    legacy_local_manifest
        .as_object_mut()
        .expect("local manifest object")
        .remove("restart_policy");
    legacy_local_manifest
        .as_object_mut()
        .expect("local manifest object")
        .remove("service_mode");
    fs::write(
        local_root.join("service_manifest.json"),
        serde_json::to_string_pretty(&legacy_local_manifest).expect("serialize legacy manifest"),
    )
    .expect("write legacy local manifest");
    let local_status = run_csm(&[
        "service",
        "status",
        "--service-root",
        local_root.to_str().expect("utf8 local service root"),
        "--json",
    ]);
    assert!(
        local_status.status.success(),
        "expected local service status success, stderr:\n{}",
        String::from_utf8_lossy(&local_status.stderr)
    );
    let legacy_status: serde_json::Value =
        serde_json::from_slice(&local_status.stdout).expect("parse local status stdout");
    assert_eq!(legacy_status["restart_policy"], "always");
    assert_eq!(legacy_status["service_mode"], "rust_supervisor");

    let bounded_root = root.join("bounded-service");
    let bounded = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        bounded_root.to_str().expect("utf8 bounded service root"),
        "--manager",
        "launchd",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--no-sleep",
        "--json",
    ]);
    assert!(
        bounded.status.success(),
        "expected bounded service install success, stderr:\n{}",
        String::from_utf8_lossy(&bounded.stderr)
    );
    let bounded_manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(bounded_root.join("service_manifest.json")).expect("bounded manifest"),
    )
    .expect("parse bounded manifest");
    assert_eq!(bounded_manifest["restart_policy"], "bounded_test_only");
    assert_eq!(bounded_manifest["service_mode"], "bounded_test_only");
    let bounded_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(bounded_root.join("service_status.json")).expect("bounded status"),
    )
    .expect("parse bounded status");
    assert_eq!(bounded_status["restart_policy"], "bounded_test_only");
    assert_eq!(bounded_status["service_mode"], "bounded_test_only");
}

#[test]
fn csm_service_install_rejects_secret_bearing_otlp_endpoint() {
    let root = unique_test_temp_dir("csm-service-secret-otlp");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-secret-otlp-agent
display_name: Service Secret OTLP Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_secret_otlp_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-secret-otlp-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let out = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        root.join("service").to_str().expect("utf8 service root"),
        "--otlp-endpoint",
        "https://collector.example.invalid/v1/traces?token=secret",
        "--json",
    ]);
    assert!(
        !out.status.success(),
        "expected secret-bearing endpoint rejection, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--otlp-endpoint must not contain credentials"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csm_service_install_rejects_secret_bearing_otlp_endpoint_from_env() {
    let root = unique_test_temp_dir("csm-service-secret-otlp-env");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-secret-otlp-env-agent
display_name: Service Secret OTLP Env Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_secret_otlp_env_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-secret-otlp-env-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let out = run_csm_with_env(
        &[
            "service",
            "install",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[(
            "ADL_OTEL_EXPORTER_OTLP_ENDPOINT",
            "https://user:secret@collector.example.invalid/v1/traces",
        )],
    );
    assert!(
        !out.status.success(),
        "expected env endpoint rejection, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--otlp-endpoint must not contain credentials"),
        "stderr:\n{stderr}"
    );
    assert!(!service_root.join("service_manifest.json").exists());
    assert!(!service_root.join("csm.launchd.plist").exists());
}

#[test]
fn csm_service_local_start_stop_retains_status_checkpoint_and_observability() {
    let root = unique_test_temp_dir("csm-service-local");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-local-agent
display_name: Service Local Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_local_probe
  run_args: {}
heartbeat:
  interval_secs: 10
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-local-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let (api_probe, api_bind) = reserve_csm_test_port("service API smoke");
    drop(api_probe);
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--api-bind",
        &api_bind,
        "--label",
        "com.agentlogic.csm.test-local",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );

    let start = run_csm_with_env(
        &[
            "service",
            "start",
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[
            (
                "ADL_CSM_SERVICE_STARTUP_ATTEMPTS",
                CSM_COVERAGE_STARTUP_ATTEMPTS,
            ),
            CSM_DISK_READY_ENV[0],
            CSM_DISK_READY_ENV[1],
        ],
    );
    assert!(
        start.status.success(),
        "start stderr:\n{}\nchild stdout:\n{}\nchild stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
        read_text_or_missing(&service_root.join("logs/csm.stdout.log")),
        read_text_or_missing(&service_root.join("logs/csm.stderr.log"))
    );

    let mut service_status = None;
    let mut last_status_stderr = String::new();
    for _ in 0..40 {
        let status = run_csm(&[
            "service",
            "status",
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ]);
        if status.status.success() {
            let parsed: serde_json::Value =
                serde_json::from_slice(&status.stdout).expect("parse service status stdout");
            if parsed["startup_classification"] == "startup_runtime_ready" {
                service_status = Some(parsed);
                break;
            }
            service_status = Some(parsed);
        } else {
            last_status_stderr = String::from_utf8_lossy(&status.stderr).to_string();
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    let service_status = service_status.unwrap_or_else(|| {
        panic!("status never became available, last stderr:\n{last_status_stderr}")
    });
    assert_eq!(
        service_status["startup_classification"],
        "startup_runtime_ready"
    );
    assert_eq!(service_status["runtime_owner"], "csm");
    assert_eq!(service_status["service_state"], "observed");
    assert_eq!(service_status["broad_process_scan"], false);
    assert_eq!(service_status["uses_ps"], false);
    assert_eq!(
        service_status["network_registry"]["active_listener"]["listener_role"],
        "main_runtime_api"
    );
    assert_eq!(
        service_status["network_registry"]["active_listener"]["bind_addr"],
        api_bind
    );
    assert_eq!(
        service_status["network_registry"]["registry"]["listeners"][0]["default_bind"],
        "127.0.0.1:19997"
    );
    assert_eq!(
        service_status["startup_classification"],
        "startup_runtime_ready"
    );
    assert_eq!(service_status["first_daemon_record_observed"], true);
    assert_eq!(service_status["continuity_checkpoint_observed"], true);
    assert_eq!(service_status["cycle_ledger_observed"], true);
    assert!(root.join("state/daemon_status.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(service_root.join("logs/observability.log").exists());
    assert!(service_root.join("logs/otel_status.json").exists());
    let startup_ledger =
        fs::read_to_string(service_root.join("logs/startup_ledger.jsonl")).expect("startup ledger");
    assert!(startup_ledger.contains("\"event\":\"start_requested\""));
    assert!(startup_ledger.contains("\"event\":\"local_spawn\""));
    assert!(startup_ledger.contains("\"event\":\"rust_supervisor_started\""));
    assert!(startup_ledger.contains("\"event\":\"rust_supervisor_child_spawn\""));
    assert!(startup_ledger.contains("\"event\":\"startup_probe\""));
    assert!(startup_ledger.contains("startup_runtime_ready"));
    let supervisor_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("logs/rust_supervisor_status.json"))
            .expect("supervisor status"),
    )
    .expect("parse supervisor status");
    assert_eq!(
        supervisor_status["schema"],
        "adl.csm.rust_supervisor_status.v1"
    );
    assert_eq!(supervisor_status["restart_policy"], "always");
    assert_eq!(
        supervisor_status["runtime_api"]["status"],
        "embedded_in_daemon"
    );
    assert_eq!(
        supervisor_status["runtime_api"]["pid_model"],
        "same_process_as_csm_daemon_child"
    );
    assert_eq!(
        supervisor_status["stop_policy"],
        "explicit_stop_intent_only"
    );
    assert_eq!(supervisor_status["max_cycles"], "not_applicable");
    assert_eq!(supervisor_status["request_budget"], "not_applicable");
    let api_bind = supervisor_status["runtime_api"]["bind"]
        .as_str()
        .expect("runtime API bind")
        .to_string();
    let ready = http_get_json_authenticated(&api_bind, &root.join("state"), "/ready");
    assert_eq!(ready["schema"], "adl.csm.runtime_api.ready.v1");
    assert_eq!(ready["runtime_owner"], "csm");
    assert_eq!(ready["agent_instance_id"], "service-local-agent");
    if ready["ready"] != "ready" {
        let blockers = ready["blocking_reasons"]
            .as_array()
            .expect("not-ready response includes blockers");
        assert!(
            blockers.iter().all(|blocker| {
                blocker.as_str().is_some_and(|value| {
                    value.starts_with("chronosense_time_sync_")
                        || value == "curiosity_engine_not_ready"
                        || value == "reasoning_runtime_missing"
                        || value == "constructability_gate_blocked"
                        || value == "cav_security_validation_fail_closed"
                })
            }),
            "unexpected runtime API readiness blockers: {blockers:?}"
        );
    }
    let observability =
        fs::read_to_string(service_root.join("logs/observability.log")).expect("observability log");
    assert!(observability.contains("stage=start_requested"));
    assert!(observability.contains("stage=startup_probe"));
    let otel = fs::read_to_string(service_root.join("logs/otel.jsonl")).expect("otel log");
    assert!(otel.contains("\"name\":\"csm.start_requested\""));
    assert!(otel.contains("\"name\":\"csm.startup_probe\""));

    let second_start = run_csm_with_env(
        &[
            "service",
            "start",
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[
            (
                "ADL_CSM_SERVICE_STARTUP_ATTEMPTS",
                CSM_COVERAGE_STARTUP_ATTEMPTS,
            ),
            CSM_DISK_READY_ENV[0],
            CSM_DISK_READY_ENV[1],
        ],
    );
    assert!(
        second_start.status.success(),
        "second start stderr:\n{}",
        String::from_utf8_lossy(&second_start.stderr)
    );
    let second_status: serde_json::Value =
        serde_json::from_slice(&second_start.stdout).expect("parse second start stdout");
    assert_eq!(second_status["service_state"], "running");
    assert_eq!(
        second_status["startup_classification"],
        "startup_runtime_ready"
    );
    let startup_ledger =
        fs::read_to_string(service_root.join("logs/startup_ledger.jsonl")).expect("startup ledger");
    assert!(startup_ledger.contains("\"event\":\"local_already_running\""));

    let stop = run_csm(&[
        "service",
        "stop",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        stop.status.success(),
        "stop stderr:\n{}",
        String::from_utf8_lossy(&stop.stderr)
    );
    let service_status: serde_json::Value =
        serde_json::from_slice(&stop.stdout).expect("parse stop status stdout");
    assert_eq!(service_status["service_state"], "stopped_or_requested");
    let restart = run_csm_with_env(
        &[
            "service",
            "start",
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[
            (
                "ADL_CSM_SERVICE_STARTUP_ATTEMPTS",
                CSM_COVERAGE_STARTUP_ATTEMPTS,
            ),
            CSM_DISK_READY_ENV[0],
            CSM_DISK_READY_ENV[1],
        ],
    );
    assert!(
        restart.status.success(),
        "restart after stop stderr:\n{}",
        String::from_utf8_lossy(&restart.stderr)
    );
    assert!(
        !root.join("state/stop.json").exists(),
        "service start must clear durable stop intent before creating a new runtime lifetime"
    );
    let restart_status: serde_json::Value =
        serde_json::from_slice(&restart.stdout).expect("parse restart stdout");
    assert_eq!(restart_status["service_state"], "running");
    assert_eq!(
        restart_status["startup_classification"],
        "startup_runtime_ready"
    );
    let restart_ledger =
        fs::read_to_string(service_root.join("logs/startup_ledger.jsonl")).expect("startup ledger");
    assert!(restart_ledger.contains("\"event\":\"service_start_cleared_stop_intent\""));
    let stop = run_csm(&[
        "service",
        "stop",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        stop.status.success(),
        "second stop stderr:\n{}",
        String::from_utf8_lossy(&stop.stderr)
    );
    let stale_pid = u32::MAX;
    fs::write(service_root.join("csm-service.pid"), stale_pid.to_string())
        .expect("write stale pid metadata");
    fs::write(
        root.join("state/daemon_status.json"),
        format!(
            r#"{{
  "schema": "adl.long_lived_agent_daemon_status.v1",
  "agent_instance_id": "service-local-agent",
  "state": "running",
  "supervisor_pid": {stale_pid},
  "restart_count": 0,
  "bounded_test_restart_limit": 1,
  "checkpoint_interval_secs": 1,
  "last_event": "heartbeat",
  "updated_at": "{}"
}}
"#,
            chrono::Utc::now().to_rfc3339()
        ),
    )
    .expect("write stale daemon status");
    let stale_status = run_csm(&[
        "service",
        "status",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        stale_status.status.success(),
        "stale status stderr:\n{}",
        String::from_utf8_lossy(&stale_status.stderr)
    );
    let stale_status: serde_json::Value =
        serde_json::from_slice(&stale_status.stdout).expect("parse stale status stdout");
    assert_ne!(
        stale_status["startup_classification"],
        "startup_runtime_ready"
    );
    assert_eq!(stale_status["pid_liveness"], "stale_pid");
    let agent_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/status.json")).unwrap())
            .expect("parse agent status");
    assert_eq!(agent_status["state"], "stopped");
    assert_eq!(
        agent_status["last_error"]["class"],
        "operator_stop_requested"
    );
}

#[test]
fn csm_governed_stop_records_checkpoint_safe_fail_lifelog_and_notices() {
    let root = unique_test_temp_dir("csm-governed-stop");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: governed-stop-agent
display_name: Governed Stop Agent
state_root: state
workflow:
  kind: demo_adapter
  name: governed_stop_probe
  run_args: {}
heartbeat:
  interval_secs: 10
  max_cycles: 3
  stale_lease_after_secs: 60
checkpoint:
  interval_secs: 1
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/governed-stop-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let (api_probe, api_bind) = reserve_csm_test_port("governed API smoke");
    drop(api_probe);
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--api-bind",
        &api_bind,
        "--label",
        "com.agentlogic.csm.test-governed-stop",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );
    let start = run_csm_with_env(
        &[
            "service",
            "start",
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[
            (
                "ADL_CSM_SERVICE_STARTUP_ATTEMPTS",
                CSM_COVERAGE_STARTUP_ATTEMPTS,
            ),
            CSM_DISK_READY_ENV[0],
            CSM_DISK_READY_ENV[1],
        ],
    );
    assert!(
        start.status.success(),
        "start stderr:\n{}\nchild stdout:\n{}\nchild stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
        read_text_or_missing(&service_root.join("logs/csm.stdout.log")),
        read_text_or_missing(&service_root.join("logs/csm.stderr.log"))
    );
    std::thread::sleep(std::time::Duration::from_millis(1500));

    let stop = run_csm_with_env(
        &[
            "governed-stop",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--reason",
            "operator requested recoverable polis stop",
            "--operator",
            "codex-test-operator",
            "--authorization",
            "test-approval-ticket-5005",
            "--intent",
            "emergency_polis_stop",
            "--requested-at",
            "2026-07-07T16:00:00Z",
            "--json",
        ],
        &[
            ("ADL_AWS_SIGNAL_MODE", "mock"),
            ("ADL_AWS_SIGNAL_APPROVED", "1"),
            ("ADL_AWS_HEARTBEAT_TARGET", "cloudwatch_logs"),
            ("ADL_AWS_REGION", "us-west-2"),
            ("ADL_AWS_PROFILE", "agent-logic-admin"),
            (
                "ADL_AWS_SNS_TOPIC_ARN",
                "arn:aws:sns:us-west-2:000000000000:mock",
            ),
            ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "mock"),
            ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "eventbridge"),
            ("ADL_CSM_NOTICE_EVENT_BUS", "adl-csm-notice-bus-5005"),
        ],
    );
    assert!(
        stop.status.success(),
        "governed-stop stderr:\n{}",
        String::from_utf8_lossy(&stop.stderr)
    );
    let result: serde_json::Value =
        serde_json::from_slice(&stop.stdout).expect("parse governed stop stdout");
    assert_eq!(result["schema"], "adl.csm.governed_stop.result.v1");
    assert_eq!(result["classification"], "governed_emergency_stop");
    assert_eq!(
        result["agent_recoverability"]["recoverability_class"],
        "recoverable_checkpointed"
    );
    assert_eq!(result["notice"]["notice_kind"], "governed_emergency_stop");

    let state = root.join("state");
    assert!(state.join("governed_stop.json").exists());
    assert!(state.join("stop.json").exists());
    assert!(state.join("status.json").exists());
    assert!(state.join("daemon_status.json").exists());
    assert!(state.join("continuity_checkpoint.json").exists());
    assert!(state.join("continuity_replay_manifest.json").exists());
    assert!(state.join("safe_fail_bundle.json").exists());
    assert!(state.join("csm_lifecycle_lifelog.db.jsonl").exists());
    assert!(state.join("csm_lifecycle_lifelog.index.json").exists());
    assert!(state.join("csm_governed_notices.jsonl").exists());
    assert!(state.join("csm_governed_notice_latest.json").exists());
    let notice_latest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(state.join("csm_governed_notice_latest.json"))
            .expect("governed notice latest"),
    )
    .expect("parse governed notice latest");
    let attempts = notice_latest["delivery_attempts"]
        .as_array()
        .expect("delivery attempts");
    assert!(
        attempts
            .iter()
            .any(|attempt| attempt["channel"] == "local_notice_ledger"
                && attempt["status"] == "recorded"),
        "local notice ledger attempt missing: {notice_latest}"
    );
    let direct_mock_artifacts = state.join("aws_csm_governed_notice_mock.jsonl").exists()
        && state
            .join("aws_csm_governed_notice_sns_mock.jsonl")
            .exists()
        && state
            .join("csm_governed_notice_control_plane_mock.jsonl")
            .exists();
    let typed_delivery_retained = notice_latest["typed_channel_delivery"]["cursor_advanced"]
        == false
        && matches!(
            notice_latest["typed_channel_delivery"]["status"].as_str(),
            Some("durably_spooled_waiting_for_verified_transport_receipt")
                | Some("observer_command_defers_to_daemon_channel_owner")
                | Some("blocked_before_sequence_reservation")
        );
    assert!(
        direct_mock_artifacts || typed_delivery_retained,
        "governed notice delivery evidence missing: {notice_latest}"
    );

    let governed_stop: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(state.join("governed_stop.json")).expect("governed stop artifact"),
    )
    .expect("parse governed stop");
    assert_eq!(governed_stop["schema"], "adl.csm.governed_stop.v1");
    assert_eq!(
        governed_stop["authorization_policy"]["ordinary_api_requests_can_stop_runtime"],
        false
    );
    assert!(governed_stop["operator_intent"]["authorization_ref"]
        .as_str()
        .expect("authorization ref")
        .starts_with("sha256:"));
    assert_eq!(
        governed_stop["agent_recoverability"]["recoverability_class"],
        "recoverable_checkpointed"
    );
    let lifelog = fs::read_to_string(state.join("csm_lifecycle_lifelog.db.jsonl"))
        .expect("read lifecycle lifelog");
    assert!(lifelog.contains("governed_emergency_stop_requested"));
    assert!(lifelog.contains("governed_emergency_stop_recorded"));
    let operator_events =
        fs::read_to_string(state.join("operator_events.jsonl")).expect("operator events");
    assert!(operator_events.contains("governed_emergency_stop_requested"));
    assert!(operator_events.contains("governed_emergency_stop_recorded"));

    let api_options = adl::csm_runtime_api::CsmRuntimeApiOptions {
        spec_path: spec.clone(),
        bind: "127.0.0.1:19950".to_string(),
        test_max_requests: None,
        idle_timeout_ms: None,
        shutdown_file: None,
        otel_status_path: None,
        otel_log_path: None,
    };
    let status_deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let api_status = loop {
        let status = adl::csm_runtime_api::runtime_api_response(&api_options, "/status")
            .expect("governed API status response");
        if status["daemon_liveness"]["state"] == "governed_stopped" {
            break status;
        }
        assert!(
            std::time::Instant::now() < status_deadline,
            "daemon did not reach governed_stopped state: {}",
            serde_json::to_string(&status).unwrap()
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    };
    let api_ready = adl::csm_runtime_api::runtime_api_response(&api_options, "/ready")
        .expect("governed API ready response");
    assert!(
        api_status["status"] == "healthy" || api_status["status"] == "degraded",
        "unexpected governed API status: {api_status}"
    );
    assert_eq!(api_status["ready"], "not_ready");
    assert_eq!(api_status["daemon_liveness"]["state"], "governed_stopped");
    assert_eq!(api_ready["ready"], "not_ready");
    assert!(api_ready["blocking_reasons"]
        .as_array()
        .expect("ready blockers")
        .contains(&serde_json::json!("daemon_state_governed_stopped")));

    let missing_operator = run_csm(&[
        "governed-stop",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--reason",
        "missing operator must fail",
        "--authorization",
        "ticket",
        "--intent",
        "emergency_polis_stop",
        "--requested-at",
        "2026-07-07T16:00:00Z",
        "--json",
    ]);
    assert!(!missing_operator.status.success());
    assert!(String::from_utf8_lossy(&missing_operator.stderr).contains("--operator"));
}

#[test]
fn csm_service_rust_supervisor_restarts_real_daemon_child() {
    let root = unique_test_temp_dir("csm-service-rust-supervisor");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: rust-supervisor-agent
display_name: Rust Supervisor Agent
state_root: state
workflow:
  kind: demo_adapter
  name: rust_supervisor_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/rust-supervisor-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let (api_probe, api_bind) = reserve_csm_test_port("supervisor API smoke");
    drop(api_probe);
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--api-bind",
        &api_bind,
        "--label",
        "com.agentlogic.csm.test-rust-supervisor",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );
    let start = run_csm_with_env(
        &[
            "service",
            "start",
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[
            (
                "ADL_CSM_SERVICE_STARTUP_ATTEMPTS",
                CSM_COVERAGE_STARTUP_ATTEMPTS,
            ),
            CSM_DISK_READY_ENV[0],
            CSM_DISK_READY_ENV[1],
        ],
    );
    assert!(
        start.status.success(),
        "start stderr:\n{}",
        String::from_utf8_lossy(&start.stderr)
    );
    let restart_deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut observed_restart = false;
    let mut last_supervisor_status = serde_json::Value::Null;
    while std::time::Instant::now() < restart_deadline {
        if let Ok(raw) = fs::read_to_string(service_root.join("logs/rust_supervisor_status.json")) {
            if let Ok(supervisor_status) = serde_json::from_str::<serde_json::Value>(&raw) {
                if supervisor_status["restart_count"].as_u64().unwrap_or(0) >= 1 {
                    observed_restart = true;
                    break;
                }
                last_supervisor_status = supervisor_status;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
    assert!(
        observed_restart,
        "expected Rust supervisor to restart a real csm daemon child after bounded child completion; last supervisor status:\n{}",
        serde_json::to_string_pretty(&last_supervisor_status).unwrap()
    );
    let supervisor_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("logs/rust_supervisor_status.json"))
            .expect("supervisor status"),
    )
    .expect("parse supervisor status");
    let startup_ledger =
        fs::read_to_string(service_root.join("logs/startup_ledger.jsonl")).expect("startup ledger");
    let stop = run_csm(&[
        "service",
        "stop",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        stop.status.success(),
        "stop stderr:\n{}",
        String::from_utf8_lossy(&stop.stderr)
    );
    assert!(
        observed_restart,
        "expected Rust supervisor restart within 30s; status: {}; startup ledger:\n{}",
        serde_json::to_string(&supervisor_status).unwrap(),
        startup_ledger
    );
    assert_eq!(supervisor_status["restart_policy"], "always");
    assert_eq!(supervisor_status["max_cycles"], "not_applicable");
    assert_eq!(supervisor_status["request_budget"], "not_applicable");
    assert!(startup_ledger.contains("\"event\":\"rust_supervisor_child_exit\""));
    assert!(startup_ledger.contains("\"event\":\"rust_supervisor_restart_scheduled\""));
}

#[test]
fn csm_service_local_start_classifies_missing_first_daemon_record() {
    let root = unique_test_temp_dir("csm-service-startup-missing-record");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-startup-missing-record-agent
display_name: Service Startup Missing Record Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_startup_missing_record_probe
  run_args: {}
heartbeat:
  interval_secs: 10
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-startup-missing-record-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let false_bin = if std::path::Path::new("/usr/bin/false").exists() {
        "/usr/bin/false"
    } else {
        "/bin/false"
    };
    let (api_probe, api_bind) = reserve_csm_test_port("startup missing record API");
    drop(api_probe);
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--api-bind",
        &api_bind,
        "--label",
        "com.agentlogic.csm.test-startup-missing-record",
        "--csm-bin",
        false_bin,
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );
    fs::create_dir_all(root.join("state")).expect("state dir");
    fs::write(
        root.join("state/daemon_status.json"),
        format!(
            r#"{{
  "schema": "adl.long_lived_agent_daemon_status.v1",
  "agent_instance_id": "service-startup-missing-record-agent",
  "state": "running",
  "supervisor_pid": {},
  "restart_count": 0,
  "bounded_test_restart_limit": 1,
  "checkpoint_interval_secs": 1,
  "last_event": "child_exit",
  "last_child_exit": "success",
  "updated_at": "2026-07-06T00:00:00Z"
}}
"#,
            std::process::id()
        ),
    )
    .expect("write stale daemon status");

    let start = run_csm_with_env(
        &[
            "service",
            "start",
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[("ADL_CSM_SERVICE_STARTUP_ATTEMPTS", "5")],
    );
    assert!(
        !start.status.success(),
        "expected startup classification failure, stdout:\n{}",
        String::from_utf8_lossy(&start.stdout)
    );
    let stderr = String::from_utf8_lossy(&start.stderr);
    assert!(
        stderr.contains("startup failed before runtime readiness"),
        "stderr:\n{stderr}"
    );
    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_status.json")).expect("service status"),
    )
    .expect("parse service status");
    assert_eq!(status["service_state"], "startup_failed");
    assert!(
        status["startup_classification"] == "startup_stale_before_runtime_ready"
            || status["startup_classification"] == "startup_waiting_for_runtime_ready",
        "status:\n{}",
        serde_json::to_string_pretty(&status).unwrap()
    );
    assert_eq!(status["first_daemon_record_observed"], false);
    assert_eq!(status["runtime_api_observed"], false);
    fs::write(
        service_root.join("csm-service.pid"),
        std::process::id().to_string(),
    )
    .expect("write current pid metadata");
    fs::write(
        root.join("state/daemon_status.json"),
        format!(
            r#"{{
  "schema": "adl.long_lived_agent_daemon_status.v1",
  "agent_instance_id": "service-startup-missing-record-agent",
  "state": "running",
  "supervisor_pid": {},
  "restart_count": 0,
  "bounded_test_restart_limit": 1,
  "checkpoint_interval_secs": 1,
  "last_event": "heartbeat",
  "updated_at": "{}"
}}
"#,
            std::process::id(),
            chrono::Utc::now().to_rfc3339()
        ),
    )
    .expect("write fresh daemon status");
    fs::write(root.join("state/continuity_checkpoint.json"), "{}\n")
        .expect("write recovered checkpoint");
    fs::write(root.join("state/cycle_ledger.jsonl"), "{}\n").expect("write recovered cycle ledger");
    let recovered_status = run_csm(&[
        "service",
        "status",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        recovered_status.status.success(),
        "status stderr:\n{}",
        String::from_utf8_lossy(&recovered_status.stderr)
    );
    let recovered_status: serde_json::Value =
        serde_json::from_slice(&recovered_status.stdout).expect("parse recovered status");
    assert_eq!(recovered_status["pid_liveness"], "live_pid");
    assert_eq!(recovered_status["first_daemon_record_observed"], true);
    assert_ne!(
        recovered_status["startup_classification"],
        "startup_runtime_ready"
    );
    assert_eq!(recovered_status["runtime_api_observed"], false);
    let startup_ledger =
        fs::read_to_string(service_root.join("logs/startup_ledger.jsonl")).expect("startup ledger");
    assert!(startup_ledger.contains("\"event\":\"start_requested\""));
    assert!(startup_ledger.contains("\"event\":\"local_spawn\""));
    assert!(startup_ledger.contains("\"event\":\"startup_probe\""));
    let observability =
        fs::read_to_string(service_root.join("logs/observability.log")).expect("observability log");
    assert!(observability.contains("stage=start_requested"));
    assert!(observability.contains("stage=startup_probe"));
    let otel = fs::read_to_string(service_root.join("logs/otel.jsonl")).expect("otel log");
    assert!(otel.contains("\"name\":\"csm.start_requested\""));
    assert!(otel.contains("\"name\":\"csm.startup_probe\""));
}

#[test]
fn csm_service_start_fails_readiness_when_embedded_api_bind_is_unavailable() {
    let root = unique_test_temp_dir("csm-service-api-bind-unavailable");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-api-bind-unavailable-agent
display_name: Service API Bind Unavailable Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_api_bind_unavailable_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
checkpoint:
  interval_secs: 1
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-api-bind-unavailable-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let (occupied, api_bind) = reserve_csm_test_port("occupied API port");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--api-bind",
        &api_bind,
        "--label",
        "com.agentlogic.csm.test-api-bind-unavailable",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );
    let start = run_csm_with_env(
        &[
            "service",
            "start",
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[
            ("ADL_CSM_SERVICE_STARTUP_ATTEMPTS", "20"),
            CSM_DISK_READY_ENV[0],
            CSM_DISK_READY_ENV[1],
        ],
    );
    assert!(
        !start.status.success(),
        "expected API bind readiness failure, stdout:\n{}",
        String::from_utf8_lossy(&start.stdout)
    );
    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_status.json")).expect("service status"),
    )
    .expect("parse service status");
    assert_eq!(status["service_state"], "startup_failed");
    assert_ne!(status["startup_classification"], "startup_runtime_ready");
    assert_eq!(status["runtime_api_observed"], false);
    let startup_ledger =
        fs::read_to_string(service_root.join("logs/startup_ledger.jsonl")).expect("startup ledger");
    assert!(startup_ledger.contains("\"runtime_api_observed\":false"));
    assert!(startup_ledger.contains("startup_daemon_live_waiting_for_runtime_api"));

    drop(occupied);
    let stop = run_csm(&[
        "service",
        "stop",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        stop.status.success(),
        "stop stderr:\n{}",
        String::from_utf8_lossy(&stop.stderr)
    );
}

#[test]
fn csm_service_launchd_bootstrap_failure_retains_startup_observability() {
    let root = unique_test_temp_dir("csm-service-launchd-stale-record");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-launchd-stale-record-agent
display_name: Service Launchd Stale Record Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_launchd_stale_record_probe
  run_args: {}
heartbeat:
  interval_secs: 10
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-launchd-stale-record-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "launchd",
        "--label",
        "com.agentlogic.csm.test-launchd-stale-record",
        "--csm-bin",
        "/usr/bin/false",
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );
    fs::create_dir_all(root.join("state")).expect("state dir");
    fs::write(
        root.join("state/daemon_status.json"),
        format!(
            r#"{{
  "schema": "adl.long_lived_agent_daemon_status.v1",
  "agent_instance_id": "service-launchd-stale-record-agent",
  "state": "running",
  "supervisor_pid": {},
  "restart_count": 0,
  "bounded_test_restart_limit": 1,
  "checkpoint_interval_secs": 1,
  "last_event": "child_exit",
  "last_child_exit": "success",
  "updated_at": "2026-07-06T00:00:00Z"
}}
"#,
            std::process::id()
        ),
    )
    .expect("write stale daemon status");

    let start = run_csm(&[
        "service",
        "start",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        !start.status.success(),
        "expected launchd startup classification failure, stdout:\n{}",
        String::from_utf8_lossy(&start.stdout)
    );
    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_status.json")).expect("service status"),
    )
    .expect("parse service status");
    assert_eq!(status["service_state"], "startup_failed");
    assert_ne!(status["startup_classification"], "startup_runtime_ready");
    assert_eq!(status["first_daemon_record_observed"], false);
    let startup_ledger =
        fs::read_to_string(service_root.join("logs/startup_ledger.jsonl")).expect("startup ledger");
    assert!(startup_ledger.contains("launchd_bootstrap_failed"));
    let observability =
        fs::read_to_string(service_root.join("logs/observability.log")).expect("observability log");
    assert!(observability.contains("stage=launchd_bootstrap_failed"));
    let otel = fs::read_to_string(service_root.join("logs/otel.jsonl")).expect("otel log");
    assert!(otel.contains("\"name\":\"csm.launchd_bootstrap_failed\""));
}

#[test]
fn csm_service_local_start_refuses_unverified_live_pid_metadata() {
    let root = unique_test_temp_dir("csm-service-unverified-pid");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-unverified-pid-agent
display_name: Service Unverified PID Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_unverified_pid_probe
  run_args: {}
heartbeat:
  interval_secs: 10
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-unverified-pid-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let (api_probe, api_bind) = reserve_csm_test_port("unverified pid API");
    drop(api_probe);
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--api-bind",
        &api_bind,
        "--label",
        "com.agentlogic.csm.test-unverified-pid",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );
    fs::write(
        service_root.join("csm-service.pid"),
        std::process::id().to_string(),
    )
    .expect("write live but unowned pid");

    let start = run_csm(&[
        "service",
        "start",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        !start.status.success(),
        "expected unverified live pid refusal, stdout:\n{}",
        String::from_utf8_lossy(&start.stdout)
    );
    let stderr = String::from_utf8_lossy(&start.stderr);
    assert!(
        stderr.contains("refused live but unverified pid metadata"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csm_daemon_bounded_test_supervisor_failure_leaves_recoverable_checkpoint() {
    let root = unique_test_temp_dir("csm-daemon-failure");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-failure-agent
display_name: Daemon Failure Agent
state_root: state
workflow:
  kind: unsupported_adapter
  name: failing_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 5
memory:
  namespace: smoke/daemon-failure-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let disk_ready_env = [
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ];
    let out = run_csm_with_env_without_aws_credentials(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--test-supervisor-failure-after-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &disk_ready_env,
    );
    assert!(
        !out.status.success(),
        "expected daemon failure, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(daemon_status["state"], "failed");
    assert_eq!(daemon_status["service_mode"], "bounded_test_only");
    assert_eq!(daemon_status["bounded_test_mode"], true);
    assert_eq!(daemon_status["restart_count"], 1);
    assert_eq!(
        daemon_status["last_event"],
        "bounded_test_supervisor_failure"
    );
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());
    assert!(root.join("state/safe_fail_bundle.json").exists());
    assert!(root
        .join("state/safe_fail_artifacts/safe-fail-000001.json")
        .exists());

    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/status.json")).expect("read status"),
    )
    .expect("parse status");
    assert_eq!(status["state"], "failed");
    assert_eq!(status["last_error"]["class"], "daemon_child_failed");

    let operator_events =
        fs::read_to_string(root.join("state/operator_events.jsonl")).expect("operator events");
    assert!(operator_events.contains("\"event\":\"restart_scheduled\""));
    assert!(operator_events.contains("\"event\":\"restart_attempted\""));
    assert!(operator_events.contains("\"event\":\"bounded_test_supervisor_failure\""));
    assert!(operator_events.contains("\"event\":\"safe_fail_serialization\""));
    assert!(operator_events.contains("\"event\":\"governed_runtime_notice\""));
    assert!(operator_events.contains("\"checkpoint_ref\":\"continuity_checkpoint.json\""));

    let safe_fail: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/safe_fail_bundle.json")).expect("safe fail bundle"),
    )
    .expect("parse safe fail bundle");
    assert_eq!(safe_fail["schema"], "adl.csm.safe_fail_bundle.v1");
    assert_eq!(safe_fail["runtime_owner"], "csm");
    assert_eq!(safe_fail["trigger"], "bounded_test_supervisor_failure");
    assert_eq!(safe_fail["agent_outcome"]["state"], "recoverable");
    assert_eq!(
        safe_fail["recoverability"]["class"],
        "recoverable_checkpointed"
    );
    assert_eq!(
        safe_fail["monotonicity"]["does_not_rewrite_continuity_checkpoint"],
        true
    );
    assert_eq!(
        safe_fail["observability"]["otel_service_name"],
        "csm-runtime-daemon"
    );
    assert!(safe_fail["serialized_refs"]
        .as_array()
        .expect("serialized refs")
        .iter()
        .any(|artifact| artifact["role"] == "continuity_checkpoint"
            && artifact["status"] == "retained"));
    assert_eq!(
        safe_fail["serialized_state"]["status"]["value"]["last_error"]["class"],
        "daemon_child_failed"
    );

    let notice_latest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/csm_governed_notice_latest.json"))
            .expect("read governed notice latest"),
    )
    .expect("parse governed notice latest");
    assert_eq!(notice_latest["schema"], "adl.csm.governed_notice.v1");
    assert_eq!(notice_latest["runtime_owner"], "csm");
    assert_eq!(notice_latest["notice_kind"], "shutdown");
    assert_eq!(notice_latest["severity"], "critical");
    assert_eq!(notice_latest["trigger"], "bounded_test_supervisor_failure");
    assert_eq!(
        notice_latest["local_first_policy"]["source_of_truth"],
        "local_safe_fail_and_checkpoint_artifacts"
    );
    assert_eq!(
        notice_latest["local_first_policy"]["transport_failure_policy"],
        "retain_delivery_failure_and_continue_recovery"
    );
    let attempts = notice_latest["delivery_attempts"]
        .as_array()
        .expect("delivery attempts");
    assert!(
        attempts
            .iter()
            .any(|attempt| attempt["channel"] == "local_notice_ledger"
                && attempt["status"] == "recorded"),
        "local notice ledger attempt missing: {notice_latest}"
    );
    let queued_behind_prior_notice = matches!(
        notice_latest["typed_channel_delivery"]["status"].as_str(),
        Some("durably_spooled_waiting_for_replay" | "durably_spooled_behind_unacknowledged_sequence")
    )
        && notice_latest["typed_channel_delivery"]["cursor_advanced"] == false;
    let blocked_before_route_sequence = notice_latest["typed_channel_delivery"]["status"]
        == "blocked_before_sequence_reservation"
        && notice_latest["typed_channel_delivery"]["cursor_advanced"] == false
        && notice_latest["typed_channel_delivery"]["preflight"]["failure_class"]
            == "csm_notice_route_not_configured";
    assert!(
        attempts
            .iter()
            .any(|attempt| attempt["channel"] == "cloudwatch_logs"
                && attempt["status"] == "not_configured")
            || queued_behind_prior_notice
            || blocked_before_route_sequence,
        "cloudwatch notice attempt missing: {notice_latest}"
    );
    assert!(
        attempts.iter().any(
            |attempt| attempt["channel"] == "acip_sns" && attempt["status"] == "not_configured"
        ) || queued_behind_prior_notice
            || blocked_before_route_sequence,
        "acip_sns notice attempt missing: {notice_latest}"
    );
    assert!(
        attempts
            .iter()
            .any(|attempt| attempt["channel"] == "cloudfront_control_plane"
                && attempt["status"] == "not_configured"
                && attempt["dependency"] == "#4915")
            || queued_behind_prior_notice
            || blocked_before_route_sequence,
        "cloudfront notice attempt missing: {notice_latest}"
    );
    let notice_ledger =
        fs::read_to_string(root.join("state/csm_governed_notices.jsonl")).expect("notice ledger");
    assert!(notice_ledger.contains("\"trigger\":\"daemon_child_failed\""));
    assert!(notice_ledger.contains("\"trigger\":\"bounded_test_supervisor_failure\""));
    let ledger_entries = notice_ledger
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("notice ledger JSON"))
        .collect::<Vec<_>>();
    let blocked_child_notice = ledger_entries
        .iter()
        .rev()
        .find(|entry| entry["trigger"] == "daemon_child_failed")
        .expect("failed-child notice");
    assert_eq!(
        blocked_child_notice["publish_preflight"]["status"],
        "blocked"
    );
    assert_eq!(
        blocked_child_notice["typed_channel_delivery"]["status"],
        "blocked_before_sequence_reservation"
    );
    let child_attempts = blocked_child_notice["delivery_attempts"]
        .as_array()
        .expect("failed-child delivery attempts");
    assert_eq!(child_attempts.len(), 1);
    assert_eq!(child_attempts[0]["channel"], "local_notice_ledger");
    assert_eq!(child_attempts[0]["status"], "recorded");
}
