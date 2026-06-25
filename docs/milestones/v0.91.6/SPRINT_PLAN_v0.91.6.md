# v0.91.6 Sprint Plan

## Metadata

- Sprint: `v0.91.6-bridge-tranche-1`
- Milestone: `v0.91.6`
- Start date: not scheduled
- End date: not scheduled
- Owner: ADL maintainers
- Status: bridge issue waves opened; WP-03 through WP-10 and ACIP runtime now have bounded merged/closed truth; integrated runtime soak and closeout-tail lanes remain open downstream work

## Status

The first bridge tranche is no longer just a WP-03/WP-04 opening plan. The
milestone now has retained closeout truth for WP-03 `#3968`, WP-04 `#3969`,
WP-05 `#3970`, WP-06 `#3971`, WP-07 `#3972`, WP-08 `#3973`, WP-09 `#3974`,
and WP-10 `#3975`, while open downstream runtime and closeout-tail work
remains explicitly routed.

Current issue truth for this sprint should be consumed from:

- [review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md](review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md)
  for closed bridge umbrellas and their retained evidence posture
- [CLOSEOUT_TAIL_SPRINT_v0.91.6.md](CLOSEOUT_TAIL_SPRINT_v0.91.6.md)
  for the ordered open release-tail issue wave

This sprint plan may summarize those surfaces, but it is not the canonical
per-issue current-state ledger.

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
| 2 | Open/promote first-tranche implementation issues | first-tranche issue wave opened and executed through completed umbrellas `#3968`, `#3969`, `#3970`, `#3971`, `#3972`, `#3973`, and `#3975` | ADL maintainers | complete for the completed-wave set |
| 3 | Resilience, persistence, sleep/wake execution route | `#3967` plus child wave `#3986`-`#3993` | ADL maintainers | merged and closed with residual continuity work routed |
| 4 | Tooling proof-loop reliability route | `#3968`, `#3995`-`#4001` | ADL maintainers | merged |
| 5 | Public prompt records export route | `#3969`, `#4002`-`#4006` | ADL maintainers | merged and closed through umbrella `#3969` |
| 6 | Provider/model reliability route | `#3970`, `#4007`-`#4012`, `#4053` merged and closed; `#4111` follow-on reconciliation | ADL maintainers | merged and closed with reconciliation follow-on |
| 7 | ACIP/A2A/provider communications route | `#3971`, `#4013`-`#4018`, `#4055` | ADL maintainers | merged and closed |
| 8 | Security bridge and CAV route | `#3972`, `#4019`-`#4024`, `#4064` | ADL maintainers | merged and closed with explicit downstream residual routes |
| 9 | Identity, Observatory/Unity, AEE, Memory/ObsMem, ACP accounting | `#3973`, `#3974`, and `#3975` merged and closed or in final umbrella closeout publication | ADL maintainers | closeout-ready: identity, Observatory/Unity, and WP-10 retained truth is bounded |
| 10 | Bridge-ledger refresh and `v0.91.7` handoff | not opened | ADL maintainers | planned |
| 11 | `agent-logic.ai` AWS account/setup planning | `#3902` | ADL maintainers | complete; AWS Activate review tracked as post-close external follow-up |
| 12 | CodeFriend v1 / adapter v2 and guild route preservation | feature-list routes | ADL maintainers | planned |
| 13 | Runtime integration soak sprint planning and Soak #1 route | `#4185` | ADL maintainers | planned |

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
- Dependency: Soak #1 must wait for Tokio substrate readiness and stay scoped
  to a walking-skeleton integration proof in `v0.91.6`. Soak #2 in `v0.91.7`
  owns the full feature-list integration target, with Soak #3 as contingency if
  needed before `v0.92`.
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
- runtime Soak #1 placement, Soak #2/#3 handoff, and non-claims in
  [RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md](RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md).

## Closeout Bar

- All planned docs exist.
- Review findings are fixed or routed.
- `v0.91.7` residuals remain visible.
- `v0.92` consumption truth is explicit.

## Closeout-tail sprint standardization

The milestone closeout tail is now treated as one ordered sprint surface rather than a set of unrelated mini-sprints. For the standard sequence, dependency gates, watcher expectations, remediation routing, and automation guidance, use [CLOSEOUT_TAIL_SPRINT_v0.91.6.md](CLOSEOUT_TAIL_SPRINT_v0.91.6.md).

For current release-tail issue truth, read the canonical ordered issue wave in
[CLOSEOUT_TAIL_SPRINT_v0.91.6.md](CLOSEOUT_TAIL_SPRINT_v0.91.6.md) rather than
maintaining a second hand-updated list here.
