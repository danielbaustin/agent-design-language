# Canonical Evidence View v1

Status: Canonical design-stage schema/spec for issue #610 (`Canonical Evidence View`)

## Purpose

`Canonical Evidence View v1` is the deterministic evidence normalization artifact used by the Gödel workflow.

It defines a stable, reviewable representation of evidence for a run/experiment so downstream artifacts can compare outcomes without relying on volatile runtime fields.

## Design Invariants

- Deterministic ordering for all comparison-relevant lists.
- Replay-compatible evidence linkage via repo-relative artifact references.
- No secrets, tokens, raw prompts, tool arguments, or absolute host paths.
- Canonicalization metadata is explicit so consumers can reproduce normalization decisions.

## Canonical Artifact Location

Recommended path pattern:

- `artifacts/evidence/<evidence_view_id>/canonical_evidence_view.v1.json`

All references inside the artifact are repository-relative.

## Top-Level Shape

`Canonical Evidence View v1` is a JSON object with:

- `schema_name = "canonical_evidence_view"`
- `schema_version = 1`

Required top-level fields:

- `schema_name`
- `schema_version`
- `evidence_view_id`
- `run_context`
- `canonicalization_profile`
- `failure_codes`
- `verification_results`
- `artifact_hashes`
- `trace_bundle_ref`
- `activation_log_ref`
- `comparison_axes`
- `privacy`

Optional top-level fields:

- `derived_metrics`
- `notes`

## Field Semantics

### `evidence_view_id` (required)

Stable evidence-view identifier.

### `run_context` (required)

Required fields:

- `run_id`
- `workflow_id`
- `subject`

Optional:

- `variant_label` (for baseline/variant comparisons)

### `canonicalization_profile` (required)

Required fields:

- `profile_name`
- `profile_version`
- `volatile_fields_excluded` (ordered list)

Defines exactly what was excluded/normalized.

### `failure_codes` (required)

Ordered list of stable failure codes observed in canonical evidence.

### `verification_results` (required)

Ordered list of verification check results.

Each item required fields:

- `check_id`
- `status` (`pass|fail|not_applicable`)
- `evidence_ref` (repo-relative)

### `artifact_hashes` (required)

Ordered list of `{ path, sha256 }` objects for canonical evidence integrity.

### `trace_bundle_ref` (required)

Repo-relative reference to trace bundle used as an evidence source.

### `activation_log_ref` (required)

Repo-relative reference to activation log artifact used as an evidence source.

### `comparison_axes` (required)

Required fields:

- `primary_metric`
- `direction` (`increase_is_better|decrease_is_better|target_match`)

Optional:

- `secondary_metrics` (ordered list)

### `privacy` (required)

Required fields:

- `secrets_present` (must be false)
- `raw_prompt_or_tool_args_present` (must be false)
- `absolute_host_paths_present` (must be false)

Optional:

- `redaction_notes` (ordered list)

### `derived_metrics` (optional)

Ordered list of normalized metric summaries for downstream ranking.

### `notes` (optional)

Ordered list of bounded human-readable notes.

## Deterministic Ordering Rules

- `failure_codes`: lexicographic ascending.
- `verification_results`: ordered by `check_id` ascending.
- `artifact_hashes`: ordered by `path` ascending.
- `comparison_axes.secondary_metrics`: ordered by metric id/name ascending.
- `canonicalization_profile.volatile_fields_excluded`: lexicographic ascending.

## Relationships to Neighbor Artifacts

### Relation to `ExperimentRecord` (#609)

`ExperimentRecord.evidence` references this artifact as the canonical evidence source (`canonical_evidence_ref`).

`ExperimentRecord` holds hypothesis/mutation/decision context; `Canonical Evidence View` holds normalized evidence surfaces used for comparison.

### Relation to `Mutation` (#611)

Mutation artifacts define what changed. `Canonical Evidence View` records what evidence was observed after mutation execution, using deterministic normalization.

### Relation to `EvaluationPlan` (#612)

`verification_results` and `comparison_axes` are expected to align with EvaluationPlan check identifiers and metric semantics.

## Security / Privacy Rules

Forbidden in this artifact:

- secrets or credentials
- raw prompts
- tool argument payloads
- absolute host paths

References must be repository-relative.

## Versioning Rules

- `schema_name` is immutable for this artifact family.
- `schema_version = 1` is the initial stable version.
- Breaking changes require a schema version increment.
- Additive future fields must remain optional.

## Canonical Machine-Readable Artifacts

- Schema: `docs/milestones/v0.8/canonical_evidence_view.v1.schema.json`
- Example: `docs/milestones/v0.8/canonical_evidence_view.v1.example.json`
