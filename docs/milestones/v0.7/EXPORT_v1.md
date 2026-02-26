# Learning Export v1 (v0.7)

Learning Export v1 currently provides deterministic JSONL export for completed
run artifacts.

## CLI

`swarm learn export --format jsonl [--runs-dir <dir>] [--run-id <id> ...] --out <file>`

This issue intentionally ships JSONL-only export. Full bundle export
(directory/archive + `provenance.json`) is tracked as follow-up in #509.

## Row schema (v1)

Each JSONL line contains:

- `dataset_version`
- `run_id`, `workflow_id`, `adl_version`, `swarm_version`, `status`
- `feedback_present`
- `pointers` (stable hashes for available artifact files)
- `step_records[]`:
  - `step_id`, `provider_id`, `provider_profile`, `status`, `output_pointer_hash`
- `scores_summary` (if present)
- `suggestions_summary` (`ids`, `categories`)

## Determinism + safety

- run ordering is sorted by `run_id`
- step ordering is sorted by `step_id`
- suggestion IDs/categories are sorted deterministically
- export contains hashes/pointers, not raw large prompt payloads
- output excludes absolute host paths and token-like secrets

## Deferred

- Bundle export v1 (directory/archive packaging + `provenance.json`) is deferred to #509.
