# Cognitive Scheduler v1 for #4107

## Scope

This packet records the bounded Scheduler v1 implementation for `#4107`.

The implementation is a deterministic library surface in `adl/src/scheduler.rs`.
It consumes the `#4106` economics input bundle shape and returns an auditable
plan with one decision artifact per task.

## Implemented Surface

`adl/src/scheduler.rs` now defines:

- `CognitiveSchedulerLaneV1`
- `CognitiveSchedulerDecisionV1`
- `CognitiveSchedulerPlanV1`
- `schedule_economics_input`
- `schedule_economics_bundle`

The scheduler emits:

- selected lane
- alternatives considered
- rejection or fallback reasons
- score breakdown
- dependency status
- manual override status
- confidence
- deterministic scheduling rank key
- recommended order

## Lanes

The v1 lane set is:

| Lane | Purpose |
| --- | --- |
| `LOCAL` | low-risk, low-cost, dependency-clear local work |
| `CHEAP_REMOTE` | review, test generation, or non-low validation/coordination work |
| `PREMIUM` | implementation, refactor, security, high-risk, or critical-value work |
| `GOVERNOR` | release, architecture, critical-risk, human-required, or override work |
| `DELAYED` | blocked work, low-urgency work under constrained premium capacity, or non-immediate governor work when governor attention is constrained |

## Fixture-Proven Behavior

The fixture at `adl/tests/fixtures/scheduler/economics_inputs_v1.json` proves:

| Task | Expected lane |
| --- | --- |
| `release-authority` | `GOVERNOR` |
| `premium-code-repair` | `PREMIUM` |
| `first-pass-review` | `CHEAP_REMOTE` |
| `partial-dependency-review` | `CHEAP_REMOTE` |
| `docs-status-check` | `LOCAL` |
| `low-urgency-cleanup` | `DELAYED` |
| `blocked-proof` | `DELAYED` |

The deterministic order starts with `release-authority` and leaves
`blocked-proof` last because a hard blocker should not outrank schedulable
work.

The scheduling rank key follows the documented tie-breaker order: blocked
status, dependency posture, authority/risk gate, urgency, expected value,
validation burden, premium capacity pressure, coordination cost, parallelism,
confidence, and stable task id. It does not sort by lane priority before those
policy factors.

The focused tests also cover constrained governor attention: a non-immediate
architecture/governor task is delayed when governor attention is constrained,
while immediate critical release authority still routes to `GOVERNOR`.

## Validation

Focused validation:

- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- `cargo test --manifest-path adl/Cargo.toml cognitive_scheduler -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml scheduler_economics -- --nocapture`

## Dependency Note

`#4107` is implemented on top of the `#4106` economics contract. During
authoring, this branch temporarily carried the `#4106` dependency commit while
PR `#4131` was pending.

PR `#4131` is now merged, and the `#4107` worktree has been rebased onto
`origin/main`; the remaining #4107 diff is the Scheduler v1 decision-function
work and this proof packet.

## Non-Claims

This slice does not claim:

- autonomous sprint conduction
- live provider or model selection
- GitHub issue, PR, branch, or worktree mutation
- timed execution or use of the `schedules` crate
- measured speedup or measured cost reduction
- full PVF execution
