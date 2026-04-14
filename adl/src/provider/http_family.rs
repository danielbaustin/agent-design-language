use super::*;
use reqwest::Url;
use std::thread;
use std::time::Duration;

fn cfg_str<'a>(cfg: &'a HashMap<String, Value>, key: &str) -> Option<&'a str> {
    cfg.get(key).and_then(|v| v.as_str()).map(str::trim)
}

struct InvocationArtifactLock {
    path: PathBuf,
}

impl Drop for InvocationArtifactLock {
    fn drop(&mut self) {
        let _ = fs::remove_dir(&self.path);
    }
}

fn invocation_lock_path(path: &Path) -> PathBuf {
    let mut os = path.as_os_str().to_os_string();
    os.push(".lock");
    PathBuf::from(os)
}

fn acquire_invocation_artifact_lock(path: &Path) -> std::io::Result<InvocationArtifactLock> {
    let lock_path = invocation_lock_path(path);
    for _attempt in 0..200 {
        match fs::create_dir(&lock_path) {
            Ok(()) => return Ok(InvocationArtifactLock { path: lock_path }),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(err) => return Err(err),
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "timed out waiting for invocation artifact lock",
    ))
}

fn auth_env_for(spec: &adl::ProviderSpec, default_env: &str) -> Result<String> {
    let Some(auth_val) = spec.config.get("auth") else {
        return Ok(default_env.to_string());
    };
    let obj = auth_val
        .as_object()
        .ok_or_else(|| invalid_config("native", "config.auth must be an object"))?;
    let auth_type = obj
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| invalid_config("native", "config.auth.type is required"))?;
    if auth_type != "bearer" {
        return Err(invalid_config(
            "native",
            format!("config.auth.type must be 'bearer' (got '{auth_type}')"),
        ));
    }
    let env_key = obj
        .get("env")
        .and_then(|v| v.as_str())
        .ok_or_else(|| invalid_config("native", "config.auth.env is required"))?;
    let trimmed = env_key.trim();
    if trimmed.is_empty() {
        return Err(invalid_config(
            "native",
            "config.auth.env must not be empty",
        ));
    }
    Ok(trimmed.to_string())
}

fn vendor_endpoint(
    spec: &adl::ProviderSpec,
    target: &ProviderInvocationTargetV1,
    default_endpoint: &str,
    provider_label: &str,
) -> Result<String> {
    let endpoint = target
        .endpoint
        .clone()
        .or_else(|| target.base_url.clone())
        .unwrap_or_else(|| default_endpoint.to_string());
    if !is_allowed_remote_endpoint(&endpoint) {
        return Err(invalid_config(
            provider_label,
            "endpoint must use https://; plaintext http:// is only allowed for localhost/loopback test endpoints",
        ));
    }
    if let Some(override_endpoint) = cfg_str(&spec.config, "endpoint") {
        if override_endpoint.is_empty() {
            return Err(invalid_config(
                provider_label,
                "config.endpoint must not be empty when provided",
            ));
        }
    }
    Ok(endpoint)
}

fn ollama_generate_endpoint(spec: &adl::ProviderSpec) -> Result<String> {
    let explicit_endpoint = cfg_str(&spec.config, "endpoint").map(ToString::to_string);
    let base_url = spec.base_url.clone();
    let source = explicit_endpoint.or(base_url).ok_or_else(|| {
        invalid_config(
            "ollama",
            "remote Ollama transport requires base_url or config.endpoint",
        )
    })?;
    if !is_allowed_ollama_endpoint(&source) {
        return Err(invalid_config(
            "ollama",
            "remote Ollama endpoint must use http:// or https://",
        ));
    }

    let mut url = Url::parse(&source)
        .map_err(|err| invalid_config("ollama", format!("invalid Ollama endpoint: {err}")))?;
    let path = url.path().trim_end_matches('/');
    let explicit_path = !path.is_empty() && path != "/";

    if path.ends_with("/api/generate") {
        return Ok(url.to_string());
    }

    if spec.base_url.is_some() || !explicit_path {
        url.set_path("/api/generate");
    }

    Ok(url.to_string())
}

fn truncate_provider_body(text: &str) -> &str {
    let trimmed = text.trim();
    if trimmed.len() > 200 {
        &trimmed[..200]
    } else {
        trimmed
    }
}

