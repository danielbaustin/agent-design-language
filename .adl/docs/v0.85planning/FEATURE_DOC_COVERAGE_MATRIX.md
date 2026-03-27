# Feature Doc Coverage Matrix

## Purpose

This table maps the currently planned feature bands to the docs that define
them. The goal is to ensure every planned feature area has at least one
governing document and to make `1:1` vs `n:1` coverage explicit.

Coverage meanings:

- `1:1` = one primary doc clearly owns the feature
- `n:1` = several docs together define one feature area or milestone band
- `partial` = some doc coverage exists, but the feature is still underspecified
- `gap` = no clear governing feature doc yet

## Matrix

| Roadmap band / feature area | Current governing docs | Coverage | Status | Notes |
| --- | --- | --- | --- | --- |
| `v0.85` execution substrate | `CLUSTER_EXECUTION.md`, `SWARM_REMOVAL_PLANNING.md` | `n:1` | covered | Runtime/ops feature area is spread across more than one doc. |
| `v0.85` dependable execution / verifiable inference | `DESIGN_v0.85.md`, `WBS_v0.85.md`, `DEMO_MATRIX_v0.85.md` | `n:1` | covered | Controlled by milestone docs rather than one deep feature spec. |
| `v0.85` authoring/editor surfaces | `EDITING_ARCHITECTURE.md`, `HTA_EDITOR_PLANNING.md`, `STRUCTURED_PROMPT_ARCHITECTURE.md` | `n:1` | covered | Strong multi-doc feature area, already promoted to milestone canon. |
| `v0.85` Layer 8 / provider-contract maturation | `LAYER_8_IMPLEMENTATION.md` | `1:1` | covered | One clear primary feature doc. |
| `v0.85` reasoning graph schema | `REASONING_GRAPH_SCHEMA_V0.85.md` | `1:1` | covered | Clear single governing schema doc. |
| `v0.85` affect signal model | `AFFECTIVE_REASONING_MODEL.md` | `1:1` | covered | Canonical short milestone doc. |
| `v0.85` bounded affect subsystem | `AFFECT_MODEL_v0.85.md`, `BOUNDED_AFFECT_MODEL.md` | `n:1` | covered | One implementation-oriented doc plus one conceptual framing doc. |
| `v0.85` cognitive framing in milestone canon | `COGNITIVE_LOOP_MODEL_v0.85.md`, `COGNITIVE_STACK_v0.85.md` | `n:1` | covered | Architecture reference, not proof of runtime completeness. |
| `v0.86` cognitive control | `COGNITIVE_LOOP_MODEL.md`, `COGNITIVE_STACK.md`, `COGNITIVE_ARBITRATION.md`, `FAST_SLOW_THINKING_MODEL.md`, `FREEDOM_GATE.md` | `n:1` | covered | Core cognition/control band is intentionally multi-doc. |
| `v0.88` persistence / instinct / bounded agency | `SUBSTANCE_OF_TIME.md`, `PHI_METRICS_FOR_ADL.md`, `WP_INSTINCT_AND_BOUNDED_AGENCY.md`, `INSTINCT_MODEL.md`, `INSTINCT_RUNTIME_SURFACE.md` | `n:1` | covered | Persistence and bounded-agency band is intentionally multi-doc. |
| `v0.89` AEE convergence / security | `AEE_CONVERGENCE_MODEL.md`, `SECURITY_AND_THREAT_MODELING.md` | `n:1` | covered | Convergence and bounded trust/risk are paired in this band. |
| `v0.90` reasoning graph / signed trace / query | `HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md`, `SIGNED_TRACE_ARCHITECTURE.md`, `TRACE_QUERY_LANGUAGE.md` | `n:1` | covered | Reasoning graph observability/provenance band is intentionally multi-doc. |
| `v0.91` affect / moral cognition | `AFFECT_MODEL_v0.90.md`, `KINDNESS_MODEL.md`, `MORAL_RESOURCES_SUBSYSTEM.md`, `HUMOR_AND_ABSURDITY.md` | `n:1` | covered | Multi-doc band by design. |
| `v0.92` identity / continuity / provider capability | `ADL_IDENTITY_ARCHITECTURE.md`, `ADL_PROVIDER_CAPABILITIES.md`, `NARRATIVE_IDENTITY_CONTINUITY.md` | `n:1` | covered | Identity/capability/continuity are now one coherent band. |
| `v0.93` governance / delegation | `ADL_CONSTITUTIONAL_DELEGATION.md`, `ADL_AGENT_SOCIAL_CONTRACT.md`, `ADL_AGENT_RIGHTS_AND_DUTIES.md`, `V09_CONSTITUTIONAL_DELEGATION_WORKPLAN.md` | `n:1` | covered | Governance band is inherently multi-doc. |
| `v0.95` MVP convergence / tooling migration | `MVP_WALKTHROUGH_AND_DEMOS.md`, `PLATFORM_CONVERGENCE_PLAN.md`, `TOOLING_RUST_MIGRATION_PLAN.md`, `ZED_INTEGRATION_WITH_ADL.md` | `n:1` | covered | Convergence band combines demo closure, platform integration, tooling hardening, and optional Zed work. |

