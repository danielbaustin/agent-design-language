use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    channel, load_control_tls, serve_control_listener, serve_control_listener_until,
    serve_control_listener_until_ready, write_observability_event, write_payload, ClockAuthority,
    ComponentId, ComponentRegistry, ContinuityHead, ControlAction, ControlAuthority,
    ControlCapability, ControlError, ControlExit, ControlObservabilityEvent, ControlOutcome,
    ControlService, DiskWeather, Kernel, KernelExit, LifecycleControl, ObservabilityDegradation,
    ObservabilityHealth, Observation, ResourceState, RuntimeEvent, RuntimeRecorder,
    RuntimeTlsInitConfig, ShutdownDecision, SignedControlCommand, TrustedControlKey, WeatherConfig,
    WeatherHealthReport, WeatherSample,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use rcgen::{generate_simple_self_signed, CertifiedKey};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Notify,
};
use tokio_rustls::{
    rustls::{
        pki_types::{CertificateDer, ServerName},
        ClientConfig, RootCertStore,
    },
    TlsConnector,
};

const TEST_BIND_HOST: &str = "127.0.0.1";

async fn test_https() -> (axum_server::tls_rustls::RustlsConfig, TlsConnector) {
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(["localhost".to_owned(), TEST_BIND_HOST.to_owned()]).unwrap();
    let certificate = cert.pem();
    let server = axum_server::tls_rustls::RustlsConfig::from_pem(
        certificate.as_bytes().to_vec(),
        signing_key.serialize_pem().into_bytes(),
    )
    .await
    .unwrap();
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(cert.der().to_vec()))
        .unwrap();
    let client = TlsConnector::from(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ));
    (server, client)
}

async fn https_request(
    client: &TlsConnector,
    address: std::net::SocketAddr,
    request: &[u8],
) -> String {
    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let mut stream = client
        .connect(ServerName::try_from("localhost").unwrap(), stream)
        .await
        .unwrap();
    stream.write_all(request).await.unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    String::from_utf8(response).unwrap()
}

struct FakeLifecycle {
    calls: Arc<AtomicUsize>,
}

struct BlockingLifecycle {
    calls: Arc<AtomicUsize>,
    started: Arc<Notify>,
    release: Arc<Notify>,
}

#[async_trait]
impl LifecycleControl for BlockingLifecycle {
    async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.started.notify_one();
        self.release.notified().await;
        Ok(KernelExit::Clean)
    }
}

#[async_trait]
impl LifecycleControl for FakeLifecycle {
    async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(KernelExit::Clean)
    }
}

fn authority(
    key: &SigningKey,
    capabilities: impl IntoIterator<Item = ControlCapability>,
) -> ControlAuthority {
    ControlAuthority::new(BTreeMap::from([(
        "operator-key".to_owned(),
        TrustedControlKey {
            principal: "operator".to_owned(),
            verifying_key: key.verifying_key(),
            capabilities: capabilities.into_iter().collect::<BTreeSet<_>>(),
        },
    )]))
}

fn signed(key: &SigningKey, id: &str, action: ControlAction) -> SignedControlCommand {
    let correlation_id = blake3::hash(id.as_bytes()).to_hex()[..32].to_owned();
    SignedControlCommand::sign(
        id,
        correlation_id,
        "instance-1",
        "operator",
        action,
        "operator-key",
        key,
    )
    .unwrap()
}

