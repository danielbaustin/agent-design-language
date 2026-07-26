use std::{
    future::pending,
    io::{BufRead, BufReader, Read},
    net::ToSocketAddrs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    channel,
    proof::{build_proof_runtime, run_proof},
    ChannelFullPolicy, Component, ComponentContext, ComponentError, ComponentFactory, ComponentId,
    ComponentRegistry, ComponentSpec, ControlAction, DomainWork, FailurePolicy, Kernel, KernelExit,
    RuntimeRecorder, SendError, SignedControlCommand, DOMAIN_WORK_SCHEMA,
};
use async_trait::async_trait;

const CONTROL_TEST_HOST: &str = "localhost";
const CONTROL_TEST_PORT: u16 = 20_997;

#[test]
fn packaging_preserves_one_guardian_neutral_child_contract() {
    let rustysd = include_str!("../../infra/rustysd/adl-runtime-kernel.service");
    let systemd = include_str!("../../infra/systemd/adl-runtime-kernel.service");
    let horust = include_str!("../../infra/horust/adl-runtime-kernel.toml");
    let horust_bakeoff = include_str!("../../infra/horust/adl-runtime-kernel-bakeoff.toml");
    for package in [rustysd, systemd, horust] {
        assert!(package.contains("adl-runtime-kernel"));
    }
    assert!(systemd.contains("KillMode=control-group"));
    assert!(systemd.contains("NoNewPrivileges=true"));
    assert!(systemd.contains("RestartPreventExitStatus=78"));
    assert!(systemd.contains("DynamicUser=yes"));
    assert!(horust.contains("strategy = \"on-failure\""));
    assert!(horust.contains("successful-exit-code = [0]"));
    assert!(horust.contains("signal = \"TERM\""));
    assert!(horust.contains(" serve "));
    assert!(horust.contains("ADL_RUNTIME_V3_LOCAL_STATE_DIR"));
    assert!(systemd.contains("ADL_RUNTIME_V3_LOCAL_STATE_DIR=%S/adl/runtime-v3/local-state"));
    assert!(systemd.contains("StateDirectory=adl/runtime-v3"));
    assert!(rustysd.contains("ADL_RUNTIME_V3_LOCAL_STATE_DIR=/var/lib/adl/runtime-v3/local-state"));
    assert!(horust_bakeoff.contains(" fatal-once "));
    let matrix: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_guardian_matrix.v1.json"
    ))
    .unwrap();
    assert_eq!(matrix["candidates"].as_array().unwrap().len(), 3);
    let provenance: serde_json::Value = serde_json::from_str(include_str!(
        "../../infra/horust/horust-0.1.13.provenance.json"
    ))
    .unwrap();
    assert_eq!(provenance["name"], "horust");
    assert_eq!(provenance["version"], "0.1.13");
    assert_eq!(provenance["license"], "MIT");
    assert_eq!(
        provenance["crate_sha256"],
        "a1ee5cbfda91cd77652dfd8849f68ecb40af8d2359ccc91f96fbff3d5a3976a3"
    );
    assert_eq!(provenance["qualification"]["bounded_restart"], "blocked");
    let qualification: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_horust_qualification_report.v1.json"
    ))
    .unwrap();
    assert_eq!(qualification["issue"], 5211);
    let evidence: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/architecture/runtime_v3_horust_qualification_evidence.v1.json"
    ))
    .unwrap();
    assert_eq!(
        evidence["tested_commit"],
        "85326915d25bfedfa78e8cad7496126ca647921c"
    );
    assert_eq!(evidence["runs"].as_array().unwrap().len(), 4);
    assert_eq!(qualification["runtime_source_loc"], 8446);
    assert_eq!(
        qualification["gates"]["macos_native_lifecycle"]["status"],
        "passed"
    );
    assert_eq!(
        qualification["gates"]["linux_native_lifecycle"]["status"],
        "focused_pass"
    );
    assert_eq!(
        qualification["gates"]["linux_systemd_containment"]["status"],
        "passed"
    );
    assert_eq!(
        qualification["gates"]["bounded_restart"]["status"],
        "blocked"
    );
    assert_eq!(
        qualification["gates"]["smallest_gpu_spot"]["status"],
        "blocked_before_launch"
    );
    assert_eq!(qualification["cutover_authorized"], false);
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
#[test]
#[ignore = "requires ADL_HORUST_BIN and exercises native process supervision"]
fn horust_restarts_once_and_restores_continuity() {
    let directory = tempfile::tempdir().unwrap();
    let capsule = directory.path().join("horust-restart.json");
    let mut command = Command::new(horust_binary());
    command
        .arg("--services-path")
        .arg(repo_path("infra/horust/adl-runtime-kernel-bakeoff.toml"))
        .arg("--uds-folder-path")
        .arg(directory.path().join("uds"))
        .env("ADL_RUNTIME_BIN", env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .env("ADL_RUNTIME_CAPSULE", &capsule);
    let output = bounded_output(&mut command);
    assert!(
        output.status.success(),
        "Horust failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let continuity: serde_json::Value =
        serde_json::from_slice(&std::fs::read(capsule).unwrap()).unwrap();
    assert_eq!(continuity["schema"], "adl.runtime_kernel.continuity.v1");
    assert_eq!(continuity["generation"], 2);
}

#[cfg(unix)]
#[test]
#[ignore = "requires ADL_HORUST_BIN and exercises native process supervision"]
fn horust_does_not_restart_configuration_failure() {
    let directory = tempfile::tempdir().unwrap();
    let init = write_test_runtime_init(directory.path(), control_test_addr().parse().unwrap());
    let mut command = Command::new(horust_binary());
    command
        .arg("--services-path")
        .arg(repo_path("infra/horust/adl-runtime-kernel.toml"))
        .arg("--uds-folder-path")
        .arg(directory.path().join("uds"))
        .env("ADL_RUNTIME_BIN", env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .env("ADL_RUNTIME_INIT", init)
        .env(
            "ADL_RUNTIME_CONTINUITY_ROOT",
            directory.path().join("config"),
        );
    let output = bounded_output(&mut command);
    assert!(output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        combined
            .matches("runtime continuity signing key is missing or invalid")
            .count(),
        1,
        "configuration failure must execute exactly once: {combined}"
    );
}

#[cfg(unix)]
#[test]
#[ignore = "requires ADL_HORUST_BIN and reproduces upstream Horust issue 318"]
fn horust_qualification_detects_unbounded_restart_budget() {
    let directory = tempfile::tempdir().unwrap();
    let attempt_log = directory.path().join("attempts.log");
    let child = directory.path().join("always-fail.sh");
    write_executable(
        &child,
        "#!/bin/sh\nprintf 'run\\n' >> \"$ADL_ATTEMPT_LOG\"\nexit 70\n",
    );
    let service = directory.path().join("exhaustion.toml");
    std::fs::write(
        &service,
        format!(
            r#"name = "adl-runtime-kernel-exhaustion"
command = "{}"
stdout = "STDOUT"
stderr = "STDERR"

[restart]
strategy = "on-failure"
backoff = "50ms"
attempts = 3

[failure]
successful-exit-code = [0]
strategy = "ignore"

[environment]
keep-env = false
re-export = ["ADL_ATTEMPT_LOG"]

[termination]
signal = "TERM"
wait = "1s"
"#,
            toml_path(&child)
        ),
    )
    .unwrap();

    let mut guardian = ChildGuard::new({
        let mut command = Command::new(horust_binary());
        configure_process_group(&mut command);
        command
            .arg("--services-path")
            .arg(&service)
            .arg("--uds-folder-path")
            .arg(directory.path().join("uds"))
            .env("ADL_ATTEMPT_LOG", &attempt_log)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
    });
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    let mut observed_attempts = 0;
    let attempts = loop {
        if let Ok(contents) = std::fs::read_to_string(&attempt_log) {
            let attempts = contents.lines().count();
            observed_attempts = attempts;
            if attempts > 4 {
                break attempts;
            }
        }
        if let Some(status) = guardian.0.as_mut().unwrap().try_wait().unwrap() {
            panic!("Horust unexpectedly exhausted the configured budget: {status}");
        }
        assert!(
            std::time::Instant::now() < deadline,
            "Horust did not reproduce the restart-budget defect before the deadline; observed {observed_attempts} launches"
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(
        unsafe { libc::kill(guardian.0.as_ref().unwrap().id() as i32, libc::SIGTERM) },
        0
    );
    assert!(guardian.0.as_mut().unwrap().wait().unwrap().success());
    assert!(
        attempts > 4,
        "Horust unexpectedly respected attempts=3: observed {attempts} launches"
    );
}

#[cfg(unix)]
#[test]
#[ignore = "requires ADL_HORUST_BIN and exercises native environment isolation"]
fn horust_allowlists_child_environment() {
    let directory = tempfile::tempdir().unwrap();
    let environment_log = directory.path().join("environment.log");
    let child = directory.path().join("capture-env.sh");
    write_executable(&child, "#!/bin/sh\nenv > \"$ADL_ENV_OUT\"\n");
    let service = directory.path().join("environment.toml");
    std::fs::write(
        &service,
        format!(
            r#"name = "adl-runtime-kernel-environment"
command = "{}"
stdout = "STDOUT"
stderr = "STDERR"

[restart]
strategy = "never"
backoff = "10ms"
attempts = 1

[failure]
successful-exit-code = [0]
strategy = "ignore"

[environment]
keep-env = false
re-export = ["ADL_ENV_OUT", "ADL_ALLOWED_TEST"]

[termination]
signal = "TERM"
wait = "1s"
"#,
            toml_path(&child)
        ),
    )
    .unwrap();

    let mut command = Command::new(horust_binary());
    command
        .arg("--services-path")
        .arg(&service)
        .arg("--uds-folder-path")
        .arg(directory.path().join("uds"))
        .env("ADL_ENV_OUT", &environment_log)
        .env("ADL_ALLOWED_TEST", "visible")
        .env("OPENAI_API_KEY", "must-not-leak")
        .env("AWS_SECRET_ACCESS_KEY", "must-not-leak");
    let output = bounded_output(&mut command);
    assert!(output.status.success());
    let captured = std::fs::read_to_string(environment_log).unwrap();
    assert!(captured
        .lines()
        .any(|line| line == "ADL_ALLOWED_TEST=visible"));
    assert!(!captured.contains("OPENAI_API_KEY"));
    assert!(!captured.contains("AWS_SECRET_ACCESS_KEY"));
    assert!(!captured.contains("must-not-leak"));
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "requires ADL_HORUST_BIN and binds Runtime v3 control test port"]
async fn horust_forwards_sigterm_and_runtime_checkpoints_cleanly() {
    use ed25519_dalek::SigningKey;
    use tokio_rustls::rustls::{pki_types::CertificateDer, ClientConfig, RootCertStore};

    let directory = tempfile::tempdir().unwrap();
    let continuity_root = directory.path().join("horust-sigterm");
    let address = (CONTROL_TEST_HOST, CONTROL_TEST_PORT)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let (init, certificate_der) =
        write_test_runtime_init_with_certificate(directory.path(), address);
    let control_key = SigningKey::from_bytes(&[19_u8; 32]);
    let mut command = Command::new(horust_binary());
    configure_process_group(&mut command);
    let mut guardian = ChildGuard::new(
        command
            .arg("--services-path")
            .arg(repo_path("infra/horust/adl-runtime-kernel.toml"))
            .arg("--uds-folder-path")
            .arg(directory.path().join("uds"))
            .env("ADL_RUNTIME_BIN", env!("CARGO_BIN_EXE_adl-runtime-kernel"))
            .env("ADL_RUNTIME_INIT", init)
            .env("ADL_RUNTIME_CONTINUITY_ROOT", &continuity_root)
            .env(
                "ADL_RUNTIME_V3_LOCAL_STATE_DIR",
                local_state_root(directory.path(), "horust-sigterm-local-state"),
            )
            .env(
                "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
                hex::encode(control_key.verifying_key().as_bytes()),
            )
            .env("ADL_RUNTIME_CONTROL_KEY_ID", "guardian-test")
            .env("ADL_RUNTIME_CONTROL_PRINCIPAL", "guardian-test")
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
                "guardian-observatory-token-00000001",
            )
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );

    wait_for_control_port(guardian.0.as_mut().unwrap());
    let mut roots = RootCertStore::empty();
    roots.add(CertificateDer::from(certificate_der)).unwrap();
    let connector = tokio_rustls::TlsConnector::from(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ));
    let observatory = tls_request(&connector, address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer guardian-observatory-token-00000001\r\nConnection: close\r\n\r\n").await;
    let feed: serde_json::Value =
        serde_json::from_str(observatory.split("\r\n\r\n").nth(1).unwrap()).unwrap();
    let instance_id = feed["runtime_instance_id"].as_str().unwrap();
    let command = SignedControlCommand::sign(
        "guardian-submit",
        blake3::hash(b"guardian-submit").to_hex()[..32].to_owned(),
        instance_id,
        "guardian-test",
        ControlAction::Submit {
            work: DomainWork {
                schema: DOMAIN_WORK_SCHEMA.to_owned(),
                work_id: "guardian-work".to_owned(),
                kind: "parity-a".to_owned(),
                payload: live_agent_work("horust-live-ingress"),
            },
        },
        "guardian-test",
        &control_key,
    )
    .unwrap();
    let body = serde_json::to_vec(&command).unwrap();
    let head = format!("POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
    let mut request = head.into_bytes();
    request.extend(body);
    let submitted = tls_request(&connector, address, &request).await;
    assert!(submitted.starts_with("HTTP/1.1 200 OK"));
    assert_eq!(
        unsafe { libc::kill(-(guardian.0.as_ref().unwrap().id() as i32), libc::SIGTERM) },
        0
    );
    let status = guardian.0.as_mut().unwrap().wait().unwrap();
    assert!(status.success(), "Horust exited with {status}");

    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    while std::net::TcpStream::connect(control_test_addr()).is_ok() {
        assert!(
            std::time::Instant::now() < deadline,
            "Runtime v3 remained live after guardian termination"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let continuity: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity_root.join("generation-1/manifest.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(continuity["schema"], "adl.runtime.checkpoint.v1");
    assert_eq!(continuity["generation"], 1);
    let checkpoint: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity_root.join("generation-1/0000-live_kernel.bin")).unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint["ingress"]["accepted_through"], 1);
    assert!(checkpoint["ingress"]["completed"]["guardian-work"]["result_hash"].is_string());
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

fn horust_binary() -> PathBuf {
    let configured = PathBuf::from(
        std::env::var_os("ADL_HORUST_BIN")
            .expect("ADL_HORUST_BIN must name the pinned Horust executable"),
    );
    if configured.is_absolute() {
        configured
    } else {
        repo_path(configured.to_str().expect("Horust path must be UTF-8"))
    }
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

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(relative)
}

fn control_test_addr() -> String {
    format!("{CONTROL_TEST_HOST}:{CONTROL_TEST_PORT}")
}

#[cfg(unix)]
fn write_test_runtime_init(directory: &Path, address: std::net::SocketAddr) -> PathBuf {
    write_test_runtime_init_with_certificate(directory, address).0
}

#[cfg(unix)]
fn write_test_runtime_init_with_certificate(
    directory: &Path,
    address: std::net::SocketAddr,
) -> (PathBuf, Vec<u8>) {
    use rcgen::{generate_simple_self_signed, CertifiedKey};

    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(["localhost".to_owned()]).unwrap();
    let certificate = directory.join("cert.pem");
    let private_key = directory.join("key.pem");
    std::fs::write(&certificate, cert.pem()).unwrap();
    std::fs::write(&private_key, signing_key.serialize_pem()).unwrap();
    let init = directory.join("runtime-init.toml");
    std::fs::write(
        &init,
        format!(
            r#"schema = "adl.runtime_v3.init.v1"
[api]
address = "{}"
public_base_url = "https://localhost:{}"
[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
[observatory]
allowed_origins = ["https://localhost:8765"]
[agents]
count = 1
sample_limit = 1
"#,
            address,
            address.port(),
            toml_path(&certificate),
            toml_path(&private_key),
        ),
    )
    .unwrap();
    (init, cert.der().to_vec())
}

#[cfg(unix)]
fn write_executable(path: &Path, contents: &str) {
    use std::os::unix::fs::PermissionsExt;

    std::fs::write(path, contents).unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
}

#[cfg(unix)]
fn toml_path(path: &Path) -> String {
    let value = path.to_string_lossy();
    assert!(!value.contains(['\n', '\r']));
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(unix)]
fn local_state_root(directory: &Path, name: &str) -> PathBuf {
    let root = directory.join(name);
    std::fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap()
}

fn live_agent_work(input: &str) -> Vec<u8> {
    serde_json::json!({
        "schema":"adl.runtime.local_agent_work.v1",
        "tasks":[{"op":"blake3","input":input}]
    })
    .to_string()
    .into_bytes()
}

#[cfg(unix)]
fn wait_for_control_port(guardian: &mut std::process::Child) {
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        if std::net::TcpStream::connect(control_test_addr()).is_ok() {
            return;
        }
        if let Some(status) = guardian.try_wait().unwrap() {
            panic!("Horust exited before Runtime v3 became ready: {status}");
        }
        assert!(
            std::time::Instant::now() < deadline,
            "Runtime v3 did not become ready under Horust"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(unix)]
#[test]
#[ignore = "binds the Runtime v3 control test port"]
fn serve_handles_guardian_sigterm_with_a_clean_checkpointed_exit() {
    use ed25519_dalek::SigningKey;

    let directory = tempfile::tempdir().unwrap();
    let continuity_root = directory.path().join("sigterm-continuity");
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let init = write_test_runtime_init(directory.path(), address);
    let verifying_key = SigningKey::from_bytes(&[17_u8; 32]).verifying_key();
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .arg(&continuity_root)
        .env(
            "ADL_RUNTIME_V3_LOCAL_STATE_DIR",
            local_state_root(directory.path(), "sigterm-local-state"),
        )
        .env(
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
            hex::encode(verifying_key.as_bytes()),
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
            "guardian-observatory-token-00000002",
        )
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let stderr = child.stderr.take().unwrap();
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let stderr_reader = std::thread::spawn(move || {
        let mut output = String::new();
        for line in BufReader::new(stderr).lines() {
            let line = line.unwrap();
            output.push_str(&line);
            output.push('\n');
            if line.contains("event=control_ready") {
                let _ = ready_tx.send(());
            }
        }
        output
    });
    ready_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("serve did not report control readiness");
    assert_eq!(unsafe { libc::kill(child.id() as i32, libc::SIGTERM) }, 0);
    let status = child.wait().unwrap();
    let stderr = stderr_reader.join().unwrap();
    assert!(
        status.success(),
        "serve shutdown failed ({status}): {stderr}"
    );
    let continuity: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity_root.join("generation-1/manifest.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(continuity["schema"], "adl.runtime.checkpoint.v1");
    assert_eq!(continuity["generation"], 1);
    assert_eq!(continuity["signing_algorithm"], "ed25519");
}

#[cfg(unix)]
#[test]
fn pressure_checkpoint_failure_keeps_signal_shutdown_responsive() {
    use ed25519_dalek::SigningKey;

    let directory = tempfile::tempdir().unwrap();
    let continuity_root = directory.path().join("pressure-continuity");
    std::fs::create_dir_all(&continuity_root).unwrap();
    std::fs::create_dir(continuity_root.join(".generation-1.pending")).unwrap();
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let init = write_test_runtime_init(directory.path(), address);
    let mut init_text = std::fs::read_to_string(&init).unwrap();
    init_text.push_str(
        r#"
[weather]
sample_millis = 100
memory_recover_used_basis_points = 0
memory_warning_used_basis_points = 1
memory_stop_used_basis_points = 2
cpu_recover_basis_points = 0
cpu_warning_basis_points = 1
cpu_stop_basis_points = 2
"#,
    );
    std::fs::write(&init, init_text).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .arg("--continuity-root")
        .arg(&continuity_root)
        .env(
            "ADL_RUNTIME_V3_LOCAL_STATE_DIR",
            local_state_root(directory.path(), "pressure-failure-local-state"),
        )
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
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
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
    let continuity_root = directory.path().join("pressure-success");
    let probe = std::net::TcpListener::bind((CONTROL_TEST_HOST, 0)).unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let (init, certificate_der) =
        write_test_runtime_init_with_certificate(directory.path(), address);
    let mut init_text = std::fs::read_to_string(&init).unwrap();
    init_text.push_str(&format!(
        r#"
[weather]
sample_millis = 500
disk_stop_free_bytes = {}
disk_warning_free_bytes = {}
disk_recover_free_bytes = {}
"#,
        9_000_000_000_000_000_000_u64, 9_000_000_000_000_000_001_u64, 9_000_000_000_000_000_002_u64
    ));
    std::fs::write(&init, init_text).unwrap();
    let control_key = SigningKey::from_bytes(&[43_u8; 32]);
    let mut child = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .arg("--continuity-root")
        .arg(&continuity_root)
        .env(
            "ADL_RUNTIME_V3_LOCAL_STATE_DIR",
            local_state_root(directory.path(), "pressure-success-local-state"),
        )
        .env(
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
            hex::encode(control_key.verifying_key().as_bytes()),
        )
        .env("ADL_RUNTIME_CONTROL_KEY_ID", "pressure-test")
        .env("ADL_RUNTIME_CONTROL_PRINCIPAL", "pressure-test")
        .env(
            "ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX",
            hex::encode([47_u8; 32]),
        )
        .env("ADL_RUNTIME_CONTINUITY_MIN_GENERATION", "0")
        .env(
            "ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX",
            hex::encode(
                SigningKey::from_bytes(&[53_u8; 32])
                    .verifying_key()
                    .as_bytes(),
            ),
        )
        .env(
            "ADL_RUNTIME_OBSERVATORY_TOKEN",
            "pressure-observatory-token-000001",
        )
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
        "pressure-test",
        ControlAction::Submit {
            work: DomainWork {
                schema: DOMAIN_WORK_SCHEMA.to_owned(),
                work_id: "pressure-work".to_owned(),
                kind: "parity-a".to_owned(),
                payload: live_agent_work("serialize-before-stop"),
            },
        },
        "pressure-test",
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
fn serve_requires_explicit_local_state_root_before_live_adapters_start() {
    use ed25519_dalek::SigningKey;

    let directory = tempfile::tempdir().unwrap();
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let init = write_test_runtime_init(directory.path(), address);
    let output = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .arg("--continuity-root")
        .arg(directory.path().join("continuity"))
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
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
            hex::encode(
                SigningKey::from_bytes(&[17_u8; 32])
                    .verifying_key()
                    .as_bytes(),
            ),
        )
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(78));
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("runtime local adapter state root is missing"));
}

#[cfg(unix)]
#[test]
fn serve_refuses_reused_continuity_and_operation_keys() {
    use ed25519_dalek::SigningKey;

    let directory = tempfile::tempdir().unwrap();
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let init = write_test_runtime_init(directory.path(), address);
    let reused = SigningKey::from_bytes(&[23_u8; 32]);
    let output = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(init)
        .arg("--continuity-root")
        .arg(directory.path().join("continuity"))
        .env(
            "ADL_RUNTIME_V3_LOCAL_STATE_DIR",
            local_state_root(directory.path(), "reused-key-local-state"),
        )
        .env(
            "ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX",
            hex::encode(reused.to_bytes()),
        )
        .env("ADL_RUNTIME_CONTINUITY_MIN_GENERATION", "0")
        .env(
            "ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX",
            hex::encode(reused.verifying_key().as_bytes()),
        )
        .env(
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
            hex::encode(
                SigningKey::from_bytes(&[17_u8; 32])
                    .verifying_key()
                    .as_bytes(),
            ),
        )
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(78));
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("runtime continuity and operation keys must be distinct"));
}

#[cfg(unix)]
#[tokio::test]
async fn signed_https_shutdown_checkpoints_and_forgery_cannot_stop_the_process() {
    use ed25519_dalek::SigningKey;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio_rustls::rustls::{
        pki_types::{CertificateDer, ServerName},
        ClientConfig, RootCertStore,
    };

    let directory = tempfile::tempdir().unwrap();
    let continuity_root = directory.path().join("remote-continuity");
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let (init, certificate_der) =
        write_test_runtime_init_with_certificate(directory.path(), address);
    let control_key = SigningKey::from_bytes(&[17_u8; 32]);
    let mut child = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .arg("--continuity-root")
        .arg(&continuity_root)
        .env(
            "ADL_RUNTIME_V3_LOCAL_STATE_DIR",
            local_state_root(directory.path(), "remote-control-local-state"),
        )
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
            "guardian-observatory-token-00000004",
        )
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

    let mut roots = RootCertStore::empty();
    roots.add(CertificateDer::from(certificate_der)).unwrap();
    let connector = tokio_rustls::TlsConnector::from(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ));
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
            "remote-test",
            action,
            "remote-test",
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
    stream.write_all(b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer guardian-observatory-token-00000004\r\nConnection: close\r\n\r\n").await.unwrap();
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
    let mut restore_with_different_state = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"));
    restore_with_different_state
        .arg("serve")
        .arg("--init")
        .arg(&init)
        .arg("--continuity-root")
        .arg(&continuity_root)
        .env(
            "ADL_RUNTIME_V3_LOCAL_STATE_DIR",
            local_state_root(directory.path(), "remote-control-different-local-state"),
        )
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
    assert_eq!(checkpoint["ingress"]["accepted_through"], 1);
    assert_eq!(
        checkpoint["ingress"]["completed"]["guardian-work-1"]["result_hash"],
        submit["outcome"]["work_result"]["result_hash"]
    );
}

#[tokio::test]
#[ignore = "bounded 100-cycle Runtime v3 soak; run explicitly for #5175 evidence"]
async fn bounded_runtime_v3_guardian_soak() {
    let directory = tempfile::tempdir().unwrap();
    let capsule = directory.path().join("continuity.json");
    let cycles = 100_u64;
    for generation in 1..=cycles {
        let (_, state) = run_proof(&capsule, 16).await.unwrap();
        assert_eq!(state.generation, generation);
        assert_eq!(state.processed_sequences.len(), 16);
    }

    let (sender, _receiver) = channel(1, ChannelFullPolicy::Reject);
    sender.send(1_u8).await.unwrap();
    assert_eq!(sender.send(2_u8).await.unwrap_err(), SendError::Full);
    assert_eq!(sender.metrics().sent(), 1);
    assert_eq!(sender.metrics().rejected(), 1);
    assert_eq!(sender.metrics().depth(), 1);

    let builds = Arc::new(AtomicU32::new(0));
    let mut restart_registry = ComponentRegistry::new();
    restart_registry.register(FailOnceFactory {
        builds: builds.clone(),
    });
    let restart_handle = Kernel::new(
        restart_registry.validate().unwrap(),
        RuntimeRecorder::new(16),
    )
    .start()
    .await
    .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        while builds.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        restart_handle
            .shutdown(Duration::from_secs(1))
            .await
            .unwrap(),
        KernelExit::Clean
    );

    let corrupt = directory.path().join("corrupt.json");
    let corrupt_bytes = b"corrupt continuity";
    std::fs::write(&corrupt, corrupt_bytes).unwrap();
    let proof = build_proof_runtime(&corrupt, 1).unwrap();
    assert!(matches!(
        proof.recorder.snapshot().clock,
        adl_runtime_kernel::ClockAuthority::Degraded { .. }
    ));
    let handle = proof.kernel.start().await.unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(matches!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::ShutdownFailed { .. }
    ));
    assert_eq!(std::fs::read(&corrupt).unwrap(), corrupt_bytes);

    let mut registry = ComponentRegistry::new();
    registry.register(StuckFactory);
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    assert!(matches!(
        handle.shutdown(Duration::from_millis(10)).await.unwrap(),
        KernelExit::ShutdownDeadlineExceeded { .. }
    ));

    let binary = env!("CARGO_BIN_EXE_adl-runtime-kernel");
    let child_capsule = directory.path().join("child.json");
    let first = std::process::Command::new(binary)
        .arg("fatal-once")
        .arg(&child_capsule)
        .output()
        .unwrap();
    assert_eq!(first.status.code(), Some(70));
    let second = std::process::Command::new(binary)
        .arg("fatal-once")
        .arg(&child_capsule)
        .status()
        .unwrap();
    assert!(second.success());

    if let Ok(path) = std::env::var("ADL_RUNTIME_V3_SOAK_REPORT") {
        let path = std::path::PathBuf::from(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let report = serde_json::json!({
            "schema": "adl.runtime_v3.guardian_soak_execution.v1",
            "cycles": cycles,
            "items_per_cycle": 16,
            "processed_items": cycles * 16,
            "continuity_generation": cycles,
            "result": "pass",
            "faults": {
                "component_failure": "restart succeeded after injected post-readiness failure",
                "child_crash_restart": "classified fatal exit followed by continuity generation recovery",
                "queue_saturation": "bounded reject policy and counters observed",
                "corrupt_continuity": "failed closed without replacing corrupt bytes",
                "clock_degradation": "observed_before_authority_promotion",
                "shutdown_deadline": "non-cooperative component forcibly aborted"
            },
            "automatic_cutover": false
        });
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
}

#[derive(Clone)]
struct FailOnceFactory {
    builds: Arc<AtomicU32>,
}

impl ComponentFactory for FailOnceFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("fail-once-soak-component"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::restart(1, Duration::from_millis(1)),
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(FailOnce {
            attempt: self.builds.fetch_add(1, Ordering::SeqCst),
        })
    }
}

struct FailOnce {
    attempt: u32,
}

#[async_trait]
impl Component for FailOnce {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        if self.attempt == 0 {
            tokio::task::yield_now().await;
            return Err(ComponentError::new("injected component failure"));
        }
        context.cancellation.cancelled().await;
        Ok(())
    }
}

struct StuckFactory;

impl ComponentFactory for StuckFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("stuck-soak-component"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(Stuck)
    }
}

struct Stuck;

#[async_trait]
impl Component for Stuck {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        pending::<()>().await;
        Ok(())
    }
}
