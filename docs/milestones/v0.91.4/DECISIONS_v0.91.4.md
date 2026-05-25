# v0.91.4 Decisions

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-25`
- Owner: ADL maintainers

## Purpose

Capture milestone-critical decisions for completing the C-SDLC rollout.

## How To Use

Use this decision log to preserve the milestone's strategic constraints while
the issue wave is seeded and executed. New decisions should be added here only
when they affect v0.91.4 scope, sequencing, release truth, or sidecar
boundaries.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
| --- | --- | --- | --- | --- | --- | --- |
| D-01 | `v0.91.4` completes C-SDLC default operation after the `v0.91.3` first slice. | accepted | The process must become usable by future ADL software-development issues. | Leave C-SDLC optional. | Validators, conductor, sprint state, trace, and memory must be hardened. | [README](README.md) |
| D-02 | Durable C-SDLC workflow records must be tracked in Git. | accepted | Public auditability is required; local-only `.adl` state is insufficient. | Store durable truth only in local execution cache. | `docs/milestones/v0.91.4/review/evidence/csdlc/` becomes the default-operation namespace. | [README](README.md) |
| D-03 | Signed trace proof is in scope. | accepted | Trace proof should not be postponed again. | Defer trace signing to a later milestone. | Evidence convergence must include trace/digest/signature verification. | [WBS](WBS_v0.91.4.md) |
| D-04 | Sprint conductor cannot advance or close over stale child truth. | accepted | Past process drift showed sprint state can overclaim cleanliness. | Rely on human memory. | Sprint default lane must enforce closeout truth. | [SPRINT](SPRINT_v0.91.4.md) |
| D-05 | Active issue migration policy is required. | accepted | Existing open work may not match the final C-SDLC lane. | Rewrite every historical issue or ignore drift. | Open issues must classify into migrate, defer, leave, fold, or block. | [WBS](WBS_v0.91.4.md) |
| D-06 | Five-minute sprint repeatability must be measured, not merely asserted. | accepted | One fast run is not a dependable process. | Treat a single demo as enough. | Repeatability metrics are part of the milestone exit bar. | [DEMO_MATRIX](DEMO_MATRIX_v0.91.4.md) |
| D-07 | C-SDLC core completion remains separate from optional workspace or product work. | accepted | v0.91.4 must finish the software-development control plane without making GWS, CodeFriend, or other product surfaces required C-SDLC machinery. | Mix optional workspace/product execution into the C-SDLC default-operation contract. | Product and workspace work may be planned separately, but C-SDLC proof, trace, memory, and workflow state stand on tracked ADL repo evidence. | [DESIGN](DESIGN_v0.91.4.md), [NEXT_MILESTONE_HANDOFF](NEXT_MILESTONE_HANDOFF_v0.91.4.md) |
| D-08 | CodeFriend pre-alpha repo/S3 welcome-page setup is a v0.91.4 sidecar mini-sprint. | accepted | The setup plan was intentionally scheduled for v0.91.4 and should be visible before the milestone opens. | Defer all CodeFriend setup to a later alpha milestone or hide it outside the milestone issue wave. | Adds a bounded sidecar wave for repo bootstrap, welcome page, AWS S3/CloudFront/ACM/Route 53 setup, and publication handoff without changing the C-SDLC core or closeout tail. | [CODEFRIEND_PRE_ALPHA_REPO_AND_S3_WELCOME_MINI_SPRINT](../../planning/codefriend/CODEFRIEND_PRE_ALPHA_REPO_AND_S3_WELCOME_MINI_SPRINT.md) |
| D-09 | Parallel Validation Fabric has an explicit v0.91.4 feature/proof surface. | accepted | The validation-tail bottleneck is central to making five-minute sprints real; it cannot remain an implied subsection of repeatability work. | Leave PVF embedded only in WP-10 repeatability wording. | WP-10 owns the first PVF plan/proof, and WP-14 validates that pending/deferred/blocking proof is represented truthfully. | [PARALLEL_VALIDATION_FABRIC](features/PARALLEL_VALIDATION_FABRIC.md) |

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
