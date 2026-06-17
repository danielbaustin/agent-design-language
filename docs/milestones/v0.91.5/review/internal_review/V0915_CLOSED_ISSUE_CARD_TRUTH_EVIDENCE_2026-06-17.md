# v0.91.5 Closed-Issue Card-Truth Evidence

Date: `2026-06-17`
Issue: `#3942`
Related milestone issue: `#3575`
Purpose: preserve the WP-14 closed-issue card-truth blocker evidence in a
tracked packet instead of relying on ignored local issue-bundle paths.

## Summary

This packet records the exact closed-issue truth mismatch that kept the
`v0.91.5` quality gate blocked during the 2026-06-17 WP-14 application pass.

It preserves:

- the live GitHub closeout truth for sampled issues `#3891` and `#3898`
- the historically sampled local-card fields that informed the blocker
- the reason the blocker was real on 2026-06-17 without treating the sampled
  local card text as a portable canonical source

It intentionally does not publish raw ignored `.adl` card paths, absolute host
paths, or secrets.

## Scope Boundary

This packet is only the durable evidence sidecar for the closed-issue
card-truth blocker cited by:

- `docs/milestones/v0.91.5/QUALITY_GATE_v0.91.5.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md`

It does not reopen `#3891` or `#3898`, does not normalize their retained local
cards, and does not widen into broader release-tail closeout repair.

## Live GitHub Issue Truth

The current live issue state shows both sampled issues are closed:

| Issue | Current state | Closed at | Closing PR | Interpretation |
| --- | --- | --- | --- | --- |
| `#3891` | `CLOSED` | `2026-06-17T07:47:26Z` | `#3901` | The issue is terminal in GitHub, so any sampled local card still claiming in-flight integration truth is historical residue rather than live workflow state. |
| `#3898` | `CLOSED` | `2026-06-17T08:51:08Z` | `#3906` | The issue is also terminal in GitHub and provides a comparison point for a more-normalized sampled local record. |

## Historical Local-Card Excerpts Preserved Here

During the 2026-06-17 WP-14 quality-gate application, the reviewer sampled the
ignored local SOR snapshots for `#3891` and `#3898`. This packet keeps only the
minimal excerpted fields needed for the blocker claim.

| Issue | Historical sampled fields | Why it matters |
| --- | --- | --- |
| `#3891` | `Status: DONE`; `Card Status: ready`; `Integration state: worktree_only` | This combination conflicts with the now-closed GitHub issue state because it still reads like a pre-publication or pre-closeout record. |
| `#3898` | `Status: DONE`; `Integration state: merged` | This sample shows the same review sweep could observe a more-terminal local record, which is why the blocker was mixed closeout truth rather than a claim that every sampled closed issue was stale. |

## Why The Blocker Was Truthful

On 2026-06-17, the WP-14 gate was correct to remain blocked because:

1. the sampled local closeout layer did not tell one uniformly terminal story;
2. `#3891` still exposed retained historical card fields associated with an
   in-flight issue state; and
3. release-tail docs should not pretend that closeout truth is uniformly clean
   when sampled evidence still shows residue.

This remains a historical blocker-evidence packet, not a claim that GitHub
issue truth was wrong.

## Durable Reference Contract

Quality-gate and review packets should reference this tracked packet for the
closed-issue card-truth blocker instead of citing ignored local card paths
directly.

If future work fully normalizes the retained local closeout layer, that work
should update the active quality-gate packet and cite this document as
historical context rather than deleting the blocker evidence.

## Redaction And Path Hygiene

- No absolute host paths are recorded.
- No secret values or token-bearing command lines are recorded.
- No ignored local card paths are used as the durable reference surface.
- Only minimal field excerpts needed for the blocker claim are preserved.
