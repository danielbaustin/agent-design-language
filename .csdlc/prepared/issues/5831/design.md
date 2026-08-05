# Issue 5831 Design: Runtime v3 Adaptive Learning DAG

## Outcome And Sources

Implement WP-13A's evaluated, policy-governed graph-change path in current Runtime v3 authority, `adl-runtime-kernel`, as described in `docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md`. Consume `adl-runtime-kernel/src/reasoning.rs`, `adl-runtime-kernel/src/cognition.rs`, `adl-runtime-kernel/src/governance.rs`, `adl-runtime-kernel/src/durable_state.rs`, and `.csdlc/evidence/5817/prerequisite-and-loop-runtime-requalification.md`. Retained `adl/src/runtime_v2/` loop and graph semantics are compatibility evidence only, not the implementation target.

## Owned Paths

- `adl-runtime-kernel/src/adaptive_learning.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/adaptive_learning.rs`
- `adl-runtime-kernel/tests/fixtures/adaptive_learning`
- `docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md`
- `.csdlc/prepared/issues/5831/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5831/produce-native-receipt.rb`
- `.csdlc/evidence/5831`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-birthday-kernel-registration-v1",
    "paths": [
      "adl-runtime-kernel/src/lib.rs"
    ],
    "issues": [
      5825,
      5826,
      5827,
      5828,
      5829,
      5830,
      5831,
      5833
    ],
    "order": [
      5825,
      5826,
      5827,
      5828,
      5829,
      5830,
      5831,
      5833
    ]
  }
]
```

## Contract

Evaluation bindings connect one loop iteration to feedback source, confidence, evidence refs, and policy context. An adaptation delta records before/after state hashes, rationale, rollback note, and canonical order. Graph changes begin as proposals; policy emits accepted or rejected disposition before mutation. Durable history links loop events, evaluation, state delta, proposal, policy decision, graph delta, and replay evidence.

## Dependencies And Invariants

WP-01/#5817, WP-13/#5830, merged #5104 semantics, and current Runtime v3 qualification must be verified. WP-01B/#5818 is a distinct documentation activation package and cannot satisfy the WP-01 gate. Same durable inputs replay identically. Missing evidence never becomes feedback; resume requires prefix continuity; rejected proposals cannot mutate state; bounds and cancellation remain Runtime authority.

## Validation And Rollback

The exact `adaptive_learning` Runtime v3 integration-test target must run a nonzero count proving accepted/rejected mutation, deterministic replay, forged/substituted history rejection, invalid graph bindings, discontinuous resume, recurrence bounds, missing evidence, unauthorized mutation, and rollback mismatch. The lane uses `adl-runtime-kernel/Cargo.toml`, not `adl/Cargo.toml`. The issue-local producer must run that target on native GitHub Actions macOS and Linux jobs at exact candidate HEAD and retain a hashed source manifest, complete nextest log, and canonical semantic-output artifact. The independent validator recomputes those files and producer digest, parses the positive test count, verifies workflow/run/job identity, and requires byte-identical semantic outputs; ancestral SHA equivalence is forbidden. Rollback replays the recorded inverse or restores prior graph/state hashes without deleting rejected proposal history.

## Non-Goals

Unconstrained self-modification, autonomous retraining, hidden model-memory mutation, policy bypass, production autonomous learning, consciousness/personhood claims, and v0.94 signed trace completion are excluded.
