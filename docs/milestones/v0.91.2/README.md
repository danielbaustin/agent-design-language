# v0.91.2 Milestone README

## Status

Active milestone package. v0.91.2 is the follow-on pressure-release milestone
after v0.91.1. The WP issue wave was opened as `#3000` through `#3023`, and the
sprint-conductor umbrella issues were opened as `#3025` through `#3028`. Sprint
4 is in corrective review/remediation: `WP-17`, the bounded `WP-17A` demo
follow-on, `WP-18`, and `WP-19` are closed; the first `WP-20` internal review
packet was too thin for external handoff; `WP-20B` is the controlling full
internal review packet; and accepted `WP-20B` findings must be fixed and
rechecked before `WP-21` external review. External review, remediation,
next-milestone planning, and ceremony work remain.

## Purpose

v0.91.2 is the tooling, evaluation, productization, publication, and repo-health
milestone that clears the remaining high-value TBD backlog without overloading
the v0.91 moral-governance wave or the v0.91.1 inhabited-runtime readiness
wave.

The milestone should turn several recurring pain points into implemented,
reviewable systems:

- UTS + ACC multi-model benchmarking that distinguishes ADL JSON proposal mode
  from provider-native tool/function-call mode.
- Runtime and test-cycle recovery so coverage and proof gates stop burning
  entire days.
- CodeFriend/review packet productization.
- Google Workspace CMS bridge implementation, bounded live-safety surfaces, and
  reusable operational package, while keeping C-SDLC workflow truth separate
  from GWS.
- Moderne / OpenRewrite LST modernization workflow support.
- Bounded speculative-decoding evaluation inside ADL's deterministic runtime
  posture.
- Repo-visibility follow-on work so canonical-source and linkage surfaces
  become more operationally useful.
- Publication program artifacts, including the general-intelligence paper
  packet and future Gödel Agents paper backlog.
- Rustdoc and documentation cleanup.
- Workflow guardrails that prevent main-branch writes, hung closeout watchers,
  and unsafe report-generation shell behavior from recurring.
- ADR candidates that make the milestone's durable architecture boundaries
  explicit before closeout, including the C-SDLC tracked-state and signed-trace
  direction for v0.91.3/v0.91.4.

## Milestone Role

v0.91.2 should be practical and cleanup-oriented without becoming small. It
should convert the remaining planning backlog into concrete work products that
make later identity, birthday, constitutional, product, and publication work
faster and safer.

## Boundaries

v0.91.2 should not:

- reopen v0.91 moral-governance implementation
- reopen v0.91.1 runtime-inhabitant readiness
- claim production external tool execution based on model output
- silently move canonical docs into Google Workspace
- publish papers or customer reports without review
- treat Moderne or OpenRewrite recipe execution as automatic mass rewrite
- use test-cycle recovery as an excuse to weaken proof obligations

## Source Map

This package is grounded in:

- `.adl/docs/TBD/tools/UTS_ACC_MULTI_MODEL_BENCHMARK_PLAN.md`
- `.adl/docs/TBD/tools/RUNTIME_V2_TEST_CYCLE_RECOVERY_PLAN.md`
- `.adl/docs/TBD/v0.90.5_TEST_RUNTIME_REDUCTION_PLAN.md`
- `.adl/docs/TBD/codebuddy_ai/` legacy working-name source cluster for CodeFriend productization
- `.adl/docs/TBD/google_workspace_cms/`
- `.adl/docs/TBD/code_modernization/`
- `.adl/docs/TBD/ADL_AND_GENERIC_SPECULATIVE_DECODING.md`
- `.adl/docs/TBD/ADL_AND_SPECULATIVE_CODING_REPLAY.md`
- `.adl/docs/TBD/repo_visibility/`
- `.adl/docs/TBD/publication/`
- `.adl/docs/TBD/general-intelligence-paper/`
- `.adl/docs/TBD/RUSTDOC_GAP_ANALYSIS.md`
- `.adl/docs/TBD/ADL_DOC_CLEANUP_LEDGER.md`
- `.adl/docs/TBD/workflow_tooling/`
- `.adl/docs/TBD/ARXIV_AUTHORING_PROCESS_NOTES.md`
- `.adl/docs/TBD/V0912_ADR_PLAN_2026-05-13.md`
- `.adl/docs/TBD/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_2026-05-19.md`

