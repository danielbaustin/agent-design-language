# Structured Task Prompt

Template: 1.0.0

Issue: 5409

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Complete the runtime-owned topology, readiness, credential-renewal, and soak surfaces without redesigning unrelated provider or AWS infrastructure.

## Deliverables

- Production assembly aligned with the supervised component topology
- Readiness health covering every required component/channel
- Credential overlap renewal tests and assembled-runtime soak evidence

## Acceptance

1. Emergency stop accepts only a verifiable governed authority and rejects forged or wrong-authority credentials
2. API Gateway proof exercises /status, /health, /ready, /metrics, /events, and /chronosense plus required failure cases, or is explicitly classified as bounded smoke
3. #4906 records integrated closure or an operator-approved release disposition
4. WP-07 readiness consumers remain blocked until the final gate is truthful
5. Updated proof and review evidence is retained

## Dependencies

- #5121 runtime rearchitecture implementation
- adl-runtime supervision and topology modules
- runtime API credential policy

## Inputs

- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/supervision.rs
- adl-runtime/src/topology.rs
- adl/src/long_lived_agent.rs
- adl/src/csm_runtime_api.rs

## Non Goals

- Unrelated WP-07 hardening or API Gateway changes
- New AWS infrastructure
- Provider and model work
