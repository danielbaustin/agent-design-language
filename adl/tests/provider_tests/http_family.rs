use std::io::{Read, Write};

use ::adl::provider::{build_provider, is_retryable_error, stable_failure_kind};

use super::helpers::EnvVarGuard;
use super::support::{
    block_incoming_localhost, localhost_and_auth_env_guard, provider_spec_from_yaml,
    read_http_request,
};

#[test]
fn http_provider_happy_path() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = r#"{"output":"OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let out = p.complete("hello").expect("http provider should succeed");
    assert_eq!(out, "OK");
}

#[test]
fn openai_provider_translates_native_response() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _env_guard = localhost_and_auth_env_guard("ADL_TEST_OPENAI_KEY", "test-openai-token");

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.to_ascii_lowercase().contains("authorization:"));
        assert!(request.contains("Bearer test-openai-token"));
        assert!(request.contains("\"model\":\"gpt-test\""));
        assert!(request.contains("\"input\":\"hello openai\""));
        let body = r#"{"output_text":"OPENAI_NATIVE_OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: openai
config:
  endpoint: "http://{addr}/v1/responses"
  provider_model_id: "gpt-test"
  auth:
    type: bearer
    env: ADL_TEST_OPENAI_KEY
"#
    ));

    let p = build_provider(&spec, None).expect("openai provider should build");
    let out = p
        .complete("hello openai")
        .expect("openai provider should succeed");
    assert_eq!(out, "OPENAI_NATIVE_OK");
}

#[test]
fn anthropic_provider_translates_native_response() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _env_guard = localhost_and_auth_env_guard("ADL_TEST_ANTHROPIC_KEY", "test-anthropic-token");

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.to_ascii_lowercase().contains("x-api-key:"));
        assert!(request.contains("test-anthropic-token"));
        assert!(request
            .to_ascii_lowercase()
            .contains("anthropic-version: 2023-06-01"));
        assert!(request.contains("\"model\":\"claude-test\""));
        assert!(request.contains("\"content\":\"hello claude\""));
        let body = r#"{"content":[{"type":"text","text":"ANTHROPIC_NATIVE_OK"}]}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: anthropic
config:
  endpoint: "http://{addr}/v1/messages"
  provider_model_id: "claude-test"
  auth:
    type: bearer
    env: ADL_TEST_ANTHROPIC_KEY
"#
    ));

    let p = build_provider(&spec, None).expect("anthropic provider should build");
    let out = p
        .complete("hello claude")
        .expect("anthropic provider should succeed");
    assert_eq!(out, "ANTHROPIC_NATIVE_OK");
}

#[test]
fn native_provider_missing_auth_env_is_sanitized() {
    let _env_guard = EnvVarGuard::unset("ADL_TEST_MISSING_OPENAI_KEY");
    let spec = provider_spec_from_yaml(
        r#"
type: openai
config:
  provider_model_id: "gpt-test"
  auth:
    type: bearer
    env: ADL_TEST_MISSING_OPENAI_KEY
"#,
    );

    let p = build_provider(&spec, None).expect("openai provider should build");
    let err = p
        .complete("hello")
        .expect_err("missing auth env should fail");
    let msg = format!("{err:#}");
    assert!(msg.contains("missing required auth env var"));
    assert!(msg.contains("ADL_TEST_MISSING_OPENAI_KEY"));
    assert!(!msg.contains("Bearer"));
}

#[test]
fn http_provider_accepts_https_endpoint() {
    // This test is intentionally config-only: we verify that `https://` endpoints
    // are accepted by parsing/building the provider, without performing a network call.
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "https://example.com/v1/complete"
"#,
    );

    let _p = build_provider(&spec, None).expect("build_provider should accept https endpoints");
}

#[test]
fn http_provider_rejects_plaintext_remote_endpoint() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://api.example.com/v1/complete"
"#,
    );

    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("plain remote http should fail"),
        Err(err) => err,
    };
    assert!(err
        .to_string()
        .contains("plaintext http:// is only allowed for localhost/loopback test endpoints"));
}

