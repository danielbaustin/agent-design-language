# Work Breakdown Structure - v0.89

## Metadata
- Milestone: `v0.89`
- Version: `v0.89`
- Date: `2026-04-12`
- Owner: `Daniel Austin`

## How To Use
- Break work into independently mergeable issues.
- Keep each item measurable and testable.
- `WP-01` owns the canonical milestone package and issue-wave seeding, not just templates.
- Reserve the final WPs for demos, quality, docs/review convergence, and release ceremony.

## WBS Summary

`v0.89` is organized as a governed-adaptation milestone:
- first establish the canonical milestone package and issue wave
- then land the core convergence / gate / action / skill / experiment / memory / security band
- then package demos, quality, docs/review, and release closure

The adversarial runtime/proof package is intentionally planned as the `v0.89.2` carry-forward band rather than silently swelling the main `v0.89` milestone.

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | Finalize the canonical `v0.89` package, promote the main feature docs, and map every source planning doc to an implementation home. | coherent milestone docs, feature index, seeded issue-wave plan | none | `#1662` |
| WP-02 | AEE convergence | Turn AEE 1.0 convergence into an explicit runtime/trace/review contract with bounded stop conditions and progress signals. | code/doc/proof surface for AEE convergence | `WP-01` | planned issue wave |
| WP-03 | Freedom Gate v2 | Strengthen the gate from minimal bounded refusal into a richer judgment boundary. | gate design + implementation/proof surface | `WP-01`, `WP-02` | planned issue wave |
| WP-04 | Decision surfaces and decision schema | Make points of choice and their record shape explicit and reusable across the runtime. | decision-surface contract + decision record support | `WP-01`, `WP-03` | planned issue wave |
| WP-05 | Action mediation and proposal schema | Establish the authority boundary between model intent and runtime execution. | action proposal + mediation surfaces | `WP-01`, `WP-03`, `WP-04` | planned issue wave |
| WP-06 | Skill model and execution protocol | Promote skills from repo practice into canonical governed execution contracts. | skill model + invocation protocol surfaces | `WP-01`, `WP-05` | planned issue wave |
| WP-07 | Godel experiment system | Deepen bounded scientific-loop behavior into explicit experiments and adopt/reject records. | experiment record and evaluation package | `WP-02`, `WP-04`, `WP-06` | planned issue wave |
| WP-08 | ObsMem evidence and ranking | Make retrieval evidence-aware, provenance-sensitive, and reviewer-legible. | ranking/explanation package | `WP-02`, `WP-07` | planned issue wave |
| WP-09 | Security, trust, and posture package | Land the main `v0.89` security contract: threat model, posture model, and trust-under-adversary framing. | security/trust/posture package | `WP-04`, `WP-05`, `WP-06` | planned issue wave |
| WP-10 | `v0.89.2` handoff planning | Convert adversarial-runtime carry-forward into an explicit `v0.89.2` package with no ambiguity. | follow-on planning package for `v0.89.2` | `WP-01`, `WP-09` | planned issue wave |
| WP-11 | Demo scaffolding and proof entry points | Define and land the bounded demo entry points for convergence, gate behavior, experiment evidence, and security review surfaces. | runnable or reviewer-legible demo surfaces | `WP-02` - `WP-09` | planned issue wave |
| WP-12 | Milestone convergence and follow-on mapping | Reconcile issue graph, carry-forward, and proof surfaces before the release tail starts. | converged issue graph and milestone status surfaces | `WP-02` - `WP-11` | planned issue wave |
| WP-13 | Demo matrix + integration demos | Validate the milestone claims through bounded demos and integration review. | canonical demo matrix and demo artifacts | `WP-02` - `WP-12` | planned issue wave |
| WP-14 | Coverage / quality gate (ratchet + exclusions) | Run quality gates and record any bounded exceptions truthfully. | green quality gate or documented exceptions | `WP-02` - `WP-13` | planned issue wave |
| WP-15 | Docs + review pass (repo-wide alignment) | Align docs, review surfaces, and release-tail truth across the repo. | converged docs/review package | `WP-13`, `WP-14` | planned issue wave |
| WP-16 | Release ceremony (final validation + tag + notes + cleanup) | Close the milestone cleanly after validation and documentation are complete. | release tag, notes, and closeout | `WP-15` | planned issue wave |

## Sequencing
- Phase 1: establish the canonical package and seed the issue wave (`WP-01`)
- Phase 2: land the core feature band (`WP-02` - `WP-09`)
- Phase 3: package demos, quality, docs/review, and release closure (`WP-10` - `WP-16`)

## Acceptance Mapping
- WP-01 (Design pass) -> no template drift remains, every source planning doc has an explicit home, and the issue wave can seed directly from the package
- WP-02 -> AEE convergence states, progress signals, and bounded stop conditions are explicit enough to implement and review
- WP-03 -> Freedom Gate v2 is defined as a richer judgment surface rather than an aspirational note
- WP-04 -> decision points and decision records are explicit and consistent across docs
- WP-05 -> model intent is clearly separated from authorized action
- WP-06 -> skills have a canonical definition and invocation protocol
- WP-07 -> experiment records and governed adopt/reject behavior are explicit
- WP-08 -> ObsMem retrieval becomes evidence-aware and explainable
- WP-09 -> trust boundaries, posture, and threats are explicit enough to drive implementation and review
- WP-10 -> the `v0.89.2` adversarial package is clear rather than implicit
- WP-11 -> demo/proof entry points exist for the main milestone claims
- WP-12 -> the milestone package and issue graph are converged
- WP-13 (Demos) -> milestone claims have bounded proof surfaces
- WP-14 (Quality gate) -> quality/coverage posture is truthful and reviewable
- WP-15 (Docs/review) -> repo-wide docs and review surfaces are aligned with shipped truth
- WP-16 (Release ceremony) -> milestone closes with truthful notes, tag, and follow-on capture

## Exit Criteria
- every in-scope requirement maps to at least one WBS item
- every WBS item has a concrete deliverable and an issue-wave home
- dependency order is explicit enough to execute without reconstructing milestone logic by hand