#[tokio::test]
async fn snapshot_is_revisioned_and_contains_complete_health_state() {
    let recorder = RuntimeRecorder::new(8);
    recorder.set_topology_generation(9);
    recorder.set_component_state(
        ComponentId::new("scheduler"),
        adl_runtime_kernel::RunningState::Degraded,
    );
    recorder.set_restart_count(ComponentId::new("scheduler"), 2);
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "sntp".to_owned(),
        unix_millis: 42,
    });
    let (sender, mut receiver) = channel(1, adl_runtime_kernel::ChannelFullPolicy::Reject);
    sender.send("first").await.unwrap();
    assert!(sender.send("second").await.is_err());
    recorder.set_queue_health("control", &sender.metrics());
    recorder.set_continuity_head(ContinuityHead {
        generation: 4,
        accepted_through: 77,
        topology_hash: "topology".to_owned(),
        config_hash: "config".to_owned(),
        integrity: "manifest-hash".to_owned(),
    });

    let snapshot = recorder.snapshot();
    assert!(snapshot.revision >= 6);
    assert_eq!(snapshot.topology_generation, 9);
    assert_eq!(
        snapshot.components[&ComponentId::new("scheduler")],
        adl_runtime_kernel::RunningState::Degraded
    );
    assert_eq!(snapshot.restart_counts[&ComponentId::new("scheduler")], 2);
    assert_eq!(snapshot.queues["control"].capacity, 1);
    assert_eq!(snapshot.queues["control"].generation, 2);
    assert_eq!(snapshot.queues["control"].depth, 1);
    assert_eq!(snapshot.queues["control"].high_water, 1);
    assert_eq!(snapshot.queues["control"].rejected, 1);
    assert_eq!(snapshot.continuity_head.unwrap().accepted_through, 77);
    assert_eq!(receiver.recv().await, Some("first"));

    let (sender, mut waiting_receiver) = channel(1, adl_runtime_kernel::ChannelFullPolicy::Block);
    let metrics = sender.metrics();
    let waiter = tokio::spawn(async move { waiting_receiver.recv().await });
    tokio::task::yield_now().await;
    sender.send("direct").await.unwrap();
    assert_eq!(waiter.await.unwrap(), Some("direct"));
    assert_eq!(metrics.depth(), 0);
}

