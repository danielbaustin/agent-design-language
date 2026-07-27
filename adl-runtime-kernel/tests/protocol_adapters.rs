use std::{
    collections::{BTreeSet, VecDeque},
    env,
    sync::{Arc, Mutex as StdMutex},
    time::Duration,
};

use adl_runtime_kernel::{
    build_production_operation_executors, build_protocol_production_operation_executors,
    AdapterKind, AdapterPolicy, AuthorityMode, ExecutionPermit, FailureClass, OperationError,
    OperationExecutor, OperationRequest, OperationalAdapter, ProtocolAdapter, ProtocolEndpoint,
    ProtocolFrame, ProtocolResponse, ProtocolSecret, ProtocolSecurity, ProtocolStatus,
    MAX_PROTOCOL_FRAME_FRESHNESS_MILLIS, MAX_PROTOCOL_RESPONSE_BYTES, OPERATION_REQUEST_SCHEMA,
};
use ed25519_dalek::SigningKey;
use rcgen::{generate_simple_self_signed, CertifiedKey};
use tempfile::TempDir;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::{Mutex, Notify},
};
use tokio_rustls::{
    rustls::{
        pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
        ClientConfig, RootCertStore, ServerConfig,
    },
    TlsAcceptor,
};
use tokio_util::sync::CancellationToken;

static ENV_LOCK: StdMutex<()> = StdMutex::new(());

fn operation(id: &str, payload: &[u8]) -> OperationRequest {
    OperationRequest {
        schema: OPERATION_REQUEST_SCHEMA.to_owned(),
        request_id: id.to_owned(),
        idempotency_key: format!("idempotency-{id}"),
        principal: "agent-alpha".to_owned(),
        payload: payload.to_vec(),
        permit: None,
    }
}

fn operation_with_permit(id: &str, payload: &[u8], key: &SigningKey) -> OperationRequest {
    let mut request = operation(id, payload);
    request.permit = Some(
        ExecutionPermit {
            permit_id: format!("permit-{id}"),
            request_hash: blake3::hash(id.as_bytes()).to_hex().to_string(),
            request_id: id.to_owned(),
            principal: request.principal.clone(),
            action: "provider.invoke".to_owned(),
            resource: "provider".to_owned(),
            units: 1,
            payload_hash: blake3::hash(payload).to_hex().to_string(),
            policy_hash: blake3::hash(b"policy").to_hex().to_string(),
            evidence_hash: blake3::hash(b"evidence").to_hex().to_string(),
            signing_key_id: "permit-key".to_owned(),
            signature: String::new(),
        }
        .sign(key)
        .unwrap(),
    );
    request
}

fn endpoint(
    address: std::net::SocketAddr,
    kind: AdapterKind,
    secret: ProtocolSecret,
) -> ProtocolEndpoint {
    ProtocolEndpoint {
        address,
        security: ProtocolSecurity::PlainForLocalTest,
        timeout: Duration::from_millis(250),
        frame_freshness: Duration::from_millis(250),
        secret,
        capabilities: BTreeSet::from([kind.service_name().to_owned()]),
    }
}

fn policy() -> AdapterPolicy {
    AdapterPolicy {
        capacity: 8,
        max_in_flight: 2,
        shutdown_grace_millis: 500,
        max_attempts: 3,
        idempotency_entries: 16,
        authority: AuthorityMode::External,
    }
}

async fn spawn_peer(
    secret: ProtocolSecret,
    responses: Vec<ProtocolStatus>,
    delay: Duration,
) -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let remaining = Arc::new(Mutex::new(VecDeque::from(responses)));
    let seen = Arc::new(Mutex::new(BTreeSet::<String>::new()));
    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let secret = secret.clone();
            let remaining = remaining.clone();
            let seen = seen.clone();
            tokio::spawn(async move {
                handle_peer_stream(stream, secret, remaining, seen, delay).await;
            });
        }
    });
    address
}

