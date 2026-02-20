# ADL v0.5 Decisions Log

## Metadata
- Milestone: `v0.5`
- Version: `0.5`
- Date: `2026-02-18`
- Owner: `Daniel Austin`

## Purpose
Capture architectural, language, runtime, and process decisions that define v0.5. This milestone focuses on maturing ADL as a language (explicit primitive schemas + composition rules) while evolving the runtime scheduler in a deterministic and configurable way.

## Decision Log
| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|----|----------|--------|-----------|-------------|--------|------|
| D-01 | Treat ADL as a first-class language with explicit schemas for all 6 primitives (Agents, Runs, Providers, Tasks, Tools, Workflows). | accepted | Prevent runtime-driven drift and implicit structure; enable validation, tooling, and composition. | Continue implicit struct-based modeling only. | Enables schema validation, better docs, stronger demos. | #330 |
| D-02 | Separate “primitive definition” from “composition rules.” | accepted | Clarifies language core vs orchestration semantics; reduces coupling. | Embed composition rules inside workflow-only schema. | Cleaner mental model and easier extension. | #330 |
| D-03 | Runtime concurrency must remain deterministic even when configurable. | accepted | Determinism is foundational to trace + replay guarantees. | Allow nondeterministic scheduling for performance. | Preserves replay invariants and CI stability. | v0.4 precedent |
| D-04 | Configurable concurrency limit (schema → runtime) will land in v0.5 but default remains conservative. | accepted | Adds flexibility without breaking deterministic behavior. | Keep hardcoded MAX_PARALLEL indefinitely. | Expands runtime usability while preserving stability. | v0.5 WP-05 |
| D-05 | v0.5 includes a formal Demo Matrix exercising each primitive alone and in composition. | accepted | Demos are now first-class deliverables, not afterthoughts. | Ad hoc example growth. | Improves clarity, onboarding, and release credibility. | v0.5 docs |
| D-06 | Milestones follow fixed ceremony structure: Init → Work Units → Demo Pass → Doc Pass → Review → Closing Ceremony. | accepted | Prevents drift and partial completion. | Flexible / informal sprint structure. | Predictable release quality and cadence. | Process decision |
| D-07 | Observable memory + Bayesian indexing is deferred to v0.6 (or separate module if scope expands). | deferred | Avoid mixing language/runtime stabilization with experimental memory system. | Ship memory system inside v0.5. | Keeps v0.5 focused and finishable. | Future epic |

## Open Questions
- Should concurrency configuration live at the Run level, Workflow level, or both? (Owner: Daniel) (Issue: TBD)
- Should composition rules be validated statically (compile phase) or partially at runtime? (Owner: Daniel) (Issue: TBD)
- Do we introduce a formal "adl validate" command in v0.5? (Owner: Daniel) (Issue: TBD)

## Exit Criteria
- All language-level and scheduler-level decisions for v0.5 are explicitly recorded.
- Deferred items clearly moved to future milestone.
- No structural ambiguity remains about primitive schemas or composition model.
