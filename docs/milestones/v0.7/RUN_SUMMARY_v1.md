# run_summary.json v1 (v0.7)

`run_summary.json` is the canonical deterministic run summary artifact for v0.7 learning surfaces.

Location:
- `.adl/runs/<run_id>/run_summary.json`

Versioning:
- `run_summary_version: 1`
- `artifact_model_version: 1`

## Schema (v1)

Top-level fields:
- `run_summary_version: number`
- `artifact_model_version: number`
- `run_id: string`
- `workflow_id: string`
- `adl_version: string`
- `swarm_version: string`
- `status: "success" | "failure" | "paused"`
- `error_kind: string | null`
- `counts: object`
- `policy: object`
- `links: object`

### counts
- `total_steps: number`
- `completed_steps: number`
- `failed_steps: number`
- `provider_call_count: number`
- `delegation_steps: number`
- `delegation_requires_verification_steps: number`

### policy
- `security_envelope_enabled: bool`
- `signing_required: bool`
- `key_id_required: bool`
- `verify_allowed_algs: string[]` (sorted/deduped)
- `verify_allowed_key_sources: string[]` (sorted/deduped)
- `sandbox_policy: string`
- `security_denials_by_code: object<string, number>` (stable key order)

### links (run-root relative)
- `run_json`
- `steps_json`
- `pause_state_json` (present for paused runs)
- `outputs_dir`
- `logs_dir`
- `learning_dir`
- `scores_json`
- `suggestions_json`
- `overlays_dir`
- `trace_json` (currently `null`/absent in v1 emission)

## Determinism rules

- Path is canonical via Artifact Model v1 (`RunArtifactPaths`).
- No wall-clock timestamps are emitted in run_summary by default.
- Stable field names and stable error-kind extraction.
- Sorted/deduped policy lists prevent order drift.
- `security_denials_by_code` uses stable map ordering.

## Notes

- This artifact is intentionally minimal and audit-friendly.
- It is a substrate for #483–#486 and demo/export workflows.
