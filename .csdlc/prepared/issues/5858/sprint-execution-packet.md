# Foundation and Throughput Sprint Design Execution Packet

## Metadata

- Sprint issue: `#5858`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Status: `prepared`
- Machine packet: `.csdlc/prepared/issues/5858/sprint-execution-packet.yaml`

## Sprint Goal

Establish current documentation, repository ownership, reliable proof infrastructure, and fast operator workflows.

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
| `#5818` | WP-01B | initialized | v0.92 current-version truth across docs/planning/ADL_FEATURE_LIST.md, canonical docs, READMEs, manifests, Cargo metadata, skills, and runbooks | child session owner |
| `#5819` | WP-02 | initialized | five serially transferred company repositories with danielbaustin/asksifu retained as personal and Horust excluded | child session owner |
| `#5812` | supporting | initialized | Clippy-clean Freedom Gate defaults with unchanged runtime behavior | child session owner |
| `#5801` | WP-02A | initialized | reliable focused and slow test routing, coverage aggregation, and platform parity | child session owner |
| `#5853` | WP-02B | initialized | a measured and reversible post-migration build acceleration decision for the standard and 16-core GitHub-hosted runner comparison | child session owner |
| `#5822` | WP-05 | initialized | measured estimation, reconnection, and simplified lifecycle path | child session owner |
| `#5823` | WP-06 | initialized | portable bounded runner with provenance and failover | child session owner |
| `#5824` | WP-07 | initialized | historical-delivery audit and only the proven remaining enum/schema correction | child session owner |

## Recommended Execution Order

1. Route `#5818` only when its issue-wave dependencies and this packet serial gates are satisfied.
2. Route `#5819` only when its issue-wave dependencies and this packet serial gates are satisfied.
3. Route `#5812` only when its issue-wave dependencies and this packet serial gates are satisfied.
4. Route `#5801` only when its issue-wave dependencies and this packet serial gates are satisfied.
5. Route `#5853` only when its issue-wave dependencies and this packet serial gates are satisfied.
6. Route `#5822` only when its issue-wave dependencies and this packet serial gates are satisfied.
7. Route `#5823` only when its issue-wave dependencies and this packet serial gates are satisfied.
8. Route `#5824` only when its issue-wave dependencies and this packet serial gates are satisfied.

## Watcher Policy

- Each active child session owns its PR/check/review watch or explicitly hands it to a watcher.
- Waiting is not failure; blockers and changed gates are recorded without moving unrelated children.
- Completion requires live issue/PR truth and typed child terminal truth to agree.

## Budget And Goal Accounting

- No sprint-global token budget is preallocated.
- After WP-01 releases its publication claim, every implementation session
  registers its child worktree, reacquires the exact issue-local claim, binds,
  and creates its own issue-bound goal before implementation.
- Actual time and token use are recorded per child when available and are never inferred as zero.

## Watcher Plan

| Issue | Watcher | Current focus | Next terminal state |
|---|---|---|---|
| `#5818` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5819` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5812` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5801` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5853` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5822` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5823` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5824` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| lane 1 | `#5822`, `#5823` | C-SDLC efficiency and remote validation use separate child worktrees. | issue 5801 complete |
| lane 2 | `#5812` | Freedom Gate cleanup remains a bounded child repair. | coordinate with issue 5801 |

## Candidate Parallel Lanes

| Lane | Classification | Issues | Expected write sets | Dependency gate | Collision posture |
|---|---|---|---|---|---|
| candidate 1 | safe_parallel | `#5822`, `#5823` | disjoint child worktrees | issue 5801 complete | collapse to serial on overlap |
| candidate 2 | safe_parallel | `#5812` | disjoint child worktrees | coordinate with issue 5801 | collapse to serial on overlap |

## Serial Gates

| Gate | Blocks | Exit condition | Owner |
|---|---|---|---|
| gate 1 | downstream children | 5818 before 5819 | sprint session |
| gate 2 | downstream children | 5819 before 5801 and 5853 | sprint session |
| gate 3 | downstream children | 5801 before 5853, 5822, and 5823 | sprint session |
| gate 4 | downstream children | 5822 before 5824 | sprint session |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- The umbrella validator proves membership, packet completeness, and routing boundaries only.
- Any overlap, unmet dependency, or unsupported completion claim fails closed.

## Parallelism Outcome Plan

- Start only the lanes classified safe in this packet.
- Collapse a lane to serial execution immediately if real write or proof surfaces overlap.
- Record planned versus actual parallelism in the sprint review; parallelism is an optimization, not acceptance evidence.

## Sprint Activity Log

- Declared path: `.csdlc/evidence/5858/activity.jsonl`
- Record child start, bind, validation, review, PR state, terminal state, and any gate change.

## Sprint-Level Review

- Declared path: `.csdlc/evidence/5858/sprint-review.md`
- Review every child result, integration boundary, failed or deferred lane, and residual route before closing the umbrella.

## Sprint Closeout Rollup Expectations

- Roll up every child issue and PR state without converting unknown or waiting states into success.
- Record budget variance only from actual child goal data.
- Record which parallel lanes were safe, collapsed to serial, blocked, or not attempted.
- Close the umbrella only after every child has truthful terminal state.
