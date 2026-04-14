# Feature Docs - v0.89

## Metadata
- Milestone: `v0.89`
- Version: `v0.89`
- Date: `2026-04-13`
- Owner: `Daniel Austin`

## Purpose

Provide the canonical feature index for `v0.89`.

This page defines:
- which `v0.89` feature docs are promoted into the tracked milestone package now
- which local planning docs remain supporting inputs but are not yet promoted
- which security-heavy or adversarial runtime docs are intentionally carried into `v0.89.1`

The goal is to eliminate floating feature notes. Every source planning doc should have an explicit implementation home or reserved later-band home.

## Scope Interpretation

`v0.89` is the milestone where ADL turns bounded cognition into governed adaptive behavior.

The core tracked feature band is:
- AEE 1.0 convergence
- Freedom Gate v2
- explicit decision and action-mediation surfaces
- bounded skill execution as a governed runtime contract
- experiment-system continuation for Gödel-style improvement
- evidence-aware ObsMem retrieval
- security, trust, and posture as explicit runtime planning surfaces

This package intentionally does not absorb the full adversarial runtime and exploit package into the main `v0.89` core. That package is planned in the explicit `v0.89.1` follow-on band.

## Promoted Tracked Feature Docs

| Feature doc | Primary concern | Planned WPs |
|---|---|---|
| `features/AEE_CONVERGENCE_MODEL.md` | bounded convergence, stop conditions, adaptation legibility | `WP-02` |
| `features/FREEDOM_GATE_V2.md` | richer governed allow/defer/refuse/escalate boundary | `WP-03` |
| `features/DECISION_SURFACES.md` | explicit workflow decision points | `WP-04` |
| `features/DECISION_SCHEMA.md` | decision-event record contract | `WP-04` |
| `features/ACTION_MEDIATION_LAYER.md` | authority boundary between model intent and runtime action | `WP-05` |
| `features/ACTION_PROPOSAL_SCHEMA.md` | canonical non-authoritative proposal contract | `WP-05` |
| `features/SKILL_MODEL.md` | canonical skill definition and boundaries | `WP-06` |
| `features/SKILL_EXECUTION_PROTOCOL.md` | invocation lifecycle, I/O, trace expectations | `WP-06` |
| `features/GODEL_EXPERIMENT_SYSTEM.md` | experiment records and bounded adopt/reject logic | `WP-07` |
| `features/OBSMEM_EVIDENCE_AND_RANKING.md` | evidence-aware retrieval and ranking explanations | `WP-08` |
| `features/SECURITY_AND_THREAT_MODELING.md` | trust boundaries, abuse models, mitigations | `WP-09` |
| `features/ADL_SECURITY_POSTURE_MODEL.md` | declared posture as an execution contract | `WP-09` |
| `features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md` | trust assumptions under contested operation | `WP-09` |

## Source Planning Corpus -> Implementation Home

### Core `v0.89` source docs

