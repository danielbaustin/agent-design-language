# Source Packet: ADL Runtime and the Cognitive Spacetime Manifold

## Brief

- Audience: runtime, distributed-systems, and agent-platform engineers.
- Promise: show why long-lived agents need a governed world model, not only a request loop.
- Angle: identity, memory, time, causality, and policy become architectural state.
- Series role: move from ADL's overview into its runtime substrate.

## Evidence

| Source | Posture | Supported use |
| --- | --- | --- |
| `docs/explainers/CSM.md` | Current architecture | CSM definition, polis relationship, and current boundary. |
| `docs/architecture/RUNTIME_V3_OPERATIONAL_COMPONENTS.md` | Implemented boundary | Runtime v3 component topology. |
| `docs/architecture/RUNTIME_V3_GOVERNED_EXECUTION_ARCHITECTURE.md` | Implemented boundary | Freedom Gate, Adaptive Execution Engine (AEE), audit, continuity, and proof limits. |
| `docs/adr/0012-runtime-v2-bounded-csm-run.md` | Accepted decision | Bounded CSM execution model. |
| `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md` | Active plan | Identity-continuity and birthday requirements. |

## Claim Posture

- Current: bounded CSM runs, runtime packets, continuity contracts, governance services, and operator-facing proof surfaces exist.
- Current but bounded: Runtime v3 governance has focused tests and typed contracts; production transport and distributed authority are not claimed.
- Planned: a fully inhabited runtime with complete citizen, identity, migration, and birthday surfaces.

## Article Shape

1. Why a chat session is not a world.
2. Time, memory, causality, identity, and policy as first-class state.
3. The manifold and the polis.
4. Runtime v3's implemented governed-execution slice.
5. The evidence required before calling persistence identity continuity.

## Guardrails

Do not equate process uptime with identity, describe all Runtime v2 plans as implemented, or claim production-grade distributed operation.
