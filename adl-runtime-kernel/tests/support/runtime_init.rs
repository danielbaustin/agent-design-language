use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn write_for_state(
    directory: &Path,
    address: std::net::SocketAddr,
    state_root: &Path,
) -> PathBuf {
    write_with_certificate_for_state(directory, address, state_root).0
}

pub fn write_with_certificate_for_state(
    directory: &Path,
    address: std::net::SocketAddr,
    state_root: &Path,
) -> (PathBuf, Vec<u8>) {
    use rcgen::{
        date_time_ymd, BasicConstraints, CertificateParams, CertifiedIssuer, IsCa, KeyPair,
    };

    let mut ca_params = CertificateParams::new(["adl-runtime-v3-test-ca".to_owned()]).unwrap();
    ca_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
    ca_params.not_before = date_time_ymd(2026, 1, 1);
    ca_params.not_after = date_time_ymd(2036, 1, 1);
    let ca_key = KeyPair::generate().unwrap();
    let ca = CertifiedIssuer::self_signed(ca_params, ca_key).unwrap();
    let leaf_key = KeyPair::generate().unwrap();
    let mut leaf_params = CertificateParams::new([
        "localhost".to_owned(),
        "127.0.0.1".to_owned(),
        "::1".to_owned(),
    ])
    .unwrap();
    leaf_params.not_before = date_time_ymd(2026, 1, 1);
    leaf_params.not_after = date_time_ymd(2036, 1, 1);
    let leaf = leaf_params.signed_by(&leaf_key, &ca).unwrap();
    let tls_root = state_root.join("tls");
    std::fs::create_dir_all(&tls_root).unwrap();
    let certificate = tls_root.join("localhost-cert.pem");
    let private_key = tls_root.join("localhost-key.pem");
    std::fs::write(&certificate, leaf.pem()).unwrap();
    std::fs::write(&private_key, leaf_key.serialize_pem()).unwrap();
    let credentials_root = state_root.join("credentials");
    std::fs::create_dir_all(&credentials_root).unwrap();
    let control_public_key = credentials_root.join("control-public-key.hex");
    let operation_public_key = credentials_root.join("operation-public-key.hex");
    let continuity_signing_key = credentials_root.join("continuity-signing-key.hex");
    let observatory_token = credentials_root.join("observatory-token.txt");
    std::fs::write(
        &control_public_key,
        hex::encode(
            ed25519_dalek::SigningKey::from_bytes(&[17_u8; 32])
                .verifying_key()
                .as_bytes(),
        ),
    )
    .unwrap();
    std::fs::write(
        &operation_public_key,
        hex::encode(
            ed25519_dalek::SigningKey::from_bytes(&[29_u8; 32])
                .verifying_key()
                .as_bytes(),
        ),
    )
    .unwrap();
    std::fs::write(&continuity_signing_key, hex::encode([23_u8; 32])).unwrap();
    std::fs::write(&observatory_token, "guardian-observatory-token-00000001").unwrap();
    let vector = repo_vector_binary();
    let kernel = std::env::current_exe().unwrap();
    let init = directory.join("runtime-init.toml");
    std::fs::write(
        &init,
        format!(
            r#"schema = "adl.runtime_v3.init.v1"
state_root = "{}"
[binaries]
kernel_path = "{}"
[paths]
continuity_dir = "continuity"
tls_dir = "tls"
credentials_dir = "credentials"
observability_dir = "observability"
[kernel]
recorder_capacity = 32
control_history_capacity = 64
checkpoint_channel_capacity = 4
component_readiness_timeout_millis = 5000
observability_poll_millis = 50
weather_stale_after_millis = 75
guardian_lease_connect_millis = 500
guardian_lease_auth_millis = 500
trusted_time_sample_timeout_millis = 3000
trusted_time_max_offset_millis = 5000
trusted_time_max_round_trip_millis = 2000
trusted_time_retry_millis = 1000
trusted_time_refresh_millis = 60000
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
[credentials]
control_public_key_path = "{}"
control_key_id = "operator"
control_principal = "operator"
operation_public_key_path = "{}"
operation_key_id = "runtime-operations"
continuity_signing_key_path = "{}"
continuity_key_id = "runtime-continuity"
observatory_token_path = "{}"
continuity_min_generation = 0
sntp_server = "time.cloudflare.com"
[shutdown]
checkpoint_deadline_millis = 5000
kernel_grace_millis = 10000
api_drain_millis = 3000
guardian_margin_millis = 500
[guardian]
restart_budget = 3
backoff_base_millis = 100
backoff_cap_millis = 5000
healthy_window_millis = 60000
lease_auth_timeout_millis = 5000
lease_auth_attempts = 3
capture_max_bytes = 65536
capture_drain_grace_millis = 2000
configuration_exit_codes = [64]
[qualification]
readiness_timeout_millis = 10000
readiness_poll_millis = 10
shutdown_wait_millis = 50000
[observatory]
allowed_origins = ["https://localhost:8765"]
[observability_pipeline]
vector_binary_path = "{}"
service_name = "adl-runtime-v3"
revision = "test-revision"
guardian_id = "guardian-process-0"
lifecycle_suite = "runtime"
lifecycle_run = "runtime-run"
lifecycle_cycle = "runtime-cycle"
trace_filter = "adl_runtime_kernel=info,adl_runtime=info"
otlp_timeout_millis = 5000
vector_startup_attempts = 3
vector_startup_backoff_millis = 100
vector_shutdown_limit_millis = 3000
drain_timeout_millis = 5000
vector_config_path = "config/runtime-v3-vector.json"
ingress_spool_path = "spool/runtime-v3.current.jsonl"
master_log_path = "durable/master.log.jsonl"
audit_path = "durable/master-log-audit.json"
sequence_checkpoint_path = "durable/sequence.json"
vector_data_dir = "vector-data"
spool_max_bytes = 8388608
spool_retained_files = 4
[weather]
sample_millis = 25
history_capacity = 60
disk_warning_free_bytes = 5368709120
disk_stop_free_bytes = 2147483648
disk_recover_free_bytes = 8589934592
memory_warning_used_basis_points = 8500
memory_stop_used_basis_points = 9500
memory_recover_used_basis_points = 7500
cpu_warning_basis_points = 9000
cpu_stop_basis_points = 9800
cpu_recover_basis_points = 8000
checkpoint_deadline_millis = 750
snapshot_concurrency = 4
"#,
            toml_path(state_root),
            toml_path(&kernel),
            address,
            address.port(),
            toml_path(&certificate),
            toml_path(&private_key),
            toml_path(&control_public_key),
            toml_path(&operation_public_key),
            toml_path(&continuity_signing_key),
            toml_path(&observatory_token),
            toml_path(&vector),
        ),
    )
    .unwrap();
    (init, ca.der().to_vec())
}

pub fn toml_path(path: &Path) -> String {
    let value = path.to_string_lossy();
    assert!(!value.contains(['\n', '\r']));
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn repo_vector_binary() -> PathBuf {
    if let Ok(path) = std::env::var("ADL_RUNTIME_TEST_VECTOR_BINARY") {
        return PathBuf::from(path);
    }
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let worktree_vector = repo_root.join(".adl/bin/vector");
    if worktree_vector.is_file() {
        return worktree_vector;
    }
    let output = std::process::Command::new("git")
        .current_dir(repo_root)
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .output();
    if let Ok(output) = output {
        if output.status.success() {
            let git_common = String::from_utf8(output.stdout).unwrap();
            let primary_vector = PathBuf::from(git_common.trim())
                .parent()
                .unwrap()
                .join(".adl/bin/vector");
            if primary_vector.is_file() {
                return primary_vector;
            }
        }
    }
    worktree_vector
}
