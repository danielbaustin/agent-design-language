# Sprint Plan - v0.90.5

## Sprint 1: Specification And Authority

- WP-01 Design pass (milestone docs + planning)
- WP-02 Tool-call threat model and semantics
- WP-03 UTS public compatibility and conformance plan
- WP-04 UTS v1 schema finalization
- WP-05 UTS fixture and conformance suite

Goal: make the portable schema precise enough to be public-compatible without
pretending it grants execution authority.

## Get-Well Wave: Test Runtime Reduction

- GW-00 Get-well baseline, runtime budget, and wave tracking artifact
- GW-01 Collapse external counterparty proof-family tests
- GW-02 Collapse private-state observatory proof-family tests
- GW-03 Collapse delegation subcontract proof-family tests
- GW-04 Collapse contract-market and resource-stewardship proof-family tests
- GW-05 Shrink CLI and demo proof-matrix tail

Goal: reduce heavyweight proof-family validation cost as early as practical
without changing the canonical WP sequence or weakening governed-tools proof
claims.

## Parallel Comms Sprint: ACIP Foundation And Specialization

- Comms-01 Promote ACIP v1 general protocol architecture
- Comms-02 ACIP message envelope schema and identity model
- Comms-03 ACIP invocation contract and Freedom Gate event binding
- Comms-04 ACIP validation fixtures and conformance suite
- Comms-05 Review-agent invocation specialization and SRP policy binding
- Comms-06 Coding-agent invocation specialization and provider-neutral runner
- Comms-07 ACIP trace, replay, redaction, and evidence integration
- Comms-08 ACIP demo and proof coverage

Goal: make agent communication a first-class substrate feature instead of a
review-only prompt lane, while keeping execution authority under UTS, ACC,
Freedom Gate, and governed execution.

Sprint boundary:

- Comms-01 is the guaranteed `v0.90.5` planning and prerequisite-alignment
  slice.
- Comms-02 through Comms-04 are candidate foundation follow-ons that should be
  reviewed against the source ACIP split plan before they are treated as kept
  in `v0.90.5`.
- Comms-05 through Comms-08 are specialization and proof slices that may stay
  in `v0.90.5` or defer to `v0.91` depending on milestone pressure.
- This wave is parallel to, not a replacement for, the governed-tools WP
  sequence and should not be mistaken for WP-14 scope creep.

## Sprint 2: ACC, Compiler, And Policy

- WP-06 ACC v1 authority schema
- WP-07 ACC privacy, visibility, and delegation model
- WP-08 Tool registry and binding model
- WP-09 UTS to ACC compiler
- WP-10 Normalization and argument validation
- WP-11 Policy injection and authority evaluation
- WP-12 Freedom Gate integration

Goal: turn model-facing descriptions into governed ADL capability contracts.

## Sprint 3: Execution, Evidence, Model Testing, And Demo

- WP-13 Governed executor
- WP-14 Trace, replay, redaction, and evidence contract
- WP-15 Dangerous tool negative suite
- WP-16 Model proposal benchmark harness
- WP-17 Local model and Gemma-focused evaluation demo
- WP-18 Governed Tools v1.0 flagship demo

Goal: prove that model proposals cannot bypass authority and that the new tool
suite works across real and adversarial model behavior without taking on the
full multi-model comparison report before `v0.91`.

## Sprint 4: Quality, Review, Release, And Handoff

- WP-19 Demo matrix and feature proof coverage
- WP-20 Coverage / quality gate
- WP-21 Docs + review pass
- WP-22 Internal review
- WP-23 External / 3rd-party review
- WP-24 Review findings remediation
- WP-25 Next milestone planning
- WP-26 Release ceremony

Goal: make the milestone reviewable, publication-safe, and ready to hand off to
later tool adapters, CodeBuddy automation, and citizen command packets.

WP-25 specifically needs to leave the next milestone in a ready state rather
than only naming it. That means the tracked `v0.91` package should already
carry the accepted core feature surfaces, the structured-planning and SRP
workflow features, and the explicit `v0.91` / `v0.91.1` split for adjacent
systems such as the full comparison report and A2A hardening.

## Parallelization Notes

WP-03 and WP-06 can proceed after WP-02 if the proposal/action boundary is
stable. WP-04 and WP-05 should stay behind the UTS conformance plan. WP-07
should stay behind ACC authority schema. WP-08 can begin once UTS and ACC names
are stable, but WP-09 must wait for fixtures and registry binding. WP-15 through
WP-26 should remain sequential because negative safety, model testing, flagship
demo, proof coverage, quality, review, remediation, next planning, and release
truth depend on one another.

## Release-Tail Rule

Sprint 4 preserves the established ADL closeout state machine:

- demo matrix / feature proof coverage
- coverage / quality gate
- docs + review pass
- internal review
- external / 3rd-party review
- review findings remediation
- next milestone planning
- release ceremony
