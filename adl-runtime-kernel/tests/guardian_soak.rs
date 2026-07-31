use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
    time::Duration,
};
#[cfg(unix)]
use std::{
    io::{Read, Write},
    net::SocketAddr,
};

use adl_runtime_kernel::{
    encode_acip_envelope, ControlAction, DomainWork, SignedControlCommand, ACIP_WEBSOCKET_SCHEMA,
    DOMAIN_WORK_SCHEMA,
};

#[path = "support/runtime_init.rs"]
mod runtime_init;
use runtime_init::{
    toml_path, write_for_state as write_test_runtime_init_for_state,
    write_with_certificate_for_state as write_test_runtime_init_with_certificate_for_state,
};

#[cfg(unix)]
const CONTROL_TEST_HOST: &str = "127.0.0.1";

#[cfg(unix)]
struct TestGuardianLease {
    address: SocketAddr,
    token: String,
    release: Option<std::sync::mpsc::Sender<()>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

#[cfg(unix)]
impl TestGuardianLease {
    fn new(label: &str) -> Self {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let token = format!("runtime-kernel-test-guardian-lease-{label}");
        let token_for_thread = token.clone();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let thread = std::thread::spawn(move || {
            let (mut stream, peer) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if release_rx.try_recv().is_ok() {
                            return;
                        }
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("test Guardian lease accept failed: {error}"),
                }
            };
            assert!(peer.ip().is_loopback());
            let mut supplied = vec![0_u8; token_for_thread.len()];
            stream.read_exact(&mut supplied).unwrap();
            assert_eq!(supplied, token_for_thread.as_bytes());
            stream.write_all(b"ok").unwrap();
            let _ = release_rx.recv();
        });
        Self {
            address,
            token,
            release: Some(release_tx),
            thread: Some(thread),
        }
    }

    fn apply(&self, command: &mut Command) {
        command
            .env(
                "ADL_RUNTIME_GUARDIAN_LEASE_ADDRESS",
                self.address.to_string(),
            )
            .env("ADL_RUNTIME_GUARDIAN_LEASE_TOKEN", &self.token);
    }
}

#[cfg(unix)]
impl Drop for TestGuardianLease {
    fn drop(&mut self) {
        if let Some(release) = self.release.take() {
            let _ = release.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[test]
fn production_like_soak_rollback_packet_retains_cutover_boundaries() {
    let packet: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_soak_rollback_5253.v1.json"
    ))
    .unwrap();
    assert_eq!(packet["schema"], "adl.runtime_v3.soak_rollback_proof.v1");
    assert_eq!(packet["issue"], 5253);
    assert_eq!(packet["explicit_runtime_v3_selection"], true);
    assert_eq!(packet["default_runtime_changed"], false);
    assert_eq!(packet["runtime_v2_deleted"], false);
    assert_eq!(packet["cutover_authorized"], false);
    assert_eq!(packet["control_api"]["port"], 20997);
    assert_eq!(packet["soak"]["cycles"], 100);
    assert_eq!(packet["rollback"]["default_backend"], "v2");
    assert_eq!(packet["rollback"]["rollback_target"], "v2");
    assert_eq!(
        packet["deferred_lanes"]["gpu"],
        "deferred_non_cutover_surface; no approved GPU host was available in this issue lane"
    );
    assert!(packet["non_claims"]
        .as_array()
        .unwrap()
        .iter()
        .any(|claim| claim == "This packet does not claim a fixed Horust release."));

    let resolved = packet["parity_routing"]["resolved_here"]
        .as_array()
        .unwrap();
    assert_eq!(resolved, &[serde_json::json!("guardian.packaging_soak")]);

    let classification: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json"
    ))
    .unwrap();
    let guardian = classification["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .find(|capability| capability["id"] == "guardian.packaging_soak")
        .unwrap();
    assert_eq!(guardian["disposition"], "accepted_intentional_divergence");
    assert!(guardian["proof"].as_str().unwrap().contains("#5253"));
    assert!(
        classification["capabilities"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|capability| capability.get("blocking_issue") == Some(&serde_json::json!(5253)))
            .count()
            == 0
    );
}

