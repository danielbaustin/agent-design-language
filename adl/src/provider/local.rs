use super::http_family::timeout_secs;
use super::*;

#[derive(Debug, Clone)]
pub struct MockProvider {
    model: String,
}

impl MockProvider {
    pub fn from_target(target: &ProviderInvocationTargetV1) -> Self {
        Self {
            model: target.model_ref.clone(),
        }
    }
}

impl Provider for MockProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let _model = &self.model;
        Ok(prompt.to_string())
    }
}

/// Ollama provider (blocking) using the local `ollama` CLI.
/// This keeps v0.1 dependency-light and works well for local prototyping.
#[derive(Debug, Clone)]
pub struct OllamaProvider {
    pub model: String,
    pub temperature: Option<f32>,
}

impl OllamaProvider {
    pub fn from_spec(spec: &adl::ProviderSpec, model_override: Option<&str>) -> Result<Self> {
        let target = provider_substrate::provider_invocation_target_v1(
            spec.id.as_deref().unwrap_or("<anonymous-provider>"),
            spec,
            model_override,
        )?;
        Self::from_target(spec, &target)
    }

    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let temperature = cfg_f32(&spec.config, "temperature");

        Ok(Self {
            // Local CLI execution has no separate provider-native model identifier surface,
            // so the stable model_ref is the runtime model we should actually invoke.
            model: target.model_ref.clone(),
            temperature,
        })
    }

    fn complete_streaming(
        &self,
        prompt: &str,
        mut on_chunk: Option<&mut dyn FnMut(&str)>,
    ) -> Result<String> {
        let timeout_secs =
            timeout_secs().map_err(|err| invalid_config("ollama", err.to_string()))?;

        // v0.1: We parse `temperature` from provider config for forward-compatibility,
        // but the `ollama` CLI does not consistently expose a stable flag across versions.
        // Read the field so it does not trip `-D dead-code`, and keep behavior deterministic.
        let _temperature = self.temperature;
        let mut child = Command::new(ollama_bin())
            .arg("run")
            .arg(&self.model)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| "failed to spawn `ollama run` (is Ollama installed and on PATH?)")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let stdout = child
            .stdout
            .take()
            .context("failed to open stdout for ollama")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        let stderr = child
            .stderr
            .take()
            .context("failed to open stderr for ollama")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let out_handle = thread::spawn(move || -> std::io::Result<()> {
            let mut r = stdout;
            let mut buf = [0u8; 4096];
            loop {
                let n = r.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                if tx.send(buf[..n].to_vec()).is_err() {
                    break;
                }
            }
            Ok(())
        });

        let err_handle = thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let mut r = stderr;
            let mut buf = Vec::new();
            r.read_to_end(&mut buf)?;
            Ok(buf)
        });

        {
            let mut stdin = child
                .stdin
                .take()
                .context("failed to open stdin for ollama")
                .map_err(|err| runtime_error("ollama", err.to_string()))?;
            stdin
                .write_all(prompt.as_bytes())
                .context("failed writing prompt to ollama stdin")
                .map_err(|err| runtime_error("ollama", err.to_string()))?;
            drop(stdin);
        }

        let start = Instant::now();
        let timeout = Duration::from_secs(timeout_secs);
        let mut out_buf = Vec::new();

        let status = loop {
            while let Ok(chunk) = rx.try_recv() {
                out_buf.extend_from_slice(&chunk);
                if let Some(cb) = on_chunk.as_deref_mut() {
                    cb(&String::from_utf8_lossy(&chunk));
                }
            }

            if let Some(status) = child
                .try_wait()
                .context("failed waiting for ollama process")
                .map_err(|err| runtime_error("ollama", err.to_string()))?
            {
                break status;
            }

            if start.elapsed() >= timeout {
                let _ = child.kill();
                let kill_start = Instant::now();
                loop {
                    if let Some(_status) = child
                        .try_wait()
                        .context("failed waiting for ollama process")
                        .map_err(|err| runtime_error("ollama", err.to_string()))?
                    {
                        break;
                    }
                    if kill_start.elapsed() >= Duration::from_secs(1) {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                return Err(timeout_error(
                    "ollama",
                    format!("timed out after {timeout_secs}s (set ADL_TIMEOUT_SECS to override)"),
                ));
            }

            std::thread::sleep(Duration::from_millis(10));
        };

        while let Ok(chunk) = rx.try_recv() {
            out_buf.extend_from_slice(&chunk);
            if let Some(cb) = on_chunk.as_deref_mut() {
                cb(&String::from_utf8_lossy(&chunk));
            }
        }

        out_handle
            .join()
            .map_err(|_| panic_error("ollama", "stdout reader thread panicked"))?
            .context("failed reading ollama stdout")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        let err_buf = err_handle
            .join()
            .map_err(|_| panic_error("ollama", "stderr reader thread panicked"))?
            .context("failed reading ollama stderr")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        if !status.success() {
            let stderr = String::from_utf8_lossy(&err_buf);
            return Err(runtime_error(
                "ollama",
                format!(
                    "ollama run failed (exit={:?}): {}",
                    status.code(),
                    stderr.trim()
                ),
            ));
        }

        let stdout = String::from_utf8(out_buf)
            .context("ollama output was not valid UTF-8")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        Ok(stdout)
    }
}

impl Provider for OllamaProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        self.complete_streaming(prompt, None)
    }

    fn complete_stream(&self, prompt: &str, on_chunk: &mut dyn FnMut(&str)) -> Result<String> {
        self.complete_streaming(prompt, Some(on_chunk))
    }
}

fn ollama_bin() -> PathBuf {
    // Allows tests (and power users) to override the binary path.
    // Defaults to `ollama` on PATH.
    std::env::var_os("ADL_OLLAMA_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("ollama"))
}

pub(crate) fn cfg_f32(cfg: &HashMap<String, Value>, key: &str) -> Option<f32> {
    cfg.get(key).and_then(|v| {
        if let Some(f) = v.as_f64() {
            Some(f as f32)
        } else if let Some(i) = v.as_i64() {
            Some(i as f32)
        } else if let Some(s) = v.as_str() {
            s.parse::<f32>().ok()
        } else {
            None
        }
    })
}
