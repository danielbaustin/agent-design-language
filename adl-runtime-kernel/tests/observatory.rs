use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    encode_acip_envelope, serve_control_listener, AdapterKind, AdapterPolicy, AuthorityMode,
    CanonicalIngress, ComponentRegistry, ControlAction, ControlApiPolicy, ControlAuthority,
    ControlCapability, ControlService, ExecutorError, FailureClass, Kernel, KernelExit,
    LifecycleControl, OperationExecutor, OperationRequest, OperationalAdapter, OperationalFactory,
    RuntimeRecorder, SignedControlCommand, TrustedControlKey, ACIP_WEBSOCKET_SCHEMA,
    OBSERVATORY_FEED_SCHEMA, OBSERVATORY_WS_AUTH_SCHEMA, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
    OBSERVATORY_WS_PATH,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use futures::{SinkExt, StreamExt};
use rcgen::{generate_simple_self_signed, CertifiedKey};
use tokio_rustls::rustls::{pki_types::CertificateDer, ClientConfig, RootCertStore};
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{
        client::IntoClientRequest,
        http::{HeaderValue, Request},
        Message,
    },
    Connector, MaybeTlsStream, WebSocketStream,
};

struct FakeLifecycle;

struct EchoExecutor;

struct FailOnceExecutor {
    attempts: AtomicUsize,
}

#[async_trait]
impl LifecycleControl for FakeLifecycle {
    async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
        Ok(KernelExit::Clean)
    }
}

#[async_trait]
impl OperationExecutor for EchoExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        Ok(request.payload.clone())
    }
}

#[async_trait]
impl OperationExecutor for FailOnceExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        if self.attempts.fetch_add(1, Ordering::SeqCst) == 0 {
            return Err(ExecutorError {
                class: FailureClass::Fatal,
                message: "injected dispatch failure".to_owned(),
            });
        }
        Ok(request.payload.clone())
    }
}

#[derive(Clone)]
struct TestService {
    service: Arc<ControlService<FakeLifecycle>>,
    operation: OperationalFactory,
    ingress: CanonicalIngress,
    signing_key: SigningKey,
}

fn service(token: &str) -> TestService {
    service_with_executor(token, Arc::new(EchoExecutor))
}

fn service_with_executor(token: &str, executor: Arc<dyn OperationExecutor>) -> TestService {
    let key = SigningKey::from_bytes(&[42; 32]);
    let authority = ControlAuthority::new(BTreeMap::from([(
        "operator-key".to_owned(),
        TrustedControlKey {
            principal: "operator".to_owned(),
            verifying_key: key.verifying_key(),
            capabilities: BTreeSet::from([ControlCapability::Read]),
        },
    )]));
    let recorder = RuntimeRecorder::new(8);
    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Acip,
            AdapterPolicy {
                capacity: 8,
                max_in_flight: 4,
                shutdown_grace_millis: 1_000,
                max_attempts: 1,
                idempotency_entries: 16,
                authority: AuthorityMode::Internal,
            },
            executor,
        )
        .unwrap(),
    );
    let operation = OperationalFactory::new(adapter, vec![]);
    let ingress = CanonicalIngress::new(
        8,
        recorder.clone(),
        BTreeMap::from([("acip".to_owned(), operation.clone())]),
    );
    let service = Arc::new(
        ControlService::new_with_observatory_config(
            "instance-ws",
            recorder,
            FakeLifecycle,
            authority,
            8,
            ["https://observatory.example.test".to_owned()],
        )
        .with_canonical_ingress(ingress.clone()),
    );
    service.set_observatory_bearer_token(token).unwrap();
    service
        .set_public_base_url("https://observatory.example.test:20997")
        .unwrap();
    TestService {
        service,
        operation,
        ingress,
        signing_key: key,
    }
}

async fn websocket_server(
    test_service: TestService,
) -> (
    std::net::SocketAddr,
    Connector,
    tokio::task::JoinHandle<Result<(), adl_runtime_kernel::ControlApiError>>,
) {
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(["localhost".to_owned()]).unwrap();
    let tls = axum_server::tls_rustls::RustlsConfig::from_pem(
        cert.pem().as_bytes().to_vec(),
        signing_key.serialize_pem().into_bytes(),
    )
    .await
    .unwrap();
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(cert.der().to_vec()))
        .unwrap();
    let connector = Connector::Rustls(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let mut registry = ComponentRegistry::new();
    registry.register(test_service.operation);
    registry.register(test_service.ingress);
    let kernel = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(8))
        .start()
        .await
        .unwrap();
    let server = tokio::spawn(async move {
        let result = serve_control_listener(
            test_service.service,
            listener,
            tls,
            ControlApiPolicy::new(
                Duration::from_secs(2),
                Duration::from_secs(5),
                Duration::from_secs(1),
                64 * 1024,
            )
            .unwrap(),
        )
        .await;
        let _ = kernel.shutdown(Duration::from_secs(1)).await;
        result
    });
    (address, connector, server)
}

