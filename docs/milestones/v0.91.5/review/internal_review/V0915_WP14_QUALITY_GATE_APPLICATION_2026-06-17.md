# v0.91.5 WP-14 Quality Gate Application

Date: `2026-06-17`
Issue: `#3575`
Sprint: `#3574`
Scope: apply the reusable quality-gate checklist to the live v0.91.5 release
tail before docs/review alignment and second-pass internal review continue.

## Summary

Result: `blocked`

This is a successful truthful gate. The milestone is reviewable, but not ready
for release or Sprint 4 closeout. The main blockers are the still-open
release-tail issues, stale opening-era milestone-doc status lines, and a
sampled closeout-truth drift on recent closed issue `#3891`.

## Evidence Used

- `docs/milestones/v0.91.5/QUALITY_GATE_v0.91.5.md`
- `docs/milestones/v0.91.5/MILESTONE_CHECKLIST_v0.91.5.md`
- live issue truth for `#3531`, `#3574`, `#3575`, `#3576`, `#3577`, `#3578`,
  `#3579`, `#3580`, `#3581`, and `#3923`
- `bash adl/tools/report_large_rust_modules.sh --format tsv`
- tracked closed-issue card-truth evidence packet:
  - `docs/milestones/v0.91.5/review/internal_review/V0915_CLOSED_ISSUE_CARD_TRUTH_EVIDENCE_2026-06-17.md`

## Checklist Application

| Check | Status | Notes |
| --- | --- | --- |
| Test coverage gap analysis | `deferred` | No dedicated v0.91.5 coverage-gap packet was found in this sweep. This gate records the gap explicitly and routes it to internal review / WP-18 remediation instead of pretending coverage truth exists. |
| Rust module size tracker | `passed` | `bash adl/tools/report_large_rust_modules.sh --format tsv` ran successfully. The manual tracker path named in earlier docs was not present in this bound worktree, so the live report was used as the proving surface. |
| Issue closeout truth | `blocked` | Sampled closed issues show mixed truth. The tracked evidence packet preserves the historical local-card excerpts: `#3898` already shows a terminal `merged` integration state, while `#3891` still records `Integration state: worktree_only` and `Card Status: ready` despite the issue being closed in GitHub. |
| Internal review readiness | `passed` | A second-pass internal review plan is now recorded at `docs/milestones/v0.91.5/review/internal_review/V0915_SECOND_PASS_INTERNAL_REVIEW_PLAN_2026-06-17.md`. |
| PVF lane health | `deferred` | This docs/control-plane gate did not rerun lane executions. Existing lane contracts remain an input to WP-16 and WP-18, and no new lane-health proof was generated here. |
| Changed-file risk review | `passed` | Current high-risk areas remain GitHub/client workflow control, prompt-template/editor plumbing, provider/runtime breadth, and release-truth docs. This gate keeps those surfaces visible instead of collapsing them into a green release claim. |
| Test runtime regression | `deferred` | No standalone runtime-comparison packet was found in this sweep. This remains a useful future quality-gate activity but is not invented here. |
| Prompt/card lifecycle audit | `follow_on` | The active WP-14 issue bundle itself still reflects older WP-18 naming in local lifecycle records. That drift does not block this docs packet, but it should be normalized in issue-local records before final closeout. |
| PR stack/base hygiene | `passed` | At Sprint 4 kickoff there is no open WP-14 PR stack to unwind. The release tail is blocked by open issues and review sequencing, not by an active wrong-base PR chain. |
| Docs truth scan | `blocked` | The touched control docs are now updated, but the sweep still found stale opening-era status lines in `DECISIONS_v0.91.5.md`, `DESIGN_v0.91.5.md`, `README.md`, `RELEASE_NOTES_v0.91.5.md`, `RELEASE_PLAN_v0.91.5.md`, `VISION_v0.91.5.md`, `features/README.md`, `features/PUBLIC_PROMPT_RECORDS_v0.91.5.md`, and `features/V092_ACTIVATION_READINESS_v0.91.5.md`. Route these to WP-15 rather than claiming docs truth is clean. |
| ADR / decision readiness | `not_applicable` | This gate did not introduce a new architecture decision surface. |
| Demo/proof artifacts | `passed` | WP-14 consumes existing demo/proof evidence from prior sprint work and does not claim new demo execution. |
| Security/privacy/redaction | `passed` | No new exported secret-bearing or host-path-sensitive artifacts were introduced by this packet. |
| Follow-on issue hygiene | `follow_on` | Open release-tail issues remain correctly visible. Second-pass internal review `#3923` should be treated as an active prerequisite, not hidden optional work. |

## Rust Module Snapshot

Largest currently reported modules from the live tracker command:

1. `adl/src/cli/pr_cmd/github/tests.rs` at `1965`
2. `adl/src/cli/pr_cmd/finish_support.rs` at `1952`
3. `adl/src/csdlc_prompt_editor.rs` at `1812`
4. `adl/src/cli/tests/pr_cmd_inline/basics.rs` at `1771`
5. `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs` at `1683`

This gate records the tracker check; it does not create new refactor scope.

## Closeout Truth Sample

Tracked packet:

- `docs/milestones/v0.91.5/review/internal_review/V0915_CLOSED_ISSUE_CARD_TRUTH_EVIDENCE_2026-06-17.md`

Key comparison preserved there:

- `#3898`: historical sampled local record already showed `Integration state:
  merged` and `Status: DONE`.
- `#3891`: historical sampled local record remained stale against live issue
  truth, still recording `Integration state: worktree_only` and
  `Card Status: ready`.

Disposition:

- treat stale closed-issue SOR truth as a Sprint 4 remediation input before
  final release closeout
- do not claim the recent-closeout layer is uniformly clean yet

## Internal Review Readiness

Second-pass internal review may proceed after this WP-14 packet and the WP-15
docs/review alignment issue are in place. It should use:

- the first-pass findings register
- the first-pass remediation queue
- this WP-14 quality-gate application packet
- the current open-issue release tail under Sprint 4

## Bottom Line

v0.91.5 is ready to continue into WP-15 and second-pass internal review, but it
is not ready for release. The truthful gate result is `blocked`.