## Document Map

- WBS: [WBS_v0.91.2.md](WBS_v0.91.2.md)
- Vision: [VISION_v0.91.2.md](VISION_v0.91.2.md)
- Design: [DESIGN_v0.91.2.md](DESIGN_v0.91.2.md)
- Decisions: [DECISIONS_v0.91.2.md](DECISIONS_v0.91.2.md)
- ADR plan: [ADR_PLAN_v0.91.2.md](ADR_PLAN_v0.91.2.md)
- Sprint plan: [SPRINT_v0.91.2.md](SPRINT_v0.91.2.md)
- Sprint-conductor execution plan:
  [SPRINT_CONDUCTOR_EXECUTION_PLAN_v0.91.2.md](SPRINT_CONDUCTOR_EXECUTION_PLAN_v0.91.2.md)
- Active issue wave: [WP_ISSUE_WAVE_v0.91.2.yaml](WP_ISSUE_WAVE_v0.91.2.yaml)
- Execution readiness:
  [WP_EXECUTION_READINESS_v0.91.2.md](WP_EXECUTION_READINESS_v0.91.2.md)
- Demo matrix: [DEMO_MATRIX_v0.91.2.md](DEMO_MATRIX_v0.91.2.md)
- Feature proof coverage:
  [FEATURE_PROOF_COVERAGE_v0.91.2.md](FEATURE_PROOF_COVERAGE_v0.91.2.md)
- Quality gate: [QUALITY_GATE_v0.91.2.md](QUALITY_GATE_v0.91.2.md)
- Feature index: [features/README.md](features/README.md)
- Card bundle readiness:
  [CARD_BUNDLE_READINESS_v0.91.2.md](CARD_BUNDLE_READINESS_v0.91.2.md)
- SPP readiness: [SPP_READINESS_v0.91.2.md](SPP_READINESS_v0.91.2.md)
- Milestone checklist:
  [MILESTONE_CHECKLIST_v0.91.2.md](MILESTONE_CHECKLIST_v0.91.2.md)
- Release plan: [RELEASE_PLAN_v0.91.2.md](RELEASE_PLAN_v0.91.2.md)
- Release readiness: [RELEASE_READINESS_v0.91.2.md](RELEASE_READINESS_v0.91.2.md)
- Release evidence: [RELEASE_EVIDENCE_v0.91.2.md](RELEASE_EVIDENCE_v0.91.2.md)
- Release notes: [RELEASE_NOTES_v0.91.2.md](RELEASE_NOTES_v0.91.2.md)
- Third-party review handoff:
  [ADL_v0.91.2_THIRD_PARTY_REVIEW_HANDOFF.md](ADL_v0.91.2_THIRD_PARTY_REVIEW_HANDOFF.md)
- Next milestone handoff:
  [NEXT_MILESTONE_HANDOFF_v0.91.2.md](NEXT_MILESTONE_HANDOFF_v0.91.2.md)
- End-of-milestone report:
  [END_OF_MILESTONE_REPORT_v0.91.2.md](END_OF_MILESTONE_REPORT_v0.91.2.md)

## Success Criteria

v0.91.2 is ready to close only when the project has credible multi-model
UTS+ACC evidence, a healthier test/runtime gate strategy, a
review/productization path, a bounded Workspace CMS bridge with reusable
operational guidance, Moderne/OpenRewrite LST modernization and publication
packages, workflow guardrails that reduce the operational failures that slowed
v0.90.5 and v0.91, a reviewable ADR candidate packet for the durable
architecture decisions introduced or clarified by the milestone, and completed
review/remediation/ceremony truth. Accepted `WP-20B` findings must be fixed or
explicitly dispositioned before external review or release ceremony claims.

Execution note: WP-01 opened and carded the issue wave. Each WP starts only
when routed through `workflow-conductor`, bound with `pr run`, reviewed before
PR publication, and closed out after merge.
