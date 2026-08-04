use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use serde_json::json;

use super::csm_cmd::real_csm_standalone;
use super::process_cmd::real_process;
use adl::long_lived_agent::load_spec;
use adl_runtime::runtime_api_auth::RuntimeApiCredentialStore;

pub(crate) fn real_csmctl(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("runtime") => real_runtime(&args[1..]),
        Some("status") => real_status(&args[1..]),
        Some("diagnostics") => real_diagnostics(&args[1..]),
        Some("api") => real_api(&args[1..]),
        Some("cloud") => real_cloud(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl module '{other}'. Expected runtime, api, status, diagnostics, cloud, help, or --version.\n\n{}",
            csmctl_usage()
        )),
    }
}

pub(crate) fn csmctl_usage() -> &'static str {
    "csmctl - CSM runtime administration control plane\n\n\
Usage:\n\
  csmctl runtime service <install|start|status|stop|remove> ...\n\
  csmctl runtime governed-stop --spec <agent-spec.yaml> --reason <text> ...\n\
  csmctl runtime continuity <capture|stage|restore|drill> ...\n\
  csmctl runtime backpressure prove ...\n\
  csmctl runtime storage prove-s3 ...\n\
  csmctl runtime observatory --packet <visibility-packet.json> ...\n\
  csmctl api get --spec <agent-spec.yaml> [--path /status] [--bind 127.0.0.1:19997]\n\
  csmctl api credential <status|rotate|revoke> --spec <agent-spec.yaml>\n\
  csmctl status [--pid <pid>|--pid-file <path>|--port <port> [--host 127.0.0.1]] [--json]\n\
  csmctl diagnostics process status [--pid <pid>|--pid-file <path>|--port <port>] [--json]\n\
  csmctl cloud aws-signal acip-sns-proof ...\n\
  csmctl cloud cloud-control cloudfront-status ...\n\
  csmctl --help\n\
  csmctl --version\n\n\
Modules:\n\
  runtime      Administer the CSM service, governed stop, embedded API bind, continuity, backpressure, storage, and observatory surfaces.\n\
  api          Authenticated client and credential lifecycle for the embedded runtime API.\n\
  status       Permission-safe liveness checks for CSM process metadata or loopback ports.\n\
  diagnostics  Explicit diagnostic wrappers around permission-safe process probes.\n\
  cloud        Governed runtime cloud-control and signal proof surfaces.\n\n\
Boundaries:\n\
  - csm is the runtime owner and executes the permanent daemon loop.\n\
  - csmctl is the operator/admin control plane for that runtime.\n\
  - adl remains ADL language authoring, compilation, validation, and runtime workflow tooling.\n\
  - C-SDLC issue execution resolves through csdlc-install and the independent typed v2 binaries."
}

fn real_api(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("get") => csmctl_api_get(&args[1..]),
        Some("credential") => csmctl_api_credential(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_api_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl api command '{other}'. Expected get, credential, or help.\n\n{}",
            csmctl_api_usage()
        )),
    }
}

fn csmctl_api_usage() -> &'static str {
    "csmctl api - authenticated CSM runtime API control plane\n\n\
Usage:\n\
  csmctl api get --spec <agent-spec.yaml> [--path /status] [--bind 127.0.0.1:19997]\n\
  csmctl api credential status --spec <agent-spec.yaml>\n\
  csmctl api credential rotate --spec <agent-spec.yaml>\n\
  csmctl api credential revoke --spec <agent-spec.yaml>\n\n\
Notes:\n\
  Credentials are read from the runtime state root, sent only in the Authorization header, and never printed.\n\
  Rotation and revocation are observed by the running CSM API without a restart."
}

