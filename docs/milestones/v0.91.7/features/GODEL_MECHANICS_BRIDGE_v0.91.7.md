# Godel Mechanics Implementation Boundary

## Metadata

- Feature Name: Godel Mechanics Implementation Boundary
- Milestone Target: `v0.91.7`
- Status: `boundary_proven` through closed #4753 and review remediation #5405
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy, runtime-boundary
- Proof Modes: review, replay, runtime-focused-test

## Purpose

Map experiment, hypothesis, mutation, evaluation, promotion, and proof
boundaries so birthday evidence can consume Godel mechanics safely. Issue
`#4753` implements this as a Runtime v2 Godel/constructability boundary packet
that composes the retained WP-11 Godel agent runtime packet, its
CSM-supervised launch plan, and the WP-10 constructability anchor validator.

## Scope

In scope:

- experiment and hypothesis artifact boundaries;
- mutation/evaluation/promotion boundaries;
- proof and replay expectations;
- relationship to Runtime v2 reasoning graph and loop runtime evidence;
- CSM-supervised provider-request admission readiness for the retained Godel
  agents;
- constructability anchor requirements for shared-reality or public birthday
  promotion;
- v0.92 allowed and prohibited claim lists.

Out of scope:

- a new Godel runtime beyond the retained WP-11 Runtime v2 Godel agent runtime;
- broad self-improvement claims;
- public superiority claims.

## Implemented Boundary

The implemented boundary surface is
`runtime_v2.godel_constructability_boundary.v1` in
`adl/src/runtime_v2/godel_constructability_boundary.rs`.

It validates all of the following before `v0.92` can consume Godel mechanics
as birthday evidence:

- the WP-11 Runtime v2 Godel agent runtime packet validates and provides 10+
  independent Godel-agent scheduling/provider-binding evidence;
- the retained Godel launch plan admits all 10 Godel agents through CSM
  supervision, lifecycle, provider request, provider response, evidence, and
  checkpoint channels;
- the launch plan requires Freedom Gate, CAV, constructability-anchor,
  constitutional-policy, and advisory-output gates before provider requests are
  admitted;
- hosted provider targets remain `provider_target_resolved_not_invoked` until
  live hosted invocation proof exists;
- Godel non-claims retain no unbounded recursive self-improvement, no live
  hosted provider invocation, no unreviewed source-code mutation, and no
  v0.92 adaptive-learning-DAG completion;
- the WP-10 constructability validator packet validates and requires anchors,
  validator pass, and operator review before external/shared publication;
- v0.92 public birthday copy may describe only a bounded reviewed event backed
  by retained Runtime v2 and constructability evidence.

## Required Decisions

- Admission-ready before `v0.92`: Runtime v2 Godel agent plan readiness,
  deterministic scheduling/provider binding, CSM-supervised launch admission,
  and constructability-gated claim promotion. Every hosted provider request
  remains `provider_target_resolved_not_invoked`.
- Retained artifacts: WP-11 Godel runtime packet, WP-10 constructability anchor
  validator packet, the Godel launch-plan fields in the runtime packet, and the
  WP-13 bridge proof packet.
- Promotions requiring Constructability/operator review: any shared-reality,
  external, public birthday, or publication claim about Godel mechanics.
- Blocked until later proof: autonomous or unbounded recursive
  self-improvement, live hosted-provider invocation, unreviewed source-code
  mutation, and v0.92 adaptive-learning DAG completion.

## Dependencies

- Curiosity Engine feature doc.
- Reasoning graph / skill-standard implementation.
- Constructability Gate.

## Validation And Review

Focused local validation for `#4753`:

```sh
cargo test --manifest-path adl/Cargo.toml --lib runtime_v2_godel_constructability_boundary -- --nocapture
cargo test --manifest-path adl/Cargo.toml --lib runtime_v2_godel_agent_runtime -- --nocapture
git diff --check
```

The Rust proof validates the bridge packet, canonical JSON stability, retained
Godel non-claims, constructability anchor/operator-review requirements, unsafe
v0.92 claim rejection, Godel agent-count drift rejection, admission-plan
construction, complete non-invoked provider-request coverage for all 10 agents,
and launch-plan gate enforcement.

## v0.92 Consumption

`v0.92` may consume the implemented Godel/constructability boundary as a
reviewed claim gate and may consume the Godel launch plan as CSM-supervised
provider-request admission readiness. It must not claim autonomous
self-improvement, live hosted provider invocation, unreviewed source mutation,
or adaptive-learning-DAG completion from this bridge.

## Non-Goals

- No autonomous self-improvement claim.
- No new runtime completion claim beyond the retained Runtime v2 Godel agent
  runtime readiness packet.
- No benchmark superiority claim.
- No live hosted-provider invocation claim.
- No shared-reality or public birthday claim without constructability anchors,
  validator pass, and operator review.
