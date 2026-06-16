# v0.91.6 Demo Matrix

## Status

planned

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Owner: ADL maintainers
- Related issue: `#3824`

## Purpose

Define proof surfaces for the first pre-`v0.92` bridge tranche. `v0.91.6` has
no required runnable demo at planning-package time; it uses reviewable docs,
validation checks, and later issue-local proofs.

## Scope

In scope:

- docs existence and cross-link proof;
- bridge-surface classification;
- follow-on validation/proof routes;
- non-claim boundaries for runtime behavior.

Out of scope:

- birthday demo execution;
- Unity/Observatory runtime rehearsal;
- provider benchmark reruns;
- public prompt export execution.

## Runtime Preconditions

Working directory:

```bash
git rev-parse --show-toplevel
```

No provider credentials or runtime services are required for this docs tranche.

## Related Docs

- Design contract: `DESIGN_v0.91.6.md`
- WBS: `WBS_v0.91.6.md`
- Sprint plan: `SPRINT_PLAN_v0.91.6.md`
- Checklist: `MILESTONE_CHECKLIST_v0.91.6.md`
- Feature index: `FEATURE_DOCS_v0.91.6.md`

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| D1 | Documentation package proof | `#3824` docs package exists and links truthfully | `find docs/milestones/v0.91.6 -maxdepth 2 -type f` | tracked docs | Expected planning and feature docs are present | deterministic filesystem check | planned |
| D2 | Bridge overclaim scan | Docs do not claim runtime or `v0.92` readiness | `rg "ready|complete|shipped" docs/milestones/v0.91.6` | review notes | claims are bounded by non-goals and consumption rules | deterministic text review | planned |
| D3 | Residual routing proof | `v0.91.7` residuals remain explicit | `rg "v0.91.7|#3825|residual" docs/milestones/v0.91.6` | index and feature docs | residual routes are visible | deterministic text review | planned |

## Coverage Rules

- Runnable demos are not required for this package.
- Each future implementation issue must define its own proof surface.
- Substitute proof is acceptable only when docs state the non-runtime boundary.

## Known Limits

- This matrix proves documentation readiness, not runtime behavior.
- Provider/model, ACIP, public export, Observatory/Unity, and security proofs
  remain future issue work unless explicitly completed later.
