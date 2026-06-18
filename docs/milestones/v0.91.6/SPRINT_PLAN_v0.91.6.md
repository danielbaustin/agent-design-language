# v0.91.6 Sprint Plan

## Metadata

- Sprint: `v0.91.6-bridge-tranche-1`
- Milestone: `v0.91.6`
- Start date: not scheduled
- End date: not scheduled
- Owner: ADL maintainers
- Status: issue waves opened; WP-03 merged; WP-04 child wave merged; umbrella closeout pending merge

## Status

Issue opening has completed for the WP-03/WP-04 tranche. WP-04 is no longer
just executing at the umbrella truth layer: child implementation issues
`#4002` through `#4006` are merged, and only the final umbrella closeout
remains.

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
| 5 | Public prompt records export route | `#3969`, `#4002`-`#4006` | ADL maintainers | child lanes merged through `#4006`; umbrella closeout pending |
| 6 | Provider/model reliability route | not opened | ADL maintainers | planned |
| 7 | ACIP/A2A/provider communications route | not opened | ADL maintainers | planned |
| 8 | Security bridge and CAV route | not opened | ADL maintainers | planned |
| 9 | Identity, Observatory/Unity, AEE, Memory/ObsMem, ACP accounting | not opened | ADL maintainers | planned |
| 10 | Bridge-ledger refresh and `v0.91.7` handoff | not opened | ADL maintainers | planned |
| 11 | `agent-logic.ai` AWS account/setup planning | `#3902` | ADL maintainers | planned |
| 12 | CodeFriend v1 / adapter v2 and guild route preservation | feature-list routes | ADL maintainers | planned |

## Execution Policy

- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Planning docs alone never prove runtime readiness.
- Each feature-like WP must finish as complete, blocked, deferred, or routed.
- `v0.92` remains blocked until bridge truth is reviewed.

## Risks / Dependencies

- Dependency: `#3825` must complete the second-tranche docs package, building
  on the `#3801` planning package.
- Dependency: `#3902` must remain an account/setup planning item and must not
  be treated as v0.92 activation proof.
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