fn csmctl_api_get(args: &[String]) -> Result<()> {
    const CONNECT_RETRY_ATTEMPTS: usize = 200;
    const CONNECT_RETRY_DELAY: Duration = Duration::from_millis(25);
    const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

    let spec = required_path_arg(args, "--spec")?;
    let bind = optional_arg(args, "--bind").unwrap_or("127.0.0.1:19997");
    let path = optional_arg(args, "--path").unwrap_or("/status");
    if !path.starts_with('/') || path.contains(['\r', '\n']) {
        return Err(anyhow!("csmctl api --path must be an absolute HTTP path"));
    }
    let addr: SocketAddr = bind
        .parse()
        .with_context(|| format!("parse csmctl API bind {bind}"))?;
    if !addr.ip().is_loopback() {
        return Err(anyhow!(
            "csmctl refuses to send the runtime API credential to a non-loopback address"
        ));
    }
    let loaded = load_spec(&spec).context("load CSM spec for authenticated API client")?;
    let store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
    let url = format!("http://{bind}{path}");
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(REQUEST_TIMEOUT)
        .build()
        .context("build csmctl runtime API client")?;
    let response = store
        .with_bearer_token(|token| {
            for attempt in 0..CONNECT_RETRY_ATTEMPTS {
                match client.get(&url).bearer_auth(token).send() {
                    Ok(response) => return Ok(response),
                    Err(err) if err.is_connect() && attempt + 1 < CONNECT_RETRY_ATTEMPTS => {
                        std::thread::sleep(CONNECT_RETRY_DELAY);
                    }
                    Err(err) => return Err(err),
                }
            }
            unreachable!("bounded runtime API connection retry loop always returns")
        })
        .map_err(anyhow::Error::msg)?
        .context("call authenticated CSM runtime API")?;
    let status = response.status();
    let body = response.text().context("read CSM runtime API response")?;
    if !status.is_success() {
        return Err(anyhow!("CSM runtime API returned HTTP {status}: {body}"));
    }
    println!("{body}");
    Ok(())
}

fn csmctl_api_credential(args: &[String]) -> Result<()> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or_else(|| anyhow!("csmctl api credential requires status, rotate, or revoke"))?;
    let spec = required_path_arg(&args[1..], "--spec")?;
    let loaded = load_spec(&spec).context("load CSM spec for credential administration")?;
    let store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
    let metadata = match action {
        "status" => {
            let metadata = store.metadata().map_err(anyhow::Error::msg)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "schema": "adl.csmctl.runtime_api_credential.v1",
                    "action": action,
                    "status": if metadata.is_some() { "present" } else { "missing" },
                    "credential": metadata,
                    "secret_printed": false
                }))?
            );
            return Ok(());
        }
        "rotate" => store.rotate(),
        "revoke" => store.revoke(),
        other => return Err(anyhow!("unknown credential action '{other}'")),
    }
    .map_err(anyhow::Error::msg)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema": "adl.csmctl.runtime_api_credential.v1",
            "action": action,
            "status": "completed",
            "credential": metadata,
            "secret_printed": false
        }))?
    );
    Ok(())
}

fn required_path_arg(args: &[String], flag: &str) -> Result<PathBuf> {
    optional_arg(args, flag)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("missing required {flag} <path>"))
}

fn optional_arg<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].as_str())
}

fn real_runtime(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("service") => {
            let mapped = runtime_service_args(args)?;
            delegate_to_csm(&mapped)
        }
        Some("governed-stop") | Some("continuity") | Some("backpressure") | Some("storage")
        | Some("observatory") => delegate_to_csm(args),
        Some("daemon") => Err(anyhow!(
            "csmctl does not execute the runtime daemon loop. Use `csm daemon ...` for direct runtime execution or `csmctl runtime service ...` for administered service control."
        )),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_runtime_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl runtime command '{other}'. Expected service, governed-stop, continuity, backpressure, storage, observatory, help, or --version.\n\n{}",
            csmctl_runtime_usage()
        )),
    }
}

fn csmctl_runtime_usage() -> &'static str {
    "csmctl runtime - administer CSM runtime-owned local surfaces\n\n\
Usage:\n\
  csmctl runtime service <install|start|status|stop|remove> ...\n\
  csmctl runtime governed-stop --spec <agent-spec.yaml> --reason <text> ...\n\
  csmctl runtime continuity <capture|stage|restore|drill> ...\n\
  csmctl runtime backpressure prove ...\n\
  csmctl runtime storage prove-s3 ...\n\
  csmctl runtime observatory --packet <visibility-packet.json> ...\n\n\
Notes:\n\
  csmctl runtime delegates administered operations to CSM-owned parsers so runtime semantics stay single-sourced.\n\
  The runtime API is embedded in csm daemon and administered through csmctl runtime service ... --api-bind.\n\
  Direct daemon-loop execution remains owned by `csm daemon`, not csmctl."
}

fn real_status(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", csmctl_status_usage());
        return Ok(());
    }
    let mut mapped = Vec::with_capacity(args.len() + 1);
    mapped.push("status".to_string());
    mapped.extend(args.iter().cloned());
    real_process(&mapped)
}

fn csmctl_status_usage() -> &'static str {
    "csmctl status - permission-safe CSM liveness check\n\n\
Usage:\n\
  csmctl status --pid <pid> [--json]\n\
  csmctl status --pid-file <path> [--json]\n\
  csmctl status --port <port> [--host 127.0.0.1|::1|localhost] [--json]\n\n\
Notes:\n\
  This is a thin control-plane alias for `adl process status` using exact metadata or exact loopback probes only."
}