async fn spawn_tampered_peer(secret: ProtocolSecret) -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            handle_tampered_peer_stream(stream, secret).await;
        }
    });
    address
}

async fn spawn_expiring_peer(secret: ProtocolSecret, delay: Duration) -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            handle_expiring_peer_stream(stream, secret, delay).await;
        }
    });
    address
}

async fn spawn_oversized_peer() -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            if reader.read_line(&mut line).await.unwrap_or_default() == 0 {
                return;
            }
            let mut stream = reader.into_inner();
            let oversized = vec![b'x'; MAX_PROTOCOL_RESPONSE_BYTES + 1];
            let _ = stream.write_all(&oversized).await;
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });
    address
}

async fn spawn_hanging_peer() -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            tokio::spawn(async move {
                let _stream = stream;
                tokio::time::sleep(Duration::from_secs(5)).await;
            });
        }
    });
    address
}

async fn spawn_read_then_hang_peer() -> (std::net::SocketAddr, Arc<Notify>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let read = Arc::new(Notify::new());
    let read_peer = read.clone();
    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let read_peer = read_peer.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stream);
                let mut line = String::new();
                if reader.read_line(&mut line).await.unwrap_or_default() == 0 {
                    return;
                }
                read_peer.notify_one();
                tokio::time::sleep(Duration::from_secs(5)).await;
            });
        }
    });
    (address, read)
}

async fn spawn_tls_peer(
    secret: ProtocolSecret,
    responses: Vec<ProtocolStatus>,
) -> (std::net::SocketAddr, ProtocolSecurity) {
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(["localhost".to_owned()]).unwrap();
    let certificate = CertificateDer::from(cert.der().to_vec());
    let server = Arc::new(
        ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                vec![certificate.clone()],
                PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(signing_key.serialize_der())),
            )
            .unwrap(),
    );
    let mut roots = RootCertStore::empty();
    roots.add(certificate).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let remaining = Arc::new(Mutex::new(VecDeque::from(responses)));
    let seen = Arc::new(Mutex::new(BTreeSet::<String>::new()));
    tokio::spawn(async move {
        let acceptor = TlsAcceptor::from(server);
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let acceptor = acceptor.clone();
            let secret = secret.clone();
            let remaining = remaining.clone();
            let seen = seen.clone();
            tokio::spawn(async move {
                if let Ok(stream) = acceptor.accept(stream).await {
                    handle_peer_stream(stream, secret, remaining, seen, Duration::ZERO).await;
                }
            });
        }
    });
    (
        address,
        ProtocolSecurity::RustlsClient {
            config: Arc::new(
                ClientConfig::builder()
                    .with_root_certificates(roots)
                    .with_no_client_auth(),
            ),
            server_name: "localhost".to_owned(),
        },
    )
}

async fn handle_peer_stream<S>(
    stream: S,
    secret: ProtocolSecret,
    remaining: Arc<Mutex<VecDeque<ProtocolStatus>>>,
    seen: Arc<Mutex<BTreeSet<String>>>,
    delay: Duration,
) where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    if reader.read_line(&mut line).await.unwrap_or_default() == 0 {
        return;
    }
    let frame = serde_json::from_str::<ProtocolFrame>(&line);
    let (status, nonce, frame) = match frame {
        Ok(frame) if frame.verify(&secret) => {
            if seen.lock().await.contains(&frame.nonce) {
                (ProtocolStatus::Fatal, None, frame)
            } else {
                let status = remaining
                    .lock()
                    .await
                    .pop_front()
                    .unwrap_or(ProtocolStatus::Ok);
                let nonce = frame.nonce.clone();
                (status, Some(nonce), frame)
            }
        }
        Ok(frame) => (ProtocolStatus::Unauthorized, None, frame),
        Err(_) => return,
    };
    if status == ProtocolStatus::Ok {
        if let Some(nonce) = nonce {
            seen.lock().await.insert(nonce);
        }
    }
    tokio::time::sleep(delay).await;
    let response = ProtocolResponse::signed(
        &secret,
        &frame,
        status,
        b"peer-response",
        (status != ProtocolStatus::Ok).then(|| "peer rejected".to_owned()),
    );
    let mut stream = reader.into_inner();
    let mut bytes = serde_json::to_vec(&response).unwrap();
    bytes.push(b'\n');
    let _ = stream.write_all(&bytes).await;
}

