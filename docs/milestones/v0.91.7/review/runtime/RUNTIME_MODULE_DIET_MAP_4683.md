# Runtime Module Diet Map (#4683)

Status: `current_pre_soak2_map`

Issue: `#4683`

Umbrella: `#4634`

Generated at: `2026-07-04T09:49:15Z`

Machine-readable register:
`docs/milestones/v0.91.7/review/runtime/runtime_module_diet_map_4683.json`

## Claim Boundary

This packet records module-diet candidates from current WP-07 implementation
evidence. It does not claim broad runtime refactoring, runtime Soak 2, or
v0.92 activation readiness is complete.

The map is intentionally pre-Soak-2. `#4843` and `#4784` are ready PRs, while
`#4681` and `#4783` are ready but still waiting for current check completion.
`#4682` remains the final Soak 2 execution owner after those upstreams are
consumable.

## Evidence Inputs

| Issue | Current state | Evidence used | Diet signal |
| --- | --- | --- | --- |
| `#4681` | ready PR `#4868`, checks still pending | `MINIMAL_INTEGRATED_RUNTIME_PATH_4681.md`, diff touching `runtime_v2`, runtime-v2 CLI, usage, and retained evidence writer fix | Minimal runtime path assembly required a Runtime v2 contract, CLI command, retained evidence packet, validation commands, non-claims, and a fix to keep retained-evidence refs backed by written files. |
| `#4842` | merged/closed through PR `#4851` | runtime-v2 command wiring and current-runtime reconciliation proof | Runtime v2 prototype reconciliation is coupled to both runtime CLI and current-runtime substrate evidence. |
| `#4718` | merged/closed through PR `#4862` | `INTEGRATED_LOGGING_OTEL_PROOF_4718.md`, proof scripts, and retained generated samples | Logging/OTel is currently an export-compatible proof/mapping boundary, not a production collector/exporter. |
| `#4783` | ready PR `#4869`, checks still pending | diff touching `execute`, `instrumentation`, `trace`, `obsmem_indexing`, `resilience`; local PR-fast coverage rerun passed after remote coverage failure | Scheduler watcher and AEE resilience middleware cross execution, instrumentation, trace, ObsMem, and resilience surfaces in one change. |
| `#4784` | ready-green PR `#4871` | failure-injection proof bundle, control-plane evidence, and blocker register | Existing resilience primitives have executable failure-injection proof, but final Soak 2 consumption still depends on upstream integration state. |
| `#4843` | ready-green PR `#4870` | Soak 2 feature-list matrix, JSON fixture, validator, and execution packet update | Soak 2 matrix work gives `#4682` a canonical feature-list input, but it is not final Soak execution evidence by itself. |
| `#4682` | blocked packet committed; rerun pending upstream consumability | `soak2_4682` blocker/status packet | Final Soak 2 run evidence is not yet available to this map. |

## Size Signals

- `adl/src/runtime_v2` currently contains 163 Rust files. That is not a bug by
  itself, but it means integrated runtime paths need a stable facade and
  evidence registry instead of direct ad-hoc consumption of individual proof
  modules.
- `adl/src/resilience.rs` is 5,225 lines. `#4783` and `#4784` both expose the
  review cost of keeping policy, decision models, middleware semantics, and
  proof helpers concentrated there.
- `adl/src/execute/tests.rs` is 1,878 lines. Watcher/AEE middleware behavior
  needs narrower fixture/assertion modules before broad resilience refactors
  are safe.

## Diet Candidates

### D-4683-01: Runtime V2 Integrated Proof Facade

Priority: `P1`

Owner surface: `runtime_v2` integrated proof facade.

Current surfaces:

- `adl/src/runtime_v2/minimal_integrated_runtime_path.rs`
- `adl/src/runtime_v2/integrated_csm_run.rs`
- `adl/src/cli/runtime_v2_cmd/commands.rs`
- `adl/src/cli/runtime_v2_cmd/tests.rs`

Observed coupling:
Runtime path assembly currently requires CLI command code and contract code to
agree on proof packets, retained evidence files, validation commands, and
non-claims. The `#4681` review fix moved retained-evidence writing into the
contract writer because a CLI-only completion step could otherwise leave direct
callers with dangling retained-evidence refs.

Recommended follow-on:
Add a small Runtime v2 integrated-proof registry/facade that exposes
issue-independent evidence-bundle descriptors consumed by CLI commands and Soak
2 matrix fixtures.

Non-goal:
Do not collapse Runtime v2 proof modules or change proof semantics while the
registry is introduced.

Residual risk:
Without this facade, `#4682` and later runtime evidence consumers may keep
binding directly to individual proof artifacts and repeat non-claim/path-hygiene
logic.

### D-4683-02: Resilience Policy And Execution Boundary

Priority: `P1`

Owner surface: resilience policy and execution boundary.

Current surfaces:

- `adl/src/resilience.rs`
- `adl/src/execute/runner.rs`
- `adl/src/execute/mod.rs`
- `adl/src/execute/tests.rs`

Observed coupling:
`#4783` resilience integration crosses runner execution, resilience decision
data, cancellation/backpressure/bulkhead behavior, trace projection,
instrumentation formatting, and broad execution tests.

