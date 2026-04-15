# Feature Docs - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`

## Purpose

Provide the canonical feature index for `v0.89.1`.

This page defines:
- which `v0.89.1` feature docs are promoted into the tracked milestone package now
- which local planning docs remain supporting inputs but are not yet promoted
- how the `v0.89` carry-forward corpus maps into this follow-on band

## Scope Interpretation

`v0.89.1` is the milestone where ADL turns adversarial/runtime carry-forward into a concrete package:

- adversarial runtime assumptions become explicit
- exploit artifacts and replay evidence become first-class
- red/blue/self-attack behavior becomes architecture rather than implication
- demo/review proof surfaces for this band become explicit
- the bounded manuscript/publication workflow becomes explicit enough to support the initial three-paper arXiv program

## Promoted Tracked Feature Docs

| Feature doc | Primary concern | Planned WPs |
|---|---|---|
| `features/ADL_ADVERSARIAL_RUNTIME_MODEL.md` | contested-runtime framing and adversarial cognition | `WP-02` |
| `features/RED_BLUE_AGENT_ARCHITECTURE.md` | persistent adversarial roles and bounded interaction model | `WP-03` |
| `features/ADVERSARIAL_EXECUTION_RUNNER.md` | orchestration surface for adversarial execution | `WP-04` |
| `features/EXPLOIT_ARTIFACT_SCHEMA.md` | canonical exploit artifact family | `WP-05` |
| `features/ADVERSARIAL_REPLAY_MANIFEST.md` | replay and reproduction contract | `WP-05` |
| `features/CONTINUOUS_VERIFICATION_AND_EXPLOIT_GENERATION.md` | ongoing verification and exploit-generation execution pattern | `WP-06` |
| `features/SELF_ATTACKING_SYSTEMS.md` | self-attack architectural pattern | `WP-06` |
| `features/ADL_ADVERSARIAL_DEMO.md` | flagship adversarial proof/demo contract | `WP-07` |
| `features/OPERATIONAL_SKILLS_SUBSTRATE.md` | runtime execution substrate for operational skills | `WP-08` |
| `features/SKILL_COMPOSITION_MODEL.md` | explicit composition primitives and execution semantics | `WP-08` |

## Source Planning Corpus -> Implementation Home

### Core `v0.89.1` source docs

| Local source doc | Disposition | Implementation home |
|---|---|---|
| `ADL_ADVERSARIAL_RUNTIME_MODEL.md` | promoted | `v0.89.1 / WP-02` |
| `RED_BLUE_AGENT_ARCHITECTURE.md` | promoted | `v0.89.1 / WP-03` |
| `ADVERSARIAL_EXECUTION_RUNNER.md` | promoted | `v0.89.1 / WP-04` |
| `EXPLOIT_ARTIFACT_SCHEMA.md` | promoted | `v0.89.1 / WP-05` |
| `ADVERSARIAL_REPLAY_MANIFEST.md` | promoted | `v0.89.1 / WP-05` |
| `CONTINUOUS_VERIFICATION_AND_EXPLOIT_GENERATION.md` | promoted | `v0.89.1 / WP-06` |
| `SELF_ATTACKING_SYSTEMS.md` | promoted | `v0.89.1 / WP-06` |
| `ADL_ADVERSARIAL_DEMO.md` | promoted | `v0.89.1 / WP-07` |
| `OPERATIONAL_SKILLS_SUBSTRATE.md` | promoted | `v0.89.1 / WP-08` |
| `SKILL_COMPOSITION_MODEL.md` | promoted | `v0.89.1 / WP-08` |
| `DELEGATION_AND_REFUSAL.md` | supporting planning input | informs `WP-09` governance boundary work |
| `MULTI_AGENT_NEGOTIATION.md` | supporting planning input | informs `WP-09` coordination and disagreement surfaces |
| `PROPOSED_OPERATIONAL_SKILLS.md` | supporting planning input | informs `WP-08` and `WP-09` skill-surface packaging |
| `.adl/docs/TBD/ARXIV_PAPER_PROGRAM_PLAN.md` | supporting planning input | informs the committed `WP-08` bounded `arxiv-paper-writer` skill and the `WP-13` three-paper publication packet |
| `ADL_SECURITY_DEMOS.md` | under-authored supporting input | do not promote until authored; informs later demo packaging |
| `PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md` | under-authored supporting input | do not promote until authored; candidate later security-extension slice |

## Relationship To `v0.89`

This package exists because `v0.89` explicitly carried these surfaces forward instead of absorbing them silently.

That means:
- the `v0.89` carry-forward was real
- the follow-on package is now explicit
- the boundary between the milestones remains reviewable

## Review Guidance

- Treat `README.md`, `VISION_v0.89.1.md`, `DESIGN_v0.89.1.md`, `WBS_v0.89.1.md`, and `SPRINT_v0.89.1.md` as the canonical milestone planning package.
- Treat the files in `features/` as the promoted tracked feature commitments for the main `v0.89.1` band.
- Treat `WP_ISSUE_WAVE_v0.89.1.yaml` as the mechanical source for later issue creation once review is complete.
- Treat the remaining local `v0.89.1` planning inputs as planning material unless the milestone docs explicitly absorb them into a named work package.
- Treat contradictions between the planning package, promoted feature docs, and source mapping as defects.
