# Structured Intent Prompt

Template: 1.0.0

Issue: 5853

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Determine with controlled same-SHA evidence whether selected ADL CI lanes should use a restricted GitHub-hosted 16-core Ubuntu runner.

## Required Outcome

A complete, reversible control-versus-candidate experiment with retained measurements, proof parity, one canary, and an adopt/reject/defer decision for every measured lane.

## Scope

- .adl/docs/TBD/POST_GITHUB_MIGRATION_BUILD_ACCELERATION_EXPERIMENT_PLAN.md
- .github/workflows/ci.yaml
- docs/tooling/BUILD_PLATFORM_BENCHMARKS.md
- docs/tooling/VALIDATION_PLATFORM_ROUTING.md
- .csdlc/issues/5853
- .csdlc/prepared/issues/5853
- .csdlc/evidence/5853

## Authority

- Issue 5853 owns the bounded 16-core GitHub-hosted runner experiment and any accepted routing change
- WP-02 owns repository transfer and WP-02A owns CI correctness and coverage reliability
- Organization billing and runner-group changes require Agent Logic organization-owner approval
- Further runner sizes, coverage topology, custom images, AWS, and self-hosting require separate authorization

## Assumptions

- none

## Operator Constraints

- Run only after WP-02 and WP-02A gates pass
- Use one restricted selected-repository runner group with maximum concurrency one
- Freeze exact comparison inputs and preserve required-check identity
- Retain all samples and explain every excluded result
- Never edit tracked work on main
- Use one bounded pre-PR review
