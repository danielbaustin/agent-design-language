# WP-18 External Review Remediation Disposition

Date: `2026-06-17`
Milestone: `v0.91.5`
Issue: `#3577`
Status: `remediation_fixed_or_routed`

## Purpose

Record the WP-18 disposition for the v0.91.5 external review findings before
next-milestone planning and release ceremony work continues.

This packet does not release v0.91.5 and does not claim that WP-19 or WP-20 are
complete.

## Source Review

- External review findings:
  `docs/milestones/v0.91.5/review/external_review/V0915_EXTERNAL_REVIEW_FINDINGS_2026-06-17.md`
- External review handoff:
  `docs/milestones/v0.91.5/review/external_review/V0915_EXTERNAL_REVIEW_HANDOFF_2026-06-17.md`
- Second internal review remediation queue:
  `docs/milestones/v0.91.5/review/internal_review/V0915_SECOND_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-17.md`

## Finding Dispositions

| Finding | Severity | Disposition | Evidence |
| --- | --- | --- | --- |
| `R1` `pr finish` can skip committing finish-written truth on a narrowed path | `P2` | `fixed` | Issue `#3953`, PR `#3959`, merged `2026-06-17`. The finish path now recomputes dirty state after finish-written SOR/output truth is synced and re-staged, and includes a focused regression for docs-only finish evidence written during `pr finish`. |
| `R2` LAN endpoint fixture cleanup remains open | `P3` | `routed_to_v0.91.6` | Issue `#3954`, PR `#3957`, merged `2026-06-17`. The concern is explicitly deferred to `#3946` with non-blocking rationale because current public review packets normalize private LAN details. |
| `R3` Root README contains unverifiable positioning language | `P3` | `fixed` | Issue `#3955`, PR `#3960`, merged `2026-06-17`. The README language was softened to evidence-bound continuous-review/security wording. |
| Refactor behavior-preservation caveat | review caveat | `recorded` | Issue `#3956`, PR `#3958`, merged `2026-06-17`. Behavior-preservation decision packet added at `docs/milestones/v0.91.5/review/external_review/WP18_REFACTOR_BEHAVIOR_PRESERVATION_DECISION_2026-06-17.md`. |

## Related Already-Landed P1 Remediation

The external review also verified the earlier P1 remediation wave:

- `#3947`: release-tail documentation truth normalized.
- `#3949`: PR validation latest-check handling fixed.
- `#3950`: main `pr finish` SOR truth staging path fixed.
- `#3951`: OpenRouter raw prompt/output evidence redacted.

## v0.92 Preflight Disposition

WP-18 preflight status: `conditional_pass_for_next_milestone_planning`

The review findings known to WP-18 are fixed or routed. The remaining open
v0.91.5 work is release-tail sequencing rather than unresolved external-review
remediation:

- `#3581` WP-19 next milestone planning remains open.
- `#3578` WP-20 release ceremony remains open.
- `#3574` Sprint 4 umbrella remains open.

The v0.92 activation map remains the handoff surface for birthday readiness:
`docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`.

## Residual Risks

- The LAN endpoint fixture cleanup is intentionally deferred to v0.91.6 through
  `#3946`; it should not be allowed to re-enter publication proof packets
  without sanitization.
- Historical manual GitHub provenance cleanup remains routed to v0.91.6 through
  `#3945`.
- v0.92 readiness still depends on WP-19 planning and WP-20 release ceremony
  completion.

## Non-Claims

- This packet does not claim v0.91.5 is released.
- This packet does not close WP-19, WP-20, or Sprint 4.
- This packet does not prove full Rust refactor behavior equivalence beyond the
  recorded behavior-preservation decision and CI/focused validation evidence.
