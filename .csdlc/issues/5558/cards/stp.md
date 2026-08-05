# Structured Task Prompt

Template: 1.0.0

Issue: 5558

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Correct only active live guidance, executable compatibility surfaces, and owner-lane validation policy.

## Deliverables

- Updated live guidance and compatibility tests
- Removed editor start route
- Owner lane Gate 10A proof

## Acceptance

1. AC-1: owner lane does not require sunset v1 lifecycle commands
2. AC-2: live guidance resolves through csdlc-install and typed v2 binaries or skills
3. AC-3: active tests and guidance do not teach adl/tools/pr.sh run
4. AC-4: focused guidance tests and full C-SDLC owner lane pass without AWS

## Dependencies

- Gate 10D2 final v1_sunset authority

## Inputs

- AGENTS.md
- csdlc-v2/operator/generation-selector.json
- .csdlc/prepared/issues/5748/fail-closed-exceptions.md

## Non Goals

- Changing historical evidence
- Changing Runtime v3 behavior
- Using AWS validation
