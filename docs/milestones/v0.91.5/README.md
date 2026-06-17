# v0.91.5 Milestone README

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-02`
- Owner: ADL maintainers
- Status: `sprint_4_release_tail_active`
- Related setup umbrella: `#3506` (closed)
- Closed setup children: `#3507`, `#3508`, `#3509`, `#3510`, `#3511`
- Planning template set: `docs/templates/planning/1.0.0`
- Canonical WP standard: [ADL_MILESTONE_WP_ORDERING_STANDARD.md](../../planning/ADL_MILESTONE_WP_ORDERING_STANDARD.md)

## Status

Current status: `sprint_4_release_tail_active`

- Planning: v0.91.5 bridge package, issue wave, sprint umbrellas, and closeout
  tail are seeded and in use.
- Execution: Sprint 1, Sprint 2, and Sprint 3 delivery are materially landed;
  the first internal-review remediation mini-sprint closed through `#3899` on
  `2026-06-17`.
- Current frontier: Sprint 4 `#3574` remains active after WP-14 `#3575` and
  WP-15 `#3579` closed on `2026-06-17`; the remaining active release-tail
  issues are WP-16 `#3576`, WP-17 `#3580`, WP-18 `#3577`, WP-19 `#3581`,
  WP-20 `#3578`, and the open second-pass internal-review execution slice
  `#3923`.
- Validation: focused milestone docs, issue-state, quality-gate, and review
  packet validation is active in the release tail.
- Release readiness: `blocked_for_release_tail` until Sprint 4 closeout truth,
  second-pass internal review, final remediation/preflight, and release
  ceremony are complete.

Setup truth:

- `#3506` is the closed v0.91.4-origin setup umbrella that created the bridge
  package and child setup wave.
- `#3507`-`#3511` are the closed v0.91.5 setup child issues for planning
  package, issue reallocation, v0.91.4 release-tail reconciliation, v0.92
  dependency reconciliation, and migration-truth review.
- `#3568` is the closed WP-01 opening follow-up after v0.91.4 release.
- `#3567` closed the reusable milestone WP ordering standard.
- `#3569` defines the portable ADL project adapter contract and templates.
- `#3582` is scheduled to rewrite/normalize downstream v0.91.5 card bundles
  after prompt templates v1.1 lands through `#3553`.
- `#3571`, `#3572`, and `#3573` are closed.
- `#3574` remains open as the active Sprint 4 umbrella.
- `#3575` and `#3579` closed on `2026-06-17` as the completed WP-14 and WP-15
  release-tail steps.
- `#3576`, `#3580`, `#3577`, `#3581`, and `#3578` are the remaining open
  closeout-tail issues.
- `#3899` closed on `2026-06-17` as the completed first internal-review
  remediation tranche.
- `#3923` is open as the active second-pass internal-review issue and now
  serves as the live review-entry follow-on under the WP-16 / Sprint 4 lane.

## Purpose

Provide the canonical entry point for v0.91.5: a real bridge milestone between
v0.91.4 C-SDLC hardening and v0.92 first birthday.

v0.91.5 originally gave the remaining pre-v0.92 work a tracked, reviewable
home while `v0.91.4` finished its own release-tail closeout. It now stands as
the active bridge milestone whose remaining Sprint 4 work must close cleanly
before `v0.92` opens.

## Milestone Role

`v0.91.5` moves ADL from a hardened mostly single-agent C-SDLC lane to a
pre-v0.92 operating posture where AEE completion routing, multi-agent
execution, provider/model breadth, public prompt records, demos, and activation
testing are ready enough for the first-birthday milestone.

This milestone exists to:

- make bounded multi-agent C-SDLC execution work truthfully;
- define AEE completion as a first-class closure tranche instead of leaving it
  implicit until `v0.95`;
- prepare the provider/model matrix, including OpenRouter, hosted models, local
  Ollama, and remote Ollama;
- move public prompt records and `.adl` cleanup/archive planning into a
  governed transition;
- prepare demo and Unity Observatory readiness;
- complete the v0.92 activation-test map and first-birthday launch preflight.
- make external ADL-adjacent repositories portable through a tracked adapter
  contract and templates before relying on paper/UTS/demo repo evidence.

Expected outcomes:

- v0.92 can open from v0.91.5 closeout, not direct v0.91.4 chat memory.
- `#3377` has a complete first-birthday go/no-go readiness packet.
- Multi-agent work is proven, blocked, or explicitly not relied on by v0.92.

