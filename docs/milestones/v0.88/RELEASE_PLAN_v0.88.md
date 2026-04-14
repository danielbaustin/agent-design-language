# Release Plan - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Owner: `Daniel Austin`

## Current State

`v0.88` is at the final ceremony gate now.

The current tracked work for `v0.88` is:
- completed implementation wave through `WP-13`
- completed quality / docs / review / remediation / next-milestone tail through `WP-19`
- only `WP-20` release ceremony remains open

## Purpose

Define the exact closeout contract for `v0.88`.

This is not just a reminder that a release will happen later. When the milestone reaches release tail, this doc should function as the executable ceremony and validation checklist for whether `v0.88` is actually ready to ship.

`v0.88` must ship real temporal, PHI, and instinct proof surfaces, not just aligned documents.

## Release Readiness (GO / NO-GO Gate)

All of the following must be true before ceremony:

- [x] canonical quality gate doc is current and truthful
- [x] milestone checklist is fully updated and honest
- [x] implementation issue wave for `WP-02` through `WP-13` is complete or explicitly deferred
- [x] each completed implementation WP produced concrete code, tests, artifacts, demos, or an explicit defer record
- [x] demo matrix rows map to real runnable commands and proof artifacts
- [x] D11 quality-gate walkthrough is runnable and aligned to the current CI and coverage posture
- [x] docs, WBS, demos, and implementation agree on the same bounded milestone
- [x] PHI, instinct, temporal schema, and execution-policy/cost claims are each proven in at least one reviewable surface
- [x] Paper Sonata is strong enough to act as a flagship public-facing demo without overclaiming autonomy

### Explicit GO / NO-GO Questions

Answer all before release:

- [x] Does the runtime expose temporal structure and execution posture in a reviewer-legible way?
- [x] Can a reviewer inspect at least one proof path for commitments / retrieval / causality behavior?
- [x] Is requested execution policy visibly related to realized cost?
- [x] Do PHI-style metrics produce a bounded, useful comparison rather than rhetoric?
- [x] Does instinct visibly affect routing or prioritization while remaining policy-bounded?
- [x] Is Paper Sonata strong enough to show publicly as a serious ADL multi-agent demo?
- [x] Can a reviewer find the milestone proof commands and artifacts from the demo matrix alone?

If any answer is NO, do not release.

## Branch And Tag Preparation

- [x] target branch confirmed
- [x] working tree clean
- [x] required PRs merged
- [x] version references updated if needed
- [x] tag plan prepared

## Intended Closeout Sequence

`v0.88` has completed the same bounded closeout sequence used in `v0.86` and `v0.87`:
- quality gate
- docs + review pass
- internal review
- 3rd-party review
- review findings remediation
- next milestone planning
- release ceremony

## Verification

Immediately before and after ceremony:

- [x] CI status checked on `main`
- [x] `bash adl/tools/demo_v088_quality_gate.sh` succeeds
- [x] release notes reviewed against actual milestone truth
- [x] demo commands run successfully from a clean checkout
- [x] proof artifacts are generated and inspectable
- [x] docs and feature surfaces still agree after final merges

If any failure occurs:
- open issue
- triage immediately
- decide patch vs defer before closing the milestone

## Communication

- [x] roadmap / status docs updated
- [x] milestone completion note prepared
- [x] any follow-on milestone links or deferred items captured clearly

## Post-Release Closure

- [ ] milestone issues closed with release reference
- [ ] deferred items moved forward explicitly
- [ ] follow-up bugs / design debts captured as issues
- [ ] next milestone package linked from the ceremony output

## Exit Criteria

The release is complete when:

- the temporal / PHI / instinct package is proven by runnable demos and inspectable artifacts
- docs match implementation and demo truth
- review findings are remediated or explicitly deferred
- next milestone planning is prepared before closeout
- release ceremony outputs are complete and the repo is ready for the next milestone
