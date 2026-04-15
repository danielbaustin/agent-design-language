# Release Notes - v0.89

## Metadata
- Product: `ADL`
- Version: `v0.89`
- Release date: `TBD`
- Tag: `v0.89`

## How To Use
- keep statements implementation-accurate and test-validated
- prefer concise bullets over marketing language
- explicitly separate shipped behavior from `What's Next`

# `ADL` `v0.89` Release Notes

## Summary

`v0.89` is the milestone where ADL turns governed adaptive execution into a first-class package. The current draft release story is that the convergence / gate / action / skill / experiment / ObsMem / security core has landed, while the quality, docs/review, remediation, and final release tail remain in flight.

## Current Draft Highlights
- AEE 1.0 convergence as a real bounded runtime contract
- Freedom Gate v2 plus explicit decision and action mediation surfaces
- canonical skill execution, experiment, and ObsMem evidence surfaces
- main-band security / trust / posture package now landed as the final core-band contract

## What's New In Detail

### Governed adaptive execution
- bounded convergence, stop conditions, and adaptation evidence
- richer judgment behavior through Freedom Gate v2

### Runtime authority and skills
- explicit decision and action boundaries
- canonical skill model and skill invocation protocol

### Evidence and security
- experiment records and evidence-aware ObsMem continuation
- threat model, trust model, and declared security posture package

## Upgrade Notes
- exact user-facing upgrade notes are still `TBD` until the quality/review/release tail lands
- this pre-release draft should still be rewritten from shipped proof surfaces during `WP-15` and `WP-16`

## Known Limitations
- this document is pre-release and should not be treated as a shipped-claims document yet
- the adversarial runtime/demo package is intentionally deferred to `v0.89.1`
- the remaining open work is release-tail convergence, not missing core-band feature scope

## Validation Notes
- final release notes must be updated from shipped proof surfaces only
- demo/review package and quality-gate outputs should be cited before release

## What's Next
- `v0.89.1` adversarial runtime and exploit/replay package
- later reasoning/signature/identity/governance bands continue after this milestone

## Exit Criteria
- notes reflect only shipped behavior
- known limitations and future work are explicitly separated
- final text is ready to paste into GitHub Release UI without further editing