## Ownership View

| Feature area | Primary owner doc | Supporting docs | Follow-up doc needed? | Notes |
| --- | --- | --- | --- | --- |
| `v0.85` execution substrate | `CLUSTER_EXECUTION.md` | `SWARM_REMOVAL_PLANNING.md`, milestone PMO docs | no | Multi-surface but adequately bounded for `v0.85`. |
| `v0.85` authoring/editor surfaces | `EDITING_ARCHITECTURE.md` | `HTA_EDITOR_PLANNING.md`, `STRUCTURED_PROMPT_ARCHITECTURE.md` | no | Strong enough as an `n:1` bundle. |
| `v0.85` Layer 8 / provider-contract maturation | `LAYER_8_IMPLEMENTATION.md` | milestone PMO docs | no | Clear owner doc exists. |
| `v0.85` reasoning graph schema | `REASONING_GRAPH_SCHEMA_V0.85.md` | `AFFECTIVE_REASONING_MODEL.md` | no | Schema ownership is clear. |
| `v0.85` bounded affect | `AFFECT_MODEL_v0.85.md` | `BOUNDED_AFFECT_MODEL.md`, `AFFECTIVE_REASONING_MODEL.md` | no | One implementation-oriented doc plus conceptual framing. |
| `v0.86` cognitive control | `COGNITIVE_LOOP_MODEL.md` | `COGNITIVE_STACK.md`, `COGNITIVE_ARBITRATION.md`, `FAST_SLOW_THINKING_MODEL.md`, `FREEDOM_GATE.md` | no | Bundle is dense but coherent. |
| `v0.88` persistence / instinct / bounded agency | `INSTINCT_MODEL.md` | `SUBSTANCE_OF_TIME.md`, `PHI_METRICS_FOR_ADL.md`, `WP_INSTINCT_AND_BOUNDED_AGENCY.md`, `INSTINCT_RUNTIME_SURFACE.md` | no | Persistence and bounded-agency surfaces are intentionally grouped. |
| `v0.89` AEE convergence / security | `AEE_CONVERGENCE_MODEL.md` | `SECURITY_AND_THREAT_MODELING.md` | no | Security/threat modeling now sits beside the AEE convergence story. |
| `v0.90` reasoning graph / signed trace / query | `HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md` | `SIGNED_TRACE_ARCHITECTURE.md`, `TRACE_QUERY_LANGUAGE.md` | no | Reasoning graph observability/provenance cluster. |
| `v0.91` affect / moral cognition | `AFFECT_MODEL_v0.90.md` | `KINDNESS_MODEL.md`, `MORAL_RESOURCES_SUBSYSTEM.md`, `HUMOR_AND_ABSURDITY.md` | no | Intentionally multi-doc. |
| `v0.92` identity / continuity / provider capability | `ADL_IDENTITY_ARCHITECTURE.md` | `ADL_PROVIDER_CAPABILITIES.md`, `NARRATIVE_IDENTITY_CONTINUITY.md`, `LAYER_8_IMPLEMENTATION.md` | no | Identity/capability/continuity are intentionally grouped. |
| `v0.93` governance / delegation | `ADL_CONSTITUTIONAL_DELEGATION.md` | `ADL_AGENT_SOCIAL_CONTRACT.md`, `ADL_AGENT_RIGHTS_AND_DUTIES.md`, `V09_CONSTITUTIONAL_DELEGATION_WORKPLAN.md` | no | Multi-doc by nature. |
| `v0.95` MVP convergence / tooling migration | `PLATFORM_CONVERGENCE_PLAN.md` | `MVP_WALKTHROUGH_AND_DEMOS.md`, `TOOLING_RUST_MIGRATION_PLAN.md`, `ZED_INTEGRATION_WITH_ADL.md` | no | Convergence band closes the roadmap and carries optional Zed work. |

## Net Assessment

- Most planned feature areas now have at least one governing doc.
- Several major bands are intentionally `n:1` rather than `1:1`:
  - `v0.86` cognitive control
  - `v0.88` persistence / instinct / bounded agency
  - `v0.90` reasoning graph / signed trace / query
  - `v0.91` affect / moral cognition
  - `v0.92` identity / continuity / provider capability
  - `v0.93` governance / delegation
  - `v0.95` MVP convergence / tooling migration
- The roadmap is now more balanced because the old `v0.86` catch-all future band has been redistributed into clearer cognitive, persistence, convergence, reasoning, identity, and governance slices.

## v0.86 Note

`v0.86` is now intentionally narrow: it is the cognition-control entry band,
not an authoring/process catch-all. The authoring lifecycle work that used to
make this band feel crowded has been left in the `v0.85` milestone canon and in
the roadmap/control documents rather than treated as a future feature-doc band
of its own.

## Recommended Rule

For future reconciliation and milestone-start work:

- every roadmap feature area should have either:
  - one primary governing doc, or
  - an explicit `n:1` doc bundle recorded in this matrix
- if neither is true, treat the feature area as under-documented until the gap
  is resolved
