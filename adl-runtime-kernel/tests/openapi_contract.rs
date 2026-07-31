use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

const RUNTIME_OPENAPI: &str = include_str!("../../docs/api/runtime-v3/v1/openapi.json");
const OBSERVATORY_OPENAPI: &str =
    include_str!("../../docs/api/runtime-v3/v1/observatory.openapi.json");
const CONTROL_RS: &str = include_str!("../src/control.rs");

#[test]
fn runtime_and_observatory_openapi_contracts_are_valid_and_disjoint() {
    let runtime = parse_openapi(RUNTIME_OPENAPI);
    let observatory = parse_openapi(OBSERVATORY_OPENAPI);

    assert_eq!(runtime["openapi"], "3.1.0");
    assert_eq!(observatory["openapi"], "3.1.0");
    assert_eq!(runtime["x-adl-api-identity"], "runtime-core");
    assert_eq!(observatory["x-adl-api-identity"], "observatory");
    assert_eq!(runtime["x-adl-api-version"], "v1");
    assert_eq!(observatory["x-adl-api-version"], "v1");
    assert_eq!(observatory["x-adl-html-application"], "separate");

    assert_all_refs_resolve(&runtime);
    assert_all_refs_resolve(&observatory);
    assert_no_unavailable_operational_claims(&runtime);
    assert_no_unavailable_operational_claims(&observatory);

    for shared_schema in ["ClockAuthority", "ObservabilityHealth"] {
        assert_eq!(
            runtime["components"]["schemas"][shared_schema],
            observatory["components"]["schemas"][shared_schema],
            "{shared_schema} must not drift between the Core and Observatory contracts"
        );
    }

    let runtime_routes = documented_routes(&runtime);
    let observatory_routes = documented_routes(&observatory);
    assert!(runtime_routes.is_disjoint(&observatory_routes));
}

#[test]
fn openapi_paths_match_current_runtime_v3_axum_route_inventory() {
    let runtime = parse_openapi(RUNTIME_OPENAPI);
    let observatory = parse_openapi(OBSERVATORY_OPENAPI);

    let mut documented = documented_routes(&runtime);
    documented.extend(documented_routes(&observatory));

    let real = real_control_routes();
    assert_eq!(
        documented, real,
        "OpenAPI contracts must document each current production route exactly once and must not invent discovery endpoints before the router serves them"
    );
}

#[test]
fn runtime_serves_machine_and_human_readable_openapi_docs() {
    assert!(CONTROL_RS.contains("pub const RUNTIME_OPENAPI_PATH: &str = \"/v1/openapi.json\""));
    assert!(CONTROL_RS
        .contains("pub const OBSERVATORY_OPENAPI_PATH: &str = \"/v1/observatory/openapi.json\""));
    assert!(CONTROL_RS.contains("pub const API_DOCS_PATH: &str = \"/v1/docs/\""));
    assert!(CONTROL_RS
        .contains("pub const OBSERVATORY_API_DOCS_PATH: &str = \"/v1/observatory/docs/\""));
    assert!(CONTROL_RS.contains("SwaggerUi::new(API_DOCS_PATH)"));
    assert!(CONTROL_RS.contains("SwaggerUi::new(OBSERVATORY_API_DOCS_PATH)"));
    assert!(CONTROL_RS.contains(".validator_url(\"none\")"));
    assert!(CONTROL_RS.contains("SwaggerUrl::with_primary(\"Runtime Core\""));
    assert!(CONTROL_RS.contains("SwaggerUrl::new(\"Observatory\""));
    assert!(CONTROL_RS.contains("include_str!(\"../../docs/api/runtime-v3/v1/openapi.json\")"));
    assert!(CONTROL_RS
        .contains("include_str!(\"../../docs/api/runtime-v3/v1/observatory.openapi.json\")"));
}

