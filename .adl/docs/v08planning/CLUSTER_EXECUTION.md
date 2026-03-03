# v0.8 Cluster Execution Planning Spec

Status: Planning
Owner issue: #339

## v0.8 Commitment Summary
- Define a deterministic-aware multi-node execution model that extends the current single-remote-executor MVP.
- Keep scheduler authority local in ADL runtime; cluster services execute assigned units, not global orchestration.
- Specify protocol extensions for placement/claims/coordination with explicit backward compatibility.
- Specify replay-critical artifacts and trace fields so multi-node runs remain auditable and reproducible.
- Define security/trust boundaries for node identity, signing, authorization, and redaction.
- Deliver a scoped v0.8 slice (capability-based placement + deterministic scheduling artifact) and defer broader resilience/distributed operations.

## Inputs (Current Reality)
- Remote execution MVP boundary: local scheduler, remote single-step execution via `/v1/execute`, `/v1/health`, protocol `0.1`.
- Current trust boundary: deterministic request signing/canonicalization, verification profile checks, and deterministic remote error codes.
- Current artifact model: `.adl/runs/<run_id>/` with stable machine-readable files (`run_summary.json`, `run_status.json`, `steps.json`, `trace.json`, learning artifacts).
- Existing demos: signed remote execution (`v0-7-enterprise-signed-remote`) and remote provider MVP examples.

## A) Cluster Model + Terminology

### Terms
- `coordinator`: service that tracks available nodes and lease/claim state for placement requests.
- `scheduler`: local ADL runtime component that computes execution ordering and chooses placements.
- `node`: remotely reachable execution endpoint with declared capabilities and trust identity.
- `worker`: execution process on a node that runs a claimed step payload.
- `capability`: deterministic key/value feature set used for placement filtering (e.g., provider kind, tool allowlist, region tag).
- `placement`: deterministic mapping from step_id to node_id plus claim metadata.

### Roles and Responsibilities
- Local ADL scheduler remains source of truth for workflow ordering and dependency correctness.
- Coordinator and nodes provide capacity/capability surfaces; they do not reorder DAG execution.
- Nodes execute only fully resolved, scheduler-authorized step payloads.

### Targeting Modes
- Single remote executor (current MVP): one endpoint, no cluster placement.
- Cluster mode (planned): scheduler asks coordinator for placement candidates and records final deterministic assignment per step.

## B) Scheduling + Determinism Strategy

### Deterministic Placement Rules
- Placement inputs must be explicit:
  - workflow/run placement policy
  - step capability requirements
  - coordinator node inventory snapshot (versioned)
- Tie-break order must be stable:
  1. capability match score (deterministic function)
  2. node priority (explicit integer)
  3. lexicographic `node_id`

### Nondeterminism Capture
- Runtime variability (availability/latency) is captured as a scheduling artifact, not implicit behavior.
- Each assigned step records:
  - `step_id`
  - `node_id`
  - `claim_id`
  - `inventory_snapshot_id`
  - placement reason/tie-break trace

### Replay Anchor
- Store a deterministic scheduling artifact (`cluster_schedule.json`) under run artifacts.
- Replay mode reuses schedule artifact; no fresh placement decisions when replaying deterministic mode.

## C) Remote Protocol Extension Notes

### Baseline
- Existing endpoint contract includes `protocol_version`, run/workflow/step IDs, resolved step payload, and structured errors.

### Planned Extension (Spec)
- Add coordinator-facing messages (names illustrative):
  - `POST /v1/cluster/claims` -> request placement claim for `(run_id, workflow_id, step_id, capability_requirements)`.
  - `POST /v1/cluster/claims/{claim_id}/commit` -> confirm scheduler-selected placement.
  - `POST /v1/cluster/claims/{claim_id}/release` -> deterministic release/cancel path.
- Extend execute request metadata (without changing step semantics):
  - `node_id`
  - `claim_id`
  - `inventory_snapshot_id`
  - `placement_policy_version`

### Versioning + Compatibility
- Preserve backward compatibility with current single-step protocol path.
- Introduce explicit protocol minor version bump for cluster fields; reject unsupported combinations with deterministic schema errors.

### Required Error Semantics
- Keep stable machine codes and deterministic classification buckets:
  - schema/policy/timeouts/retryable transport/provider-execution
- Add cluster-specific deterministic codes (planned):
  - `REMOTE_CLUSTER_NO_CAPABLE_NODE`
  - `REMOTE_CLUSTER_CLAIM_CONFLICT`
  - `REMOTE_CLUSTER_INVENTORY_MISMATCH`

## D) Trace / Artifact Model Changes

### Replay-Critical Trace Additions
- For each remotely placed step, trace must include:
  - `node_id`
  - `claim_id`
  - `inventory_snapshot_id`
  - placement decision source and deterministic tie-break details
- Event order remains deterministic by existing scheduler ordering rules.

### Artifact Layout (Relative Paths)
- Extend run artifact tree with cluster subdir:
  - `.adl/runs/<run_id>/cluster/cluster_schedule.json`
  - `.adl/runs/<run_id>/cluster/inventory_snapshot.json`
  - `.adl/runs/<run_id>/cluster/claims.json`
- Keep all paths run-relative and portable; no host-absolute paths.

### Learning Export Interaction
- Learning exports should reference cluster placement using stable IDs/hashes only.
- Do not export raw credentials, raw host paths, or unredacted remote environment details.

## E) Security + Trust Boundaries

### Identity / Signing / Authorization
- Node and coordinator requests must be signed and verified under the same trust-profile principles as current remote signing work.
- Scheduler-authorized scope is least-privilege: node may execute only claimed step payload for a specific `(run_id, step_id, claim_id)`.

### Threat Notes
- Compromised node:
  - limit blast radius by scoped claim authorization and artifact-level audit trail.
- Replay attacks:
  - include nonce/claim identity and schedule hash binding in signed metadata.
- Log tampering:
  - rely on signed request metadata + deterministic artifact hashes + immutable trace chain strategy (future hardening).

### Redaction Boundaries
- Never persist secrets or raw credential material in trace/artifacts.
- Avoid raw tool args/input payload copies unless explicitly redacted/sanitized by policy.

## F) Phased Roadmap

### Phase 0 (Current MVP)
- Single remote executor with signed request validation and deterministic error taxonomy.

### Phase 1 (v0.8 Slice)
- Capability-based placement through coordinator claims.
- Deterministic schedule artifact (`cluster_schedule.json`) and replay pinning.
- Cluster-aware trace metadata for placement and execution identity.

### Phase 2 (Later)
- Resilience/checkpoint integration for multi-node recovery.
- Advanced scheduling policies and distributed failover controls.
- Broader fleet policy management and operational hardening.

## Out of Scope for this Spec
- Kubernetes/operator implementation details.
- Full production PKI/IAM design.
- Runtime code changes in issue #339.

## References
- #339
- `docs/adr/0003-remote-exec-mvp.md`
- `docs/adr/0006-remote-signing-canonicalization.md`
- `docs/milestones/v0.7/DEMOS_v0.7.md`
- `swarm/src/remote_exec.rs`
- `swarm/src/signing.rs`
- `swarm/src/sandbox.rs`
- `swarm/src/learning_export.rs`