async fn handle_expiring_peer_stream<S>(stream: S, secret: ProtocolSecret, delay: Duration)
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    if reader.read_line(&mut line).await.unwrap_or_default() == 0 {
        return;
    }
    let Ok(frame) = serde_json::from_str::<ProtocolFrame>(&line) else {
        return;
    };
    tokio::time::sleep(delay).await;
    let status = if frame.verify(&secret) {
        ProtocolStatus::Ok
    } else {
        ProtocolStatus::Unauthorized
    };
    let response = ProtocolResponse::signed(
        &secret,
        &frame,
        status,
        b"peer-response",
        (status != ProtocolStatus::Ok).then(|| "peer rejected".to_owned()),
    );
    let mut stream = reader.into_inner();
    let mut bytes = serde_json::to_vec(&response).unwrap();
    bytes.push(b'\n');
    let _ = stream.write_all(&bytes).await;
}

async fn handle_tampered_peer_stream<S>(stream: S, secret: ProtocolSecret)
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    if reader.read_line(&mut line).await.unwrap_or_default() == 0 {
        return;
    }
    let Ok(frame) = serde_json::from_str::<ProtocolFrame>(&line) else {
        return;
    };
    let mut response =
        ProtocolResponse::signed(&secret, &frame, ProtocolStatus::Ok, b"peer-response", None);
    response.payload_hex = hex::encode(b"tampered-response");
    let mut stream = reader.into_inner();
    let mut bytes = serde_json::to_vec(&response).unwrap();
    bytes.push(b'\n');
    let _ = stream.write_all(&bytes).await;
}

#[tokio::test]
async fn provider_dispatch_uses_real_authenticated_transport_and_caches_replay_result() {
    let secret = ProtocolSecret::from_key([7; 32]);
    let address = spawn_peer(secret.clone(), vec![ProtocolStatus::Ok], Duration::ZERO).await;
    let provider = ProtocolAdapter::new(
        AdapterKind::Provider,
        endpoint(address, AdapterKind::Provider, secret),
        CancellationToken::new(),
    )
    .unwrap();

    let payload = provider
        .execute(&operation("provider-ok", b"dispatch"))
        .await
        .unwrap();
    assert_eq!(payload, b"peer-response");
    let replay = provider
        .execute(&operation("provider-ok", b"dispatch"))
        .await
        .unwrap();
    assert_eq!(replay, b"peer-response");
}

#[test]
fn endpoint_validation_rejects_plaintext_nonloopback_zero_timeout_and_missing_capability() {
    let secret = ProtocolSecret::from_key([19; 32]);
    let nonloopback = ProtocolEndpoint {
        address: "192.0.2.1:443".parse().unwrap(),
        ..endpoint(
            "127.0.0.1:1".parse().unwrap(),
            AdapterKind::Provider,
            secret.clone(),
        )
    };
    assert!(
        ProtocolAdapter::new(AdapterKind::Provider, nonloopback, CancellationToken::new()).is_err()
    );

    let zero_timeout = ProtocolEndpoint {
        timeout: Duration::ZERO,
        ..endpoint(
            "127.0.0.1:1".parse().unwrap(),
            AdapterKind::Provider,
            secret.clone(),
        )
    };
    assert!(ProtocolAdapter::new(
        AdapterKind::Provider,
        zero_timeout,
        CancellationToken::new()
    )
    .is_err());

    let missing_capability = ProtocolEndpoint {
        capabilities: BTreeSet::from(["agent_runtime".to_owned()]),
        ..endpoint(
            "127.0.0.1:1".parse().unwrap(),
            AdapterKind::Provider,
            secret,
        )
    };
    assert!(ProtocolAdapter::new(
        AdapterKind::Provider,
        missing_capability,
        CancellationToken::new()
    )
    .is_err());
}

