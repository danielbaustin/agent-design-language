# v0.91.5 Work Breakdown Structure

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-02`
- Owner: ADL maintainers
- Status: `first_internal_review_remediation_active`
- Canonical WP standard: [ADL_MILESTONE_WP_ORDERING_STANDARD.md](../../planning/ADL_MILESTONE_WP_ORDERING_STANDARD.md)

## Status

Active WBS for post-Sprint 1 execution, the first internal-review remediation
execution under `#3899`, and the remaining Sprint 3 / Sprint 4 execution bands.

## How To Use

Use this WBS as the source of truth for v0.91.5 sequencing. If live issue
truth diverges from the milestone plan, update this WBS and the issue wave
together.

## WBS Summary

v0.91.5 contains four sprint bands after WP-01: prompt-template/public prompt
records and portable ADL readiness, provider/model and multi-agent proof, demo
matrix/showcase refresh, and coverage/review/remediation/next-milestone/release
closeout. The current retained execution-prep state is the first internal
review remediation queue `#3899`, which stages the initial WP-18 remediation
wave before the remaining closeout-tail issues advance. Live execution also
surfaced repo-native tooling gaps that must be captured as WP-18 remediation
inputs rather than lost as operator-side residue.

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
- Closed issue `#3567` owns the reusable template/standard for this milestone
  structure.
- `#3569` owns the portable ADL project adapter contract and templates; WP-01
  schedules it, but external repository migration does not become WP-01 work.
- `#3582` owns the downstream card rewrite/normalization pass after prompt
  templates, structure schemas, and field-level values editing land. Its audit
  records phase-aware validation and avoids polishing every downstream card
  when no lifecycle-truth defect exists.

## Sprint Umbrella Issues

| Sprint | Umbrella issue | Scope | Child issues |
| --- | --- | --- | --- |
| Sprint 1 | `#3571` | Prompt-template/public prompt records and portable ADL readiness. | `#3553`, `#3582`, `#3476`, `#3472`, `#3473`, `#3474`, `#3475`, `#3569` |
| Sprint 2 | `#3572` | Provider/model breadth and multi-agent proof. | `#3505`, `#3501`, `#3504`, `#3415`, `#3503`; `#3484` satisfied evidence |
| Sprint 3 | `#3573` | Demo matrix and demo showcase refresh. | `#3455`; supporting inputs `#3460`, `#3461` |
| Sprint 4 | `#3574` | Coverage, review, remediation, next-milestone planning, and release. | `#3575`, `#3579`, `#3576`, `#3580`, `#3577`, `#3581`, `#3578`; supporting inputs `#3531`, `#3502`, `#3534`, `#3377` |

## Queued Review-Remediation Mini-Sprint

- `#3899` is the queued first internal-review remediation umbrella created from
  WP-16 review output.
- Its child execution order is `#3891`, `#3892`, `#3893`, parallel-safe
  `#3894` / `#3895`, then parallel-safe `#3896` / `#3897` / `#3898`.
- This queue is a bounded staging wave under WP-18 and does not replace the
  canonical Sprint 4 umbrella `#3574`.

## Sequencing

1. Route issue truth first.
2. Finish public prompt-record transition before `.adl` cleanup.
3. Add provider/model breadth before claiming multi-agent usefulness.
4. Run multi-agent proof before relying on it for v0.92.
5. Prepare demos and activation readiness before first-birthday preflight.
6. Route AEE completion explicitly before v0.92 consumes the bridge.
7. Run the first internal-review remediation queue `#3899`.
8. Review, remediate, preflight v0.92, then release.

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
- WP-01 explicitly routes downstream card rewriting through `#3582`, so old
  card scaffolds are validated or dispositioned before later sprint work
  consumes them.
- WP-02 through WP-07 -> prompt-template and public prompt records are safe to review.
- WP-08 through WP-12 -> provider/model and multi-agent usefulness are tested.
- WP-13 -> demo matrix and showcase readiness are explicit.
- WP-14 through WP-20 -> coverage/quality, docs/review alignment, internal
  review, external review, remediation/final v0.92 preflight, next-milestone
  planning, and release close the bridge in the standard order. The first
  remediation tranche is currently staged through `#3899` and child issues
  `#3891` through `#3898` before the broader WP-18 final-preflight issue
  `#3577` should claim completion.

## Exit Criteria

- Every work package has a seeded issue, moved issue, or explicit routing
  decision.
- The closeout tail preserves review, remediation, final preflight, and release.
- v0.92 can consume v0.91.5 without reconstructing intent from chat.
