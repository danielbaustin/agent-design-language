use super::*;
use crate::model_identity::{ModelIdentityStrengthV1, ModelIdentityV1};
use crate::provider::is_retryable_error;
use crate::provider_substrate::{
    CapabilityModeV1, CapabilitySupportV1, ProviderCapabilitiesV1, ProviderInvocationTargetV1,
    ProviderTransportV1,
};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::net::TcpListener;
use std::process::Command;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use tiny_http::{Header, Response, Server};

macro_rules! bedrock_invocation_record {
    ($model:expr, $prompt:expr, $output:expr, $http_status:expr, $profile:expr, $region:expr, $account_id_sha256:expr, $account_profile_validation_status:expr $(,)?) => {
        BedrockInvocationRecord {
            model: $model,
            prompt: $prompt,
            output: $output,
            http_status: $http_status,
            profile: $profile,
            region: $region,
            account_id_sha256: $account_id_sha256,
            account_profile_validation_status: $account_profile_validation_status,
        }
    };
}

#[derive(Debug, Clone)]
struct CapturedRequest {
    url: String,
    headers: HashMap<String, String>,
    body: String,
}

type SpawnedJsonServer = (
    String,
    Arc<Mutex<Option<CapturedRequest>>>,
    thread::JoinHandle<()>,
);

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .expect("env lock")
}

fn reserve_local_port() -> Option<u16> {
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener,
        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => return None,
        Err(err) => panic!("bind ephemeral port: {err}"),
    };
    let port = listener.local_addr().expect("local addr").port();
    drop(listener);
    Some(port)
}

#[allow(clippy::type_complexity)]
fn spawn_json_server(status: u16, response_body: &'static str) -> Option<SpawnedJsonServer> {
    let port = reserve_local_port()?;
    let bind_addr = format!("127.0.0.1:{port}");
    let server = Server::http(&bind_addr).expect("bind tiny_http server");
    let captured = Arc::new(Mutex::new(None));
    let captured_for_thread = Arc::clone(&captured);
    let handle = thread::spawn(move || {
        if let Some(mut request) = server.incoming_requests().next() {
            let mut body = String::new();
            let _ = request.as_reader().read_to_string(&mut body);
            let headers = request
                .headers()
                .iter()
                .map(|header| (header.field.to_string(), header.value.as_str().to_string()))
                .collect::<HashMap<_, _>>();
            *captured_for_thread.lock().expect("capture lock") = Some(CapturedRequest {
                url: request.url().to_string(),
                headers,
                body,
            });

            let mut response =
                Response::from_string(response_body.to_string()).with_status_code(status);
            if let Ok(header) = Header::from_bytes("Content-Type", "application/json") {
                response = response.with_header(header);
            }
            let _ = request.respond(response);
        }
    });

    Some((format!("http://{bind_addr}"), captured, handle))
}

fn provider_caps() -> ProviderCapabilitiesV1 {
    ProviderCapabilitiesV1 {
        tool_calling: CapabilitySupportV1 {
            supported: true,
            mode: CapabilityModeV1::Native,
        },
        structured_json: CapabilitySupportV1 {
            supported: true,
            mode: CapabilityModeV1::Native,
        },
        semantic_tool_fallback: CapabilitySupportV1 {
            supported: false,
            mode: CapabilityModeV1::None,
        },
    }
}

fn provider_target(
    provider_kind: &str,
    endpoint: String,
    provider_model_id: &str,
) -> ProviderInvocationTargetV1 {
    ProviderInvocationTargetV1 {
        provider_id: format!("{provider_kind}_primary"),
        provider_kind: provider_kind.to_string(),
        vendor: provider_kind.to_string(),
        transport: ProviderTransportV1::Http,
        profile: None,
        endpoint: Some(endpoint.clone()),
        base_url: None,
        model_ref: provider_model_id.to_string(),
        provider_model_id: provider_model_id.to_string(),
        model_identity: ModelIdentityV1 {
            provider_kind: provider_kind.to_string(),
            provider: format!("{provider_kind}_primary"),
            model_ref: provider_model_id.to_string(),
            provider_model_id: provider_model_id.to_string(),
            runtime_surface: "hosted_http".to_string(),
            identity_strength: ModelIdentityStrengthV1::ProviderAsserted,
            observed_at: "unix:1".to_string(),
            resolved_digest: None,
            source_registry: Some(endpoint),
            runtime_fingerprint: None,
            inference_parameter_fingerprint: None,
            tool_surface: None,
            governance_surface: None,
            evaluator_ref: None,
            lane_ref: None,
            benchmark_ref: None,
        },
        capabilities: provider_caps(),
    }
}

fn ollama_provider_spec_with_base_url(base_url: &str) -> adl::ProviderSpec {
    adl::ProviderSpec {
        id: Some("ollama_primary".to_string()),
        profile: None,
        kind: "ollama".to_string(),
        base_url: Some(base_url.to_string()),
        default_model: Some("phi4-mini".to_string()),
        config: HashMap::new(),
    }
}

fn restore_env_var(key: &str, previous: Option<std::ffi::OsString>) {
    match previous {
        Some(value) => env::set_var(key, value),
        None => env::remove_var(key),
    }
}

#[test]
fn bedrock_request_and_response_shapes_cover_nova_messages() {
    let body = bedrock_nova_request_body("hello bedrock", 123);
    assert_eq!(body["schemaVersion"], "messages-v1");
    assert!(body.get("model").is_none());
    assert_eq!(body["messages"][0]["role"], "user");
    assert_eq!(body["messages"][0]["content"][0]["text"], "hello bedrock");
    assert_eq!(body["inferenceConfig"]["maxTokens"], 123);

    let output = extract_bedrock_nova_output_text(&json!({
        "output": {
            "message": {
                "role": "assistant",
                "content": [{"text": "bedrock ok"}]
            }
        }
    }));
    assert_eq!(output.as_deref(), Some("bedrock ok"));
}

#[test]
fn bedrock_provider_requires_agent_logic_profile_before_live_call() {
    let spec = adl::ProviderSpec {
        id: Some("bedrock_primary".to_string()),
        profile: None,
        kind: "bedrock".to_string(),
        base_url: None,
        default_model: Some("hosted:adl-bedrock:amazon.nova-lite-v1:0".to_string()),
        config: HashMap::from([
            (
                "provider_model_id".to_string(),
                json!("amazon.nova-lite-v1:0"),
            ),
            ("profile".to_string(), json!("default")),
        ]),
    };
    let target = provider_target(
        "bedrock",
        "aws-bedrock-runtime".to_string(),
        "amazon.nova-lite-v1:0",
    );
    let err = AwsBedrockProvider::from_target(&spec, &target)
        .expect_err("wrong profile should fail before AWS calls");
    assert!(err.to_string().contains("agent-logic-admin"));
}

#[test]
fn bedrock_error_sanitizer_removes_signed_aws_values() {
    let sanitized = sanitize_bedrock_error(
        "Authorization: AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20260707/us-west-2/bedrock/aws4_request, SignedHeaders=host, X-Amz-Signature=abc123def456",
    );

    assert!(sanitized.contains("Authorization: <redacted>"));
    assert!(sanitized.contains("Credential=<redacted>"));
    assert!(sanitized.contains("X-Amz-Signature=<redacted>"));
    assert!(!sanitized.contains("AWS4-HMAC-SHA256"));
    assert!(!sanitized.contains("AKIAIOSFODNN7EXAMPLE"));
    assert!(!sanitized.contains("abc123def456"));
}

#[test]
fn bedrock_error_sanitizer_removes_arns_and_account_ids() {
    let sanitized = sanitize_bedrock_error(
        "AccessDeniedException: User arn:aws:iam::123456789012:role/adl-prod denied bedrock:InvokeModel for account 123456789012 by resource policy",
    );

    assert!(sanitized.contains("AccessDeniedException"));
    assert!(sanitized.contains("denied bedrock:InvokeModel"));
    assert!(sanitized.contains("resource policy"));
    assert!(sanitized.contains("<redacted-aws-arn>"));
    assert!(sanitized.contains("<redacted-aws-account-id>"));
    assert!(!sanitized.contains("arn:aws:"));
    assert!(!sanitized.contains("123456789012"));
    assert!(!sanitized.contains("adl-prod"));
}

#[test]
fn bedrock_error_sanitizer_removes_partition_arns() {
    let sanitized = sanitize_bedrock_error(
        "AccessDeniedException: principals arn:aws-us-gov:iam::123456789012:role/gov-role, arn:aws-cn:iam::210987654321:role/cn-role, and arn:aws-iso:iam::111122223333:role/iso-role denied bedrock:InvokeModel",
    );

    assert!(sanitized.contains("AccessDeniedException"));
    assert!(sanitized.contains("principals"));
    assert!(sanitized.contains("denied bedrock:InvokeModel"));
    assert!(sanitized.contains("<redacted-aws-arn>"));
    assert!(!sanitized.contains("arn:aws-us-gov:"));
    assert!(!sanitized.contains("arn:aws-cn:"));
    assert!(!sanitized.contains("arn:aws-iso:"));
    assert!(!sanitized.contains("123456789012"));
    assert!(!sanitized.contains("210987654321"));
    assert!(!sanitized.contains("111122223333"));
    assert!(!sanitized.contains("gov-role"));
    assert!(!sanitized.contains("cn-role"));
    assert!(!sanitized.contains("iso-role"));
}

