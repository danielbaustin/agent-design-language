# v0.91.6 to v0.91.7 Handoff Addendum

## Metadata

- Source milestone: `v0.91.6`
- Target milestone: `v0.91.7`
- Prepared by issue: `#3982`
- Prepared during: `v0.91.6` release tail
- Date: `2026-06-28`
- Status: `dependency_gated_planning_refresh`

## Purpose

Record the current v0.91.6-to-v0.91.7 planning handoff after the WP-15
external review failed on stale handoff truth and WP-16 consumed the
remediation/final-preflight findings.

This document is not v0.91.6 closeout truth. It is a next-milestone planning
addendum that lets v0.91.7 start quickly after v0.91.6 closes without losing
the late release-tail decisions, runtime integration concerns, build-throughput
work, and C-SDLC operational repairs discovered during v0.91.6.

## Current Release-Tail Gate

v0.91.7 planning may be prepared now, but v0.91.7 execution must not begin
until the v0.91.6 release-tail gate is consumed from current issue truth.

The required handoff inputs are:

- WP-14A `#4582`: closed internal review and pre-v0.92 burn-down truth.
- `#4609`: closed release-tail documentation truth findings.
- `#4610`: closed pre-v0.92 activation and C-SDLC adoption residual routing.
- `#4611`: closed numbered-SRP SOR fact fix and PR-inventory route.
- `#4612`: closed runtime AWS heartbeat cursor semantics fix.
- WP-15 `#3980`: closed external / third-party review owner. The review ran
  and failed on stale `draft_pre_send` handoff truth; the failure is recorded
  as release-tail truth rather than rewritten as a clean pass.
- WP-16 `#3981`: closed review findings remediation and final preflight owner.
- WP-16 child `#4621`: closed failed external-review truth and release-tail
  docs repair.
- WP-16 child `#4620`: closed external-review proof-gap verification.
- Tooling child `#4622`: closed and delivered repo-native PR inventory for the
  gap from `#4611`; it is not a v0.91.6 closeout blocker. Future release-tail
  reviews should consume that proof instead of falling back to the failed owner
  binary path.

## Current Handoff Truth

- v0.91.7 is still a planning package, not an executing milestone.
- v0.91.7 remains the final pre-v0.92 bridge/readiness tranche.
- The WP-15 external review is complete and failed; v0.91.7 must consume that
  failure as release-tail truth, not rerun the review to erase it.
- WP-16 has closed as the remediation and final-preflight owner for the failed
  review.
- v0.92 activation must remain blocked until every named activation surface is
  integrated/proven, explicitly non-claimed with operator approval, or blocked
  with evidence and operator approval.
- Planning docs alone do not prove runtime, demo, C-SDLC, provider, scheduler,
  AWS, or validation readiness.
- Mocks, seams, docs, and component tests count as prerequisites, not as
  product/runtime feature completion.

## First v0.91.7 Execution Priorities

When v0.91.7 opens, start with the following order. WP-16 is closed; WP-01
consumes the closed WP-16 truth rather than waiting for a future WP-16 gate:

1. WP-01: promote planning and consume final v0.91.6 closeout truth, including
   the failed WP-15 review, closed WP-16 remediation/final-preflight truth,
   closed `#4620` / `#4621` dispositions, and closed v0.91.7 `#4622` proof.
2. WP-02 / WP-03: consume C-SDLC control-plane truth and implement or
   explicitly block any remaining tooling defects before relying on
   sprint-scale parallel execution.
3. WP-06: preserve Nessus as the immediate Phase 1 remote validation lane and
   prove the next build/validation acceleration route early, including EC2 Spot
   or an alternative disposable builder, because v0.91.6 exposed build and
   validation latency as a release-risk multiplier.
4. WP-07 / WP-08: run runtime integration and AWS/signal bridge work after the
   build/validation route is no longer a wait-state trap.

## Remote Build Route

v0.91.6 established that Nessus is the current Phase 1 operational remote
validation lane for focused heavyweight local-offload proof when the lane is
remote-safe.

It also established that EC2 works as an SSM-driven on-demand proof after Spot
quota blocks, but EC2 Spot and disposable cloud builders are not routine lanes
yet. They need orchestration, cache bootstrap, quota discovery, cost accounting,
and teardown guards before they can become release-critical validation paths.

v0.91.7 should treat EC2 Spot / disposable builder work as an early WP-06 proof:

- use a disposable instance or runner;
- install Rust and `sccache`;
- clone ADL from the canonical repository;
- run focused owner/build lanes;
- record wall-clock time, instance type, region, interruption behavior, and
  estimated cost;
- terminate the instance and record cleanup evidence;
- compare EC2 Spot against local WUJI, the operational Nessus lane,
  CodeBuild-style alternatives, and any other remote-builder candidate;
- keep the total experiment bounded and do not make the lane release-critical
  until it has repeatable proof.

This is v0.91.7 proof work, not a v0.91.6 release blocker.

## v0.92 Activation Surfaces To Preserve

The v0.92 bridge must still resolve, prove, non-claim, or explicitly block:

- AEE completion;
- Memory / ObsMem handoff and Memory Palace context topology;
- ACP / cognitive profiles;
- provider/model matrix and suitability evidence;
- Observatory / Unity readiness;
- ACIP / provider communications and protobuf/JSON/WebSocket choices;
- public prompt records;
- security / CAV / access-rule residuals;
- runtime integration and scheduler operation;
- C-SDLC operational reliability, including watchers, session ledger, VPP/PVF,
  SOR fact capture, and issue metrics.

Every surface must be integrated/proven, explicitly non-claimed with operator
approval, or blocked with evidence and operator approval before v0.92 activation
docs may consume it.

## WP-01 Consumption Checklist

WP-01 should update v0.91.7 issue-wave truth from:

- this addendum;
- `PLANNING_SOURCE_CAPTURE_v0.91.7.md`;
- `WP_ISSUE_WAVE_v0.91.7.yaml`;
- failed WP-15 and final WP-16 v0.91.6 outputs, including closed `#4620` /
  `#4621` and the v0.91.7-routed `#4622`;
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`;
- any final v0.91.6 review/remediation packets merged after this addendum.

WP-01 should not reconstruct scope from chat or local scratch files when a
tracked handoff or issue route exists.

## Explicit Non-Claims

This addendum does not claim:

- v0.91.6 is complete;
- WP-15 passed external review;
- `#4622` has been implemented;
- v0.91.7 is approved for execution;
- EC2 Spot builds have been proven;
- runtime Soak #2 has run;
- v0.92 activation is ready.
