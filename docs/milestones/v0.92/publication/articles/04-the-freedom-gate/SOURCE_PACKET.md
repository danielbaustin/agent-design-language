# Source Packet: The Freedom Gate

## Brief

- Audience: security engineers, platform architects, and AI governance practitioners.
- Promise: explain the concrete boundary between a generated proposal and an authorized external effect.
- Angle: freedom for a capable agent requires accountable constraints, not the absence of constraints.
- Series role: connect cognition to execution authority.

## Evidence

| Source | Posture | Supported use |
| --- | --- | --- |
| `docs/milestones/v0.86/features/FREEDOM_GATE.md` | Architecture lineage | Original mediation model and policy boundary. |
| `docs/architecture/RUNTIME_V3_GOVERNED_EXECUTION_ARCHITECTURE.md` | Implemented boundary | Signed commitments, grants, permits, replay refusal, and audit. |
| `docs/adr/0015-governed-tools-execution-authority-architecture.md` | Accepted decision | Model proposal versus trusted execution authority. |
| `docs/adr/0021-adl-capability-contract-runtime-authority-boundary.md` | Accepted decision | Capability contracts and runtime authority separation. |
| `adl-runtime-kernel/src/governance.rs` | Implementation | Runtime v3 governance mechanics. |

## Claim Posture

- Current: Runtime v3 implements a focused Freedom Gate contract with signed authority, attenuation, revocation, replay resistance, resource checks, and auditable refusals.
- Current but bounded: the tested kernel boundary does not claim a complete policy-authoring system, distributed authority, or operator UI.
- Planned: broader integration across providers, tools, and inhabited-runtime governance.

## Article Shape

1. Models propose; trusted systems authorize.
2. What a request must carry.
3. Why one-shot permits, attenuation, revocation, and replay controls matter.
4. Refusal and appeal as evidence, not silent failure.
5. What the implemented gate proves and does not prove.

## Guardrails

Do not claim formal verification, universal policy correctness, production deployment, or that cryptographic signatures alone make an action safe.
