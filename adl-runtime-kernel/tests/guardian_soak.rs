use std::{
    future::pending,
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
    ComponentRegistry, ComponentSpec, FailurePolicy, Kernel, KernelExit, RuntimeRecorder,
    SendError,
};
use async_trait::async_trait;

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
    assert!(horust.contains("successful-exit-code = [0, 78]"));
    assert!(horust.contains("signal = \"TERM\""));
    assert!(horust.contains(" serve "));
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
}

#[cfg(unix)]
#[test]
#[ignore = "requires ADL_HORUST_BIN and exercises native process supervision"]
fn horust_restarts_once_and_restores_continuity() {
    let directory = tempfile::tempdir().unwrap();
    let capsule = directory.path().join("horust-restart.json");
    let output = Command::new(horust_binary())
        .arg("--services-path")
        .arg(repo_path("infra/horust/adl-runtime-kernel-bakeoff.toml"))
        .arg("--uds-folder-path")
        .arg(directory.path().join("uds"))
        .env("ADL_RUNTIME_BIN", env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .env("ADL_RUNTIME_CAPSULE", &capsule)
        .output()
        .unwrap();
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
    let output = Command::new(horust_binary())
        .arg("--services-path")
        .arg(repo_path("infra/horust/adl-runtime-kernel.toml"))
        .arg("--uds-folder-path")
        .arg(directory.path().join("uds"))
        .env("ADL_RUNTIME_BIN", env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .env("ADL_RUNTIME_CAPSULE", directory.path().join("config.json"))
        .output()
        .unwrap();
    assert!(output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        combined
            .matches("runtime control key is missing or invalid")
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

    let mut guardian = Command::new(horust_binary())
        .arg("--services-path")
        .arg(&service)
        .arg("--uds-folder-path")
        .arg(directory.path().join("uds"))
        .env("ADL_ATTEMPT_LOG", &attempt_log)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
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
        if let Some(status) = guardian.try_wait().unwrap() {
            panic!("Horust unexpectedly exhausted the configured budget: {status}");
        }
        assert!(
            std::time::Instant::now() < deadline,
            "Horust did not reproduce the restart-budget defect before the deadline; observed {observed_attempts} launches"
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(
        unsafe { libc::kill(guardian.id() as i32, libc::SIGTERM) },
        0
    );
    assert!(guardian.wait().unwrap().success());
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

    let output = Command::new(horust_binary())
        .arg("--services-path")
        .arg(&service)
        .arg("--uds-folder-path")
        .arg(directory.path().join("uds"))
        .env("ADL_ENV_OUT", &environment_log)
        .env("ADL_ALLOWED_TEST", "visible")
        .env("OPENAI_API_KEY", "must-not-leak")
        .env("AWS_SECRET_ACCESS_KEY", "must-not-leak")
        .output()
        .unwrap();
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
#[test]
#[ignore = "requires ADL_HORUST_BIN and binds Runtime v3 control port 20997"]
fn horust_forwards_sigterm_and_runtime_checkpoints_cleanly() {
    use ed25519_dalek::SigningKey;

    let directory = tempfile::tempdir().unwrap();
    let capsule = directory.path().join("horust-sigterm.json");
    let verifying_key = SigningKey::from_bytes(&[19_u8; 32]).verifying_key();
    let mut guardian = Command::new(horust_binary())
        .arg("--services-path")
        .arg(repo_path("infra/horust/adl-runtime-kernel.toml"))
        .arg("--uds-folder-path")
        .arg(directory.path().join("uds"))
        .env("ADL_RUNTIME_BIN", env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .env("ADL_RUNTIME_CAPSULE", &capsule)
        .env(
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
            hex::encode(verifying_key.as_bytes()),
        )
        .env("ADL_RUNTIME_CONTROL_KEY_ID", "guardian-test")
        .env("ADL_RUNTIME_CONTROL_PRINCIPAL", "guardian-test")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    wait_for_control_port(&mut guardian);
    assert_eq!(
        unsafe { libc::kill(guardian.id() as i32, libc::SIGTERM) },
        0
    );
    let status = guardian.wait().unwrap();
    assert!(status.success(), "Horust exited with {status}");

    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    while std::net::TcpStream::connect("127.0.0.1:20997").is_ok() {
        assert!(
            std::time::Instant::now() < deadline,
            "Runtime v3 remained live after guardian termination"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let continuity: serde_json::Value =
        serde_json::from_slice(&std::fs::read(capsule).unwrap()).unwrap();
    assert_eq!(continuity["schema"], "adl.runtime_kernel.continuity.v1");
    assert_eq!(continuity["generation"], 1);
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

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(relative)
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
fn wait_for_control_port(guardian: &mut std::process::Child) {
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        if std::net::TcpStream::connect("127.0.0.1:20997").is_ok() {
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
#[ignore = "binds the fixed Runtime v3 control port 20997"]
fn serve_handles_guardian_sigterm_with_a_clean_checkpointed_exit() {
    use ed25519_dalek::SigningKey;

    let directory = tempfile::tempdir().unwrap();
    let capsule = directory.path().join("sigterm-continuity.json");
    let verifying_key = SigningKey::from_bytes(&[17_u8; 32]).verifying_key();
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"))
        .arg("serve")
        .arg(&capsule)
        .env(
            "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX",
            hex::encode(verifying_key.as_bytes()),
        )
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    while std::net::TcpStream::connect("127.0.0.1:20997").is_err() {
        assert!(
            std::time::Instant::now() < deadline,
            "serve did not become ready"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    assert_eq!(unsafe { libc::kill(child.id() as i32, libc::SIGTERM) }, 0);
    let status = child.wait().unwrap();
    assert!(status.success());
    let continuity: serde_json::Value =
        serde_json::from_slice(&std::fs::read(capsule).unwrap()).unwrap();
    assert_eq!(continuity["schema"], "adl.runtime_kernel.continuity.v1");
    assert_eq!(continuity["generation"], 1);
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
