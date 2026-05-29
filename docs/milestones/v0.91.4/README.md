# v0.91.4 Milestone README

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-25`
- Owner: ADL maintainers
- Related issues: `#3210`, seeded v0.91.4 issue wave with Sprint 1 closeout recorded

## Status

Sprint 1 is complete and closed: WP-01 `#3346`, WP-02 `#3348`, WP-03 `#3349`,
and WP-04 `#3350` all merged, and umbrella `#3347` is closed with clean sprint
truth.

Sprint 2 is seeded as the transition-operation batch: Sprint 2 `#3352`,
WP-05 `#3353`, WP-06 `#3354`, WP-07 `#3355`, and WP-08 `#3356`.
Sprint 3 is seeded as the sprint-default and metrics batch: Sprint 3 `#3357`,
WP-09 `#3358`, WP-10 `#3359`, WP-11 `#3360`, and WP-12 `#3361`.
Sprint 4 is seeded as the review/remediation/planning/release batch:
Sprint 4 `#3362`, WP-13 `#3363`, WP-14 `#3364`, WP-15 `#3365`, WP-16
`#3366`, WP-17 `#3367`, WP-18 `#3368`, WP-19 `#3369`, WP-20 `#3370`, and
WP-21 `#3371`. The CodeFriend sidecar has also been seeded: umbrella `#3372`,
CF-PRE-01 `#3373`, CF-PRE-02 `#3374`, CF-PRE-03 `#3375`, and CF-PRE-04
`#3376`. The WildClawBench benchmark spike sidecar is seeded: umbrella
`#3378`, WC-PRE-01 `#3379`, WC-PRE-02 `#3380`, WC-PRE-03 `#3381`, and
WC-PRE-04 `#3382`. First-birthday readiness has been promoted as standalone
side issue `#3377` for v0.92 launch preparation. Sprint 2, Sprint 3, and
Sprint 4 child execution now depends on prior sprint sequencing and their own
dependency gates rather than on Sprint 1 opening. The CodeFriend sidecar
remains non-core and wait-gated by its own routing/dependency lane. The
WildClawBench sidecar has now started through WC-PRE-01 `#3379`, which
completed upstream reconnaissance and ended in a truthful blocked setup
baseline pending benchmark image, workspace payload, helper tools, and API-key
availability. First-birthday readiness does not alter the v0.91.4 release-tail
sequence and should feed WP-19/WP-20 next-milestone planning/review. Remaining
list-only side issues remain queued until explicitly promoted.

This package is intentionally stacked after the v0.91.3 first-slice package. It
assumes v0.91.3 proves one bounded Cognitive State Transition and then defines
the work needed to make C-SDLC repeatable enough to become ADL's normal
software-development path.

## Purpose

v0.91.4 completes the Cognitive SDLC rollout.

Where v0.91.3 proves the crown jewel, v0.91.4 makes it operational:

- stricter lifecycle validators
- reliable conductor and editor routing
- Software Development Polis roles and actor standing
- enforceable shard ownership and interface-freeze rules
- repeatable evidence convergence
- robust review, merge-readiness, closeout, and ObsMem handoff
- metrics for coordination latency and five-minute-sprint repeatability
- validation-tail handling so short sprints do not get swallowed by long
  blocking proof cycles
- Parallel Validation Fabric planning so validation can be shardable,
  cache-aware, and truthful about pending/deferred/blocking proof
- migration of future ADL software-development issues onto the C-SDLC default
- tracked durable workflow records for all C-SDLC truth
- minimal signed trace proof for durable C-SDLC runs

v0.91.4 also carries one bounded product sidecar: the CodeFriend pre-alpha
repo/S3 welcome-page setup mini-sprint described in
`docs/planning/codefriend/CODEFRIEND_PRE_ALPHA_REPO_AND_S3_WELCOME_MINI_SPRINT.md`.
That sidecar is part of v0.91.4 scheduling, but it is not C-SDLC core
machinery and must not be used as proof that C-SDLC default operation is
complete.

## Milestone Role

v0.91.4 should leave ADL with one default issue execution model:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

wrapped by Cognitive State Transition identity, evidence, review, merge gates,
and memory.

In that default model, `SPP` is the issue-local operative execution plan. It
records the current step, next step, required proof, stop/replan conditions, and
issue bounds. It is public/tracked C-SDLC truth, but it is not sprint
orchestration, review-result truth, or output truth.