fn real_diagnostics(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("process") => real_process(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_diagnostics_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl diagnostics command '{other}'. Expected process or help.\n\n{}",
            csmctl_diagnostics_usage()
        )),
    }
}

fn csmctl_diagnostics_usage() -> &'static str {
    "csmctl diagnostics - CSM runtime diagnostic probes\n\n\
Usage:\n\
  csmctl diagnostics process status [--pid <pid>|--pid-file <path>|--port <port>] [--json]\n\n\
Notes:\n\
  Diagnostics are intentionally permission-safe and do not use broad host process scans."
}

fn real_cloud(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("aws-signal") | Some("cloud-control") => delegate_to_csm(args),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_cloud_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl cloud command '{other}'. Expected aws-signal, cloud-control, or help.\n\n{}",
            csmctl_cloud_usage()
        )),
    }
}

fn csmctl_cloud_usage() -> &'static str {
    "csmctl cloud - governed CSM cloud-control surfaces\n\n\
Usage:\n\
  csmctl cloud aws-signal acip-sns-proof --out <proof-dir> ...\n\
  csmctl cloud cloud-control cloudfront-status --out <proof-dir> ...\n\n\
Notes:\n\
  Cloud operations use the same CSM runtime-owned parsers and Agent Logic AWS guardrails as `csm`."
}

fn delegate_to_csm(args: &[String]) -> Result<()> {
    real_csm_standalone(args)
}

