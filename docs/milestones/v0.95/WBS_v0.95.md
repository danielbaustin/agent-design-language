# v0.95 Candidate Work Breakdown Structure

## Status

Candidate allocation only. `v0.95` has no final issue wave yet.

## WBS Summary

`v0.95` should be the MVP convergence and feature-freeze milestone. It should
close the remaining user-facing, evaluator-facing, editor-facing, and
control-plane-facing gaps without silently admitting a new architectural band.

## Candidate Work Areas

| Candidate | Work Area | Description | Primary deliverable | Key dependencies |
| --- | --- | --- | --- | --- |
| A | HTML milestone dashboard and compression reporting | Converge milestone dashboard and compression surfaces into one maintained operator/reviewer entrypoint. | HTML dashboard/reporting package and refresh rules. | `v0.90` milestone compression baseline, `v0.95` milestone docs. |
| B | Shepherd/Gemma training and evaluator path | Finish the MVP-facing training/evaluation lane for the Shepherd/Gemma line. | Training/evaluation plan, artifact expectations, and proof posture. | `v0.91.1` ANRM/Gemma placement, later evaluation surfaces. |
| C | Aptitude Atlas product lane | Turn the harness/evaluation groundwork into a coherent model-evaluation platform story. | Product-facing evaluation contract and reporting boundary. | `v0.90.1` Aptitude Atlas planning, `v0.91.1` capability harness, `v0.91.2` productization/evaluation work. |
| D | Distributed execution integration | Close the bounded distributed-execution story without sacrificing deterministic reviewability. | Integration contract and reviewable placement boundary. | ADR 0003, remote/cluster groundwork, `v0.94` secure-execution boundary. |
| E | Demo catalog and MVP walkthrough | Turn the landed demo families into one reviewer/customer-facing catalog and walkthrough. | Catalog, classification rules, and flagship walkthrough package. | Earlier milestone demo matrices, productization/publication surfaces. |
| F | Control-plane Rust migration and tooling hardening | Finish the highest-value migration/hardening tranche for the MVP control plane. | Hardened control-plane package and residual-language boundary. | Python-elimination staged plan, workflow guardrails, review tooling. |
| G | Web-based code editor integration | Establish the minimum required web editor/operator baseline for the MVP. | Web editor integration contract and lifecycle coupling rules. | HTA editor planning, control-plane lifecycle, `v0.95` convergence package. |
| H | Zed decision boundary | Decide whether Zed ships as part of MVP or remains explicitly outside the must-have set. | Decision record and success/defer/drop criteria. | Web editor baseline, MVP boundary planning, control-plane workflow truth. |
| I | MVP convergence packet | Integrate the milestone rows above into one explicit MVP boundary and launch-shape story. | Convergence packet tying demos, tooling, editor, and evaluation surfaces together. | A through H. |
| J | Demo matrix and proof program | Define the bounded demos that prove MVP convergence rather than isolated feature fragments. | Demo matrix and candidate proof surfaces. | D, E, G, I. |
| K | Quality gate and release readiness | Define the v0.95 quality/readiness evidence needed before ceremony. | Checklist, quality gate, and release-readiness plan. | I and J. |
| L | Docs, review, and release tail | Run the final alignment, review, and ceremony package for MVP closeout. | Review package, release notes, and ceremony evidence. | All prior work. |

## Sequencing Pressure

1. Start with the non-user-facing convergence surfaces: dashboard/reporting,
   evaluator/training, and distributed execution.
2. Land the walkthrough/catalog and control-plane hardening before making the
   editor decision final.
3. Treat the web editor as the required baseline and Zed as an explicit
   decision boundary rather than an implicit default.
4. Build the final MVP convergence packet before the demo/review/release tail.

## Acceptance Mapping

- Every `v0.95` feature row in the feature list maps to a candidate work area
  in this WBS.
- Dashboard/reporting, evaluator/training, and evaluation-platform work are
  explicit rather than buried in backlog language.
- Distributed execution remains bounded and subordinate to secure-execution
  truth from `v0.94`.
- Demo/walkthrough work proves the integrated MVP story rather than only
  isolated feature claims.
- The editor story remains explicit across required web baseline and optional
  Zed decision boundary.
- The milestone remains an MVP convergence band, not a new greenfield platform
  expansion.
