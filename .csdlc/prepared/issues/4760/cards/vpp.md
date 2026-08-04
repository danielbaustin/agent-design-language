# Prepared VPP Draft: #4760 Memory Palace

Status: ready_for_typed_application_after_execution_claim
Initial PVF lane: runtime
Planned PVF lane: runtime plus focused deterministic contract lanes
Lane registry: `docs/validation/pvf_lanes.json`

## Validation DAG

### Lane P0: Preparation Integrity

- Proof role: preparation diff and source-reference hygiene.
- Command: `git diff --check`.
- Budget: 30 seconds, 500 tokens, small/local/deterministic.
- Release gate: preparation only; does not prove implementation.

### Lane P1: Memory Palace Unit Contract

- Proof role: canonical ordering, bounded working set, provenance, temporal
  compatibility, stale-context and redaction/path rejection.
- Command: `cargo test --locked --manifest-path adl/Cargo.toml memory_palace --lib`.
- Budget: 180 seconds, 2,000 tokens, small/local/deterministic.
- Acceptance: AC-1, AC-2, AC-3, AC-6.

### Lane P2: Integrated Runtime Consumer

- Proof role: configured long-lived cycle writes the packet and places its
  relative reference in `decision_request.memory_refs`; unconfigured cycle
  remains unchanged.
- Command: `cargo test --locked --manifest-path adl/Cargo.toml --test memory_palace_tests`.
- Budget: 300 seconds, 3,000 tokens, medium/local/deterministic.
- Acceptance: AC-4, AC-5.

### Lane P3: Runtime Owner Regression

- Proof role: touched runtime owner surface remains coherent.
- Command: `bash adl/tools/run_owner_validation_lane.sh runtime`.
- Budget: 1,200 seconds, 5,000 tokens, medium/local.
- Acceptance: AC-4, AC-5, AC-7.

### Lane P4: Static Rust Quality

- Proof role: compile/lint quality for the touched library surface.
- Command: `cargo clippy --locked --manifest-path adl/Cargo.toml --lib -- -D warnings`.
- Budget: 600 seconds, 2,000 tokens, medium/local/deterministic.
- Acceptance: AC-7.

### Lane P5: Retained Runtime Replay Proof

- Proof role: materialize two packets from the same declared fixture, compare
  bytes/hashes, and retain one cycle's input, packet, decision request, stale
  report, command transcript, and exact HEAD under `.csdlc/evidence/4760/`.
- Command: `ADL_MEMORY_PALACE_EVIDENCE_DIR=.csdlc/evidence/4760/runtime-replay cargo test --locked --manifest-path adl/Cargo.toml --test memory_palace_tests identical_input_emits_identical_runtime_packet -- --exact --nocapture`.
- Contract: the named test must reject an evidence directory outside the
  worktree and must retain both normalized packet hashes plus the consumed
  `decision_request.json`.
- Budget: 300 seconds, 3,000 tokens, small/local/deterministic.
- Acceptance: AC-1, AC-4, AC-7, AC-8.

## Planned Budget

- Total local validation ceiling: 2,610 seconds (43.5 minutes) if sequential.
- Total validation token budget: 15,500 tokens.
- Parallel groups: P1 and P4 may overlap after compile artifacts are warm,
  reducing planned critical-path elapsed time to 2,430 seconds; P2 precedes P3
  and P5; P0 runs last on the final diff.

## No-Deferral Policy

P1 through P5 are required local lanes before #4760 can claim
implementation/proof. CI may repeat them but cannot substitute for any missing
local P1-P5 evidence.
Remote, AWS, Unity, provider-live, and paid lanes are not selected because the
bounded implementation adds no such surface. If execution adds one, VPP must be
replanned before the change.

## Failure Policy

Any failed or missing required lane leaves #4760 open and #5007 deferred.
Flaky reruns do not erase first-failure evidence. Nondeterminism, privacy/path
leakage, stale-context acceptance, or invalid runtime consumption triggers the
rollback criteria in SPP/design rather than a validation waiver.
