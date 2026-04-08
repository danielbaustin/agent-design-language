# Codex CLI + Ollama Operational Skills Demo

This is a bounded, operator-facing demo for running the tracked operational
skills through Codex CLI against a local OSS provider, with Ollama as the
intended default and a tool-capable local model as the default execution path.

It is intentionally smaller than the full issue lifecycle. The proof target is:

- install the tracked skills from the repo
- run Codex CLI against a local Ollama-backed model
- invoke real tracked skills rather than ad hoc prose alone
- complete one bounded task similar to the card-cleanup work we do during real
  issue flow

## What It Proves

This demo proves:

- the tracked skills root under `adl/tools/skills` can be installed into a
  demo-local `CODEX_HOME`
- Codex CLI can be directed to use those installed skills
- a local Ollama model can be used for the session
- Codex can complete a small real task by applying the `stp-editor` and
  `sip-editor` skills to a prepared local bundle fixture
- the Codex run can be bounded to the copied fixture workspace rather than the
  live repository root

This demo does **not** claim:

- full GitHub issue bootstrap
- PR publication
- full `pr-init -> pr-ready -> pr-run -> pr-finish -> pr-closeout` automation
- that every Ollama model supports Codex tool calling equally well

## Prerequisites

- Codex CLI installed and available as `codex`
- local Ollama service running and reachable at `http://127.0.0.1:11434` by default
- the target local model pulled in Ollama
- repository checked out locally

Suggested baseline model:

```bash
ollama pull gpt-oss:latest
```

If you want a different local model, override `CODEX_OLLAMA_MODEL`.

DeepSeek remains an important target model for this demo, but current Ollama
DeepSeek variants may reject Codex tool calls with a provider-side
`does not support tools` error. The demo script now reports that failure
cleanly, while the most reliable local baseline is still a tool-capable model
such as `gpt-oss:latest`.

If your local Ollama API is not on the default host, set `OLLAMA_HOST` or
`OLLAMA_HOST_URL` before running the demo script.

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
4. run:

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

5. validate the edited `stp.md` and bootstrap-phase `sip.md`
6. write artifacts under `artifacts/v0871/codex_ollama_skills/`

Before the model call, the script checks the Ollama HTTP API directly at
`/api/tags` rather than depending on the `ollama` CLI.

The Codex working root is the copied fixture workspace. The prompt uses
fixture-relative paths only and explicitly tells the model to read with shell
commands and edit just the two target files with `apply_patch`. The live
repository root is added only as a reference surface so the demo stays bounded
to fixture edits.

Because this fixture is intentionally pre-run, the script validates the SIP
with `--phase bootstrap` after the edit rather than requiring a bound
execution branch.

## Artifact Layout

Primary artifact root:

- `artifacts/v0871/codex_ollama_skills/`

Key files:

- `demo_manifest.json`
- `codex_prompt.md`
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
- Local Ollama availability and model quality are operator-dependent.
- On machines where the local Ollama service is unavailable, the dry-run path
  still proves the install, prompt, fixture, and manifest surfaces.
- If the local model struggles, the dry-run path still proves the install and
  prompt-preparation surfaces.
- The fixture is intentionally card-focused because card editing is one of the
  most frequent bounded tasks in the operational workflow.