| Local source doc | Disposition | Implementation home |
|---|---|---|
| `.adl/docs/v0.89planning/AEE_CONVERGENCE_MODEL.md` | promoted | `v0.89 / WP-02` |
| `.adl/docs/v0.89planning/FREEDOM_GATE_V2.md` | promoted | `v0.89 / WP-03` |
| `.adl/docs/v0.89planning/DECISION_SURFACES.md` | promoted | `v0.89 / WP-04` |
| `.adl/docs/v0.89planning/DECISION_SCHEMA.md` | promoted | `v0.89 / WP-04` |
| `.adl/docs/v0.89planning/ACTION_MEDIATION_LAYER.md` | promoted | `v0.89 / WP-05` |
| `.adl/docs/v0.89planning/ACTION_PROPOSAL_SCHEMA.md` | promoted | `v0.89 / WP-05` |
| `.adl/docs/v0.89planning/SKILL_MODEL.md` | promoted | `v0.89 / WP-06` |
| `.adl/docs/v0.89planning/SKILL_EXECUTION_PROTOCOL.md` | promoted | `v0.89 / WP-06` |
| `.adl/docs/v0.89planning/GODEL_EXPERIMENT_SYSTEM.md` | promoted | `v0.89 / WP-07` |
| `.adl/docs/v0.89planning/OBSMEM_EVIDENCE_AND_RANKING.md` | promoted | `v0.89 / WP-08` |
| `.adl/docs/v0.89planning/SECURITY_AND_THREAT_MODELING.md` | promoted | `v0.89 / WP-09` |
| `.adl/docs/v0.89planning/ADL_SECURITY_POSTURE_MODEL.md` | promoted | `v0.89 / WP-09` |
| `.adl/docs/v0.89planning/ADL_TRUST_MODEL_UNDER_ADVERSARY.md` | promoted | `v0.89 / WP-09` |
| `.adl/docs/v0.89planning/ADL_AND_REASONABLENESS.md` | supporting planning input | informs `v0.89` governance framing; broader home later `v0.93` |
| `.adl/docs/v0.89planning/ADL_CONSTITUTION.md` | supporting planning input | later constitutional/governance band `v0.93` |
| `.adl/docs/v0.89planning/ADL_LEARNING_MODEL.md` | supporting planning input | later learning/identity band `v0.92+` |
| `.adl/docs/v0.89planning/APTITUDE_MODEL.md` | not in `v0.89` core | `v0.92` capability/identity band |
| `.adl/docs/v0.89planning/CONSTITUTIONAL_SURFACE_MAP.md` | supporting planning input | later governance band `v0.93` |
| `.adl/docs/v0.89planning/GHB_ALGORITHM_AND_STATE_SPACE_COMPRESSION.md` | supporting planning input | later reasoning band `v0.90` |
| `.adl/docs/v0.89planning/GHB_EXECUTION_MODEL.md` | supporting planning input | later reasoning band `v0.90` |
| `.adl/docs/v0.89planning/GOVERNANCE_CLUSTER_MAP.md` | local planning map | stays local until broader governance promotion |
| `.adl/docs/v0.89planning/LEARNING_AND_SKILLS_CLUSTER_MAP.md` | local planning map | stays local until broader learning/skills promotion |
| `.adl/docs/v0.89planning/REASONING_PATTERNS_CATALOG.md` | supporting planning input | later reasoning band `v0.90` |

### `v0.89.1` carry-forward package

The carry-forward is now represented by a tracked planning package under:

- `docs/milestones/v0.89.1/`

The rows below remain useful because they identify the original local source inputs for that package.

| Local source doc | Disposition | Implementation home |
|---|---|---|
| `.adl/docs/v0.89.1planning/ADL_ADVERSARIAL_RUNTIME_MODEL.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/ADL_ADVERSARIAL_DEMO.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/ADL_SECURITY_DEMOS.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/ADVERSARIAL_EXECUTION_RUNNER.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/ADVERSARIAL_REPLAY_MANIFEST.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/CONTINUOUS_VERIFICATION_AND_EXPLOIT_GENERATION.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/EXPLOIT_ARTIFACT_SCHEMA.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/RED_BLUE_AGENT_ARCHITECTURE.md` | carry forward | `v0.89.1` |
| `.adl/docs/v0.89.1planning/SELF_ATTACKING_SYSTEMS.md` | carry forward | `v0.89.1` |

## Review Guidance

- Treat `README.md`, `VISION_v0.89.md`, `DESIGN_v0.89.md`, `WBS_v0.89.md`, and `SPRINT_v0.89.md` as the canonical milestone planning package.
- Treat the files in `features/` as the promoted tracked feature commitments for the main `v0.89` band.
- Treat the remaining `.adl/docs/v0.89planning/*` and `.adl/docs/v0.89.1planning/*` docs as local planning inputs, not already-shipped promises.
- Treat contradictions between the planning package, promoted feature docs, and carry-forward mapping as defects.
- Treat every promoted feature doc as an engineering commitment that must resolve to code, tests, artifacts, demos, or explicit defer records.
