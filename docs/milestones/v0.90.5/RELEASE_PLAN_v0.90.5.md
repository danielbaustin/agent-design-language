# Release Plan - v0.90.5

## Release Theme

Governed Tools v1.0: model tool proposals become governed, accountable,
privacy-aware ADL capability actions only after validation, compilation, policy,
Freedom Gate mediation, and trace.

## Required Evidence

- UTS schema and conformance results
- ACC authority and visibility fixtures
- compiler mapping and rejection tests
- governed executor proof
- trace/redaction evidence
- dangerous negative suite
- model proposal benchmark
- simple bounded local/Gemma evaluation demo or explicit skip
- flagship demo proof packet
- feature proof coverage record
- [coverage / quality gate record](QUALITY_GATE_v0.90.5.md), including
  first-level Comms / ACIP evidence
- [release readiness record](RELEASE_READINESS_v0.90.5.md) as the reviewer
  entry surface for WP-21 through WP-26
- [release evidence record](RELEASE_EVIDENCE_v0.90.5.md)
- get-well wave disposition
- public-spec language audit and boundary note
- internal and external review notes
- accepted-finding disposition record
- [next milestone planning handoff](NEXT_MILESTONE_HANDOFF_v0.90.5.md)
- explicit `v0.91` follow-on for the full Gemma/local/remote comparison report
- [end-of-milestone report](END_OF_MILESTONE_REPORT_v0.90.5.md)

## Release Risks

- UTS public-compatible language could overclaim standard status
- tool traces could leak private data if redaction is incomplete
- demo could imply arbitrary tool execution if non-goals are weak
- model benchmark could become anecdotal if scoring is not stable
- public UTS compatibility could be mistaken for public standardization or
  runtime permission

## Release Rule

Do not release v0.90.5 as successful unless the negative suite and redaction
evidence prove that model output cannot bypass governed execution.

Do not move into release review until every feature claim has a proof,
non-proving classification, or explicit deferral, and the coverage / quality
gate has recorded the milestone validation posture for both Governed Tools v1.0
and the landed first-level Comms tranche.

Do not describe UTS as a public standard, stable external contract, or
standalone execution authority unless separate evidence and review approve that
claim.

## Release-Tail Order

After the Governed Tools v1.0 flagship demo, release work must preserve this
order:

- demo matrix and feature proof coverage
- coverage / quality gate
- docs + review pass
- internal review
- external / 3rd-party review
- review findings remediation
- next milestone planning
- release ceremony
