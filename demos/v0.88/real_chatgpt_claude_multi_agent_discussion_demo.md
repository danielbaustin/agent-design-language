# Rust-Native Live ChatGPT + Claude Multi-Agent Discussion Demo

This demo replaces the v0.87.1 Python proof shim with Rust-native OpenAI and
Anthropic provider invocation.

## Command

```bash
bash adl/tools/demo_v088_real_multi_agent_discussion.sh
```

## Credentials

The demo uses operator-managed credentials only:

- `OPENAI_API_KEY` if already set, otherwise `$HOME/keys/openai.key`
- `ANTHROPIC_API_KEY` if already set, otherwise `$HOME/keys/claude.key`

Secret values and raw credential headers must not be printed or written to
generated artifacts.

## Proof Surfaces

Default artifact root:

```text
artifacts/v088/real_multi_agent_discussion/
```

Primary proof surfaces:

- `transcript.md`
- `provider_invocations.json`
- `runtime/runs/v0-88-real-multi-agent-tea-discussion/run_summary.json`

The provider invocation artifact records provider family, model, HTTP status,
timestamp, and prompt/output character counts. It intentionally does not record
prompt bodies, response bodies, API keys, or Authorization headers.