fn runtime_service_args(args: &[String]) -> Result<Vec<String>> {
    if args.get(1).map(String::as_str) != Some("install") || has_flag(args, "--csm-bin") {
        return Ok(args.to_vec());
    }
    let mut mapped = args.to_vec();
    mapped.push("--csm-bin".to_string());
    mapped.push(default_csm_owner_binary()?.display().to_string());
    Ok(mapped)
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

fn default_csm_owner_binary() -> Result<PathBuf> {
    let current = std::env::current_exe().context("resolve current csmctl executable")?;
    let parent = current
        .parent()
        .ok_or_else(|| anyhow!("current executable has no parent directory"))?;
    Ok(parent.join(format!("csm{}", std::env::consts::EXE_SUFFIX)))
}

#[cfg(test)]
mod tests {
    use super::{
        csmctl_api_get, csmctl_api_usage, csmctl_cloud_usage, csmctl_diagnostics_usage,
        csmctl_runtime_usage, csmctl_status_usage, csmctl_usage, real_csmctl, runtime_service_args,
    };
    use adl::csm_runtime_api::{serve_runtime_api, CsmRuntimeApiOptions};
    use adl_runtime::runtime_api_auth::RuntimeApiCredentialStore;
    use serde_json::Value;
    use std::fs;
    use std::net::TcpListener;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn temp_root(prefix: &str) -> PathBuf {
        let seq = TEMP_SEQ.fetch_add(1, Ordering::SeqCst);
        let root =
            std::env::temp_dir().join(format!("adl-csmctl-{prefix}-{}-{seq}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp root");
        root
    }

    fn write_spec(root: &std::path::Path) -> PathBuf {
        let spec = root.join("agent.yaml");
        fs::write(
            &spec,
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: csmctl-service-agent
display_name: CSMCTL Service Agent
state_root: state
workflow:
  kind: demo_adapter
  name: csmctl_service_probe
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
"#,
        )
        .expect("write spec");
        spec
    }

    struct GovernedCsmTestPort {
        bind: String,
        lock_dir: PathBuf,
    }

    impl Drop for GovernedCsmTestPort {
        fn drop(&mut self) {
            let _ = fs::remove_dir(&self.lock_dir);
        }
    }

    fn reserve_governed_csm_test_port(label: &str) -> GovernedCsmTestPort {
        let start = ((std::process::id() as u64)
            .wrapping_add(TEMP_SEQ.fetch_add(1, Ordering::SeqCst)))
            % 50;
        let lock_root = std::env::current_dir()
            .expect("resolve current test directory")
            .join(".adl")
            .join("test-port-locks")
            .join("csm");
        fs::create_dir_all(&lock_root).expect("create governed CSM test port lock root");
        for offset in 0..50 {
            let port = 19_950 + ((start + offset) % 50) as u16;
            let lock_dir = lock_root.join(format!("port-{port}.lock"));
            if fs::create_dir(&lock_dir).is_err() {
                continue;
            }
            match TcpListener::bind(("127.0.0.1", port)) {
                Ok(listener) => {
                    let bind = listener.local_addr().expect("read governed CSM test port");
                    drop(listener);
                    return GovernedCsmTestPort {
                        bind: bind.to_string(),
                        lock_dir,
                    };
                }
                Err(_) => {
                    let _ = fs::remove_dir(&lock_dir);
                }
            }
        }
        panic!("could not bind one governed CSM test port for {label} in 19950-19999");
    }

    fn assert_err_contains(result: anyhow::Result<()>, needle: &str) {
        let err = result.expect_err("expected error");
        assert!(
            err.to_string().contains(needle),
            "expected {needle:?} in {err}"
        );
    }

    #[test]
    fn csmctl_usage_documents_modular_runtime_control_plane() {
        let usage = csmctl_usage();
        assert!(usage.contains("csmctl runtime service"));
        assert!(usage.contains("csmctl api get"));
        assert!(usage.contains("csmctl status"));
        assert!(usage.contains("csmctl diagnostics process status"));
        assert!(usage.contains("csmctl cloud aws-signal"));
        assert!(usage.contains("csm is the runtime owner"));
        assert!(usage.contains("adl remains ADL language"));
        assert!(!usage.contains("adl compile"));
        assert!(usage.contains("csdlc-install"));
        assert!(usage.contains("independent typed v2 binaries"));
    }

    #[test]
    fn csmctl_authenticated_api_client_uses_runtime_owned_credential() {
        let root = temp_root("api-client");
        let spec = write_spec(&root);
        let port = reserve_governed_csm_test_port("api-client");
        let bind = port.bind.clone();
        let server_spec = spec.clone();
        let server_bind = bind.clone();
        let server = std::thread::spawn(move || {
            serve_runtime_api(CsmRuntimeApiOptions {
                spec_path: server_spec,
                bind: server_bind,
                test_max_requests: Some(1),
                idle_timeout_ms: Some(5_000),
                shutdown_file: None,
                otel_status_path: None,
                otel_log_path: None,
            })
        });
        let loaded = adl::long_lived_agent::load_spec(&spec).unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
        for _ in 0..100 {
            if store.path().exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        csmctl_api_get(&[
            "--spec".to_string(),
            spec.display().to_string(),
            "--bind".to_string(),
            bind,
            "--path".to_string(),
            "/status".to_string(),
        ])
        .expect("csmctl authenticated API request");
        let result = server.join().unwrap().unwrap();
        assert_eq!(result.served_requests, 1);
    }

    #[test]
    fn csmctl_authenticated_api_client_waits_for_slow_listener_startup() {
        let root = temp_root("api-client-slow-listener");
        let spec = write_spec(&root);
        let port = reserve_governed_csm_test_port("api-client-slow-listener");
        let bind = port.bind.clone();
        let loaded = adl::long_lived_agent::load_spec(&spec).unwrap();
        RuntimeApiCredentialStore::for_state_root(&loaded.state_root)
            .ensure()
            .expect("pre-create runtime API credential");

        let client_spec = spec.clone();
        let client_bind = bind.clone();
        let client = std::thread::spawn(move || {
            csmctl_api_get(&[
                "--spec".to_string(),
                client_spec.display().to_string(),
                "--bind".to_string(),
                client_bind,
                "--path".to_string(),
                "/status".to_string(),
            ])
        });

        std::thread::sleep(std::time::Duration::from_millis(750));
        let result = serve_runtime_api(CsmRuntimeApiOptions {
            spec_path: spec,
            bind,
            test_max_requests: Some(1),
            idle_timeout_ms: Some(5_000),
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        })
        .expect("serve delayed runtime API request");

        client
            .join()
            .expect("join delayed csmctl API client")
            .expect("csmctl API client waits for listener startup");
        assert_eq!(result.served_requests, 1);
    }

    #[test]
    fn csmctl_api_refuses_to_send_credentials_off_loopback() {
        let root = temp_root("api-non-loopback");
        let spec = write_spec(&root);
        let err = csmctl_api_get(&[
            "--spec".to_string(),
            spec.display().to_string(),
            "--bind".to_string(),
            "192.0.2.1:19997".to_string(),
        ])
        .unwrap_err();
        assert!(err.to_string().contains("non-loopback"));
        assert!(csmctl_api_usage().contains("never printed"));
    }

    #[test]
    fn csmctl_help_and_unknown_module_paths_are_bounded_to_admin_surface() {
        assert!(real_csmctl(&args(&[])).is_ok());
        assert!(real_csmctl(&args(&["help"])).is_ok());
        assert!(real_csmctl(&args(&["--help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["compile"])),
            "unknown csmctl module 'compile'",
        );
    }

    #[test]
    fn csmctl_module_usage_surfaces_document_owned_boundaries() {
        assert!(csmctl_runtime_usage().contains("Direct daemon-loop execution"));
        assert!(csmctl_status_usage().contains("exact metadata or exact loopback probes"));
        assert!(csmctl_diagnostics_usage().contains("permission-safe"));
        assert!(csmctl_cloud_usage().contains("Agent Logic AWS guardrails"));
    }

    #[test]
    fn csmctl_rejects_direct_daemon_execution() {
        assert_err_contains(
            real_csmctl(&args(&["runtime", "daemon", "--help"])),
            "does not execute the runtime daemon loop",
        );
    }

    #[test]
    fn csmctl_rejects_removed_standalone_runtime_api_route() {
        assert_err_contains(
            real_csmctl(&args(&["runtime", "api", "--help"])),
            "unknown csmctl runtime command 'api'",
        );
    }

    #[test]
    fn csmctl_runtime_help_and_unknown_paths_stay_runtime_scoped() {
        assert!(real_csmctl(&args(&["runtime"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "--help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["runtime", "compile"])),
            "unknown csmctl runtime command 'compile'",
        );
    }

    #[test]
    fn csmctl_runtime_help_paths_delegate_to_csm_owned_parsers() {
        assert!(real_csmctl(&args(&["runtime", "service", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "governed-stop", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "continuity", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "backpressure", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "storage", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "observatory", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["cloud", "aws-signal", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["cloud", "cloud-control", "--help"])).is_ok());
    }

    #[test]
    fn csmctl_status_and_diagnostics_help_paths_are_permission_safe() {
        assert!(real_csmctl(&args(&["status", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["diagnostics"])).is_ok());
        assert!(real_csmctl(&args(&["diagnostics", "help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["diagnostics", "scan"])),
            "unknown csmctl diagnostics command 'scan'",
        );
    }

    #[test]
    fn csmctl_status_requires_exact_target() {
        assert_err_contains(
            real_csmctl(&args(&["status", "--json"])),
            "requires exactly one of --pid, --pid-file, --port, or --name",
        );
    }

    #[test]
    fn csmctl_cloud_help_and_unknown_paths_stay_cloud_scoped() {
        assert!(real_csmctl(&args(&["cloud"])).is_ok());
        assert!(real_csmctl(&args(&["cloud", "--help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["cloud", "billing"])),
            "unknown csmctl cloud command 'billing'",
        );
    }

    #[test]
    fn csmctl_service_args_only_default_install_without_explicit_csm_bin() {
        let explicit = args(&["service", "install", "--csm-bin", "/tmp/csm"]);
        assert_eq!(
            runtime_service_args(&explicit).expect("explicit csm-bin args"),
            explicit
        );

        let status = args(&["service", "status"]);
        assert_eq!(
            runtime_service_args(&status).expect("service status args"),
            status
        );

        let install = args(&["service", "install"]);
        let mapped = runtime_service_args(&install).expect("default install args");
        assert_eq!(&mapped[..2], &install[..]);
        assert_eq!(mapped[mapped.len() - 2], "--csm-bin");
        assert!(
            mapped
                .last()
                .expect("default csm binary")
                .ends_with(&format!("csm{}", std::env::consts::EXE_SUFFIX)),
            "install should default to csm owner binary: {mapped:?}"
        );
    }

    #[test]
    fn csmctl_service_install_defaults_managed_binary_to_csm_owner() {
        let root = temp_root("service-default-csm");
        let spec = write_spec(&root);
        let service_root = root.join("service");
        let args = vec![
            "runtime".to_string(),
            "service".to_string(),
            "install".to_string(),
            "--spec".to_string(),
            spec.display().to_string(),
            "--service-root".to_string(),
            service_root.display().to_string(),
            "--manager".to_string(),
            "local".to_string(),
            "--json".to_string(),
        ];
        real_csmctl(&args).expect("install service through csmctl");

        let manifest: Value = serde_json::from_str(
            &fs::read_to_string(service_root.join("service_manifest.json"))
                .expect("read service manifest"),
        )
        .expect("parse service manifest");
        let csm_bin = manifest["csm_bin"].as_str().expect("manifest csm_bin");
        assert!(
            csm_bin.ends_with(&format!("csm{}", std::env::consts::EXE_SUFFIX)),
            "csmctl service install must configure csm daemon owner, got {csm_bin}"
        );
        assert!(
            !csm_bin.ends_with(&format!("csmctl{}", std::env::consts::EXE_SUFFIX)),
            "csmctl must not become the managed daemon executable"
        );
    }
}