#[test]
fn observatory_wss_documents_real_bidirectional_frame_boundary() {
    let runtime = parse_openapi(RUNTIME_OPENAPI);
    let observatory = parse_openapi(OBSERVATORY_OPENAPI);
    assert_eq!(
        runtime["paths"]["/v1/health"]["get"]["security"],
        serde_json::json!([])
    );
    assert_eq!(
        runtime["paths"]["/v1/metrics"]["get"]["security"],
        serde_json::json!([])
    );
    assert_eq!(
        observatory["paths"]["/v1/observatory"]["get"]["security"],
        serde_json::json!([])
    );
    assert_eq!(
        observatory["paths"]["/v1/observatory/ws"]["get"]["security"],
        serde_json::json!([])
    );
    let ws = &observatory["paths"]["/v1/observatory/ws"]["get"]["x-adl-websocket"];

    assert_eq!(ws["scheme"], "wss");
    assert_eq!(ws["maxFrameBytes"], 65_536);
    assert_eq!(ws["refreshSeconds"], 1);
    assert_eq!(ws["writeAuthentication"], "observatory_bearer_frame");
    assert_eq!(ws["publicReadFrames"], true);
    assert_eq!(
        ws["credentialRevocation"],
        "write_authority_removed_read_stream_continues"
    );
    assert!(ws["serverFrames"]
        .as_array()
        .expect("serverFrames array")
        .iter()
        .any(|frame| frame["$ref"] == "#/components/schemas/ObservatoryFeed"));
    assert_eq!(
        ws["signedTextControlAuthority"],
        "signed_control_command_only"
    );
    assert_eq!(
        ws["binaryAcipAuthority"],
        "authenticated_session_and_canonical_ingress_policy"
    );
}

#[test]
fn runtime_core_wss_documents_real_bidirectional_acip_boundary() {
    let runtime = parse_openapi(RUNTIME_OPENAPI);
    let ws = &runtime["paths"]["/v1/acip/ws"]["get"]["x-adl-websocket"];

    assert_eq!(ws["scheme"], "wss");
    assert_eq!(ws["maxFrameBytes"], 65_536);
    assert_eq!(ws["authentication"], "http_bearer_upgrade_header");
    assert_eq!(
        ws["serverFirstFrame"]["$ref"],
        "#/components/schemas/AcipAuthenticatedFrame"
    );
    assert_eq!(ws["bidirectional"], true);
    assert!(ws["clientFrames"]
        .as_array()
        .expect("clientFrames array")
        .iter()
        .any(|frame| frame["binarySchema"] == "adl.csm.acip_carrier.protobuf_envelope.v1"));
    assert!(ws["serverFrames"]
        .as_array()
        .expect("serverFrames array")
        .iter()
        .any(|frame| frame["$ref"] == "#/components/schemas/AcipDispatchResult"));

    let reasons: BTreeSet<&str> = ws["policyCloseReasons"]
        .as_array()
        .expect("policyCloseReasons array")
        .iter()
        .map(|value| value.as_str().expect("close reason string"))
        .collect();
    assert_eq!(
        reasons,
        BTreeSet::from(["binary_acip_frame_required", "credential_revoked"])
    );
}

#[test]
fn openapi_operations_are_deterministic_client_generation_ready() {
    let runtime = parse_openapi(RUNTIME_OPENAPI);
    let observatory = parse_openapi(OBSERVATORY_OPENAPI);
    let mut operation_ids = BTreeSet::new();

    for document in [&runtime, &observatory] {
        for (route, methods) in document["paths"].as_object().expect("OpenAPI paths object") {
            for (method, operation) in methods.as_object().expect("path methods object") {
                if !matches!(
                    method.as_str(),
                    "get" | "put" | "post" | "delete" | "options" | "head" | "patch" | "trace"
                ) {
                    continue;
                }
                let operation_id = operation["operationId"]
                    .as_str()
                    .unwrap_or_else(|| panic!("{method} {route} missing operationId"));
                assert!(
                    operation_id
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_'),
                    "{operation_id} must be generator-safe"
                );
                assert!(
                    operation_ids.insert(operation_id.to_owned()),
                    "duplicate operationId {operation_id}"
                );
                assert!(
                    operation["responses"].is_object(),
                    "{method} {route} must declare responses"
                );
            }
        }
    }
}

fn parse_openapi(source: &str) -> Value {
    serde_json::from_str(source).expect("OpenAPI document must parse as JSON")
}

fn documented_routes(document: &Value) -> BTreeSet<(String, String)> {
    let mut routes = BTreeSet::new();
    let paths = document["paths"]
        .as_object()
        .expect("OpenAPI paths must be an object");
    for (path, methods) in paths {
        let methods = methods.as_object().expect("path item must be an object");
        for method in methods.keys() {
            if matches!(
                method.as_str(),
                "get" | "put" | "post" | "delete" | "options" | "head" | "patch" | "trace"
            ) {
                assert!(
                    routes.insert((method.clone(), path.clone())),
                    "duplicate documented route {method} {path}"
                );
            }
        }
    }
    routes
}

fn real_control_routes() -> BTreeSet<(String, String)> {
    real_kernel_control_routes()
}

