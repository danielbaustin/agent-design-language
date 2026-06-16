# v0.91.7 Sprint Plan

## Metadata

- Sprint: `v0.91.7-bridge-tranche-2`
- Milestone: `v0.91.7`
- Start date: not scheduled
- End date: not scheduled
- Owner: ADL maintainers
- Status: planned

## Status

Planned. This document defines execution order; it does not start runtime work.

## Sprint Overview

Complete the second bridge tranche before `v0.92` activation refresh.

## Sprint Goals

- Promote the `v0.91.7` planning package.
- Complete second-tranche residual feature docs.
- Preserve exact activation consumption limits for `#3780`.
- Keep security and ACIP/A2A residuals visible.
- Prevent major conceptual surfaces from moving into `v0.92` as implicit work.

## Work Plan

| Order | Item | Issue | Owner | Status |
| --- | --- | --- | --- | --- |
| 1 | Complete planning and feature-doc package | `#3825` | ADL maintainers | in progress |
| 2 | Open/promote second-tranche implementation issues | not opened | ADL maintainers | planned |
| 3 | Curiosity Engine / Discovery Substrate route | not opened | ADL maintainers | planned |
| 4 | Constructability Gate route | not opened | ADL maintainers | planned |
| 5 | Reasoning graph / loop / skill standard route | not opened | ADL maintainers | planned |
| 6 | Security and ACIP/A2A residual route | not opened | ADL maintainers | planned |
| 7 | Affect, Godel, and economics bridge route | not opened | ADL maintainers | planned |
| 8 | Bridge-ledger refresh and `#3780` handoff | `V092_HANDOFF_v0.91.7.md` | ADL maintainers | doc-ready |

## Execution Policy

- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Planning docs alone never prove runtime readiness.
- Each surface exits as complete, blocked, deferred, or routed.
- `v0.92` remains blocked until bridge truth is reviewed.

## Risks / Dependencies

- Dependency: `#3824` first-tranche package is merged.
- Risk: second-tranche docs become narrative rather than actionable.
  - Mitigation: each feature doc requires decisions, validation, and
    consumption limits.
- Risk: security/ACIP residuals get deferred silently.
  - Mitigation: keep them as explicit feature docs and decision rows.

## Demo / Review Plan

No runnable demo is required for this docs tranche. Review should inspect:

- created docs and indexes;
- `#3780` consumption limits;
- absence of runtime completion claims;
- distinct handling of every residual surface.

## Closeout Bar

- All planned docs exist.
- Review findings are fixed or routed.
- `#3780` handoff truth is explicit in `V092_HANDOFF_v0.91.7.md`.
