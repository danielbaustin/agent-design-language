# Release Plan - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Owner: `Daniel Austin`

## Current State

`v0.88` is in release-tail execution now.

The current tracked work for `v0.88` is:
- completed implementation wave through `WP-13`
- active quality / docs / review / release tail through `WP-14` to `WP-20`
- next-milestone planning already underway under `#1662`

## Purpose

Define the exact closeout contract for `v0.88`.

This is not just a reminder that a release will happen later. When the milestone reaches release tail, this doc should function as the executable ceremony and validation checklist for whether `v0.88` is actually ready to ship.

`v0.88` must ship real temporal, PHI, and instinct proof surfaces, not just aligned documents.

## Release Readiness (GO / NO-GO Gate)

All of the following must be true before ceremony:

- [ ] milestone checklist is fully updated and honest
- [x] implementation issue wave for `WP-02` through `WP-13` is complete or explicitly deferred
- [ ] each completed implementation WP produced concrete code, tests, artifacts, demos, or an explicit defer record
- [ ] demo matrix rows map to real runnable commands and proof artifacts
- [ ] docs, WBS, demos, and implementation agree on the same bounded milestone
- [ ] PHI, instinct, temporal schema, and execution-policy/cost claims are each proven in at least one reviewable surface
- [ ] Paper Sonata is strong enough to act as a flagship public-facing demo without overclaiming autonomy

### Explicit GO / NO-GO Questions

Answer all before release:

- [ ] Does the runtime expose temporal structure and execution posture in a reviewer-legible way?
- [ ] Can a reviewer inspect at least one proof path for commitments / retrieval / causality behavior?
- [ ] Is requested execution policy visibly related to realized cost?
- [ ] Do PHI-style metrics produce a bounded, useful comparison rather than rhetoric?
- [ ] Does instinct visibly affect routing or prioritization while remaining policy-bounded?
- [ ] Is Paper Sonata strong enough to show publicly as a serious ADL multi-agent demo?
- [ ] Can a reviewer find the milestone proof commands and artifacts from the demo matrix alone?

If any answer is NO, do not release.

## Branch And Tag Preparation

- [ ] target branch confirmed
- [ ] working tree clean
- [ ] required PRs merged
- [ ] version references updated if needed
- [ ] tag plan prepared

## Intended Closeout Sequence

`v0.88` is currently traversing the same bounded closeout sequence used in `v0.86` and `v0.87`:
- quality gate
- docs + review pass
- internal review
- 3rd-party review
- review findings remediation
- next milestone planning
- release ceremony

## Verification

Immediately before and after ceremony:

- [ ] CI status checked on `main`
- [ ] release notes reviewed against actual milestone truth
- [ ] demo commands run successfully from a clean checkout
- [ ] proof artifacts are generated and inspectable
- [ ] docs and feature surfaces still agree after final merges

If any failure occurs:
- open issue
- triage immediately
- decide patch vs defer before closing the milestone

## Communication

- [ ] roadmap / status docs updated
- [ ] milestone completion note prepared
- [ ] any follow-on milestone links or deferred items captured clearly

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
