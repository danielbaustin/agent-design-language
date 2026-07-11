use super::{
    assert_api_response_redacted, CSM_RUNTIME_API_API_GATEWAY_BRIDGE_SCHEMA,
    CSM_RUNTIME_API_ENDPOINTS,
};
use crate::observability::emit_event;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
#[cfg(test)]
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};
#[cfg(test)]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
use adl_runtime::runtime_api_auth::{
    RuntimeApiCredentialStore, RuntimeApiGatewayIdentityClaims,
    CSM_RUNTIME_API_GATEWAY_IDENTITY_AUDIENCE, CSM_RUNTIME_API_GATEWAY_IDENTITY_SCHEMA,
};

const SCHEMA: &str = "adl.csm.api_gateway_bridge_proof.v1";
const EVENT_SCHEMA: &str = "adl.csm.api_gateway_bridge.event.v1";
const API_GATEWAY_EXCLUDED_RUNTIME_ROUTES: [&str; 1] = ["/acip/ws"];
const API_GATEWAY_ADDITIONAL_RUNTIME_ROUTES: [&str; 1] = ["/reasoning"];

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct RuntimeGatewayIdentityHeaders {
    pub identity: String,
    pub signature: String,
}

#[cfg(test)]
pub(crate) fn prepare_runtime_gateway_identity_headers(
    state_root: &Path,
    authorizer_principal: &str,
) -> Result<RuntimeGatewayIdentityHeaders> {
    if authorizer_principal.trim().is_empty() {
        bail!("API Gateway authorizer principal must not be empty");
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("read time for API Gateway runtime identity")?
        .as_secs();
    let claims = RuntimeApiGatewayIdentityClaims {
        schema: CSM_RUNTIME_API_GATEWAY_IDENTITY_SCHEMA.to_string(),
        issuer: "aws_api_gateway_authorizer".to_string(),
        principal: authorizer_principal.to_string(),
        audience: CSM_RUNTIME_API_GATEWAY_IDENTITY_AUDIENCE.to_string(),
        authorization_scopes: vec!["csm.runtime.read".to_string()],
        issued_at_epoch_secs: now,
        expires_at_epoch_secs: now + 60,
    };
    let store = RuntimeApiCredentialStore::for_state_root(state_root);
    let (identity, signature) = store
        .sign_gateway_identity(&claims)
        .map_err(anyhow::Error::msg)
        .context("sign credential-free API Gateway runtime identity")?;
    Ok(RuntimeGatewayIdentityHeaders {
        identity,
        signature,
    })
}

#[derive(Debug, Clone)]
pub struct ApiGatewayBridgeOptions {
    pub out_dir: PathBuf,
    pub run_id: String,
    pub polis_id: String,
    pub profile: String,
    pub region: String,
    pub expected_account_sha256: String,
    pub api_id: Option<String>,
    pub stage_name: Option<String>,
    pub invoke_url: String,
    pub operator_token: String,
    pub cloudwatch_log_group: String,
    pub eventbridge_bus: String,
    pub aws_bin: String,
    pub http_bin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGatewayBridgeSummary {
    pub schema: String,
    pub issue: u32,
    pub status: String,
    pub run_id: String,
    pub aws_profile: String,
    pub aws_region: String,
    pub aws_account_hash: String,
    pub polis_ingress: PolisIngressSummary,
    pub api_gateway: ApiGatewayStateSummary,
    pub bridge: BridgeInvocationSummary,
    pub observability: BridgeObservabilitySummary,
    pub event_schema: Value,
    pub negative_case_policy: Value,
    pub live_negative_cases: Value,
    pub redaction: Value,
    pub local_csm_api_policy: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolisIngressSummary {
    pub polis_id_hash: String,
    pub ingress_model: String,
    pub route_target: String,
    pub per_polis_api: bool,
    pub runtime_identity_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGatewayStateSummary {
    pub api_count: usize,
    pub selected_api_id_hash: String,
    pub selected_api_name_hash: String,
    pub selected_protocol_type: String,
    pub selected_stage_name_hash: String,
    pub selected_stage_auto_deploy: Option<bool>,
    pub supported_route_keys: Vec<String>,
    pub planned_route_keys: Vec<String>,
    pub route_target_count: usize,
    pub integration_count: usize,
    pub integration_target_hashes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeInvocationSummary {
    pub correlation_id: String,
    pub endpoint: String,
    pub http_status: u16,
    pub response_schema: String,
    pub runtime_owner: String,
    pub status_class: String,
    pub ready_class: String,
    pub redacted_payload_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeObservabilitySummary {
    pub cloudwatch_log_group_hash: String,
    pub cloudwatch_correlation_observed: bool,
    pub cloudwatch_event_count: usize,
    pub eventbridge_bus_hash: String,
    pub eventbridge_rule_count: usize,
    pub eventbridge_correlation_policy: String,
}

pub fn prove_api_gateway_bridge(
    options: ApiGatewayBridgeOptions,
) -> Result<ApiGatewayBridgeSummary> {
    fs::create_dir_all(&options.out_dir)
        .with_context(|| format!("failed creating {}", options.out_dir.display()))?;
    validate_required_inputs(&options)?;
    let expected_polis_id = options.polis_id.trim();

    let account = aws_output(
        &options.aws_bin,
        &[
            "sts",
            "get-caller-identity",
            "--profile",
            &options.profile,
            "--region",
            &options.region,
            "--query",
            "Account",
            "--output",
            "text",
        ],
    )
    .map_err(|err| anyhow::anyhow!(classify_bridge_error(&err)))?;
    let account = account.trim();
    let account_sha = sha256(account);
    if account_sha != options.expected_account_sha256 {
        emit_bridge_event(
            "auth_denial",
            "blocked",
            &options.run_id,
            Some("api_gateway_account_hash_mismatch"),
        );
        bail!("AWS profile did not resolve to the approved Agent Logic account hash");
    }

    emit_bridge_event("poll", "started", &options.run_id, None);
    let apis_output = aws_output(
        &options.aws_bin,
        &[
            "apigatewayv2",
            "get-apis",
            "--profile",
            &options.profile,
            "--region",
            &options.region,
            "--output",
            "json",
        ],
    )
    .map_err(|err| anyhow::anyhow!(classify_bridge_error(&err)))?;
    let apis_json: Value = serde_json::from_str(&apis_output).context("parse get-apis")?;
    let apis = apis_json
        .get("Items")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if apis.is_empty() {
        emit_bridge_event(
            "unavailable_service",
            "blocked",
            &options.run_id,
            Some("api_gateway_not_provisioned"),
        );
        bail!("no API Gateway v2 APIs are provisioned in the Agent Logic account");
    }
    let requested_api_id = options
        .api_id
        .as_deref()
        .context("csm cloud-control api-gateway-bridge requires --api-id for the per-polis API")?;
    let selected_api = select_api(&apis, requested_api_id)?;
    let selected_api_id = selected_api
        .get("ApiId")
        .and_then(Value::as_str)
        .context("selected API missing ApiId")?;
    let selected_api_name = selected_api
        .get("Name")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let protocol = selected_api
        .get("ProtocolType")
        .and_then(Value::as_str)
        .unwrap_or("unknown");

    let stages = aws_json(
        &options.aws_bin,
        &[
            "apigatewayv2",
            "get-stages",
            "--api-id",
            selected_api_id,
            "--profile",
            &options.profile,
            "--region",
            &options.region,
            "--output",
            "json",
        ],
    )?;
    let empty_stages = Vec::new();
    let requested_stage_name = options
        .stage_name
        .as_deref()
        .context("csm cloud-control api-gateway-bridge requires --stage for the per-polis API")?;
    let stage = select_stage(
        stages
            .get("Items")
            .and_then(Value::as_array)
            .unwrap_or(&empty_stages),
        requested_stage_name,
    )?;
    let stage_name = stage
        .get("StageName")
        .and_then(Value::as_str)
        .context("selected API Gateway stage missing StageName")?;

    let routes = aws_json(
        &options.aws_bin,
        &[
            "apigatewayv2",
            "get-routes",
            "--api-id",
            selected_api_id,
            "--profile",
            &options.profile,
            "--region",
            &options.region,
            "--output",
            "json",
        ],
    )?;
    let route_keys = route_keys(&routes);
    validate_required_routes(&route_keys)?;
    let integrations = aws_json(
        &options.aws_bin,
        &[
            "apigatewayv2",
            "get-integrations",
            "--api-id",
            selected_api_id,
            "--profile",
            &options.profile,
            "--region",
            &options.region,
            "--output",
            "json",
        ],
    )?;
    let route_targets = route_targets(&routes);
    let integration_targets = integration_targets(&integrations);
    validate_required_route_targets(&route_targets, &integration_targets)?;

    let correlation_id = format!("csm-5039-{}", short_hash(&options.run_id));
    let positive = http_json(
        &options.http_bin,
        &options.invoke_url,
        "/api-gateway-bridge",
        Some(&options.operator_token),
        &correlation_id,
    )?;
    if positive.status_code != 200 {
        emit_bridge_event(
            "upstream_failure",
            "blocked",
            &options.run_id,
            Some("api_gateway_status_call_failed"),
        );
        bail!(
            "API Gateway status call failed closed with HTTP {}",
            positive.status_code
        );
    }
    assert_api_response_redacted(&positive.body)?;
    let response_schema = positive
        .body
        .get("schema")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    if response_schema != CSM_RUNTIME_API_API_GATEWAY_BRIDGE_SCHEMA {
        bail!("API Gateway bridge call did not return CSM runtime API Gateway bridge schema");
    }
    if positive.body.get("runtime_owner").and_then(Value::as_str) != Some("csm") {
        bail!("API Gateway bridge call did not return CSM runtime owner");
    }
    validate_polis_ingress_response(&positive.body, expected_polis_id)?;
    if positive
        .body
        .get("agent_instance_id")
        .and_then(Value::as_str)
        != Some(expected_polis_id)
    {
        emit_bridge_event(
            "upstream_failure",
            "blocked",
            &options.run_id,
            Some("api_gateway_polis_identity_mismatch"),
        );
        bail!("API Gateway bridge call did not return the expected polis runtime identity");
    }
    let payload_path = options
        .out_dir
        .join("redacted_api_gateway_bridge_payload.json");
    fs::write(
        &payload_path,
        serde_json::to_string_pretty(&positive.body)? + "\n",
    )
    .with_context(|| format!("failed writing {}", payload_path.display()))?;

    let negative = run_negative_auth_case(&options, &correlation_id)?;
    let cloudwatch = query_cloudwatch(&options, &correlation_id)?;
    let eventbridge = query_eventbridge(&options)?;
    let cloudwatch_correlation_observed = cloudwatch
        .get("correlation_observed")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let eventbridge_rule_count = eventbridge
        .get("rule_count")
        .and_then(Value::as_u64)
        .unwrap_or(0) as usize;
    if !cloudwatch_correlation_observed {
        bail!("CloudWatch proof did not retain the API Gateway request correlation id");
    }
    if eventbridge_rule_count == 0 {
        bail!("EventBridge proof did not retain any bridge routing rules");
    }

    let summary = ApiGatewayBridgeSummary {
        schema: SCHEMA.to_string(),
        issue: 5039,
        status: "passed".to_string(),
        run_id: options.run_id.clone(),
        aws_profile: options.profile.clone(),
        aws_region: options.region.clone(),
        aws_account_hash: short_hash(account),
        polis_ingress: PolisIngressSummary {
            polis_id_hash: short_hash(expected_polis_id),
            ingress_model: "one_api_gateway_api_per_polis".to_string(),
            route_target: "authorized_api_gateway_to_csm_loopback_runtime_api".to_string(),
            per_polis_api: true,
            runtime_identity_verified: true,
        },
        api_gateway: ApiGatewayStateSummary {
            api_count: apis.len(),
            selected_api_id_hash: short_hash(selected_api_id),
            selected_api_name_hash: short_hash(selected_api_name),
            selected_protocol_type: protocol.to_string(),
            selected_stage_name_hash: short_hash(stage_name),
            selected_stage_auto_deploy: stage.get("AutoDeploy").and_then(Value::as_bool),
            supported_route_keys: public_route_keys(&route_keys),
            planned_route_keys: api_gateway_required_runtime_routes()
                .iter()
                .map(|endpoint| format!("GET {endpoint}"))
                .collect(),
            route_target_count: route_targets.len(),
            integration_count: integration_targets.len(),
            integration_target_hashes: integration_targets
                .iter()
                .map(|target| short_hash(target))
                .collect(),
        },
        bridge: BridgeInvocationSummary {
            correlation_id,
            endpoint: "/api-gateway-bridge".to_string(),
            http_status: positive.status_code,
            response_schema: response_schema.to_string(),
            runtime_owner: "csm".to_string(),
            status_class: positive
                .body
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            ready_class: positive
                .body
                .get("ready")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            redacted_payload_ref: "redacted_api_gateway_bridge_payload.json".to_string(),
        },
        observability: BridgeObservabilitySummary {
            cloudwatch_log_group_hash: short_hash(&options.cloudwatch_log_group),
            cloudwatch_correlation_observed,
            cloudwatch_event_count: cloudwatch
                .get("event_count")
                .and_then(Value::as_u64)
                .unwrap_or(0) as usize,
            eventbridge_bus_hash: short_hash(&options.eventbridge_bus),
            eventbridge_rule_count,
            eventbridge_correlation_policy:
                "retained CloudWatch correlation id plus EventBridge rule inventory".to_string(),
        },
        event_schema: json!({
            "schema": EVENT_SCHEMA,
            "event_kinds": [
                "success",
                "denied",
                "throttled",
                "malformed_request",
                "upstream_failure",
                "degraded_csm_state"
            ],
            "redaction": "only request correlation IDs, route labels, classifications, and hashes are retained"
        }),
        negative_case_policy: json!({
            "missing_token": "api_gateway_authorization_denied",
            "malformed_request": "api_gateway_malformed_request",
            "throttling": "api_gateway_throttled",
            "upstream_failure": "api_gateway_upstream_failure",
            "degraded_csm_state": "api_gateway_degraded_csm_state"
        }),
        live_negative_cases: negative,
        redaction: json!({
            "raw_account_id_recorded": false,
            "raw_api_id_recorded": false,
            "raw_invoke_url_recorded": false,
            "raw_authorization_material_recorded": false,
            "credentials_recorded": false
        }),
        local_csm_api_policy: json!({
            "embedded_daemon_api": "loopback_only",
            "runtime_api_path": "/api-gateway-bridge",
            "bridge_mode": "aws_api_gateway_to_authorized_loopback_runtime_api",
            "direct_public_daemon_bind": false,
            "per_polis_api_gateway": true,
            "polis_id_hash": short_hash(expected_polis_id)
        }),
    };

    let summary_path = options.out_dir.join("api_gateway_bridge_summary.json");
    fs::write(
        &summary_path,
        serde_json::to_string_pretty(&summary)? + "\n",
    )
    .with_context(|| format!("failed writing {}", summary_path.display()))?;
    emit_bridge_event("success", "completed", &options.run_id, None);
    Ok(summary)
}

fn validate_required_inputs(options: &ApiGatewayBridgeOptions) -> Result<()> {
    if options.polis_id.trim().is_empty() {
        bail!("csm cloud-control api-gateway-bridge requires --polis-id or ADL_CSM_POLIS_ID");
    }
    if options.expected_account_sha256.trim().is_empty() {
        bail!(
            "csm cloud-control api-gateway-bridge requires --expected-account-sha256 or ADL_AWS_CSM_API_GATEWAY_ACCOUNT_SHA256"
        );
    }
    if options.invoke_url.trim().is_empty() {
        bail!("csm cloud-control api-gateway-bridge requires --invoke-url");
    }
    if options
        .api_id
        .as_deref()
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        bail!("csm cloud-control api-gateway-bridge requires --api-id for the per-polis API");
    }
    if options
        .stage_name
        .as_deref()
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        bail!("csm cloud-control api-gateway-bridge requires --stage for the per-polis API");
    }
    if options.operator_token.trim().is_empty() {
        bail!("csm cloud-control api-gateway-bridge requires --operator-token");
    }
    if options.cloudwatch_log_group.trim().is_empty() {
        bail!("csm cloud-control api-gateway-bridge requires --cloudwatch-log-group");
    }
    if options.eventbridge_bus.trim().is_empty() {
        bail!("csm cloud-control api-gateway-bridge requires --eventbridge-bus");
    }
    Ok(())
}

fn select_api(items: &[Value], api_id: &str) -> Result<Value> {
    items
        .iter()
        .find(|item| item.get("ApiId").and_then(Value::as_str) == Some(api_id))
        .cloned()
        .context("requested per-polis API Gateway API was not returned by get-apis")
}

fn select_stage(items: &[Value], stage_name: &str) -> Result<Value> {
    items
        .iter()
        .find(|item| item.get("StageName").and_then(Value::as_str) == Some(stage_name))
        .cloned()
        .context("requested per-polis API Gateway stage was not returned by get-stages")
}

fn route_keys(routes: &Value) -> Vec<String> {
    routes
        .get("Items")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|route| route.get("RouteKey").and_then(Value::as_str))
        .map(ToString::to_string)
        .collect()
}

fn route_targets(routes: &Value) -> Vec<String> {
    routes
        .get("Items")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|route| route.get("Target").and_then(Value::as_str))
        .map(ToString::to_string)
        .collect()
}

fn integration_targets(integrations: &Value) -> Vec<String> {
    integrations
        .get("Items")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|integration| integration.get("IntegrationId").and_then(Value::as_str))
        .map(|id| format!("integrations/{id}"))
        .collect()
}

fn validate_required_routes(routes: &[String]) -> Result<()> {
    for endpoint in api_gateway_required_runtime_routes() {
        let get_route = format!("GET {endpoint}");
        if !routes
            .iter()
            .any(|route| route == &get_route || route == "$default")
        {
            bail!("API Gateway bridge is missing required route {get_route}");
        }
    }
    Ok(())
}

fn validate_required_route_targets(
    route_targets: &[String],
    integration_targets: &[String],
) -> Result<()> {
    if route_targets.is_empty() {
        bail!("API Gateway bridge routes did not report integration targets");
    }
    if integration_targets.is_empty() {
        bail!("API Gateway bridge did not report integrations");
    }
    for target in route_targets {
        if !integration_targets
            .iter()
            .any(|integration| integration == target)
        {
            bail!("API Gateway route target does not resolve to a returned integration");
        }
    }
    Ok(())
}

fn validate_polis_ingress_response(body: &Value, expected_polis_id: &str) -> Result<()> {
    if body
        .pointer("/polis_ingress/polis_id")
        .and_then(Value::as_str)
        != Some(expected_polis_id)
    {
        bail!("API Gateway bridge response did not retain the expected polis ingress id");
    }
    if body
        .pointer("/polis_ingress/ingress_model")
        .and_then(Value::as_str)
        != Some("one_api_gateway_api_per_polis")
    {
        bail!("API Gateway bridge response did not retain the per-polis ingress model");
    }
    if body
        .pointer("/polis_ingress/route_target")
        .and_then(Value::as_str)
        != Some("authorized_api_gateway_to_csm_loopback_runtime_api")
    {
        bail!("API Gateway bridge response did not retain the governed runtime route target");
    }
    if body
        .pointer("/polis_ingress/per_polis_api")
        .and_then(Value::as_bool)
        != Some(true)
    {
        bail!("API Gateway bridge response did not confirm per-polis API ownership");
    }
    Ok(())
}

fn public_route_keys(routes: &[String]) -> Vec<String> {
    let mut public = Vec::new();
    for endpoint in api_gateway_required_runtime_routes() {
        let route = format!("GET {endpoint}");
        if routes.iter().any(|candidate| candidate == &route) {
            public.push(route);
        }
    }
    if routes.iter().any(|candidate| candidate == "$default") {
        public.push("$default".to_string());
    }
    public
}

fn api_gateway_required_runtime_routes() -> Vec<&'static str> {
    let mut routes: Vec<_> = CSM_RUNTIME_API_ENDPOINTS
        .iter()
        .copied()
        .filter(|endpoint| !API_GATEWAY_EXCLUDED_RUNTIME_ROUTES.contains(endpoint))
        .collect();
    for endpoint in API_GATEWAY_ADDITIONAL_RUNTIME_ROUTES {
        if !routes.contains(&endpoint) {
            routes.push(endpoint);
        }
    }
    routes
}

