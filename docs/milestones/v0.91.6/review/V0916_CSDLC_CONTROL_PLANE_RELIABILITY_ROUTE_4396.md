# v0.91.6 C-SDLC Control-Plane Reliability Route

Issue: `#4396`
Status: `retained_route_packet`
Date: 2026-06-23
Parent sprint: `#4388`

## Scope

This packet records the truthful routed disposition for the bounded
`#4396` reliability lane inside the `#4388` C-SDLC integration control-plane
sprint.

`#4396` did not land as one standalone merged implementation slice. Instead,
the remaining reliability rough edges were split into concrete late
control-plane owner issues and then consumed by v0.91.7 planning as explicit
closeout input.

## Route Result

`#4396` is satisfiable as a routed reliability lane.

The bounded acceptance bar for `#4396` was:

- fix remaining rough edges in-scope where practical; or
- route them into concrete follow-on owners with evidence and sprint routing.

The repository now has that second form of completion. The remaining rough-edge
surface is explicitly routed through the late v0.91.6 / early v0.91.7
control-plane stream rather than left as unowned residue.

## Routed Reliability Surfaces

The reliability surface named by `#4396` is now distributed across these
tracked owner issues:

| Surface | Routed owner(s) | Why this satisfies `#4396` |
| --- | --- | --- |
| Root-checkout and multi-session safety | `#4405` | Records the root-checkout and session-coordination guardrails needed for reliable multi-session execution. |
| Session ledger and cross-session lifecycle continuity | `#4412` | Owns the durable session-ledger and cross-session coordination command surface instead of leaving it implicit. |
| Lifecycle delegate stalls and long-running command liveness | `#4413` | Owns the lifecycle-liveness defect class that showed up in `pr run` / doctor behavior and other long-running control-plane calls. |
| Validation throughput, path ownership, and lifecycle automation | `#4417`-`#4421` | Own the validation-manager and lifecycle-automation reliability slices that were too broad to hide inside `#4396`. |
| Generated VPP planning from ownership/validation facts | `#4425` | Owns turning validation planning into generated truth instead of chat-memory policy. |
| Forward issue-goal metrics capture | `#4431` | Owns durable time/token capture for issue execution rather than leaving reliability accounting as manual memory. |
| Bounded v0.91.6 metrics backfill | `#4441` | Owns the limited backfill needed to normalize v0.91.6 process truth without overclaiming archaeology precision. |
| Operational adoption of watcher/VPP/shepherd defaults | `#4433`-`#4438` | Own the operational-adoption slices required to make the control plane reliably used rather than merely available. |
| Goal snapshots and lifecycle shepherding | `#4442`, `#4443` | Own the remaining v0.91.7-facing lifecycle reliability and shepherding surfaces above watcher/janitor/closeout. |

## Evidence

The routing above is already reflected in tracked planning truth:

- `docs/milestones/v0.91.7/PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- `docs/milestones/v0.91.7/WBS_v0.91.7.md`
- `docs/milestones/v0.91.7/SPRINT_PLAN_v0.91.7.md`
- `docs/milestones/v0.91.7/README.md`
- `docs/milestones/v0.91.7/FEATURE_DOCS_v0.91.7.md`
- `docs/milestones/v0.91.7/MILESTONE_CHECKLIST_v0.91.7.md`

Those planning surfaces already treat `#4388` plus the late input wave
`#4405`, `#4412`-`#4413`, `#4417`-`#4421`, `#4425`, `#4431`, `#4441`,
`#4433`-`#4438`, and `#4442`-`#4443` as the required truth-consumption stream
for reliable sprint-scale execution.

## Findings

No new P1/P2/P3 findings remain inside the bounded `#4396` route packet.

Residual caveat:

- `#4396` itself is still open in GitHub at the time this packet is written.
  This packet establishes the retained evidence needed to close it truthfully
  as a routed lane rather than a missing implementation.

## Validation And Evidence

Focused local checks for this repair:

```text
git diff --check
```

Live/tracked evidence consumed:

- live GitHub issue state for `#4396`;
- tracked sprint packet
  `.adl/v0.91.6/sprints/issue-4388__csdlc-integration-control-plane/SPRINT_EXECUTION_PACKET.md`;
- tracked v0.91.7 planning and handoff surfaces listed above.

## Non-Claims

- This packet does not claim every routed owner issue is already closed.
- This packet does not claim `#4396` produced one merged implementation PR.
- This packet does not claim the whole control-plane reliability program is
  complete; it claims the `#4396` lane is no longer unowned.
- This packet does not replace issue-local review or closeout on the routed
  owner issues themselves.

## Closeout Position

`#4396` can be closed truthfully after this retained route packet lands because
its remaining reliability rough edges are now explicitly routed to concrete
owner issues instead of remaining as unresolved sprint residue.