#[cfg(unix)]
async fn tls_request(
    connector: &tokio_rustls::TlsConnector,
    address: std::net::SocketAddr,
    request: &[u8],
) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio_rustls::rustls::pki_types::ServerName;
    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let mut stream = connector
        .connect(ServerName::try_from("localhost").unwrap(), stream)
        .await
        .unwrap();
    stream.write_all(request).await.unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    String::from_utf8(response).unwrap()
}

#[cfg(unix)]
struct ChildGuard(Option<std::process::Child>, i32);

#[cfg(unix)]
impl ChildGuard {
    fn new(child: std::process::Child) -> Self {
        let pgid = child.id() as i32;
        Self(Some(child), pgid)
    }
}

#[cfg(unix)]
impl Drop for ChildGuard {
    fn drop(&mut self) {
        cleanup_process_group(self.1);
        if let Some(child) = self.0.as_mut() {
            if matches!(child.try_wait(), Ok(None)) {
                let _ = child.kill();
            }
            let _ = child.wait();
        }
    }
}

#[cfg(unix)]
fn bounded_output(command: &mut Command) -> std::process::Output {
    configure_process_group(command);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = ChildGuard::new(command.spawn().unwrap());
    let mut stdout = child.0.as_mut().unwrap().stdout.take().unwrap();
    let mut stderr = child.0.as_mut().unwrap().stderr.take().unwrap();
    let stdout_reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).unwrap();
        bytes
    });
    let stderr_reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).unwrap();
        bytes
    });
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        if child.0.as_mut().unwrap().try_wait().unwrap().is_some() {
            let status = child.0.as_mut().unwrap().wait().unwrap();
            let output = std::process::Output {
                status,
                stdout: stdout_reader.join().unwrap(),
                stderr: stderr_reader.join().unwrap(),
            };
            drop(child);
            return output;
        }
        if std::time::Instant::now() >= deadline {
            drop(child);
            let _ = stdout_reader.join();
            let _ = stderr_reader.join();
            panic!("Horust did not terminate within the bounded test deadline");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(unix)]
fn cleanup_process_group(pgid: i32) {
    let signaled = unsafe { libc::kill(-pgid, libc::SIGTERM) == 0 };
    if signaled {
        std::thread::sleep(Duration::from_millis(100));
        unsafe {
            let _ = libc::kill(-pgid, libc::SIGKILL);
        }
    }
}

