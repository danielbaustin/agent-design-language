# v0.91.7 Demo Matrix

## Status

planned

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-16`
- Owner: ADL maintainers
- Related issue: `#3825`

## Purpose

Define proof surfaces for the second pre-`v0.92` bridge tranche. `v0.91.7`
has no required runnable demo at planning-package time; it uses reviewable docs,
validation checks, and later issue-local proofs.

## Scope

In scope:

- docs existence and cross-link proof;
- residual bridge-surface classification;
- follow-on validation/proof routes;
- non-claim boundaries for runtime behavior.

Out of scope:

- birthday demo execution;
- Curiosity runtime proof;
- Constructability validator implementation;
- protocol implementation.

## Runtime Preconditions

Working directory:

```bash
git rev-parse --show-toplevel
```

No provider credentials or runtime services are required for this docs tranche.

## Related Docs

- Design contract: `DESIGN_v0.91.7.md`
- WBS: `WBS_v0.91.7.md`
- Sprint plan: `SPRINT_PLAN_v0.91.7.md`
- Checklist: `MILESTONE_CHECKLIST_v0.91.7.md`
- Feature index: `FEATURE_DOCS_v0.91.7.md`

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| D1 | Documentation package proof | `#3825` docs package exists and links truthfully | `find docs/milestones/v0.91.7 -maxdepth 2 -type f` | tracked docs | Expected planning and feature docs are present | deterministic filesystem check | planned |
| D2 | Bridge overclaim scan | Docs do not claim runtime or `v0.92` readiness | text scan over `docs/milestones/v0.91.7` | review notes | claims are bounded by non-goals and consumption rules | deterministic text review | planned |
| D3 | Residual visibility proof | Every second-tranche surface remains distinct | text scan over feature index and docs | index and feature docs | all eight surfaces are visible | deterministic text review | planned |

## Known Limits

- This matrix proves documentation readiness, not runtime behavior.
- Curiosity, Constructability, ACIP, security, and reasoning-graph proofs remain
  future issue work unless explicitly completed later.
