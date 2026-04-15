# Rust-Native Live ChatGPT + Claude Long-Form Discussion Demo

This demo replaces the v0.87.1 Python proof shim with Rust-native OpenAI and
Anthropic provider invocation, and expands the conversation into a 20-turn,
act-structured tea discussion.

## Command

```bash
bash adl/tools/demo_v088_real_multi_agent_discussion.sh
```

## Credentials

The demo uses operator-managed credentials only:

- `OPENAI_API_KEY` and `ANTHROPIC_API_KEY` when already set
- otherwise `$HOME/keys/openai2.key` for OpenAI and `$HOME/keys/claude.key` for Anthropic
- explicit `ADL_OPENAI_KEY_FILE` and `ADL_ANTHROPIC_KEY_FILE` overrides when the operator chooses different key-file paths

Secret values, key-file paths, and raw credential headers must not be printed
or written to generated artifacts.

## Proof Surfaces

Default artifact root:

```text
artifacts/v088/real_multi_agent_discussion/
```

Primary proof surfaces:

- `transcript.md`
- `synthesis.md`
- `provider_invocations.json`
- `runtime/runs/v0-88-real-multi-agent-tea-discussion/run_summary.json`

The provider invocation artifact records provider family, model, HTTP status,
timestamp, and prompt/output character counts. It intentionally does not record
prompt bodies, response bodies, API keys, or Authorization headers.

## Conversation Shape

The flagship version is intentionally longer and more interesting than the
original five-turn exchange.

- 20 explicit turns
- 5 acts: opening positions, first exchange, deepening, convergence work, closing
- `ChatGPT` framed as `Builder`
- `Claude` framed as `Reflective Critic`
- one visible disagreement and one explicit synthesis phase

The transcript is still bounded and reviewable. This is not a general
conversation runtime claim.
