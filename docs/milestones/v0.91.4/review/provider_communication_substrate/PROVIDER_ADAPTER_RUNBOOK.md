# Provider Adapter Runbook

## Status

`adl-provider-adapter` is the Rust execution boundary for one provider invocation. It consumes the shared provider communication substrate and writes both a structured result JSON file and a tail-friendly JSONL run log.

## Command

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- \
  --request /path/to/request.json \
  --out /path/to/result.json \
  --log /path/to/run.log.jsonl
```

The command prints:

```text
result=/path/to/result.json
run_log=/path/to/run.log.jsonl
watch=tail -f /path/to/run.log.jsonl
```

Provider failures are normalized into the result file. The process exits non-zero only for adapter I/O or request parsing failures.

## Request Shape

The request is `ProviderInvocationRequestV1`. The adapter requires `input_text` for execution while preserving existing `model_ref` / `provider_model_id` compatibility.

Hosted OpenAI example. The current hosted adapter intentionally supports `provider: "openai"` only; other hosted providers must use a future provider-specific transport rather than being silently routed through OpenAI.

```json
{
  "request_id": "req-001",
  "run_id": "run-001",
  "route": {
    "provider_kind": "hosted",
    "provider": "openai",
    "provider_model_id": "gpt-5.3-codex",
    "runtime_surface": "hosted_api",
    "credential_ref": "env:OPENAI_API_KEY"
  },
  "input_text": "Return exactly: ok",
  "attempt_policy": {
    "timeout_ms": 120000,
    "max_attempts": 1,
    "retry_backoff_ms": 1000
  }
}
```

Ollama HTTP example. The adapter calls `/api/show`, then falls back to `/api/tags`, before `/api/generate` and records a pinned model identity when Ollama returns a digest.

```json
{
  "request_id": "req-ollama-001",
  "run_id": "run-ollama-001",
  "route": {
    "provider_kind": "local",
    "provider": "ollama",
    "provider_model_id": "gemma3:27b",
    "runtime_surface": "ollama_http",
    "endpoint_ref": "http://127.0.0.1:11434"
  },
  "input_text": "Return exactly: ok",
  "attempt_policy": {
    "timeout_ms": 120000,
    "max_attempts": 1,
    "retry_backoff_ms": 1000
  }
}
```

## Outputs

- `result.json`: one `ProviderInvocationResultV1`, including final status, normalized failure if any, attempts, model identity, and output text on success.
- `run.log.jsonl`: one flushed JSON object per event for `tail -f` monitoring.

Logs intentionally do not contain raw prompt text, credentials, or raw model output. Result files may contain successful model output because callers need the text for scoring; do not publish result artifacts without normal benchmark redaction.

## Live Checks

Live checks are opt-in. For hosted calls, set the credential env var named by `credential_ref`. For Ollama, make sure the model is already available and prefer a single local model at a time.

This adapter does not start or stop Ollama models. Model lifecycle control belongs to the benchmark runner or operator script.
