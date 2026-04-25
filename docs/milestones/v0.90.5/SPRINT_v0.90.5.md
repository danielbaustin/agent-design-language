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