#[cfg(unix)]
fn configure_process_group(command: &mut Command) {
    use std::os::unix::process::CommandExt;

    unsafe {
        command.pre_exec(|| {
            if libc::setpgid(0, 0) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}

fn update_weather_config(init: &Path, values: &[(&str, i64)]) {
    let text = std::fs::read_to_string(init).unwrap();
    let mut document = toml::from_str::<toml::Value>(&text).unwrap();
    let weather = document
        .get_mut("weather")
        .and_then(toml::Value::as_table_mut)
        .unwrap();
    for (field, value) in values {
        weather.insert((*field).to_owned(), toml::Value::Integer(*value));
    }
    std::fs::write(init, toml::to_string_pretty(&document).unwrap()).unwrap();
}

#[cfg(unix)]
fn local_state_root(directory: &Path, name: &str) -> PathBuf {
    let root = directory.join(name);
    std::fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap()
}

#[cfg(unix)]
fn runtime_kernel_command(init: &Path, lease: &TestGuardianLease) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"));
    command.arg("serve").arg("--init").arg(init);
    lease.apply(&mut command);
    command
}

#[cfg(unix)]
fn copy_directory(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let source = entry.path();
        let target = to.join(entry.file_name());
        if source.is_dir() {
            copy_directory(&source, &target);
        } else {
            std::fs::copy(&source, &target).unwrap();
        }
    }
}

fn live_agent_work(input: &str) -> Vec<u8> {
    serde_json::json!({
        "schema":"adl.runtime.local_agent_work.v1",
        "tasks":[{"op":"blake3","input":input}]
    })
    .to_string()
    .into_bytes()
}

#[tokio::test]
async fn guardian_lease_loss_checkpoints_and_stops_the_real_kernel() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let directory = tempfile::tempdir().unwrap();
    let state_root = local_state_root(directory.path(), "guardian-lease-state");
    let continuity_root = state_root.join("continuity");
    let api_probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let api_address = api_probe.local_addr().unwrap();
    drop(api_probe);
    let init = write_test_runtime_init_for_state(directory.path(), api_address, &state_root);
    let lease_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let lease_address = lease_listener.local_addr().unwrap();
    let lease_token = "portable-guardian-lease-token-00000001";
    let mut child = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .env(
            "ADL_RUNTIME_GUARDIAN_LEASE_ADDRESS",
            lease_address.to_string(),
        )
        .env("ADL_RUNTIME_GUARDIAN_LEASE_TOKEN", lease_token)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let (mut lease, peer) = tokio::time::timeout(Duration::from_secs(5), lease_listener.accept())
        .await
        .expect("kernel did not connect to its Guardian lease")
        .unwrap();
    assert!(peer.ip().is_loopback());
    let mut supplied = vec![0_u8; lease_token.len()];
    lease.read_exact(&mut supplied).await.unwrap();
    assert_eq!(supplied, lease_token.as_bytes());
    lease.write_all(b"ok").await.unwrap();

    let stderr = child.stderr.take().unwrap();
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let stderr_reader = std::thread::spawn(move || {
        let mut output = String::new();
        for line in BufReader::new(stderr).lines() {
            let line = line.unwrap();
            if line.contains("event=control_ready") {
                let _ = ready_tx.send(());
            }
            output.push_str(&line);
            output.push('\n');
        }
        output
    });
    ready_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("kernel did not become ready under its Guardian lease");
    drop(lease);

    let status = tokio::task::spawn_blocking(move || child.wait().unwrap())
        .await
        .unwrap();
    let stderr = stderr_reader.join().unwrap();
    assert!(
        status.success(),
        "Guardian lease shutdown failed ({status}): {stderr}"
    );
    assert!(stderr.contains("event=guardian_lease_lost action=checkpoint_shutdown"));
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity_root.join("generation-1/manifest.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(manifest["generation"], 1);
    assert_eq!(manifest["signing_algorithm"], "ed25519");
}

#[cfg(unix)]
#[test]
fn pressure_checkpoint_failure_keeps_signal_shutdown_responsive() {
    use ed25519_dalek::SigningKey;

    let directory = tempfile::tempdir().unwrap();
    let state_root = local_state_root(directory.path(), "pressure-failure-state");
    let continuity_root = state_root.join("continuity");
    std::fs::create_dir_all(&continuity_root).unwrap();
    std::fs::create_dir(continuity_root.join(".generation-1.pending")).unwrap();
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let init = write_test_runtime_init_for_state(directory.path(), address, &state_root);
    update_weather_config(
        &init,
        &[
            ("sample_millis", 100),
            ("memory_recover_used_basis_points", 0),
            ("memory_warning_used_basis_points", 1),
            ("memory_stop_used_basis_points", 2),
            ("cpu_recover_basis_points", 0),
            ("cpu_warning_basis_points", 1),
            ("cpu_stop_basis_points", 2),
        ],
    );
    let lease = TestGuardianLease::new("pressure-failure");
    let mut command = runtime_kernel_command(&init, &lease);
    let mut child = command
        .env(
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
            hex::encode(
                SigningKey::from_bytes(&[17_u8; 32])
                    .verifying_key()
                    .as_bytes(),
            ),
        )
        .env(
            "ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX",
            hex::encode([23_u8; 32]),
        )
        .env("ADL_RUNTIME_CONTINUITY_MIN_GENERATION", "0")
        .env(
            "ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX",
            hex::encode(
                SigningKey::from_bytes(&[29_u8; 32])
                    .verifying_key()
                    .as_bytes(),
            ),
        )
        .env(
            "ADL_RUNTIME_OBSERVATORY_TOKEN",
            "guardian-observatory-token-00000003",
        )
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let stderr = child.stderr.take().unwrap();
    let (failure_tx, failure_rx) = std::sync::mpsc::channel();
    let stderr_reader = std::thread::spawn(move || {
        let mut output = String::new();
        for line in BufReader::new(stderr).lines() {
            let line = line.unwrap();
            if line.contains("runtime pressure continuity checkpoint failed") {
                let _ = failure_tx.send(());
            }
            output.push_str(&line);
            output.push('\n');
        }
        output
    });
    failure_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("pressure checkpoint collision was not observed");
    assert!(
        child.try_wait().unwrap().is_none(),
        "checkpoint failure must keep Runtime v3 alive"
    );
    assert!(
        std::net::TcpStream::connect(address).is_ok(),
        "checkpoint failure must leave the control API reachable"
    );
    std::fs::remove_dir(continuity_root.join(".generation-1.pending")).unwrap();
    assert_eq!(unsafe { libc::kill(child.id() as i32, libc::SIGTERM) }, 0);
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            panic!("Runtime v3 did not handle SIGTERM during pressure retry delay");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let stderr = stderr_reader.join().unwrap();

    assert!(
        status.success(),
        "signal shutdown after pressure failure failed ({status}): {stderr}"
    );
    assert!(stderr.contains("event=resource_pressure_stop"));
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity_root.join("generation-1/manifest.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(manifest["generation"], 1);
    assert_eq!(manifest["signing_algorithm"], "ed25519");
}

#[cfg(unix)]
#[tokio::test]
async fn pressure_closes_ingress_serializes_live_work_and_stops_cleanly() {
    use ed25519_dalek::SigningKey;
    use tokio_rustls::rustls::{pki_types::CertificateDer, ClientConfig, RootCertStore};

    let directory = tempfile::tempdir().unwrap();
    let state_root = local_state_root(directory.path(), "pressure-success-state");
    let continuity_root = state_root.join("continuity");
    let probe = std::net::TcpListener::bind((CONTROL_TEST_HOST, 0)).unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let (init, certificate_der) =
        write_test_runtime_init_with_certificate_for_state(directory.path(), address, &state_root);
    update_weather_config(
        &init,
        &[
            ("sample_millis", 3_000),
            ("disk_stop_free_bytes", 9_000_000_000_000_000_000),
            ("disk_warning_free_bytes", 9_000_000_000_000_000_001),
            ("disk_recover_free_bytes", 9_000_000_000_000_000_002),
        ],
    );
    let control_key = SigningKey::from_bytes(&[43_u8; 32]);
    std::fs::write(
        state_root.join("credentials/control-public-key.hex"),
        hex::encode(control_key.verifying_key().as_bytes()),
    )
    .unwrap();
    std::fs::write(
        state_root.join("credentials/continuity-signing-key.hex"),
        hex::encode([47_u8; 32]),
    )
    .unwrap();
    std::fs::write(
        state_root.join("credentials/operation-public-key.hex"),
        hex::encode(
            SigningKey::from_bytes(&[53_u8; 32])
                .verifying_key()
                .as_bytes(),
        ),
    )
    .unwrap();
    std::fs::write(
        state_root.join("credentials/observatory-token.txt"),
        "pressure-observatory-token-000001",
    )
    .unwrap();
    let lease = TestGuardianLease::new("pressure-success");
    let mut command = runtime_kernel_command(&init, &lease);
    let mut child = command
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let stderr = child.stderr.take().unwrap();
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let stderr_reader = std::thread::spawn(move || {
        let mut output = String::new();
        for line in BufReader::new(stderr).lines() {
            let line = line.unwrap();
            if line.contains("event=control_ready") {
                let instance = line
                    .split_whitespace()
                    .find_map(|field| field.strip_prefix("instance_id="))
                    .unwrap()
                    .to_owned();
                let _ = ready_tx.send(instance);
            }
            output.push_str(&line);
            output.push('\n');
        }
        output
    });
    let instance_id = ready_rx.recv_timeout(Duration::from_secs(5)).unwrap();
    let mut roots = RootCertStore::empty();
    roots.add(CertificateDer::from(certificate_der)).unwrap();
    let connector = tokio_rustls::TlsConnector::from(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ));
    let command = SignedControlCommand::sign(
        "pressure-submit",
        blake3::hash(b"pressure-submit").to_hex()[..32].to_owned(),
        instance_id,
        "operator",
        ControlAction::Submit {
            work: DomainWork {
                schema: DOMAIN_WORK_SCHEMA.to_owned(),
                work_id: "pressure-work".to_owned(),
                kind: "parity-a".to_owned(),
                payload: live_agent_work("serialize-before-stop"),
            },
        },
        "operator",
        &control_key,
    )
    .unwrap();
    let body = serde_json::to_vec(&command).unwrap();
    let mut request = format!("POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len()).into_bytes();
    request.extend(body);
    assert!(tls_request(&connector, address, &request)
        .await
        .starts_with("HTTP/1.1 200 OK"));
    let status = child.wait().unwrap();
    let stderr = stderr_reader.join().unwrap();
    assert!(
        status.success(),
        "pressure shutdown failed ({status}): {stderr}"
    );
    assert!(stderr.contains("event=resource_pressure_stop"));
    let checkpoint: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity_root.join("generation-1/0000-live_kernel.bin")).unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint["ingress"]["accepted_through"], 1);
    assert!(checkpoint["ingress"]["completed"]["pressure-work"]["result_hash"].is_string());
}

