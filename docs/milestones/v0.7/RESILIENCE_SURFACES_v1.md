# Resilience Surfaces v1 (v0.7)

`run_status.json` is the canonical deterministic resilience artifact for v0.7.

Location:
- `.adl/runs/<run_id>/run_status.json`

## Schema

Required fields:
- `run_status_version: 1`
- `run_id`
- `workflow_id`
- `overall_status`
- `completed_steps`
- `pending_steps`
- `attempt_counts_by_step`

Optional fields:
- `failure_kind`
- `failed_step_id`
- `started_steps`
- `effective_max_concurrency`
- `effective_max_concurrency_source`

## Overall Status Semantics

`overall_status` is one of:
- `running`
- `succeeded`
- `failed`
- `canceled`

v0.7 note:
- a paused run is emitted as `overall_status: "running"` because the run is incomplete but resumable

## Failure Taxonomy

`failure_kind` is stable and intentionally coarse. Current values are:
- `provider_error`
- `tool_error`
- `schema_error`
- `policy_denied`
- `sandbox_denied`
- `io_error`
- `timeout`
- `panic`

Rules:
- failure kinds are derived from structured/runtime error sources
- no raw provider stderr or endpoint strings are persisted in `run_status.json`
- no substring scraping of arbitrary error text is required for the artifact

## Resume Rules

Resume is deterministic and explicit:
- the engine reuses a `run_id` only when the user explicitly resumes that run
- a completed step is skipped only when:
  - the step is recorded as completed in pause state, and
  - its expected artifact is present and validates against the stored fingerprint
- if the expected artifact is missing or invalid, the step is rerun
- resume emits deterministic notes describing whether each previously completed step was skipped or rerun

Examples:
- `RESUME step=s1 action=skip reason=completed_artifact_verified`
- `RESUME step=s1 action=rerun reason=missing_expected_artifact`

## Determinism Guarantees

- no timestamps in `run_status.json`
- sorted lists/maps only
- stable scheduler policy source values
- artifact path is canonical via `RunArtifactPaths`
- file writes use `artifacts::atomic_write`

## Privacy / Security

`run_status.json` must not include:
- secrets
- raw provider output
- raw tool arguments
- absolute host paths

Resume does not weaken:
- sandbox boundaries
- signature / trust validation
- scheduler policy semantics
