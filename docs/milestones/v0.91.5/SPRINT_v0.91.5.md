# v0.91.5 Sprint Plan

## Metadata
- Sprint: `v0.91.5`
- Milestone: `v0.91.5`
- Start date: `pending`
- End date: `pending`
- Owner: ADL maintainers
- Status: `draft_pre_open`

## Status

`draft_pre_open`

## How To Use

- List bridge work in planned execution order.
- Track blockers here rather than scattered chat notes.
- Keep public prompt, demo, and provider side work visible and bounded.
- Record closeout expectations before execution begins.

## Sprint Overview

v0.91.5 is split into four sprint bands.

Planned scope:

- public prompt records and `.adl` transition controls;
- provider/model matrix and multi-agent proof;
- demo readiness and v0.92 activation preflight;
- review, remediation, final v0.92 preflight, and release.

## Bridge Sprint Work

- Scope: pre-v0.92 operational bridge work.
- Boundary: no first-birthday implementation.
- Proof surface: issue wave, moved labels, prompt packets, provider matrix,
  multi-agent proof, demo index, activation map, and `#3377`.

## Sprint Goals

- Sprint 1: route issues and make public prompt records reviewable.
- Sprint 2: make provider/model breadth and multi-agent execution work.
- Sprint 3: prepare demos and v0.92 activation readiness.
- Sprint 4: review, remediate, preflight v0.92, and close the bridge.

## Sprint Goal

Make v0.92 openable without hidden operational debt.

## Planned Scope

- Public prompt packet export, redaction, and archive planning.
- OpenRouter and model-role matrix.
- Multi-agent C-SDLC workcell proof.
- Demo showcase and Unity Observatory readiness.
- v0.92 activation map and first-birthday launch packet.

## Work Plan

| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | Bridge planning and migration | `#3506` | ADL maintainers | draft |
| 2 | Public prompt records | `#3472`-`#3476` | ADL maintainers | moved |
| 3 | Provider/model matrix | `#3501`, `#3505` | ADL maintainers | moved |
| 4 | Multi-agent proof | `#3415`, `#3484`, `#3503`, `#3504` | ADL maintainers | moved |
| 5 | Demo readiness | `#3455`, `#3460`, `#3461` | ADL maintainers | moved |
| 6 | v0.92 activation and birthday preflight | `#3502`, `#3377` | ADL maintainers | moved |
| 7 | Review, remediation, release | pending | ADL maintainers | planned |

## Execution Policy

- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Keep changes scoped per issue; use draft PRs until checks pass.
- Run the smallest meaningful validation for each touched surface.
- Record proof truthfully in issue-local output records or review docs.

## Cadence Expectations

- Preserve ordered execution where dependencies matter.
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
