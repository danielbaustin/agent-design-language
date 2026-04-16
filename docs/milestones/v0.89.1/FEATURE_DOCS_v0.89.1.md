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
- which concept/backgrounder docs are promoted into `ideas/` rather than `features/`
- how the `v0.89` carry-forward corpus maps into this follow-on band

## Ideas / Backgrounder Lane

`ideas/README.md` indexes reader-visible rationale that supports the milestone
without becoming a standalone feature commitment.

Use `features/` for executable commitments, schemas, demos, runtime contracts,
or proof surfaces. Use `ideas/` for supporting governance and coordination
concepts such as refusal, delegation, and multi-agent negotiation.

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

WP-08 proof hooks:
- `adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json`
- `adl identity skill-composition --out .adl/state/skill_composition_model_v1.json`

The bounded `arxiv-paper-writer` skill is part of the WP-08 operational substrate and composition contracts. The later three-paper manuscript packet remains owned by `WP-13`.

WP-09 proof hook:
- `adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json`

The delegation/refusal and negotiation notes are tracked in `ideas/` as
supporting backgrounders rather than promoted feature docs. WP-09 integrates
their bounded runtime distinctions into a repo-visible contract so reviewers can
see delegation, refusal, approval gates, and coordination outcomes without
over-claiming final constitutional or negotiation governance.

WP-10 proof hook:
- `adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json`

WP-10 keeps the existing provider substrate capability metadata in the milestone as a bounded packaging/proof surface. It does not promote the broader provider-security extension into `v0.89.1`; provider attestation, trust-tier scoring, network posture enforcement, secret lifecycle enforcement, provider sandboxing, and external provider-security demos remain later security-extension work.

WP-11 proof hook:
- `adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json`

WP-11 makes the demo matrix copy/paste-ready by collecting the landed adversarial-runtime, replay, verification, skill-governance, provider-packaging, and flagship demo entry points into one reviewer-facing contract. It keeps the five-agent Hey Jude integration demo and final manuscript convergence owned by `WP-13`.

WP-12 convergence note:
- the promoted feature-doc band is considered landed through `WP-11`
- remaining `v0.89.1` work should consume these feature docs as source-of-truth proof surfaces rather than reopening their scope
- `WP-13` consumes this convergence in the integration demo package, five-agent Hey Jude demo, and three-paper manuscript packet
- full provider-security extension, broader long-lived-agent runtime work, and later governance/identity themes remain follow-on scope outside this milestone

WP-13 proof hook:
- `bash adl/tools/demo_v0891_wp13_demo_integration.sh`

WP-13 lands the D7/D8/D9 integration packet, the five-agent Hey Jude MIDI demo, and the bounded three-paper manuscript workflow packet. It records the no-submission boundary for the publication track and consumes the merged WP-12 convergence state without reopening feature scope.

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
| `DELEGATION_AND_REFUSAL.md` | promoted idea/backgrounder | `ideas/DELEGATION_AND_REFUSAL.md`; integrated into the bounded `WP-09` delegation/refusal/coordination contract without promotion as a standalone feature doc |
| `MULTI_AGENT_NEGOTIATION.md` | promoted idea/backgrounder | `ideas/MULTI_AGENT_NEGOTIATION.md`; integrated into the bounded `WP-09` coordination and disagreement surface without final negotiation-law claims |
| internal operational-skills process notes | internal-only supporting input | not promoted to tracked reader docs; inform internal skill-surface packaging discipline only |
| local arXiv paper-program planning doc | supporting planning input | informs the committed `WP-08` bounded `arxiv-paper-writer` skill and the `WP-13` three-paper publication packet |
| `ADL_SECURITY_DEMOS.md` | under-authored supporting input | do not promote until authored; informs later demo packaging |
| `PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md` | under-authored supporting input | not promoted by `WP-10`; candidate later security-extension slice |

## Relationship To `v0.89`

This package exists because `v0.89` explicitly carried these surfaces forward instead of absorbing them silently.

That means:
- the `v0.89` carry-forward was real
- the follow-on package is now explicit
- the boundary between the milestones remains reviewable

## Review Guidance

- Treat `README.md`, `VISION_v0.89.1.md`, `DESIGN_v0.89.1.md`, `WBS_v0.89.1.md`, and `SPRINT_v0.89.1.md` as the canonical milestone planning package.
- Treat the files in `features/` as the promoted tracked feature commitments for the main `v0.89.1` band.
- Treat the files in `ideas/` as conceptual backgrounders and rationale, not feature commitments.
- Treat `WP_ISSUE_WAVE_v0.89.1.yaml` as the mechanical source for later issue creation once review is complete.
- Treat the remaining local `v0.89.1` planning inputs as planning material unless the milestone docs explicitly absorb them into a named work package.
- Treat contradictions between the planning package, promoted feature docs, and source mapping as defects.