#[tokio::test]
async fn forged_and_unauthorized_commands_never_reach_lifecycle_authority() {
    let key = SigningKey::from_bytes(&[3; 32]);
    let calls = Arc::new(AtomicUsize::new(0));
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: calls.clone(),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));
    let shutdown = signed(&key, "stop-1", ControlAction::Shutdown { grace_millis: 5 });
    assert_eq!(
        service.execute(shutdown).await.unwrap_err(),
        ControlError::Unauthorized
    );

    let mut forged = signed(&key, "read-1", ControlAction::Snapshot);
    forged.correlation_id = "correlation-forged".to_owned();
    assert_eq!(
        service.execute(forged).await.unwrap_err(),
        ControlError::Authentication
    );

    let stale_service = Arc::new(ControlService::new(
        "instance-2",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: calls.clone(),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));
    assert_eq!(
        stale_service
            .execute(signed(&key, "read-2", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::StaleRuntimeInstance
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn pressure_admission_gate_refuses_new_commands_until_reopened() {
    let key = SigningKey::from_bytes(&[31; 32]);
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));

    assert!(service.pause_admission_if_idle());
    assert_eq!(
        service
            .execute(signed(&key, "read-paused", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::AdmissionClosed
    );
    service.reopen_admission();
    service
        .execute(signed(&key, "read-open", ControlAction::Snapshot))
        .await
        .unwrap();
}

#[tokio::test]
async fn duplicate_shutdown_executes_once_and_conflicting_reuse_fails() {
    let key = SigningKey::from_bytes(&[4; 32]);
    let calls = Arc::new(AtomicUsize::new(0));
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: calls.clone(),
        },
        authority(&key, [ControlCapability::Stop]),
        4,
    ));
    let command = signed(&key, "stop-1", ControlAction::Shutdown { grace_millis: 5 });
    let first = service.execute(command.clone()).await.unwrap();
    let second = service.execute(command).await.unwrap();
    assert_eq!(first, second);
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    let conflict = signed(&key, "stop-1", ControlAction::Shutdown { grace_millis: 6 });
    assert_eq!(
        service.execute(conflict).await.unwrap_err(),
        ControlError::IdempotencyConflict
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn idempotency_refresh_preserves_the_recent_completed_response() {
    let key = SigningKey::from_bytes(&[10; 32]);
    let recorder = RuntimeRecorder::new(8);
    let service = Arc::new(ControlService::new(
        "instance-1",
        recorder.clone(),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        2,
    ));
    let first_command = signed(&key, "read-first", ControlAction::Snapshot);
    let first = service.execute(first_command.clone()).await.unwrap();
    service
        .execute(signed(&key, "read-second", ControlAction::Snapshot))
        .await
        .unwrap();
    assert_eq!(service.execute(first_command.clone()).await.unwrap(), first);
    service
        .execute(signed(&key, "read-third", ControlAction::Snapshot))
        .await
        .unwrap();
    recorder.emit(None, RuntimeEvent::KernelStarting);
    assert_eq!(service.execute(first_command).await.unwrap(), first);
}

#[tokio::test]
async fn cancelled_client_does_not_cancel_execution_or_exceed_idempotency_bound() {
    let key = SigningKey::from_bytes(&[6; 32]);
    let calls = Arc::new(AtomicUsize::new(0));
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        BlockingLifecycle {
            calls: calls.clone(),
            started: started.clone(),
            release: release.clone(),
        },
        authority(&key, [ControlCapability::Read, ControlCapability::Stop]),
        1,
    ));
    let command = signed(
        &key,
        "stop-cancelled",
        ControlAction::Shutdown { grace_millis: 5 },
    );
    let request = {
        let service = service.clone();
        let command = command.clone();
        tokio::spawn(async move { service.execute(command).await })
    };
    started.notified().await;
    assert!(!service.pause_admission_if_idle());
    request.abort();
    assert_eq!(
        service
            .execute(signed(&key, "read-capacity", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::IdempotencyCapacity
    );
    release.notify_one();
    let response = loop {
        match service.execute(command.clone()).await {
            Ok(response) => break response,
            Err(ControlError::InFlight) => tokio::task::yield_now().await,
            other => panic!("unexpected retry result: {other:?}"),
        }
    };
    assert_eq!(
        response.outcome,
        ControlOutcome::Shutdown {
            exit: ControlExit::Clean
        }
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn axum_adapter_serves_signed_control_payloads() {
    let key = SigningKey::from_bytes(&[7; 32]);
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let (ready_sender, ready_receiver) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(serve_control_listener_until_ready(
        service,
        listener,
        tls,
        ready_sender,
        std::future::pending(),
    ));
    assert_eq!(ready_receiver.await.unwrap(), address);
    let body = serde_json::to_vec(&signed(&key, "read-http", ControlAction::Snapshot)).unwrap();
    let request = format!(
        "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        String::from_utf8(body).unwrap()
    );
    let response = https_request(&client, address, request.as_bytes()).await;
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains(adl_runtime_kernel::CONTROL_RESPONSE_SCHEMA));

    let response = https_request(
        &client,
        address,
        b"POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: 1\r\nConnection: close\r\n\r\n{",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 400 Bad Request"));
    assert!(response.contains("adl.runtime.control_error.v1"));
    server.abort();
}

#[tokio::test]
async fn observatory_https_feed_requires_bearer_and_reports_weather_freshness() {
    let key = SigningKey::from_bytes(&[12; 32]);
    let recorder = RuntimeRecorder::new(8);
    recorder.set_topology_generation(11);
    recorder.set_component_state(
        ComponentId::new("runtime_api"),
        adl_runtime_kernel::RunningState::Running,
    );
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "sntp".to_owned(),
        unix_millis: 1_789_000_000,
    });
    recorder.set_continuity_head(ContinuityHead {
        generation: 3,
        accepted_through: 99,
        topology_hash: "topology-hash".to_owned(),
        config_hash: "config-hash".to_owned(),
        integrity: "snapshot-hash".to_owned(),
    });
    recorder.promote_observability();
    let service = Arc::new(ControlService::new(
        "instance-1",
        recorder,
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));
    let weather_config = WeatherConfig {
        disk_stop_free_bytes: 256,
        disk_warning_free_bytes: 512,
        disk_recover_free_bytes: 1024,
        ..WeatherConfig::default()
    };
    let weather = WeatherHealthReport::from_sample(
        &weather_config,
        WeatherSample {
            platform: "test".to_owned(),
            cpu_basis_points: Observation {
                value: Some(250),
                source: "fixture".to_owned(),
            },
            per_core_basis_points: Observation {
                value: Some(vec![250]),
                source: "fixture".to_owned(),
            },
            memory_total_bytes: Observation {
                value: Some(1024),
                source: "fixture".to_owned(),
            },
            memory_available_bytes: Observation {
                value: Some(768),
                source: "fixture".to_owned(),
            },
            disks: Observation {
                value: Some(vec![DiskWeather {
                    mount: "/".to_owned(),
                    total_bytes: 4096,
                    available_bytes: 2048,
                }]),
                source: "fixture".to_owned(),
            },
            network_received_bytes: Observation {
                value: Some(13),
                source: "fixture".to_owned(),
            },
            network_transmitted_bytes: Observation {
                value: Some(21),
                source: "fixture".to_owned(),
            },
            max_temperature_millicelsius: Observation {
                value: Some(42_000),
                source: "fixture".to_owned(),
            },
            gpus: Observation {
                value: Some(Vec::new()),
                source: "fixture".to_owned(),
            },
        },
        ResourceState::Healthy,
    );
    assert_eq!(weather.shutdown_decision, ShutdownDecision::Continue);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    service.set_weather_stale_after(Duration::from_secs(1));
    service.set_weather_report_at(weather.clone(), now);
    service
        .set_observatory_bearer_token("test-observatory-token-0000000001")
        .unwrap();

    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener(service.clone(), listener, tls));
    let preflight = https_request(
        &client,
        address,
        b"OPTIONS /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://localhost:8765\r\nAccess-Control-Request-Method: GET\r\nAccess-Control-Request-Headers: authorization\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(preflight.starts_with("HTTP/1.1 204 No Content"));
    assert!(preflight.contains("access-control-allow-origin: https://localhost:8765"));
    assert!(preflight.contains("access-control-allow-methods: GET"));
    assert!(preflight.contains("access-control-allow-headers: Authorization"));
    assert!(preflight.contains("cache-control: no-store"));
    let unauthorized = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://localhost:8765\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(unauthorized.starts_with("HTTP/1.1 401 Unauthorized"));
    assert!(unauthorized.contains("cache-control: no-store"));
    let response = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://localhost:8765\r\nAuthorization: Bearer test-observatory-token-0000000001\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("cache-control: no-store"));
    assert!(!response.contains(adl_runtime_kernel::LEGACY_OBSERVATORY_FEED_SCHEMA));
    assert!(response.contains("access-control-allow-origin: https://localhost:8765"));
    assert!(response.contains(adl_runtime_kernel::OBSERVATORY_FEED_SCHEMA));
    assert!(response.contains("\"runtime_selection\":\"runtime_v3_explicit_opt_in\""));
    assert!(response.contains("\"signed_commands_required_for_mutation\":true"));
    assert!(response.contains("\"bearer_token_required_for_read\":true"));
    assert!(response.contains("\"browser_mutation_authority\":false"));
    assert!(response.contains(&format!("\"port\":{}", address.port())));
    assert!(response.contains("\"event\":\"state:Running\""));
    assert!(response.contains("\"event\":\"clock_authority_updated\""));
    assert!(response.contains("\"accepted_through\":99"));
    assert!(response.contains("\"cloudwatch_route\":\"vector.runtime_v3_cloudwatch_emf\""));
    assert!(response.contains("\"runtime_v2_decommission_authorized\":false"));
    assert!(response.contains("\"total_count\":1"));
    assert!(response.contains("\"id\":\"agent-0001\""));
    assert!(response.contains("\"stale\":false"));

    service.set_weather_report_at(weather, 0);
    let stale = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer test-observatory-token-0000000001\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(stale.starts_with("HTTP/1.1 200 OK"));
    assert!(stale.contains("\"stale\":true"));
    server.abort();
}

#[tokio::test]
async fn observatory_feed_reports_large_agent_population_as_bounded_sample() {
    let key = SigningKey::from_bytes(&[14; 32]);
    let service = Arc::new(ControlService::new_with_observatory_config_and_agents(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
        ["https://observatory.example.test".to_owned()],
        adl_runtime_kernel::AgentPopulationFeed {
            total_count: 10_000,
            rendered_sample_count: 2,
            sample: vec![
                adl_runtime_kernel::AgentSample {
                    id: "agent-00001".to_owned(),
                    label: "Runtime agent 1".to_owned(),
                    role: "runtime agent".to_owned(),
                    state: "running".to_owned(),
                    detail: "sample 1 of 10000".to_owned(),
                },
                adl_runtime_kernel::AgentSample {
                    id: "agent-00002".to_owned(),
                    label: "Runtime agent 2".to_owned(),
                    role: "runtime agent".to_owned(),
                    state: "running".to_owned(),
                    detail: "sample 2 of 10000".to_owned(),
                },
            ],
        },
    ));
    let feed = service.observatory_feed();
    assert_eq!(feed.agents.total_count, 10_000);
    assert_eq!(feed.agents.rendered_sample_count, 2);
    assert_eq!(feed.agents.sample.len(), 2);
    assert_eq!(feed.agents.sample[1].id, "agent-00002");
}

#[tokio::test]
async fn observatory_cors_allows_only_configured_origins_and_reports_canonical_port() {
    let key = SigningKey::from_bytes(&[13; 32]);
    let service = Arc::new(ControlService::new_with_observatory_config(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
        ["https://observatory.example.test".to_owned()],
    ));
    service
        .set_observatory_bearer_token("test-observatory-token-0000000002")
        .unwrap();
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener(service, listener, tls));

    let response = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.example.test\r\nAuthorization: Bearer test-observatory-token-0000000002\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("access-control-allow-origin: https://observatory.example.test"));
    assert!(response.contains(&format!("\"port\":{}", address.port())));

    let response = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://other.example.test\r\nAuthorization: Bearer test-observatory-token-0000000002\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(!response.contains("access-control-allow-origin"));

    server.abort();
}

