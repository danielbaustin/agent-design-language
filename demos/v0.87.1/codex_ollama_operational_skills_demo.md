# Codex CLI + Ollama Operational Skills Demo

This is a bounded, operator-facing demo for running the tracked operational
skills through Codex CLI against an Ollama-backed OSS provider, with a local
Ollama service as the default and a configured remote Ollama host also
supported at the demo-wrapper layer.

It is intentionally smaller than the full issue lifecycle. The proof target is:

- install the tracked skills from the repo
- run Codex CLI against an Ollama-backed model
- invoke real tracked skills rather than ad hoc prose alone
- complete one bounded task similar to the card-cleanup work we do during real
  issue flow

## What It Proves

This demo proves:

- the tracked skills root under `adl/tools/skills` can be installed into a
  demo-local `CODEX_HOME`
- Codex CLI can be directed to use those installed skills
- a local or configured remote Ollama-hosted model can be used for the session
- Codex can complete a small real task by applying the `stp-editor` and
  `sip-editor` skills to a prepared local bundle fixture
- the Codex run can be bounded to the copied fixture workspace rather than the
  live repository root
- the runtime can distinguish a native tool-capable local model from a
  non-tool local model and select the appropriate execution mode

This demo does **not** claim:

- full GitHub issue bootstrap
- PR publication
- full `pr-init -> pr-ready -> pr-run -> pr-finish -> pr-closeout` automation
- that every Ollama model supports Codex tool calling equally well

## Prerequisites

- Codex CLI installed and available as `codex`
- Ollama service running and reachable at `http://127.0.0.1:11434` by default, or at a configured remote host
- the target local model pulled in Ollama
- repository checked out locally

Suggested baseline model:

```bash
ollama pull gpt-oss:latest
```

If you want a different local model, override `CODEX_OLLAMA_MODEL`.

DeepSeek remains an important target model for this demo, but current Ollama
DeepSeek variants do not expose the same native tool-calling surface here. The
demo now models that explicitly through a capability declaration plus a runtime
semantic fallback path, while the most reliable native-tool baseline is still a
tool-capable model such as `gpt-oss:latest`.

If your Ollama API is not on the default host, set `OLLAMA_HOST` or
`OLLAMA_HOST_URL` before running the demo script.

Examples:

```bash
OLLAMA_HOST=192.168.68.73 \
bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run
```

```bash
OLLAMA_HOST_URL=http://192.168.68.73:11434 \
bash adl/tools/demo_codex_ollama_operational_skills.sh
```

This support is intentionally bounded to the demo wrapper. It does **not**
claim that the full ADL runtime `ollama` / `local_ollama` provider surfaces
already have first-class remote Ollama transport.

If you need a longer or shorter semantic-fallback wait window for a slower
local model, set `ADL_OLLAMA_GENERATE_TIMEOUT_SECS`.

## Fastest Smoke Path

Dry-run only:

```bash
bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run
```

That prepares:

- a demo-local `CODEX_HOME`
- the installed skills
- a copied local fixture workspace
- the exact Codex prompt file
- a JSON manifest describing the run inputs

No model call is made in dry-run mode.

## Full Demo Run

```bash
bash adl/tools/demo_codex_ollama_operational_skills.sh
```

To probe DeepSeek explicitly:

```bash
CODEX_OLLAMA_MODEL=deepseek-r1:latest \
bash adl/tools/demo_codex_ollama_operational_skills.sh
```

The script will:

1. install the tracked skills into `artifacts/v0871/codex_ollama_skills/codex_home`
2. copy a prepared local bundle fixture into
   `artifacts/v0871/codex_ollama_skills/workspace`
3. generate a Codex prompt that explicitly invokes:
   - `stp-editor`
   - `sip-editor`
4. resolve the model capability profile from
   `adl/tools/local_model_capabilities.v1.json`
5. choose one of two execution modes:
   - `native_tool_calling` for tool-capable local models such as `gpt-oss:latest`
   - `semantic_tool_fallback` for non-tool local models such as `deepseek-r1:latest`
6. run the bounded edit path appropriate to that capability mode

For the native tool path, the script runs:

```bash
codex exec \
  --full-auto \
  --oss \
  --local-provider ollama \
  --model gpt-oss:latest \
  --cd <artifact-root>/workspace \
  --sandbox workspace-write \
  --skip-git-repo-check \
  --add-dir <artifact-root>/workspace \
  --add-dir <artifact-root> \
  --add-dir <repo-root> \
  ...
```

For the semantic fallback path, the runtime sends a bounded prompt directly to
the local model, expects one strict JSON object containing full replacement
`stp.md` and `sip.md` content, validates it, and then applies the file writes
deterministically. The Ollama generate request is time-bounded so non-tool
models fail clearly instead of appearing to hang forever.

7. validate the edited `stp.md` and bootstrap-phase `sip.md`
8. write artifacts under `artifacts/v0871/codex_ollama_skills/`

Before the model call, the script checks the configured Ollama HTTP API directly at
`/api/tags` rather than depending on the `ollama` CLI.

The Codex working root is the copied fixture workspace. The prompt uses
fixture-relative paths only and explicitly tells the model to read with shell
commands and edit just the two target files with `apply_patch`. The live
repository root is added only as a reference surface so the demo stays bounded
to fixture edits.

Because this fixture is intentionally pre-run, the script validates the SIP
with `--phase bootstrap` after the edit rather than requiring a bound
execution branch.

The capability declaration is explicit in
`adl/tools/local_model_capabilities.v1.json`, and the run manifest records both
the selected capability profile and the execution mode that was used.

## Artifact Layout

Primary artifact root:

- `artifacts/v0871/codex_ollama_skills/`

Key files:

- `demo_manifest.json`
- `codex_prompt.md`
- `semantic_tool_fallback_prompt.md` when fallback mode is selected
- `semantic_tool_fallback_parsed.json` when fallback mode is selected
- `codex_events.jsonl`
- `codex_stdout.log`
- `codex_last_message.md`
- `workspace/.adl/v0.87.1/.../stp.md`
- `workspace/.adl/v0.87.1/.../sip.md`

## Why This Task

The fixture task is deliberately small and similar to the real work we use
these skills for:

- `stp-editor` tightens a weak task card
- `sip-editor` normalizes a sloppy but structurally valid input card

That gives a bounded proof that Codex CLI can:

- discover the installed skills
- follow their instructions
- perform concrete repo-style card cleanup

without requiring live GitHub issue creation or a full PR lifecycle.

## Truth Boundaries

- The demo uses a local fixture bundle, not a live GitHub issue.
- The demo is intended to be rerunnable and reviewer-friendly.
- Ollama host availability and model quality are operator-dependent.
- On machines where the local Ollama service is unavailable, the dry-run path
  still proves the install, prompt, fixture, and manifest surfaces.
- If the local model struggles, the dry-run path still proves the install and
  prompt-preparation surfaces, and the fallback path still records the
  capability distinction explicitly.
- The fixture is intentionally card-focused because card editing is one of the
  most frequent bounded tasks in the operational workflow.
