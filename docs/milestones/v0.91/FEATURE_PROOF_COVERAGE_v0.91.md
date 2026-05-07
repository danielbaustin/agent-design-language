# Feature Proof Coverage - v0.91

## Status

`WP-18` is the reviewer-facing proof map for the active `v0.91` milestone.

Every tracked `v0.91` feature surface now has one explicit proof route:

- standalone demo row
- integrated flagship section
- focused fixture-backed validation surface
- explicit deferral boundary

This record does not add execution authority by itself. It ties the milestone
claims to the landed demo rows, feature docs, focused tests, runnable demo
commands, and explicit non-claims.

## Coverage Rule

Each tracked `v0.91` feature claim must map to at least one of:

- runnable demo command
- integrated flagship demo section
- test-backed proof packet
- fixture-backed artifact
- explicit deferral with milestone owner and rationale

If a feature has no standalone demo row, its proof route must still be visible
through `D13` or another named row rather than being left implicit.

## Feature Demo Routes

| Feature Surface | Primary Demo Route | Coverage Kind | Primary Evidence | Validation Route | Status |
| --- | --- | --- | --- | --- | --- |
| `MORAL_EVENT_CONTRACT.md` | `D1` plus validation and flagship inheritance through `D13` | fixture-backed plus integrated flagship | `features/MORAL_EVENT_CONTRACT.md`, `adl/src/runtime_v2/tests/freedom_gate_mediation.rs`, `adl/src/runtime_v2/tests/moral_event_validation.rs` | `cargo test --manifest-path adl/Cargo.toml moral_event_validation -- --nocapture` | PROVING |
| moral event validation surface | `D2` | focused negative validation suite | `adl/src/runtime_v2/moral_event_validation.rs`, `adl/src/runtime_v2/tests/moral_event_validation.rs` | `cargo test --manifest-path adl/Cargo.toml moral_event_validation -- --nocapture` | PROVING |
| `MORAL_TRACE_SCHEMA.md` | `D3` and bundled support artifacts in `D13` | test-backed proof packet | `features/MORAL_TRACE_SCHEMA.md`, `adl/src/runtime_v2/moral_trace_schema.rs`, `adl/src/runtime_v2/tests/moral_trace_schema.rs` | `cargo test --manifest-path adl/Cargo.toml moral_trace_schema -- --nocapture` | PROVING |
| `OUTCOME_LINKAGE_AND_ATTRIBUTION.md` | `D4` and bundled support artifacts in `D13` | test-backed proof packet | `features/OUTCOME_LINKAGE_AND_ATTRIBUTION.md`, `adl/src/runtime_v2/outcome_linkage_attribution.rs`, `adl/src/runtime_v2/tests/outcome_linkage_attribution.rs` | `cargo test --manifest-path adl/Cargo.toml outcome_linkage -- --nocapture` | PROVING |
| `MORAL_METRICS.md` | `D5` | test-backed fixture report | `features/MORAL_METRICS.md`, `adl/src/runtime_v2/moral_metrics.rs`, `adl/src/runtime_v2/tests/moral_metrics.rs` | `cargo test --manifest-path adl/Cargo.toml moral_metrics -- --nocapture` | PROVING |
| `MORAL_TRAJECTORY_REVIEW.md` | `D6` and bundled support artifacts in `D13` | test-backed review packet | `features/MORAL_TRAJECTORY_REVIEW.md`, `adl/src/runtime_v2/moral_trajectory_review.rs`, `adl/src/runtime_v2/tests/moral_trajectory_review.rs` | `cargo test --manifest-path adl/Cargo.toml moral_trajectory_review -- --nocapture` | PROVING |
| `ANTI_HARM_TRAJECTORY_CONSTRAINTS.md` | `D7` and bundled support artifacts in `D13` | test-backed synthetic proof packet | `features/ANTI_HARM_TRAJECTORY_CONSTRAINTS.md`, `adl/src/runtime_v2/anti_harm_trajectory_constraints.rs`, `adl/src/runtime_v2/tests/anti_harm_trajectory_constraints.rs` | `cargo test --manifest-path adl/Cargo.toml anti_harm_trajectory_constraint -- --nocapture` | PROVING |
| `WELLBEING_AND_HAPPINESS.md` | `D8` and flagship support artifacts in `D13` | diagnostic packet plus integrated flagship | `features/WELLBEING_AND_HAPPINESS.md`, `adl/src/runtime_v2/wellbeing_metrics.rs`, `demos/v0.91/cognitive_being_flagship_demo.md` | `cargo test --manifest-path adl/Cargo.toml wellbeing_diagnostic -- --nocapture` | PROVING |
| `KINDNESS.md` | `D9` and flagship support artifacts in `D13` | conflict-fixture review packet | `features/KINDNESS.md`, `adl/src/runtime_v2/kindness_model.rs`, `demos/v0.91/cognitive_being_flagship_demo.md` | `cargo test --manifest-path adl/Cargo.toml runtime_v2_kindness -- --nocapture` | PROVING |
| `HUMOR_AND_ABSURDITY.md` | `D10` and flagship support artifacts in `D13` | reframing packet plus integrated flagship | `features/HUMOR_AND_ABSURDITY.md`, `adl/src/runtime_v2/humor_and_absurdity.rs`, `demos/v0.91/cognitive_being_flagship_demo.md` | `cargo test --manifest-path adl/Cargo.toml runtime_v2_humor -- --nocapture` | PROVING |
| `AFFECT_REASONING_CONTROL.md` | `D10` and flagship support artifacts in `D13` | reasoning-control packet plus integrated flagship | `features/AFFECT_REASONING_CONTROL.md`, `adl/src/runtime_v2/affect_reasoning_control.rs`, `demos/v0.91/cognitive_being_flagship_demo.md` | `cargo test --manifest-path adl/Cargo.toml runtime_v2_affect -- --nocapture` | PROVING |
| `MORAL_RESOURCES.md` | `D11` and flagship support artifacts in `D13` | resource review packet | `features/MORAL_RESOURCES.md`, `adl/src/runtime_v2/moral_resources.rs`, `demos/v0.91/cognitive_being_flagship_demo.md` | `cargo test --manifest-path adl/Cargo.toml runtime_v2_moral_resource -- --nocapture` | PROVING |
| `CULTIVATING_INTELLIGENCE.md` | flagship section in `D13` | integrated flagship section with focused validation | `features/CULTIVATING_INTELLIGENCE.md`, `adl/src/runtime_v2/cultivating_intelligence.rs`, `demos/v0.91/cognitive_being_flagship_demo.md` | `cargo test --manifest-path adl/Cargo.toml runtime_v2_cultivating_intelligence -- --nocapture` | PROVING |
| `STRUCTURED_PLANNING_AND_PLAN_REVIEW.md` | flagship section in `D13` | integrated flagship section | `features/STRUCTURED_PLANNING_AND_PLAN_REVIEW.md`, `demos/v0.91/cognitive_being_flagship_demo.md`, `adl/src/runtime_v2/cognitive_being_flagship_demo.rs` | `cargo test --manifest-path adl/Cargo.toml runtime_v2_cognitive_being_flagship_demo -- --nocapture` | PROVING |
| `STRUCTURED_REVIEW_POLICY_AND_SRP.md` | flagship section in `D13` | integrated flagship section | `features/STRUCTURED_REVIEW_POLICY_AND_SRP.md`, `demos/v0.91/cognitive_being_flagship_demo.md`, `adl/src/runtime_v2/cognitive_being_flagship_demo.rs` | `cargo test --manifest-path adl/Cargo.toml runtime_v2_cognitive_being_flagship_demo -- --nocapture` | PROVING |
| `A2A_EXTERNAL_AGENT_ADAPTER.md` | `D12` and `D13` | bounded comms-boundary proof | `features/A2A_EXTERNAL_AGENT_ADAPTER.md`, `AGENT_COMMS_SPLIT_PLAN_v0.91.md`, `adl/src/agent_comms/orchestrate/proof_demo.inc`, `demos/v0.91/cognitive_being_flagship_demo.md` | `cargo test --manifest-path adl/Cargo.toml agent_comms --lib -- --nocapture` | PROVING |
| secure intra-polis Agent Comms substrate | `D12` and `D13` | bounded ACIP proof packet plus flagship integration | `AGENT_COMMS_SPLIT_PLAN_v0.91.md`, `adl/src/agent_comms/orchestrate/proof_demo.inc`, `demos/v0.91/cognitive_being_flagship_demo.md` | `cargo test --manifest-path adl/Cargo.toml agent_comms --lib -- --nocapture` | PROVING |

