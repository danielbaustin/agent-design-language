# ToolResult Contract v1

## Purpose
Define a deterministic, replay-safe, and privacy-safe contract for tool execution outputs consumed by ADL workflows and review surfaces.

In current v0.8 runtime code, this contract is enforced at the bounded `adl learn export` command boundary. Successful exports emit a validated `tool_result.v1` sidecar describing the export outcome and its primary artifact reference.

## Schema Identity
- Schema ID: `tool_result.v1`
- Canonical schema file: `adl-spec/schemas/v0.8/tool_result.v1.schema.json`
- Canonical example file: `adl-spec/examples/v0.8/tool_result.v1.example.json`

## Contract Shape

### Required fields
- `schema_version` (string): must be `tool_result.v1`.
- `tool_name` (string): stable tool identifier.
- `invocation_id` (string): stable invocation ID within a run.
- `status` (enum): `success | failure | partial`.
- `artifact_refs` (array): zero or more replay-safe artifact references.

### Optional fields
- `payload` (json value): bounded structured output.
- `error` (object): stable failure descriptor.
- `metadata` (object): bounded machine-readable metadata.

## Status Semantics
- `success`
  - The tool operation completed successfully.
  - `error` must be absent.
- `failure`
  - The operation failed.
  - `error` is required with stable `code` and safe `message`.
- `partial`
  - The operation produced usable partial output but did not fully complete.
  - `payload` is required.
  - `error` is optional, but when present must use stable code semantics.

## Error Contract
`error` fields:
- `code` (required): stable, machine-readable identifier.
- `message` (required): safe, human-readable summary.
- `category` (optional): coarse deterministic bucket.

Unknown error strings from providers/tools must not be treated as stable contract identifiers.

## Artifact Reference Contract
Each `artifact_refs[]` entry includes:
- `kind` (required): logical artifact kind (for example `trace`, `report`, `log`, `output`).
- `path` (required): repo-relative path only.
- `sha256` (required): lowercase 64-hex content hash.

Absolute host paths are disallowed.

## Metadata Contract
`metadata` is optional and bounded.

Recommended keys (when available):
- `duration_ms` (number)
- `attempt` (number)
- `provider` (string)

Metadata must not include secrets, raw prompts, raw tool arguments, or environment dumps.

## Determinism Requirements
For identical tool inputs and environment:
- `status` semantics are stable.
- `error.code` remains stable.
- `artifact_refs` ordering is deterministic:
  1. sort by `kind` (lexicographic)
  2. then by `path` (lexicographic)
  3. then by `sha256` (lexicographic)
- Map-like metadata emitted into persisted artifacts must use stable key ordering.

## Security / Privacy
The contract explicitly forbids:
- secrets/tokens
- raw prompts
- raw tool arguments
- absolute host paths

If sensitive details are needed for debugging, store sanitized references and hashes only.

## Relationship to v0.8 Gödel Surfaces
ToolResult v1 is compatible with:
- `ExperimentRecord` (`adl-spec/schemas/v0.8/experiment_record.v1.schema.json`)
- `Canonical Evidence View` (`adl-spec/schemas/v0.8/canonical_evidence_view.v1.schema.json`)
- `EvaluationPlan` (`adl-spec/schemas/v0.8/evaluation_plan.v1.json`)

It provides an explicit result envelope that downstream evidence/review flows can compare without parsing ad hoc text blobs.

## Non-goals
- Defining tool orchestration policy.
- Implementing runtime adapter refactors.
- Adding autonomous mutation acceptance.

## Current Runtime Integration
- Enforced boundary: `adl learn export`
- Runtime path: `swarm/src/cli/commands.rs` via `real_learn_export`
- Validation/writing module: `swarm/src/tool_result.rs`
- Current bounded behavior:
  - export succeeds as before
  - command emits a validated `tool_result.v1` sidecar
  - malformed ToolResult payloads fail with deterministic contract errors before the sidecar is written
