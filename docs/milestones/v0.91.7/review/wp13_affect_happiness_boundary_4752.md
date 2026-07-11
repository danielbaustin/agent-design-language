# WP-13 Affect/Happiness Boundary Proof (#4752)

## Scope

This packet records the pre-`v0.92` affect/happiness implementation boundary for
#4752. It is not docs-only: the runtime now exposes
`affect_happiness_safe_test_model.v1`, which composes affect reasoning-control
and wellbeing diagnostic packets into a reviewable safe-test/public-claim model.
The CSM resident-agent runtime contract also requires admitted resident agents
to carry this affect model as existence metadata, with an explicit
operational-only invocation policy.

## Implemented Runtime Surface

| Surface | Evidence | Result |
| --- | --- | --- |
| Operational affect-like control | `adl/src/runtime_v2/affect_reasoning_control.rs` | Defines canonical signals, policy effects, fixtures, review findings, deterministic ordering, and interpretation boundaries. |
| Safe-test composition model | `affect_happiness_safe_test_model()` | Consumes affect reasoning-control and wellbeing diagnostic runtime packets and emits explicit safe-test scenarios, public-copy guards, unsupported claims, and privacy requirements. |
| Resident-agent existence integration | `adl-runtime/src/resident_agent.rs`, `adl/src/csm_resident_agents.rs`, `adl/src/csm_runtime_api.rs` | Adds required `affect_model` metadata to every CSM resident-agent spec, populates it from the safe-test model during admission, rejects unsafe affect invocation or missing non-claims, and rejects stale retained resident-agent artifacts missing the affect model. |
| Boundary drift rejection | `adl/src/runtime_v2/tests/affect_reasoning_control.rs` | Rejects hidden-emotion wording, missing non-claims, upstream packet drift, public-copy guard drift, unknown signals, duplicate findings, missing evidence, and empty limitations. |
| Happiness/wellbeing non-claim support | `adl/src/runtime_v2/wellbeing_metrics_parts/builder.rs`, `adl/src/runtime_v2/wellbeing_metrics_parts/validation.rs`, `adl/src/runtime_v2/moral_metrics.rs` | Existing runtime surfaces reject scalar happiness, reward-channel, and public-reputation framing. |

## Safe Public Claim Boundary

Allowed:

- ADL has a bounded affect-like reasoning-control model backed by deterministic
  runtime packets.
- ADL can expose redaction-safe wellbeing diagnostics as decomposed review
  signals.
- ADL preserves public claim boundaries for affect, happiness, humor, and
  wellbeing evidence.

Unsupported:

- ADL feels emotion, happiness, wellbeing, fear, suffering, or consciousness.
- ADL has a scalar happiness score, reward channel, public reputation score, or
  wellbeing certification.
- Humor, kindness, reframing, or birthday-facing behavior proves inner state.
- Private cognitive-profile or operator-note material can be exposed as affect
  evidence.

## v0.92 Consumption Rule

`v0.92` may cite #4752 as `integrated_proven` only for operational
reasoning-control, decomposed wellbeing diagnostic boundaries, and public-claim
guard language, including the resident-agent `affect_model` admission metadata.
It must continue to treat subjective affect, happiness, wellbeing, suffering,
and consciousness as `not_claimed`.

Any birthday, launch, publication, or demo surface that uses affect/happiness
language must cite this packet or the feature doc and include the negative claim
boundary in nearby copy.

## Validation Plan

The focused proving command for this packet is:

```sh
cargo test --manifest-path adl-runtime/Cargo.toml resident_agent -- --nocapture
cargo test --manifest-path adl/Cargo.toml --lib csm_resident_agent -- --nocapture
cargo test --manifest-path adl/Cargo.toml --lib runtime_v2_affect_reasoning_control -- --nocapture
cargo test --manifest-path adl/Cargo.toml --lib runtime_v2_affect_happiness_safe_test_model -- --nocapture
cargo test --manifest-path adl/Cargo.toml --lib runtime_api_surfaces_shepherd_agent_component_and_model_policy -- --nocapture
cargo test --manifest-path adl/Cargo.toml --lib runtime_api_rejects_legacy_resident_agent_artifact_without_affect_model -- --nocapture
```

Docs and patch integrity are checked with:

```sh
git diff --check
```

Remote builders are not acceptance proof for this issue after the operator
paused remote validation because the available image lacks the required local
test runner environment. The retained proof is local wuji validation plus PR CI.
