# v0.91.6 Release Notes

## Metadata

- Product: ADL
- Version: `v0.91.6`
- Release date: ceremony pending
- Tag: `v0.91.6` pending ceremony approval

## How To Use

Keep these notes implementation-accurate. `v0.91.6` has executed a substantial
bridge tranche and is now in release ceremony. These notes are not a public
tag/GitHub Release publication claim until the ceremony script passes and the
operator approves mutating release actions.

# ADL v0.91.6 Release Notes

## Summary

`v0.91.6` is the first bridge/readiness tranche before `v0.92`. It organizes
activation-critical surfaces into tracked planning, feature, review, and proof
surfaces so implementation and review can proceed without rediscovery.

The bridge tranche is materially landed through retained evidence for the
closed bridge umbrellas. The ordered release tail is complete through WP-18; the
active frontier is WP-19 release ceremony under sprint umbrella `#4604`.

## Current Highlights

- Resilience, persistence, sleep/wake, and continuity proof route retained.
- Tooling proof-loop and logging/observability reliability route retained.
- Public prompt record export/redaction/indexing route retained.
- Provider/model reliability and multi-agent readiness route retained.
- ACIP/A2A/provider communications decision route retained.
- Security bridge and Continuous Adversarial Verification route retained.
- Identity/continuity, capability evidence, Observatory/Unity, AEE,
  Memory/ObsMem, and ACP accounting route retained.
- Control-plane rescue gate `#4588` completed before release-tail resumption.

## Known Limitations

- These notes do not claim public release publication yet.
- These notes do not claim full runtime/product integration from prerequisite
  proof.
- WP-11 through WP-18 are complete. WP-15 external review failed on stale
  handoff truth; WP-16 remediated the accepted findings, and that failed-review
  truth remains retained rather than rewritten as approval.
- WP-19 `#3984` and umbrella `#4604` remain open until ceremony and sprint
  closeout complete.
- `v0.91.7` remains required for second-tranche conceptual surfaces.
- `v0.92` activation remains blocked until bridge truth is reviewed.

## Validation Notes

Expected validation for this documentation package:

- docs existence check;
- `git diff --check`;
- placeholder and host-local path scan;
- bounded docs review.

## What's Next

- Complete `v0.91.7` residual bridge docs.
- Refresh `v0.92` activation docs from reviewed bridge truth.
- Open implementation issues only after doc and issue routes are clear.

## Exit Criteria

- Final notes reflect only shipped or reviewed behavior.
- Known limitations and future work remain explicitly separated.
