# Quality and Release-Tail Sprint Design Execution Packet

## Metadata

- Sprint issue: `#5856`
- Milestone: `v0.92`
- Execution mode: `sequential`
- Status: `prepared`
- Machine packet: `.csdlc/prepared/issues/5856/sprint-execution-packet.yaml`

## Sprint Goal

Reduce and refactor safely, enforce the quality gate, complete reviews, remediate findings, and close the milestone.

## Sprint Boundary

In scope:

- Coordinate only the listed child issues through their existing typed v2 lifecycles.
- Preserve exact dependencies, separate worktrees, issue-bound goals, proof, review, and terminal truth.

Out of scope:

- Implementing child code or documentation in the umbrella session.
- Replacing child validation, review, publication, or closeout authority.

## Child Issue Wave

| Issue | Role | Status | Primary surface | Watcher |
|---|---|---|---|---|
| `#5786` | WP-21 | initialized | behavior-preserving cleanup with exact deletion denominator | child session owner |
| `#5841` | WP-21A | initialized | behavior-preserving simplification of active Rust ownership boundaries, duplication, and maintainability hotspots before review | child session owner |
| `#5842` | WP-22 | initialized | quality gate that blocks internal review until every indexed v0.92 feature is landed with accepted exact-revision proof | child session owner |
| `#5843` | WP-23 | initialized | current canonical docs, release notes, feature list, ADR plan, skills, agent guidance, and milestone docs | child session owner |
| `#5846` | WP-25 | initialized | internal review report and finding register | child session owner |
| `#5847` | WP-26 | initialized | external review handoff and received review | child session owner |
| `#5848` | WP-27 | initialized | finding dispositions and remediation PRs | child session owner |
| `#5849` | WP-28 | initialized | v0.93 handoff and downstream planning update | child session owner |
| `#5850` | WP-28A | initialized | exact terminal issue, PR, receipt, and ceremony sequence | child session owner |
| `#5851` | WP-29 | initialized | review pass over v0.93 planning and closeout readiness | child session owner |
| `#5852` | WP-30 | initialized | release evidence package and ceremony closeout | child session owner |

## Recommended Execution Order

1. Route `#5786` only when its issue-wave dependencies and this packet serial gates are satisfied.
2. Route `#5841` only when its issue-wave dependencies and this packet serial gates are satisfied.
3. Route `#5842` only when its issue-wave dependencies and this packet serial gates are satisfied.
4. Route `#5843` only when its issue-wave dependencies and this packet serial gates are satisfied.
5. Route `#5846` only when its issue-wave dependencies and this packet serial gates are satisfied.
6. Route `#5847` only when its issue-wave dependencies and this packet serial gates are satisfied.
7. Route `#5848` only when its issue-wave dependencies and this packet serial gates are satisfied.
8. Route `#5849` only when its issue-wave dependencies and this packet serial gates are satisfied.
9. Route `#5850` only when its issue-wave dependencies and this packet serial gates are satisfied.
10. Route `#5851` only when its issue-wave dependencies and this packet serial gates are satisfied.
11. Route `#5852` only when its issue-wave dependencies and this packet serial gates are satisfied.

## Watcher Policy

- Each active child session owns its PR/check/review watch or explicitly hands it to a watcher.
- Waiting is not failure; blockers and changed gates are recorded without moving unrelated children.
- Completion requires live issue/PR truth and typed child terminal truth to agree.

## Budget And Goal Accounting

- No sprint-global token budget is preallocated.
- Every implementation session creates its own issue-bound goal after bind and readiness.
- Actual time and token use are recorded per child when available and are never inferred as zero.

## Watcher Plan

| Issue | Watcher | Current focus | Next terminal state |
|---|---|---|---|
| `#5786` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5841` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5842` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5843` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5846` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5847` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5848` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5849` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5850` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5851` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5852` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| none | none | This sprint is intentionally sequential. | Honor every serial gate. |

## Candidate Parallel Lanes

| Lane | Classification | Issues | Expected write sets | Dependency gate | Collision posture |
|---|---|---|---|---|---|
| release tail | serial_gate | `#5786`, `#5841`, `#5842`, `#5843`, `#5846`, `#5847`, `#5848`, `#5849`, `#5850`, `#5851`, `#5852` | child-owned | prior child terminal | parallel execution prohibited |

## Serial Gates

| Gate | Blocks | Exit condition | Owner |
|---|---|---|---|
| gate 1 | downstream children | 5786 before 5841 | sprint session |
| gate 2 | downstream children | 5841 before 5842 | sprint session |
| gate 3 | downstream children | 5842 before 5843 | sprint session |
| gate 4 | downstream children | 5843 before 5846 | sprint session |
| gate 5 | downstream children | issues 5846 through 5852 execute in dependency order | sprint session |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- The umbrella validator proves membership, packet completeness, and routing boundaries only.
- Any overlap, unmet dependency, or unsupported completion claim fails closed.

## Parallelism Outcome Plan

- Start only the lanes classified safe in this packet.
- Collapse a lane to serial execution immediately if real write or proof surfaces overlap.
- Record planned versus actual parallelism in the sprint review; parallelism is an optimization, not acceptance evidence.

## Sprint Activity Log

- Declared path: `.csdlc/evidence/5856/activity.jsonl`
- Record child start, bind, validation, review, PR state, terminal state, and any gate change.

## Sprint-Level Review

- Declared path: `.csdlc/evidence/5856/sprint-review.md`
- Review every child result, integration boundary, failed or deferred lane, and residual route before closing the umbrella.

## Sprint Closeout Rollup Expectations

- Roll up every child issue and PR state without converting unknown or waiting states into success.
- Record budget variance only from actual child goal data.
- Record which parallel lanes were safe, collapsed to serial, blocked, or not attempted.
- Close the umbrella only after every child has truthful terminal state.
