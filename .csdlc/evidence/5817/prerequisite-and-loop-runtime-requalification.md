# v0.92 Prerequisite And Loop-Runtime Requalification

Issue: `#5817`

Status: `wp01_prerequisite_evidence`

## Result

The v0.92 issue wave may proceed. The prerequisite sources are present and the
historical `#5104` Runtime v2 loop-runtime contract is requalified against the
current Runtime v3 implementation. This is a bounded compatibility conclusion,
not a claim that Adaptive Learning DAG graph mutation is complete.

## Prerequisite Dispositions

| Input | Current evidence | WP-01 disposition |
| --- | --- | --- |
| v0.91.8 release and handoff | `docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md` and `FEATURE_PROOF_COVERAGE_v0.91.8.md` retain release-tail and v0.92 handoff truth. | consumed |
| `#3377` birthday readiness | Repo-native GitHub read confirms the issue is closed; the v0.92 birthday package preserves its requirements and non-claims. | consumed as planning input; v0.92 child issues own implementation |
| `#5359` WP-22 planning | Repo-native GitHub read confirms the issue is closed; the retained v0.92 package and source-disposition ledger provide the reviewed planning authority. | consumed |
| AEE completion | v0.91.5 and v0.91.6 AEE feature/evidence packets remain the bounded source for AEE completion and bridge accounting. | consumed as historical implementation input |
| `#5104` loop runtime | Merge `48e0081bb` and the v0.91.7 WP-11 evidence packet prove bounded Runtime v2 loop objects. Current Runtime v3 source and tests independently prove the required current contract. | requalified |

## Runtime v3 Requalification

The current Runtime v3 authority is `adl-runtime-kernel`, not the retained
Runtime v2 command. The following current surfaces satisfy the reusable
loop-runtime contract:

| Required behavior | Runtime v3 evidence |
| --- | --- |
| Bounded execution | `LoopDefinition` rejects zero or excessive iteration and deadline bounds; `execute_loop` enforces both. |
| Reasoning graph and state binding | `execute_loop` validates the observation and graph/state identity before execution. |
| Deterministic replay | Every accepted iteration emits a chained `ReplayEvent` from canonical serialized iteration state. |
| Exact terminal outcomes | `LoopStatus` distinguishes `Converged`, `Exhausted`, and `Cancelled`. |
| Live cancellation | The loop consumes a `CancellationToken`; focused tests cancel running work. |
| Resume continuity and integrity | Focused tests resume from a checkpoint and reject forged, substituted, or discontinuous replay. |
| Runtime supervision | The Runtime v3 reasoning component factory runs the loop through the kernel component context and child cancellation token. |

Source authority:

- `adl-runtime-kernel/src/reasoning.rs`
- `adl-runtime-kernel/tests/reasoning.rs`
- `adl-runtime-kernel/tests/parity_b_live_kernel.rs`

Historical semantic input:

- merged PR `#5104`, merge commit `48e0081bb`
- `docs/milestones/v0.91.7/review/V0917_WP11_RUNTIME_V2_COGNITIVE_CONTROL_EVIDENCE_4638.md`

## Boundary

This requalification authorizes v0.92 to treat bounded loops as current,
validated Runtime v3 objects. It does not prove:

- adaptive learning;
- autonomous or ungoverned graph mutation;
- production provider invocation;
- a public loop-control API beyond the current Runtime v3 authority surfaces;
- the disabled cross-process Runtime v2/Runtime v3 parity test as release proof.

WP-13A must still deliver evaluation bindings, durable adaptation deltas,
policy-governed graph-change proposals, accepted/rejected mutation evidence,
and integrated replay proof before Adaptive Learning DAG is complete.

## Focused Validation

Current Runtime v3 proof was rerun from the WP-01 worktree:

```text
cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml \
  --target-dir /Volumes/FastWork/adl-wp-5817/target --test reasoning
13 passed; 0 failed; 0 ignored
```

The WP-01 package validator separately checks all 39 work packages, all 41
child, supporting, and sprint umbrella issue records, all 552 generated card artifacts, exact wave/card
alignment, dependency acyclicity, source dispositions, feature coverage, and
the milestone completion rules.
