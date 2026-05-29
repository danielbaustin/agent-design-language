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

Hosted providers use `runtime_surface: "hosted_api"` and dispatch by `route.provider`:

- `openai` or `chatgpt`: OpenAI Responses API, default credential `OPENAI_API_KEY`.
- `anthropic` or `claude`: Anthropic Messages API, default credential `ANTHROPIC_API_KEY`.
- `google` or `gemini`: Gemini `generateContent` API, default credential `GEMINI_API_KEY` or `GOOGLE_API_KEY`.

Unsupported hosted providers are rejected instead of being silently routed through the wrong transport.

OpenAI example:

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

Claude example:

```json
{
  "route": {
    "provider_kind": "hosted",
    "provider": "anthropic",
    "provider_model_id": "claude-sonnet-4-5",
    "runtime_surface": "hosted_api",
    "credential_ref": "env:ANTHROPIC_API_KEY"
  },
  "model_identity": {
    "provider_kind": "hosted",
    "provider": "anthropic",
    "model_ref": "claude-sonnet-4-5",
    "provider_model_id": "claude-sonnet-4-5",
    "runtime_surface": "hosted_api",
    "identity_strength": "provider_asserted",
    "observed_at": "unix:1"
  },
  "prompt_contract_ref": "provider_adapter.live_smoke.v1",
  "lane_ref": "live_smoke",
  "input_text": "Return exactly: ok",
  "attempt_policy": {
    "timeout_ms": 120000,
    "max_attempts": 1,
    "retry_backoff_ms": 1000
  }
}
```

Gemini example:

```json
{
  "route": {
    "provider_kind": "hosted",
    "provider": "gemini",
    "provider_model_id": "gemini-2.5-flash",
    "runtime_surface": "hosted_api",
    "credential_ref": "env:GEMINI_API_KEY"
  },
  "model_identity": {
    "provider_kind": "hosted",
    "provider": "gemini",
    "model_ref": "gemini-2.5-flash",
    "provider_model_id": "gemini-2.5-flash",
    "runtime_surface": "hosted_api",
    "identity_strength": "provider_asserted",
    "observed_at": "unix:1"
  },
  "prompt_contract_ref": "provider_adapter.live_smoke.v1",
  "lane_ref": "live_smoke",
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

Live smoke results recorded during `#3486`:

| Provider | Model | Result | Notes |
| --- | --- | --- | --- |
| OpenAI | `gpt-5.3-codex` | `ok`, HTTP 200, ~4.9s | Returned `adapter ok`; credential was supplied from a local env var only. |
| Anthropic | `claude-sonnet-4-5` | `ok`, HTTP 200, ~1.6s | Returned `adapter ok`; credential was supplied from a local env var only. |
| Gemini | `gemini-2.5-flash` | `ok`, HTTP 200, ~1.1s | Transport/extraction succeeded; model returned `hello world` despite the exact-output prompt. |
| Ollama | `gemma:2b` | `ok`, HTTP 200, ~2.7s | Local identity pinned from `/api/tags`; model was stopped after the smoke. |

This adapter does not start or stop Ollama models. Model lifecycle control belongs to the benchmark runner or operator script.

## UTS Benchmark Runner Integration

`adl/tools/uts_benchmark_runner.py` routes regular and UTS-only provider calls through `adl-provider-adapter`. The Python runner remains responsible for task prompts, scoring, lane summaries, hosted auth setup, local Ollama residency checks, and benchmark artifacts. The Rust adapter owns only provider execution, normalized provider results, model identity, and redacted JSONL adapter logs.

For each regular or UTS-only model call, the runner retains redacted adapter evidence beside the benchmark output under `<output_stem>_adapter_evidence/`. These retained files intentionally omit raw prompts and full model outputs while preserving the normalized adapter result, model identity, final status, attempts, and tail-friendly adapter JSONL log reference. Each benchmark case records the retained `adapter_result`, `adapter_run_log`, `adapter_final_status`, and `adapter_model_identity` fields.

Adapter requests carry the benchmark lane as `lane_ref` (`regular` or `uts_only`) so adapter logs and retained evidence can be inspected without guessing which scoring lane produced a provider call.

Offline runner-to-adapter smoke:

```bash
python3 adl/tools/uts_benchmark_runner.py --adapter-smoke-self-test
```

Expected output is a compact JSON object with `status: "ok"`. The smoke starts an in-process fake OpenAI-compatible endpoint, invokes the Rust adapter, and does not require live credentials.
