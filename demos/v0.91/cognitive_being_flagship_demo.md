# Cognitive-Being Flagship Demo

## Summary

This bounded `v0.91` demo is the reviewer-facing D13 flagship for the
cognitive-being slice.

It does not invent a new runtime. Instead, it composes the already-landed
v0.91 moral-governance, wellbeing, structured planning/review, and secure local
comms proof surfaces into one replayable bundle.

## Scope Boundary

This demo proves:

- moral trace and trajectory review are present as first-class artifacts
- anti-harm and wellbeing remain decomposed and reviewable
- kindness, reframing, and affect-like control remain bounded and non-theatrical
- moral resources and cultivation posture are durable review surfaces
- structured planning (`SPP`) and structured review policy (`SRP`) can appear in
  the same reviewer packet
- secure local comms artifacts can sit beside the cognitive-being review bundle

It does **not** prove:

- first birthday completion
- legal personhood
- production moral agency
- consciousness or subjective feeling
- external or cross-polis communication
- public disclosure of private wellbeing state

## Canonical Command

From repository root:

```bash
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 cognitive-being-flagship-demo --out artifacts/v091/demo-d13-cognitive-being-flagship
```

## Primary Proof Surfaces

- `runtime_v2/cognitive_being/flagship_proof_packet.json`
- `runtime_v2/cognitive_being/flagship_sections.json`
- `runtime_v2/cognitive_being/flagship_reviewer_report.md`

## Section Surfaces

- `runtime_v2/cognitive_being/support/moral_trace_examples.json`
- `runtime_v2/cognitive_being/support/outcome_linkage_examples.json`
- `runtime_v2/cognitive_being/support/moral_trajectory_review_packet.json`
- `runtime_v2/cognitive_being/support/anti_harm_trajectory_constraint_packet.json`
- `runtime_v2/cognitive_being/support/wellbeing_diagnostic_packet.json`
- `runtime_v2/cognitive_being/support/kindness_review_packet.json`
- `runtime_v2/cognitive_being/support/humor_and_absurdity_review_packet.json`
- `runtime_v2/cognitive_being/support/affect_reasoning_control_packet.json`
- `runtime_v2/cognitive_being/support/moral_resource_review_packet.json`
- `runtime_v2/cognitive_being/support/cultivating_intelligence_review_packet.json`
- `runtime_v2/cognitive_being/support/structured_planning_prompt.md`
- `runtime_v2/cognitive_being/support/structured_review_policy.md`
- `runtime_v2/cognitive_being/support/acip_proof_demo_packet_v1.json`
- `runtime_v2/cognitive_being/support/acip_invocation_fixture_set_v1.json`
- `runtime_v2/cognitive_being/support/acip_a2a_fixture_set_v1.json`

## Success Signal

The demo is successful when:

- the bundle writes repository-relative proof artifacts
- the reviewer report names every section and its artifact set
- the replay command is preserved in the proof packet
- the global non-claims remain explicit and truthful
- secure local comms stay local rather than drifting into cross-polis claims
