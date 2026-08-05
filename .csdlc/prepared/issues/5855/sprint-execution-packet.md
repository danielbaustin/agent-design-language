# Runtime, Observatory, Polis, and Protocol Sprint Design Execution Packet

## Metadata

- Sprint issue: `#5855`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Status: `prepared`
- Machine packet: `.csdlc/prepared/issues/5855/sprint-execution-packet.yaml`

## Sprint Goal

Deliver one resilient Runtime and Observatory path, then distributed, protocol, provider, and consumer integration.

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
| `#5800` | supporting | initialized | browser-trusted local Observatory HTTPS with reproducible browser and health proof | child session owner |
| `#5820` | WP-03 | initialized | one Guardian-owned launch path with resilient startup, configuration, recovery, and lifecycle behavior | child session owner |
| `#5795` | supporting | initialized | real local model-backed Shepherd dialogue through governed Runtime v3 and Observatory surfaces | child session owner |
| `#5821` | WP-04 | initialized | architecture and security gate followed by completion of the bounded 16-issue distributed-runtime program within v0.92 | child session owner |
| `#5832` | WP-14 | initialized | reconciled versioned protocol family, protobuf schema, public catalog, JSON projection, and authenticated full-duplex WSS carrier | child session owner |
| `#5837` | WP-18A | initialized | separate consumers integrated with the versioned Runtime API and WSS | child session owner |

## Recommended Execution Order

1. Route `#5800` only when its issue-wave dependencies and this packet serial gates are satisfied.
2. Route `#5820` only when its issue-wave dependencies and this packet serial gates are satisfied.
3. Route `#5795` only when its issue-wave dependencies and this packet serial gates are satisfied.
4. Route `#5821` only when its issue-wave dependencies and this packet serial gates are satisfied.
5. Route `#5832` only when its issue-wave dependencies and this packet serial gates are satisfied.
6. Route `#5837` only when its issue-wave dependencies and this packet serial gates are satisfied.

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
| `#5800` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5820` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5795` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5821` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5832` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5837` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| lane 1 | `#5821` | Distributed-polis architecture and security work remains in its child worktree. | Runtime ingress contracts from issue 5820 are stable |
| lane 2 | `#5832` | Protocol work is isolated after its distributed-runtime dependency. | issue 5821 and the declared ACIP substrate and trace baselines are complete |
| lane 3 | `#5795` | Local-provider work cannot redefine Observatory, Runtime, or WP-14 protocol contracts. | preparation may precede integration; integration waits for issues 5800 and 5820 plus WP-14 issue 5832 contract stability |

## Candidate Parallel Lanes

| Lane | Classification | Issues | Expected write sets | Dependency gate | Collision posture |
|---|---|---|---|---|---|
| candidate 1 | safe_parallel | `#5821` | child worktree | Runtime ingress contracts from issue 5820 are stable | collapse to serial on overlap |
| candidate 2 | safe_parallel | `#5832` | child worktree | issue 5821 and declared protocol baselines are complete | collapse to serial on overlap |
| candidate 3 | safe_parallel | `#5795` | child worktree | preparation may precede integration; integration waits for issues 5800 and 5820 plus WP-14 issue 5832 contract stability | collapse to serial on overlap |

## Serial Gates

| Gate | Blocks | Exit condition | Owner |
|---|---|---|---|
| gate 1 | downstream children | issues 5800 and 5820 establish the trusted local launch baseline | sprint session |
| gate 2 | downstream children | issue 5795 integrates after issues 5800 and 5820 plus WP-14 issue 5832 contract stability | sprint session |
| gate 3 | downstream children | issue 5837 integrates after issues 5820 and 5832 and its WP-18 dependency | sprint session |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- The umbrella validator proves membership, packet completeness, and routing boundaries only.
- Any overlap, unmet dependency, or unsupported completion claim fails closed.

## Parallelism Outcome Plan

- Start only the lanes classified safe in this packet.
- Collapse a lane to serial execution immediately if real write or proof surfaces overlap.
- Record planned versus actual parallelism in the sprint review; parallelism is an optimization, not acceptance evidence.

## Sprint Activity Log

- Declared path: `.csdlc/evidence/5855/activity.jsonl`
- Record child start, bind, validation, review, PR state, terminal state, and any gate change.

## Sprint-Level Review

- Declared path: `.csdlc/evidence/5855/sprint-review.md`
- Review every child result, integration boundary, failed or deferred lane, and residual route before closing the umbrella.

## Sprint Closeout Rollup Expectations

- Roll up every child issue and PR state without converting unknown or waiting states into success.
- Record budget variance only from actual child goal data.
- Record which parallel lanes were safe, collapsed to serial, blocked, or not attempted.
- Close the umbrella only after every child has truthful terminal state.
