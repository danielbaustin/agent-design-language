# TBD Plan Allocation Through v0.95

## Status

Tracked allocation map for the local-only `.adl/docs/TBD/` planning corpus.

## Purpose

`.adl/docs/TBD/` is ignored workspace state, but it contains much of the
source material that feeds milestone planning. This document records where that
material belongs now that the `v0.91.2` through `v0.95` roadmap spine exists.

This is not a feature list replacement. The canonical feature list remains
`docs/planning/ADL_FEATURE_LIST.md`. This file is the bridge from local TBD
source notes to tracked execution homes.

## Allocation Rules

- Keep `.adl/docs/TBD/` as local source provenance unless a milestone issue
  explicitly promotes, retires, or publishes the material.
- Use `docs/milestones/<version>/features/` for milestone feature contracts.
- Use `docs/planning/` only for durable cross-milestone planning maps.
- Treat review findings, logs, and generated traces as evidence or archive
  material, not feature plans.
- Do not claim "unowned: none" unless every root plan file and major cluster
  has an explicit tracked home, historical home, or deferred bucket.

## Directory Allocation

| TBD directory | Current home | Disposition |
| --- | --- | --- |
| `ToM/` | `v0.93` social cognition and relationship features | Allocated. Older `v0.91.1` wording is superseded by `docs/milestones/v0.93/features/THEORY_OF_MIND_AND_SOCIAL_COGNITION_v0.93.md`. |
| `a2a/` | `v0.91.1` A2A baseline; `v0.93` provider/security governance | Allocated as source provenance and later security/governance input. |
| `acip/` | `v0.91.1` ACIP hardening baseline; `v0.93` enterprise/security follow-on | Allocated. Not a loose pre-MVP plan. |
| `anrm/` | `v0.91.1` placement baseline; `v0.95` CSM Shepherd/Gemma training path | Allocated. README language that points only to `v0.91.1` is stale. |
| `capability_testing/` | `v0.91.1` harness baseline; `v0.95` Aptitude Atlas platform | Allocated. Product and reporting docs feed `v0.95`. |
| `citizen_standing/` | `v0.91.1` standing baseline; `v0.93` citizenship/governance | Allocated. |
| `citizen_state/` | `v0.91.1` state substrate baseline; `v0.93` citizenship/governance | Allocated. |
| `code_modernization/` | `v0.91.2` WP-10 modernization demo lane | Allocated. Old "not scheduled for immediate execution" wording is stale. |
| `codebuddy_ai/` | `v0.91.2` CodeFriend productization; `v0.95` MVP product polish | Allocated, but the local directory name still preserves the pre-CodeFriend working name. |
| `csm_observatory/` | `v0.91.1` active surface baseline; `v0.95` MVP operator surface | Allocated. |
| `demo_candidates/` | `v0.95` demo catalog and MVP walkthrough | Allocated as demo source material; individual candidates may already be historical. |
| `economics/` | `v0.90.4` contract-market baseline; `v0.94.1` payments/x402 | Allocated. Residual business model material is not a separate technical feature before `v0.95`. |
| `general-intelligence-paper/` | `v0.91.2` general-intelligence paper packet | Allocated. Generated LaTeX build artifacts should remain local or be cleaned by a paper-specific issue. |
| `google_workspace_cms/` | `v0.91.2` WP-08/WP-09 Workspace bridge and Rust adapter | Allocated. |
| `hey_jude_demo/` | Delivered demo provenance; `v0.95` demo catalog polish if reused | Allocated as historical/demo-catalog source. |
| `intelligence/` | `v0.91.1` metric baseline; `v0.91.2` publication lane; `v0.92` through `v0.95` cognitive/MVP features | Allocated across the roadmap. |
| `learning_model/` | `v0.91.1` governed-learning baseline; `v0.95` shepherd/evaluator path | Allocated. |
| `long_lived_agents/` | `v0.90` delivered baseline; `v0.91.1` inhabited-runtime consumption | Delivered/provenance. |
| `memory_identity/` | `v0.92` identity, stable name, memory grounding, and witness features | Partially allocated. The identity-continuity slice is scheduled, but `ADL_MEMORY_PALACE_ARCHITECTURE.md` remains deferred provenance until a bounded runtime, replay, visualization, or identity-continuity slice explicitly consumes it. |
| `moral_governance/` | `v0.91` moral baseline; `v0.93` constitutional citizenship and polis governance | Allocated. |
| `multiagent_demos/` | `v0.95` demo catalog and MVP walkthrough | Allocated. Keep as source candidates until the demo catalog issue chooses what ships. |
| `planning/` | Operator-edited GTM/business planning | Excluded from this allocation pass. These docs are still being edited and should not be treated as unallocated technical execution work. |
| `publication/` | `v0.91.2` publication program and paper packet lane; `v0.95` publication polish | Allocated. |
| `repo_visibility/` | `v0.91.2` repo visibility follow-on; `v0.95` repo-cognition convergence | Allocated. |
| `retired/` | Local archive | Allocated as archive. Do not treat as active execution scope. |
| `runtime_v2/` | `v0.91.1` runtime/polis baseline; `v0.91.2` runtime/test recovery; later security/MVP hardening | Allocated. |
| `tools/` | `v0.90.5` governed tools baseline; `v0.91.2` UTS/ACC benchmark; `v0.94` trust; `v0.95` hardening | Allocated. README language should point at the active UTS/ACC follow-on. |
| `v0.91-card-review/` | Historical card-review evidence | Archive/provenance. |
| `workflow_tooling/` | `v0.91.2` workflow guardrails; `v0.95` control-plane hardening | Allocated. |

