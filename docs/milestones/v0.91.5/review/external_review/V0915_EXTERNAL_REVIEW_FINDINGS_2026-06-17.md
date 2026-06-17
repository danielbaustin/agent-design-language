# v0.91.5 External Review Findings

Date: `2026-06-17`
Review type: third-party / external review
Milestone: `v0.91.5`
Input handoff: `docs/milestones/v0.91.5/review/external_review/V0915_EXTERNAL_REVIEW_HANDOFF_2026-06-17.md`
Status: `findings_recorded_for_wp18_remediation`

## Overall Assessment

The v0.91.5 handoff is substantially more honest and reviewable than earlier milestone packets. It clearly separates review prompts from asserted findings, records explicit non-claims, and preserves release-tail caveats instead of smoothing them over.

The strongest evidence surface in this review is the second-pass internal review. It found eight findings, including four P1s, and stated plainly that the milestone was not yet clean enough for external review without remediation. The external review therefore focused on whether the merged remediation PRs actually fixed the highest-severity findings in live code and tracked evidence.

The result is encouraging: the four P1 remediation paths were largely verified against the repository state. The review still found one narrowed residual issue in the `pr finish` closeout path and two lower-severity documentation/evidence concerns.

## Scope Reviewed

Reviewed surfaces included:

- the external review handoff packet;
- the second-pass internal review register and remediation queue;
- the live remediation surfaces for PRs `#3947` through `#3951`;
- PR validation rollup logic;
- OpenRouter redaction evidence;
- `pr finish` closeout / SOR staging logic;
- release-tail quality-gate documentation;
- root README positioning language.

This was not a full re-review of every file in the repository and was not a broad Rust behavior-equivalence proof across the refactor. The review emphasis was the milestone's central closeout claim: that the internal-review findings were found, routed, and remediated truthfully.

## Verified Remediation

### P1: PR validation latest-check handling

Status: `verified_fixed`

The PR validation rollup fix from `#3949` is real. The disposition logic now operates on effective PR validation checks after deduplicating by check name and keeping only the newest run by `job_run_id`. This means a superseded cancelled run no longer globally poisons the PR rollup when a newer run of the same check succeeded.

This fixes the class of problem where merged green PRs could be reported as cancelled due to stale check runs. The fix is at the right layer: deduplication happens before cancelled-state classification.

Residual note: the comparison relies on GitHub `job_run_id` monotonicity. That is a GitHub-assigned property rather than an ADL-controlled invariant. This is acceptable for the current implementation but should remain understood as an external API assumption.

### P1: OpenRouter raw prompt/output redaction

Status: `verified_fixed`

The OpenRouter redaction finding from `#3951` is fixed in tracked evidence. The raw prompt content was removed from the tracked lane-request artifact and replaced with review-safe metadata:

- character count;
- SHA-256 digest;
- prompt contract reference;
- redacted excerpt marker instead of raw content.

This matches the recommended remediation route and is verifiable from tracked repository state without relying on ignored local files.

### P1: Release-tail documentation truth

Status: `verified_fixed`

The docs-truth normalization from `#3947` landed for the quality gate. The quality gate now reflects the corrected release-tail state, including WP-15 / `#3579` as closed, while still keeping the release gate honestly blocked pending remaining review-tail work.

### P1: `pr finish` SOR truth staging

Status: `mostly_fixed_with_residual_risk`

The main `pr finish` staging-order fix from `#3950` is real. The finish flow now records docs-only validation evidence, syncs completed output surfaces, and then re-stages finish-written output truth before committing. This addresses the main path behind stale closeout-card truth in PR commits.

The surrounding hardening is also meaningful:

- local `.adl` issue-bundle staging is guarded;
- completed issue-bundle truth is checked for closed issues;
- validation subprocess environment handling strips sensitive GitHub token variables;
- heartbeat and observability tests exist for the touched surface.

However, one narrower residual remains and should be resolved or proven unreachable.

## Findings

### R1 - P2: `pr finish` can still skip committing finish-written truth on a narrowed path

The `pr finish` fix re-stages finish-written output truth after SOR mutation and output synchronization, but the final commit remains gated by a `has_uncommitted` value sampled before those later mutations and restaging steps.

If `has_uncommitted` is false at the earlier sampling point, and finish then mutates/syncs/re-stages output truth afterward, the final commit can be skipped even though finish-created truth has been staged. That would leave the re-staged SOR/output truth uncommitted.

This is a narrower form of the original bug class: finish-written lifecycle truth can still fail to land in the commit under a specific sequencing condition.

Recommended remediation:

- Recompute the commit-needed state after finish-written output truth is synced and re-staged; or
- make finish-written output truth itself force the commit path; or
- add a focused regression proving the suspected path is unreachable in normal `pr finish` execution.

Acceptance bar:

- A focused test demonstrates that docs-only finish evidence written during `pr finish` is included in the created commit even when no other user-authored paths were dirty before finish mutation.

### R2 - P3: LAN endpoint fixture cleanup remains open

The internal review already recorded LAN endpoint fixture concerns, and this external review did not find evidence that the lower-severity fixture cleanup has been fully resolved.

This is not a release blocker by itself, but it should remain routed rather than disappearing behind the P1 remediation wave.

Recommended remediation:

- Keep the LAN endpoint fixture concern in the WP-18 remediation register or explicitly defer it with owner, rationale, and target milestone.

### R3 - P3: Root README contains unverifiable positioning language

The root README now includes broad framing language around the "Mythos problem" and frontier vulnerability-finding systems. That language may be directionally useful, but it reads more like aspirational positioning than source-backed engineering truth.

For a milestone that has otherwise worked hard to separate claims from evidence, this is claim-inflation risk.

Recommended remediation:

- Either ground the language with a tracked feature/strategy reference, or soften it so the README remains an evidence-bound engineering entrypoint rather than a marketing surface.

## Refactor Behavior-Preservation Caveat

This review verified high-severity remediation sites and selected release-tail truth surfaces. It did not verify behavior equivalence across the refactor by running the test suite against both pre-refactor and post-refactor trees.

Given the milestone's refactoring emphasis, CI being green is useful but not sufficient by itself to prove behavior preservation unless the relevant assertions are known to have run and passed. Before WP-18 closes, the project should confirm that behavior-preservation proof is either:

- already covered by focused and CI validation, with evidence; or
- explicitly deferred with rationale; or
- run as part of WP-18 final preflight.

## What Looks Strong

- The second-pass internal review was unusually strong and candid.
- The review found failures in previous fixes instead of treating closure as proof.
- The redaction breach was recorded as a breach against ADL's own contract rather than minimized.
- The P1 remediation PRs mostly fixed the live code/evidence surfaces they claimed to fix.
- The release-tail handoff is honest about non-claims and remaining gates.

## Required WP-18 Routing

WP-18 should consume this review as follows:

1. Treat R1 as the main actionable remediation candidate.
2. Route R2 as either cleanup or explicit deferral.
3. Decide whether R3 is accepted, softened, or intentionally retained with a source reference.
4. Confirm behavior-preservation evidence for the refactor before release closeout.
5. Keep WP-18 open until these dispositions are recorded truthfully.

## Non-Claims

This review does not claim:

- v0.91.5 is release-ready;
- WP-18 may close immediately;
- the full Rust refactor has been behavior-equivalence proven;
- every documentation surface was reviewed exhaustively;
- all lower-severity internal-review issues were independently reverified.

## Suggested Next Step

Create or update WP-18 remediation records for R1 through R3, then run a final focused preflight that includes the refactor behavior-preservation evidence decision.
