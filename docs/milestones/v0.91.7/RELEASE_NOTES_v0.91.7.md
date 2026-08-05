# v0.91.7 Release Notes

## Metadata

- Product: ADL
- Version: `v0.91.7`
- Target release closeout date: `2026-07-20`
- Tag: `v0.91.7`
- GitHub release: https://github.com/danielbaustin/agent-design-language/releases/tag/v0.91.7

## How To Use

Keep these notes implementation-accurate. WP-01 through WP-22, including
WP-21A, are closed with retained evidence. WP-20 closed through #4647 / PR
#5588 after all 22 WP-19 findings were fixed. WP-23 #4650 is the sole remaining
v0.91.7 issue and this release-ceremony change is its integration boundary.

# ADL v0.91.7 Release Notes

## Summary

`v0.91.7` is the completed implementation/readiness tranche feeding the
required [v0.91.8 bridge](../v0.91.8/README.md). WP-01 through WP-22,
including WP-21A, are closed with retained implementation, proof, boundary,
review, remediation, or routing evidence. WP-23 integrates the final ceremony
packet. `v0.92` may consume only a reviewed v0.91.8 exact-revision handoff.

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

WP-23 validation for this release-closeout package includes:

- live ADL issue inventory proving #4650 is the only open v0.91.7 issue before
  ceremony integration;
- live ADL PR inventory proving no unrelated PR is open;
- retained WP-20 remediation matrix proving all 22 WP-19 findings fixed;
- YAML/JSON parse and documentation-link checks;
- `git diff --check`;
- bounded exact-head review before publication.

## Release Boundary

This closeout records the v0.91.7 milestone evidence boundary. It does not
create a Git tag, publish a binary, deploy a service, switch the default
runtime, or activate v0.92. Those actions require their own reviewed authority.

## What's Next

- Refresh `v0.92` activation docs from reviewed implementation/proof truth.
- Consume unresolved implementation only through named issues and evidence.
- Carry security/governance work into `v0.93` only when explicitly assigned with evidence and operator approval.

## Exit Criteria

- Final notes reflect only shipped or reviewed behavior.
- Known limitations and future work remain explicitly separated.