#[test]
fn bedrock_invocation_artifact_records_profile_region_and_account_hash() {
    let _guard = env_lock();
    let artifact = std::env::temp_dir().join(format!(
        "adl-bedrock-provider-invocations-{}-{}.json",
        std::process::id(),
        "record"
    ));
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let _ = fs::remove_file(&artifact);
    let _ = fs::remove_dir(invocation_lock_path(&artifact));
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-lite-v1:0",
        "hello bedrock",
        "bedrock ok",
        200,
        "agent-logic-admin",
        "us-west-2",
        Some("account-hash"),
        "account_hash_verified",
    ))
    .expect("first bedrock invocation record should write");
    write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-pro-v1:0",
        "second",
        "ok",
        202,
        "agent-logic-admin",
        "us-east-1",
        None,
        "account_hash_verified",
    ))
    .expect("second bedrock invocation record should append");

    let payload: serde_json::Value =
        serde_json::from_slice(&fs::read(&artifact).expect("artifact should exist"))
            .expect("artifact should be json");
    assert_eq!(
        payload["credential_policy"],
        "operator_env_or_aws_profile_only_no_secret_material_recorded"
    );
    let invocations = payload["invocations"]
        .as_array()
        .expect("invocations array");
    assert_eq!(invocations.len(), 2);
    assert_eq!(invocations[0]["family"], "bedrock");
    assert_eq!(invocations[0]["model"], "amazon.nova-lite-v1:0");
    assert_eq!(invocations[0]["http_status"], 200);
    assert_eq!(invocations[0]["aws_profile"], "agent-logic-admin");
    assert_eq!(invocations[0]["aws_region"], "us-west-2");
    assert_eq!(invocations[0]["account_id_sha256"], "account-hash");
    assert_eq!(
        invocations[0]["account_profile_validation_status"],
        "account_hash_verified"
    );
    assert_eq!(invocations[1]["account_id_sha256"], serde_json::Value::Null);

    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    fs::remove_file(&artifact).expect("cleanup artifact");
}

#[test]
fn bedrock_invocation_artifact_rejects_invalid_existing_payloads() {
    let _guard = env_lock();
    let artifact = std::env::temp_dir().join(format!(
        "adl-bedrock-provider-invocations-{}-{}.json",
        std::process::id(),
        "invalid"
    ));
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let _ = fs::remove_file(&artifact);
    let _ = fs::remove_dir(invocation_lock_path(&artifact));
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    fs::write(&artifact, b"{not-json").expect("write invalid json");
    let err = write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-lite-v1:0",
        "prompt",
        "output",
        200,
        "agent-logic-admin",
        "us-west-2",
        Some("account-hash"),
        "account_hash_verified",
    ))
    .expect_err("invalid existing artifact should fail closed");
    assert!(err
        .to_string()
        .contains("provider invocation artifact is invalid JSON"));

    fs::write(&artifact, br#"{"schema_version":"x"}"#).expect("write missing array");
    let err = write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-lite-v1:0",
        "prompt",
        "output",
        200,
        "agent-logic-admin",
        "us-west-2",
        None,
        "account_hash_verified",
    ))
    .expect_err("missing invocations array should fail closed");
    assert!(err
        .to_string()
        .contains("provider invocation artifact missing invocations array"));

    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    fs::remove_file(&artifact).expect("cleanup artifact");
}

#[test]
#[cfg(unix)]
fn bedrock_invocation_artifact_read_io_failure_is_non_retryable_partial_success_unknown() {
    let _guard = env_lock();
    use std::os::unix::fs::PermissionsExt;

    let temp_root = std::env::temp_dir().join(format!(
        "adl-bedrock-provider-invocation-read-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&temp_root).expect("temp root");
    let artifact = temp_root.join("invocations.json");
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let _ = fs::remove_dir(invocation_lock_path(&artifact));
    fs::write(&artifact, br#"{"invocations":[]}"#).expect("artifact seed");
    let mut permissions = fs::metadata(&artifact)
        .expect("artifact metadata")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&artifact, permissions).expect("remove artifact read permissions");
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    let err = write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-lite-v1:0",
        "prompt-after-provider",
        "output-after-provider",
        200,
        "agent-logic-admin",
        "us-west-2",
        Some("account-hash"),
        "account_hash_verified",
    ))
    .expect_err("post-success artifact read failure should fail closed");
    assert!(
        err.to_string()
            .contains("partial_success_unknown_invocation_record_io_failure"),
        "post-success Bedrock artifact read I/O failure must be partial-success-unknown: {err}"
    );
    assert!(
        !is_retryable_error(&err),
        "post-success Bedrock artifact read I/O failure must not retry"
    );

    let mut permissions = fs::metadata(&artifact)
        .expect("artifact metadata after failure")
        .permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(&artifact, permissions).expect("restore artifact permissions");
    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    fs::remove_dir_all(&temp_root).expect("cleanup temp root");
}

#[test]
fn bedrock_invocation_artifact_write_io_failure_is_non_retryable_partial_success_unknown() {
    let _guard = env_lock();
    let temp_root = std::env::temp_dir().join(format!(
        "adl-bedrock-provider-invocation-write-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&temp_root).expect("temp root");
    let artifact = temp_root.join("invocations.json");
    fs::create_dir(&artifact).expect("directory artifact path");
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let _ = fs::remove_dir(invocation_lock_path(&artifact));
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    let err = write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-lite-v1:0",
        "prompt-after-provider",
        "output-after-provider",
        200,
        "agent-logic-admin",
        "us-west-2",
        Some("account-hash"),
        "account_hash_verified",
    ))
    .expect_err("post-success artifact write failure should fail closed");
    assert!(
        err.to_string()
            .contains("partial_success_unknown_invocation_record_io_failure"),
        "post-success Bedrock artifact write I/O failure must be partial-success-unknown: {err}"
    );
    assert!(
        !is_retryable_error(&err),
        "post-success Bedrock artifact write I/O failure must not retry"
    );

    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    fs::remove_dir_all(&temp_root).expect("cleanup temp root");
}

#[test]
fn bedrock_invocation_artifact_create_dir_failure_is_non_retryable_partial_success_unknown() {
    let _guard = env_lock();
    let temp_root = std::env::temp_dir().join(format!(
        "adl-bedrock-provider-invocation-create-dir-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&temp_root).expect("temp root");
    let parent_file = temp_root.join("not-a-directory");
    fs::write(&parent_file, b"blocks mkdir").expect("parent blocker file");
    let artifact = parent_file.join("invocations.json");
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let _ = fs::remove_dir(invocation_lock_path(&artifact));
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    let err = write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-lite-v1:0",
        "prompt-after-provider",
        "output-after-provider",
        200,
        "agent-logic-admin",
        "us-west-2",
        Some("account-hash"),
        "account_hash_verified",
    ))
    .expect_err("post-success artifact directory creation failure should fail closed");
    assert!(
        err.to_string()
            .contains("partial_success_unknown_invocation_record_io_failure"),
        "post-success Bedrock artifact create-dir failure must be partial-success-unknown: {err}"
    );
    assert!(
        !is_retryable_error(&err),
        "post-success Bedrock artifact create-dir failure must not retry"
    );

    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    fs::remove_dir_all(&temp_root).expect("cleanup temp root");
}

