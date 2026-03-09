# ExperimentRecord Schema v1

Status: Canonical design-stage schema/spec for issue #609 (`ExperimentRecord schema v1`)

## Purpose

`ExperimentRecord v1` is the canonical artifact that captures one baseline-vs-variant experiment outcome in a deterministic, replay-compatible, and indexing-safe form.

This schema is designed to be strong enough for direct downstream use by:
- #610 Canonical Evidence View
- #611 Mutation format v1
- #612 EvaluationPlan v1
- #614/#615 ObsMem indexing and demo path

## Design Invariants

- Deterministic replay compatibility: all references use stable IDs and repo-relative paths.
- Deterministic comparison compatibility: baseline and variant outcomes are represented side-by-side with explicit metric deltas.
- Privacy/safety: no secrets, tokens, raw prompts, tool arguments, or absolute host paths.
- Canonical ordering: arrays that affect comparison/indexing must be sorted as defined below.

## Canonical Artifact Location

- Recommended path pattern: `artifacts/experiments/<experiment_id>/experiment_record.v1.json`
- Paths embedded in this schema must be repository-relative.

## Top-Level Shape

`ExperimentRecord v1` is a JSON object with `schema_name = "experiment_record"` and `schema_version = 1`.

Required top-level fields:
- `schema_name`
- `schema_version`
- `experiment_id`
- `comparison_key`
- `runs`
- `hypothesis`
- `mutation`
- `evaluation_plan`
- `evidence`
- `outcome`
- `decision`
- `replay`
- `obsmem_index`

Optional top-level fields:
- `policy_approval_ref`
- `notes`

## Field Semantics

### `schema_name` (required)
- Type: string
- Value: `"experiment_record"`
- Why: explicit artifact-type discriminator for parsing/indexing.

### `schema_version` (required)
- Type: integer
- Value: `1`
- Why: deterministic schema evolution boundary.

### `experiment_id` (required)
- Type: string
- Pattern: `^[a-z0-9][a-z0-9._-]{2,127}$`
- Why: stable join key across evidence/mutation/evaluation/ObsMem.

### `comparison_key` (required)
- Type: object
- Required fields:
  - `subject`: stable experiment subject label (for example profile/module under test)
  - `baseline_label`: stable baseline label
  - `variant_label`: stable variant label
- Why: defines deterministic comparison identity independent of human prose.

### `runs` (required)
- Type: object
- Required fields:
  - `baseline_run_id`
  - `variant_run_id`
- Why: canonical linkage to replay/run artifacts.

### `hypothesis` (required)
- Type: object
- Required fields:
  - `statement`: natural-language hypothesis
  - `expected_effect`: concise machine-readable expected direction/outcome
- Optional fields:
  - `risk_class` (`low|medium|high`)
  - `success_criteria` (ordered list)
- Why: captures intent and acceptance framing without embedding prompts.

### `mutation` (required)
- Type: object
- Required fields:
  - `mutation_id`
  - `mutation_version`
  - `mutation_ref` (repo-relative path/URI to canonical Mutation artifact)
  - `scope` (ordered list of repo-relative paths or logical targets)
- Optional fields:
  - `change_summary` (ordered list of bounded, human-readable bullet strings)
- Why: directly compatible with #611 while keeping ExperimentRecord self-descriptive.

### `evaluation_plan` (required)
- Type: object
- Required fields:
  - `evaluation_plan_id`
  - `evaluation_plan_version`
  - `evaluation_plan_ref` (repo-relative path/URI)
  - `check_ids` (ordered list of deterministic check identifiers)
- Why: direct contract bridge to #612 and deterministic evaluation reproducibility.

### `evidence` (required)
- Type: object
- Required fields:
  - `canonical_evidence_view_id`
  - `canonical_evidence_view_version`
  - `canonical_evidence_ref` (repo-relative path/URI)
  - `evidence_items` (ordered list)
- `evidence_items` item required fields:
  - `evidence_id`
  - `kind` (`trace|verification|artifact_hash|failure_code|metric`)
  - `value`
  - `source_ref` (repo-relative)
- Why: designed to consume #610 directly while remaining indexable.

### `outcome` (required)
- Type: object
- Required fields:
  - `primary_metric`
  - `direction` (`increase_is_better|decrease_is_better|target_match`)
  - `baseline_value` (number)
  - `variant_value` (number)
  - `delta` (number)
- Optional fields:
  - `secondary_metrics` (ordered list of same shape)
- Why: deterministic experiment comparison for automation and review.

### `decision` (required)
- Type: object
- Required fields:
  - `result` (`adopt|reject|requires_human_review`)
  - `rationale`
- Optional fields:
  - `blocking_reasons` (ordered list)
- Why: explicit decision boundary with machine-readable disposition.

### `replay` (required)
- Type: object
- Required fields:
  - `replay_profile` (for example `strict`)
  - `artifact_manifest` (ordered list of `{ path, sha256 }`)
- Optional fields:
  - `replay_notes`
- Why: preserves replay/artifact invariants and deterministic integrity checks.

### `obsmem_index` (required)
- Type: object
- Required fields:
  - `memory_kind` (value: `experiment_record`)
  - `index_key` (stable key derived from experiment_id + comparison_key)
  - `tags` (sorted ascending, unique)
  - `facets` (object)
- Required `facets` fields:
  - `decision_result`
  - `primary_metric`
  - `subject`
- Why: supports #614/#615 indexing and demo retrieval without schema churn.

### `policy_approval_ref` (optional)
- Type: string (repo-relative path/URI or policy decision ID)
- Why: link to delegated policy approval when applicable.

### `notes` (optional)
- Type: ordered list of strings
- Why: bounded human context that does not affect deterministic comparison.

## Deterministic Ordering Rules

- `evidence.evidence_items` sorted by `evidence_id` ascending.
- `evaluation_plan.check_ids` ordered execution list, stable across reruns.
- `mutation.scope` sorted lexicographically ascending.
- `obsmem_index.tags` sorted lexicographically ascending and de-duplicated.
- `replay.artifact_manifest` sorted by `path` ascending.

## Security and Privacy Rules

The schema forbids storing:
- secrets or access tokens
- raw prompts
- tool argument payloads
- absolute host paths

All artifact references must be repository-relative.

## Versioning Rules

- `schema_name` is immutable for this artifact family.
- `schema_version = 1` is frozen once accepted.
- Any breaking field or semantic change increments `schema_version`.
- Additive non-breaking fields in future versions must remain optional.

## Canonical Machine-Readable Artifacts

- Schema: `docs/milestones/v0.8/experiment_record.v1.schema.json`
- Example: `docs/milestones/v0.8/experiment_record.v1.example.json`
