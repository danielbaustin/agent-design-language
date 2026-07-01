# V0.91.6 External-Review Proof Gap Verification (`#4620`)

Issue: `#4620`  
Date: `2026-06-28`  
Milestone: `v0.91.6`

## Summary

This packet records the minimum proof-gap verification required after the failed
v0.91.6 external review pass.

Two remediation issues landed before this packet:

- `#4611` closed on `2026-06-28` with the numbered-SRP-finding SOR-facts fix.
- `#4612` closed on `2026-06-28` with the blocked-live AWS heartbeat cursor fix.

This packet does not rerun the broad test suite, PVF lanes, or runtime waves.
It proves the two named regressions directly, then classifies the remaining
external-review gaps against the retained issue and evidence surfaces already in
the repo.

## Commands Executed

1. `cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::pr_cmd::tests::finish::arg_render::sor_emitted_facts_capture_numbered_review_findings_and_dispositions' -- --exact`
   Verified the `#4611` SOR-facts parser regression using the exact numbered
   findings test in `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs`.
2. `cargo test --manifest-path adl/Cargo.toml --lib 'runtime_aws_signal::tests::runtime_aws_signal_live_blocked_mode_preserves_existing_cursor_state' -- --exact`
   Verified the `#4612` blocked-live heartbeat cursor regression using the
   exact runtime test in `adl/src/runtime_aws_signal.rs`.

## Focused Verification Result

| Surface | Owner | Result | Evidence |
| --- | --- | --- | --- |
| Numbered SRP findings survive into machine-readable SOR facts | `#4611` | `executed_pass` | Exact `--bin adl` regression passed; issue `#4611` is closed. |
| Blocked live AWS heartbeat mode preserves cursor state | `#4612` | `executed_pass` | Exact `--lib` regression passed; issue `#4612` is closed. |

## External-Review Gap Status Table

Post-closeout supersession note for `#4661`: the table below records the
release-tail state at the time `#4620` ran. `#3980`, `#3981`, `#3984`, `#4620`,
and `#4621` are now closed, and v0.91.7 consumes their final state through
`docs/milestones/v0.91.7/review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md`.

| Gap surface | Classification | Owner / route | Evidence and note |
| --- | --- | --- | --- |
| Numbered SRP findings parser fix | `executed_pass` | `#4611` | Focused regression executed in this packet and passed; `#4611` is already closed. |
| Blocked-live heartbeat cursor semantics | `executed_pass` | `#4612` | Focused regression executed in this packet and passed; `#4612` is already closed. |
| Runtime / ops Soak #1 umbrella completion | `routed` | `#4543` | Retained packet [`docs/milestones/v0.91.6/review/runtime/V0916_RUNTIME_SOAK_1_STATUS_4543.md`](../runtime/V0916_RUNTIME_SOAK_1_STATUS_4543.md) records `partial_pass_with_blockers`, not milestone-wide runtime completion. |
| Scheduler advisory path and CLI artifact proof | `routed` | `#4544` | `#4543` retained packet records scheduler as `integrated_proven`; base scheduler packet remains [`docs/milestones/v0.91.6/review/scheduler/COGNITIVE_SCHEDULER_V1_4107.md`](../scheduler/COGNITIVE_SCHEDULER_V1_4107.md). No fresh rerun was required in this issue. |
| Live AWS profile / CloudWatch / SSM node health | `routed` | `#4545` | [`docs/milestones/v0.91.6/review/runtime/v0916_runtime_aws_ssm_health_4545/proof_summary.json`](../runtime/v0916_runtime_aws_ssm_health_4545/proof_summary.json) shows `checked_at_utc: 2026-06-26T16:08:00Z`, intended profile `agent-logic-admin`, healthy `wuji`, `nessus`, and `opticon`, plus explicit residual that the default AWS profile points at the wrong account. |
| ACIP + AEE temporary-agent + memory handoff runtime path | `routed` | `#4546` | [`docs/milestones/v0.91.6/review/runtime/v0916_acip_aee_memory_4546/review_summary.md`](../runtime/v0916_acip_aee_memory_4546/review_summary.md) proves one bounded integrated slice and explicitly does not claim full scheduler, Observatory/Unity, or v0.92 readiness. |
| Integrated failure injection / resilience proof | `routed` | `#4547` | [`docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/README.md`](../runtime/v0916_runtime_failure_injection_4547/README.md) proves the bounded long-lived-agent and resilience slice without widening to full runtime readiness. |
| Unity / Observatory runtime consumption | `routed` | `#4548` | Issue `#4548` is closed, but this packet does not rerun Unity proof. It remains a separately consumed retained surface and must not be smuggled into runtime-complete claims. |
| Soak #2 full feature-list integration gate | `routed` | `#4549` | Issue `#4549` is closed as the `v0.91.7` planning gate. It remains handoff/planning truth, not `v0.91.6` runtime proof. |
| External review disposition itself | `blocked` | `#3980` | At packet-writing time, issue `#3980` was open. This packet verified named proof gaps after the failed external review, but did not by itself close the external-review lane. `#3980` is now closed and consumed by `#4661`. |
| Findings remediation and final preflight | `in_progress` | `#3981` | At packet-writing time, issue `#3981` was open and was the canonical sink for these final preflight dispositions. `#3981` is now closed and consumed by `#4661`. |
| `v0.91.7` residual handoff | `routed` | `#3982` | At packet-writing time, issue `#3982` was open and was expected to consume residuals that were planning-only or post-closeout. `#3982` is now closed and consumed by `#4661`. |
| Release ceremony readiness | `blocked` | `#3984` | At packet-writing time, issue `#3984` had not yet closed. `#3984` is now closed; the failed-review non-approval boundary remains consumed by `#4661`. |

## Release-Tail Decision

Decision: `must_wait`

Reason:

- The two explicitly named remediation regressions now have direct executed
  proof and both passed.
- At packet-writing time, the broader external-review lane was not settled
  because `#3980`, `#3981`, and `#3984` had not yet closed. Those issues are
  now closed and consumed by `#4661`.
- Retained runtime, scheduler, AWS, ACIP/AEE/ObsMem, resilience, and Unity
  surfaces exist, but this packet does not convert them into a new blanket
  `integrated_proven` claim for the milestone.

## What This Packet Proves

- v0.91.6 is no longer relying on unexecuted claims for the `#4611` and
  `#4612` remediation slices.
- The remaining external-review gaps are named with concrete owner issues and
  retained evidence paths.
- Release ceremony should still wait for WP-15/WP-16/WP-19 truth rather than
  treating this packet as a release approval.

## Non-Claims

- This packet does not claim the failed external review is repaired by itself.
- This packet does not claim full runtime coherence, full PVF execution, or
  `v0.92` activation readiness.
- This packet does not claim Unity / Observatory, Soak #1, or Soak #2 were
  rerun here.
