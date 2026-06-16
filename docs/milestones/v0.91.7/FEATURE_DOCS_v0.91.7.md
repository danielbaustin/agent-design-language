# v0.91.7 Feature-Doc Index

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-16`
- Setup issue: `#3801`
- Documentation completion issue: `#3825`

## Status

Feature-doc package created for the second pre-`v0.92` bridge tranche. These
docs define planning, decisions, validation expectations, and `v0.92`
consumption limits; they do not implement runtime behavior.

## Required Feature Docs And Bridge Records

| Feature doc | Surface | Required questions | Exit state before v0.92 |
| --- | --- | --- | --- |
| [`CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md`](features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md) | Curiosity Engine / Discovery Substrate | What artifacts, hooks, hypotheses, budgets, governance, Freedom Gate, ObsMem/reasoning-graph updates, and proof are required? | Bridge doc exists; governed proof remains future issue work. |
| [`CONSTRUCTABILITY_GATE_v0.91.7.md`](features/CONSTRUCTABILITY_GATE_v0.91.7.md) | Constructability Gate | What construction events, external anchors, validators, and shared-reality boundaries are required? | Bridge doc exists; validator proof remains future issue work. |
| [`REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md`](features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md) | Reasoning graph / loop runtime / `adl.skill.v1` | How do prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1` connect before v0.92? | Bridge doc exists; full standard remains later work. |
| [`SECURITY_RESIDUAL_READINESS_v0.91.7.md`](features/SECURITY_RESIDUAL_READINESS_v0.91.7.md) | Residual security readiness | What remains after v0.91.6 security/CAV, and what blocks activation? | Bridge doc exists; blockers must be named by implementation issues. |
| [`ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md`](features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md) | Residual ACIP/A2A/protobuf decisions | Which JSON/protobuf/WebSocket/access-rule choices remain, and what can v0.92 consume? | Bridge doc exists; ambiguous protocol posture remains a blocker. |
| [`AFFECT_HAPPINESS_BRIDGE_v0.91.7.md`](features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md) | Affect and happiness surfaces | What safe tests and non-claims govern affect, humor, happiness, and wellbeing evidence? | Bridge doc exists; public evidence remains bounded by non-claims. |
| [`GODEL_MECHANICS_BRIDGE_v0.91.7.md`](features/GODEL_MECHANICS_BRIDGE_v0.91.7.md) | Godel mechanics | What experiment, hypothesis, mutation, evaluation, and promotion mechanics can birthday evidence consume? | Bridge doc exists; runtime mechanics remain future issue work. |
| [`ECONOMICS_CONTEXT_DECISION_v0.91.7.md`](features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md) | Economics context | Is economics context-only for v0.92, or does it require explicit tests? | Bridge doc exists; default posture is context-only unless promoted. |

## Cross-Doc Requirements

- Every doc must name non-goals and unsupported claims.
- Every doc must include validation and review expectations.
- Every doc must say what `#3780` / `v0.92` may consume.
- Security, ACIP/A2A, Curiosity, Constructability, and reasoning graphs must
  not be collapsed into generic future-work language.
- `#3780` consumption truth is summarized in `V092_HANDOFF_v0.91.7.md`.

## Validation

When this index is consumed:

- verify each planned implementation/proof route has an owning issue or
  explicit blocked/deferred route
- scan for `v0.92` readiness overclaims
- scan for local authoring-workspace links or host-local paths
- verify all second-tranche surfaces remain visible
