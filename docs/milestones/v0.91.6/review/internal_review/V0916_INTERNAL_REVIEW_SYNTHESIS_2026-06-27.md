# v0.91.6 Internal Review Synthesis

Date: `2026-06-27`
Owner issue: `#4582`
Status: `findings_first_synthesis`

## Findings Summary

WP-14A found three P1 findings, four P2 findings, and two P3 findings. The P1s
are release-tail control findings, not evidence that v0.91.6 failed:

1. WP-13 merged, but some reviewer-facing docs still present WP-13 as pending or
   pre-publication.
2. v0.91.6 remains a bridge milestone; v0.92 activation cannot proceed from the
   current evidence without consuming the burn-down checklist and v0.91.7 gates.
3. `pr finish` can lose numbered SRP findings when emitting machine-readable SOR
   facts, which weakens release-tail truth automation.

## Overall Assessment

`v0.91.6` is in materially better shape than earlier in the milestone. The repo
now contains a broad retained evidence tree, completed sprint reviews, runtime
soak proof boundaries, C-SDLC adoption audits, PR/tooling hardening packets,
provider/model suitability evidence, security packets, and a release-tail issue
sequence.

The remaining problem is not absence of work. It is release-tail truth hygiene:
current-state docs, checklist rows, PR inventory tooling, SOR fact extraction,
runtime heartbeat cursor semantics, and activation-surface classification must
be finished before external review and ceremony can be trusted.

## Code Review Note

The expanded code review inventoried the repository and inspected high-risk
executable surfaces: PR lifecycle/finish machinery, SRP/SOR fact emission,
runtime AWS signaling, prompt/card workflow control, GitHub transport, and
large control-plane modules. It found one P1 workflow-truth bug, one P2 runtime
heartbeat state bug, and one P3 maintainability risk. The full code-review pass
is recorded in:

- `docs/milestones/v0.91.6/review/internal_review/V0916_FULL_CODE_REVIEW_2026-06-27.md`

## Recommended Outcome

Proceed to WP-15 only after the WP-14A PR publishes these artifacts and the
operator accepts that the P1 findings are explicitly handed to WP-16 remediation
and final preflight. If external review should see a cleaner package, fix the
stale WP-13 state and SOR fact extraction issue before starting `#3980`.