#[test]
fn bedrock_constructor_and_helpers_cover_default_safe_paths() {
    let _guard = env_lock();
    let prev_profile = env::var_os("ADL_AWS_PROFILE");
    let prev_aws_profile = env::var_os("AWS_PROFILE");
    let prev_region = env::var_os("AWS_REGION");
    let prev_default_region = env::var_os("AWS_DEFAULT_REGION");
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let prev_expected_account = env::var_os("ADL_AWS_BEDROCK_ACCOUNT_SHA256");
    env::remove_var("ADL_AWS_PROFILE");
    env::remove_var("AWS_PROFILE");
    env::remove_var("AWS_REGION");
    env::remove_var("AWS_DEFAULT_REGION");
    env::remove_var("ADL_PROVIDER_INVOCATIONS_PATH");
    env::remove_var("ADL_AWS_BEDROCK_ACCOUNT_SHA256");

    let spec = adl::ProviderSpec {
        id: Some("bedrock_primary".to_string()),
        profile: None,
        kind: "bedrock".to_string(),
        base_url: None,
        default_model: Some("hosted:adl-bedrock:amazon.nova-lite-v1:0".to_string()),
        config: HashMap::from([
            (
                "provider_model_id".to_string(),
                json!("amazon.nova-lite-v1:0"),
            ),
            ("max_output_tokens".to_string(), json!("321")),
        ]),
    };
    let target = provider_target(
        "bedrock",
        "aws-bedrock-runtime".to_string(),
        "amazon.nova-lite-v1:0",
    );
    let provider = AwsBedrockProvider::from_target(&spec, &target)
        .expect("default Agent Logic profile should construct without AWS calls");
    assert_eq!(provider.model, "amazon.nova-lite-v1:0");
    assert_eq!(provider.region, DEFAULT_BEDROCK_REGION);
    assert_eq!(provider.profile, DEFAULT_BEDROCK_PROFILE);
    assert_eq!(provider.max_tokens, 321);
    assert_eq!(provider.timeout_secs, None);
    assert_eq!(provider.expected_account_sha256, None);

    let env_hash = sha256_hex("env-agent-logic-account");
    let config_hash = sha256_hex("config-agent-logic-account");
    env::set_var(
        "ADL_AWS_BEDROCK_ACCOUNT_SHA256",
        env_hash.to_ascii_uppercase(),
    );
    let mut matching_spec = spec.clone();
    matching_spec.config.insert(
        "expected_account_sha256".to_string(),
        json!(env_hash.clone()),
    );
    let matching_provider = AwsBedrockProvider::from_target(&matching_spec, &target)
        .expect("matching env/config account pins should construct");
    assert_eq!(
        matching_provider.expected_account_sha256.as_deref(),
        Some(env_hash.as_str())
    );
    let mut conflicting_spec = spec.clone();
    conflicting_spec
        .config
        .insert("expected_account_sha256".to_string(), json!(config_hash));
    let conflict = AwsBedrockProvider::from_target(&conflicting_spec, &target)
        .expect_err("host env account pin must be authoritative");
    assert!(conflict
        .to_string()
        .contains("ADL_AWS_BEDROCK_ACCOUNT_SHA256 is authoritative"));

    let fallback = extract_bedrock_nova_output_text(&json!({
        "outputText": " fallback bedrock text "
    }));
    assert_eq!(fallback.as_deref(), Some("fallback bedrock text"));
    assert!(extract_bedrock_nova_output_text(&json!({"output": {}})).is_none());

    let retryable = bedrock_sdk_error("ServiceUnavailable: try later".to_string());
    assert!(retryable.to_string().contains("ServiceUnavailable"));
    let non_retryable = bedrock_sdk_error("ValidationException: bad model".to_string());
    assert!(non_retryable.to_string().contains("ValidationException"));
    let account_hash = sha256_hex("agent-logic-account");
    assert_eq!(account_hash.len(), 64);
    assert!(account_hash.chars().all(|ch| ch.is_ascii_hexdigit()));

    write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-lite-v1:0",
        "prompt",
        "output",
        200,
        DEFAULT_BEDROCK_PROFILE,
        DEFAULT_BEDROCK_REGION,
        None,
        "account_hash_verified",
    ))
    .expect("missing artifact path should be a no-op");

    restore_env_var("ADL_AWS_PROFILE", prev_profile);
    restore_env_var("AWS_PROFILE", prev_aws_profile);
    restore_env_var("AWS_REGION", prev_region);
    restore_env_var("AWS_DEFAULT_REGION", prev_default_region);
    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    restore_env_var("ADL_AWS_BEDROCK_ACCOUNT_SHA256", prev_expected_account);
}

#[test]
fn bedrock_account_identity_requires_operator_approved_hash() {
    let observed = sha256_hex("123456789012");

    verify_bedrock_account_identity(Some(&observed), Some(&observed))
        .expect("matching account hash should verify");
    verify_bedrock_account_identity(Some(&observed), Some(&observed.to_ascii_uppercase()))
        .expect("uppercase expected account hash should normalize and verify");

    let missing_expected = verify_bedrock_account_identity(Some(&observed), None)
        .expect_err("missing expected account hash should fail closed");
    assert!(missing_expected
        .to_string()
        .contains("requires operator-approved expected account hash"));

    let missing_observed = verify_bedrock_account_identity(None, Some(&observed))
        .expect_err("missing STS account should fail closed");
    assert!(missing_observed
        .to_string()
        .contains("STS identity did not include an account id"));

    let mismatch = verify_bedrock_account_identity(Some(&observed), Some(&sha256_hex("other")))
        .expect_err("mismatched account should fail closed");
    assert!(mismatch
        .to_string()
        .contains("does not match expected Agent Logic account hash"));

    let malformed = verify_bedrock_account_identity(Some(&observed), Some("not-a-sha"))
        .expect_err("malformed expected account hash should fail closed");
    assert!(malformed
        .to_string()
        .contains("64-character SHA-256 hex digest"));
}

#[test]
fn invocation_artifact_lock_child_process_helper() {
    let Some(lock_path) = env::var_os("ADL_INVOCATION_LOCK_CHILD_PATH") else {
        return;
    };
    let artifact = PathBuf::from(lock_path);
    let marker =
        PathBuf::from(env::var_os("ADL_INVOCATION_LOCK_CHILD_MARKER").expect("child marker env"));
    let _lock = acquire_invocation_artifact_lock(&artifact).expect("child lock");
    fs::write(marker, "locked").expect("child marker write");
    std::thread::sleep(Duration::from_millis(50));
    std::process::exit(0);
}

fn provider_spec(
    kind: &str,
    endpoint: &str,
    auth_env: Option<&str>,
    extra_headers: &[(&str, &str)],
) -> adl::ProviderSpec {
    let mut config = HashMap::new();
    config.insert("endpoint".to_string(), json!(endpoint));
    if let Some(auth_env) = auth_env {
        config.insert(
            "auth".to_string(),
            json!({
                "type": "bearer",
                "env": auth_env,
            }),
        );
    }
    if !extra_headers.is_empty() {
        let headers = extra_headers
            .iter()
            .map(|(k, v)| ((*k).to_string(), json!(v)))
            .collect();
        config.insert("headers".to_string(), serde_json::Value::Object(headers));
    }
    adl::ProviderSpec {
        id: Some(format!("{kind}_primary")),
        profile: None,
        kind: kind.to_string(),
        base_url: None,
        default_model: Some("model-x".to_string()),
        config,
    }
}

