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
does not claim runnable demo completion from planning docs alone; it uses reviewable docs, validation checks, and issue-local proofs to assign and verify runtime and Observatory evidence.

## Scope

In scope:

- docs existence and cross-link proof;
- residual bridge-surface classification;
- follow-on validation/proof assignments;
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
| D4 | Runtime Soak #2 assignment | Runtime proof is scheduled, not implied | inspect `RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md` and linked sprint/WBS/source-capture docs | runtime assignment rows and feature-list matrix | Soak #2 has a tracked owner packet, feature-list proof modes per row, and blocker/non-claim policy tied to `v0.92` activation | issue-local proof later | ready |
| D5 | Observatory/birthday-visible proof status | Visible demo surfaces are proven or explicitly non-claimed without overclaiming | Unity-MCP proof for `#4652` and `#4704`; inspect demo matrix and Observatory proof status | `docs/milestones/v0.91.7/review/unity_observatory_4652/4652-unity-shell-proof-summary.md`; `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-unity-mcp-proof-summary.md`; retained camera renders | #4652 proves the flagship shell/runtime-polis surface with investor lighting and runtime contract refs; #4704 proves project binding, scene loading, runtime/polis objects, and retained nonblank visual evidence. Full build-player and final parent-sprint closeout remain non-claimed until #4702 reconciles the wave. | live Unity-MCP proof plus retained images; no player build claimed | proven-limited |

## Known Limits

- This matrix records documentation readiness and links to issue-local runtime/demo proof where that proof exists; rows without linked issue-local proof still do not claim runtime behavior.
- Curiosity, Constructability, ACIP, security, and reasoning-graph proofs require
  issue-local evidence or evidence-backed blockers before `v0.92` can consume them.
