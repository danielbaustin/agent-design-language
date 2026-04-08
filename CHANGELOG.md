# Changelog

All notable project-level changes are summarized here by milestone/release.

## v0.87 (Sprint 3 Convergence In Progress)

Status: Docs/review/quality/release tail in progress.

Summary:
- ADL now has a real `v0.87` substrate milestone on `main`, centered on the canonical milestone spine:
  `contracts -> execution -> trace -> review -> documentation`
- The milestone’s promoted feature-doc set now covers trace, provider portability, shared ObsMem, operational skills, control-plane behavior, and reviewer-facing proof surfaces as canonical `v0.87` docs
- Canonical `v0.87` milestone docs now reflect the real Sprint 1 / Sprint 2 implementation sequence and the active Sprint 3 closeout issues
- The bounded `v0.87` demo and reviewer package exists through the demo matrix, runbook, and `demo_v087_suite.sh` entry surfaces
- The next-milestone planning shell for `v0.87.1` now exists as an explicit handoff target for runtime-completion work

References:
- `docs/milestones/v0.87/README.md`
- `docs/milestones/v0.87/WBS_v0.87.md`
- `docs/milestones/v0.87/SPRINT_v0.87.md`
- `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- `docs/milestones/v0.87/FEATURE_DOCS_v0.87.md`
- `docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md`
- `docs/milestones/v0.87/RELEASE_PLAN_v0.87.md`
- `docs/milestones/v0.87/RELEASE_NOTES_v0.87.md`

Not yet claimed in v0.87:
- final docs/review convergence and 3rd-party review closeout
- final release ceremony completion
- later-milestone continuity, chronosense, governance, signed-trace, or broader runtime-completion work that belongs to `v0.87.1+`

## v0.86 (Sprint 7 Closeout In Progress)

Status: Docs/review/release tail in progress.

Summary:
- ADL now has its first working bounded cognitive system on `main`, centered on one canonical bounded cognitive path:
  `signals -> candidate selection -> arbitration -> reasoning -> bounded execution -> evaluation -> reframing -> memory participation -> Freedom Gate`
- Canonical runtime artifacts now cover the bounded cognitive path and related proof surfaces, including:
  `signals.json`, `candidate_selection.json`, `arbitration.json`, `execution_iterations.json`, `evaluation.json`, `reframing.json`, `memory.json`, `freedom_gate.json`, and `final_result.json`
- Local demo and review surfaces exist for the integrated milestone proof set:
  D1 canonical bounded cognitive path, D2 fast/slow routing, D3 candidate selection, D4 Freedom Gate enforcement, and D5 review-surface walkthrough
- Sprint 7 quality-gate work landed with passing local `fmt`, `clippy`, `test`, coverage, and demo-validation proof
- Docs, release-tail surfaces, and reviewer entry points are being aligned so milestone truth matches implementation and proof artifacts

References:
- `docs/milestones/v0.86/README.md`
- `docs/milestones/v0.86/WBS_v0.86.md`
- `docs/milestones/v0.86/SPRINT_v0.86.md`
- `docs/milestones/v0.86/DEMO_MATRIX_v0.86.md`
- `docs/milestones/v0.86/MILESTONE_CHECKLIST_v0.86.md`
- `docs/milestones/v0.86/RELEASE_PLAN_v0.86.md`
- `docs/milestones/v0.86/RELEASE_NOTES_v0.86.md`

Not yet claimed in v0.86:
- final internal/external review completion and release ceremony closeout
- later-milestone persistence, identity, governance, signed-trace, or broader AEE convergence work
- anything beyond the bounded `v0.86` cognitive-system scope

## v0.85 (Planning And Tooling Foundation Milestone)

Status: historical bridge milestone.

Summary:
- Established the tracked milestone-planning and execution architecture that later `v0.86` work now relies on
- Landed the core milestone surfaces for `v0.85`, including design, WBS, sprint, checklist, release, and roadmap-tracking docs
- Defined the editing/control-plane model around structured prompts, issue/task bundles, and the `init/create/start/run/finish` lifecycle
- Strengthened quality/release discipline and issue reconciliation so later milestone work could be executed in smaller reviewable units
- Preserved and promoted major planning surfaces for cognition, affect, reasoning, Layer 8/provider work, and future convergence bands

References:
- `docs/milestones/v0.85/README.md`
- `docs/milestones/v0.85/DESIGN_v0.85.md`
- `docs/milestones/v0.85/WBS_v0.85.md`
- `docs/milestones/v0.85/SPRINT_v0.85.md`
- `docs/milestones/v0.85/MILESTONE_CHECKLIST_v0.85.md`
- `docs/milestones/v0.85/RELEASE_PLAN_v0.85.md`
- `docs/milestones/v0.85/RELEASE_NOTES_v0.85.md`
- `docs/milestones/v0.85/EDITING_ARCHITECTURE.md`

Not yet claimed in v0.85:
- the full bounded cognitive system that later lands in `v0.86`
- later milestone runtime identity, governance, or signed-trace behavior
- final productionization of the longer-horizon planning concepts documented under the `v0.85` milestone corpus

## v0.8 (Active Development Milestone)

Status: In progress.

Summary:
- Bounded Godel runtime and demo surfaces now exist on `main`, including the explicit seven-stage loop:
  `failure -> hypothesis -> mutation -> experiment -> evaluation -> record -> indexing`
- Canonical runtime artifacts for the Godel review loop are now emitted and validated, including:
  `mutation.v1`, `canonical_evidence_view.v1`, `evaluation_plan.v1`, and `experiment_record.v1`
- New user-facing CLI and demo surfaces were added for bounded Godel execution and inspection, alongside the v0.8 demo matrix
- New reviewer-facing demo runbooks under `demos/` cover the bounded Gödel CLI flow and bounded AEE recovery flow
- The Rust transpiler remains a bounded demo scaffold for deterministic fixture-to-runtime verification, not a production transpiler
- Major review-tail work landed to align milestone docs, schemas, and release-facing repository truth with current implementation

References:
- `docs/milestones/v0.8/RELEASE_PLAN_V0.8.md`
- `docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`
- `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`
- `docs/milestones/v0.8/GODEL_LOOP_INTEGRATION_V0.8.md`

Not yet claimed in v0.8:
- fully finished Adaptive Execution Engine behavior
- unconstrained self-modification or autonomous policy learning
- production graduation of the Rust transpiler demo

## v0.75 (Previous Milestone)

Status: prior milestone reference.

References:
- `docs/milestones/v0.75/RELEASE_PLAN_0.75.md`
- `docs/milestones/v0.75/RELEASE_NOTES_0.75.md`
- `docs/milestones/v0.75/MILESTONE_CHECKLIST_0.75.md`

## v0.7.0 (Released)

Status: Released (`v0.7.0`).

Summary:
- Foundation runtime hardening for deterministic, replayable execution.
- Security envelope and trust/signing surfaces integrated into core execution flows.
- Runtime identity migration to canonical `adl` naming with compatibility-window shims.

References:
- `docs/milestones/v0.7/RELEASE_NOTES_v0.7.md`
- `docs/milestones/v0.7/RELEASE_PLAN_v0.7.md`
