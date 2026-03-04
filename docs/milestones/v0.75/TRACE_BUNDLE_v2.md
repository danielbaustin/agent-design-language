# ADL v0.75 Trace Bundle v2 (WP-05)

## Purpose
Define the canonical, deterministic export contract for replay and downstream ingestion.

## Root layout
Bundle root directory:
- `trace_bundle_v2/`

Required files:
- `trace_bundle_v2/manifest.json`
- `trace_bundle_v2/runs/<run_id>/metadata.json`
- `trace_bundle_v2/runs/<run_id>/run.json`
- `trace_bundle_v2/runs/<run_id>/steps.json`
- `trace_bundle_v2/runs/<run_id>/run_summary.json`
- `trace_bundle_v2/runs/<run_id>/run_status.json`
- `trace_bundle_v2/runs/<run_id>/logs/activation_log.json`

Optional files:
- `trace_bundle_v2/runs/<run_id>/learning/scores.json`
- `trace_bundle_v2/runs/<run_id>/learning/suggestions.json`

## Manifest contract
`manifest.json` fields:
- `trace_bundle_version` (integer, value: `2`)
- `run_count` (integer)
- `runs` (sorted run id list)
- `files` (sorted by `path`, each entry includes):
  - `path` (bundle-relative path)
  - `hash` (deterministic content fingerprint)
  - `size_bytes` (file byte length)

Determinism rules:
- File entries are sorted lexicographically by `path`.
- JSON payloads are serialized deterministically from typed structs.
- Export output is byte-stable for identical source run artifacts.

## Security and hygiene rules
- Bundle payloads must not include absolute host paths (for example `/Users/...`, `/home/...`).
- Bundle payloads must not include token-like secret strings (for example `gho_`, `sk-`).
- Bundle payloads must not embed absolute exporter filesystem paths.
- Paths in the manifest are always relative to `trace_bundle_v2/`.

## CLI surface (v0.75)
Export command:
- `adl learn export --format trace-bundle-v2 --runs-dir .adl/runs --out /tmp/trace-bundle`

Compatibility note:
- `swarm` shim remains available during compatibility window.

## Replay sufficiency boundary
Trace Bundle v2 export provides replay-sufficient artifacts for WP-06 import/replay proof:
- frozen activation log (`logs/activation_log.json`)
- run state + plan/status surfaces (`run.json`, `run_status.json`, `run_summary.json`)
- step mapping (`steps.json`)

Output files and learning summaries are optional and not required for replay correctness.