#[tokio::test]
async fn protocol_frame_freshness_is_mac_bound_and_peer_verifiable() {
    let secret = ProtocolSecret::from_key([17; 32]);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let adapter = ProtocolAdapter::new(
        AdapterKind::Provider,
        endpoint(address, AdapterKind::Provider, secret.clone()),
        CancellationToken::new(),
    )
    .unwrap();
    let permit_key = SigningKey::from_bytes(&[41; 32]);
    let client = tokio::spawn(async move {
        adapter
            .execute(&operation_with_permit(
                "freshness-ok",
                b"dispatch",
                &permit_key,
            ))
            .await
            .unwrap()
    });
    let (stream, _) = listener.accept().await.unwrap();
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    assert!(reader.read_line(&mut line).await.unwrap() > 0);
    let frame = serde_json::from_str::<ProtocolFrame>(&line).unwrap();
    assert!(frame.verify(&secret));
    assert!(!frame.challenge.is_empty());
    assert!(frame.permit.is_some());
    assert!(frame.nonce.ends_with(&frame.challenge));
    assert!(frame.expires_unix_millis > frame.issued_unix_millis);
    assert!(
        frame.expires_unix_millis - frame.issued_unix_millis <= MAX_PROTOCOL_FRAME_FRESHNESS_MILLIS
    );
    assert!(!frame.verify_at(&secret, frame.expires_unix_millis + 1));
    let mut tampered = frame.clone();
    tampered.expires_unix_millis += 1;
    assert!(!tampered.verify_at(&secret, frame.issued_unix_millis));
    let mut tampered_permit = frame.clone();
    tampered_permit.permit.as_mut().unwrap().payload_hash =
        blake3::hash(b"other").to_hex().to_string();
    assert!(!tampered_permit.verify_at(&secret, frame.issued_unix_millis));

    let response =
        ProtocolResponse::signed(&secret, &frame, ProtocolStatus::Ok, b"peer-response", None);
    let mut stream = reader.into_inner();
    let mut bytes = serde_json::to_vec(&response).unwrap();
    bytes.push(b'\n');
    stream.write_all(&bytes).await.unwrap();
    assert_eq!(client.await.unwrap(), b"peer-response");
}

#[tokio::test]
async fn stale_protocol_frame_is_rejected_without_local_replay_state() {
    let secret = ProtocolSecret::from_key([18; 32]);
    let address = spawn_expiring_peer(secret.clone(), Duration::from_millis(80)).await;
    let adapter = ProtocolAdapter::new(
        AdapterKind::Provider,
        ProtocolEndpoint {
            timeout: Duration::from_millis(500),
            frame_freshness: Duration::from_millis(20),
            ..endpoint(address, AdapterKind::Provider, secret)
        },
        CancellationToken::new(),
    )
    .unwrap();
    let error = adapter
        .execute(&operation("stale-frame", b"dispatch"))
        .await
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Fatal);
    assert!(error.message.contains("unauthorized"));
}