#[derive(Debug)]
struct HttpJsonResponse {
    status_code: u16,
    body: Value,
}

fn http_json(
    http_bin: &str,
    base_url: &str,
    path: &str,
    bearer: Option<&str>,
    correlation_id: &str,
) -> Result<HttpJsonResponse> {
    let url = format!("{}{}", base_url.trim_end_matches('/'), path);
    let mut config = format!(
        "silent\nshow-error\nwrite-out = \"\\n%{{http_code}}\"\nheader = \"X-ADL-Correlation-Id: {correlation_id}\"\n"
    );
    if let Some(token) = bearer {
        config.push_str(&format!("header = \"Authorization: Bearer {token}\"\n"));
    }
    config.push_str(&format!("url = \"{url}\"\n"));
    let mut child = Command::new(http_bin)
        .arg("--config")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    child
        .stdin
        .as_mut()
        .context("open HTTP probe config stdin")?
        .write_all(config.as_bytes())
        .context("write HTTP probe config")?;
    let output = child
        .wait_with_output()
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(classify_bridge_error(
            &String::from_utf8_lossy(&output.stderr)
        )));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let (body, code) = stdout
        .rsplit_once('\n')
        .context("HTTP probe did not include status code trailer")?;
    let status_code = code
        .trim()
        .parse::<u16>()
        .context("parse HTTP probe status code")?;
    let body = serde_json::from_str::<Value>(body).context("parse HTTP probe JSON body")?;
    Ok(HttpJsonResponse { status_code, body })
}

