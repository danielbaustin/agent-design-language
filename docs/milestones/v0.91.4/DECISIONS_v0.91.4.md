# v0.91.4 Decisions

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-22`
- Owner: ADL maintainers

## Purpose

Capture milestone-critical decisions for completing the C-SDLC rollout.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
| --- | --- | --- | --- | --- | --- | --- |
| D-01 | `v0.91.4` completes C-SDLC default operation after the `v0.91.3` first slice. | accepted | The process must become usable by future ADL software-development issues. | Leave C-SDLC optional. | Validators, conductor, sprint state, trace, and memory must be hardened. | [README](README.md) |
| D-02 | Durable C-SDLC workflow records must be tracked in Git. | accepted | Public auditability is required; local-only `.adl` state is insufficient. | Store durable truth only in local execution cache. | `workflow/c-sdlc/v0.91.4/` becomes the default-operation namespace. | [README](README.md) |
| D-03 | Signed trace proof is in scope. | accepted | Trace proof should not be postponed again. | Defer trace signing to a later milestone. | Evidence convergence must include trace/digest/signature verification. | [WBS](WBS_v0.91.4.md) |
| D-04 | Sprint conductor cannot advance or close over stale child truth. | accepted | Past process drift showed sprint state can overclaim cleanliness. | Rely on human memory. | Sprint default lane must enforce closeout truth. | [SPRINT](SPRINT_v0.91.4.md) |
| D-05 | Active issue migration policy is required. | accepted | Existing open work may not match the final C-SDLC lane. | Rewrite every historical issue or ignore drift. | Open issues must classify into migrate, defer, leave, fold, or block. | [WBS](WBS_v0.91.4.md) |
| D-06 | Five-minute sprint repeatability must be measured, not merely asserted. | accepted | One fast run is not a dependable process. | Treat a single demo as enough. | Repeatability metrics are part of the milestone exit bar. | [DEMO_MATRIX](DEMO_MATRIX_v0.91.4.md) |

## Open Questions

- Which exact downstream milestone receives CodeFriend alpha execution? Owner:
  ADL maintainers. Tracking: CodeFriend planning.
- Which post-`v0.91.4` milestone receives broader Software Development Polis
  feature expansion? Owner: ADL maintainers. Tracking: next milestone planning.

## Exit Criteria

- Milestone-critical decisions are logged.
- Signed trace and tracked workflow state remain in scope.
- Deferred product/social-cognition questions are routed outside the C-SDLC
  completion milestone.
