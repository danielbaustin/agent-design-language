use std::{collections::BTreeSet, sync::Arc, time::Duration};

use adl_runtime::{
    runtime_api::{
        runtime_api_health_report, runtime_api_telemetry_event, serve_runtime_api_listener_until,
        RuntimeApiCapabilityHealth, RuntimeApiFeatureMatrix, RuntimeApiHealthState,
        RuntimeApiService, RuntimeApiTelemetryConfig, RuntimeApiTelemetrySink,
        CSM_RUNTIME_API_DEFAULT_PORT, CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA,
        CSM_RUNTIME_API_WSS_SESSION_SCHEMA,
    },
    runtime_api_auth::RuntimeApiCredentialStore,
};
use futures::{SinkExt, StreamExt};
use rcgen::{generate_simple_self_signed, CertifiedKey};
use tokio_rustls::rustls::{pki_types::CertificateDer, ClientConfig, RootCertStore};
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{
        client::IntoClientRequest,
        http::{header::AUTHORIZATION, HeaderValue, Request},
        Message,
    },
    Connector,
};

fn health() -> adl_runtime::runtime_api::RuntimeApiHealthReport {
    runtime_api_health_report(vec![
        RuntimeApiCapabilityHealth {
            capability: "authenticated_wss".into(),
            state: RuntimeApiHealthState::Healthy,
            reason_code: "loopback_tls_wss_exchange_passed".into(),
            evidence_ref: "adl-runtime/tests/runtime_api_wss.rs".into(),
        },
        RuntimeApiCapabilityHealth {
            capability: "html_observatory_ui".into(),
            state: RuntimeApiHealthState::Unimplemented,
            reason_code: "separate_client_boundary".into(),
            evidence_ref: "demos/html-observatory/README.md".into(),
        },
        RuntimeApiCapabilityHealth {
            capability: "cloud_sink".into(),
            state: RuntimeApiHealthState::Unavailable,
            reason_code: "no_configured_sink".into(),
            evidence_ref: "local_no_aws".into(),
        },
        RuntimeApiCapabilityHealth {
            capability: "adapter_probe_negative_case".into(),
            state: RuntimeApiHealthState::Failed,
            reason_code: "negative_case_retained_for_observatory".into(),
            evidence_ref: "docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json"
                .into(),
        },
    ])
}

fn telemetry() -> RuntimeApiTelemetryConfig {
    RuntimeApiTelemetryConfig {
        schema: "adl.csm.runtime_api.telemetry_config.v1".into(),
        sinks: vec![RuntimeApiTelemetrySink {
            sink: "local_jsonl".into(),
            supported_fields: BTreeSet::from([
                "runtime_instance_id".into(),
                "event".into(),
                "health_state".into(),
            ]),
        }],
    }
}

fn matrix() -> RuntimeApiFeatureMatrix {
    serde_json::from_str(include_str!(
        "../../docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json"
    ))
    .unwrap()
}

async fn server(
    store: RuntimeApiCredentialStore,
) -> (
    std::net::SocketAddr,
    Connector,
    tokio::sync::oneshot::Sender<()>,
    tokio::task::JoinHandle<Result<(), String>>,
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
    let service = Arc::new(RuntimeApiService::new(
        store,
        health(),
        telemetry(),
        matrix(),
    ));
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(serve_runtime_api_listener_until(
        service,
        listener,
        tls,
        async move {
            let _ = stop_rx.await;
        },
    ));
    (address, connector, stop_tx, task)
}

fn request(address: std::net::SocketAddr, token: &str) -> Request<()> {
    let mut request = format!("wss://localhost:{}/v1/acip/ws", address.port())
        .into_client_request()
        .unwrap();
    request.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );
    request
}

