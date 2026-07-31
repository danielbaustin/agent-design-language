# v0.91.8 Milestone README

## Metadata

- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Status: active readiness; implementation not started
- Active WP-01 readiness issue: `#5594`
- Milestone sprint umbrella: `#5595`
- Historical planning sources: `#5335` and `#5383`
- Restored source issue: `#4641` remains `v0.91.7` WP-14
- v0.91.8 platform acceptance parent: `#5384` / WP-14A
- Downstream milestone: `v0.92`

## Purpose

`v0.91.8` is a bridge milestone between the `v0.91.7` implementation tranche
and the `v0.92` birthday milestone. It exists to finish the ADL Core
Rearchitecture platform prerequisite: ADL v2, Runtime v3, and C-SDLC v2 must be
accepted at exact revisions, installed on stable operational paths, exercised
through declared lifecycle contracts, and handed off to `v0.92` without
overstating birthday readiness.

The milestone is planned from live issues that already exist. This package is
the missing local source of truth for those issues.

## Current Routing Truth

- `#4641` is restored to `[v0.91.7][WP-14] Launch and v0.92 birthday handoff`.
- `#5384` preserves the v0.91.8 integrated platform acceptance/deployment
  content that temporarily overwrote `#4641`.
- `#5383` is the historical v0.91.7 setup authority for this package and is
  closed; do not describe setup as currently in progress.
- `#5335` is retained as historical setup context and no longer carries active
  WP-01 ownership.
- `#5594` is the active WP-01 readiness and canonical reconciliation gate.
- `#5595` is the single milestone sprint umbrella. Nested umbrellas `#5497`,
  `#5361`, and `#5384` own bounded multi-agent, Runtime v3, and integrated
  acceptance child sets without duplicating implementation ownership.
- WP-14A accepts only the platform revisions. Unity proof is owned by WP-15,
  C-SDLC tooling remediation by WP-20, and the exact-revision handoff,
  Memory Palace, launch/identity, and Adaptive Learning inputs by WP-21.

## Status

The milestone is not release-approved. Planning text in this directory does not
prove implementation, parity, deployment, deletion, or `v0.92` readiness.

Every work package must exit as one of:

- implemented and proven at an exact revision;
- already closed with current evidence;
- blocked with evidence and operator approval;
- explicitly deferred with downstream consumption truth.

## Document Map

- Vision: [VISION_v0.91.8.md](VISION_v0.91.8.md)
- Design: [DESIGN_v0.91.8.md](DESIGN_v0.91.8.md)
- Decisions: [DECISIONS_v0.91.8.md](DECISIONS_v0.91.8.md)
- Work breakdown: [WBS_v0.91.8.md](WBS_v0.91.8.md)
- Sprint plan: [SPRINT_PLAN_v0.91.8.md](SPRINT_PLAN_v0.91.8.md)
- Parallel execution plan: [PARALLEL_EXECUTION_PLAN_v0.91.8.md](PARALLEL_EXECUTION_PLAN_v0.91.8.md)
- WP-01 readiness report: [review/V0918_WP01_EXECUTION_READINESS_5594.md](review/V0918_WP01_EXECUTION_READINESS_5594.md)
- Issue wave: [WP_ISSUE_WAVE_v0.91.8.yaml](WP_ISSUE_WAVE_v0.91.8.yaml)
- Canonical document inventory: [CANONICAL_DOC_INVENTORY_v0.91.8.md](CANONICAL_DOC_INVENTORY_v0.91.8.md)
- Demo matrix: [DEMO_MATRIX_v0.91.8.md](DEMO_MATRIX_v0.91.8.md)
- Checklist: [MILESTONE_CHECKLIST_v0.91.8.md](MILESTONE_CHECKLIST_v0.91.8.md)
- Release plan: [RELEASE_PLAN_v0.91.8.md](RELEASE_PLAN_v0.91.8.md)
- Release notes draft: [RELEASE_NOTES_v0.91.8.md](RELEASE_NOTES_v0.91.8.md)
- Quality gate: [QUALITY_GATE_v0.91.8.md](QUALITY_GATE_v0.91.8.md)
- Feature proof coverage: [FEATURE_PROOF_COVERAGE_v0.91.8.md](FEATURE_PROOF_COVERAGE_v0.91.8.md)
- Feature preservation crosswalk: [FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md](FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md)
- Execution readiness: [WP_EXECUTION_READINESS_v0.91.8.md](WP_EXECUTION_READINESS_v0.91.8.md)
- ADR plan: [ADR_PLAN_v0.91.8.md](ADR_PLAN_v0.91.8.md)
- v0.92 handoff: [NEXT_MILESTONE_HANDOFF_v0.91.8.md](NEXT_MILESTONE_HANDOFF_v0.91.8.md)
- v0.92 activation test map: [V092_ACTIVATION_TEST_MAP_v0.91.8.md](V092_ACTIVATION_TEST_MAP_v0.91.8.md)
- Feature index: [features/README.md](features/README.md)
- Review index: [review/README.md](review/README.md)
- Third-party review handoff: [review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md](review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md)

## Non-Goals

- Do not implement v0.91.8 product changes in the setup issue.
- Do not delete incumbent ADL code from planning text.
- Do not claim `v0.92` birthday readiness before review and proof converge.
- Do not pull Runtime v3 or C-SDLC v2 ownership back into ADL core.

## Documentation Responsibility

WP-01 `#5594` owns the present readiness reconciliation. Closed v0.91.7 WP-21A
`#5489` is historical preparation evidence. Future v0.91.8 WP-21A `#5355`
revalidates the canonical packet at the release tail and fails closed if any
surface named in [CANONICAL_DOC_INVENTORY_v0.91.8.md](CANONICAL_DOC_INVENTORY_v0.91.8.md)
is missing, contradictory, stale against live issue truth, or presents planned
work as proven.
