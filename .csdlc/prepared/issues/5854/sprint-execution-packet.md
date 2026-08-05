# Demonstration, Handoff, and Publication Sprint Design Execution Packet

## Metadata

- Sprint issue: `#5854`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Status: `prepared`
- Machine packet: `.csdlc/prepared/issues/5854/sprint-execution-packet.yaml`

## Sprint Goal

Produce real demonstrations, consumer proofs, governance handoff, and complete launch media.

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
| `#5835` | WP-17 | initialized | bounded birthday-identity movement semantics and non-goals | child session owner |
| `#5836` | WP-18 | initialized | runnable first-birthday proof demo and negative suite | child session owner |
| `#5838` | WP-18B | initialized | provider-neutral multi-agent proof matrix and artifacts | child session owner |
| `#5839` | WP-19 | initialized | v0.93 governance handoff map | child session owner |
| `#5840` | WP-20 | initialized | demo matrix, AEE proof routing or packet, proof coverage, and validation commands | child session owner |
| `#5844` | WP-24 | initialized | all ten planned articles complete and ready for editorial review, followed by final release-grounded publication disposition | child session owner |
| `#5845` | WP-24A | initialized | all first ten episodes complete as review-ready production packages, not topic or schema placeholders | child session owner |

## Recommended Execution Order

1. Route `#5835` only when its issue-wave dependencies and this packet serial gates are satisfied.
2. Route `#5836` only when its issue-wave dependencies and this packet serial gates are satisfied.
3. Route `#5838` only when its issue-wave dependencies and this packet serial gates are satisfied.
4. Route `#5839` only when its issue-wave dependencies and this packet serial gates are satisfied.
5. Route `#5840` only when its issue-wave dependencies and this packet serial gates are satisfied.
6. Route `#5844` only when its issue-wave dependencies and this packet serial gates are satisfied.
7. Route `#5845` only when its issue-wave dependencies and this packet serial gates are satisfied.

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
| `#5835` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5836` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5838` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5839` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5840` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5844` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5845` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| lane 1 | `#5844`, `#5845` | Articles and podcasts are independently reviewable publication packages. | issue 5819 complete |
| lane 2 | `#5835`, `#5836` | Demo and migration planning retain separate child worktrees. | issue 5834 and each remaining declared child dependency are complete |
| lane 3 | `#5839` | Governance handoff starts only after its migration-planning dependency. | issues 5834 and 5835 and the v0.93 allocation are complete |

## Candidate Parallel Lanes

| Lane | Classification | Issues | Expected write sets | Dependency gate | Collision posture |
|---|---|---|---|---|---|
| candidate 1 | safe_parallel | `#5844`, `#5845` | disjoint child worktrees | issue 5819 complete | collapse to serial on overlap |
| candidate 2 | safe_parallel | `#5835`, `#5836` | disjoint child worktrees | issue 5834 and each remaining declared child dependency are complete | collapse to serial on overlap |
| candidate 3 | safe_parallel | `#5839` | child worktree | issues 5834 and 5835 and the v0.93 allocation are complete | collapse to serial on overlap |

## Serial Gates

| Gate | Blocks | Exit condition | Owner |
|---|---|---|---|
| gate 1 | downstream children | issues 5835 and 5836 follow issue 5834 | sprint session |
| gate 2 | downstream children | issue 5838 follows issues 5832, 5834, and 5836 | sprint session |
| gate 3 | downstream children | issue 5840 follows issues 5836, 5837, 5838, and 5839 | sprint session |
| gate 4 | downstream children | final publication claims align only after issue 5843 | sprint session |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- The umbrella validator proves membership, packet completeness, and routing boundaries only.
- Any overlap, unmet dependency, or unsupported completion claim fails closed.

## Parallelism Outcome Plan

- Start only the lanes classified safe in this packet.
- Collapse a lane to serial execution immediately if real write or proof surfaces overlap.
- Record planned versus actual parallelism in the sprint review; parallelism is an optimization, not acceptance evidence.

## Sprint Activity Log

- Declared path: `.csdlc/evidence/5854/activity.jsonl`
- Record child start, bind, validation, review, PR state, terminal state, and any gate change.

## Sprint-Level Review

- Declared path: `.csdlc/evidence/5854/sprint-review.md`
- Review every child result, integration boundary, failed or deferred lane, and residual route before closing the umbrella.

## Sprint Closeout Rollup Expectations

- Roll up every child issue and PR state without converting unknown or waiting states into success.
- Record budget variance only from actual child goal data.
- Record which parallel lanes were safe, collapsed to serial, blocked, or not attempted.
- Close the umbrella only after every child has truthful terminal state.
