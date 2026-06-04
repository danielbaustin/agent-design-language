# Long-Lived Agent Run-Loop Extraction Plan (#3625)

Issue: #3625
Umbrella: #3592
Captured: 2026-06-04
Status: ready_for_follow_on_execution

## Purpose

This packet plans a conservative extraction path for the long-lived agent
run-loop. The long-lived agent is an operational runtime surface, so the first
safe step is characterization and invariant capture, not immediate code
movement.

The desired end state is a smaller, more reviewable orchestration surface that
preserves tick, lease, status, stop, guardrail, artifact, and persistence
behavior exactly.

## Scope

Included:

- Source-shape review for `adl/src/long_lived_agent.rs`.
- Source-shape review for `adl/src/long_lived_agent/`.
- Persistence and public-artifact invariant inventory.
- Characterization proof plan for future behavior-preserving extraction.
- Follow-on routing for runtime action logs and observability.

Excluded:

- Code movement in this issue.
- Persistence layout changes.
- Runtime scheduling behavior changes.
- OpenTelemetry or runtime action-log implementation.
- Multi-agent behavior activation.

## Source Map

Deterministic line-count command:

```bash
wc -l adl/src/long_lived_agent.rs adl/src/long_lived_agent/*.rs
```

Observed source shape:

| Surface | LoC | Current role |
| --- | ---: | --- |
| `adl/src/long_lived_agent.rs` | 1,377 | Root orchestration for `tick`, `run`, `status`, `stop`, lease handling, cycle artifact writing, guardrail handling, continuity records, and operator events. |
| `adl/src/long_lived_agent/inspection.rs` | 191 | Reviewer-facing inspection packet creation and safe cycle-reference handling. |
| `adl/src/long_lived_agent/schema.rs` | 22 | Schema-version and default-path constants. |
| `adl/src/long_lived_agent/storage.rs` | 177 | Path resolution, JSON/JSONL helpers, atomic state writes, and operator event helpers. |
| `adl/src/long_lived_agent/tests.rs` | 707 | Integration-style behavior coverage for status, tick, run, inspection, guardrails, lease, stop, and sanitizer behavior. |
| `adl/src/long_lived_agent/types.rs` | 146 | Public serializable types for specs, status, leases, stop requests, options, and inspection cursors. |

## Current Behavior Anchors

The current root module owns these high-risk behaviors:

- `tick(spec_path, TickOptions)` loads the spec, checks stop state, acquires a
  lease, writes running status, writes cycle artifacts, removes the lease, and
  records success or failure status.
- `run(spec_path, RunOptions)` enforces non-zero `max_cycles`, loops through
  `tick`, honors stop requests, supports no-sleep bounded test execution, and
  records supervisor-stop state after the configured consecutive-failure
  threshold.
- `status(spec_path)` initializes state if needed, recovers status from the
  cycle ledger when status is missing, and reports active or stale leases.
- `stop(spec_path, reason)` rejects empty reasons and records a stop request,
  operator event, and stopped status.
- Lease acquisition blocks overlapping active ticks, recovers stale leases, and
  records stale-lease recovery as an operator event.
- Cycle artifact writing produces reviewable artifacts even for blocked
  execution paths where guardrails reject actions before a normal cycle can
  complete.

## Invariants To Preserve

### Persistence Layout

Future extraction must not change these paths without a separate behavior-change
issue:

- `state/status.json`
- `state/lease.json`
- `state/stop.json`
- `state/cycle_ledger.jsonl`
- `state/provider_binding_history.jsonl`
- `state/memory_index.json`
- `cycles/<cycle_id>/cycle_manifest.json`
- `cycles/<cycle_id>/observations.json`
- `cycles/<cycle_id>/decision_request.json`
- `cycles/<cycle_id>/decision_result.json`
- `cycles/<cycle_id>/run_ref.json`
- `cycles/<cycle_id>/memory_writes.jsonl`
- `cycles/<cycle_id>/guardrail_report.json`
- `cycles/<cycle_id>/cycle_summary.md`

