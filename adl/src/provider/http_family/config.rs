use super::*;
use reqwest::Url;

pub(super) fn cfg_str<'a>(cfg: &'a HashMap<String, Value>, key: &str) -> Option<&'a str> {
    cfg.get(key).and_then(|v| v.as_str()).map(str::trim)
}

pub(super) fn auth_env_for(spec: &adl::ProviderSpec, default_env: &str) -> Result<String> {
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

pub(super) fn vendor_endpoint(
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

pub(super) fn ollama_generate_endpoint(spec: &adl::ProviderSpec) -> Result<String> {
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

#[derive(Debug, Clone)]
pub(super) struct HttpAuth {
    pub env: String,
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
