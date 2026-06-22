# v0.91.6 Demo Matrix

## Status

partially_proven_with_downstream_runtime_residuals

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Owner: ADL maintainers
- Related issue: `#3824`

## Purpose

Define proof surfaces for the first pre-`v0.92` bridge tranche. `v0.91.6`
started as a docs-planning matrix, but later issue-local proofs now provide
bounded Observatory/Unity evidence while downstream runtime, demo-convergence,
and release-tail lanes stay explicitly open.

## Scope

In scope:

- docs existence and cross-link proof;
- bridge-surface classification;
- follow-on validation/proof routes;
- bounded WP-09 Observatory/Unity proof surfaces;
- non-claim boundaries for runtime behavior.

Out of scope:

- birthday demo execution;
- final Unity/Observatory runtime rehearsal;
- provider benchmark reruns;
- public prompt export execution.

## Runtime Preconditions

Working directory:

```bash
git rev-parse --show-toplevel
```

No provider credentials or runtime services are required for the bounded proof
surfaces recorded here.

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
| D4 | Unity Observatory bounded closeout proof | WP-09 closeout packet and classification surfaces are refreshed to the current bounded closeout posture and justify umbrella closeout without overclaiming production readiness | `rg "#4030|#4031|#4032|#4033|#4034|#4035|#4341|#3974" docs/milestones/v0.91.6/review/observatory/WP09_WORKING_UNITY_OBSERVATORY_CLOSEOUT_4035.md docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md` | `docs/milestones/v0.91.6/review/observatory/WP09_WORKING_UNITY_OBSERVATORY_CLOSEOUT_4035.md` | closeout packet and classification surfaces preserve the closed-child/downstream-open split and retain explicit WP-09 ownership boundaries | deterministic doc-truth review | proved |
| D5 | Portable governed Observatory proof | portable reviewer-facing Observatory surface exists as a landed bounded proof surface while richer downstream runtime/release-tail convergence remains separate | `bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh` | `demos/v0.90.4/csm_observatory_governed_prototype.html` | governed Observatory prototype loads and remains available as a bounded reviewer-facing surface with the mobile-capable proof lane landed in `#4341` | deterministic local demo smoke | proved |

## Coverage Rules

- Runnable demos are not required for this package.
- Each future implementation issue must define its own proof surface.
- Substitute proof is acceptable only when docs state the non-runtime boundary.
- Closed child proof does not by itself prove umbrella closure.

## Known Limits

- This matrix proves documentation readiness, not runtime behavior.
- Provider/model, ACIP, and public export proofs remain future or separate issue
  work unless explicitly completed later.
- WP-09 child proof is closed, but this matrix still does not prove broader
  runtime integration, demo convergence, or release-tail readiness.
