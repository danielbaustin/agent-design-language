# #5007 v0.91.8 WP-21 Preparation Design

Preparation only for the Memory Palace ADR acceptance follow-on. This issue prepares the exact design-time handoff for a later execution session; it does not draft, accept, publish, merge, or close the ADR.

## Execution Gate

#5007 execution remains blocked until #4760 is closed with actual completed Memory Palace implementation proof. The proof must show an integrated pre-v0.92 handoff path, continuity semantics, storage/retrieval boundaries, negative or boundary behavior, retained runtime or review evidence, and the Memory Palace / ObsMem / Chronosense boundary. Typed closeout receipts and claim reconciliation may be completed later and are not preparation blockers, but they cannot substitute for the implementation proof.

## Source Dependencies

- #4760: Memory Palace context handoff implementation/proof, currently open at preparation time.
- #4765: Chronosense integration, closed; execution must consume the landed proof, not the issue state alone.
- #4768: temporal query index for ObsMem and Memory Palace, closed; execution must verify retained temporal-index evidence.
- #4771: long-running context continuity proof, closed; execution must map only supported continuity claims.
- ADR 0051: deferred Chronosense and Memory Palace disposition.
- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`
- `docs/milestones/v0.91.8/WBS_v0.91.8.md`
- `docs/milestones/v0.91.8/ADR_PLAN_v0.91.8.md`
- `docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md`
- `docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md`

## Intended Execution Paths

- Candidate ADR: `docs/adr/0058-memory-palace-context-handoff-architecture.md`, unless a newer ADR number exists at execution time.
- ADR index update: `docs/adr/README.md`.
- ADR planning update: `docs/milestones/v0.91.8/ADR_PLAN_v0.91.8.md`.
- Handoff references, only if proof supports them: `docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md` and `docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md`.
- Optional issue-local proof-to-claim table, if the ADR does not embed the full table: `.csdlc/evidence/5007/proof-to-claim-table.md`.

## COTS And Boundary

This preparation has no new runtime or product COTS. Later ADR execution is documentation-only unless #4760 proof exposes a source-link/index update need; allowed dependencies are the repository Markdown/ADR conventions, Git/GitHub issue metadata, Mermaid for the design diagram, and typed C-SDLC v2 lifecycle tooling. It must not add crates, hosted services, AWS, providers, databases, or runtime code.

## Budgets

Preparation budget: up to 650 changed lines across issue-local cards, design, diagram, review, and validation evidence; up to 45 minutes local elapsed time; up to 15 minutes validation time. Later ADR execution budget: target <= 350 documentation LoC and <= 90 minutes elapsed time after #4760 proof is available. Any code change, new COTS dependency, runtime proof rerun, or accepted ADR claim without retained implementation proof requires replan/review before execution continues.

## Rollback And No-Deferral

Rollback for preparation is to revert the #5007 preparation commit on `codex/5007-v0918-wp14-preparation`. Later execution rollback is documentation-only revert of the candidate ADR/index/handoff paths. No acceptance criterion may be deferred in execution: missing #4760 proof, missing proof-to-claim mapping, ambiguous Memory Palace/ObsMem/Chronosense boundary, stale main integration, stale review truth, or unresolved actionable review findings must block rather than become residual acceptance.

## Review

The bounded preparation review is retained at `.csdlc/evidence/5007/preparation/gpt-5.5-preparation-review.md`; it reviews the preparation packet only and does not authorize ADR acceptance or implementation.