fn provider_http_json(
    provider_label: &str,
    req: reqwest::blocking::RequestBuilder,
) -> Result<(Value, u16)> {
    let resp = match req.send() {
        Ok(resp) => resp,
        Err(err) => {
            if err.is_timeout() {
                return Err(timeout_error(
                    provider_label,
                    "kind=timeout native provider request timed out",
                ));
            }
            return Err(runtime_error(
                provider_label,
                format!("kind=request_failed native provider request failed: {err}"),
            ));
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        let class = if status.is_client_error() {
            "client_error"
        } else if status.is_server_error() {
            "server_error"
        } else {
            "http_error"
        };
        let msg = format!(
            "kind={class} status={status} body={}",
            truncate_provider_body(&text)
        );
        if status.is_client_error() {
            return Err(runtime_error_non_retryable(provider_label, msg));
        }
        return Err(runtime_error(provider_label, msg));
    }

    let http_status = resp.status().as_u16();
    let json = resp
        .json()
        .context("native provider response was not valid JSON")
        .map_err(|err| runtime_error_non_retryable(provider_label, err.to_string()))?;
    Ok((json, http_status))
}

fn write_native_invocation_record(
    family: &str,
    model: &str,
    prompt: &str,
    output: &str,
    http_status: u16,
) -> Result<()> {
    let Some(path) = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH") else {
        return Ok(());
    };
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            runtime_error(
                family,
                format!("failed to create provider invocation artifact directory: {err}"),
            )
        })?;
    }
    let _artifact_lock = acquire_invocation_artifact_lock(&path).map_err(|err| {
        runtime_error(
            family,
            format!("failed to acquire provider invocation artifact lock: {err}"),
        )
    })?;
    let mut payload = if path.is_file() {
        serde_json::from_slice::<Value>(&fs::read(&path).map_err(|err| {
            runtime_error(
                family,
                format!("failed to read provider invocation artifact: {err}"),
            )
        })?)
        .map_err(|err| {
            runtime_error_non_retryable(
                family,
                format!("provider invocation artifact is invalid JSON: {err}"),
            )
        })?
    } else {
        serde_json::json!({
            "schema_version": "adl.native_provider_invocations.v1",
            "credential_policy": "operator_env_only_no_secret_material_recorded",
            "invocations": []
        })
    };

    let Some(invocations) = payload
        .get_mut("invocations")
        .and_then(|v| v.as_array_mut())
    else {
        return Err(runtime_error_non_retryable(
            family,
            "provider invocation artifact missing invocations array",
        ));
    };
    let timestamp_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    invocations.push(serde_json::json!({
        "family": family,
        "model": model,
        "http_status": http_status,
        "timestamp_unix_ms": timestamp_unix_ms,
        "prompt_chars": prompt.chars().count(),
        "output_chars": output.chars().count()
    }));
    let bytes = serde_json::to_vec_pretty(&payload).map_err(|err| {
        runtime_error_non_retryable(
            family,
            format!("failed to serialize provider invocation artifact: {err}"),
        )
    })?;
    write_file_atomic(&path, &bytes).map_err(|err| {
        runtime_error(
            family,
            format!("failed to write invocation artifact: {err}"),
        )
    })
}