## Boundaries

In scope:

- Multi-agent C-SDLC stabilization and usefulness review.
- Provider/model matrix and OpenRouter readiness.
- Public C-SDLC prompt packet export, validation, redaction, and archive
  disposition.
- Demo showcase, Celestial Rescue, and Unity Observatory readiness routing.
- v0.92 activation-test map and first-birthday preflight.

Out of scope:

- Implementing the v0.92 first birthday.
- Claiming unbounded autonomous multi-agent software development.
- Deleting `.adl` historical state without review.
- Moving v0.93 constitutional governance into the bridge.

Known risks:

- Multi-agent coordination may not yet beat single-agent execution on small
  tasks.
- Model/provider breadth may expose role aptitude gaps rather than solve them.
- Public prompt records may need stricter redaction than initially expected.

Open questions:

- Which model/provider lanes are suitable for planner, worker, reviewer,
  janitor, and watcher roles?
- Does Celestial Rescue double as Unity Observatory proof or only prepare that
  path for v0.92?

## Source Map

Primary planning and proof sources:

- Vision: [VISION_v0.91.5.md](VISION_v0.91.5.md)
- Design: [DESIGN_v0.91.5.md](DESIGN_v0.91.5.md)
- Work Breakdown Structure: [WBS_v0.91.5.md](WBS_v0.91.5.md)
- Sprint plan: [SPRINT_v0.91.5.md](SPRINT_v0.91.5.md)
- Decisions log: [DECISIONS_v0.91.5.md](DECISIONS_v0.91.5.md)
- Demo matrix: [DEMO_MATRIX_v0.91.5.md](DEMO_MATRIX_v0.91.5.md)
- Milestone checklist: [MILESTONE_CHECKLIST_v0.91.5.md](MILESTONE_CHECKLIST_v0.91.5.md)
- Release plan: [RELEASE_PLAN_v0.91.5.md](RELEASE_PLAN_v0.91.5.md)
- Release notes: [RELEASE_NOTES_v0.91.5.md](RELEASE_NOTES_v0.91.5.md)
- Quality gate: [QUALITY_GATE_v0.91.5.md](QUALITY_GATE_v0.91.5.md)
- Feature proof coverage: [FEATURE_PROOF_COVERAGE_v0.91.5.md](FEATURE_PROOF_COVERAGE_v0.91.5.md)
- Pre-v0.92 bridge feature-doc ledger: [PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md](PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md)
- WP execution readiness: [WP_EXECUTION_READINESS_v0.91.5.md](WP_EXECUTION_READINESS_v0.91.5.md)
- ADR plan: [ADR_PLAN_v0.91.5.md](ADR_PLAN_v0.91.5.md)
- Next milestone handoff: [NEXT_MILESTONE_HANDOFF_v0.91.5.md](NEXT_MILESTONE_HANDOFF_v0.91.5.md)
- Milestone WP ordering standard: [ADL_MILESTONE_WP_ORDERING_STANDARD.md](../../planning/ADL_MILESTONE_WP_ORDERING_STANDARD.md)

Supporting / domain-specific docs:

- v0.92 activation test map: [V092_ACTIVATION_TEST_MAP_v0.91.5.md](V092_ACTIVATION_TEST_MAP_v0.91.5.md)
- Docs-only validation bundle policy: [DOCS_ONLY_VALIDATION_BUNDLE_3736.md](DOCS_ONLY_VALIDATION_BUNDLE_3736.md)
- Logging validation checklist: [LOGGING_VALIDATION_CHECKLIST_3711.md](LOGGING_VALIDATION_CHECKLIST_3711.md)
- Feature index: [features/README.md](features/README.md)
- Issue wave: [WP_ISSUE_WAVE_v0.91.5.yaml](WP_ISSUE_WAVE_v0.91.5.yaml)

Sprint and review support:

