# Design - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`

## Purpose

Capture the design interpretation of `v0.89.1` so the milestone package stays coherent before implementation begins.

## Design Interpretation

`v0.89.1` is the adversarial-runtime and exploit-evidence milestone that follows the governed-adaptation base established in `v0.89`.

The package is intentionally structured around three layers:

1. runtime architecture
2. artifact and replay contracts
3. demo/review proof surfaces

This keeps the milestone focused on real operational proof rather than drifting into pure theory.

## Design Layers

### 1. Adversarial runtime architecture

Owned mainly by:
- `ADL_ADVERSARIAL_RUNTIME_MODEL.md`
- `RED_BLUE_AGENT_ARCHITECTURE.md`
- `ADVERSARIAL_EXECUTION_RUNNER.md`
- `SELF_ATTACKING_SYSTEMS.md`

This layer answers:
- what the runtime assumes about opposition
- which persistent roles exist
- how those roles are scheduled and coordinated
- how self-attack or red/blue dynamics remain bounded and reviewable

### 2. Artifact and replay contracts

Owned mainly by:
- `EXPLOIT_ARTIFACT_SCHEMA.md`
- `ADVERSARIAL_REPLAY_MANIFEST.md`
- `CONTINUOUS_VERIFICATION_AND_EXPLOIT_GENERATION.md`

This layer answers:
- what artifacts are emitted
- how exploit evidence is represented
- how replay and verification can be re-run
- how ongoing verification differs from one-off demos

### 3. Demo and governed execution surfaces

Owned mainly by:
- `ADL_ADVERSARIAL_DEMO.md`
- `OPERATIONAL_SKILLS_SUBSTRATE.md`
- `SKILL_COMPOSITION_MODEL.md`

Supporting idea inputs:
- `ideas/DELEGATION_AND_REFUSAL.md`
- `ideas/MULTI_AGENT_NEGOTIATION.md`

This layer answers:
- how the adversarial band is demonstrated
- how skills and compositions execute these scenarios
- which surrounding governance concepts should influence implementation without being over-promoted

## Boundary Against `v0.89`

`v0.89` already owns:
- the main-band security posture and threat model
- the governed execution substrate for the core adaptation band

`v0.89.1` adds:
- contested-runtime behavior
- exploit and replay evidence
- adversarial demoability

That boundary must stay explicit.

## Ideas And Non-Promoted Inputs

These are tracked as ideas/backgrounders rather than promoted tracked feature commitments:

- `ideas/DELEGATION_AND_REFUSAL.md`
- `ideas/MULTI_AGENT_NEGOTIATION.md`

These remain local-only or non-promoted supporting inputs:

- `ADL_SECURITY_DEMOS.md`
- `PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md`

Reasons:
- the tracked ideas are conceptually important but narrower than the main exploit/runtime core
- internal operational-skills process notes are not reader-facing milestone backgrounders
- two source files are currently empty and should not be over-promoted
- the milestone should avoid claiming more maturity than the corpus currently supports

## Risks

- over-expanding the milestone into all security/governance work
- promoting under-authored source docs as though they were mature feature contracts
- confusing review/demo surfaces with real replayable evidence

## Design Exit Criteria

- promoted feature set is explicit
- non-promoted inputs are explicitly accounted for
- runtime, artifact, and proof layers are each represented in the milestone package
- the package is strong enough to be reviewed and later issueized without reopening milestone intent