#[tokio::test]
async fn response_tamper_and_concurrent_replay_are_rejected() {
    let tamper_secret = ProtocolSecret::from_key([11; 32]);
    let tamper_address = spawn_tampered_peer(tamper_secret.clone()).await;
    let tamper_adapter = ProtocolAdapter::new(
        AdapterKind::Provider,
        endpoint(tamper_address, AdapterKind::Provider, tamper_secret),
        CancellationToken::new(),
    )
    .unwrap();
    let tamper = tamper_adapter
        .execute(&operation("tampered-response", b"dispatch"))
        .await
        .unwrap_err();
    assert_eq!(tamper.class, FailureClass::Fatal);
    assert!(tamper.message.contains("mac mismatch"));

    let race_secret = ProtocolSecret::from_key([12; 32]);
    let race_address = spawn_peer(
        race_secret.clone(),
        vec![ProtocolStatus::Ok],
        Duration::from_millis(25),
    )
    .await;
    let adapter = ProtocolAdapter::new(
        AdapterKind::A2a,
        endpoint(race_address, AdapterKind::A2a, race_secret),
        CancellationToken::new(),
    )
    .unwrap();
    let first = adapter.clone();
    let second = adapter.clone();
    let request_a = operation("concurrent-replay", b"message");
    let request_b = operation("concurrent-replay", b"message");
    let (left, right) = tokio::join!(first.execute(&request_a), second.execute(&request_b));
    let successes = [left.as_ref().ok(), right.as_ref().ok()]
        .into_iter()
        .flatten()
        .count();
    let replay_rejections = [left.as_ref().err(), right.as_ref().err()]
        .into_iter()
        .flatten()
        .filter(|error| {
            error.class == FailureClass::Fatal && error.message.contains("replay rejected")
        })
        .count();
    assert_eq!(successes, 1);
    assert_eq!(replay_rejections, 1);
}

#[tokio::test]
async fn timeout_after_write_preserves_unknown_outcome_and_blocks_duplicate_side_effects() {
    let secret = ProtocolSecret::from_key([14; 32]);
    let (address, read) = spawn_read_then_hang_peer().await;
    let adapter = ProtocolAdapter::new(
        AdapterKind::Provider,
        ProtocolEndpoint {
            timeout: Duration::from_millis(100),
            ..endpoint(address, AdapterKind::Provider, secret)
        },
        CancellationToken::new(),
    )
    .unwrap();
    let request = operation("timeout-after-write", b"dispatch");

    let first_task = {
        let adapter = adapter.clone();
        let request = request.clone();
        tokio::spawn(async move { adapter.execute(&request).await })
    };
    read.notified().await;
    let first = first_task.await.unwrap().unwrap_err();
    assert_eq!(first.class, FailureClass::Retryable);
    assert!(first.message.contains("timed out"));

    let duplicate = adapter.execute(&request).await.unwrap_err();
    assert_eq!(duplicate.class, FailureClass::Fatal);
    assert!(duplicate.message.contains("outcome unknown after write"));
}

#[tokio::test]
async fn oversized_non_newline_response_fails_closed() {
    let secret = ProtocolSecret::from_key([13; 32]);
    let address = spawn_oversized_peer().await;
    let adapter = ProtocolAdapter::new(
        AdapterKind::Provider,
        endpoint(address, AdapterKind::Provider, secret),
        CancellationToken::new(),
    )
    .unwrap();

    let error = adapter
        .execute(&operation("oversized-response", b"dispatch"))
        .await
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Fatal);
    assert!(error.message.contains("byte limit"));
}

#[tokio::test]
async fn acip_and_a2a_exchange_authenticated_frames() {
    for kind in [AdapterKind::Acip, AdapterKind::A2a] {
        let secret = ProtocolSecret::from_key([kind as u8 + 1; 32]);
        let address = spawn_peer(secret.clone(), vec![ProtocolStatus::Ok], Duration::ZERO).await;
        let adapter = ProtocolAdapter::new(
            kind,
            endpoint(address, kind, secret),
            CancellationToken::new(),
        )
        .unwrap();

        let payload = adapter
            .execute(&operation(kind.service_name(), b"bidirectional-message"))
            .await
            .unwrap();
        assert_eq!(payload, b"peer-response");
    }
}

