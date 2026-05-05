# WP-22 Quality Gate Review - v0.90.5

## Finding

### `IR-001` P1

The canonical authoritative coverage posture on `main` remains red.

## Evidence

- `QUALITY_GATE_v0.90.5.md` records push-to-main run `25272620889`
- `adl-ci`: success
- `adl-coverage`: failure at coverage-policy enforcement

## Review Interpretation

This is a release-tail blocker, not a hidden pass. The milestone may proceed
to external review, but must not be described as release-ready while this
exception remains open.
