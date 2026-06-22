# Runtime Fire-Up Plan v0.91.6

## Status

Current operator-facing plan for the first ADL runtime fire-up rehearsal.

This plan reconciles the older Runtime v2 prototype direction with the current
v0.91.6 Tokio runtime substrate work. It is a rehearsal plan, not a claim that
Runtime v2 is complete, production-ready, or fully integrated with Polis,
Observatory, memory, governance, or provider execution.

It is also not the full Soak `#1` closeout contract for `#4185`. This document
governs the bounded fire-up sub-phase that should happen before the later
standard or extended soak acceptance target.

## Source Inputs

- `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md`
- `.adl/docs/TBD/runtime_v2/RUNTIME_V2_MINIMAL_PROTOTYPE.md`
- `docs/milestones/v0.91.6/RUNTIME_OBSERVABILITY_COMPLETION_SCHEDULE_v0.91.6.md`
- `docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md`
- current runtime/Tokio issue wave: `#4177` through `#4183`
- integrated runtime soak owner: `#4185`
- C-SDLC integration control-plane sprint: `#4388` and children `#4389`
  through `#4396`, especially VPP/PVF, SEP, goal metrics, GitHub convergence,
  template bootstrap repair, and tooling reliability routing

## Goal

Fire up the current runtime substrate in a bounded way and produce evidence that
ADL can start, supervise, observe, and stop a minimal runtime loop without
confusing rehearsal proof with full Runtime v2 completion.

The point is not to demonstrate the whole cognitive society. The point is to
prove that the current runtime substrate is executable enough to become the
basis for later Runtime v2, Observatory, provider, ACIP, and CAV work.

## First Fire-Up Definition

The first fire-up succeeds if it can demonstrate all of the following:

1. A runtime entry point starts through the current shared Tokio runtime
   substrate.
2. The process has a bounded lifetime, cancellation, timeout, or shutdown path.
3. The run emits inspectable observability events using the current logging
   channel contract.
4. The run writes no secrets and does not expose local credential material.
5. The run records enough metadata for later Observatory consumption without
   claiming live Unity integration.
6. The run can be repeated by an operator from documented commands.
7. Any skipped or blocked runtime capability is recorded as skipped or blocked,
   not silently treated as pass.

## Non-Goals

- No production Runtime v2 claim.
- No claim that a full Polis, citizen lifecycle, economic model, memory palace,
  ACIP network, provider execution layer, or CAV runtime is complete.
- No live Unity Observatory dependency for the first fire-up.
- No requirement to run hosted model/provider calls during first fire-up.
- No broad release validation unless the touched issue explicitly requires it.

## Rehearsal Sequence

1. Confirm the runtime/Tokio issue wave status.
   - Required check: `#4177` through `#4183` should be merged or explicitly
     classified as safe to rehearse against.
   - If any required runtime substrate issue is still open, record the fire-up
     as blocked or rehearsal-only.

2. Confirm the C-SDLC control-plane dependency route.
   - Required check: the `#4388` child surfaces used by the fire-up issue are
     merged, truthfully consumable, or explicitly classified as temporary
     manual fallbacks.
   - At minimum, confirm the fire-up issue can rely on: VPP presence, PVF lane
     assignment truth, issue goal/time/token accounting, native GitHub
     publication, and prompt-card bootstrap without hand-authored repair.
   - If those control-plane surfaces are still unstable, classify the fire-up
     as `rehearsal-only` or `blocked`; do not let a successful runtime process
     run silently stand in for missing workflow truth.

3. Select the smallest runtime entry point.
   - Prefer a current tracked binary/helper that already uses the shared Tokio
     runtime substrate.
   - Do not invent a new runtime entry point unless the fire-up issue explicitly
     owns that implementation.

4. Run local preflight checks.
   - Verify no required secrets are read for the first fire-up.
   - Verify the process-status helper can inspect the bounded runtime process if
     a PID or port is involved.
   - Verify log output preserves stdout/stderr and redaction expectations.

5. Start the runtime rehearsal.
   - Capture command, start time, end time, exit status, and log path.
   - Prefer a short bounded run that proves lifecycle and observability over a
     long daemon run.

6. Inspect the run evidence.
   - Confirm startup event.
   - Confirm heartbeat or progress event if supported.
   - Confirm shutdown/cancellation/timeout event.
   - Confirm no credential material or absolute host path leakage beyond
     explicitly approved operator-local paths.

7. Record the result.
   - `passed_rehearsal`: all first fire-up criteria met.
   - `blocked`: required substrate or environment missing.
   - `failed`: runtime starts but violates a criterion.
   - `skipped`: operator intentionally deferred the run.

## Evidence Packet Shape

The fire-up issue should produce a review packet with:

- exact command run;
- runtime entry point used;
- start/end timestamps;
- elapsed time;
- process status / PID / port evidence when applicable;
- redacted log excerpt or log artifact reference;
- observability event summary;
- control-plane dependency classification for the `#4388` surfaces the fire-up
  relied on;
- skipped/blocked capabilities;
- follow-on issues for missing runtime capabilities;
- explicit non-claims.

## Relationship To Observatory

The first fire-up should produce evidence that the Observatory can consume later,
but it must not claim the Unity Observatory is live. Observatory readiness
remains owned by WP-09 and issues `#3974`, `#4030`, `#4031`, `#4032`, `#4033`,
`#4034`, and `#4035`.

For `#4185`, that means:

- the bounded fire-up sub-phase may stop at redacted metadata, status events,
  and retained runtime evidence that a later Observatory-backed soak can
  consume;
- the later Soak `#1` acceptance slice may still require live
  Observatory/Unity consumption if the integrated soak plan keeps that bar.

## Relationship To Runtime V2 Prototype

The older Runtime v2 prototype document remains useful as architectural intent:
persistent manifold, identity-bearing citizens, kernel invariants, snapshot,
resource pressure, policy rejection, and trace visibility.

For this milestone, those are not first fire-up requirements. They are follow-on
Runtime v2 proof targets. The first fire-up should validate the executable
substrate that later makes those requirements realistic.

## Runtime Fire-Up Owner

Use existing issue `#4185` as the owner for the bounded first fire-up /
integrated runtime soak path. Do not create a duplicate runtime fire-up issue
unless `#4185` is explicitly split or superseded.

Within that owner issue, treat this document as the bounded fire-up entry slice,
not as the full Soak `#1` closeout contract. `#4185` is only closeout-complete
when the broader soak acceptance bar from
`RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md` is also satisfied or truthfully
deferred/blocked.

Acceptance criteria for the `#4185` fire-up slice:

- uses this plan as source input;
- runs from a bound issue worktree;
- records exact command and elapsed time;
- records startup/shutdown observability;
- proves no secret exposure;
- classifies result as passed, blocked, failed, or skipped;
- opens or routes follow-on issues for any missing runtime capabilities under `#4185` closeout truth.
