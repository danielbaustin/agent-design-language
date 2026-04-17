# Five-Command Editing Demo

This is the bounded, truthful proof surface for the historical five-command authoring lifecycle.

Run:

- `bash adl/tools/demo_five_command_editing.sh`

## What It Proves

The demo exercises, in order:

1. `pr init`
2. the validated legacy editor adapter dry-run for `pr start`
3. `pr start`
4. `pr run`
5. `pr finish`

The adapter surface is truthful to the compatibility contract:

- browser/editor direct support remains unavailable
- legacy command compatibility remains bounded to `adl/tools/editor_action.sh start`
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
- The editor/browser adapter remains thin; the current taught path prepares copy-only lifecycle commands, while this demo keeps the older `pr start` compatibility check alive.
- The demo does not claim direct browser execution for `pr init`, `pr run`, or `pr finish`.
- The finish step intentionally uses `--no-checks` so the demo proves lifecycle sequencing and artifact linkage rather than full CI/validation throughput.

## Why This Counts

This is not a hidden parallel workflow. It uses the real `adl/tools/pr.sh` command surface, plus the documented validated adapter surface from:

- `docs/tooling/editor/command_adapter.md`

So it is a bounded but honest end-to-end compatibility proof for the older authoring story.
