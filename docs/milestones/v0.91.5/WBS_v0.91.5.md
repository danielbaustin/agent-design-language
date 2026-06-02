# v0.91.5 Work Breakdown Structure

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-02`
- Owner: ADL maintainers
- Status: `active_wp_01_opening`

## Status

Active WBS for v0.91.5 WP-01 opening, issue seeding, and sprint readiness.

## How To Use

Use this WBS as the source of truth for v0.91.5 sequencing until concrete
issues are opened or adjusted. If issue reality diverges, update this WBS and
the issue wave together.

## WBS Summary

v0.91.5 contains six work bands: AEE completion-tranche routing, public prompt
transition, provider/model matrix, multi-agent proof, demo readiness, and v0.92
activation preflight.

## Candidate WP Sequence

| WP | Work Package | Primary deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 | Milestone opening and issue-wave readiness | Active planning package, issue/card/sprint readiness, canonical WP ordering scheduling, and portable ADL adapter planning route. | v0.91.4 release closeout complete. |
| WP-02 | Public prompt transition umbrella | Public prompt transition plan and child readiness. | WP-01. |
| WP-03 | Prompt packet exporter | Exporter for public C-SDLC prompt packets. | WP-02. |
| WP-04 | `.adl` inventory and archive disposition | Inventory, archive plan, and review-before-delete policy. | WP-02. |
| WP-05 | Prompt packet pilot and reviewer index | Pilot packets and reviewer-facing index. | WP-03, WP-04. |
| WP-06 | Prompt packet validation and redaction gates | Validation/redaction checks. | WP-05. |
| WP-07 | OpenRouter provider | OpenRouter provider or bounded implementation plan. | WP-01. |
| WP-08 | Provider/model test matrix | Hosted, local Ollama, remote Ollama, and OpenRouter role matrix. | WP-07. |
| WP-09 | Multi-agent usefulness checklist | Reviewer checklist and usefulness criteria. | WP-08. |
| WP-10 | Multi-agent workcell proof | Bounded workcell proof. | WP-08, WP-09. |
| WP-11 | Parallel hosted Codex lanes | Complete bounded issue execution with parallel hosted lanes. | WP-10. |
| WP-12 | Single-agent vs multi-agent overhead comparison | Timing and coordination comparison. | WP-10. |
| WP-13 | Demo showcase refresh | Demo index and deferred creative demo routing. | WP-01. |
| WP-14 | Celestial Rescue / Unity Observatory readiness | Demo artifact or readiness decision. | WP-13. |
| WP-15 | Demo showcase proof map | Demo proof index. | WP-13, WP-14. |
| WP-16 | v0.92 activation-test ledger | Complete activation-test map and deferred-work ledger. | WP-01. |
| WP-17 | AEE completion tranche and v0.92 launch packet | AEE closure-routing plan plus `#3377` go/no-go packet. | WP-10, WP-15, WP-16. |
| WP-18 | Docs, quality-gate checklist, and release-truth pass | Aligned bridge and v0.92 planning docs, plus reusable quality-gate checklist integration. | WP-17. |
| WP-19 | Internal review | Internal review and finding register. | WP-18. |
| WP-20 | Remediation and v0.92 final preflight | Finding dispositions and v0.92 go/no-go. | WP-19. |
| WP-21 | Release ceremony | Release evidence and closeout. | WP-20. |

## Work Packages

The candidate WP sequence above is reflected in
[WP_ISSUE_WAVE_v0.91.5.yaml](WP_ISSUE_WAVE_v0.91.5.yaml). Existing moved
issues should be reused rather than duplicated.

## Canonical Milestone Structure

v0.91.5 follows the ADL milestone script from prior milestones:

- `WP-01` is always the milestone planning and setup gate. It closes only after
  planned issues, cards, sprint umbrellas, and initial sequencing are ready to
  begin.
- Execution work is grouped into sprint bands, with sprint issues or umbrellas
  where the band has multiple child issues.
- Up to six execution sprints may exist before the closeout tail.
- The closeout-tail WP numbers may shift depending on the number of working
  WPs, but the closeout-tail order must not shift: demo matrix / showcase,
  coverage-quality gate, docs-review alignment, internal review, external /
  third-party review, review-findings remediation plus final preflight when
  applicable, next-milestone planning, and release ceremony.
- `#3567` owns the reusable template/standard for this milestone structure.
- `#3569` owns the portable ADL project adapter contract and templates; WP-01
  schedules it, but adapter implementation does not become WP-01 work.

## Sequencing

1. Route issue truth first.
2. Finish public prompt-record transition before `.adl` cleanup.
3. Add provider/model breadth before claiming multi-agent usefulness.
4. Run multi-agent proof before relying on it for v0.92.
5. Prepare demos and activation readiness before first-birthday preflight.
6. Route AEE completion explicitly before v0.92 consumes the bridge.
7. Review, remediate, preflight v0.92, then release.

## Sequencing Notes

The exact WP count is not process-critical. The sequence is critical. Public
records must not be published before redaction gates, multi-agent proof must
not overclaim usefulness, and v0.92 must not open before activation readiness
is explicit.

## Acceptance Mapping

- WP-01 -> bridge scope is visible in docs and GitHub labels; all scheduled
  issues/cards/sprint umbrellas are ready to begin or explicitly routed.
- WP-02 through WP-06 -> public prompt records are safe to review.
- WP-07 through WP-12 -> provider/model and multi-agent usefulness are tested.
- WP-13 through WP-15 -> demo readiness is explicit.
- WP-16 through WP-17 -> v0.92 activation, AEE closure routing, and
  first-birthday readiness are ready for WP-01 consumption.
- WP-18 through WP-21 -> docs, quality-gate checklist application, review,
  remediation, preflight, and release close the bridge.

## Exit Criteria

- Every work package has a seeded issue, moved issue, or explicit routing
  decision.
- The closeout tail preserves review, remediation, final preflight, and release.
- v0.92 can consume v0.91.5 without reconstructing intent from chat.