#[tokio::test]
async fn unauthorized_malformed_timeout_retry_and_shutdown_fail_closed() {
    let peer_secret = ProtocolSecret::from_key([1; 32]);
    let client_secret = ProtocolSecret::from_key([2; 32]);
    let unauthorized = spawn_peer(peer_secret, vec![ProtocolStatus::Ok], Duration::ZERO).await;
    let adapter = ProtocolAdapter::new(
        AdapterKind::CloudBridge,
        endpoint(unauthorized, AdapterKind::CloudBridge, client_secret),
        CancellationToken::new(),
    )
    .unwrap();
    assert_eq!(
        adapter
            .execute(&operation("unauthorized", b"cloud"))
            .await
            .unwrap_err()
            .class,
        FailureClass::Fatal
    );

    let secret = ProtocolSecret::from_key([3; 32]);
    let retry_peer = spawn_peer(
        secret.clone(),
        vec![ProtocolStatus::Retryable, ProtocolStatus::Ok],
        Duration::ZERO,
    )
    .await;
    let executor = ProtocolAdapter::new(
        AdapterKind::Provider,
        endpoint(retry_peer, AdapterKind::Provider, secret),
        CancellationToken::new(),
    )
    .unwrap();
    let bounded = OperationalAdapter::new(AdapterKind::Provider, policy(), executor).unwrap();
    let retried = bounded
        .invoke(operation("retry-then-ok", b"provider"))
        .await
        .unwrap();
    assert_eq!(retried.attempts, 2);
    assert_eq!(retried.payload, b"peer-response");

    let timeout_secret = ProtocolSecret::from_key([4; 32]);
    let timeout_peer = spawn_hanging_peer().await;
    let timeout_executor = ProtocolAdapter::new(
        AdapterKind::A2a,
        ProtocolEndpoint {
            timeout: Duration::from_millis(20),
            ..endpoint(timeout_peer, AdapterKind::A2a, timeout_secret)
        },
        CancellationToken::new(),
    )
    .unwrap();
    let bounded_timeout =
        OperationalAdapter::new(AdapterKind::A2a, policy(), timeout_executor).unwrap();
    let timeout_error = bounded_timeout
        .invoke(operation("timeout", b"message"))
        .await
        .unwrap_err();
    assert!(
        matches!(timeout_error, OperationError::Fatal(ref message) if message.contains("outcome unknown after write")),
        "unexpected timeout error: {timeout_error:?}"
    );

    let shutdown_secret = ProtocolSecret::from_key([5; 32]);
    let shutdown_peer = spawn_peer(
        shutdown_secret.clone(),
        vec![ProtocolStatus::Ok],
        Duration::ZERO,
    )
    .await;
    let token = CancellationToken::new();
    let shutdown = ProtocolAdapter::new(
        AdapterKind::Acip,
        endpoint(shutdown_peer, AdapterKind::Acip, shutdown_secret),
        token.clone(),
    )
    .unwrap();
    shutdown.shutdown();
    let error = shutdown
        .execute(&operation("after-shutdown", b"acip"))
        .await
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Fatal);
    assert!(error.message.contains("shut down"));
}

#[tokio::test]
async fn cloud_bridge_capability_and_rustls_boundary_are_explicit() {
    let secret = ProtocolSecret::from_key([9; 32]);
    let address = spawn_peer(secret.clone(), vec![ProtocolStatus::Ok], Duration::ZERO).await;
    let missing = ProtocolEndpoint {
        capabilities: BTreeSet::from(["provider".to_owned()]),
        ..endpoint(address, AdapterKind::CloudBridge, secret.clone())
    };
    assert!(
        ProtocolAdapter::new(AdapterKind::CloudBridge, missing, CancellationToken::new()).is_err()
    );

    let config = ClientConfig::builder()
        .with_root_certificates(RootCertStore::empty())
        .with_no_client_auth();
    let tls_endpoint = ProtocolEndpoint {
        security: ProtocolSecurity::RustlsClient {
            config: Arc::new(config),
            server_name: "localhost".to_owned(),
        },
        ..endpoint(address, AdapterKind::CloudBridge, secret)
    };
    let adapter = ProtocolAdapter::new(
        AdapterKind::CloudBridge,
        tls_endpoint,
        CancellationToken::new(),
    )
    .unwrap();
    let error = adapter
        .execute(&operation("tls-to-plain-peer", b"cloud"))
        .await
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Retryable);

    let tls_secret = ProtocolSecret::from_key([10; 32]);
    let (tls_address, security) =
        spawn_tls_peer(tls_secret.clone(), vec![ProtocolStatus::Ok]).await;
    let adapter = ProtocolAdapter::new(
        AdapterKind::CloudBridge,
        ProtocolEndpoint {
            security,
            ..endpoint(tls_address, AdapterKind::CloudBridge, tls_secret)
        },
        CancellationToken::new(),
    )
    .unwrap();
    assert_eq!(
        adapter
            .execute(&operation("tls-cloud-ok", b"cloud"))
            .await
            .unwrap(),
        b"peer-response"
    );
}

