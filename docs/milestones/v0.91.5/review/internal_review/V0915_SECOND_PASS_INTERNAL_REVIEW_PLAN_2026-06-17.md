# v0.91.5 Second-Pass Internal Review Plan

Date: `2026-06-17`
Sprint anchor: `#3574`
Review issues: `#3576` and `#3923`
Prepared by: WP-14 `#3575`

## Purpose

Provide a concrete, reviewable second-pass internal-review handoff so Sprint 4
does not rely on chat memory or stale first-pass assumptions.

## Scope

Review the live v0.91.5 release tail after first-pass remediation, with focus
on:

- workflow and GitHub-client control-plane truth
- prompt-template and lifecycle-record truth
- provider/multi-agent proof claims
- demo/proof coverage and milestone-control docs
- release-tail blocker routing for WP-17 through WP-20

## Source Surfaces

- `docs/milestones/v0.91.5/QUALITY_GATE_v0.91.5.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_FIRST_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-16.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_FIRST_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-16.md`
- `docs/milestones/v0.91.5/MILESTONE_CHECKLIST_v0.91.5.md`
- `docs/milestones/v0.91.5/SPRINT_v0.91.5.md`
- live Sprint 4 issue set: `#3574`, `#3575`, `#3579`, `#3576`, `#3580`,
  `#3577`, `#3581`, `#3578`, and `#3923`

## Review Lanes

1. Code/control-plane lane
   Check correctness and lifecycle truth for workflow tooling, GitHub transport,
   PR/issue operations, and closeout behavior changed during v0.91.5.
2. Test/validation lane
   Check whether validation and proof claims are proportionate to the changed
   surfaces and whether known gaps are explicitly routed.
3. Docs/release-truth lane
   Check milestone docs, checklists, proof packets, release-tail sequencing,
   and blocker language for stale or misleading claims.
4. Synthesis lane
   Merge findings into one current register with severity, disposition, and
   WP-18 or follow-on routing.

## Entry Gate

Start second-pass internal review only when:

- WP-14 quality gate packet exists
- WP-15 docs/review alignment is at least started
- the first-pass findings register and remediation queue remain available as
  source truth

## Output Expectations

- updated second-pass findings register
- explicit list of fixed vs unresolved first-pass findings
- residual-risk summary for WP-18 final preflight
- no false release-ready claim while Sprint 4 tail remains open

## Non-goals

- do not perform release ceremony
- do not silently remediate findings inside the review packet
- do not collapse external review into internal review

## Reviewer Handoff Notes

- treat the WP-14 gate result as `blocked`, not failed
- keep findings-first reporting
- route newly discovered substantive defects to WP-18 or follow-on issues
  rather than hiding them inside review prose
