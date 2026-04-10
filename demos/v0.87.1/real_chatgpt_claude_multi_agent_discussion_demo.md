# Live ChatGPT + Claude Multi-Agent Discussion Demo

This is the live-provider companion to the deterministic v0.87.1 tea-discussion
demo.

The deterministic demo remains the CI-safe proof surface. This live demo proves
that the same bounded multi-agent shape can call real OpenAI and Anthropic
models through ADL's current local HTTP completion boundary.

Proof boundary:
- A credentialed run that completes and writes the named invocation/transcript
  artifacts is the proving path for D13L.
- A no-credential run of
  `adl/tools/test_demo_v0871_real_multi_agent_discussion.sh` may exit
  successfully for local ergonomics, but that outcome is an explicit
  non-proving skip disposition rather than live-provider proof.

## Command

```bash
bash adl/tools/demo_v0871_real_multi_agent_discussion.sh
```

## Credentials

The demo uses operator-managed credentials only:

- `OPENAI_API_KEY` if already set, otherwise `$HOME/keys/openai.key`
- `ANTHROPIC_API_KEY` if already set, otherwise `$HOME/keys/claude.key`

The scripts must not print, persist, or commit secret values. Generated proof
artifacts record provider family, model id, status, request-id presence, timing,
and prompt/output lengths only.

## Proof Surfaces

Default artifact root:

```text
artifacts/v0871/real_multi_agent_discussion/
```

Primary proof surfaces:

- `transcript.md`
- `provider_invocations.json`
- `runtime/runs/v0-87-1-real-multi-agent-tea-discussion/run_summary.json`

Secondary proof surfaces:

- `transcript_contract.json`
- `runtime/runs/v0-87-1-real-multi-agent-tea-discussion/steps.json`
- `runtime/runs/v0-87-1-real-multi-agent-tea-discussion/logs/trace_v1.json`
- `demo_manifest.json`
- `run_log.txt`

## Scope

This is not a general conversation runtime and it is not a CI-required test. The
local adapter exists only because ADL's current HTTP provider expects a bounded
completion contract:

```json
{"prompt": "..."} -> {"output": "..."}
```

The adapter translates that contract to vendor-native OpenAI and Anthropic API
calls, then returns the text response to ADL.