## Root Document Allocation

| Root TBD document | Current home | Disposition |
| --- | --- | --- |
| `ADL_AND_GENERIC_SPECULATIVE_DECODING.md` | `v0.91.2` WP-11 speculative decoding prototype | Allocated. |
| `ADL_AND_SLEEP.md` | `v0.91.1` agent lifecycle/wellbeing provenance | Delivered/provenance, not an unowned future feature. |
| `ADL_AND_SPECULATIVE_CODING_REPLAY.md` | `v0.91.2` WP-11 speculative decoding/replay lane | Allocated. |
| `ADL_DOC_CLEANUP_LEDGER.md` | `v0.91.2` WP-15 rustdoc/doc cleanup | Allocated. |
| `AI_CHARACTER_AUDIT.md` | None before `v0.95` | Unallocated/deferred. Candidate home is a CodeFriend/product-voice or docs-style issue. |
| `ARXIV_AUTHORING_PROCESS_NOTES.md` | `v0.91.2` publication program | Allocated. |
| `AXIOM_OF_CONSTRUCTABILITY.md` | None before `v0.95` | Unallocated/deferred. Candidate home is a future paper/source-philosophy issue. |
| `BROADCAST_AUDIO_ROADMAP_v0911.md` | `v0.91.1` broadcast-audio sprint provenance | Delivered/provenance. Preserve in place per operator direction. |
| `CODEFRIEND_PLANNING.md` | `v0.91.2` CodeFriend productization; `v0.95` MVP polish | Allocated. |
| `HEY_JUDE_AUDIO_UPGRADE_PLAN.md` | Delivered audio/demo provenance; `v0.95` demo catalog if reused | Allocated as demo source material. |
| `LOCAL_BACKLOG.md` | Local backlog source, not a milestone feature by itself | Active local control surface. |
| `MILESTONE_CLOSEOUT_CHECKLIST.md` | Recurring release-tail process; `v0.91.2` workflow guardrails | Allocated as process guidance, not a feature. |
| `MILESTONE_COMPRESSION_PLAN.md` | `v0.95` dashboard/compression reporting | Allocated. |
| `NEW_FEATURE_MILESTONE_ASSIGNMENT_PLAN.md` | Superseded by feature list and milestone packages through `v0.95` | Retire or keep as provenance after this allocation pass. |
| `RUSTDOC_GAP_ANALYSIS.md` | `v0.91.2` WP-15 rustdoc/doc cleanup | Allocated. |
| `SPRINT_CONDUCTOR_RETROSPECTIVE_2026-05-09.md` | `v0.91.2` sprint-conductor execution plan and workflow guardrails | Allocated as process source evidence. |
| `STARTUP_GRANTS_PLAN_0.1.md` | Operator-edited GTM/business planning | Excluded from this allocation pass. Do not classify until the business-planning edit pass is done. |
| `TBD_DOC_STATUS_INVENTORY.md` | Local inventory, superseded by this tracked allocation map until refreshed | Active local control surface requiring refresh. |
| `V0911_BROADCAST_AUDIO_SPRINT_CLOSEOUT_REVIEW_2026-05-10.md` | `v0.91.1` broadcast-audio closeout evidence | Archive/provenance. |
| `V0911_ISSUE_2940_REVIEW_FINDINGS_2026-05-10.md` | `v0.91.1` / `v0.91.2` feature-doc cleanup evidence | Archive after remediation. |
| `V0911_PR2941_REVIEW_FINDINGS_2026-05-10.md` | `v0.91.1` feature-list cleanup evidence | Archive after remediation. |
| `V0911_SPRINT1_REVIEW_FINDINGS_2026-05-09.md` | `v0.91.1` sprint-review evidence | Archive/provenance. |
| `V0911_V0912_DOCS_REVIEW_REPORT_2026-05-07.md` | `v0.91.2` docs package source evidence | Archive after v0.91.2 planning stabilization. |
| `V0911_WP19_REVIEW_FINDINGS_2026-05-10.md` | `v0.91.1` docs cleanup evidence | Archive after remediation. |
| `V0912_ADR_PLAN_2026-05-13.md` | No tracked ADR-authoring issue yet | Deferred/process-planning. It should become one or more ADR issues before being treated as allocated execution work. |
| `V0912_DOCS_REVIEW_FINDINGS_2026-05-09.md` | `v0.91.2` docs package review evidence | Archive after remediation. |
| `V0912_MODERNE_DEMO_PLAN_2026-05-10.md` | `v0.91.2` WP-10 modernization demo | Allocated. |
| `V0912_SPRINT_CONDUCTOR_EXECUTION_PLAN_REVIEW_FINDINGS_2026-05-11.md` | `v0.91.2` sprint-conductor plan repair evidence | Archive after remediation. |
| `V0_91_1_DOCS_AND_CARDS_REVIEW_ISSUE_BODY.md` | Historical issue-body evidence | Archive/provenance. |
| `V0_91_1_DOCS_REVIEW_REPORT.md` | Historical docs-review evidence | Archive/provenance. |
| `WP_01_REVIEW_PLAN_v0.91.2.md` | `v0.91.2` WP-01 review source evidence | Archive after WP-01 closeout. |
| `v0.90.5_TEST_RUNTIME_REDUCTION_PLAN.md` | `v0.91.2` runtime/test-cycle recovery | Allocated. |
| `v0.91.1_gap_review.md` | Historical gap-review evidence | Archive/provenance. |
| `v0.91_1_runtime_observatory_dependency_note.md` | `v0.91.1` runtime observatory provenance | Archive/provenance. |
| `v0.91_gap_review.md` | Historical gap-review evidence | Archive/provenance. |

