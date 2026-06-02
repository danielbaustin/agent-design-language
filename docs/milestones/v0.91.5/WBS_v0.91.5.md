# v0.91.5 Work Breakdown Structure

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-02`
- Owner: ADL maintainers
- Status: `active_wp_01_opening`
- Canonical WP standard: [ADL_MILESTONE_WP_ORDERING_STANDARD.md](../../planning/ADL_MILESTONE_WP_ORDERING_STANDARD.md)

## Status

Active WBS for v0.91.5 WP-01 opening, issue seeding, and sprint readiness.

## How To Use

Use this WBS as the source of truth for v0.91.5 sequencing until concrete
issues are opened or adjusted. If issue reality diverges, update this WBS and
the issue wave together.

## WBS Summary

v0.91.5 contains four sprint bands after WP-01: prompt-template/public prompt
records and portable ADL readiness, provider/model and multi-agent proof, demo
matrix/showcase refresh, and coverage/review/remediation/next-milestone/release
closeout.

## Candidate WP Sequence

| WP | Work Package | Primary deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 | Milestone opening and issue-wave readiness | Active planning package, issue/card/sprint readiness, canonical WP ordering scheduling, and portable ADL adapter planning route. | v0.91.4 release closeout complete. |
| WP-02 | Prompt-template values renderer | Deterministic prompt-template value filling and static validation path. | WP-01. |
| WP-03 | Public prompt transition umbrella | Public prompt transition plan and child readiness. | WP-02. |
| WP-04 | Prompt packet exporter | Exporter for public C-SDLC prompt packets. | WP-03. |
| WP-05 | `.adl` inventory and archive disposition | Inventory, archive plan, and review-before-delete policy. | WP-03. |
| WP-06 | Prompt packet pilot and reviewer index | Pilot packets and reviewer-facing index. | WP-04, WP-05. |
| WP-07 | Prompt packet validation and redaction gates | Validation/redaction checks. | WP-06. |
| WP-08 | OpenRouter provider | OpenRouter provider or bounded implementation plan. | WP-01. |
| WP-09 | Provider/model test matrix | Hosted, local Ollama, remote Ollama, and OpenRouter role matrix. | WP-08. |
| WP-10 | Multi-agent usefulness checklist | Reviewer checklist and usefulness criteria. | WP-09. |
| WP-11 | Multi-agent workcell proof | Bounded workcell proof; consumes closed parallel hosted Codex lane evidence. | WP-09, WP-10. |
| WP-12 | Single-agent vs multi-agent overhead comparison | Timing and coordination comparison. | WP-10. |
| WP-13 | Demo matrix / demo showcase refresh | Demo matrix, demo showcase refresh, and demo proof/routing map. | WP-12. |
| WP-14 | Coverage / quality gate | Quality gate, coverage gap analysis, module-size tracker check, issue closeout check, and review-readiness gate. | WP-13. |
| WP-15 | Docs + review alignment | Docs, reviewer entrypoints, and internal-review handoff aligned to milestone truth. | WP-13, WP-14. |
| WP-16 | Internal review | Internal review and finding register. | WP-15. |
| WP-17 | External / 3rd-party review | External review handoff/result or explicit blocked/deferred review record. | WP-16. |
| WP-18 | Review findings remediation + final v0.92 preflight | Finding dispositions, AEE routing, activation ledger consumption, and v0.92 go/no-go. | WP-16, WP-17. |
| WP-19 | Next milestone planning | v0.92 opening/handoff planning and follow-on routing. | WP-18. |
| WP-20 | Release ceremony | Release evidence and closeout. | WP-19. |

## Work Packages

The candidate WP sequence above is reflected in
[WP_ISSUE_WAVE_v0.91.5.yaml](WP_ISSUE_WAVE_v0.91.5.yaml). Existing moved
issues should be reused rather than duplicated.

## Canonical Milestone Structure

v0.91.5 follows the ADL milestone script from prior milestones and now points
to the reusable standard at
[ADL_MILESTONE_WP_ORDERING_STANDARD.md](../../planning/ADL_MILESTONE_WP_ORDERING_STANDARD.md):

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
- `#3582` owns the downstream card rewrite/normalization pass after prompt
  templates v1.1 lands. WP-01 records that route instead of blocking on
  polishing every downstream card before the new templates exist.

## Sprint Umbrella Issues

| Sprint | Umbrella issue | Scope | Child issues |
| --- | --- | --- | --- |
| Sprint 1 | `#3571` | Prompt-template/public prompt records and portable ADL readiness. | `#3553`, `#3582`, `#3476`, `#3472`, `#3473`, `#3474`, `#3475`, `#3569` |
| Sprint 2 | `#3572` | Provider/model breadth and multi-agent proof. | `#3505`, `#3501`, `#3504`, `#3415`, `#3503`; `#3484` satisfied evidence |
| Sprint 3 | `#3573` | Demo matrix and demo showcase refresh. | `#3455`; supporting inputs `#3460`, `#3461` |
| Sprint 4 | `#3574` | Coverage, review, remediation, next-milestone planning, and release. | `#3575`, `#3579`, `#3576`, `#3580`, `#3577`, `#3581`, `#3578`; supporting inputs `#3531`, `#3502`, `#3534`, `#3377` |

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
  Sprint umbrellas are `#3571` through `#3574`; closeout-tail issues are
  `#3575`, `#3579`, `#3576`, `#3580`, `#3577`, `#3581`, and `#3578`.
- WP-01 explicitly routes downstream card rewriting through `#3582` after
  `#3553` lands, so old card scaffolds are not treated as final execution
  truth and do not block opening the milestone.
- WP-02 through WP-07 -> prompt-template and public prompt records are safe to review.
- WP-08 through WP-12 -> provider/model and multi-agent usefulness are tested.
- WP-13 -> demo matrix and showcase readiness are explicit.
- WP-14 through WP-20 -> coverage/quality, docs/review alignment, internal
  review, external review, remediation/final v0.92 preflight, next-milestone
  planning, and release close the bridge in the standard order.

## Exit Criteria

- Every work package has a seeded issue, moved issue, or explicit routing
  decision.
- The closeout tail preserves review, remediation, final preflight, and release.
- v0.92 can consume v0.91.5 without reconstructing intent from chat.