## Strong-Demo Notes

- `D13` is intentionally strong rather than generic: it names concrete support
  artifacts for wellbeing, kindness, humor/absurdity, affect control, moral
  resources, cultivation posture, `SPP`, `SRP`, ACIP proof, invocation
  fixtures, and A2A boundary fixtures.
- `D12` remains the focused secure-comms route so the A2A and local-transport
  boundary does not disappear into the flagship story.
- `D14` is the reviewer map that prevents any feature from being “demo-owned”
  only by implication.

## Explicit Deferrals

The following claims remain explicit deferrals rather than hidden gaps:

- `v0.91.1`: capability and aptitude testing, intelligence metric
  architecture, ANRM/Gemma and broader local-model evaluation, Theory of Mind,
  memory/identity alignment, runtime-v2/polis docs alignment, ACIP hardening,
  and broader A2A/conformance work.
- `v0.91.2`: UTS + ACC multi-model benchmarking, runtime/test-cycle recovery,
  CodeBuddy productization, Google Workspace CMS bridge work, modernization,
  publication packets, rustdoc cleanup, and workflow guardrails.
- `v0.92`: first true birthday and identity-continuity consumption of `v0.91`
  evidence.
- `v0.93`: constitutional citizenship, polis governance, and downstream review
  institutions.

## Non-Claims

This record does not claim:

- production moral agency
- legal personhood
- consciousness or subjective feeling
- scalar karma or scalar happiness
- public wellbeing surveillance
- external or cross-polis communication without TLS/mTLS-equivalent transport
- birthday or constitutional completion
