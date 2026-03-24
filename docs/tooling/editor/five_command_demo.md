# Five-Command Editing Demo

This is the bounded, truthful proof surface for the full v0.85 five-command editing lifecycle.

Run:

- `bash adl/tools/demo_five_command_editing.sh`

## What It Proves

The demo exercises, in order:

1. `pr init`
2. `pr create`
3. the validated editor adapter dry-run for `pr start`
4. `pr start`
5. `pr run`
6. `pr finish`

The adapter surface is truthful to the current editor contract:

- browser/editor direct support remains bounded to `adl/tools/editor_action.sh start`
- the broader lifecycle commands still run through the repo-local control plane

## Proof Surface

The demo emits one manifest path at the end:

- `five-command editing demo manifest: <path>`

The manifest records:

- per-step command transcripts
- the emitted STP/input/output card paths
- the canonical run artifacts:
  - `run.json`
  - `run_status.json`
  - `run_summary.json`
- the mocked GitHub transcript and reconciled issue body
- the tracked file used by the bounded `pr finish` step

## Truth Boundaries

- The demo uses mocked GitHub and mocked model-provider surfaces to keep the proof local and deterministic.
- The editor/browser adapter remains thin and only prepares the `pr start` action.
- The demo does not claim direct browser execution for `pr init`, `pr create`, `pr run`, or `pr finish`.
- The finish step intentionally uses `--no-checks` so the demo proves lifecycle sequencing and artifact linkage rather than full CI/validation throughput.

## Why This Counts

This is not a hidden parallel workflow. It uses the real `adl/tools/pr.sh` command surface, plus the documented validated adapter surface from:

- `docs/tooling/editor/command_adapter.md`

So it is a bounded but honest end-to-end proof of the current v0.85 editing story.