fn run_negative_auth_case(
    options: &ApiGatewayBridgeOptions,
    correlation_id: &str,
) -> Result<Value> {
    let response = http_json(
        &options.http_bin,
        &options.invoke_url,
        "/api-gateway-bridge",
        None,
        correlation_id,
    )?;
    match response.status_code {
        401 | 403 => {
            emit_bridge_event(
                "denied",
                "blocked",
                &options.run_id,
                Some("api_gateway_authorization_denied"),
            );
            Ok(json!({
                "missing_token": "api_gateway_authorization_denied",
                "http_status": response.status_code,
                "raw_error_recorded": false
            }))
        }
        other => {
            emit_bridge_event(
                "denied",
                "failed",
                &options.run_id,
                Some("api_gateway_negative_auth_unexpected_success"),
            );
            bail!("API Gateway missing-token negative case returned HTTP {other}")
        }
    }
}

fn query_cloudwatch(options: &ApiGatewayBridgeOptions, correlation_id: &str) -> Result<Value> {
    let mut events = Vec::new();
    let mut observed = false;
    let filter_pattern = format!("\"{correlation_id}\"");
    for attempt in 1..=8 {
        let logs = aws_json(
            &options.aws_bin,
            &[
                "logs",
                "filter-log-events",
                "--log-group-name",
                &options.cloudwatch_log_group,
                "--filter-pattern",
                &filter_pattern,
                "--profile",
                &options.profile,
                "--region",
                &options.region,
                "--output",
                "json",
            ],
        )?;
        events = logs
            .get("events")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        observed = serde_json::to_string(&events)
            .unwrap_or_default()
            .contains(correlation_id);
        if observed || attempt == 8 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    Ok(json!({
        "correlation_observed": observed,
        "event_count": events.len(),
        "raw_log_messages_recorded": false
    }))
}

fn query_eventbridge(options: &ApiGatewayBridgeOptions) -> Result<Value> {
    let rules = aws_json(
        &options.aws_bin,
        &[
            "events",
            "list-rules",
            "--event-bus-name",
            &options.eventbridge_bus,
            "--name-prefix",
            "adl-csm",
            "--profile",
            &options.profile,
            "--region",
            &options.region,
            "--output",
            "json",
        ],
    )?;
    let rule_count = rules
        .get("Rules")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    Ok(json!({
        "rule_count": rule_count,
        "raw_rule_arns_recorded": false
    }))
}

fn aws_json(aws_bin: &str, args: &[&str]) -> Result<Value> {
    let raw =
        aws_output(aws_bin, args).map_err(|err| anyhow::anyhow!(classify_bridge_error(&err)))?;
    serde_json::from_str(&raw).context("parse AWS JSON")
}

fn aws_output(aws_bin: &str, args: &[&str]) -> std::result::Result<String, String> {
    let output = Command::new(aws_bin)
        .args(args)
        .output()
        .map_err(|err| err.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn classify_bridge_error(error: &str) -> String {
    let lower = error.to_ascii_lowercase();
    if lower.contains("throttl") || lower.contains("too many requests") {
        "api_gateway_throttled".to_string()
    } else if lower.contains("accessdenied")
        || lower.contains("access denied")
        || lower.contains("not authorized")
        || lower.contains("unauthorized")
        || lower.contains("forbidden")
    {
        "api_gateway_authorization_denied".to_string()
    } else if lower.contains("profile") && lower.contains("could not be found") {
        "api_gateway_profile_missing".to_string()
    } else if lower.contains("notfound") || lower.contains("not found") {
        "api_gateway_not_found".to_string()
    } else if lower.contains("malformed") || lower.contains("bad request") {
        "api_gateway_malformed_request".to_string()
    } else {
        "api_gateway_unavailable_or_not_provisioned".to_string()
    }
}

fn emit_bridge_event(event_kind: &str, result: &str, run_id: &str, failure_class: Option<&str>) {
    emit_event(
        "csm",
        "api_gateway_bridge",
        result,
        &[
            ("schema", EVENT_SCHEMA),
            ("provider", "aws"),
            ("service", "apigateway"),
            ("event_kind", event_kind),
            ("run_id", run_id),
            ("failure_class", failure_class.unwrap_or("none")),
        ],
    );
}

fn sha256(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn short_hash(value: &str) -> String {
    sha256(value).chars().take(16).collect()
}

#[cfg(test)]
mod tests {
    use super::{classify_bridge_error, prove_api_gateway_bridge, ApiGatewayBridgeOptions};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn api_gateway_error_classification_covers_negative_cases() {
        assert_eq!(
            classify_bridge_error("AccessDeniedException: not authorized"),
            "api_gateway_authorization_denied"
        );
        assert_eq!(
            classify_bridge_error("TooManyRequestsException: throttling"),
            "api_gateway_throttled"
        );
        assert_eq!(
            classify_bridge_error("The config profile (missing) could not be found"),
            "api_gateway_profile_missing"
        );
        assert_eq!(
            classify_bridge_error("Bad Request: malformed"),
            "api_gateway_malformed_request"
        );
    }

    #[test]
    fn api_gateway_bridge_writes_redacted_summary_with_fake_aws_and_http() {
        let root = temp_dir("api-gateway-bridge");
        let aws = write_fake_aws(&root, false);
        let http = write_fake_http(&root, false);
        let out_dir = root.join("proof");
        let account_sha =
            "2a33349e7e606a8ad2e30e3c84521f9377450cf09083e162e0a9b1480ce0f972".to_string();
        let summary = prove_api_gateway_bridge(ApiGatewayBridgeOptions {
            out_dir: out_dir.clone(),
            run_id: "fixture-run".to_string(),
            polis_id: "api-agent".to_string(),
            profile: "agent-logic-admin".to_string(),
            region: "us-west-2".to_string(),
            expected_account_sha256: account_sha,
            api_id: Some("api-1234567890".to_string()),
            stage_name: Some("prod".to_string()),
            invoke_url: "https://fixture.execute-api.us-west-2.amazonaws.com/prod".to_string(),
            operator_token: "fixture-token".to_string(),
            cloudwatch_log_group: "/aws/apigateway/adl-csm".to_string(),
            eventbridge_bus: "adl-csm-bus".to_string(),
            aws_bin: aws.display().to_string(),
            http_bin: http.display().to_string(),
        })
        .expect("fake API Gateway proof");

        assert_eq!(summary.status, "passed");
        assert_eq!(summary.aws_account_hash, "2a33349e7e606a8a");
        assert_eq!(summary.api_gateway.api_count, 1);
        assert_eq!(
            summary.bridge.response_schema,
            "adl.csm.runtime_api.api_gateway_bridge.v1"
        );
        assert!(summary.observability.cloudwatch_correlation_observed);
        assert_eq!(
            summary.live_negative_cases["missing_token"],
            "api_gateway_authorization_denied"
        );
        let text = fs::read_to_string(out_dir.join("api_gateway_bridge_summary.json")).unwrap();
        assert!(!text.contains("123456789012"));
        assert!(!text.contains("api-1234567890"));
        assert!(!text.contains("fixture.execute-api"));
        assert!(!text.contains("fixture-token"));
    }

    #[test]
    fn api_gateway_bridge_fails_closed_when_negative_auth_succeeds() {
        let root = temp_dir("api-gateway-negative-success");
        let aws = write_fake_aws(&root, false);
        let http = write_fake_http(&root, true);
        let out_dir = root.join("proof");
        let account_sha =
            "2a33349e7e606a8ad2e30e3c84521f9377450cf09083e162e0a9b1480ce0f972".to_string();
        let error = prove_api_gateway_bridge(ApiGatewayBridgeOptions {
            out_dir,
            run_id: "fixture-run".to_string(),
            polis_id: "api-agent".to_string(),
            profile: "agent-logic-admin".to_string(),
            region: "us-west-2".to_string(),
            expected_account_sha256: account_sha,
            api_id: Some("api-1234567890".to_string()),
            stage_name: Some("prod".to_string()),
            invoke_url: "https://fixture.execute-api.us-west-2.amazonaws.com/prod".to_string(),
            operator_token: "fixture-token".to_string(),
            cloudwatch_log_group: "/aws/apigateway/adl-csm".to_string(),
            eventbridge_bus: "adl-csm-bus".to_string(),
            aws_bin: aws.display().to_string(),
            http_bin: http.display().to_string(),
        })
        .expect_err("unexpected auth success must fail closed");
        assert!(
            error.to_string().contains("missing-token negative case"),
            "{error}"
        );
    }

    #[test]
    fn api_gateway_bridge_fails_closed_on_polis_identity_mismatch() {
        let root = temp_dir("api-gateway-polis-mismatch");
        let aws = write_fake_aws(&root, false);
        let http = write_fake_http_with_agent(&root, false, "other-polis");
        let out_dir = root.join("proof");
        let account_sha =
            "2a33349e7e606a8ad2e30e3c84521f9377450cf09083e162e0a9b1480ce0f972".to_string();
        let error = prove_api_gateway_bridge(ApiGatewayBridgeOptions {
            out_dir,
            run_id: "fixture-run".to_string(),
            polis_id: "api-agent".to_string(),
            profile: "agent-logic-admin".to_string(),
            region: "us-west-2".to_string(),
            expected_account_sha256: account_sha,
            api_id: Some("api-1234567890".to_string()),
            stage_name: Some("prod".to_string()),
            invoke_url: "https://fixture.execute-api.us-west-2.amazonaws.com/prod".to_string(),
            operator_token: "fixture-token".to_string(),
            cloudwatch_log_group: "/aws/apigateway/adl-csm".to_string(),
            eventbridge_bus: "adl-csm-bus".to_string(),
            aws_bin: aws.display().to_string(),
            http_bin: http.display().to_string(),
        })
        .expect_err("wrong polis identity must fail closed");
        assert!(error.to_string().contains("expected polis"), "{error}");
    }

    fn temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("adl-{label}-{unique}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_fake_aws(root: &Path, missing_routes: bool) -> PathBuf {
        let path = root.join("aws");
        let routes = if missing_routes {
            r#"{"Items":[{"RouteKey":"GET /status"}]}"#
        } else {
            r#"{"Items":[{"RouteKey":"GET /status","Target":"integrations/int-1234567890"},{"RouteKey":"GET /health","Target":"integrations/int-1234567890"},{"RouteKey":"GET /ready","Target":"integrations/int-1234567890"},{"RouteKey":"GET /metrics","Target":"integrations/int-1234567890"},{"RouteKey":"GET /events","Target":"integrations/int-1234567890"},{"RouteKey":"GET /chronosense","Target":"integrations/int-1234567890"},{"RouteKey":"GET /weather","Target":"integrations/int-1234567890"},{"RouteKey":"GET /shepherd","Target":"integrations/int-1234567890"},{"RouteKey":"GET /cav","Target":"integrations/int-1234567890"},{"RouteKey":"GET /curiosity","Target":"integrations/int-1234567890"},{"RouteKey":"GET /acip","Target":"integrations/int-1234567890"},{"RouteKey":"GET /freedom-gate","Target":"integrations/int-1234567890"},{"RouteKey":"GET /reasoning","Target":"integrations/int-1234567890"},{"RouteKey":"GET /api-gateway-bridge","Target":"integrations/int-1234567890"},{"RouteKey":"GET /persistence","Target":"integrations/int-1234567890"},{"RouteKey":"GET /constructability","Target":"integrations/int-1234567890"}]}"#
        };
        fs::write(
            &path,
            format!(
                r#"#!/usr/bin/env bash
set -euo pipefail
case "$1 $2" in
  "sts get-caller-identity")
    printf '%s\n' "123456789012"
    ;;
  "apigatewayv2 get-apis")
    printf '%s\n' '{{"Items":[{{"ApiId":"api-1234567890","Name":"adl-csm-fixture","ProtocolType":"HTTP"}}]}}'
    ;;
  "apigatewayv2 get-stages")
    printf '%s\n' '{{"Items":[{{"StageName":"prod","AutoDeploy":true}}]}}'
    ;;
  "apigatewayv2 get-routes")
    printf '%s\n' '{routes}'
    ;;
  "apigatewayv2 get-integrations")
    printf '%s\n' '{{"Items":[{{"IntegrationId":"int-1234567890","IntegrationType":"HTTP_PROXY","IntegrationUri":"https://loopback-proxy.invalid/csm"}}]}}'
    ;;
  "logs filter-log-events")
    printf '%s\n' '{{"events":[{{"eventId":"evt-1","message":"bridge csm-5039-a91b3eafa2b703d4 success"}}]}}'
    ;;
  "events list-rules")
    printf '%s\n' '{{"Rules":[{{"Name":"adl-csm-api-gateway-bridge","Arn":"arn:aws:events:us-west-2:123456789012:rule/adl-csm"}}]}}'
    ;;
  *)
    echo "unexpected aws args: $*" >&2
    exit 2
    ;;