The durable workflow namespace for default operation is
`docs/milestones/v0.91.4/review/evidence/csdlc/`. Local `.adl/` state may support execution, but the
public, inspectable C-SDLC record must live in tracked repo files.

## Boundaries

v0.91.4 should not:

- weaken AGENTS.md requirements for conductor use, editor-only card edits,
  worktrees, review, or closeout
- replace GitHub issues, PRs, CI, branch protection, or human review
- turn five-minute-sprint speed into permission to skip governance
- hide remaining legacy card drift as acceptable default behavior
- leave durable C-SDLC cards, review, closeout, trace, proof, or release
  evidence local-only
- treat CodeFriend product setup as a required C-SDLC dependency

## Source Map

This package is grounded in:

- `docs/cognitive-sdlc/README.md`
- `docs/milestones/v0.91.3/README.md`
- `docs/milestones/v0.91.3/NEXT_MILESTONE_HANDOFF_v0.91.3.md`
- `docs/milestones/v0.91.3/WBS_v0.91.3.md`
- `docs/milestones/v0.91.3/SPRINT_v0.91.3.md`
- `docs/milestones/v0.91.3/WP_EXECUTION_READINESS_v0.91.3.md`
- `docs/milestones/v0.91.3/QUALITY_GATE_v0.91.3.md`
- `docs/milestones/v0.91.3/features/COGNITIVE_SDLC_FIRST_SLICE.md`
- `docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md`
- `docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md`
- `docs/milestones/v0.91.3/features/TRANSITION_DAG_AND_SHARD_COORDINATION.md`
- `docs/milestones/v0.91.3/features/EVIDENCE_BUNDLE_AND_REVIEW_SYNTHESIS.md`
- `docs/milestones/v0.91.3/features/GOVERNED_MERGE_READINESS_GATE.md`
- `docs/milestones/v0.91.3/features/SRP_SOR_OBSMEM_HANDOFF.md`
- `docs/milestones/v0.91.3/features/FIVE_MINUTE_SPRINT_FIRST_PROOF.md`
- `docs/milestones/v0.91.3/C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md`
- `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`
- `docs/cognitive-sdlc/architecture.md`
- `docs/cognitive-sdlc/transition-schema.md`
- `docs/tooling/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`

Earlier cognitive-sdlc planning notes are drafting history. The tracked C-SDLC
docs home, the tracked v0.91.3 source package, and this v0.91.4 migration plan
are the branch-verifiable C-SDLC planning surfaces for review.

## Document Map

- Vision: [VISION_v0.91.4.md](VISION_v0.91.4.md)
- Design: [DESIGN_v0.91.4.md](DESIGN_v0.91.4.md)
- Decisions: [DECISIONS_v0.91.4.md](DECISIONS_v0.91.4.md)
- WBS: [WBS_v0.91.4.md](WBS_v0.91.4.md)
- Sprint plan: [SPRINT_v0.91.4.md](SPRINT_v0.91.4.md)
- Planned issue wave: [WP_ISSUE_WAVE_v0.91.4.yaml](WP_ISSUE_WAVE_v0.91.4.yaml)
- Execution readiness:
  [WP_EXECUTION_READINESS_v0.91.4.md](WP_EXECUTION_READINESS_v0.91.4.md)
- Feature proof coverage:
  [FEATURE_PROOF_COVERAGE_v0.91.4.md](FEATURE_PROOF_COVERAGE_v0.91.4.md)
- Demo matrix: [DEMO_MATRIX_v0.91.4.md](DEMO_MATRIX_v0.91.4.md)
- WildClawBench setup notes:
  [WILDCLAW_SETUP_NOTES.md](WILDCLAW_SETUP_NOTES.md)
- Quality gate: [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md)
- Release plan: [RELEASE_PLAN_v0.91.4.md](RELEASE_PLAN_v0.91.4.md)
- Release notes: [RELEASE_NOTES_v0.91.4.md](RELEASE_NOTES_v0.91.4.md)
- Milestone checklist:
  [MILESTONE_CHECKLIST_v0.91.4.md](MILESTONE_CHECKLIST_v0.91.4.md)