## Not Yet Allocated For Execution Before v0.95

The current audit found three root plan notes that are not clearly
allocated to execution before or through `v0.95`:

- `AI_CHARACTER_AUDIT.md`: useful style/product signal, but no milestone issue
  currently owns it.
- `AXIOM_OF_CONSTRUCTABILITY.md`: research/philosophy source note, but no
  tracked paper or feature issue currently consumes it.
- `V0912_ADR_PLAN_2026-05-13.md`: process-planning note for ADRs that should
  become tracked ADR authoring issues before it is counted as scheduled work.

The `planning/` subdirectory and the root startup-grants note are explicitly
out of scope for this pass because the operator is still editing the GTM and
business-planning material.

## Cleanup Recommendations

- Refresh local `.adl/docs/TBD/TBD_DOC_STATUS_INVENTORY.md` to match this
  tracked map, replacing stale `v0.91.1` and pre-CodeFriend language.
- Rename or signpost `codebuddy_ai/` to CodeFriend in local planning notes
  without losing historical provenance.
- Move root review findings and old gap reports into a local review/archive
  bucket after their remediation issues are closed.
- Keep broadcast-audio source material in place unless the operator explicitly
  authorizes moving it.
- Do not delete generated paper artifacts or logs in this pass. Open a
  paper-specific or hygiene-specific issue if those should be pruned.

## Validation Notes

This map was checked against:

- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.91.2/`
- `docs/milestones/v0.92/`
- `docs/milestones/v0.93/`
- `docs/milestones/v0.94/`
- `docs/milestones/v0.94.1/`
- `docs/milestones/v0.95/`
- local `.adl/docs/TBD/` root and first-level directory inventory
