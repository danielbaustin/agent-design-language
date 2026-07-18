# v0.91.7 Decisions

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Owner: ADL maintainers

## Purpose

Capture significant second-tranche implementation decisions and open questions.

## Decision Log

| ID | Decision | Status | Rationale | Impact | Link |
| --- | --- | --- | --- | --- | --- |
| D-01 | Treat `v0.91.7` as required pre-`v0.92` implementation/proof work. | accepted | The remaining activation surfaces are too important to leave to activation rediscovery. | Creates tracked docs and issue owners before `#3780` proceeds. | `#3825` |
| D-02 | Curiosity Engine requires governed discovery-cycle proof expectations. | accepted | Curiosity is a major idea but cannot be consumed as narrative alone. | Curiosity doc must define artifacts, hooks, budgets, governance, and proof. | `features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md` |
| D-03 | Constructability must distinguish provisional cognition from shared reality. | accepted | ADL needs a gate before internal hypotheses become external claims. | Constructability doc defines event, anchor, validator, and boundary expectations. | `features/CONSTRUCTABILITY_GATE_v0.91.7.md` |
| D-04 | `adl.skill.v1` requires a minimal implementation/proof path before v0.92 may consume it. | accepted | The future skill standard is large and needs proof-bound boundaries first. | Reasoning graph doc maps dependencies without overclaiming completion. | `features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md` |
| D-05 | Affect/happiness surfaces require safe-test and public-claim-boundary language. | accepted | Public evidence must not imply unproved wellbeing or inner-state claims. | Affect doc constrains `v0.92` consumption. | `features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md` |
| D-06 | Economics remains context-only unless an explicit test requirement is promoted. | accepted | Economics should not dominate birthday activation without proof decision. | Economics doc records the activation boundary. | `features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md` |
| D-07 | Treat `#4368` source capture as the planning authority refresh. | accepted | The earlier package predated later v0.91.6 sprint work and local TBD routing. | `PLANNING_SOURCE_CAPTURE_v0.91.7.md` becomes the source ledger for issue-wave promotion. | `#4368` |
| D-08 | Schedule SEP/VPP/PVF/template and goal/metrics work before relying on sprint-scale execution. | accepted | v0.92 needs predictable sprint execution, validation planning, watchers, and time/token accounting. | Process/tooling work becomes an early v0.91.7 gate. | `SPRINT_PLAN_v0.91.7.md` |
| D-09 | Treat scheduler/provider/local-agent and build/validation throughput as pre-birthday operational substrate. | accepted | C-SDLC compression exposed premium cognition and validation/build latency as bottlenecks. | Scheduler, provider, build, and validation work have explicit owners rather than being rediscovered during v0.92. | `PLANNING_SOURCE_CAPTURE_v0.91.7.md` |
| D-10 | Runtime Soak #2 and Observatory/demo readiness remain required handoff surfaces. | accepted | First birthday evidence needs visible runtime/workflow confidence, not planning prose alone. | Runtime and demo readiness are scheduled before v0.92 handoff. | `SPRINT_PLAN_v0.91.7.md` |

## Open Questions

The questions below are retained from milestone planning. Current
dispositions are recorded by closed WP-10 through WP-13 issues and their
review packets; any broader activation claim still depends on the remaining
release-tail gates.

- Which Curiosity proof becomes the first governed discovery-cycle issue?
- Which Constructability validators must block public/shared-reality claims?
- Which ACIP implementation decisions must be closed before `v0.92`, and which may be blocked with evidence?
- Which security requirements move to `v0.93` enterprise security with evidence and operator approval?

## Exit Criteria

- Milestone-critical decisions are logged with rationale.
- Open questions are resolved, operator-scoped-out with evidence and approval, or blocked with evidence and operator approval before activation.