- Next milestone handoff:
  [NEXT_MILESTONE_HANDOFF_v0.91.4.md](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- Tracked workflow-state migration plan:
  [C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md](C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md)
- Public C-SDLC prompt records transition:
  [PUBLIC_CSDLC_PROMPT_RECORDS_TRANSITION_PLAN.md](PUBLIC_CSDLC_PROMPT_RECORDS_TRANSITION_PLAN.md)
- C-SDLC canonical docs:
  [../../cognitive-sdlc/README.md](../../cognitive-sdlc/README.md)
- Feature index: [features/README.md](features/README.md)

## CodeFriend Sidecar

The CodeFriend sidecar should remain bounded to the previously planned
pre-alpha setup lane:

- private product/site repository bootstrap
- minimal static welcome page
- AWS S3 asset origin in `us-west-2`
- CloudFront HTTPS delivery
- ACM certificate handling in `us-east-1`
- Route 53 DNS for `codefriend.ai` and `www.codefriend.ai`
- publication safety, verification, rollback, and handoff

The sidecar must not create customer-data systems, application runtime,
analytics, signup flows, alpha-product claims, or C-SDLC dependencies. It can
finish as completed or truthfully blocked by AWS/DNS approval, but it must not
alter the required C-SDLC closeout tail.

## Execution Model

v0.91.4 should execute through the same lifecycle discipline it is hardening:

- issue work begins from `SIP`, `STP`, and design-time `SPP` truth
- conductor routing is used for every issue and lifecycle stage
- card edits use the relevant editor skill
- implementation happens in bound worktrees, not on `main`
- review results are recorded in `SRP`
- execution and integration truth are recorded in `SOR`
- durable workflow evidence is promoted to tracked milestone evidence under
  `docs/milestones/v0.91.4/review/evidence/csdlc/`
- public prompt-record packets are exported from local execution-cache cards
  into the tracked evidence namespace after sanitization and validation

The CodeFriend sidecar may use the same issue/sprint discipline, but it remains
a sidecar product setup lane and does not replace C-SDLC core proof.

## Demo and Validation Surface

The milestone proof surface is intentionally split across:

- feature proof coverage:
  [FEATURE_PROOF_COVERAGE_v0.91.4.md](FEATURE_PROOF_COVERAGE_v0.91.4.md)
- demo matrix: [DEMO_MATRIX_v0.91.4.md](DEMO_MATRIX_v0.91.4.md)
- quality gate: [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md)
- milestone checklist:
  [MILESTONE_CHECKLIST_v0.91.4.md](MILESTONE_CHECKLIST_v0.91.4.md)
- release plan: [RELEASE_PLAN_v0.91.4.md](RELEASE_PLAN_v0.91.4.md)

Demo and validation claims must stay planned until v0.91.4 execution produces
tracked evidence.

## Success Criteria

v0.91.4 is ready to close when:

- new ADL software-development issues default to the C-SDLC lifecycle
- durable `SPP` records are tracked, issue-local, and operative before they are
  used to guide execution
- conductor/editor/doctor/validator tooling agrees on lifecycle state
- transition records preserve actor-role and standing truth for human and AI
  participants
- sprint closeout cannot truthfully complete while child issue closeout is stale
- SRP review results and SOR outcome truth feed the memory handoff boundary
- evidence bundles and merge gates are repeatable
- durable C-SDLC cards, sprint state, closeout, review, proof, trace, and
  release evidence are tracked in Git under the documented
  `docs/milestones/v0.91.4/review/evidence/csdlc/` namespace
- durable C-SDLC proof includes minimal signed trace bundles and verification
  results
- ObsMem ingestion consumes tracked evidence, not untracked local lore
- five-minute-sprint metrics are recorded across more than one transition
- legacy card/process drift is either migrated, blocked, or explicitly routed
  as follow-on work
- closeout follows the full proof, quality, docs/adoption review, internal
  review, external review, remediation, next-milestone planning,
  next-milestone review, and release ceremony sequence
- the CodeFriend sidecar setup is either complete, truthfully blocked with a
  handoff, or explicitly routed before release

## Exit Criteria

- The v0.91.4 issue wave has been opened and completed or truthfully routed.
- The C-SDLC default-operation claims in this README are supported by tracked
  proof, review, trace, and release evidence.
- Review, remediation, next-milestone planning, next-milestone review, and
  release ceremony happen in the required order.
- Any incomplete CodeFriend sidecar work is recorded as blocked or routed
  without weakening the C-SDLC release bar.
