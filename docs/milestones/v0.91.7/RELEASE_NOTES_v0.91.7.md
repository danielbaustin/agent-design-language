# v0.91.7 Release Notes

## Metadata

- Product: ADL
- Version: `v0.91.7`
- Release date: not scheduled
- Tag: not assigned

## How To Use

Keep these notes implementation-accurate. The implementation, demo, and
quality-gate waves have executed, and WP-17 closed through #4644 / PR #5539.
These notes remain a draft until WP-18, WP-19, WP-20, WP-21A, and WP-23 settle
release-tail review and ceremony truth.

# ADL v0.91.7 Release Notes

## Summary

`v0.91.7` is the implementation/readiness tranche feeding the required
[v0.91.8 bridge](../v0.91.8/README.md). WP-01 through WP-17 are closed with
retained implementation, proof, boundary, or routing evidence. The milestone
is not release-ready while WP-18, WP-19, WP-20, WP-21A, and WP-23 remain open, and
v0.92 may consume only the reviewed v0.91.8 exact-revision handoff.

## Retained Highlights

- Curiosity Engine / Discovery Substrate implementation/proof.
- Constructability Gate implementation/proof.
- Reasoning graph, loop runtime, and `adl.skill.v1` implementation/proof.
- Security readiness proof/blocker status.
- ACIP/A2A/protobuf proof/blocker status.
- Affect and happiness safe-test/public-claim-boundary implementation.
- Godel mechanics implementation/proof.
- Economics-context decision.
- Guild, CodeFriend, and publication-boundary evidence that keeps birthday
  launch claims separate from later product, paper, and customer-facing
  publication work.

## Known Limitations

- These notes claim only the bounded behavior in the linked issue-local proof
  and review packets; they do not claim broad product or runtime completion.
- `v0.92` activation remains blocked until implementation/proof truth is reviewed.
- Runtime v3 is not the default runtime in v0.91.7. #5254 records a no-go
  default-switch decision: Runtime v2 remains default and Runtime v3 remains
  explicit opt-in only until a later reviewed release gate proves cutover
  eligibility.
- Public affect, wellbeing, and cognitive claims remain bounded by safe tests
  and public claim boundaries.
- Paper/publication surfaces are not shipped artifacts in this milestone.
  Papers, public launch approval, customer-facing CodeFriend/report readiness,
  and publication claims require a later tracked artifact, redaction/public
  claim review, and human approval.

## Validation Notes

WP-17 validation for this documentation package includes:

- docs existence check;
- `git diff --check`;
- placeholder and host-local path scan;
- bounded docs review.

## What's Next

- Refresh `v0.92` activation docs from reviewed implementation/proof truth.
- Consume unresolved implementation only through named issues and evidence.
- Carry security/governance work into `v0.93` only when explicitly assigned with evidence and operator approval.

## Exit Criteria

- Final notes reflect only shipped or reviewed behavior.
- Known limitations and future work remain explicitly separated.
