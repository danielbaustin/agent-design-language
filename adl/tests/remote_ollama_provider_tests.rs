use ::adl::adl::AdlDoc;
use ::adl::provider::build_provider;
use tiny_http::{Header, Response, Server};

mod helpers;
use helpers::unique_test_temp_dir;

fn reserve_local_port() -> Option<u16> {
    let listener = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener,
        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => return None,
        Err(err) => panic!("bind ephemeral port: {err}"),
    };
    let port = listener.local_addr().expect("local addr").port();
    drop(listener);
    Some(port)
}

#[test]
fn build_provider_accepts_remote_ollama_http_transport() {
    let _temp_dir = unique_test_temp_dir("remote-ollama-provider");
    let Some(port) = reserve_local_port() else {
        return;
    };
    let bind_addr = format!("127.0.0.1:{port}");
    let server = Server::http(&bind_addr).expect("bind tiny_http server");
    let handle = std::thread::spawn(move || {
        if let Some(request) = server.incoming_requests().next() {
            let mut response = Response::from_string(
                r#"{"response":"REMOTE_OLLAMA_PROVIDER_TEST_OK","done":true}"#,
            )
            .with_status_code(200);
            if let Ok(header) = Header::from_bytes("Content-Type", "application/json") {
                response = response.with_header(header);
            }
            let _ = request.respond(response);
        }
    });

    let endpoint = format!("http://{bind_addr}");
    let doc: AdlDoc = serde_yaml::from_str(&format!(
        r#"
version: "0.5"
providers:
  remote:
    type: ollama
    base_url: "{endpoint}"
    default_model: phi4-mini
agents:
  a1:
    provider: remote
    model: phi4-mini
tasks:
  t1:
    prompt:
      user: hello
run:
  workflow:
    kind: sequential
    steps:
      - agent: a1
        task: t1
"#
    ))
    .expect("parse doc");
    doc.validate().expect("remote ollama validation");
    let provider = build_provider(&doc.providers["remote"], Some("phi4-mini"))
        .expect("build_provider should accept remote ollama");
    let output = provider
        .complete("REMOTE_OLLAMA_PROVIDER_TEST_OK")
        .expect("remote ollama provider should complete over http");
    assert_eq!(output, "REMOTE_OLLAMA_PROVIDER_TEST_OK");

    let _ = handle.join();
}

#[test]
fn validate_rejects_endpoint_on_local_ollama_kind() {
    let doc: AdlDoc = serde_yaml::from_str(
        r#"
version: "0.5"
providers:
  local:
    type: local_ollama
    base_url: "http://127.0.0.1:11434"
agents:
  a1:
    provider: local
    model: phi4-mini
tasks:
  t1:
    prompt:
      user: hello
run:
  workflow:
    kind: sequential
    steps:
      - agent: a1
        task: t1
"#,
    )
    .expect("parse doc");
    let err = doc
        .validate()
        .expect_err("local_ollama with endpoint must fail");
    assert!(
        err.to_string()
            .contains("kind 'local_ollama' is CLI-backed"),
        "{err:#}"
    );
}