fn request(address: std::net::SocketAddr, origin: &str) -> Request<()> {
    let mut request = format!("wss://localhost:{}{}", address.port(), OBSERVATORY_WS_PATH)
        .into_client_request()
        .unwrap();
    request
        .headers_mut()
        .insert("Origin", HeaderValue::from_str(origin).unwrap());
    request
}

type TestSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

async fn connect_authenticated(
    address: std::net::SocketAddr,
    connector: Connector,
    token: &str,
) -> TestSocket {
    let mut socket = connect_public(address, connector).await;
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let authenticated =
        next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(authenticated["status"], "authenticated");
    socket
}

async fn connect_public(address: std::net::SocketAddr, connector: Connector) -> TestSocket {
    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    let feed = next_json_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(feed["runtime_instance_id"], "instance-ws");
    socket
}

async fn next_json_with_schema(socket: &mut TestSocket, schema: &str) -> serde_json::Value {
    tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(message) = socket.next().await {
            if let Ok(Message::Text(payload)) = message {
                let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
                if value["schema"] == schema {
                    return value;
                }
            }
        }
        panic!("Observatory session ended before {schema}");
    })
    .await
    .unwrap()
}

async fn next_acip_status(socket: &mut TestSocket) -> serde_json::Value {
    tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(message) = socket.next().await {
            if let Ok(Message::Text(payload)) = message {
                let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
                if value["schema"] == ACIP_WEBSOCKET_SCHEMA {
                    return value;
                }
            }
        }
        panic!("authenticated Observatory session ended before ACIP status");
    })
    .await
    .unwrap()
}

