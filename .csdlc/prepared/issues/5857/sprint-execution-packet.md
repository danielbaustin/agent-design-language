# First-Birthday Core Sprint Design Execution Packet

## Metadata

- Sprint issue: `#5857`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Status: `prepared`
- Machine packet: `.csdlc/prepared/issues/5857/sprint-execution-packet.yaml`

## Sprint Goal

Build the first-birthday identity, continuity, memory, capability, learning, witness, and review contracts.

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
| `#5825` | WP-08 | initialized | birth contract, disqualifying cases, and negative fixtures | child session owner |
| `#5826` | WP-09 | initialized | stable-name and identity-root contract | child session owner |
| `#5827` | WP-10 | initialized | continuity records and bounded-cycle proof | child session owner |
| `#5828` | WP-11 | initialized | working Memory Palace context topology, bounded working-set materialization, and redaction-safe continuity proof | child session owner |
| `#5829` | WP-12 | initialized | provider, model, tool, skill, authority, and limit envelope | child session owner |
| `#5830` | WP-13 | initialized | evidence-grounded cognitive-profile contract | child session owner |
| `#5831` | WP-13A | initialized | working evaluation bindings, durable adaptation deltas, governed graph mutation, and replay-safe Adaptive Learning DAG execution | child session owner |
| `#5833` | WP-15 | initialized | witness set and citizen-facing receipt contract | child session owner |
| `#5834` | WP-16 | initialized | reviewer-facing birthday evidence packet | child session owner |

## Recommended Execution Order

1. Route `#5825` only when its issue-wave dependencies and this packet serial gates are satisfied.
2. Route `#5826` only when its issue-wave dependencies and this packet serial gates are satisfied.
3. Route `#5827` only when its issue-wave dependencies and this packet serial gates are satisfied.
4. Route `#5828` only when its issue-wave dependencies and this packet serial gates are satisfied.
5. Route `#5829` only when its issue-wave dependencies and this packet serial gates are satisfied.
6. Route `#5830` only when its issue-wave dependencies and this packet serial gates are satisfied.
7. Route `#5831` only when its issue-wave dependencies and this packet serial gates are satisfied.
8. Route `#5833` only when its issue-wave dependencies and this packet serial gates are satisfied.
9. Route `#5834` only when its issue-wave dependencies and this packet serial gates are satisfied.

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
| `#5825` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5826` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5827` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5828` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5829` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5830` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5831` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5833` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5834` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| lane 1 | `#5828`, `#5829` | Memory and capability work retain separate child ownership. | issue 5827 is complete |
| lane 2 | `#5830` | Cognitive-profile work starts only after its memory and capability dependencies. | issues 5827, 5828, and 5829 are complete |
| lane 3 | `#5831` | Adaptive-learning work starts only after its cognitive-profile dependency. | issue 5830 and the declared loop evidence are complete |

## Candidate Parallel Lanes

| Lane | Classification | Issues | Expected write sets | Dependency gate | Collision posture |
|---|---|---|---|---|---|
| candidate 1 | safe_parallel | `#5828`, `#5829` | disjoint child worktrees | issue 5827 is complete | collapse to serial on overlap |
| candidate 2 | safe_parallel | `#5830` | child worktree | issues 5827, 5828, and 5829 are complete | collapse to serial on overlap |
| candidate 3 | safe_parallel | `#5831` | child worktree | issue 5830 and the declared loop evidence are complete | collapse to serial on overlap |

## Serial Gates

| Gate | Blocks | Exit condition | Owner |
|---|---|---|---|
| gate 1 | downstream children | 5825 before 5826 and 5829 | sprint session |
| gate 2 | downstream children | 5826 before 5827 and 5828 | sprint session |
| gate 3 | downstream children | issues 5827, 5828, 5829, and 5830 before 5833 | sprint session |
| gate 4 | downstream children | issues 5831 and 5833 before 5834 | sprint session |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- The umbrella validator proves membership, packet completeness, and routing boundaries only.
- Any overlap, unmet dependency, or unsupported completion claim fails closed.

## Parallelism Outcome Plan

- Start only the lanes classified safe in this packet.
- Collapse a lane to serial execution immediately if real write or proof surfaces overlap.
- Record planned versus actual parallelism in the sprint review; parallelism is an optimization, not acceptance evidence.

## Sprint Activity Log

- Declared path: `.csdlc/evidence/5857/activity.jsonl`
- Record child start, bind, validation, review, PR state, terminal state, and any gate change.

## Sprint-Level Review

- Declared path: `.csdlc/evidence/5857/sprint-review.md`
- Review every child result, integration boundary, failed or deferred lane, and residual route before closing the umbrella.

## Sprint Closeout Rollup Expectations

- Roll up every child issue and PR state without converting unknown or waiting states into success.
- Record budget variance only from actual child goal data.
- Record which parallel lanes were safe, collapsed to serial, blocked, or not attempted.
- Close the umbrella only after every child has truthful terminal state.