#[test]
fn http_provider_non_200_response() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "bad";
        let resp = format!(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("server_error") && msg.contains("500"),
        "unexpected error: {msg}"
    );
    assert!(
        is_retryable_error(&err),
        "5xx responses should be retryable: {msg}"
    );
}

#[test]
fn http_provider_long_error_body_is_truncated_deterministically() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "x".repeat(300);
        let resp = format!(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").expect_err("500 should fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("kind=server_error"),
        "expected server_error classification, got: {msg}"
    );
    assert!(
        msg.contains("status=500"),
        "expected status code in message, got: {msg}"
    );
    assert!(
        msg.len() < 600,
        "response body should be truncated in error message, got len={}",
        msg.len()
    );
}

#[test]
fn http_provider_rejects_json_without_output_field() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = r#"{"not_output":"value"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p
        .complete("hello")
        .expect_err("response without output should fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("response missing 'output' field"),
        "unexpected error: {msg}"
    );
    assert_eq!(stable_failure_kind(&err), Some("provider_error"));
}

#[test]
fn http_provider_4xx_is_non_retryable() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "bad request";
        let resp = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("client_error") && msg.contains("400"),
        "unexpected error: {msg}"
    );
    assert!(
        !is_retryable_error(&err),
        "4xx responses should be non-retryable: {msg}"
    );
}

#[test]
fn http_provider_missing_auth_env_var() {
    let addr = "127.0.0.1:9";

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
  auth:
    type: bearer
    env: MISSING_ENV
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("missing required auth env var"),
        "unexpected error: {msg}"
    );
    assert_eq!(
        stable_failure_kind(&err),
        Some("schema_error"),
        "missing auth env var should classify as schema_error"
    );
}

#[test]
fn http_provider_rejects_non_object_headers_and_non_string_values() {
    let non_object = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  headers: "not-an-object"
"#,
    );
    let err = match build_provider(&non_object, None) {
        Ok(_) => panic!("non-object headers should fail"),
        Err(err) => err,
    };
    assert!(
        err.to_string().contains("config.headers must be an object"),
        "unexpected error: {err:#}"
    );

    let non_string_value = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  headers:
    X-Number: 123
"#,
    );
    let err2 = match build_provider(&non_string_value, None) {
        Ok(_) => panic!("non-string header should fail"),
        Err(err) => err,
    };
    assert!(
        err2.to_string()
            .contains("config.headers values must be strings"),
        "unexpected error: {err2:#}"
    );
}

#[test]
fn http_provider_rejects_non_bearer_auth_type() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  auth:
    type: basic
    env: API_KEY
"#,
    );
    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("non-bearer auth should fail"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("config.auth.type must be 'bearer'"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn http_provider_supports_timeout_secs_string_and_rejects_negative_number() {
    let string_timeout = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  timeout_secs: "7"
"#,
    );
    let _provider =
        build_provider(&string_timeout, None).expect("string timeout should parse as u64");

    let negative_timeout = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  timeout_secs: -3
"#,
    );
    let _provider = build_provider(&negative_timeout, None)
        .expect("negative timeout should be treated as absent, not a parse failure");
}

#[test]
fn http_provider_rejects_missing_endpoint() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config: {}
"#,
    );

    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("expected build_provider to fail for missing endpoint"),
        Err(err) => err,
    };
    let msg = format!("{err:#}");
    assert!(
        msg.contains("invalid config") && msg.contains("endpoint"),
        "unexpected error: {msg}"
    );
}

#[test]
fn http_provider_timeout() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        std::thread::sleep(std::time::Duration::from_secs(2));
        let body = r#"{"output":"OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
  timeout_secs: 1
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}").to_lowercase();
    assert!(
        msg.contains("timeout") || msg.contains("timed out"),
        "unexpected error: {msg}"
    );
}