#[tokio::test]
async fn observatory_websocket_allows_public_reads_and_requires_login_for_writes() {
    let token = "test-observatory-websocket-token-0001";
    let test_service = service(token);
    let signing_key = test_service.signing_key.clone();
    let (address, connector, server) = websocket_server(test_service).await;

    let denied = connect_async_tls_with_config(
        request(address, "https://denied.example.test"),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap_err();
    assert!(denied.to_string().contains("403"));

    let native_request = format!("wss://localhost:{}{}", address.port(), OBSERVATORY_WS_PATH)
        .into_client_request()
        .unwrap();
    let (mut native_socket, _) =
        connect_async_tls_with_config(native_request, None, false, Some(connector.clone()))
            .await
            .unwrap();
    let native_feed = next_json_with_schema(&mut native_socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(native_feed["runtime_instance_id"], "instance-ws");
    native_socket.close(None).await.unwrap();

    let mut socket = connect_public(address, connector).await;
    let command = SignedControlCommand::sign(
        "login-command",
        "0123456789abcdef0123456789abcdef",
        "instance-ws",
        "operator",
        ControlAction::Snapshot,
        "operator-key",
        &signing_key,
    )
    .unwrap();
    socket
        .send(Message::Text(
            serde_json::to_string(&command).unwrap().into(),
        ))
        .await
        .unwrap();
    let rejected = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["error"], "write_authentication_required");

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let authenticated =
        next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(authenticated["status"], "authenticated");

    socket
        .send(Message::Text(
            serde_json::to_string(&command).unwrap().into(),
        ))
        .await
        .unwrap();
    let accepted = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted");
    assert_eq!(accepted["response"]["outcome"]["result"], "snapshot");
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_rejects_acip_replay_after_reconnect() {
    let token = "test-observatory-websocket-token-0007";
    let (address, connector, server) = websocket_server(service(token)).await;
    let mut first = connect_authenticated(address, connector.clone(), token).await;
    first
        .send(Message::Binary(
            encode_acip_envelope(
                "acip-reconnect-1",
                "agent-source",
                "runtime-target",
                "acip",
                &serde_json::json!({"message": "first"}),
                1,
            )
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();
    let accepted = next_acip_status(&mut first).await;
    assert_eq!(accepted["status"], "completed");
    assert_eq!(accepted["message_id"], "acip-reconnect-1");
    assert_eq!(accepted["sequence_reserved"], true);
    first.close(None).await.unwrap();

    let mut second = connect_authenticated(address, connector, token).await;
    second
        .send(Message::Binary(
            encode_acip_envelope(
                "acip-reconnect-1-replay",
                "agent-source",
                "runtime-target",
                "acip",
                &serde_json::json!({"message": "replayed"}),
                1,
            )
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();
    let replayed = next_acip_status(&mut second).await;
    assert_eq!(replayed["status"], "rejected");
    assert_eq!(replayed["message_id"], "acip-reconnect-1-replay");
    assert_eq!(replayed["reason"], "monotonic_sequence_must_advance");
    assert_eq!(replayed["sequence_reserved"], false);

    second
        .send(Message::Binary(
            encode_acip_envelope(
                "acip-reconnect-2",
                "agent-source",
                "runtime-target",
                "acip",
                &serde_json::json!({"message": "second"}),
                2,
            )
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();
    let advanced = next_acip_status(&mut second).await;
    assert_eq!(advanced["status"], "completed");
    assert_eq!(advanced["message_id"], "acip-reconnect-2");
    assert_eq!(advanced["sequence_reserved"], true);
    server.abort();
}

#[tokio::test]
async fn failed_acip_dispatch_releases_sequence_for_retry() {
    let token = "test-observatory-websocket-token-0008";
    let service = service_with_executor(
        token,
        Arc::new(FailOnceExecutor {
            attempts: AtomicUsize::new(0),
        }),
    );
    let (address, connector, server) = websocket_server(service).await;
    let mut socket = connect_authenticated(address, connector, token).await;
    let frame = encode_acip_envelope(
        "acip-retry-1",
        "agent-source-retry",
        "runtime-target",
        "acip",
        &serde_json::json!({"message": "retry me"}),
        1,
    )
    .unwrap();

    socket
        .send(Message::Binary(frame.clone().into()))
        .await
        .unwrap();
    let failed = next_acip_status(&mut socket).await;
    assert_eq!(failed["status"], "rejected");
    assert_eq!(failed["sequence_reserved"], false);

    let retry = encode_acip_envelope(
        "acip-retry-2",
        "agent-source-retry",
        "runtime-target",
        "acip",
        &serde_json::json!({"message": "retry after failure"}),
        1,
    )
    .unwrap();
    socket.send(Message::Binary(retry.into())).await.unwrap();
    let retried = next_acip_status(&mut socket).await;
    assert_eq!(retried["status"], "completed");
    assert_eq!(retried["message_id"], "acip-retry-2");
    assert_eq!(retried["sequence_reserved"], true);
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_rejects_bad_auth_and_client_data() {
    let token = "test-observatory-websocket-token-0002";
    let (address, connector, server) = websocket_server(service(token)).await;
    let mut socket = connect_public(address, connector.clone()).await;
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": "invalid-observatory-token-0000000",
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let rejected = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["error"], "authentication_failed");
    let feed = next_json_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(feed["runtime_instance_id"], "instance-ws");

    socket
        .send(Message::Text("{not-json".into()))
        .await
        .unwrap();
    let malformed = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(malformed["status"], "rejected");
    assert_eq!(malformed["error"], "write_authentication_required");

    let mut socket = connect_public(address, connector).await;
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let _ = socket.next().await;
    socket
        .send(Message::Binary(vec![1, 2, 3].into()))
        .await
        .unwrap();
    let rejected = tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(message) = socket.next().await {
            if let Ok(Message::Text(payload)) = message {
                let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
                if value["schema"] == ACIP_WEBSOCKET_SCHEMA {
                    return value;
                }
            }
        }
        panic!("authenticated Observatory session ended before ACIP rejection");
    })
    .await
    .unwrap();
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["sequence_reserved"], false);
    socket.send(Message::Ping(Vec::new().into())).await.unwrap();
    let pong = tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(message) = socket.next().await {
            if matches!(message, Ok(Message::Pong(_))) {
                return true;
            }
        }
        false
    })
    .await
    .unwrap();
    assert!(pong);
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_rejects_a_token_after_rotation() {
    let token = "test-observatory-websocket-token-0003";
    let service = service(token);
    let (address, connector, server) = websocket_server(service.clone()).await;
    let mut socket = connect_public(address, connector).await;
    service
        .service
        .set_observatory_bearer_token("rotated-observatory-websocket-token-0004")
        .unwrap();
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let rejected = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["error"], "authentication_failed");
    let feed = next_json_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(feed["runtime_instance_id"], "instance-ws");
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_revokes_an_authenticated_session_after_rotation() {
    let token = "test-observatory-websocket-token-0005";
    let service = service(token);
    let (address, connector, server) = websocket_server(service.clone()).await;
    let mut socket = connect_authenticated(address, connector, token).await;
    service
        .service
        .set_observatory_bearer_token("rotated-observatory-websocket-token-0006")
        .unwrap();
    let revoked = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(revoked["status"], "rejected");
    assert_eq!(revoked["error"], "credential_revoked");
    let feed = next_json_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(feed["runtime_instance_id"], "instance-ws");
    server.abort();
}
