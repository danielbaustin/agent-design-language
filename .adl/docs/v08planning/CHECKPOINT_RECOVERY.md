# Checkpoint / Recovery Engine (v0.8 Planning Spec)

Status: Draft
Owner: EPIC-F (#430)
Related: #340, #339, #453, #454, #455, #491

## Commitment Summary
- ADL resilience in EPIC-F means crash-safe, deterministic continuation of the *same* run, not heuristic replay of a new run.
- Replay and resume are different contracts: replay re-executes from artifacts/traces; resume reuses validated persisted state at deterministic boundaries.
- Deterministic recovery matters because ADL guarantees reproducible ordering, stable error taxonomy, and auditable artifacts.
- Checkpoint/recovery must compose with existing v0.7 surfaces: run artifacts, pause/resume state, remote envelope signing, and run status taxonomy.
- v0.8 scope is a minimal deterministic resume engine for single-node and bounded remote continuation; multi-node coordination is explicitly later-phase.

## Grounded Baseline (Current Runtime)
Current ADL behavior already provides building blocks:
- Run artifacts live under `.adl/runs/<run_id>/` with atomic-write helpers for JSON artifacts.
- v0.6/v0.7 pause/resume exists at deterministic step boundaries (`pause_state.json`, strict validation, `resume <run_id>` flow).
- Remote execution MVP and security envelope define signed request verification and deterministic error codes.
- Trace + learning export surfaces already encode execution facts that can be referenced by recovery metadata.

This spec extends that baseline; it does not define a parallel persistence model.

## Terminology and Model
- Replay: Re-execute from source plan/spec and compare with prior artifacts.
- Resume: Continue an interrupted run by reusing previously completed deterministic units.
- Checkpoint: A persisted recovery boundary with enough validated metadata to safely continue.
- Activation boundary: Deterministic point where state is safe to persist (initially step boundary only).
- Deterministic reuse: Reuse is permitted only when compatibility checks pass exactly.
- Checkpoint ID: Stable identifier derived from `(run_id, activation_index, execution_plan_hash)`.
- Resume token: User-facing reference to checkpoint/run context (initially `run_id`; checkpoint IDs internal).

### What Gets Checkpointed
Minimum checkpoint payload (v0.8 Phase 1):
- `schema_version`
- `run_id`
- `workflow_id`
- `doc_version`
- `execution_plan_hash` (or canonical fingerprint)
- `activation_index` (next step cursor)
- `completed_step_ids` (sorted)
- `artifact_manifest_hash` (hash over expected outputs present at checkpoint)
- `status` (must be `paused` or `recoverable_failed` when phase allows)

Boundaries (v0.8 required):
- Step boundary only.
- No mid-step checkpoint.
- No speculative subgraph checkpoint.

## Recovery Semantics
Resume is allowed only when all compatibility checks pass:
1. `schema_version` supported.
2. `run_id` matches requested run.
3. `workflow_id` and `doc_version` match resolved document.
4. `execution_plan_hash` exactly matches resolved plan hash.
5. checkpoint `status` is resumable (`paused` in Phase 1).
6. required artifacts for completed steps exist and validate.

### Reuse vs Re-run Rules
- Completed step may be reused only if expected artifacts exist and hashes match manifest.
- If artifact missing or invalid, step is re-run deterministically.
- Resume never changes scheduler tie-break rules; ready-set ordering remains canonical.
- Conflicts are resolved deterministically by first failing invariant check and returning a stable error code.

### Remote Interaction
- Remote steps follow the same resume contract; request envelope verification still happens before execution.
- Recovery metadata never bypasses remote trust policy.
- If remote compatibility cannot be proven, behavior is deterministic fail-closed (no best-effort partial resume).

## Required Artifacts
Persisted artifacts required for deterministic recovery:
- `run_status.json` (terminal/intermediate classification)
- `run_summary.json` (stable execution summary)
- `pause_state.json` or future `checkpoint_state.json`
- `steps.json` / activation segments (ordered step outcomes)
- output artifact manifest (hashes + relative paths only)
- trace segment references (optional in v0.8 Phase 1; required later)

### Atomicity and Validation
- All checkpoint/state writes must use same-directory temp file + atomic rename (`atomic_write` contract).
- Each persisted record includes schema version and compatibility metadata.
- On load, validate schema first, then identity/version/plan invariants, then manifest integrity.
- Corruption detection is deterministic: parse/validation failure maps to stable error taxonomy.

## Security and Privacy Boundaries
- No secrets persisted by default in checkpoint payloads.
- Tool/provider outputs in checkpoint metadata must be redacted or referenced by hash/path; no raw secret-bearing payloads.
- No absolute host paths in persisted recovery artifacts; use sandbox-relative or run-relative paths.
- Signing/trust expectations:
  - local resume relies on artifact integrity + hash invariants;
  - remote resume additionally respects request signing and trust policy gates.
- Recovery state must be treated as untrusted input until validated.

## Failure Modes and Deterministic Behavior
- Partial checkpoint write: treated as invalid/incomplete checkpoint; resume denied with stable code.
- Missing artifact for reused step: deterministic rerun of that step (if policy allows) or fail with stable code.
- Version mismatch: fail closed with explicit compatibility error.
- Plan mismatch: fail closed; do not attempt heuristic mapping.
- Corrupt JSON/state: deterministic parse/validation error; no undefined fallback.

## Phased Roadmap
### Phase 0 (Current)
- Existing pause/resume and run-state artifacts.
- No generalized checkpoint engine.

### Phase 1 (Required for v0.8)
- Single-node deterministic checkpoint state model.
- Explicit checkpoint artifact schema and manifest validation.
- Deterministic resume skip/rerun policy from validated manifest.
- Stable recovery error taxonomy documented and tested.

### Phase 2 (Deferred beyond v0.8 unless explicitly pulled in)
- Remote-exec-aware resume continuity with signed continuation envelopes.
- Recovery across remote transient failures with deterministic reconciliation.

### Phase 3 (Deferred; aligns with #339)
- Multi-node checkpoint coordination and placement-aware recovery.
- Cluster-level lease/claim integration and node identity provenance in resume logs.

## v0.8 Scope Boundary
In scope for v0.8:
- Deterministic single-node checkpoint/recovery model and contracts.
- Compatibility/validation taxonomy and artifact requirements.

Out of scope for v0.8:
- Full distributed checkpoint coordination.
- Database-backed consensus checkpoint stores.
- Mid-step continuation semantics.

## Implementation Follow-up Candidates
1. Define `checkpoint_state.json` v1 schema + serde model + validator (docs + tests).
2. Add artifact manifest hashing utility shared by resume/checkpoint validation.
3. Add deterministic recovery error-code enum and mapping tests (no substring matching).
4. Extend resume integration tests for partial-artifact recovery behavior.
5. Add remote continuation compatibility checks gated behind signed envelope policy.
