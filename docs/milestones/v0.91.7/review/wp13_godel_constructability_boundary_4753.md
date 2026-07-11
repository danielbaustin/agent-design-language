# WP-13 Godel/Constructability Boundary Proof (#4753)

## Summary

Issue `#4753` implements a Runtime v2 Godel/constructability boundary for
`v0.92` claim consumption. The bridge consumes the retained WP-11 Godel agent
runtime packet, its CSM-supervised launch plan, and the WP-10 constructability
anchor validator packet. It does not implement a separate new Godel runtime,
but it does make the retained Godel agents executable as admitted provider
requests with runtime-owned channels and gates.

## Runtime Surface

- Module: `adl/src/runtime_v2/godel_constructability_boundary.rs`
- Test module: `adl/src/runtime_v2/tests/godel_constructability_boundary.rs`
- Runtime module consumed: `adl/src/runtime_v2/godel_agent_runtime.rs`
- Runtime tests consumed: `adl/src/runtime_v2/tests/godel_agent_runtime.rs`
- Schema: `runtime_v2.godel_constructability_boundary.v1`
- Launch-plan schema: `runtime_v2.godel_agent_launch_plan.v1`
- Focus: executable Godel-agent readiness and v0.92 birthday-claim boundaries
  for Godel mechanics.

## Consumed Evidence

- WP-11 Runtime v2 Godel agent runtime:
  - 10+ independent Godel-agent readiness.
  - deterministic scheduling and provider-binding evidence.
  - CSM-supervised provider-request admission plan for all 10 agents.
  - runtime-scoped supervision, lifecycle, provider request, provider response,
    evidence, and checkpoint channels.
  - Freedom Gate, CAV, constructability-anchor, constitutional-policy, and
    advisory-output gates before provider request admission.
  - hosted provider targets resolved but not invoked.
- WP-10 constructability anchor validator:
  - admissible anchor requirement.
  - validator-pass requirement.
  - operator-review requirement for shared-reality/public promotion.

## Allowed v0.92 Claims

- `v0.92` may describe a bounded Godel-agent birthday as a reviewed Runtime v2
  event when retained Godel runtime evidence and constructability validation are
  cited.
- `v0.92` may consume 10+ independent Godel-agent runtime readiness as
  deterministic scheduling and provider-binding evidence.
- `v0.92` may consume the CSM-supervised Godel launch plan as provider-request
  admission readiness, not live hosted-provider invocation proof.
- `v0.92` may promote Godel mechanics into public birthday copy only through
  constructability anchors, validator pass, and operator review.

## Non-Claims

- No autonomous or unbounded recursive self-improvement claim.
- No live hosted-provider invocation claim.
- No unreviewed source-code mutation claim.
- No shared-reality publication without constructability anchors.
- No v0.92 adaptive-learning-DAG completion claim.

## Validation

Focused local wuji validation:

```sh
cargo test --manifest-path adl/Cargo.toml --lib runtime_v2_godel_constructability_boundary -- --nocapture
cargo test --manifest-path adl/Cargo.toml --lib runtime_v2_godel_agent_runtime -- --nocapture
git diff --check
```

The Rust proof covers:

- valid bridge packet construction;
- stable canonical JSON materialization;
- missing hosted-provider non-claim rejection;
- disabled constructability anchor rejection;
- unsafe v0.92 claim rejection;
- Godel agent-count drift rejection.
- executable launch-plan construction;
- complete provider-request coverage for all 10 Godel agents;
- launch-plan policy-gate rejection;
- boundary rejection when launch-plan provider-request counts drift.

Remote builders are not used as proof for this issue.