fn real_kernel_control_routes() -> BTreeSet<(String, String)> {
    let mut routes = BTreeSet::new();
    for route in literal_routes_from_control_rs() {
        match route.as_str() {
            "/v1/observatory" => {
                routes.insert(("get".to_owned(), route.clone()));
                routes.insert(("options".to_owned(), route));
            }
            "/v1/health"
            | "/v1/metrics"
            | "/v1/acip/ws"
            | "/v1/openapi.json"
            | "/v1/observatory/openapi.json"
            | "/v1/docs/"
            | "/v1/observatory/ws"
            | "/v1/observatory/docs/" => {
                routes.insert(("get".to_owned(), route));
            }
            "/v1/control" => {
                routes.insert(("post".to_owned(), route));
            }
            other => panic!("unexpected public Runtime v3 route literal {other}"),
        }
    }
    routes
}

fn literal_routes_from_control_rs() -> BTreeSet<String> {
    let mut routes = BTreeSet::new();
    for expected in [
        "/v1/health",
        "/v1/metrics",
        "/v1/acip/ws",
        "/v1/observatory",
        "/v1/control",
    ] {
        assert!(
            CONTROL_RS.contains(&format!("\"{expected}\"")),
            "control router must still contain {expected}"
        );
        routes.insert(expected.to_owned());
    }
    assert!(
        CONTROL_RS.contains("OBSERVATORY_WS_PATH"),
        "control router must still route through OBSERVATORY_WS_PATH"
    );
    assert!(
        CONTROL_RS.contains("pub const OBSERVATORY_WS_PATH: &str = \"/v1/observatory/ws\""),
        "OBSERVATORY_WS_PATH constant must remain the documented route"
    );
    routes.insert("/v1/observatory/ws".to_owned());
    for (constant, route) in [
        ("RUNTIME_OPENAPI_PATH", "/v1/openapi.json"),
        ("OBSERVATORY_OPENAPI_PATH", "/v1/observatory/openapi.json"),
        ("API_DOCS_PATH", "/v1/docs/"),
    ] {
        assert!(
            CONTROL_RS.contains(&format!("pub const {constant}: &str = \"{route}\"")),
            "{constant} must remain the documented route"
        );
        routes.insert(route.to_owned());
    }
    assert!(
        CONTROL_RS
            .contains("pub const OBSERVATORY_API_DOCS_PATH: &str = \"/v1/observatory/docs/\""),
        "OBSERVATORY_API_DOCS_PATH constant must remain the documented route"
    );
    routes.insert("/v1/observatory/docs/".to_owned());
    routes
}

fn assert_all_refs_resolve(document: &Value) {
    let schemas = document["components"]["schemas"]
        .as_object()
        .expect("components.schemas object");
    let responses = document["components"]["responses"]
        .as_object()
        .expect("components.responses object");
    let mut refs = Vec::new();
    collect_refs(document, &mut refs);
    for reference in refs {
        let Some(name) = reference.strip_prefix("#/components/schemas/") else {
            let Some(name) = reference.strip_prefix("#/components/responses/") else {
                panic!("unsupported local OpenAPI ref {reference}");
            };
            assert!(
                responses.contains_key(name),
                "missing response ref {reference}"
            );
            continue;
        };
        assert!(schemas.contains_key(name), "missing schema ref {reference}");
    }
}

fn collect_refs(value: &Value, refs: &mut Vec<String>) {
    match value {
        Value::Object(object) => {
            if let Some(reference) = object.get("$ref").and_then(Value::as_str) {
                refs.push(reference.to_owned());
            }
            for value in object.values() {
                collect_refs(value, refs);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_refs(value, refs);
            }
        }
        _ => {}
    }
}

fn assert_no_unavailable_operational_claims(document: &Value) {
    let forbidden = [
        "fixture",
        "receipt-only",
        "simulated",
        "degraded placeholder",
        "unavailable endpoint",
        "unimplemented endpoint",
    ];
    let mut text = BTreeMap::new();
    collect_strings("$", document, &mut text);
    for (path, value) in text {
        if path.contains("x-adl-non-claims") {
            continue;
        }
        let lower = value.to_ascii_lowercase();
        for needle in forbidden {
            assert!(
                !lower.contains(needle),
                "forbidden operational wording {needle:?} at {path}: {value}"
            );
        }
    }
}

fn collect_strings(path: &str, value: &Value, strings: &mut BTreeMap<String, String>) {
    match value {
        Value::String(string) => {
            strings.insert(path.to_owned(), string.clone());
        }
        Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                collect_strings(&format!("{path}[{index}]"), value, strings);
            }
        }
        Value::Object(object) => {
            for (key, value) in object {
                collect_strings(&format!("{path}.{key}"), value, strings);
            }
        }
        _ => {}
    }
}
