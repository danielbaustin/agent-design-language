# v0.91.3 Internal Review Lane Errors

## Status

`active_error_register`

This file records errors made by the internal review process itself. These are
not product findings unless separately confirmed against the reviewed baseline.
They remain visible so WP-13 can be audited as a review process, not just as a
findings list.

## Errors

### RLE-001: Architecture lane reported stale crate-version evidence

- Source artifact: `.adl/reviews/v0.91.3/internal/wp-13/codebuddy/specialist_reviews/architecture_review.md`
- Original specialist finding: `P3: Review target is v0.91.3 but Rust crate manifest still reports v0.91.2`
- Claimed evidence: `adl/Cargo.toml:3`
- Correct baseline: review packet ref `985a0637`
- Actual state at reviewed baseline: `adl/Cargo.toml` and `adl/Cargo.lock` report `0.91.3`
- Classification: `review_lane_error`
- Product finding status: `invalid`
- Required synthesis behavior: do not include this as a product finding in `SPECIALIST_FINDINGS.md`, `FINDINGS_REGISTER.md`, `WP15_REMEDIATION_QUEUE.md`, or `REVIEW_REPORT.md`; include it only as review-process error evidence.

## Notes

Review-lane errors should be preserved, not hidden. If an error reveals a broader
review-method weakness, route that weakness separately as a review-quality
finding.
