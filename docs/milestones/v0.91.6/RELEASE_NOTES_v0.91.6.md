# v0.91.6 Release Notes

## Metadata

- Product: ADL
- Version: `v0.91.6`
- Release date: not scheduled
- Tag: not assigned

## How To Use

Keep these notes implementation-accurate. `v0.91.6` has executed a substantial
bridge tranche, but these notes remain draft until the ordered release tail
finishes and the release ceremony approves final publication truth.

# ADL v0.91.6 Release Notes

## Summary

`v0.91.6` is the first bridge/readiness tranche before `v0.92`. It organizes
activation-critical surfaces into tracked planning, feature, review, and proof
surfaces so implementation and review can proceed without rediscovery.

The bridge tranche is materially landed through retained evidence for the
closed bridge umbrellas. The milestone is not release-complete yet; the active
frontier is the ordered release-tail sprint under `#4604`.

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

- These notes do not claim release publication yet.
- These notes do not claim full runtime/product integration from prerequisite
  proof.
- WP-11 through WP-19 release-tail work remains active until `#4604` closes.
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
