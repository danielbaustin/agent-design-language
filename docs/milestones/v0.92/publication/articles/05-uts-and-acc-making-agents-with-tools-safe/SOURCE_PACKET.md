# Source Packet: UTS and ACC

## Brief

- Audience: tool-platform builders, API designers, and security reviewers.
- Promise: show why tool description and execution authority must be separate contracts.
- Angle: schema validity is necessary interoperability information, never permission.
- Series role: apply Freedom Gate principles to practical tool use.

## Evidence

| Source | Posture | Supported use |
| --- | --- | --- |
| `docs/explainers/UTS_AND_ACC.md` | Current explainer | The UTS/ACC distinction. |
| `docs/specs/uts/README.md` | Current spec entrypoint | Implemented UTS v1 and proposed v1.1 boundary. |
| `docs/specs/uts/UTS_V1.0_SCHEMA.md` | Normative current spec | Implemented tool description fields. |
| `docs/specs/acc/ACC_V1.0_SPEC.md` | Normative current spec | Implemented authority and visibility contract. |
| `docs/adr/0020-universal-tool-schema-portable-tool-description-standard.md` | Accepted decision | Portable tool-description rationale. |

## Claim Posture

- Current: UTS v1 and ACC v1.0 are implemented baselines with machine-readable schema and Rust surfaces.
- Proposed: UTS v1.1 and ACC v1.1 are additive evolution targets, not guaranteed current wire behavior.
- Current but bounded: these contracts improve inspectability and governance; they do not guarantee tool correctness or eliminate runtime policy.

## Article Shape

1. The insufficiency of name-description-parameters.
2. UTS: effects, replay, data, authentication, resources, and errors.
3. ACC: actor, grantor, authority, delegation, visibility, and evidence.
4. Admission through a trusted runtime.
5. A practical example and adoption boundary.

## Guardrails

Do not present v1.1 as implemented, claim universal ecosystem adoption, or say a valid schema guarantees safe execution.
