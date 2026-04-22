# Sprint Plan - v0.90.5

## Sprint 1: Specification And Authority

- WP-01 Promote milestone package
- WP-02 Tool-call threat model and semantics
- WP-03 UTS public compatibility and conformance plan
- WP-04 UTS v1 schema finalization
- WP-05 UTS fixture and conformance suite

Goal: make the portable schema precise enough to be public-compatible without
pretending it grants execution authority.

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
- WP-17 Local model and Gemma-focused evaluation
- WP-18 Governed Tools v1.0 flagship demo
- WP-18A Demo matrix and feature proof coverage

Goal: prove that model proposals cannot bypass authority and that the new tool
suite works across real and adversarial model behavior.

## Sprint 4: Review And Release

- WP-19 Quality, docs, review, and public-spec handoff
- WP-20 Release ceremony

Goal: make the milestone reviewable, publication-safe, and ready to hand off to
later tool adapters, CodeBuddy automation, and citizen command packets.

## Parallelization Notes

WP-03 and WP-06 can proceed after WP-02 if the proposal/action boundary is
stable. WP-04 and WP-05 should stay behind the UTS conformance plan. WP-07
should stay behind ACC authority schema. WP-08 can begin once UTS and ACC names
are stable, but WP-09 must wait for fixtures and registry binding. WP-15 through
WP-20 should remain sequential because negative safety, model testing, flagship
demo, proof coverage, review, and release truth depend on one another.