#[cfg(unix)]
#[test]
fn serve_requires_init_declared_state_root_before_live_adapters_start() {
    let directory = tempfile::tempdir().unwrap();
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let tls_root = directory.path().join("tls");
    std::fs::create_dir_all(&tls_root).unwrap();
    let certificate = tls_root.join("localhost-cert.pem");
    let private_key = tls_root.join("localhost-key.pem");
    std::fs::write(&certificate, "test certificate").unwrap();
    std::fs::write(&private_key, "test private key").unwrap();
    let init = directory
        .path()
        .join("runtime-init-missing-state-root.toml");
    std::fs::write(
        &init,
        format!(
            r#"schema = "adl.runtime_v3.init.v1"
[api]
address = "{}"
public_base_url = "https://localhost:{}"
bind_attempts = 20
bind_retry_millis = 100
websocket_auth_timeout_millis = 5000
websocket_refresh_millis = 1000
websocket_max_frame_bytes = 65536
[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
"#,
            address,
            address.port(),
            toml_path(&certificate),
            toml_path(&private_key),
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(78));
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing field `state_root`"));

    let state_root = local_state_root(directory.path(), "override-rejected-state");
    let init = write_test_runtime_init_for_state(directory.path(), address, &state_root);
    let output = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .arg("--state-root")
        .arg(&state_root)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(64));
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("runtime state_root must be declared inside --init"));
}