Recommended follow-on:
Split resilience into decision model, middleware adapter, and proof-fixture
modules, then move execution tests for watcher/AEE middleware into focused
fixture files.

Non-goal:
Do not do the split inside WP-07 until `#4783`, `#4784`, and `#4682` evidence
has landed and behavior is pinned.

Residual risk:
A premature split could hide behavior drift in scheduler/watcher/AEE paths that
are still under PR validation and Soak 2 consumption.

### D-4683-03: Review Packet Redaction And Lifecycle Gates

Priority: `P1`

Owner surface: review packet redaction and lifecycle gates.

Current surfaces:

- `adl/src/cli/tooling_cmd/code_review_build.rs`
- `adl/src/cli/tooling_cmd/tests/code_review.rs`
- `adl/src/cli/pr_cmd/finish_support.rs`
- `adl/src/cli/pr_cmd/github/transport.rs`

Observed coupling:
WP-07 runtime issues exposed that runtime proof can be blocked by lifecycle and
review-control-plane behavior. Draft-state ready promotion and review packet
hygiene affected publication even when local runtime proof passed.

Recommended follow-on:
Keep review packet redaction and ready-promotion behavior as dedicated
control-plane surfaces with regression tests for deleted lines, deleted files,
added/current secret blocking, and draft-gated check transitions.

Non-goal:
Do not mark a runtime/product feature complete merely because control-plane
publication was repaired; each runtime row still needs integrated evidence.

Residual risk:
If lifecycle gates drift again, WP-07 runtime PRs can become blocked by
publication mechanics instead of runtime defects.

### D-4683-04: Observability Proof Boundary

Priority: `P2`

Owner surface: observability proof boundary.

Current surfaces:

- `adl/tools/test_pr_v0917_integrated_observability_proof.sh`
- `docs/milestones/v0.91.7/review/observability_4718/INTEGRATED_LOGGING_OTEL_PROOF_4718.md`

Observed coupling:
The current logging/OTel surface is proof/mapping evidence and
stdout/stderr/redaction validation, not a production OTel collector/exporter.
It must be consumed by `#4682` as retained integrated evidence without widening
the claim.

Recommended follow-on:
Create a narrow observability-consumer contract that Soak 2 and future runtime
consumers can reference without overstating OTLP/exporter readiness.

Non-goal:
Do not claim production OpenTelemetry collector, OTLP exporter, hosted
telemetry service, or Unity editor execution from `#4718` alone.

Residual risk:
Downstream readiness docs could accidentally treat OTel-compatible mapping
proof as deployed telemetry.

### D-4683-05: Soak 2 Matrix And Blocker Register

Priority: `P2`

Owner surface: Soak 2 matrix and blocker register.

Current surfaces:

- `docs/milestones/v0.91.7/review/runtime/SOAK2_FEATURE_LIST_MATRIX_4843.md`
- `docs/milestones/v0.91.7/review/runtime/soak2_feature_list_matrix_4843.json`
- `docs/milestones/v0.91.7/review/runtime/v0917_integrated_resilience_failure_injection_4784/blocker_register.json`
- `docs/milestones/v0.91.7/review/runtime/soak2_4682/soak2_execution_status_4682.json`

Observed coupling:
Failure-injection proof and the feature-list matrix can classify dependency
rows, but final Soak 2 truth still belongs to `#4682` after upstream PRs are
consumable.

Recommended follow-on:
Make `#4843`'s matrix the canonical source for future diet-map deltas and
require `#4682` to emit blocker rows that can be merged into this register.

Non-goal:
Do not use this pre-Soak map as final Soak 2 architecture disposition.

Residual risk:
Module pain discovered only during `#4682` Soak 2 is not represented here.

## Blocker Register

| Blocker | Scope | Missing input | Current disposition | Owner |
| --- | --- | --- | --- | --- |
| `B-4683-01` | final Soak 2 diet disposition | `#4682` Soak 2 run evidence after upstream PRs are consumable | recorded as follow-on input | `#4682` |
| `B-4683-02` | runtime resilience publication truth | `#4783` PR checks and merge/consumption state | blocks any final v0.92 claim requiring scheduler/watcher/AEE integrated middleware | `#4783` / `#4682` |
| `B-4683-03` | canonical runtime-path consumption | `#4681` PR checks and merge/consumption state | blocks final Soak 2 run truth until consumable | `#4681` / `#4682` |

## Validation Plan

Run:

```bash
jq . docs/milestones/v0.91.7/review/runtime/runtime_module_diet_map_4683.json
rg -n "D-4683-|B-4683-|#4681|#4718|#4783|#4784|#4843|#4682" docs/milestones/v0.91.7/review/runtime/RUNTIME_MODULE_DIET_MAP_4683.md
git diff --check
```

## Non-Claims

- This artifact does not claim Soak 2 has run.
- This artifact does not claim production OTel is present.
- This artifact does not claim `#4783` scheduler/watcher/AEE middleware is
  merged or consumed by Soak 2.
- This artifact does not close `#4634`; the umbrella remains open until child
  issues are complete or explicitly blocked with evidence and operator
  approval.