#[tokio::test]
async fn graceful_api_shutdown_drains_an_active_control_response() {
    let key = SigningKey::from_bytes(&[8; 32]);
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        BlockingLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
            started: started.clone(),
            release: release.clone(),
        },
        authority(&key, [ControlCapability::Stop]),
        4,
    ));
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let shutdown = tokio_util::sync::CancellationToken::new();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener_until(
        service,
        listener,
        tls,
        shutdown.clone().cancelled_owned(),
    ));
    let body = serde_json::to_vec(&signed(
        &key,
        "stop-drain",
        ControlAction::Shutdown { grace_millis: 5 },
    ))
    .unwrap();
    let client = tokio::spawn(async move {
        let request = format!(
            "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            String::from_utf8(body).unwrap()
        );
        https_request(&client, address, request.as_bytes()).await
    });
    started.notified().await;
    shutdown.cancel();
    release.notify_one();
    assert!(client.await.unwrap().starts_with("HTTP/1.1 200 OK"));
    server.await.unwrap().unwrap();
}

#[tokio::test]
async fn tls_configuration_fails_closed_for_missing_and_mismatched_pem_material() {
    let temp = tempfile::tempdir().unwrap();
    let missing = RuntimeTlsInitConfig {
        certificate_chain_path: temp.path().join("missing-cert.pem"),
        private_key_path: temp.path().join("missing-key.pem"),
    };
    assert!(load_control_tls(&missing).await.is_err());

    let first = generate_simple_self_signed(["localhost".to_owned()]).unwrap();
    let second = generate_simple_self_signed(["localhost".to_owned()]).unwrap();
    let certificate = temp.path().join("cert.pem");
    let wrong_key = temp.path().join("wrong-key.pem");
    std::fs::write(&certificate, first.cert.pem()).unwrap();
    std::fs::write(&wrong_key, second.signing_key.serialize_pem()).unwrap();
    let mismatched = RuntimeTlsInitConfig {
        certificate_chain_path: certificate,
        private_key_path: wrong_key,
    };
    assert!(load_control_tls(&mismatched).await.is_err());
}

