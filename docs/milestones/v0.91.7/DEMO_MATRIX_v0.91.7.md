# v0.91.7 Demo Matrix

## Status

planned

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Owner: ADL maintainers
- Setup lineage: `#3801`, `#3825`, `#4368`

## Purpose

Define proof surfaces for the final pre-`v0.92` bridge/readiness tranche. `v0.91.7`
does not claim runnable demo completion from planning docs; it uses reviewable docs, validation checks, and later issue-local proofs to route runtime and Observatory evidence.

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
| D4 | Runtime Soak #2 route | Runtime proof is scheduled, not implied | inspect `RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md` and linked sprint/WBS/source-capture docs | runtime route rows and feature-list matrix | Soak #2 has a tracked owner packet, feature-list proof modes per row, and blocker/defer policy tied to `v0.92` activation | issue-local proof later | ready |
| D5 | Observatory/birthday-visible proof status | Visible demo surfaces are proven or explicitly non-claimed without overclaiming | inspect demo matrix and Observatory proof status | planning docs and later demo artifacts | Observatory/Unity/HTML evidence is integrated/proven, explicitly non-claimed with operator approval, or blocked with evidence and operator approval | issue-local proof later | planned |

## Known Limits

- This matrix proves documentation readiness, not runtime behavior.
- Curiosity, Constructability, ACIP, security, and reasoning-graph proofs remain
  future issue work unless explicitly completed later.
