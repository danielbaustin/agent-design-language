# Mutation Format v1

## Purpose
`Mutation format v1` defines the canonical, bounded representation of a candidate change used by the v0.8 Godel experiment workflow.

The format is intentionally constrained so mutations can be:
- compared deterministically across candidates,
- evaluated before execution,
- recorded in experiment artifacts without embedding unconstrained patch language.

This is a schema/spec surface only. It does not imply runtime mutation execution.

## Canonical Artifact
- `schema_name`: `mutation`
- `schema_version`: `1`

Canonical machine-readable artifacts:
- `docs/milestones/v0.8/mutation.v1.json` (JSON Schema)
- `docs/milestones/v0.8/mutation.v1.example.json` (normative example)

## Determinism Contract
For identical normalized input, a mutation artifact must remain stable in:
- structure,
- field names,
- list ordering,
- comparison fields (`canonical_fingerprint`, `ordering_key`).

Deterministic ordering rules:
- `bounded_scope`: sorted lexicographically, unique entries only.
- `operations`: sorted by `op_id` (lexicographic) before serialization.
- `metadata.tags`: sorted lexicographically, unique entries only.
- tie-break for candidate ranking: `ordering_key` then `mutation_id` (both lexicographic).

## Security / Privacy Contract
A mutation artifact must not contain:
- secrets or tokens,
- raw prompts,
- tool arguments,
- absolute host paths.

All path references must be repository-relative.

## Required Fields
- `schema_name` (must be `mutation`)
- `schema_version` (must be `1`)
- `mutation_id` (stable candidate id)
- `experiment_id` (link to experiment scope)
- `hypothesis_id` (link to originating hypothesis)
- `mutation_type` (bounded enum)
- `bounded_scope` (non-empty list of allowed scope selectors)
- `operations` (non-empty list of bounded operations)
- `constraints` (policy/size boundaries)
- `comparison` (deterministic comparison surface)
- `safety` (allowed/prohibited surfaces)

## Optional Fields
- `evidence_ref` (linkage to Canonical Evidence View)
- `evaluation_plan_ref` (linkage to EvaluationPlan v1)
- `notes` (human-readable, non-sensitive)
- `metadata` (deterministic tags/provenance)

## Mutation Type (bounded enum)
Allowed values:
- `overlay_update`
- `evaluation_adjustment`
- `guardrail_adjustment`

Out of scope for v1:
- arbitrary patch languages,
- unconstrained file edits,
- runtime self-modification instructions.

## Operation Model
Each entry in `operations[]` is bounded and explicit.

Required operation fields:
- `op_id`: stable operation identifier
- `action`: `set`, `remove`, or `append_unique`
- `target_surface`: bounded enum (`workflow_overlay`, `evaluation_plan`, `provider_profile`, `delegation_policy`, `scheduler_policy`)
- `target_pointer`: JSON Pointer-style location under the target surface

Optional operation fields:
- `value`: value for `set` / `append_unique`
- `expected_old_value`: optional optimistic-compare guard

No raw diff/patch text is canonical in v1.

## Constraints Block
`constraints` defines bounded execution intent and review safety:
- `max_operations` (1..10)
- `policy_gate_required` (boolean)
- `sandbox_required` (boolean)
- `allow_create_new_paths` (boolean)

## Comparison Block
`comparison` is used for deterministic cross-candidate ordering:
- `canonical_fingerprint` (format: `sha256:<64-hex>`)
- `ordering_key` (stable lexicographic key)

## Safety Block
`safety` explicitly records policy envelope intent:
- `allowed_surfaces[]`
- `prohibited_surfaces[]`

v1 default prohibited surfaces should include security/trust-critical controls (unless explicitly approved by policy):
- `security_envelope`
- `signing_trust_policy`
- `artifact_validation_strictness`

## Relationship to Adjacent Artifacts

### ExperimentRecord (#609)
`ExperimentRecord` references the selected mutation via `mutation_id` and associated summary fields.
Mutation remains a standalone artifact and is not duplicated as free-form text.

### Canonical Evidence View (#610)
`evidence_ref` can link mutation origin to canonicalized evidence used for hypothesis generation.
This enables deterministic provenance without embedding raw trace payloads.

### EvaluationPlan (#612)
`evaluation_plan_ref` links a mutation to deterministic checks that must be run for acceptance/rejection.
Mutation describes **what to change**; EvaluationPlan describes **how to evaluate**.

## Non-goals
- runtime mutation executor,
- autonomous policy mutation,
- unconstrained code patch transport,
- semantic interpretation of free-form prompts.