#[tokio::test]
async fn wss_auth_rotation_revocation_and_shutdown_are_real_tls_frames() {
    let root = tempfile::tempdir().unwrap();
    let store = RuntimeApiCredentialStore::for_state_root(root.path());
    store.ensure().unwrap();
    let first_token = store.with_bearer_token(str::to_owned).unwrap();
    let (address, connector, stop, task) = server(store.clone()).await;

    let denied = connect_async_tls_with_config(
        request(address, "wrong-token"),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap_err();
    assert!(denied.to_string().contains("401"));

    let (mut socket, _) = connect_async_tls_with_config(
        request(address, &first_token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    let hello = socket.next().await.unwrap().unwrap().into_text().unwrap();
    assert!(hello.contains(CSM_RUNTIME_API_WSS_SESSION_SCHEMA));
    socket
        .send(Message::Text(
            serde_json::json!({"type":"ping","body":{"n":1}})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
    let pong = socket.next().await.unwrap().unwrap().into_text().unwrap();
    assert!(pong.contains("\"type\":\"pong\""));
    assert!(pong.contains("\"n\":1"));
    socket
        .send(Message::Text(
            serde_json::json!({"type":"feature_matrix"})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
    let matrix_frame: RuntimeApiFeatureMatrix =
        serde_json::from_str(&socket.next().await.unwrap().unwrap().into_text().unwrap()).unwrap();
    assert_eq!(matrix_frame, matrix());

    store.rotate().unwrap();
    let (mut old_overlap_socket, _) = connect_async_tls_with_config(
        request(address, &first_token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    assert!(old_overlap_socket
        .next()
        .await
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap()
        .contains("authenticated"));
    let second_token = store.with_bearer_token(str::to_owned).unwrap();
    let (mut rotated_socket, _) = connect_async_tls_with_config(
        request(address, &second_token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    assert!(rotated_socket
        .next()
        .await
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap()
        .contains("authenticated"));
    rotated_socket
        .send(Message::Text(
            serde_json::json!({"type":"shutdown"}).to_string().into(),
        ))
        .await
        .unwrap();
    let shutdown = rotated_socket
        .next()
        .await
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap();
    assert!(shutdown.contains("shutdown_ack"));

    store.revoke().unwrap();
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(2), socket.next())
            .await
            .unwrap(),
        Some(Ok(Message::Close(_)))
    ));
    let _ = stop.send(());
    let _ = task.await.unwrap();
}

#[test]
fn health_telemetry_matrix_and_init_file_are_truthful() {
    let health = health();
    let states = health
        .capabilities
        .iter()
        .map(|capability| capability.state)
        .collect::<BTreeSet<_>>();
    assert!(states.contains(&RuntimeApiHealthState::Unimplemented));
    assert!(states.contains(&RuntimeApiHealthState::Unavailable));
    assert!(states.contains(&RuntimeApiHealthState::Failed));
    assert!(states.contains(&RuntimeApiHealthState::Healthy));

    let telemetry = telemetry();
    let event = runtime_api_telemetry_event(
        &telemetry,
        "local_jsonl",
        &serde_json::json!({
            "runtime_instance_id": "runtime-1",
            "event": "health",
            "health_state": "healthy",
            "unsupported_cloud_field": "must_drop"
        }),
    )
    .unwrap();
    assert_eq!(event["payload"]["runtime_instance_id"], "runtime-1");
    assert!(event["payload"].get("unsupported_cloud_field").is_none());
    assert_eq!(event["dropped_unsupported_fields"], 1);

    let matrix = matrix();
    assert_eq!(matrix.schema, CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA);
    assert!(matrix.unresolved_claimed_features.is_empty());
    let features = matrix
        .rows
        .iter()
        .map(|row| row.feature.as_str())
        .collect::<BTreeSet<_>>();
    assert!(features.contains("wss_authenticated_bidirectional_exchange"));
    assert!(features.contains("observatory_health_distinctions"));
    assert!(features.contains("sink_bounded_telemetry"));
    assert!(features.contains("html_observatory_ui_redesign"));

    let init: toml::Value =
        toml::from_str(include_str!("../../infra/runtime-v3/runtime-api-5665.toml")).unwrap();
    assert_eq!(init["runtime_api"]["mode"].as_str(), Some("api_only"));
    assert_eq!(init["runtime_api"]["port"].as_integer(), Some(20_997));
    assert_eq!(
        init["runtime_api"]["wss_path"].as_str(),
        Some("/v1/acip/ws")
    );
    assert_eq!(
        init["runtime_api"]["auth"].as_str(),
        Some("runtime_api_bearer")
    );
    assert_eq!(CSM_RUNTIME_API_DEFAULT_PORT, 20_997);
}