### Runtime Semantics

Future extraction must preserve:

- active leases blocking overlapping ticks;
- stale leases requiring recovery before the next tick proceeds;
- operator events for locked-spec revision attempts, stale-lease recovery,
  stop requests, and supervisor stops;
- status initialization without running a cycle;
- status recovery from the cycle ledger when `status.json` is missing;
- no-sleep bounded run behavior for deterministic tests;
- consecutive-failure threshold behavior;
- stop request behavior across `run`, `tick`, and `status`;
- reviewable artifact creation for blocked or guardrail-rejected cycles;
- sanitizer rejection of host-path leakage in public artifacts;
- explicit rejection of unsafe cycle references during inspection.

## Characterization Test Plan

Before moving code, future implementation should prove the behavior surface with
the existing long-lived-agent tests and a stable test listing.

Minimum characterization commands:

```bash
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests -- --list
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::status_initializes_required_continuity_files_without_running_cycle -- --nocapture
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::tick_creates_state_status_full_cycle_bundle_and_removes_lease -- --nocapture
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::run_max_cycles_no_sleep_writes_exactly_three_cycles_and_completed_status -- --nocapture
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::active_lease_blocks_overlapping_tick_and_status_reports_leased -- --nocapture
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::stale_lease_requires_recovery_then_allows_tick -- --nocapture
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::blocked_cycle_still_writes_reviewable_artifacts_before_returning_error -- --nocapture
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::forbidden_action_guardrails_block_cycle_with_specific_rejections -- --nocapture
cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::sanitizer_blocks_public_artifact_host_path_leakage -- --nocapture
```

If the future extraction changes only file boundaries, expected output and
artifact bytes should remain stable unless the implementation issue explicitly
records a reviewed exception.

## Extraction Slices

Recommended order:

| Slice | Candidate module | Moved responsibility | Required proof |
| --- | --- | --- | --- |
| 1 | `status_control.rs` | `status`, `stop`, status recovery, stop request handling | status, stop, ledger-recovery, and stale-lease status tests |
| 2 | `lease.rs` | active/stale lease acquisition and lease removal helpers | active lease and stale lease recovery tests |
| 3 | `cycle_artifacts.rs` | cycle artifact materialization, ledgers, continuity, provider binding, and memory writes | tick, run, blocked-cycle, and inspection tests |
| 4 | `guardrails.rs` | forbidden action checks, paper-only stock/league rejection, public artifact sanitizer | forbidden-action, stock-league, and sanitizer tests |
| 5 | `run_loop.rs` | `tick`, `run`, bounded no-sleep loop, failure threshold orchestration | full long-lived-agent focused test lane |

Do not split `types.rs`, `schema.rs`, `storage.rs`, or `inspection.rs` further
until the root run-loop ownership is smaller and the review surface has settled.

## Observability Routing

Runtime action-level logs remain owned by #3556. This issue should not implement
OpenTelemetry, long-running runtime spans, or new durable log sinks.

Future extraction should, however, keep the #3609 control-plane vocabulary in
mind so the long-lived-agent runtime can later emit compatible events:

- command or subsystem identity;
- bounded stage names;
- started/ok/blocked/failed result vocabulary;
- elapsed-time fields where useful;
- redaction before terminal or durable log output.

## Follow-On Routing

| Work | Route |
| --- | --- |
| Runtime action-level execution logs and OTEL-ready event emission | #3556 |
| Actual long-lived-agent code extraction | Future implementation issue after the characterization proof above is accepted |
| Runtime-v2 feature activation and ownership indexing | #3623 evidence plus v0.92 activation map |
| Broader CLI/module decomposition | #3592 mini-sprint closeout and later decomposition milestones |

## Recommendation

Do not move long-lived-agent code in #3625. Treat this issue as the
characterization and extraction contract. The first implementation issue should
start with the characterization commands above, then move one slice at a time
with no persistence or runtime behavior changes.

This preserves the runtime centerpiece while still giving the codebase a clear
path out of the current 1,377-line orchestration root.
