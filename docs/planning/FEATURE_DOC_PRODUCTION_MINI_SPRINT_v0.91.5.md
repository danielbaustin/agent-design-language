# Feature-Doc Production Mini-Sprint Setup

## Status

Setup packet for issue `#3779`.

This is a planning mini-sprint. It creates the reviewable issue wave and gates
for feature-doc production, but it does not implement feature docs, runtime
features, ADRs, or milestone activation work.

## Purpose

Make the remaining feature-documentation work durable enough that ADL can move
from the current `v0.91.5` bridge state toward `v0.95` MVP readiness without
depending on chat-memory-only planning.

The mini-sprint turns local authoring notes into a tracked issue wave:

- pre-`v0.92` bridge feature docs
- `v0.92` activation and birthday refresh docs
- `v0.93` through `v0.95` MVP feature docs
- follow-on `v0.91.5` ADR mini-sprint

## Source Evidence

Tracked sources:

- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md`
- `docs/milestones/v0.91.5/README.md`
- `docs/milestones/v0.91.5/WBS_v0.91.5.md`
- `docs/milestones/v0.91.5/ADR_PLAN_v0.91.5.md`

Local authoring sources used for setup:

- `MVP_FEATURE_DOC_PRODUCTION_PLAN_2026-06-15.md`
- `FEATURE_DOC_ISSUE_SPLIT_PLAN_2026-06-15.md`
- `MVP_SCOPE_LOCK_CROSSCHECK_2026-06-15.md`

The local authoring sources remain in the local planning workspace and are not
promoted by this setup issue.

## Issue Wave

| Order | Issue | Role | Dependency | Exit condition |
|---:|---|---|---|---|
| 1 | `#3779` Feature-doc production wave setup | Parent setup issue for this mini-sprint. Owns the issue wave, review gates, feature-list alignment, and stop rules. | Current logging/tooling sequence must remain respected. | This setup packet is tracked, child issue routing is visible, and no child work begins from the parent. |
| 2 | `#3778` Pre-v0.92 bridge feature-doc production | Produces required bridge docs and a bridge ledger for `v0.91.6` / `v0.91.7`. | Depends on `#3779`. | Every named pre-`v0.92` activation surface is complete, deferred, blocked, or routed with evidence. |
| 3 | `#3780` v0.92 activation and birthday feature-doc refresh | Refreshes activation, birthday, identity, continuity, Memory Palace, ACP, ACIP, Observatory/Unity, and demo/review docs. | Depends on `#3779` and completion or explicit routing of `#3778`. | `v0.92` WP-01 can open concrete activation work without reconstructing requirements from local notes. |
| 4 | `#3781` v0.93-v0.95 MVP feature-doc production | Produces or splits governance/security/guilds, secure execution, temporal self-projection, CodeFriend v1, Rust refactoring, demo, cleanup, MVP, and post-MVP disposition docs. | Depends on `#3779`; consumes `#3778` and `#3780` where sequencing affects MVP scope. | `v0.95` remains convergence and packaging, not the first implementation home for major cognitive/product surfaces. |
| 5 | `#3782` v0.91.5 ADR mini-sprint after feature-doc production | Docs-only ADR promotion/drafting sprint. | Runs after feature-doc production source truth exists. | ADR candidates are source-grounded, accepted/deferred/split/blocked truthfully, and no feature implementation is hidden in ADR prose. |

## Required Pre-v0.92 Bridge Surfaces

The pre-`v0.92` bridge work is not optional cleanup. `#3778` must account for:

- AEE completion
- Memory/ObsMem handoff
- ACP/cognitive profiles
- provider/model matrix and multi-agent readiness
- Observatory/Unity readiness
- logging/tooling proof-loop fixes
- ACIP/A2A/provider communications, including access rules and protobuf/JSON
  projection decisions
- public prompt records export, redaction, validation, and indexing
- security bridge readiness and Continuous Adversarial Verification
- resilience, citizen persistence, and sleep/wake behavior
- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graph, loop runtime, and `adl.skill.v1` bridge

`v0.91.6` should carry the first bridge tranche. `v0.91.7` is the planned
second bridge tranche if `v0.91.6` cannot truthfully absorb all required
pre-`v0.92` work.

## MVP Scope Rules

- CodeFriend v1 is before `v0.95`; portable adapter v2 is required for
  CodeFriend because it must support cloning an arbitrary repository,
  installing the adapter, and running CodeFriend / C-SDLC tools on it through
  ADL.
- Guilds are MVP scope and should route through `v0.93` governance/security
  feature docs with `v0.95` MVP consumption.
- Aptitude Atlas productization is post-`v0.95`; `v0.95` consumes
  capability-testing evidence only.
- Memory Palace is intended for implementation, but the detailed context
  problem note remains under development. `#3780` should represent it
  truthfully as a planned bridge surface and avoid overstating maturity.
- Rust refactoring belongs late before Sprint 4 / `v0.95` convergence and must
  reduce change-specific test burden, not merely split files into parts.
- Security is not a sidebar. It must appear in the pre-`v0.92` bridge, `v0.93`
  governance/security docs, and `v0.94` secure-execution/trust docs.

## Review Gates

- Parent issue `#3779` stops after setup, review, and publication of this
  mini-sprint packet.
- Child issues do not start until the parent setup is reviewed.
- `#3778` blocks `#3780` unless every pre-`v0.92` surface is complete,
  deferred, blocked, or explicitly routed.
- `#3782` runs after feature-doc production, not before.
- No issue in this wave may claim `v0.92` readiness without bridge-ledger
  evidence.
- No deletion or local planning-workspace file move happens in this
  mini-sprint.

## Validation Plan

For `#3779` setup:

- `git diff --check`
- direct scan of `docs/planning/FEATURE_DOC_PRODUCTION_MINI_SPRINT_v0.91.5.md`
  while it is still untracked, before relying on diff-only checks
- added-line scan for host-local paths, obvious secret markers, and newly
  introduced local-planning-workspace links in tracked public docs
- issue-wave spot check against the local feature-doc production plan
- bounded review focused on sequencing truth, missing required feature
  surfaces, and accidental scope expansion

Child issues should add their own focused validation plans when they execute.

## Non-Goals

- Do not implement feature docs in `#3779`.
- Do not implement runtime behavior.
- Do not approve ADRs.
- Do not move or delete local TBD files.
- Do not close `#3778`, `#3780`, `#3781`, or `#3782` from the parent setup.

## Setup Verdict

The mini-sprint is ready to review once this setup packet and the feature-list
alignment are published through the normal C-SDLC PR path.
