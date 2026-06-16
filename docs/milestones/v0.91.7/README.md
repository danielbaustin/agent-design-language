# v0.91.7 Milestone README

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-16`
- Owner: ADL maintainers
- Setup issue: `#3801`
- Source bridge ledger: `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`
- First-tranche input: `docs/milestones/v0.91.6/`

## Status

Current status: candidate planning package for the second required pre-`v0.92`
bridge tranche.

- Planning: created by `#3801`
- Execution: not started
- Validation: docs-readiness validation only
- Release readiness: not applicable until `v0.91.7` executes

This package does not implement runtime features and does not claim `v0.92`
activation readiness. It exists so second-tranche bridge work is planned before
`v0.92` opens.

## Purpose

`v0.91.7` is the second bridge/readiness tranche before `v0.92`.

It should turn the remaining major bridge ideas into reviewable feature docs,
decision records, and issue routes:

- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graph, loop runtime, and `adl.skill.v1`
- residual security readiness
- residual ACIP/A2A/protobuf/JSON projection decisions
- affect/happiness, Godel mechanics, and economics-context accounting

`v0.91.7` is not vague spillover. It is the place where the big pre-`v0.92`
conceptual surfaces become issue-ready without stealing the `v0.92` birthday
implementation.

## Bridge Boundary

`v0.91.7` consumes:

- the `#3778` pre-`v0.92` bridge ledger
- the `#3800` `v0.91.6` first-tranche planning package
- residuals explicitly left by `v0.91.6`

A second-tranche surface may exit only as one of:

- `complete`: reviewed feature doc and proof/review evidence exist
- `deferred`: explicitly not required for `v0.92`, with risk accepted
- `blocked`: named missing evidence or operator decision prevents completion
- `routed`: owned by a named follow-on issue/tranche with a clear exit condition

## Second-Tranche Feature Docs

| Surface | Required v0.91.7 output | v0.92 consumption rule |
| --- | --- | --- |
| Curiosity Engine / Discovery Substrate | Feature doc covering curiosity artifacts, detection hooks, hypotheses, experiment plans, discovery budget, governance, Freedom Gate integration, ObsMem/reasoning-graph update, and proof. | At least one governed discovery-cycle proof is required before `v0.92` consumes it; otherwise it must be blocked/deferred/routed. |
| Constructability Gate | Feature doc covering construction-event schema, external-anchor schema, admissibility validator, shared-reality boundary, and proof path. | Must distinguish provisional cognition from authoritative shared reality. |
| Reasoning graph, loop runtime, and `adl.skill.v1` | Feature doc defining the pre-`v0.92` bridge among prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1`. | Deeper convergence remains later, but the pre-`v0.92` bridge must be explicit. |
| Residual security readiness | Addendum or feature doc for any security/CAV residuals left by `v0.91.6`. | Security cannot be silently deferred out of activation. |
| Residual ACIP/A2A/protobuf decisions | Decision record for remaining protobuf/JSON/WebSocket/access-rule issues. | `v0.92` must know whether it consumes JSON projection, protobuf, mock carrier, or a deferred route. |
| Affect/happiness surfaces | Bridge record defining safe tests, non-claims, and public-evidence limits for affect, humor, happiness, and wellbeing. | Public birthday evidence must not imply unproved affect or wellbeing claims. |
| Godel mechanics | Bridge record mapping experiment, hypothesis, mutation, evaluation, promotion, and proof boundaries. | `v0.92` must consume a reviewed mechanics map or mark the surface blocked/deferred. |
| Economics context | Bridge record deciding whether economics is context-only or requires explicit activation tests. | Economics must not dominate `v0.92` without explicit proof decision. |

## v0.92 Activation Gate

`v0.92` remains blocked until every surface named in the `#3778` bridge ledger
is complete, deferred, blocked, or routed with evidence.

This package should make that gate easier to audit. It should not weaken the
gate by claiming narrative readiness.

## Source Map

- `#3778`: pre-`v0.92` bridge ledger and issue route
- `#3800`: `v0.91.6` first-tranche planning package
- `#3801`: this `v0.91.7` second-tranche planning package
- `#3780`: later `v0.92` activation and birthday refresh
- `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`
- `docs/milestones/v0.91.6/`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/FEATURE_DOC_PRODUCTION_MINI_SPRINT_v0.91.5.md`

## Document Map

- Work breakdown: [WBS_v0.91.7.md](WBS_v0.91.7.md)
- Feature-doc index: [FEATURE_DOCS_v0.91.7.md](FEATURE_DOCS_v0.91.7.md)
- Candidate issue wave: [WP_ISSUE_WAVE_v0.91.7.yaml](WP_ISSUE_WAVE_v0.91.7.yaml)
- Checklist: [MILESTONE_CHECKLIST_v0.91.7.md](MILESTONE_CHECKLIST_v0.91.7.md)
- Review and validation checklist:
  [REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md](REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md)
- Feature directory index: [features/README.md](features/README.md)

## Non-Goals

- Do not implement runtime features in the planning package.
- Do not claim `v0.92` activation readiness.
- Do not replace the `v0.92` birthday milestone.
- Do not move or delete local planning files.
- Do not collapse Curiosity, Constructability, reasoning graphs, security, or
  ACIP/A2A into generic follow-up language.

## Exit Criteria

- Every second-tranche surface has a feature-doc route, issue-wave route, and
  review gate.
- Residual first-tranche dependencies from `v0.91.6` are visible.
- `#3780` can refresh `v0.92` activation docs without reconstructing
  second-tranche scope from chat.