fn write_file_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut os = path.as_os_str().to_os_string();
    os.push(format!(
        ".tmp-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let tmp = PathBuf::from(os);
    fs::write(&tmp, bytes)?;
    fs::rename(tmp, path)
}

fn extract_openai_output_text(json: &Value) -> Option<String> {
    if let Some(text) = json.get("output_text").and_then(|v| v.as_str()) {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    let mut chunks = Vec::new();
    for item in json.get("output")?.as_array()? {
        for content in item.get("content").and_then(|v| v.as_array())? {
            if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                chunks.push(text);
            }
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

fn extract_anthropic_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    for content in json.get("content")?.as_array()? {
        let content_type = content.get("type").and_then(|v| v.as_str());
        if content_type == Some("text") {
            if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                chunks.push(text);
            }
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

#[derive(Debug, Clone)]
pub struct OpenAiProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_output_tokens: u64,
    timeout_secs: Option<u64>,
}

impl OpenAiProvider {
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        Ok(Self {
            endpoint: vendor_endpoint(spec, target, OPENAI_RESPONSES_ENDPOINT, "openai")?,
            auth_env: auth_env_for(spec, "OPENAI_API_KEY")?,
            model: target.provider_model_id.clone(),
            max_output_tokens: cfg_u64(&spec.config, "max_output_tokens").unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for OpenAiProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "openai",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build OpenAI client")
            .map_err(|err| runtime_error("openai", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&serde_json::json!({
                "model": self.model,
                "input": prompt,
                "max_output_tokens": self.max_output_tokens,
            }));
        let (json, http_status) = provider_http_json("openai", req)?;
        let output = extract_openai_output_text(&json)
            .ok_or_else(|| runtime_error_non_retryable("openai", "response missing text output"))?;
        write_native_invocation_record("openai", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub struct AnthropicProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl AnthropicProvider {
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        Ok(Self {
            endpoint: vendor_endpoint(spec, target, ANTHROPIC_MESSAGES_ENDPOINT, "anthropic")?,
            auth_env: auth_env_for(spec, "ANTHROPIC_API_KEY")?,
            model: target.provider_model_id.clone(),
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for AnthropicProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "anthropic",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build Anthropic client")
            .map_err(|err| runtime_error("anthropic", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("x-api-key", token)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .json(&serde_json::json!({
                "model": self.model,
                "max_tokens": self.max_tokens,
                "messages": [{"role": "user", "content": prompt}],
            }));
        let (json, http_status) = provider_http_json("anthropic", req)?;
        let output = extract_anthropic_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("anthropic", "response missing text output")
        })?;
        write_native_invocation_record("anthropic", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub struct HttpProvider {
    endpoint: String,
    auth: Option<HttpAuth>,
    headers: HashMap<String, String>,
    timeout_secs: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct OllamaHttpProvider {
    endpoint: String,
    model: String,
    temperature: Option<f32>,
    timeout_secs: Option<u64>,
}

impl OllamaHttpProvider {
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        Ok(Self {
            endpoint: ollama_generate_endpoint(spec)?,
            model: target.provider_model_id.clone(),
            temperature: super::local::cfg_f32(&spec.config, "temperature"),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for OllamaHttpProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build ollama http client")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let mut body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
        });
        if let Some(temperature) = self.temperature {
            body["options"] = serde_json::json!({ "temperature": temperature });
        }

        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&body);
        let (json, http_status) = provider_http_json("ollama", req)?;
        let output = json
            .get("response")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .ok_or_else(|| {
                runtime_error_non_retryable("ollama", "response missing 'response' text field")
            })?
            .to_string();
        write_native_invocation_record("ollama", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
struct HttpAuth {
    env: String,
}

impl HttpProvider {
    pub fn from_spec(spec: &adl::ProviderSpec) -> Result<Self> {
        let target = provider_substrate::provider_invocation_target_v1(
            spec.id.as_deref().unwrap_or("<anonymous-provider>"),
            spec,
            None,
        )?;
        Self::from_target(spec, &target)
    }

    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let cfg = &spec.config;
        let endpoint = target
            .endpoint
            .clone()
            .or_else(|| target.base_url.clone())
            .ok_or_else(|| {
                invalid_config(
                    "http",
                    "config.endpoint is required (set providers.<id>.config.endpoint)",
                )
            })?;
        if !is_allowed_remote_endpoint(&endpoint) {
            return Err(invalid_config(
                "http",
                "config.endpoint must use https://; plaintext http:// is only allowed for localhost/loopback test endpoints",
            ));
        }

        let timeout_secs = cfg_u64(cfg, "timeout_secs");

        let mut headers = HashMap::new();
        if let Some(h) = cfg.get("headers") {
            let obj = h.as_object().ok_or_else(|| {
                invalid_config("http", "config.headers must be an object of string values")
            })?;
            for (k, v) in obj {
                let v = v.as_str().ok_or_else(|| {
                    invalid_config("http", "config.headers values must be strings")
                })?;
                headers.insert(k.clone(), v.to_string());
            }
        }

        let auth = if let Some(auth_val) = cfg.get("auth") {
            let obj = auth_val
                .as_object()
                .ok_or_else(|| invalid_config("http", "config.auth must be an object"))?;
            let auth_type = obj
                .get("type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_config("http", "config.auth.type is required"))?;
            if auth_type != "bearer" {
                return Err(invalid_config(
                    "http",
                    format!("config.auth.type must be 'bearer' (got '{auth_type}')"),
                ));
            }
            let env_key = obj
                .get("env")
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_config("http", "config.auth.env is required"))?;
            Some(HttpAuth {
                env: env_key.to_string(),
            })
        } else {
            None
        };

        Ok(Self {
            endpoint,
            auth,
            headers,
            timeout_secs,
        })
    }
}

impl Provider for HttpProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build http client")
            .map_err(|err| runtime_error("http", err.to_string()))?;

        let mut req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json");

        for (k, v) in self.headers.iter() {
            req = req.header(k, v);
        }

        if let Some(auth) = &self.auth {
            let token = env::var(&auth.env).map_err(|_| {
                invalid_config(
                    "http",
                    format!(
                        "missing required auth env var '{}' (set it or remove config.auth)",
                        auth.env
                    ),
                )
            })?;
            req = req.bearer_auth(token);
        }

        let body = serde_json::json!({ "prompt": prompt });

        let resp = match req.json(&body).send() {
            Ok(resp) => resp,
            Err(err) => {
                if err.is_timeout() {
                    let msg = match self.timeout_secs {
                        Some(secs) => format!(
                            "kind=timeout timed out after {secs}s (set providers.<id>.config.timeout_secs or ADL_TIMEOUT_SECS to override)"
                        ),
                        None => {
                            "kind=timeout timed out (set providers.<id>.config.timeout_secs or ADL_TIMEOUT_SECS to override)"
                                .to_string()
                        }
                    };
                    return Err(timeout_error("http", msg));
                }

                return Err(runtime_error(
                    "http",
                    format!("kind=request_failed http provider request failed: {err}"),
                ));
            }
        };

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            let trimmed = text.trim();
            let trimmed = if trimmed.len() > 200 {
                &trimmed[..200]
            } else {
                trimmed
            };
            let class = if status.is_client_error() {
                "client_error"
            } else if status.is_server_error() {
                "server_error"
            } else {
                "http_error"
            };
            let msg = format!("kind={class} status={status} body={trimmed}");
            if status.is_client_error() {
                return Err(runtime_error_non_retryable("http", msg));
            }
            return Err(runtime_error("http", msg));
        }

        let json: serde_json::Value = resp
            .json()
            .context("http provider response was not valid JSON")
            .map_err(|err| runtime_error_non_retryable("http", err.to_string()))?;
        let out = json.get("output").and_then(|v| v.as_str()).ok_or_else(|| {
            runtime_error_non_retryable("http", "response missing 'output' field")
        })?;

        Ok(out.to_string())
    }
}

pub(crate) fn timeout_secs() -> Result<u64> {
    let raw = std::env::var("ADL_TIMEOUT_SECS").ok();
    let secs = match raw {
        None => 120_u64,
        Some(v) => {
            let parsed: u64 = v.parse().map_err(|_| {
                anyhow!("invalid ADL_TIMEOUT_SECS: '{v}' (must be a positive integer)")
            })?;
            if parsed == 0 {
                return Err(anyhow!(
                    "invalid ADL_TIMEOUT_SECS: '{v}' (must be a positive integer)"
                ));
            }
            parsed
        }
    };
    Ok(secs)
}

pub(crate) fn cfg_u64(cfg: &HashMap<String, Value>, key: &str) -> Option<u64> {
    cfg.get(key).and_then(|v| {
        if let Some(u) = v.as_u64() {
            Some(u)
        } else if let Some(i) = v.as_i64() {
            if i >= 0 {
                Some(i as u64)
            } else {
                None
            }
        } else if let Some(s) = v.as_str() {
            s.parse::<u64>().ok()
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider_substrate::{
        CapabilityModeV1, CapabilitySupportV1, ProviderCapabilitiesV1, ProviderInvocationTargetV1,
        ProviderTransportV1,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use std::env;
    use std::net::TcpListener;
    use std::sync::{Arc, Mutex, OnceLock};
    use std::thread;
    use tiny_http::{Header, Response, Server};

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
            endpoint: Some(endpoint),
            base_url: None,
            model_ref: provider_model_id.to_string(),
            provider_model_id: provider_model_id.to_string(),
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
        assert!(captured.headers.iter().any(
            |(k, v)| k.eq_ignore_ascii_case("authorization") && v == "Bearer test-openai-token"
        ));

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

        let mut bad_auth_spec =
            provider_spec("openai", &format!("{endpoint}/v1/responses"), None, &[]);
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

        let long_text = format!("  {}  ", "x".repeat(250));
        assert_eq!(truncate_provider_body(&long_text).len(), 200);
        assert_eq!(truncate_provider_body("  short body  "), "short body");

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
        env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", &artifact);

        write_native_invocation_record("openai", "gpt-test", "hello", "world", 200)
            .expect("write fresh artifact");
        let first_payload = std::fs::read_to_string(&artifact).expect("fresh artifact");
        assert!(
            first_payload.contains("\"schema_version\": \"adl.native_provider_invocations.v1\"")
        );

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

        match prev_artifact {
            Some(v) => env::set_var("ADL_PROVIDER_INVOCATIONS_PATH", v),
            None => env::remove_var("ADL_PROVIDER_INVOCATIONS_PATH"),
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
    }
}
