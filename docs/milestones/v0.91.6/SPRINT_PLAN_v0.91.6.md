# v0.91.6 Sprint Plan

## Metadata

- Sprint: `v0.91.6-bridge-tranche-1`
- Milestone: `v0.91.6`
- Start date: not scheduled
- End date: not scheduled
- Owner: ADL maintainers
- Status: issue waves opened; WP-03 merged; WP-04 merged and closed; WP-05 merged and closed with follow-on reconciliation in progress

## Status

Issue opening has completed for the WP-03/WP-04 tranche. WP-04 is complete
through umbrella `#3969` and child implementation issues `#4002` through
`#4006`.

## Sprint Overview

Complete the first bridge tranche before `v0.92` activation refresh. The sprint
turns planning docs and feature docs into issue-ready work with review gates.

## Sprint Goals

- Promote the `v0.91.6` planning package.
- Complete first-tranche bridge feature docs.
- Route or resolve tooling reliability blockers.
- Preserve explicit `v0.91.7` residuals.
- Refresh bridge-ledger truth for `v0.92` consumers.

## Work Plan

| Order | Item | Issue | Owner | Status |
| --- | --- | --- | --- | --- |
| 1 | Complete planning and feature-doc package | `#3824` | ADL maintainers | in progress |
| 2 | Open/promote first-tranche implementation issues | `#3968`, `#3969`, `#3995`-`#4006` opened for the WP-03/WP-04 tranche | ADL maintainers | in progress |
| 3 | Resilience, persistence, sleep/wake execution route | not opened | ADL maintainers | planned |
| 4 | Tooling proof-loop reliability route | `#3968`, `#3995`-`#4001` | ADL maintainers | merged |
| 5 | Public prompt records export route | `#3969`, `#4002`-`#4006` | ADL maintainers | merged and closed through umbrella `#3969` |
| 6 | Provider/model reliability route | `#3970`, `#4007`-`#4012`, `#4053` merged and closed; `#4111` follow-on reconciliation | ADL maintainers | merged and closed with reconciliation follow-on |
| 7 | ACIP/A2A/provider communications route | not opened | ADL maintainers | planned |
| 8 | Security bridge and CAV route | not opened | ADL maintainers | planned |
| 9 | Identity, Observatory/Unity, AEE, Memory/ObsMem, ACP accounting | not opened | ADL maintainers | planned |
| 10 | Bridge-ledger refresh and `v0.91.7` handoff | not opened | ADL maintainers | planned |
| 11 | `agent-logic.ai` AWS account/setup planning | `#3902` | ADL maintainers | complete; AWS Activate review tracked as post-close external follow-up |
| 12 | CodeFriend v1 / adapter v2 and guild route preservation | feature-list routes | ADL maintainers | planned |

## Execution Policy

- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Planning docs alone never prove runtime readiness.
- Each feature-like WP must finish as complete, blocked, deferred, or routed.
- `v0.92` remains blocked until bridge truth is reviewed.

## Risks / Dependencies

- Dependency: `#3825` must complete the second-tranche docs package, building
  on the `#3801` planning package.
- Dependency: `#3902` remains an account/setup planning item and is not treated
  as v0.92 activation proof. Account setup is complete; AWS Activate review
  and private credit visibility remain post-close external follow-up.
- Dependency: CodeFriend and guild routes must remain visible without widening
  the first-tranche bridge implementation scope.
- Risk: tooling validation friction slows docs-only work.
  - Mitigation: keep tooling problems captured in remediation issues and use
    focused docs validation.
- Risk: feature docs overclaim readiness.
  - Mitigation: use explicit non-goals and consumption limits.

## Demo / Review Plan

No runnable demo is required for this docs tranche. Review should inspect:

- created docs and indexes;
- bridge-ledger consumption limits;
- absence of runtime completion claims;
- residual routing to `v0.91.7`.

## Closeout Bar

- All planned docs exist.
- Review findings are fixed or routed.
- `v0.91.7` residuals remain visible.
- `v0.92` consumption truth is explicit.

## Closeout-tail sprint standardization

The milestone closeout tail is now treated as one ordered sprint surface rather than a set of unrelated mini-sprints. For the standard sequence, dependency gates, watcher expectations, remediation routing, and automation guidance, use [CLOSEOUT_TAIL_SPRINT_v0.91.6.md](CLOSEOUT_TAIL_SPRINT_v0.91.6.md).

For `v0.91.6`, the ordered closeout-tail issue wave is:

1. `#3976` demo convergence
2. `#3977` quality gate
3. `#3978` docs and review alignment
4. `#3979` internal review
5. `#3980` external review
6. `#3981` remediation and final preflight
7. `#3982` next milestone planning
8. `#3983` next milestone review
9. `#3984` release ceremony

Every issue in this closeout tail should have active watcher coverage whenever it is waiting on checks, review, mergeability, or an upstream dependency, with polling no slower than every 30 seconds while blocked.
