# v0.94 Candidate Work Breakdown Structure

## Status

Candidate allocation only. `v0.94` has no final issue wave yet.

## WBS Summary

`v0.94` should complete the secure-execution, trust-convergence, and temporal
self-projection band without stealing payments work from `v0.94.1`.

## Candidate Work Areas

| Candidate | Work Area | Description | Primary deliverable | Key dependencies |
| --- | --- | --- | --- | --- |
| A | Secure execution model | Define the runtime authority and execution boundary. | Secure execution contract and negative cases. | `v0.93` governance and security. |
| B | Policy engine | Converge authorization and policy evaluation semantics. | Policy engine architecture and fixtures. | A, `v0.93` IAM. |
| C | Identity/auth and trust | Close runtime identity/auth and provider trust surfaces. | Identity/auth boundary and trust model. | `v0.92`, `v0.93`. |
| D | Isolation and secrets | Define sandbox/runtime isolation and secrets/data governance. | Isolation/data-governance contract. | A through C. |
| E | Signed/queryable trace closure | Turn trace into a queryable reasoning/provenance substrate. | Queryable trace design and proof surface. | earlier trace/ObsMem bands. |
| F | Mental time travel | Define bounded temporal self-projection over remembered and simulated time. | `MTT-v1` contract and reasoning artifact shape. | `v0.92`, chronosense, trace, ObsMem, GHB. |
| G | Demo matrix and proof demos | Build secure-execution and temporal-self-projection proof candidates. | Demo matrix and proof candidates. | A through F. |
| H | Review and release tail | Align docs, review, and release closure. | Review-ready milestone package. | All prior work. |

## Sequencing Pressure

1. Start with secure execution, policy, and trust boundaries.
2. Add isolation and signed/queryable trace closure.
3. Land MTT only after identity, trace, and simulation prerequisites are explicit.
4. Keep payments entirely out of this band.

## Acceptance Mapping

- `v0.94` must be visibly distinct from `v0.93` governance and `v0.94.1` payments.
- MTT must remain bounded, inspectable, and identity-bearing.
- Secure execution must remain trace-backed and fail closed.
