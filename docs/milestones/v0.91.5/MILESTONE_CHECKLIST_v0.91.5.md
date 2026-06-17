# v0.91.5 Milestone Checklist

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-02`
- Owner: ADL maintainers
- Status: `sprint_4_wp_19_active`

## Purpose

Track the minimum planning, execution, quality, release, and post-release
checks needed for truthful v0.91.5 closeout.

## Planning

- [x] v0.91.5 planning package exists.
- [x] WP-01 confirms planned issues, cards, sprint umbrellas, and initial sequencing are ready.
- [x] Canonical milestone WP ordering standard is closed through `#3567`.
- [x] Portable ADL project adapter planning is scheduled through `#3569`.
- [x] Sprint umbrella issues `#3571` through `#3574` are seeded.
- [x] Closeout-tail issues `#3575`, `#3579`, `#3576`, `#3580`,
  `#3577`, `#3581`, and `#3578` are seeded.
- [x] Reusable milestone WP ordering standard and template surface exist.
- [ ] Feature docs exist for all bridge work tracks.
- [ ] Open side issues are relabeled to `version:v0.91.5`.
- [ ] v0.91.4 docs say bridge work moved, not abandoned.
- [ ] v0.91.6 docs depend on v0.91.5 closeout and `#3377`.
- [ ] v0.91.7 second-tranche rule is explicit before v0.92 opens.
- [ ] All opened v0.91.5 issues use five prompt cards from the active registry.

## Execution Discipline

- [ ] `SIP`, `STP`, and `SPP` are design-time ready before execution starts.
- [ ] `SRP` records actual review prompts and findings.
- [ ] `SOR` records actual validation, integration, and closeout truth.
- [ ] Work executes in bound worktrees.
- [ ] Pre-PR review runs before publication.
- [ ] Scope stays within each issue or is explicitly routed.

## Quality Gates

- [x] Reusable coverage / quality-gate checklist is applied or explicitly
  routed:
  [QUALITY_GATE](QUALITY_GATE_v0.91.5.md)
- [x] Test coverage gap analysis is complete or truthfully deferred.
- [x] Rust module size tracker has been checked when available:
  `.adl/reports/manual/rust_module_watch_list.md`
- [x] Recent issue closeout truth is sampled or mechanically checked.
- [x] Internal review plan exists and is ready before internal review begins.
- [x] PVF lane health is recorded, including docs-only, runtime fast/default,
  slow-proof, authoritative coverage, and release-gate lanes.
- [x] Changed-file risk, test runtime regression, PR stack/base, docs truth,
  ADR readiness, demo/proof artifacts, security/redaction, and follow-on
  routing have blocker dispositions.
- [ ] Multi-agent C-SDLC workcell proof completes or blocks truthfully.
- [ ] OpenRouter/provider matrix work completes or blocks truthfully.
- [ ] Public prompt packet export/redaction/index path completes or blocks
  truthfully.
- [ ] `.adl` cleanup/archive decisions are reviewed before deletion.
- [ ] Demo readiness and Unity Observatory routing are explicit.
- [ ] v0.92 activation test map is complete and consumed by `#3377`.
- [ ] AEE completion tranche is reviewed and routed before v0.92 opens.

## Release Packaging

- [ ] Release plan complete.
- [ ] Release notes rewritten from landed evidence.
- [x] `#3929` GitHub transport-boundary cleanup is closed and consumed as
  release-tail tooling truth.
- [x] Second-pass internal review `#3923` is complete.
- [x] Refactor behavior-preservation evidence decision is recorded:
  [WP18_REFACTOR_BEHAVIOR_PRESERVATION_DECISION_2026-06-17.md](review/external_review/WP18_REFACTOR_BEHAVIOR_PRESERVATION_DECISION_2026-06-17.md)
- [x] External/third-party review `#3580` is complete or explicitly
  blocked/deferred with rationale.
- [x] Final remediation and pre-v0.92 routing `#3577` is complete or explicitly
  routed.
- [ ] v0.91.6 handoff complete, including v0.91.7 second-tranche criteria.
- [ ] 15-minute operator break after ceremony is recorded before v0.91.6 starts.
- [ ] v0.92 final preflight routing complete.
- [x] WP-18 conditional v0.92 preflight recorded:
  [WP18_EXTERNAL_REVIEW_REMEDIATION_DISPOSITION_2026-06-17.md](review/external_review/WP18_EXTERNAL_REVIEW_REMEDIATION_DISPOSITION_2026-06-17.md)
- [x] Review findings fixed or routed:
  [WP18_EXTERNAL_REVIEW_REMEDIATION_DISPOSITION_2026-06-17.md](review/external_review/WP18_EXTERNAL_REVIEW_REMEDIATION_DISPOSITION_2026-06-17.md)
- [ ] Release evidence package assembled.

## Post-Release

- [ ] v0.91.6 WP-01 inputs are linked.
- [ ] v0.91.7 and v0.92 routing inputs are linked where applicable.
- [ ] Deferred bridge items have owners and follow-on routing.
- [ ] Residual risks are recorded in release notes or handoff.

## Exit Criteria

- All required gates are checked or exceptions have owners.
- v0.91.6 can open from v0.91.5 closeout and `#3377`, with v0.91.7/v0.92
  routing explicit.
- Multi-agent completion/blocker truth is explicit.
