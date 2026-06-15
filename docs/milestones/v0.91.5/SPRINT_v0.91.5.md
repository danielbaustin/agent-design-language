# v0.91.5 Sprint Plan

## Metadata
- Sprint: `v0.91.5`
- Milestone: `v0.91.5`
- Start date: `2026-06-02`
- End date: `pending`
- Owner: ADL maintainers
- Status: `sprint_2_review_remediation_in_progress`

## Status

`sprint_2_review_remediation_in_progress`

## How To Use

- List bridge work in planned execution order.
- Track blockers here rather than scattered chat notes.
- Keep public prompt, demo, and provider side work visible and bounded.
- Record closeout expectations before execution begins.

## Sprint Overview

v0.91.5 is split into ordered sprint bands after WP-01 opens the milestone and
confirms issue/card/sprint readiness. Sprint 1 is complete, Sprint 2 child
execution is complete, and Sprint 2 umbrella review/remediation is now the
active public planning state.

Planned scope:

- public prompt records and `.adl` transition controls;
- AEE completion-tranche routing before v0.92 consumes bridge evidence;
- provider/model matrix and multi-agent proof;
- demo matrix/showcase readiness;
- review, remediation, final v0.92 preflight, and release.

## Bridge Sprint Work

- Scope: pre-v0.92 operational bridge work.
- Boundary: no first-birthday implementation.
- Proof surface: issue wave, issue labels, AEE tranche plan, prompt packets,
  provider matrix, multi-agent proof, demo index, activation map, and `#3377`.

## Sprint Goals

- WP-01 opening: confirm active milestone truth, issue/card readiness, sprint
  umbrellas, canonical WP ordering, and newly scheduled portable adapter work.
- Sprint 1 (`#3571`): land prompt-template values rendering first, then make
  downstream cards, public prompt records, and portable project records
  reviewable.
- Sprint 2 (`#3572`): make provider/model breadth and multi-agent execution work.
- Sprint 3 (`#3573`): refresh demo matrix / demo showcase readiness.
- Sprint 4 (`#3574`): run coverage/quality, docs/review alignment, internal
  review, external review, remediation/final v0.92 preflight, next-milestone
  planning, and release ceremony.

## Sprint Goal

Make v0.92 openable without hidden operational debt.

## Planned Scope

- Public prompt packet export, redaction, and archive planning.
- AEE completion tranche and v0.92 proof routing.
- OpenRouter and model-role matrix.
- Multi-agent C-SDLC workcell proof.
- Demo showcase and Unity Observatory readiness.
- v0.92 activation map and first-birthday launch packet as WP-18 preflight
  inputs.

## Work Plan

| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | WP-01 milestone opening | `#3568`, consuming closed setup `#3506`-`#3511` | ADL maintainers | closed |
| 2 | Canonical WP ordering standard | `#3567` | ADL maintainers | closed |
| 3 | Sprint 1 umbrella: prompt/public records and portable ADL readiness | `#3571` | ADL maintainers | closed |
| 4 | Prompt-template values renderer | `#3553` | ADL maintainers | closed |
| 5 | Downstream card rewrite after prompt templates v1.1 | `#3582` | ADL maintainers | closed |
| 6 | Public prompt records | `#3472`-`#3476` | ADL maintainers | closed |
| 7 | Portable ADL adapter contract | `#3569` | ADL maintainers | closed |
| 8 | Sprint 2 umbrella: provider matrix and multi-agent proof | `#3572` | ADL maintainers | review/remediation active |
| 9 | Provider/model matrix | `#3501`, `#3505` | ADL maintainers | closed |
| 10 | Multi-agent proof | `#3415`, `#3503`, `#3504` (`#3484` satisfied/closed evidence) | ADL maintainers | closed |
| 11 | Sprint 3 umbrella: demo matrix / demo showcase refresh | `#3573` | ADL maintainers | seeded |
| 12 | Demo matrix / demo showcase refresh | `#3455` with `#3460`, `#3461` as supporting demo inputs | ADL maintainers | moved |
| 13 | Sprint 4 umbrella: coverage, review, remediation, planning, release | `#3574` | ADL maintainers | seeded |
| 14 | Coverage / quality gate | `#3575`, consuming `#3531` | ADL maintainers | seeded |
| 15 | Docs + review alignment | `#3579` | ADL maintainers | seeded |
| 16 | Internal review | `#3576` | ADL maintainers | seeded |
| 17 | External / 3rd-party review | `#3580` | ADL maintainers | seeded |
| 18 | Review findings remediation + final v0.92 preflight | `#3577`, consuming `#3502`, `#3534`, `#3377` | ADL maintainers | seeded |
| 19 | Next milestone planning | `#3581` | ADL maintainers | seeded |
| 20 | Release ceremony | `#3578` | ADL maintainers | seeded |

## Execution Policy

- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Keep changes scoped per issue; use draft PRs until checks pass.
- Run the smallest meaningful validation for each touched surface.
- Record proof truthfully in issue-local output records or review docs.

## Cadence Expectations

- Preserve ordered execution where dependencies matter.
- Do not close WP-01 until planned issues, cards, sprint umbrellas, and initial
  sequencing are ready to begin or explicitly routed.
- Downstream card rewrite is explicitly routed through `#3582` after the
  prompt-template renderer, schema guard, and field-level editor land. Its
  audit records that all downstream v0.91.5 cards validate phase-aware, so
  Sprint 1 can proceed without mass Markdown rewrites.
- Sprint umbrellas `#3571` through `#3574` own sprint-level coordination and
  closeout truth; they do not replace child issue execution.
- Use multi-agent lanes only where they are being tested or provide clear value.
- Do not merge hidden prompt-record cleanup into unrelated issues.
- Escalate blockers as findings or follow-on issues.

## Risks / Dependencies

- Dependency: v0.91.4 release closeout.
- Risk: multi-agent overhead may outweigh benefit on small tasks.
- Mitigation: compare single-agent and multi-agent overhead explicitly.
- Risk: provider/model breadth may produce brittle test results.
- Mitigation: record provider identity, model identity, and role aptitude.

## Demo / Review Plan

- Demo artifact: [DEMO_MATRIX_v0.91.5.md](DEMO_MATRIX_v0.91.5.md)
- Review date: pending Sprint 4.
- Sign-off owners: ADL maintainers and reviewer lane.

## Closeout Bar

- All planned scope items are completed or explicitly deferred with rationale.
- Linked issues and PRs are updated and traceable.
- Focused validation is recorded for every touched surface.
- Sprint summary is captured in milestone docs.

## Exit Criteria

- All planned scope items completed or explicitly deferred with rationale.
- Linked issues/PRs updated and traceable.
- CI is green for merged work, or exceptions are documented.
- Sprint summary captured in milestone docs.