#[test]
fn production_builder_returns_no_partial_executors_when_protocol_config_is_missing() {
    let _guard = ENV_LOCK.lock().unwrap();
    let keys = [
        "ADL_RUNTIME_PROVIDER_ENDPOINT",
        "ADL_RUNTIME_PROVIDER_SECRET_FILE",
        "ADL_RUNTIME_PROVIDER_CA_DER_FILE",
        "ADL_RUNTIME_PROVIDER_SERVER_NAME",
        "ADL_RUNTIME_PROVIDER_TIMEOUT_MILLIS",
        "ADL_RUNTIME_PROVIDER_FRESHNESS_MILLIS",
        "ADL_RUNTIME_PROVIDER_CAPABILITIES",
        "ADL_RUNTIME_ACIP_ENDPOINT",
        "ADL_RUNTIME_ACIP_SECRET_FILE",
        "ADL_RUNTIME_ACIP_CA_DER_FILE",
        "ADL_RUNTIME_ACIP_SERVER_NAME",
        "ADL_RUNTIME_ACIP_TIMEOUT_MILLIS",
        "ADL_RUNTIME_ACIP_FRESHNESS_MILLIS",
        "ADL_RUNTIME_ACIP_CAPABILITIES",
        "ADL_RUNTIME_A2A_ENDPOINT",
        "ADL_RUNTIME_A2A_SECRET_FILE",
        "ADL_RUNTIME_A2A_CA_DER_FILE",
        "ADL_RUNTIME_A2A_SERVER_NAME",
        "ADL_RUNTIME_A2A_TIMEOUT_MILLIS",
        "ADL_RUNTIME_A2A_FRESHNESS_MILLIS",
        "ADL_RUNTIME_A2A_CAPABILITIES",
        "ADL_RUNTIME_CLOUD_BRIDGE_ENDPOINT",
        "ADL_RUNTIME_CLOUD_BRIDGE_SECRET_FILE",
        "ADL_RUNTIME_CLOUD_BRIDGE_CA_DER_FILE",
        "ADL_RUNTIME_CLOUD_BRIDGE_SERVER_NAME",
        "ADL_RUNTIME_CLOUD_BRIDGE_TIMEOUT_MILLIS",
        "ADL_RUNTIME_CLOUD_BRIDGE_FRESHNESS_MILLIS",
        "ADL_RUNTIME_CLOUD_BRIDGE_CAPABILITIES",
    ];
    let previous = keys.map(|key| (key, env::var(key).ok()));
    for key in keys {
        env::remove_var(key);
    }
    let executors = build_protocol_production_operation_executors();
    let root = TempDir::new().unwrap();
    let canonical = build_production_operation_executors(root.path().join("local-state")).unwrap();
    for (key, value) in previous {
        if let Some(value) = value {
            env::set_var(key, value);
        }
    }
    assert!(executors.is_empty());
    assert!(canonical.contains_key(&AdapterKind::Agent));
}