- Logging mini-sprint closeout packet for umbrella `#3703`: [LOGGING_MINI_SPRINT_CLOSEOUT_3703.md](review/logging_observability/LOGGING_MINI_SPRINT_CLOSEOUT_3703.md)
- Tools remediation sprint closeout packet for umbrella `#3845`: [TOOLS_REMEDIATION_SPRINT_CLOSEOUT_3845.md](review/tooling_adoption/TOOLS_REMEDIATION_SPRINT_CLOSEOUT_3845.md)
- Strategic Cognitive Reserve project bootstrap closeout for `#3868`: [STRATEGIC_COGNITIVE_RESERVE_PROJECT_BOOTSTRAP_3868.md](review/tooling_adoption/STRATEGIC_COGNITIVE_RESERVE_PROJECT_BOOTSTRAP_3868.md)
- TBD documentation backlog closeout for `#3635`: [TBD_DOCUMENTATION_BACKLOG_CLOSEOUT_3635.md](review/tooling_adoption/TBD_DOCUMENTATION_BACKLOG_CLOSEOUT_3635.md)
- Historical first internal-review findings register: [V0915_FIRST_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-16.md](review/internal_review/V0915_FIRST_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-16.md)
- Historical first internal-review remediation queue: [V0915_FIRST_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-16.md](review/internal_review/V0915_FIRST_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-16.md)
- WP-14 quality-gate application: [V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md](review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md)
- WP-15 docs/review alignment packet: [V0915_WP15_DOCS_REVIEW_ALIGNMENT_2026-06-17.md](review/internal_review/V0915_WP15_DOCS_REVIEW_ALIGNMENT_2026-06-17.md)
- Current second-pass internal-review handoff plan: [V0915_SECOND_PASS_INTERNAL_REVIEW_PLAN_2026-06-17.md](review/internal_review/V0915_SECOND_PASS_INTERNAL_REVIEW_PLAN_2026-06-17.md)

## Document Map

Use the source map above as the canonical navigation surface. Keep this README
concise; detailed work-package, sprint, demo, and release gates live in the
linked documents.

## Bridge Work

This milestone is itself a bridge sidecar around v0.91.4 release closeout.

- Sidecar scope: pre-v0.92 multi-agent, provider/model, prompt-record,
  demo-readiness, and activation-readiness work.
- Sidecar boundary: no first-birthday implementation and no unreviewed `.adl`
  deletion.
- Sidecar proof surface: issue labels, v0.91.5 planning docs, prompt/export
  evidence, model-role matrix, demo proof map, and `#3377`.

## Execution Model

This milestone is executed as an ordered issue/PR sequence. The exact WP count
is milestone-specific.

Execution expectations:

- WP-01 is the milestone planning/setup/issue-wave readiness gate. It closes
  only after planned issues, cards, sprint umbrellas, and initial sequencing are
  ready to begin or explicitly routed.
- The canonical milestone structure is captured by closed issue `#3567`.
- Portable ADL adapter planning and templates are routed through `#3569`;
  external repository migrations live outside WP-01.
- Public prompt, provider/model, multi-agent, demo, and activation work occupy
  the middle of the sequence.
- Demo/proof, quality, docs/review convergence, v0.92 preflight, and release
  ceremony happen at the tail.
- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Each WP records focused validation and merge/readiness proof.
- Do not treat v0.91.5 setup as v0.92 birthday evidence.

## Demo and Validation Surface

Primary validation is defined in [DEMO_MATRIX_v0.91.5.md](DEMO_MATRIX_v0.91.5.md).

Additional validation surfaces:

- GitHub issue labels and titles for moved issues.
- YAML parsing for `WP_ISSUE_WAVE_v0.91.5.yaml`.
- Planning-template validation for canonical docs.
- Link checks for milestone-relative planning docs.
- Focused tool/runtime tests only where implementation changes land.

Determinism evidence:

- [WP_ISSUE_WAVE_v0.91.5.yaml](WP_ISSUE_WAVE_v0.91.5.yaml)
- [V092_ACTIVATION_TEST_MAP_v0.91.5.md](V092_ACTIVATION_TEST_MAP_v0.91.5.md)

## Success Criteria

- All moved issues carry `version:v0.91.5` and no longer claim active v0.91.4
  scope.
- Multi-agent execution has a bounded proof or a blocker that v0.92 can consume.
- AEE completion has explicit v0.92 owner/proof routing through `#3534` and
  `#3377`.
- v0.92 activation testing covers every identified feature surface.
- Public prompt records have export, validation, redaction, and archive routing.

## Exit Criteria

- All canonical milestone documents are complete and internally consistent.
- All WBS items are implemented or explicitly deferred.
- Demo matrix is runnable and validated where demos are implemented.
- Quality gates relevant to touched surfaces are passing or exceptions are
  documented.
- Milestone checklist is complete or exceptions are documented.
- Release artifacts are ready.