#[tokio::test]
async fn tls_shutdown_is_bounded_with_a_stalled_active_response() {
    let key = SigningKey::from_bytes(&[18; 32]);
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        BlockingLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
            started: started.clone(),
            release,
        },
        authority(&key, [ControlCapability::Stop]),
        4,
    ));
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let shutdown = tokio_util::sync::CancellationToken::new();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener_until(
        service,
        listener,
        tls,
        shutdown.clone().cancelled_owned(),
    ));
    let body = serde_json::to_vec(&signed(
        &key,
        "stop-stalled",
        ControlAction::Shutdown { grace_millis: 5 },
    ))
    .unwrap();
    let client = tokio::spawn(async move {
        let request = format!(
            "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            String::from_utf8(body).unwrap()
        );
        https_request(&client, address, request.as_bytes()).await
    });
    started.notified().await;
    shutdown.cancel();
    tokio::time::timeout(Duration::from_secs(3), server)
        .await
        .expect("TLS shutdown exceeded its bound")
        .unwrap()
        .unwrap();
    client.abort();
}

#[test]
fn runtime_identity_and_shutdown_bounds_are_owned_by_standard_crates() {
    let first = adl_runtime_kernel::generate_runtime_instance_id();
    let second = adl_runtime_kernel::generate_runtime_instance_id();
    assert_eq!(first.len(), 32);
    assert_ne!(first, second);

    let key = SigningKey::from_bytes(&[9; 32]);
    let result = SignedControlCommand::sign(
        "stop-too-long",
        "0123456789abcdef0123456789abcdef",
        "instance-1",
        "operator",
        ControlAction::Shutdown {
            grace_millis: adl_runtime_kernel::MAX_SHUTDOWN_GRACE_MILLIS + 1,
        },
        "operator-key",
        &key,
    );
    assert_eq!(result.unwrap_err(), ControlError::InvalidBounds);
}

