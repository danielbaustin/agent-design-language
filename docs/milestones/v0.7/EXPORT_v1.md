# Learning Export Bundle v1 (v0.7)

Learning export now supports two deterministic formats:

- `jsonl` for row-oriented dataset export
- `bundle-v1` for portable directory export with manifest + per-run summaries

## CLI

`adl learn export --format <jsonl|bundle-v1> [--runs-dir <dir>] [--run-id <id> ...] --out <path>`

Notes:
- For `jsonl`, `--out` is a file path.
- For `bundle-v1`, `--out` is a directory root, and export writes under `learning_export_v1/`.

## Bundle v1 contract

Root layout:

- `learning_export_v1/manifest.json`
- `learning_export_v1/runs/<run_id>/metadata.json`
- `learning_export_v1/runs/<run_id>/step_records.json`
- `learning_export_v1/runs/<run_id>/suggestions_summary.json`
- `learning_export_v1/runs/<run_id>/scores_summary.json` (optional, only if scores exist)

Manifest contract (`manifest.json`):

- `bundle_version: 1`
- `run_count`
- sorted `runs[]`
- sorted `files[]` entries with:
  - `path` (bundle-relative)
  - `hash` (stable FNV-1a fingerprint of emitted JSON bytes)

Per-run metadata contract (`metadata.json`):

- `bundle_run_version: 1`
- `run_id`, `workflow_id`, `adl_version`, `swarm_version`, `status`
- `feedback_present`
- `pointers` map of source artifact hashes (`run_summary_hash`, `steps_hash`, optional `scores_hash`, optional `suggestions_hash`)

## Determinism and replay safety

- run IDs are sorted deterministically
- step records are sorted by `step_id`
- suggestion IDs/categories are sorted deterministically
- manifest file list is sorted by relative path
- exported values are derived from persisted run artifacts and stable hashes
- no absolute host paths are emitted in bundle files

## Security and redaction behavior

Default behavior excludes high-risk raw payloads:

- no raw prompt/tool transcripts exported
- no provider credentials exported
- no absolute host paths exported
- token-like secret marker `gho_` is rejected in emitted payload checks

Redaction is structural and deterministic:

- step output paths are converted to stable pointer hashes (`output_pointer_hash`)
- bundle carries references/hashes, not raw secret-bearing content

## JSONL v1 schema (unchanged)

Each JSONL row includes:

- `dataset_version`
- `run_id`, `workflow_id`, `adl_version`, `swarm_version`, `status`
- `feedback_present`
- `pointers`
- `step_records[]`
- `scores_summary` (optional)
- `suggestions_summary`