esac
"#
            ),
        )
        .unwrap();
        make_executable(&path);
        path
    }

    fn write_fake_http(root: &Path, negative_succeeds: bool) -> PathBuf {
        write_fake_http_with_agent(root, negative_succeeds, "api-agent")
    }

    fn write_fake_http_with_agent(
        root: &Path,
        negative_succeeds: bool,
        agent_instance_id: &str,
    ) -> PathBuf {
        let path = root.join("curl");
        let negative_status = if negative_succeeds { 200 } else { 403 };
        fs::write(
            &path,
            format!(
                r#"#!/usr/bin/env bash
set -euo pipefail
auth="missing"
config="$(cat)"
case "$config" in
  *"Authorization: Bearer"*) auth="present" ;;
esac
if [ "$auth" = "present" ]; then
  printf '%s\n%s' '{{"schema":"adl.csm.runtime_api.api_gateway_bridge.v1","runtime_owner":"csm","agent_instance_id":"{agent_instance_id}","status":"available","runtime_api_path":"/api-gateway-bridge","polis_ingress":{{"polis_id":"{agent_instance_id}","ingress_model":"one_api_gateway_api_per_polis","route_target":"authorized_api_gateway_to_csm_loopback_runtime_api","per_polis_api":true}},"redaction":{{"secret_material":"not_returned"}}}}' "200"
else
  printf '%s\n%s' '{{"schema":"adl.csm.api_gateway_bridge.denied.v1","status":"denied"}}' "{negative_status}"
fi
"#
            ),
        )
        .unwrap();
        make_executable(&path);
        path
    }

    fn make_executable(path: &Path) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(path, perms).unwrap();
        }
    }
}