#[test]
fn ready_event_reports_the_bound_ephemeral_port() {
    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 43_123));
    let event = adl_runtime_kernel::control_ready_event("instance-1", address);
    assert!(event.contains("event=control_ready"));
    assert!(event.contains("port=43123"));
    assert!(!event.contains("port=20997"));
}

#[test]
fn payload_and_human_observability_use_separate_redacted_channels() {
    assert_eq!(adl_runtime_kernel::DEFAULT_CONTROL_API_PORT, 20_997);
    let response = adl_runtime_kernel::ControlResponse {
        schema: adl_runtime_kernel::CONTROL_RESPONSE_SCHEMA.to_owned(),
        command_id: "read-1".to_owned(),
        correlation_id: "correlation-1".to_owned(),
        outcome: ControlOutcome::Snapshot {
            snapshot: Box::new(RuntimeRecorder::new(2).snapshot()),
        },
    };
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    write_payload(&mut stdout, &response).unwrap();
    let correlation_id = "0123456789abcdef0123456789abcdef";
    write_observability_event(
        &mut stderr,
        ControlObservabilityEvent::SnapshotCompleted,
        correlation_id,
    )
    .unwrap();
    let stdout = String::from_utf8(stdout).unwrap();
    let stderr = String::from_utf8(stderr).unwrap();
    assert!(stdout.starts_with('{') && !stdout.contains("adl_event"));
    assert!(stderr.starts_with("adl_event ") && !stderr.contains("authorization"));

    let mut rejected = Vec::new();
    assert_eq!(
        write_observability_event(
            &mut rejected,
            ControlObservabilityEvent::CommandRejected,
            "authorization-secret",
        )
        .unwrap_err(),
        ControlError::InvalidIdentifier
    );
    assert!(rejected.is_empty());
}

#[test]
fn bootstrap_promotes_once_after_explicit_degraded_readiness() {
    let recorder = RuntimeRecorder::new(4);
    recorder.emit(None, RuntimeEvent::KernelStarting);
    recorder.emit(None, RuntimeEvent::ComponentsReady);
    let promoted = recorder.initialize_observability(ObservabilityHealth::Degraded {
        reason: ObservabilityDegradation::ExporterUnavailable,
    });
    assert_eq!(promoted.len(), 2);
    assert_eq!(promoted[0].sequence, 0);
    assert_eq!(promoted[1].sequence, 1);
    assert!(recorder
        .initialize_observability(ObservabilityHealth::Ready)
        .is_empty());
    assert!(matches!(
        recorder.snapshot().observability,
        ObservabilityHealth::Degraded { .. }
    ));
}

#[tokio::test]
async fn signed_shutdown_routes_through_supervisor_and_carries_correlation() {
    let key = SigningKey::from_bytes(&[5; 32]);
    let recorder = RuntimeRecorder::new(8);
    let handle = Kernel::new(
        ComponentRegistry::new().validate().unwrap(),
        recorder.clone(),
    )
    .start()
    .await
    .unwrap();
    let service = Arc::new(ControlService::new(
        "instance-1",
        recorder.clone(),
        handle.control(),
        authority(&key, [ControlCapability::Stop]),
        4,
    ));
    let command = signed(&key, "stop-1", ControlAction::Shutdown { grace_millis: 50 });
    let response = service.execute(command).await.unwrap();
    assert_eq!(
        response.outcome,
        ControlOutcome::Shutdown {
            exit: ControlExit::Clean
        }
    );
    assert_eq!(handle.wait().await.unwrap(), KernelExit::Clean);
    assert!(recorder
        .events()
        .iter()
        .any(|event| event.correlation_id.as_deref() == Some(response.correlation_id.as_str())));
}
