# ADR 0005: HITL Pause/Resume (v0.6)

## Status
Accepted (v0.6)

## Context
v0.6 introduces a minimal, strict human-in-the-loop (HITL) pause/resume
surface. The goal is to allow deterministic interruption at safe boundaries
without introducing checkpoint-engine complexity or recovery heuristics.

This ADR documents the shipped v0.6 semantics, persisted artifacts, strict
resume validation, and explicit non-goals.

## Decision
HITL pause/resume in v0.6 is step-boundary-only and strict:
- pause occurs only at deterministic step boundaries
- resume is allowed only from persisted paused state artifacts
- resume validation is fail-fast and requires exact compatibility
- no best-effort recovery is attempted on mismatch

Execution semantics remain deterministic and equivalent to uninterrupted
execution when resumed from a valid paused state.

## Pause Boundary Semantics
- Pause boundaries are step-level only.
- Mid-step pause/resume is not supported.
- Pauses are represented in run artifacts as explicit paused state.
- Non-paused failure runs do not generate pause-state artifacts.

## Persisted State Schema
Artifacts are written under:
- `.adl/runs/<run_id>/run.json`
- `.adl/runs/<run_id>/steps.json`
- `.adl/runs/<run_id>/pause_state.json` (paused runs only)

`pause_state.json` is versioned and includes:
- `schema_version` (currently `pause_state.v1`)
- `run_id`
- `workflow_id`
- `version` (ADL document version)
- `status` (`paused`)
- `adl_path`
- `execution_plan_hash` (deterministic fingerprint)
- `pause` payload:
  - `completed_step_ids`
  - `saved_state`
  - `completed_outputs`

`run.json` also carries a versioned schema and execution fingerprint
(`run_state.v1`, `execution_plan_hash`) plus terminal status information.

## Resume Validation Semantics
Resume validation is strict and fails on first mismatch. Required checks include:
- schema version compatibility
- paused-status requirement (`status == "paused"`)
- run identifier consistency (`run_id`)
- workflow consistency (`workflow_id`)
- ADL version consistency (`version`)
- execution plan compatibility (`execution_plan_hash`)

Mismatches are hard failures. No fallback, coercion, or partial recovery is
performed.

## CLI / Config Surface
Canonical v0.6 resume surface:
- `swarm resume <run_id>`
  - loads `.adl/runs/<run_id>/pause_state.json`
  - enforces strict validation before resuming

Transitional/legacy compatibility surface (still present in v0.6):
- `swarm <adl.yaml> --run --resume <run.json>`

The canonical path for operators and docs is `swarm resume <run_id>`.

## Determinism And Safety Considerations
- Pause/resume must preserve artifact equivalence relative to uninterrupted run.
- Resume compatibility depends on deterministic execution-plan serialization and
  deterministic execution-plan fingerprinting.
- Streaming remains observational-only and does not alter pause/resume
  correctness semantics.
- Pause artifacts are written atomically and schema-versioned.

## Security And Privacy Considerations
- Pause-state artifacts do not persist provider credentials/secrets by design.
- Credentials remain external (environment/operator-managed).
- Resume strictness reduces risk of accidental execution under mutated plans.

## Error Handling Semantics
v0.6 uses explicit, actionable errors for:
- missing pause-state artifact
- non-paused status
- schema mismatch
- run/workflow/version mismatch
- execution-plan hash mismatch

These are terminal resume errors by design.

## Explicit Non-Goals (v0.6)
- No generalized checkpoint/recovery engine.
- No mid-step resume.
- No distributed/multi-node resume.
- No partial replay heuristics.

## Known Limitations (v0.6)
- Resume scope is intentionally narrow: paused step-boundary runs only.
- Compatibility is strict; benign structure drift still invalidates resume.
- Historical/legacy `--resume <run.json>` path remains for compatibility but is
  not the canonical long-term interface.

## Alternatives Considered
- Mid-step checkpointing
  - Rejected: increases complexity and weakens deterministic boundaries.
- Best-effort resume with relaxed matching
  - Rejected: risks hidden nondeterministic behavior and silent divergence.
- Distributed resume coordination
  - Rejected: out of scope for v0.6 stabilization.

## Consequences
- Operationally predictable pause/resume behavior.
- Stronger safety posture through fail-fast validation.
- Limited flexibility compared to full checkpoint systems.
- Clear base for incremental v0.7+ expansion.

## Future Work (v0.7+)
- Structured checkpoint model beyond step boundaries.
- Richer recovery workflows and operator tooling.
- Explicit migration strategy away from legacy resume paths.
- Extended resume diagnostics and policy controls.