#[cfg(unix)]
#[test]
fn serve_refuses_reused_continuity_and_operation_keys() {
    use ed25519_dalek::SigningKey;

    let directory = tempfile::tempdir().unwrap();
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let reused = SigningKey::from_bytes(&[23_u8; 32]);
    let state_root = local_state_root(directory.path(), "reused-key-state");
    let init = write_test_runtime_init_for_state(directory.path(), address, &state_root);
    std::fs::write(
        state_root.join("credentials/continuity-signing-key.hex"),
        hex::encode(reused.to_bytes()),
    )
    .unwrap();
    std::fs::write(
        state_root.join("credentials/operation-public-key.hex"),
        hex::encode(reused.verifying_key().as_bytes()),
    )
    .unwrap();
    let lease = TestGuardianLease::new("reused-key");
    let mut command = runtime_kernel_command(&init, &lease);
    let output = command.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(78), "kernel stderr: {stderr}");
    assert!(
        stderr.contains("runtime continuity and operation keys must be distinct"),
        "kernel stderr: {stderr}"
    );
}

#[tokio::test]
async fn signed_https_wss_shutdown_checkpoints_and_forgery_cannot_stop_the_process() {
    use ed25519_dalek::SigningKey;
    use futures::{SinkExt, StreamExt};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio_rustls::rustls::{
        pki_types::{CertificateDer, ServerName},
        ClientConfig, RootCertStore,
    };
    use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};

    let directory = tempfile::tempdir().unwrap();
    let state_root = local_state_root(directory.path(), "remote-control-state");
    let continuity_root = state_root.join("continuity");
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let (init, certificate_der) =
        write_test_runtime_init_with_certificate_for_state(directory.path(), address, &state_root);
    let control_key = SigningKey::from_bytes(&[17_u8; 32]);
    let lease = TestGuardianLease::new("remote-control");
    let mut command = runtime_kernel_command(&init, &lease);
    let mut child = command
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let stderr = child.stderr.take().unwrap();
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let stderr_reader = std::thread::spawn(move || {
        let mut output = String::new();
        for line in BufReader::new(stderr).lines() {
            let line = line.unwrap();
            if line.contains("event=control_ready") {
                let instance = line
                    .split_whitespace()
                    .find_map(|field| field.strip_prefix("instance_id="))
                    .unwrap()
                    .to_owned();
                let _ = ready_tx.send(instance);
            }
            output.push_str(&line);
            output.push('\n');
        }
        output
    });
    let instance_id = ready_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("serve did not report control readiness");

    let client_config = Arc::new(
        ClientConfig::builder()
            .with_root_certificates({
                let mut roots = RootCertStore::empty();
                roots
                    .add(CertificateDer::from(certificate_der.clone()))
                    .unwrap();
                roots
            })
            .with_no_client_auth(),
    );
    let connector = tokio_rustls::TlsConnector::from(client_config.clone());
    let wrong_host_stream = tokio::net::TcpStream::connect(address).await.unwrap();
    assert!(
        connector
            .connect(
                ServerName::try_from("wrong.local").unwrap(),
                wrong_host_stream,
            )
            .await
            .is_err(),
        "test CA leaf unexpectedly validated for the wrong hostname"
    );
    let request = |mut command: SignedControlCommand| {
        let connector = connector.clone();
        async move {
            let stream = tokio::net::TcpStream::connect(address).await.unwrap();
            let mut stream = connector
                .connect(ServerName::try_from("localhost").unwrap(), stream)
                .await
                .unwrap();
            let body = serde_json::to_vec(&command).unwrap();
            let headers = format!(
                "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(headers.as_bytes()).await.unwrap();
            stream.write_all(&body).await.unwrap();
            let mut response = Vec::new();
            stream.read_to_end(&mut response).await.unwrap();
            command.signature.clear();
            String::from_utf8(response).unwrap()
        }
    };
    let signed = |id: &str, action: ControlAction| {
        SignedControlCommand::sign(
            id,
            blake3::hash(id.as_bytes()).to_hex()[..32].to_owned(),
            &instance_id,
            "operator",
            action,
            "operator",
            &control_key,
        )
        .unwrap()
    };
    let work = DomainWork {
        schema: DOMAIN_WORK_SCHEMA.to_owned(),
        work_id: "guardian-work-1".to_owned(),
        kind: "parity-a".to_owned(),
        payload: live_agent_work("guardian-live-ingress"),
    };
    let submit_response = request(signed(
        "valid-submit",
        ControlAction::Submit { work: work.clone() },
    ))
    .await;
    assert!(submit_response.starts_with("HTTP/1.1 200 OK"));
    let submit: serde_json::Value =
        serde_json::from_str(submit_response.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    assert_eq!(submit["outcome"]["result"], "submitted");
    assert_eq!(submit["outcome"]["work_result"]["accepted_sequence"], 1);

    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let mut stream = connector
        .connect(ServerName::try_from("localhost").unwrap(), stream)
        .await
        .unwrap();
    stream.write_all(b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer guardian-observatory-token-00000001\r\nConnection: close\r\n\r\n").await.unwrap();
    let mut observatory_response = Vec::new();
    stream.read_to_end(&mut observatory_response).await.unwrap();
    let observatory_response = String::from_utf8(observatory_response).unwrap();
    assert!(observatory_response.starts_with("HTTP/1.1 200 OK"));
    let observatory: serde_json::Value =
        serde_json::from_str(observatory_response.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    assert_eq!(observatory["ingress"]["accepted_through"], 1);
    assert_eq!(
        observatory["ingress"]["completed"]["guardian-work-1"]["result_hash"],
        submit["outcome"]["work_result"]["result_hash"]
    );

    let websocket_request = format!("wss://localhost:{}/v1/observatory/ws", address.port());
    let mut websocket_request = websocket_request.into_client_request().unwrap();
    websocket_request
        .headers_mut()
        .insert("Origin", "https://localhost:8765".parse().unwrap());
    let websocket_stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let (mut websocket, _) = tokio_tungstenite::client_async_tls_with_config(
        websocket_request,
        websocket_stream,
        None,
        Some(tokio_tungstenite::Connector::Rustls(client_config)),
    )
    .await
    .unwrap();
    websocket
        .send(Message::Text(
            serde_json::json!({
                "schema": "adl.runtime_v3.observatory_ws_auth.v1",
                "bearer_token": "guardian-observatory-token-00000001"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let feed = tokio::time::timeout(Duration::from_secs(3), websocket.next())
        .await
        .expect("WSS Observatory feed did not arrive")
        .expect("WSS Observatory connection closed")
        .expect("WSS Observatory frame failed");
    let feed = serde_json::from_str::<serde_json::Value>(feed.to_text().unwrap()).unwrap();
    assert_eq!(feed["schema"], "adl.runtime_v3.observatory_feed.v2");
    assert_eq!(feed["runtime_selection"], "runtime_v3_explicit_opt_in");
    assert_eq!(feed["control"]["websocket_full_duplex"], true);
    assert_eq!(
        feed["control"]["websocket_acip_binary_schema"],
        ACIP_WEBSOCKET_SCHEMA
    );
    let authenticated = tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let frame = websocket
                .next()
                .await
                .expect("WSS connection closed before authentication result")
                .expect("WSS authentication result frame failed");
            let value =
                serde_json::from_str::<serde_json::Value>(frame.to_text().unwrap()).unwrap();
            if value["schema"] == "adl.runtime_v3.observatory_ws_control_result.v1" {
                break value;
            }
        }
    })
    .await
    .expect("WSS authentication result did not arrive");
    assert_eq!(authenticated["status"], "authenticated");

    let mut forged_ws = signed("wss-forged", ControlAction::Snapshot);
    forged_ws.signature = hex::encode([0_u8; 64]);
    websocket
        .send(Message::Text(
            serde_json::to_string(&forged_ws).unwrap().into(),
        ))
        .await
        .unwrap();
    let rejected = tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let frame = websocket
                .next()
                .await
                .expect("WSS connection closed before rejection")
                .expect("WSS rejection frame failed");
            let value =
                serde_json::from_str::<serde_json::Value>(frame.to_text().unwrap()).unwrap();
            if value["schema"] == "adl.runtime_v3.observatory_ws_control_result.v1" {
                break value;
            }
        }
    })
    .await
    .expect("WSS rejection result did not arrive");
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["command_id"], "wss-forged");
    assert_eq!(rejected["error"], "authentication_failed");

    let snapshot = signed("wss-snapshot", ControlAction::Snapshot);
    websocket
        .send(Message::Text(
            serde_json::to_string(&snapshot).unwrap().into(),
        ))
        .await
        .unwrap();
    let accepted = tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let frame = websocket
                .next()
                .await
                .expect("WSS connection closed before control result")
                .expect("WSS control result frame failed");
            let value =
                serde_json::from_str::<serde_json::Value>(frame.to_text().unwrap()).unwrap();
            if value["schema"] == "adl.runtime_v3.observatory_ws_control_result.v1" {
                break value;
            }
        }
    })
    .await
    .expect("WSS control result did not arrive");
    assert_eq!(accepted["status"], "accepted");
    assert_eq!(accepted["command_id"], "wss-snapshot");
    assert_eq!(accepted["correlation_id"], snapshot.correlation_id);
    assert_eq!(
        accepted["response"]["schema"],
        "adl.runtime.control_response.v1"
    );
    assert_eq!(accepted["response"]["outcome"]["result"], "snapshot");
    let acip_frame = encode_acip_envelope(
        "acip-wss-1",
        "agent-a",
        "agent-b",
        "agent_runtime",
        &serde_json::json!({
            "schema": "adl.runtime.local_agent_work.v1",
            "tasks": [{
                "op": "blake3",
                "input": "Can you review this bounded proposal?"
            }]
        }),
        1,
    )
    .unwrap();
    websocket
        .send(Message::Binary(acip_frame.clone().into()))
        .await
        .unwrap();
    let acip_accepted = tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let frame = websocket
                .next()
                .await
                .expect("WSS connection closed before ACIP result")
                .expect("WSS ACIP result frame failed");
            let value =
                serde_json::from_str::<serde_json::Value>(frame.to_text().unwrap()).unwrap();
            if value["schema"] == ACIP_WEBSOCKET_SCHEMA {
                break value;
            }
        }
    })
    .await
    .expect("WSS ACIP acceptance did not arrive");
    assert_eq!(
        acip_accepted["status"], "completed",
        "unexpected ACIP response: {acip_accepted}"
    );
    assert_eq!(acip_accepted["message_id"], "acip-wss-1");
    assert_eq!(acip_accepted["sequence_reserved"], true);

    websocket
        .send(Message::Binary(acip_frame.into()))
        .await
        .unwrap();
    let acip_replayed = tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let frame = websocket
                .next()
                .await
                .expect("WSS connection closed before ACIP replay rejection")
                .expect("WSS ACIP replay frame failed");
            let value =
                serde_json::from_str::<serde_json::Value>(frame.to_text().unwrap()).unwrap();
            if value["schema"] == ACIP_WEBSOCKET_SCHEMA {
                break value;
            }
        }
    })
    .await
    .expect("WSS ACIP replay rejection did not arrive");
    assert_eq!(acip_replayed["status"], "rejected");
    assert_eq!(acip_replayed["reason"], "monotonic_sequence_must_advance");
    assert_eq!(acip_replayed["sequence_reserved"], false);

    let feed_after_control = tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let frame = websocket
                .next()
                .await
                .expect("WSS connection closed before post-control telemetry")
                .expect("WSS post-control telemetry frame failed");
            let value =
                serde_json::from_str::<serde_json::Value>(frame.to_text().unwrap()).unwrap();
            if value["schema"] == "adl.runtime_v3.observatory_feed.v2" {
                break value;
            }
        }
    })
    .await
    .expect("WSS telemetry did not continue after bidirectional control");
    assert_eq!(feed_after_control["ingress"]["accepted_through"], 2);
    assert_eq!(
        feed_after_control["ingress"]["completed"]["acip-wss-1"]["work_id"],
        "acip-wss-1"
    );
    websocket.close(None).await.unwrap();

    let mut forged = signed("forged-stop", ControlAction::Shutdown { grace_millis: 500 });
    forged.signature = hex::encode([0_u8; 64]);
    assert!(request(forged).await.starts_with("HTTP/1.1 401"));
    assert!(child.try_wait().unwrap().is_none());
    assert!(!continuity_root.join("generation-1").exists());

    assert!(request(signed(
        "valid-stop",
        ControlAction::Shutdown { grace_millis: 500 },
    ))
    .await
    .starts_with("HTTP/1.1 200 OK"));
    let status = child.wait().unwrap();
    let stderr = stderr_reader.join().unwrap();
    assert!(
        status.success(),
        "serve shutdown failed ({status}): {stderr}"
    );
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity_root.join("generation-1/manifest.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(manifest["generation"], 1);
    assert_eq!(manifest["signing_algorithm"], "ed25519");
    let different_state_root = local_state_root(directory.path(), "remote-control-different-state");
    let different_continuity_root = different_state_root.join("continuity");
    copy_directory(&continuity_root, &different_continuity_root);
    let different_init =
        write_test_runtime_init_for_state(directory.path(), address, &different_state_root);
    let restore_lease = TestGuardianLease::new("remote-control-restore");
    let mut restore_with_different_state = runtime_kernel_command(&different_init, &restore_lease);
    restore_with_different_state
        .env(
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
            hex::encode(control_key.verifying_key().as_bytes()),
        )
        .env("ADL_RUNTIME_CONTROL_KEY_ID", "remote-test")
        .env("ADL_RUNTIME_CONTROL_PRINCIPAL", "remote-test")
        .env(
            "ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX",
            hex::encode([23_u8; 32]),
        )
        .env("ADL_RUNTIME_CONTINUITY_MIN_GENERATION", "1")
        .env(
            "ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX",
            hex::encode(
                SigningKey::from_bytes(&[29_u8; 32])
                    .verifying_key()
                    .as_bytes(),
            ),
        )
        .env(
            "ADL_RUNTIME_OBSERVATORY_TOKEN",
            "guardian-observatory-token-00000004",
        );
    let restore_output = bounded_output(&mut restore_with_different_state);
    assert_eq!(restore_output.status.code(), Some(78));
    assert!(String::from_utf8_lossy(&restore_output.stderr)
        .contains("runtime continuity restore refused"));
    let checkpoint: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity_root.join("generation-1/0000-live_kernel.bin")).unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint["ingress"]["accepted_through"], 2);
    assert_eq!(
        checkpoint["ingress"]["completed"]["guardian-work-1"]["result_hash"],
        submit["outcome"]["work_result"]["result_hash"]
    );
    assert_eq!(
        checkpoint["ingress"]["completed"]["acip-wss-1"]["work_id"],
        "acip-wss-1"
    );
}
