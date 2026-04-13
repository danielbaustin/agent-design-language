use super::*;

fn cfg_str<'a>(cfg: &'a HashMap<String, Value>, key: &str) -> Option<&'a str> {
    cfg.get(key).and_then(|v| v.as_str()).map(str::trim)
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

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            runtime_error(
                family,
                format!("failed to create provider invocation artifact directory: {err}"),
            )
        })?;
    }
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
    let tmp = path.with_extension("tmp");
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
