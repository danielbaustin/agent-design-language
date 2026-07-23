# Structured Task Prompt

Template: 1.0.0

Issue: 5589

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Complete and validate the full six-card preparation contract, design, diagram, adapter/authority matrix, and disjoint preparation claim now; do not edit product code, run implementation acceptance, amend into Runtime product paths, or claim implementation readiness before #5591 has a clean reviewed Parity-A contract and typed path disjointness passes.

## Deliverables

- production or maintained COTS-backed adapters for every owned degraded operational component
- signed gate-before-actuation and live negative governance evidence
- attenuating delegation plus resource, cancellation, retry, idempotency, and cleanup evidence
- live multi-agent provider, bounded scheduler, and governed-tool execution evidence
- citizen identity, memory, private-state isolation, redaction, and revocation evidence
- qualified-time, checkpoint, lifelog, graceful shutdown, restart, and no-duplicate continuity evidence
- adapter/authority matrix with explicit zero-credit classifications
- placeholder and duplicate deletion plus exact-revision Runtime v3 source/test budget proof

## Acceptance

1. AC-1: A live initialized adl-runtime-kernel process admits representative work through the reviewed #5591 ingress and invokes only production or maintained COTS-backed Parity-C adapters; DegradedOperationExecutor, fixture, mock, metadata-only, library-only, and fixed-bootstrap evidence receives zero parity credit
2. AC-2: Signed Freedom Gate and AEE decisions bind identity, delegation, policy, operation digest, and qualified time before provider or tool actuation, with live denial, appeal disposition, revocation, quarantine, replay, expiry, and no-post-denial invocation proof
3. AC-3: Delegation only attenuates authority and live work proves bounded resource reservation, cancellation precedence, retry/idempotency limits, saturation cleanup, delegation widening rejection, and no leaked work or capacity
4. AC-4: At least two admitted agents including Shepherd execute scheduled governed work through one configured production provider and one real governed tool, with deterministic dispatch plus timeout, auth, quota, cancellation, malformed-output, and unavailable-service classifications
5. AC-5: Citizen identity and memory/private state remain partitioned by authoritative identity and capability scope across restart, while revoked identity, cross-identity reads/writes, display/provider identity substitution, and retained-evidence disclosure fail closed
6. AC-6: Chronosense supplies qualified monotonic time and stale, regressing, or unqualified time cannot authorize actuation, delegation, checkpoint transition, scheduler dispatch, or recovery
7. AC-7: Authenticated checkpoints are the sole execution-recovery authority and linked redacted lifelog entries remain non-authoritative; live restart proves no duplicate side effects, corruption/replay rejection, lifelog failure isolation, final shutdown checkpoint, and current revocation revalidation
8. AC-8: Exact-revision focused/full tests, strict format/lint, negative proof, COTS inventory, protected-path disjointness, placeholder/degraded inventory, deletion accounting, source LoC, and test count all pass without AWS, Runtime v2 edits, cutover, default switch, weakened behavior, or unresolved review findings

## Dependencies

- #5361 prepared Runtime v3 acceptance umbrella
- #5336 architecture, ownership, source-line, and test-budget authority
- #5591 current clean reviewed Parity-A ingress/service contract before any #5589 product implementation
- #5591 protected-path claim narrowed or released before a typed #5589 product-path amendment
- #5349 provider and governed-tool contract alignment
- #5592 and #5590 disjoint ownership boundaries

## Inputs

- .adl/local-artifacts/wp5594/live-v0918-issues.json
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/governance.rs
- adl-runtime-kernel/src/identity_memory.rs
- adl-runtime-kernel/src/private_state.rs
- adl-runtime-kernel/src/live_continuity.rs
- adl-runtime-kernel/src/time.rs
- adl-runtime/src/continuity_history.rs
- docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md
- .csdlc/prepared/issues/5589/adapter-authority-matrix.md

## Non Goals

- product implementation before a clean reviewed #5591 contract and collision-free typed claim
- Parity-A ingress/lifecycle, Parity-B cognition, or Parity-D access/Observatory/guardian ownership
- fixture, mock, degraded, metadata-only, or library-only parity acceptance
- Runtime v2 source modification, reuse as Runtime v3 authority, cutover, default switch, or deletion
- AWS, remote/GPU deployment, new provider product scope, or credential-bearing retained proof
- subjective consciousness, affect, suffering, happiness, or unbounded self-improvement claims