#[test]
fn openai_provider_complete_records_output_and_invocation_artifact() {
    let _guard = env_lock();
    let Some((endpoint, captured, handle)) =
        spawn_json_server(200, r#"{"output_text":"openai ok"}"#)
    else {
        return;
    };

    let artifact = std::env::temp_dir().join(format!(
        "adl-provider-invocations-{}-openai.json",
        std::process::id()
    ));
    let artifact_display = artifact.to_string_lossy().to_string();
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let prev_key = env::var_os("OPENAI_API_KEY");
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact_display);
    env::set_var("OPENAI_API_KEY", "test-openai-token");

    let spec = provider_spec(
        "openai",
        &format!("{endpoint}/v1/responses"),
        Some("OPENAI_API_KEY"),
        &[],
    );
    let target = provider_target("openai", format!("{endpoint}/v1/responses"), "gpt-test");
    let provider = OpenAiProvider::from_target(&spec, &target).expect("provider");

    let output = provider.complete("hello openai").expect("completion");
    assert_eq!(output, "openai ok");

    let captured = captured.lock().expect("capture").clone().expect("request");
    assert_eq!(captured.url, "/v1/responses");
    assert!(captured.body.contains(r#""model":"gpt-test""#));
    assert!(captured.body.contains(r#""input":"hello openai""#));
    assert!(captured
        .headers
        .iter()
        .any(|(k, v)| k.eq_ignore_ascii_case("authorization") && v == "Bearer test-openai-token"));

    let payload = std::fs::read_to_string(&artifact).expect("artifact");
    let json: serde_json::Value = serde_json::from_str(&payload).expect("json artifact");
    assert_eq!(json["schema_version"], "adl.native_provider_invocations.v1");
    assert_eq!(json["invocations"].as_array().map(|v| v.len()), Some(1));
    assert_eq!(json["invocations"][0]["family"], "openai");
    assert_eq!(json["invocations"][0]["model"], "gpt-test");
    assert_eq!(json["invocations"][0]["prompt_chars"], 12);
    assert_eq!(json["invocations"][0]["output_chars"], 9);

    match prev_artifact {
        Some(v) => env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", v),
        None => env::remove_var("ADL_PROVIDER_INVOCATIONS_PATH"),
    }
    match prev_key {
        Some(v) => env::set_var("OPENAI_API_KEY", v),
        None => env::remove_var("OPENAI_API_KEY"),
    }

    let _ = handle.join();
}

#[test]
fn ollama_http_provider_complete_posts_to_generate_endpoint() {
    let _guard = env_lock();
    let Some((endpoint, captured, handle)) =
        spawn_json_server(200, r#"{"response":"ollama ok","done":true}"#)
    else {
        return;
    };

    let spec = ollama_provider_spec_with_base_url(&endpoint);
    let target = provider_target("ollama", endpoint.clone(), "phi4-mini");
    let provider = OllamaHttpProvider::from_target(&spec, &target).expect("provider");

    let output = provider.complete("hello ollama").expect("completion");
    assert_eq!(output, "ollama ok");

    let captured = captured.lock().expect("capture").clone().expect("request");
    assert_eq!(captured.url, "/api/generate");
    assert!(captured.body.contains(r#""model":"phi4-mini""#));
    assert!(captured.body.contains(r#""prompt":"hello ollama""#));
    assert!(captured.body.contains(r#""stream":false"#));

    let _ = handle.join();
}

#[test]
fn ollama_http_provider_rejects_missing_response_text() {
    let _guard = env_lock();
    let Some((endpoint, _captured, handle)) = spawn_json_server(200, r#"{"done":true}"#) else {
        return;
    };

    let spec = ollama_provider_spec_with_base_url(&endpoint);
    let target = provider_target("ollama", endpoint, "phi4-mini");
    let provider = OllamaHttpProvider::from_target(&spec, &target).expect("provider");
    let err = provider
        .complete("hello ollama")
        .expect_err("missing response should fail");
    assert!(
        err.to_string()
            .contains("response missing 'response' text field"),
        "{err:#}"
    );

    let _ = handle.join();
}

#[test]
fn ollama_http_provider_uses_adl_timeout_secs_when_config_missing() {
    let _guard = env_lock();
    let prev_timeout = env::var_os("ADL_TIMEOUT_SECS");
    env::set_var("ADL_TIMEOUT_SECS", "321");

    let spec = ollama_provider_spec_with_base_url("http://127.0.0.1:11434");
    let target = provider_target("ollama", "http://127.0.0.1:11434".to_string(), "phi4-mini");
    let provider = OllamaHttpProvider::from_target(&spec, &target).expect("provider");

    restore_env_var("ADL_TIMEOUT_SECS", prev_timeout);

    assert_eq!(provider.timeout_secs, Some(321));
}

#[test]
fn ollama_http_provider_uses_default_timeout_when_env_missing() {
    let _guard = env_lock();
    let prev_timeout = env::var_os("ADL_TIMEOUT_SECS");
    env::remove_var("ADL_TIMEOUT_SECS");

    let spec = ollama_provider_spec_with_base_url("http://127.0.0.1:11434");
    let target = provider_target("ollama", "http://127.0.0.1:11434".to_string(), "phi4-mini");
    let provider = OllamaHttpProvider::from_target(&spec, &target).expect("provider");

    restore_env_var("ADL_TIMEOUT_SECS", prev_timeout);

    assert_eq!(provider.timeout_secs, Some(120));
}

#[test]
fn ollama_http_provider_prefers_explicit_config_timeout_over_env() {
    let _guard = env_lock();
    let prev_timeout = env::var_os("ADL_TIMEOUT_SECS");
    env::set_var("ADL_TIMEOUT_SECS", "321");

    let mut spec = ollama_provider_spec_with_base_url("http://127.0.0.1:11434");
    spec.config.insert("timeout_secs".to_string(), json!(17));
    let target = provider_target("ollama", "http://127.0.0.1:11434".to_string(), "phi4-mini");
    let provider = OllamaHttpProvider::from_target(&spec, &target).expect("provider");

    restore_env_var("ADL_TIMEOUT_SECS", prev_timeout);

    assert_eq!(provider.timeout_secs, Some(17));
}

#[test]
fn ollama_http_provider_rejects_invalid_explicit_timeout() {
    let _guard = env_lock();
    let prev_timeout = env::var_os("ADL_TIMEOUT_SECS");
    env::set_var("ADL_TIMEOUT_SECS", "321");

    let mut spec = ollama_provider_spec_with_base_url("http://127.0.0.1:11434");
    spec.config
        .insert("timeout_secs".to_string(), json!("nope"));
    let target = provider_target("ollama", "http://127.0.0.1:11434".to_string(), "phi4-mini");
    let err = OllamaHttpProvider::from_target(&spec, &target)
        .expect_err("invalid explicit timeout should fail");

    restore_env_var("ADL_TIMEOUT_SECS", prev_timeout);

    assert!(
        err.to_string()
            .contains("config.timeout_secs must be a positive integer"),
        "{err:#}"
    );
}

#[test]
fn ollama_http_provider_rejects_zero_explicit_timeout() {
    let _guard = env_lock();
    let prev_timeout = env::var_os("ADL_TIMEOUT_SECS");
    env::set_var("ADL_TIMEOUT_SECS", "321");

    let mut spec = ollama_provider_spec_with_base_url("http://127.0.0.1:11434");
    spec.config.insert("timeout_secs".to_string(), json!(0));
    let target = provider_target("ollama", "http://127.0.0.1:11434".to_string(), "phi4-mini");
    let err = OllamaHttpProvider::from_target(&spec, &target)
        .expect_err("zero explicit timeout should fail");

    restore_env_var("ADL_TIMEOUT_SECS", prev_timeout);

    assert!(
        err.to_string()
            .contains("config.timeout_secs must be a positive integer"),
        "{err:#}"
    );
}

#[test]
fn anthropic_provider_complete_records_output_and_version_header() {
    let _guard = env_lock();
    let Some((endpoint, captured, handle)) = spawn_json_server(
        200,
        r#"{"content":[{"type":"text","text":"anthropic ok"}]}"#,
    ) else {
        return;
    };

    let artifact = std::env::temp_dir().join(format!(
        "adl-provider-invocations-{}-anthropic.json",
        std::process::id()
    ));
    let artifact_display = artifact.to_string_lossy().to_string();
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let prev_key = env::var_os("ANTHROPIC_API_KEY");
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact_display);
    env::set_var("ANTHROPIC_API_KEY", "test-anthropic-token");

    let spec = provider_spec(
        "anthropic",
        &format!("{endpoint}/v1/messages"),
        Some("ANTHROPIC_API_KEY"),
        &[],
    );
    let target = provider_target(
        "anthropic",
        format!("{endpoint}/v1/messages"),
        "claude-test",
    );
    let provider = AnthropicProvider::from_target(&spec, &target).expect("provider");

    let output = provider.complete("hello anthropic").expect("completion");
    assert_eq!(output, "anthropic ok");

    let captured = captured.lock().expect("capture").clone().expect("request");
    assert_eq!(captured.url, "/v1/messages");
    assert!(captured.body.contains(r#""model":"claude-test""#));
    assert!(captured.body.contains(r#""max_tokens":220"#));
    assert!(captured
        .headers
        .iter()
        .any(|(k, v)| k.eq_ignore_ascii_case("x-api-key") && v == "test-anthropic-token"));
    assert!(captured
        .headers
        .iter()
        .any(|(k, v)| k.eq_ignore_ascii_case("anthropic-version") && v == ANTHROPIC_VERSION));

    let payload = std::fs::read_to_string(&artifact).expect("artifact");
    let json: serde_json::Value = serde_json::from_str(&payload).expect("json artifact");
    assert_eq!(json["invocations"][0]["family"], "anthropic");
    assert_eq!(json["invocations"][0]["output_chars"], 12);

    match prev_artifact {
        Some(v) => env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", v),
        None => env::remove_var("ADL_PROVIDER_INVOCATIONS_PATH"),
    }
    match prev_key {
        Some(v) => env::set_var("ANTHROPIC_API_KEY", v),
        None => env::remove_var("ANTHROPIC_API_KEY"),
    }

    let _ = handle.join();
}

#[test]
fn anthropic_provider_complete_normalizes_empty_refusal_response() {
    let _guard = env_lock();
    let Some((endpoint, _captured, handle)) =
        spawn_json_server(200, r#"{"content":[],"stop_reason":"refusal"}"#)
    else {
        return;
    };

    let prev_key = env::var_os("ANTHROPIC_API_KEY");
    env::set_var("ANTHROPIC_API_KEY", "test-anthropic-token");

    let spec = provider_spec(
        "anthropic",
        &format!("{endpoint}/v1/messages"),
        Some("ANTHROPIC_API_KEY"),
        &[],
    );
    let target = provider_target(
        "anthropic",
        format!("{endpoint}/v1/messages"),
        "claude-test",
    );
    let provider = AnthropicProvider::from_target(&spec, &target).expect("provider");

    let output = provider.complete("hello anthropic").expect("completion");
    assert_eq!(output, r#"{"refusal":"provider refused the request"}"#);

    match prev_key {
        Some(v) => env::set_var("ANTHROPIC_API_KEY", v),
        None => env::remove_var("ANTHROPIC_API_KEY"),
    }

    let _ = handle.join();
}

#[test]
fn deepseek_provider_complete_records_chat_completion_request() {
    let _guard = env_lock();
    let Some((endpoint, captured, handle)) = spawn_json_server(
        200,
        r#"{"choices":[{"message":{"role":"assistant","content":"deepseek ok"}}]}"#,
    ) else {
        return;
    };

    let artifact = std::env::temp_dir().join(format!(
        "adl-provider-invocations-{}-deepseek.json",
        std::process::id()
    ));
    let artifact_display = artifact.to_string_lossy().to_string();
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let prev_key = env::var_os("DEEPSEEK_API_KEY");
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact_display);
    env::set_var("DEEPSEEK_API_KEY", "test-deepseek-token");

    let spec = provider_spec(
        "deepseek",
        &format!("{endpoint}/chat/completions"),
        Some("DEEPSEEK_API_KEY"),
        &[],
    );
    let target = provider_target(
        "deepseek",
        format!("{endpoint}/chat/completions"),
        "deepseek-chat",
    );
    let provider = DeepSeekProvider::from_target(&spec, &target).expect("provider");

    let output = provider.complete("hello deepseek").expect("completion");
    assert_eq!(output, "deepseek ok");

    let captured = captured.lock().expect("capture").clone().expect("request");
    assert_eq!(captured.url, "/chat/completions");
    assert!(captured.body.contains(r#""model":"deepseek-chat""#));
    assert!(captured.body.contains(r#""content":"hello deepseek""#));
    assert!(captured.body.contains(r#""stream":false"#));
    assert!(
        captured
            .headers
            .iter()
            .any(|(k, v)| k.eq_ignore_ascii_case("authorization")
                && v == "Bearer test-deepseek-token")
    );

    let payload = std::fs::read_to_string(&artifact).expect("artifact");
    let json: serde_json::Value = serde_json::from_str(&payload).expect("json artifact");
    assert_eq!(json["invocations"][0]["family"], "deepseek");
    assert_eq!(json["invocations"][0]["model"], "deepseek-chat");
    assert_eq!(json["invocations"][0]["output_chars"], 11);

    match prev_artifact {
        Some(v) => env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", v),
        None => env::remove_var("ADL_PROVIDER_INVOCATIONS_PATH"),
    }
    match prev_key {
        Some(v) => env::set_var("DEEPSEEK_API_KEY", v),
        None => env::remove_var("DEEPSEEK_API_KEY"),
    }

    let _ = handle.join();
}

#[test]
fn openrouter_provider_complete_records_chat_completion_request() {
    let _guard = env_lock();
    let Some((endpoint, captured, handle)) = spawn_json_server(
        200,
        r#"{"choices":[{"message":{"role":"assistant","content":"openrouter ok"}}]}"#,
    ) else {
        return;
    };

    let artifact = std::env::temp_dir().join(format!(
        "adl-provider-invocations-{}-openrouter.json",
        std::process::id()
    ));
    let artifact_display = artifact.to_string_lossy().to_string();
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let prev_key = env::var_os("OPENROUTER_API_KEY");
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact_display);
    env::set_var("OPENROUTER_API_KEY", "test-openrouter-token");

    let spec = provider_spec(
        "openrouter",
        &format!("{endpoint}/api/v1/chat/completions"),
        Some("OPENROUTER_API_KEY"),
        &[],
    );
    let target = provider_target(
        "openrouter",
        format!("{endpoint}/api/v1/chat/completions"),
        "deepseek/deepseek-chat",
    );
    let provider = OpenRouterProvider::from_target(&spec, &target).expect("provider");

    let output = provider.complete("hello openrouter").expect("completion");
    assert_eq!(output, "openrouter ok");

    let captured = captured.lock().expect("capture").clone().expect("request");
    assert_eq!(captured.url, "/api/v1/chat/completions");
    assert!(captured
        .body
        .contains(r#""model":"deepseek/deepseek-chat""#));
    assert!(captured.body.contains(r#""content":"hello openrouter""#));
    assert!(captured.body.contains(r#""stream":false"#));
    assert!(captured.headers.iter().any(
        |(k, v)| k.eq_ignore_ascii_case("authorization") && v == "Bearer test-openrouter-token"
    ));

    let payload = std::fs::read_to_string(&artifact).expect("artifact");
    assert!(!payload.contains("test-openrouter-token"));
    let json: serde_json::Value = serde_json::from_str(&payload).expect("json artifact");
    assert_eq!(json["invocations"][0]["family"], "openrouter");
    assert_eq!(json["invocations"][0]["model"], "deepseek/deepseek-chat");
    assert_eq!(json["invocations"][0]["output_chars"], 13);

    match prev_artifact {
        Some(v) => env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", v),
        None => env::remove_var("ADL_PROVIDER_INVOCATIONS_PATH"),
    }
    match prev_key {
        Some(v) => env::set_var("OPENROUTER_API_KEY", v),
        None => env::remove_var("OPENROUTER_API_KEY"),
    }

    let _ = handle.join();
}

#[test]
fn openrouter_provider_rejects_missing_credentials_and_bad_response_shape() {
    let _guard = env_lock();
    let prev_key = env::var_os("OPENROUTER_API_KEY");
    env::remove_var("OPENROUTER_API_KEY");

    let spec = provider_spec(
        "openrouter",
        OPENROUTER_CHAT_COMPLETIONS_ENDPOINT,
        Some("OPENROUTER_API_KEY"),
        &[],
    );
    let target = provider_target(
        "openrouter",
        OPENROUTER_CHAT_COMPLETIONS_ENDPOINT.to_string(),
        "openai/gpt-4o-mini",
    );
    let provider = OpenRouterProvider::from_target(&spec, &target).expect("provider");
    let missing_key = provider
        .complete("hello")
        .expect_err("missing credential should fail");
    assert!(missing_key
        .to_string()
        .contains("missing required auth env var 'OPENROUTER_API_KEY'"));

    env::set_var("OPENROUTER_API_KEY", "test-openrouter-token");
    let Some((endpoint, _captured, handle)) = spawn_json_server(200, r#"{"choices":[]}"#) else {
        restore_env_var("OPENROUTER_API_KEY", prev_key);
        return;
    };
    let bad_spec = provider_spec(
        "openrouter",
        &format!("{endpoint}/api/v1/chat/completions"),
        Some("OPENROUTER_API_KEY"),
        &[],
    );
    let bad_target = provider_target(
        "openrouter",
        format!("{endpoint}/api/v1/chat/completions"),
        "openai/gpt-4o-mini",
    );
    let bad_provider = OpenRouterProvider::from_target(&bad_spec, &bad_target).expect("provider");
    let bad_shape = bad_provider
        .complete("hello")
        .expect_err("missing message content should fail");
    assert!(bad_shape
        .to_string()
        .contains("response missing message content"));

    restore_env_var("OPENROUTER_API_KEY", prev_key);
    let _ = handle.join();
}

#[test]
fn zai_provider_sends_chat_completion_request_and_records_sanitized_artifact() {
    let _guard = env_lock();
    let Some((endpoint, captured, handle)) = spawn_json_server(
        200,
        r#"{"model":"glm-5","choices":[{"message":{"content":"zai ok"}}]}"#,
    ) else {
        return;
    };
    let artifact = std::env::temp_dir().join(format!(
        "adl-zai-provider-artifact-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&artifact);
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let prev_key = env::var_os("ZAI_API_KEY");
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);
    env::set_var("ZAI_API_KEY", "test-zai-token");

    let spec = provider_spec(
        "z_ai",
        &format!("{endpoint}/api/paas/v4/chat/completions"),
        None,
        &[],
    );
    let target = provider_target(
        "z_ai",
        format!("{endpoint}/api/paas/v4/chat/completions"),
        "glm-5",
    );
    let provider = ZAiProvider::from_target(&spec, &target).expect("provider");

    let output = provider.complete("hello zai").expect("completion");
    assert_eq!(output, "zai ok");

    let captured = captured.lock().expect("capture").clone().expect("request");
    assert_eq!(captured.url, "/api/paas/v4/chat/completions");
    assert!(captured.body.contains(r#""model":"glm-5""#));
    assert!(captured.body.contains(r#""content":"hello zai""#));
    assert!(captured.body.contains(r#""stream":false"#));
    assert!(captured
        .headers
        .iter()
        .any(|(k, v)| k.eq_ignore_ascii_case("authorization") && v == "Bearer test-zai-token"));

    let payload = std::fs::read_to_string(&artifact).expect("artifact");
    assert!(!payload.contains("test-zai-token"));
    let json: serde_json::Value = serde_json::from_str(&payload).expect("json artifact");
    assert_eq!(json["invocations"][0]["family"], "z_ai");
    assert_eq!(json["invocations"][0]["model"], "glm-5");
    assert_eq!(json["invocations"][0]["output_chars"], 6);

    match prev_artifact {
        Some(v) => env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", v),
        None => env::remove_var("ADL_PROVIDER_INVOCATIONS_PATH"),
    }
    match prev_key {
        Some(v) => env::set_var("ZAI_API_KEY", v),
        None => env::remove_var("ZAI_API_KEY"),
    }

    let _ = handle.join();
}

#[test]
fn zai_provider_rejects_missing_credentials_and_bad_response_shape() {
    let _guard = env_lock();
    let prev_key = env::var_os("ZAI_API_KEY");
    env::remove_var("ZAI_API_KEY");

    let spec = provider_spec(
        "z_ai",
        Z_AI_CHAT_COMPLETIONS_ENDPOINT,
        Some("ZAI_API_KEY"),
        &[],
    );
    let target = provider_target("z_ai", Z_AI_CHAT_COMPLETIONS_ENDPOINT.to_string(), "glm-5");
    let provider = ZAiProvider::from_target(&spec, &target).expect("provider");
    let missing_key = provider
        .complete("hello")
        .expect_err("missing credential should fail");
    assert!(missing_key
        .to_string()
        .contains("missing required auth env var 'ZAI_API_KEY'"));

    env::set_var("ZAI_API_KEY", "test-zai-token");
    let Some((endpoint, _captured, handle)) = spawn_json_server(200, r#"{"choices":[]}"#) else {
        restore_env_var("ZAI_API_KEY", prev_key);
        return;
    };
    let bad_spec = provider_spec(
        "z_ai",
        &format!("{endpoint}/api/paas/v4/chat/completions"),
        Some("ZAI_API_KEY"),
        &[],
    );
    let bad_target = provider_target(
        "z_ai",
        format!("{endpoint}/api/paas/v4/chat/completions"),
        "glm-5",
    );
    let bad_provider = ZAiProvider::from_target(&bad_spec, &bad_target).expect("provider");
    let bad_shape = bad_provider
        .complete("hello")
        .expect_err("missing message content should fail");
    assert!(bad_shape
        .to_string()
        .contains("response missing message content"));

    restore_env_var("ZAI_API_KEY", prev_key);
    let _ = handle.join();
}

#[test]
fn deepseek_provider_rejects_missing_credentials_and_bad_response_shape() {
    let _guard = env_lock();
    let prev_key = env::var_os("DEEPSEEK_API_KEY");
    env::remove_var("DEEPSEEK_API_KEY");

    let spec = provider_spec(
        "deepseek",
        DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT,
        Some("DEEPSEEK_API_KEY"),
        &[],
    );
    let target = provider_target(
        "deepseek",
        DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT.to_string(),
        "deepseek-chat",
    );
    let provider = DeepSeekProvider::from_target(&spec, &target).expect("provider");
    let missing_key = provider
        .complete("hello")
        .expect_err("missing credential should fail");
    assert!(missing_key
        .to_string()
        .contains("missing required auth env var 'DEEPSEEK_API_KEY'"));

    env::set_var("DEEPSEEK_API_KEY", "test-deepseek-token");
    let Some((endpoint, _captured, handle)) = spawn_json_server(200, r#"{"choices":[]}"#) else {
        restore_env_var("DEEPSEEK_API_KEY", prev_key);
        return;
    };
    let bad_spec = provider_spec(
        "deepseek",
        &format!("{endpoint}/chat/completions"),
        Some("DEEPSEEK_API_KEY"),
        &[],
    );
    let bad_target = provider_target(
        "deepseek",
        format!("{endpoint}/chat/completions"),
        "deepseek-chat",
    );
    let bad_provider = DeepSeekProvider::from_target(&bad_spec, &bad_target).expect("provider");
    let bad_shape = bad_provider
        .complete("hello")
        .expect_err("missing message content should fail");
    assert!(bad_shape
        .to_string()
        .contains("response missing message content"));

    restore_env_var("DEEPSEEK_API_KEY", prev_key);
    let _ = handle.join();
}

#[test]
fn http_provider_complete_and_helper_errors_cover_status_and_validation() {
    let _guard = env_lock();
    let Some((endpoint, captured, handle)) = spawn_json_server(200, r#"{"output":"http ok"}"#)
    else {
        return;
    };

    let mut spec = provider_spec(
        "http",
        &format!("{endpoint}/v1/complete"),
        None,
        &[("X-Test-Header", "present")],
    );
    spec.config.insert("timeout_secs".to_string(), json!(5));
    let target = provider_target("http", format!("{endpoint}/v1/complete"), "http-model");
    let provider = HttpProvider::from_target(&spec, &target).expect("provider");
    let output = provider.complete("hello http").expect("completion");
    assert_eq!(output, "http ok");

    let captured = captured.lock().expect("capture").clone().expect("request");
    assert_eq!(captured.url, "/v1/complete");
    assert!(captured.body.contains(r#""prompt":"hello http""#));
    assert!(captured
        .headers
        .iter()
        .any(|(k, v)| k.eq_ignore_ascii_case("x-test-header") && v == "present"));

    let bad_http = provider_http_json(
        "http",
        reqwest::blocking::Client::new().get("http://127.0.0.1:9/v1/complete"),
    )
    .expect_err("unreachable port should fail");
    assert!(bad_http
        .to_string()
        .contains("kind=request_failed native provider request failed"));

    let status_server = spawn_json_server(503, "this server error body is intentionally very long to ensure the truncation logic is exercised when the body exceeds the provider error preview budget and the status classification remains readable in the error message")
        .expect("status server");
    let (status_endpoint, _status_capture, status_handle) = status_server;
    let status_err = provider_http_json(
        "http",
        reqwest::blocking::Client::new().post(format!("{status_endpoint}/v1/complete")),
    )
    .expect_err("503 should fail");
    assert!(status_err
        .to_string()
        .contains("kind=server_error status=503 Service Unavailable"));

    let invalid_json_server = spawn_json_server(200, "not json").expect("invalid json server");
    let (invalid_endpoint, _invalid_capture, invalid_handle) = invalid_json_server;
    let invalid_json_err = provider_http_json(
        "http",
        reqwest::blocking::Client::new().post(format!("{invalid_endpoint}/v1/complete")),
    )
    .expect_err("invalid json should fail");
    assert!(invalid_json_err.to_string().contains("not valid JSON"));

    let mut bad_auth_spec = provider_spec("openai", &format!("{endpoint}/v1/responses"), None, &[]);
    bad_auth_spec.config.insert(
        "auth".to_string(),
        json!({
            "type": "bearer",
            "env": " "
        }),
    );
    let bad_target = provider_target("openai", format!("{endpoint}/v1/responses"), "gpt-test");
    let bad_auth_err = OpenAiProvider::from_target(&bad_auth_spec, &bad_target)
        .expect_err("empty auth env should fail");
    assert!(bad_auth_err.to_string().contains("config.auth.env"));

    let bad_endpoint_spec = provider_spec(
        "openai",
        "http://example.com/v1/responses",
        Some("OPENAI_API_KEY"),
        &[],
    );
    let bad_endpoint_err = OpenAiProvider::from_target(
        &bad_endpoint_spec,
        &provider_target(
            "openai",
            "http://example.com/v1/responses".to_string(),
            "gpt-test",
        ),
    )
    .expect_err("plaintext remote endpoint should fail");
    assert!(bad_endpoint_err
        .to_string()
        .contains("endpoint must use https://"));

    let untrusted_openai_spec = provider_spec(
        "openai",
        "https://proxy.example.com/v1/responses",
        Some("OPENAI_API_KEY"),
        &[],
    );
    let untrusted_openai_err = OpenAiProvider::from_target(
        &untrusted_openai_spec,
        &provider_target(
            "openai",
            "https://proxy.example.com/v1/responses".to_string(),
            "gpt-test",
        ),
    )
    .expect_err("default OpenAI credentials should not go to untrusted remote endpoints");
    assert!(untrusted_openai_err
        .to_string()
        .contains("config.trust_custom_endpoint"));

    let mut trusted_openai_spec = untrusted_openai_spec.clone();
    trusted_openai_spec
        .config
        .insert("trust_custom_endpoint".to_string(), json!(true));
    OpenAiProvider::from_target(
        &trusted_openai_spec,
        &provider_target(
            "openai",
            "https://proxy.example.com/v1/responses".to_string(),
            "gpt-test",
        ),
    )
    .expect("explicitly trusted OpenAI custom endpoint should build");

    let untrusted_anthropic_spec = provider_spec(
        "anthropic",
        "https://proxy.example.com/v1/messages",
        Some("ANTHROPIC_API_KEY"),
        &[],
    );
    let untrusted_anthropic_err = AnthropicProvider::from_target(
        &untrusted_anthropic_spec,
        &provider_target(
            "anthropic",
            "https://proxy.example.com/v1/messages".to_string(),
            "claude-test",
        ),
    )
    .expect_err("default Anthropic credentials should not go to untrusted remote endpoints");
    assert!(untrusted_anthropic_err
        .to_string()
        .contains("config.trust_custom_endpoint"));

    let untrusted_deepseek_spec = provider_spec(
        "deepseek",
        "https://proxy.example.com/chat/completions",
        Some("DEEPSEEK_API_KEY"),
        &[],
    );
    let untrusted_deepseek_err = DeepSeekProvider::from_target(
        &untrusted_deepseek_spec,
        &provider_target(
            "deepseek",
            "https://proxy.example.com/chat/completions".to_string(),
            "deepseek-chat",
        ),
    )
    .expect_err("default DeepSeek credentials should not go to untrusted remote endpoints");
    assert!(untrusted_deepseek_err
        .to_string()
        .contains("config.trust_custom_endpoint"));

    let untrusted_openrouter_spec = provider_spec(
        "openrouter",
        "https://proxy.example.com/api/v1/chat/completions",
        Some("OPENROUTER_API_KEY"),
        &[],
    );
    let untrusted_openrouter_err = OpenRouterProvider::from_target(
        &untrusted_openrouter_spec,
        &provider_target(
            "openrouter",
            "https://proxy.example.com/api/v1/chat/completions".to_string(),
            "openai/gpt-4o-mini",
        ),
    )
    .expect_err("default OpenRouter credentials should not go to untrusted remote endpoints");
    assert!(untrusted_openrouter_err
        .to_string()
        .contains("config.trust_custom_endpoint"));

    assert_eq!(
        extract_openai_output_text(&json!({"output_text": "  openai inline  "})),
        Some("openai inline".to_string())
    );
    assert_eq!(
        extract_openai_output_text(&json!({
            "output": [{"content": [{"text": "part one"}, {"text": "part two"}]}]
        })),
        Some("part one\npart two".to_string())
    );
    assert_eq!(extract_openai_output_text(&json!({"output": []})), None);

    assert_eq!(
        extract_anthropic_output_text(&json!({
            "content": [{"type": "text", "text": "  anthropic inline  "}]})),
        Some("anthropic inline".to_string())
    );
    assert_eq!(
        extract_anthropic_output_text(&json!({
            "content": [{"type": "tool_use", "text": "ignored"}]
        })),
        None
    );
    assert_eq!(
        extract_deepseek_output_text(&json!({
            "choices": [{"message": {"content": "  deepseek inline  "}}]
        })),
        Some("deepseek inline".to_string())
    );
    assert_eq!(
        extract_deepseek_output_text(&json!({"choices": [{"message": {"role": "assistant"}}]})),
        None
    );

    let _ = handle.join();
    let _ = status_handle.join();
    let _ = invalid_handle.join();
}

#[test]
fn helper_validation_and_extraction_paths_are_exercised() {
    let default_spec = adl::ProviderSpec {
        id: Some("openai_primary".to_string()),
        profile: None,
        kind: "openai".to_string(),
        base_url: None,
        default_model: Some("gpt-test".to_string()),
        config: HashMap::new(),
    };
    assert_eq!(
        auth_env_for(&default_spec, "OPENAI_API_KEY").expect("default auth env"),
        "OPENAI_API_KEY"
    );

    let mut non_object_auth = default_spec.clone();
    non_object_auth
        .config
        .insert("auth".to_string(), json!("bad-shape"));
    assert!(auth_env_for(&non_object_auth, "OPENAI_API_KEY")
        .expect_err("non-object auth should fail")
        .to_string()
        .contains("config.auth must be an object"));

    let mut wrong_type_auth = default_spec.clone();
    wrong_type_auth.config.insert(
        "auth".to_string(),
        json!({
            "type": "basic",
            "env": "OPENAI_API_KEY"
        }),
    );
    assert!(auth_env_for(&wrong_type_auth, "OPENAI_API_KEY")
        .expect_err("wrong auth type should fail")
        .to_string()
        .contains("must be 'bearer'"));

    let mut missing_env_auth = default_spec.clone();
    missing_env_auth.config.insert(
        "auth".to_string(),
        json!({
            "type": "bearer"
        }),
    );
    assert!(auth_env_for(&missing_env_auth, "OPENAI_API_KEY")
        .expect_err("missing env should fail")
        .to_string()
        .contains("config.auth.env is required"));

    let target_with_default = provider_target("openai", String::new(), "gpt-test");
    let endpoint = vendor_endpoint(
        &default_spec,
        &ProviderInvocationTargetV1 {
            endpoint: None,
            base_url: None,
            ..target_with_default
        },
        OPENAI_RESPONSES_ENDPOINT,
        "openai",
    )
    .expect("default endpoint should be used");
    assert_eq!(endpoint, OPENAI_RESPONSES_ENDPOINT);

    let mut empty_endpoint_override = default_spec.clone();
    empty_endpoint_override
        .config
        .insert("endpoint".to_string(), json!("   "));
    assert!(vendor_endpoint(
        &empty_endpoint_override,
        &provider_target("openai", OPENAI_RESPONSES_ENDPOINT.to_string(), "gpt-test"),
        OPENAI_RESPONSES_ENDPOINT,
        "openai"
    )
    .expect_err("empty endpoint override should fail")
    .to_string()
    .contains("config.endpoint must not be empty"));

    OpenAiProvider::from_target(
        &default_spec,
        &ProviderInvocationTargetV1 {
            endpoint: None,
            base_url: None,
            ..provider_target("openai", String::new(), "gpt-test")
        },
    )
    .expect("default OpenAI vendor endpoint should build with default credentials");

    let mut bad_trust_flag = default_spec.clone();
    bad_trust_flag
        .config
        .insert("trust_custom_endpoint".to_string(), json!("yes"));
    bad_trust_flag.config.insert(
        "endpoint".to_string(),
        json!("https://proxy.example.com/v1/responses"),
    );
    assert!(OpenAiProvider::from_target(
        &bad_trust_flag,
        &provider_target(
            "openai",
            "https://proxy.example.com/v1/responses".to_string(),
            "gpt-test"
        )
    )
    .expect_err("non-boolean trust flag should fail")
    .to_string()
    .contains("config.trust_custom_endpoint must be a boolean"));

    let long_text = format!("  {}  ", "x".repeat(250));
    assert_eq!(truncate_provider_body(&long_text).len(), 200);
    assert_eq!(truncate_provider_body("  short body  "), "short body");

    let multibyte_boundary = format!("{}étail", "x".repeat(199));
    let truncated = truncate_provider_body(&multibyte_boundary);
    assert_eq!(truncated.len(), 199);
    assert_eq!(truncated.chars().count(), 199);
    assert!(truncated.ends_with('x'));

    assert_eq!(
        extract_openai_output_text(&json!({
            "output": [{"content": [{"text": ""}, {"text": " useful " }]}]
        })),
        Some("\n useful ".trim().to_string())
    );
    assert_eq!(
        extract_anthropic_output_text(&json!({
            "content": [
                {"type": "text", "text": "first"},
                {"type": "text", "text": "second"}
            ]
        })),
        Some("first\nsecond".to_string())
    );
}

#[test]
fn invocation_artifact_and_http_constructor_error_paths_are_exercised() {
    let _guard = env_lock();
    let temp_root = std::env::temp_dir().join(format!(
        "adl-http-family-tests-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_root).expect("temp root");

    let artifact = temp_root.join("invocations.json");
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let prev_lock_timeout = env::var_os("ADL_INVOCATION_LOCK_TIMEOUT_MS");
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    write_native_invocation_record("openai", "gpt-test", "hello", "world", 200)
        .expect("write fresh artifact");
    let first_payload = std::fs::read_to_string(&artifact).expect("fresh artifact");
    assert!(first_payload.contains("\"schema_version\": \"adl.native_provider_invocations.v1\""));

    std::fs::write(&artifact, "not-json").expect("invalid artifact write");
    assert!(
        write_native_invocation_record("openai", "gpt-test", "hello", "world", 200)
            .expect_err("invalid json artifact should fail")
            .to_string()
            .contains("invalid JSON")
    );

    std::fs::write(
        &artifact,
        serde_json::to_vec_pretty(&json!({
            "schema_version": "adl.native_provider_invocations.v1",
            "credential_policy": "operator_env_only_no_secret_material_recorded",
            "invocations": {}
        }))
        .expect("serialize malformed artifact"),
    )
    .expect("malformed artifact write");
    assert!(
        write_native_invocation_record("openai", "gpt-test", "hello", "world", 200)
            .expect_err("artifact without array should fail")
            .to_string()
            .contains("missing invocations array")
    );

    std::fs::remove_file(&artifact).expect("remove malformed artifact");
    let thread_count = 8usize;
    let mut handles = Vec::new();
    for idx in 0..thread_count {
        handles.push(std::thread::spawn(move || {
            write_native_invocation_record(
                "openai",
                "gpt-test",
                &format!("hello-{idx}"),
                &format!("world-{idx}"),
                200,
            )
        }));
    }
    for handle in handles {
        handle
            .join()
            .expect("concurrent writer thread should not panic")
            .expect("concurrent invocation write should succeed");
    }
    let concurrent_payload: Value =
        serde_json::from_slice(&std::fs::read(&artifact).expect("read concurrent artifact"))
            .expect("concurrent artifact json");
    let invocations = concurrent_payload
        .get("invocations")
        .and_then(|v| v.as_array())
        .expect("invocations array");
    assert_eq!(
        invocations.len(),
        thread_count,
        "concurrent writes should preserve every invocation entry"
    );

    write_native_invocation_record("openai", "gpt-test", "after-lock", "recorded", 200)
        .expect("advisory invocation lock should write after concurrent writers");
    let recovered_payload: Value =
        serde_json::from_slice(&std::fs::read(&artifact).expect("read recovered artifact"))
            .expect("recovered artifact json");
    assert_eq!(
        recovered_payload["invocations"]
            .as_array()
            .expect("recovered invocations array")
            .len(),
        thread_count + 1
    );

    let stress_iterations = 25usize;
    let contender_count = 8usize;
    for iteration in 0..stress_iterations {
        let active = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let max_seen = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut contender_handles = Vec::new();
        for _ in 0..contender_count {
            let artifact = artifact.clone();
            let active = Arc::clone(&active);
            let max_seen = Arc::clone(&max_seen);
            contender_handles.push(std::thread::spawn(move || {
                let _lock = acquire_invocation_artifact_lock(&artifact)?;
                let now = active.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                max_seen.fetch_max(now, std::sync::atomic::Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(1));
                active.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                std::io::Result::Ok(())
            }));
        }
        for handle in contender_handles {
            handle
                .join()
                .expect("lock contender thread should not panic")
                .expect("lock contender should wait safely");
        }
        assert_eq!(
            max_seen.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "advisory invocation lock must preserve mutual exclusion in iteration {iteration}"
        );
    }

    let held_lock = acquire_invocation_artifact_lock(&artifact).expect("held advisory lock");
    env::set_var("ADL_INVOCATION_LOCK_TIMEOUT_MS", "5");
    let native_timeout_err = write_native_invocation_record(
        "openai",
        "gpt-test",
        "prompt-after-provider",
        "output-after-provider",
        200,
    )
    .expect_err("held lock should force native timeout classification");
    assert!(
        native_timeout_err
            .to_string()
            .contains("partial_success_unknown_invocation_record_lock_unavailable"),
        "native timeout should be classified as non-retryable partial-success-unknown: {native_timeout_err}"
    );
    assert!(
        native_timeout_err.to_string().contains("timed out"),
        "native timeout cause should remain visible: {native_timeout_err}"
    );
    assert!(
        !is_retryable_error(&native_timeout_err),
        "native invocation artifact lock timeout after provider completion must be non-retryable"
    );
    let bedrock_timeout_err = write_bedrock_invocation_record(bedrock_invocation_record!(
        "amazon.nova-lite-v1:0",
        "prompt-after-provider",
        "output-after-provider",
        200,
        DEFAULT_BEDROCK_PROFILE,
        DEFAULT_BEDROCK_REGION,
        Some("account-hash"),
        "account_hash_verified",
    ))
    .expect_err("held lock should force timeout classification");
    assert!(
        bedrock_timeout_err
            .to_string()
            .contains("partial_success_unknown_invocation_record_lock_unavailable"),
        "Bedrock timeout should be classified as non-retryable partial-success-unknown: {bedrock_timeout_err}"
    );
    assert!(
        bedrock_timeout_err.to_string().contains("timed out"),
        "Bedrock timeout cause should remain visible: {bedrock_timeout_err}"
    );
    assert!(
        !is_retryable_error(&bedrock_timeout_err),
        "Bedrock invocation artifact lock timeout after provider completion must be non-retryable"
    );
    drop(held_lock);

    let child_artifact = temp_root.join("child-invocations.json");
    let marker = temp_root.join("child-lock-marker");
    let child_status = Command::new(env::current_exe().expect("current test executable"))
        .arg("--exact")
        .arg("provider::http_family::tests::invocation_artifact_lock_child_process_helper")
        .arg("--nocapture")
        .env("ADL_INVOCATION_LOCK_CHILD_PATH", &child_artifact)
        .env("ADL_INVOCATION_LOCK_CHILD_MARKER", &marker)
        .status()
        .expect("spawn child lock helper");
    assert!(
        child_status.success(),
        "child lock helper should exit cleanly"
    );
    assert!(
        marker.exists(),
        "child helper must prove it acquired the lock"
    );
    acquire_invocation_artifact_lock(&child_artifact)
        .expect("OS must release advisory invocation lock after child process exits");

    match prev_artifact {
        Some(v) => env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", v),
        None => env::remove_var("ADL_PROVIDER_INVOCATIONS_PATH"),
    }
    match prev_lock_timeout {
        Some(v) => env::set_var("ADL_INVOCATION_LOCK_TIMEOUT_MS", v),
        None => env::remove_var("ADL_INVOCATION_LOCK_TIMEOUT_MS"),
    }

    let target = provider_target(
        "http",
        "https://api.example.com/v1/complete".to_string(),
        "http-model",
    );

    let mut bad_headers_spec =
        provider_spec("http", "https://api.example.com/v1/complete", None, &[]);
    bad_headers_spec
        .config
        .insert("headers".to_string(), json!("bad"));
    assert!(HttpProvider::from_target(&bad_headers_spec, &target)
        .expect_err("non-object headers should fail")
        .to_string()
        .contains("config.headers must be an object"));

    let mut non_string_header_spec =
        provider_spec("http", "https://api.example.com/v1/complete", None, &[]);
    non_string_header_spec
        .config
        .insert("headers".to_string(), json!({"X-Number": 12}));
    assert!(HttpProvider::from_target(&non_string_header_spec, &target)
        .expect_err("non-string header should fail")
        .to_string()
        .contains("config.headers values must be strings"));

    let mut non_object_auth_spec =
        provider_spec("http", "https://api.example.com/v1/complete", None, &[]);
    non_object_auth_spec
        .config
        .insert("auth".to_string(), json!("bad"));
    assert!(HttpProvider::from_target(&non_object_auth_spec, &target)
        .expect_err("non-object auth should fail")
        .to_string()
        .contains("config.auth must be an object"));

    let mut missing_type_auth_spec =
        provider_spec("http", "https://api.example.com/v1/complete", None, &[]);
    missing_type_auth_spec
        .config
        .insert("auth".to_string(), json!({"env": "HTTP_API_KEY"}));
    assert!(HttpProvider::from_target(&missing_type_auth_spec, &target)
        .expect_err("missing auth type should fail")
        .to_string()
        .contains("config.auth.type is required"));

    let mut missing_env_auth_spec =
        provider_spec("http", "https://api.example.com/v1/complete", None, &[]);
    missing_env_auth_spec
        .config
        .insert("auth".to_string(), json!({"type": "bearer"}));
    assert!(HttpProvider::from_target(&missing_env_auth_spec, &target)
        .expect_err("missing auth env should fail")
        .to_string()
        .contains("config.auth.env is required"));

    let mut untrusted_bearer_spec = provider_spec(
        "http",
        "https://api.example.com/v1/complete",
        Some("HTTP_API_KEY"),
        &[],
    );
    assert!(HttpProvider::from_target(&untrusted_bearer_spec, &target)
        .expect_err("remote bearer auth should require explicit endpoint trust")
        .to_string()
        .contains("config.trust_custom_endpoint: true"));

    untrusted_bearer_spec
        .config
        .insert("trust_custom_endpoint".to_string(), json!(true));
    HttpProvider::from_target(&untrusted_bearer_spec, &target)
        .expect("explicitly trusted remote bearer endpoint should build");

    let ipv6_loopback_target = provider_target(
        "http",
        "http://[::1]:11434/v1/complete".to_string(),
        "local-model",
    );
    let ipv6_loopback_spec = provider_spec(
        "http",
        "http://[::1]:11434/v1/complete",
        Some("HTTP_API_KEY"),
        &[],
    );
    HttpProvider::from_target(&ipv6_loopback_spec, &ipv6_loopback_target)
        .expect("bracketed IPv6 loopback bearer endpoint should be trusted as loopback");
}

#[test]
#[cfg(unix)]
fn native_invocation_artifact_read_io_failure_is_non_retryable_partial_success_unknown() {
    let _guard = env_lock();
    use std::os::unix::fs::PermissionsExt;

    let temp_root = std::env::temp_dir().join(format!(
        "adl-native-provider-invocation-read-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_root).expect("temp root");
    let artifact = temp_root.join("invocations.json");
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let _ = std::fs::remove_dir(invocation_lock_path(&artifact));
    std::fs::write(&artifact, br#"{"invocations":[]}"#).expect("artifact seed");
    let mut permissions = std::fs::metadata(&artifact)
        .expect("artifact metadata")
        .permissions();
    permissions.set_mode(0o000);
    std::fs::set_permissions(&artifact, permissions).expect("remove artifact read permissions");
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    let err = write_native_invocation_record(
        "openai",
        "gpt-test",
        "prompt-after-provider",
        "output-after-provider",
        200,
    )
    .expect_err("post-success artifact read failure should fail closed");
    assert!(
        err.to_string()
            .contains("partial_success_unknown_invocation_record_io_failure"),
        "post-success native artifact read I/O failure must be partial-success-unknown: {err}"
    );
    assert!(
        !is_retryable_error(&err),
        "post-success native artifact read I/O failure must not retry"
    );

    let mut permissions = std::fs::metadata(&artifact)
        .expect("artifact metadata after failure")
        .permissions();
    permissions.set_mode(0o600);
    std::fs::set_permissions(&artifact, permissions).expect("restore artifact permissions");
    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    std::fs::remove_dir_all(&temp_root).expect("cleanup temp root");
}

#[test]
fn native_invocation_artifact_write_io_failure_is_non_retryable_partial_success_unknown() {
    let _guard = env_lock();
    let temp_root = std::env::temp_dir().join(format!(
        "adl-native-provider-invocation-write-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_root).expect("temp root");
    let artifact = temp_root.join("invocations.json");
    std::fs::create_dir(&artifact).expect("directory artifact path");
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let _ = std::fs::remove_dir(invocation_lock_path(&artifact));
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    let err = write_native_invocation_record(
        "openai",
        "gpt-test",
        "prompt-after-provider",
        "output-after-provider",
        200,
    )
    .expect_err("post-success artifact write failure should fail closed");
    assert!(
        err.to_string()
            .contains("partial_success_unknown_invocation_record_io_failure"),
        "post-success native artifact write I/O failure must be partial-success-unknown: {err}"
    );
    assert!(
        !is_retryable_error(&err),
        "post-success native artifact write I/O failure must not retry"
    );

    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    std::fs::remove_dir_all(&temp_root).expect("cleanup temp root");
}

#[test]
fn native_invocation_artifact_create_dir_failure_is_non_retryable_partial_success_unknown() {
    let _guard = env_lock();
    let temp_root = std::env::temp_dir().join(format!(
        "adl-native-provider-invocation-create-dir-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_root).expect("temp root");
    let parent_file = temp_root.join("not-a-directory");
    std::fs::write(&parent_file, b"blocks mkdir").expect("parent blocker file");
    let artifact = parent_file.join("invocations.json");
    let prev_artifact = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH");
    let _ = std::fs::remove_dir(invocation_lock_path(&artifact));
    env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

    let err = write_native_invocation_record(
        "openai",
        "gpt-test",
        "prompt-after-provider",
        "output-after-provider",
        200,
    )
    .expect_err("post-success artifact directory creation failure should fail closed");
    assert!(
        err.to_string()
            .contains("partial_success_unknown_invocation_record_io_failure"),
        "post-success native artifact create-dir failure must be partial-success-unknown: {err}"
    );
    assert!(
        !is_retryable_error(&err),
        "post-success native artifact create-dir failure must not retry"
    );

    restore_env_var("ADL_PROVIDER_INVOCATIONS_PATH", prev_artifact);
    std::fs::remove_dir_all(&temp_root).expect("cleanup temp root");
}
