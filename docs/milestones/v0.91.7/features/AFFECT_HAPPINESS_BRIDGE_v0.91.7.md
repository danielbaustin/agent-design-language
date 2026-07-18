# Affect And Happiness Implementation Boundary

## Metadata

- Feature Name: Affect And Happiness Implementation Boundary
- Milestone Target: `v0.91.7`
- Status: `boundary_proven` through closed #4752; subjective affect and wellbeing remain non-claims
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture, runtime
- Proof Modes: review, tests, resident-agent-integration

## Purpose

Establish safe tests, implementation expectations, and public-evidence limits
for affect, humor, happiness, and wellbeing surfaces before `v0.92`.

## Scope

In scope:

- affect/humor/happiness/wellbeing evidence boundaries;
- safe-test expectations;
- public claim-boundary language;
- relationship to cognitive profiles and identity evidence;
- consumption of the existing `runtime_v2` affect reasoning-control packet as
  operational proof, not inner-state proof;
- integration of the affect/happiness safe-test model into CSM resident-agent
  runtime existence metadata.

Out of scope:

- consciousness claims;
- wellbeing productization;
- subjective runtime affect, hidden emotion, or inner-state claims.

## Required Decisions

- Which affect surfaces may be tested safely?
  - `v0.92` may test affect-like control only as explicit reasoning-control
    behavior: uncertainty, urgency, attention, friction, and deferral signals
    produce review-depth, escalation, attention-retention, candidate-shift, or
    deferral effects.
  - Safe tests must preserve deterministic fixture ordering, retained evidence
    references, and negative boundary checks that reject hidden-emotion wording.
- Which public claims are explicitly unsupported?
  - No claim that ADL feels emotion, has subjective happiness, has wellbeing,
    experiences suffering, or has consciousness.
  - No scalar happiness score, reward channel, public reputation score, or
    wellbeing certification.
  - No claim that humor, kindness, or reframing proves inner emotional life.
- Which evidence may `v0.92` show without implying inner-state proof?
  - The existing `runtime_v2` affect reasoning-control packet and tests may be
    cited as operational evidence that affect-like labels are bounded,
    reviewable control signals.
  - Birthday or launch evidence may show public-facing summaries only when the
    surrounding copy states that the evidence is about governance/reasoning
    control, not feelings.
- Which profile/privacy constraints apply?
  - Affect and happiness evidence must reference redaction-safe review packets,
    fixture ids, or summarized proof artifacts. It must not expose private
    profile state, hidden operator notes, or raw cognitive-profile internals.

## Boundary Decision

`#4752` defines the WP-13 affect/happiness boundary as
`integrated_proven` for operational reasoning-control evidence and
`not_claimed` for subjective or productized inner-state evidence.

The retained proof surface is:

- implementation contract:
  `adl/src/runtime_v2/affect_reasoning_control.rs`
- focused negative and determinism tests:
  `adl/src/runtime_v2/tests/affect_reasoning_control.rs`
- proof packet:
  `docs/milestones/v0.91.7/review/wp13_affect_happiness_boundary_4752.md`
- resident-agent runtime contract:
  `adl-runtime/src/resident_agent.rs`
- CSM resident-agent admission path:
  `adl/src/csm_resident_agents.rs`
- CSM runtime API retained-artifact validation:
  `adl/src/csm_runtime_api.rs`

This proves that ADL can package affect-like labels as explicit, deterministic
reasoning-control signals with reviewable evidence references, fail-closed
interpretation boundaries, and resident-agent existence metadata. It does not
prove hidden emotion, subjective experience, general happiness, wellbeing,
consciousness, or public product readiness.

## Runtime Integration

The CSM resident-agent runtime contract now requires every
`CsmResidentAgentSpec` to carry an `affect_model` boundary. The model is
populated from `affect_happiness_safe_test_model()` for the Shepherd, Codex /
ChatGPT, and local Ollama residents during admission, and validation rejects
unsafe invocation policies or missing public non-claims.

The runtime integration is existence metadata and governance control. It records
the safe-test model each admitted agent operates under; it is not a subjective
emotion engine or happiness state.

Retained resident-agent status artifacts are accepted only when they validate
against the current resident-agent contract. Legacy serialized artifacts without
`affect_model` are treated as invalid retained evidence and replaced with a
computed fallback status.

## Safe-Test Contract

Every `v0.92` affect/happiness use must pass these checks before it can support
public or birthday evidence:

1. The evidence names a concrete fixture, packet, or review artifact.
2. The copy describes operational reasoning-control, review pressure, or
   governance-bound diagnostic signals.
3. The copy includes a negative claim boundary for hidden emotion, subjective
   happiness, wellbeing, suffering, consciousness, scalar happiness scores,
   reward channels, and public reputation.
4. The evidence avoids raw private profile material and uses redaction-safe
   references.
5. The evidence does not promote humor, kindness, or reframing from bounded
   behavior into inner-state proof.

## Dependencies

- ACP/cognitive profile readiness truth from `v0.91.6`.
- Security implementation readiness.
- `v0.92` birthday demo/public evidence docs.

## Validation And Review

- Review public language for unsupported affect/wellbeing claims.
- Require safe-test framing for any demo evidence.
- Use focused runtime proof when a runtime behavior claim is made. The retained
  proof includes the `runtime_v2_affect_reasoning_control` test family,
  `adl-runtime` resident-agent validation, and ADL CSM resident-agent admission
  proof.
- Record unproved claims as unsupported and keep required surfaces blocked with
  evidence and operator approval.

## v0.92 Consumption

`v0.92` may consume the safe-test boundary, existing
`affect_reasoning_control_packet.v1` proof, and resident-agent `affect_model`
metadata as operational reasoning-control evidence. It must not imply unproved
affect, happiness, wellbeing, consciousness, suffering, scalar happiness scores,
reward channels, or public reputation claims.

## Non-Goals

- No inner-state proof claim.
- No wellbeing certification.
- No subjective runtime affect implementation.
