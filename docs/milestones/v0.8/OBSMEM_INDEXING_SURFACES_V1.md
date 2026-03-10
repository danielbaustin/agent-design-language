# ObsMem Indexing Surfaces v1

## Purpose

This document defines deterministic, replay-compatible indexing surfaces for:

- `run_summary` (compact workflow-run index artifact)
- `experiment_index_entry` (normalized index entry derived from `ExperimentRecord`)

These artifacts are designed for search/retrieval and ranking preparation. They are not raw trace/log storage.

## Scope

This is a schema/specification artifact only. It does not implement a runtime indexing engine.

## Integrations

This spec integrates with:

- `ExperimentRecord` v1 (`EXPERIMENT_RECORD_V1.md`, #609)
- `Canonical Evidence View` v1 (`CANONICAL_EVIDENCE_VIEW_V1.md`, #610)
- `Mutation format` v1 (`MUTATION_FORMAT_V1.md`, #611)
- `EvaluationPlan` v1 (`EVALUATION_PLAN_V1.md`, #612)
- `Godel Experiment Workflow Template` v1 (`GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md`, #613)

## Design Principles

- Deterministic structure and ordering for identical inputs.
- Compact searchable metadata only (no raw logs/traces/prompts/tool args).
- Replay-compatible references to canonical artifacts.
- Explicit fields for hypothesis-aware retrieval and outcome ranking.
- Repo-relative paths only.

## Artifact 1: `run_summary`

Canonical schema file:

- `run_summary.v1.json`

Example artifact:

- `run_summary.v1.example.json`

### Required Fields

- `schema_version` (string, must be `run_summary.v1`)
- `run_id` (string)
- `workflow_id` (string)
- `workflow_digest` (string, deterministic digest of workflow definition)
- `outcome` (enum: `success`, `failure`, `partial`)
- `step_totals` (object with `planned`, `completed`, `failed`, `skipped`)
- `failure_classes` (sorted unique string array)
- `experiment_refs` (sorted unique string array)
- `hypothesis_refs` (sorted unique string array)
- `artifact_refs` (sorted array of `{kind, path, sha256}`)

### Optional Fields

- `notes` (string; must be non-secret and deterministic for same input source artifacts)

### Deterministic Ordering Rules

- `failure_classes`, `experiment_refs`, and `hypothesis_refs` must be lexicographically sorted and unique.
- `artifact_refs` must be sorted by tuple: `(kind, path, sha256)`.
- `step_totals` keys are fixed and always emitted in canonical order.

## Artifact 2: `experiment_index_entry`

Canonical schema file:

- `experiment_index_entry.v1.json`

Example artifact:

- `experiment_index_entry.v1.example.json`

### Required Fields

- `schema_version` (string, must be `experiment_index_entry.v1`)
- `experiment_id` (string)
- `run_id` (string)
- `workflow_id` (string)
- `failure_class` (string)
- `hypothesis_id` (string)
- `hypothesis_digest` (string)
- `mutation_id` (string)
- `evaluation_plan_id` (string)
- `decision` (enum: `adopt`, `reject`, `defer`)
- `outcome` (enum: `improved`, `regressed`, `neutral`, `failed`)
- `improvement_delta` (number or null; present for ranking/filtering compatibility)
- `experiment_seed` (string or null; preserves deterministic replay linkage)
- `evidence_refs` (sorted array of canonical evidence view IDs)
- `artifact_refs` (sorted array of `{kind, path, sha256}`)
- `sort_key` (object with deterministic tie-break fields)

### Optional Fields

- `related_experiments` (sorted unique string array)
- `tags` (sorted unique string array)

### Deterministic Ordering Rules

- `evidence_refs`, `related_experiments`, and `tags` are lexicographically sorted and unique.
- `artifact_refs` sorted by tuple: `(kind, path, sha256)`.
- `sort_key` is required and must contain:
  - `primary`: `failure_class`
  - `secondary`: `hypothesis_id`
  - `tertiary`: `outcome`
  - `quaternary`: `experiment_id`

## Hypothesis-Aware Retrieval Compatibility

These fields enable deterministic retrieval flows:

- Failure class -> hypothesis IDs:
  - `failure_class`, `hypothesis_id`, `hypothesis_digest`
- Hypothesis -> experiment outcomes:
  - `hypothesis_id`, `experiment_id`, `outcome`, `improvement_delta`, `decision`
- Reproducibility and replay linkage:
  - `experiment_seed`, `evaluation_plan_id`, `artifact_refs`

## Normalization Requirements

- All path references must be repo-relative and must not include absolute host paths.
- Evidence references must use canonical IDs from Canonical Evidence View artifacts.
- Unknown/optional source fields must be omitted or set to `null` deterministically according to schema type.
- No raw logs, full trace payloads, prompts, tool arguments, or secrets are persisted in these indexing artifacts.

## Security and Privacy

- Disallowed content:
  - secrets/tokens
  - raw prompts
  - tool arguments
  - absolute host paths
- Allowed content:
  - normalized IDs, hashes, bounded summary fields, and repo-relative references

## Non-goals

- Runtime indexing implementation
- Semantic/vector ranking implementation
- Raw trace persistence as index artifacts

