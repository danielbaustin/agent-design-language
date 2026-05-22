# v0.91.3 Decisions

## Metadata

- Milestone: `v0.91.3`
- Version: `v0.91.3`
- Date: `2026-05-22`
- Owner: ADL maintainers

## Purpose

Capture milestone-critical decisions for the first C-SDLC implementation slice.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
| --- | --- | --- | --- | --- | --- | --- |
| D-01 | `v0.91.3` proves one bounded Cognitive State Transition, not full default adoption. | accepted | A first slice gives real evidence without overclaiming maturity. | Jump directly to default operation. | `v0.91.4` owns repeatability and enforcement. | [README](README.md) |
| D-02 | Preserve `SIP -> STP -> SPP -> SRP -> SOR` as the canonical card lifecycle. | accepted | The lifecycle repair from `v0.91.2` must become operating truth. | Keep older card semantics or mixed names. | All WPs must preserve distinct card roles. | [README](README.md) |
| D-03 | Define `SPP` as the Structured Plan Prompt: the issue-local operative execution plan. | accepted | `SPP` is the tracked equivalent of `/plan` for one issue. | Treat `SPP` as process memo or sprint plan. | Runtime plan drift must update `SPP`. | [README](README.md) |
| D-04 | Keep GitHub issue, PR, branch, CI, and human-review truth authoritative. | accepted | C-SDLC adds structure; it does not bypass existing governance. | Replace GitHub controls with internal process state. | Merge-readiness must preserve external truth. | [DESIGN](DESIGN_v0.91.3.md) |
| D-05 | Use tracked workflow records as the public proof target. | accepted | C-SDLC must be inspectable from Git, not only local execution cache. | Leave durable evidence in `.adl` local state. | `workflow/c-sdlc/v0.91.3/` becomes the first-slice target namespace. | [README](README.md) |
| D-06 | Include the complete closeout tail in the issue wave. | accepted | Release confidence depends on proof, quality, docs, internal review, external review, remediation, next planning, next review, and ceremony. | End after proof demo or internal review. | `WP-10` through `WP-18` define the closeout tail. | [WBS](WBS_v0.91.3.md) |

## Open Questions

- Which exact signed trace bundle shape should become mandatory in `v0.91.4`?
  Owner: ADL maintainers. Tracking: `v0.91.4` planning.
- Which post-`v0.91.4` milestone should own broader Software Development Polis
  standing enforcement? Owner: ADL maintainers. Tracking: next milestone
  planning.

## Exit Criteria

- Milestone-critical decisions are logged.
- Deferred questions are routed to `v0.91.4` or later planning.
- Decisions do not claim default C-SDLC operation before `v0.91.4`.
