# v0.91.5 Decisions

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-02`
- Owner: ADL maintainers
- Status: `sprint_4_release_tail_active`

## Purpose

Record decisions that constrain v0.91.5 bridge work and Sprint 4 closeout.

## How To Use

Use these decisions as milestone guardrails. If execution or closeout needs to
change one, record a superseding decision rather than silently widening scope.

## Decision Log

| ID | Decision | Status | Rationale | Consequence |
| --- | --- | --- | --- | --- |
| D-01 | Use `v0.91.5`, not `v0.91.4A`. | accepted | The bridge is a real milestone with review and release truth. | Open side work moves out of v0.91.4 release scope. |
| D-02 | Move `#3377` to v0.91.5. | accepted | First-birthday readiness now depends on bridge outputs. | v0.92 consumes `#3377` after v0.91.5 closeout. |
| D-03 | Multi-agent must work before v0.91.5 closes or be explicitly blocked. | accepted | v0.92 needs reliable parallel C-SDLC operations. | Narrative-only multi-agent design is not sufficient. |
| D-04 | OpenRouter belongs in the provider matrix. | accepted | It gives fast access to many models for aptitude testing. | Provider identity and routing evidence must remain explicit. |
| D-05 | Public prompt records require transition controls. | accepted | Prompt cards are durable C-SDLC state, but local cache may contain cruft. | Export, redaction, archive, and deletion review are separate gates. |
| D-06 | v0.92 activation features must be mapped before v0.92 opens. | accepted | Many features were implemented earlier and will activate together in v0.92. | The activation map is a v0.91.5 exit artifact. |
| D-07 | WP-01 remains the milestone planning/setup gate. | accepted | Prior milestone structure depends on WP-01 seeding issues, cards, sprints, and ordering before implementation begins. | Implementation discovered during WP-01 must be scheduled into later WPs or sprint issues. |
| D-08 | Portable ADL adapter planning belongs in v0.91.5. | accepted | Paper repos, UTS, demos, and future projects need repo-local instructions and machine-readable adapter contracts. | `#3569` owns the contract/templates; WP-01 schedules it but does not implement external repo migration. |
| D-09 | Sprint umbrellas and closeout-tail issues are first-class WP-01 readiness artifacts. | accepted | v0.91.5 must not rely on prose-only sprint rows or pending release-tail placeholders after planning closes. | Seed sprint umbrella issues `#3571`-`#3574`, closeout-tail issues `#3575`, `#3579`, `#3576`, `#3580`, `#3577`, `#3581`, and `#3578`, and record the reusable ordering standard in `docs/planning/ADL_MILESTONE_WP_ORDERING_STANDARD.md`. |

## Open Questions

- Which local and remote Ollama models are usable for each C-SDLC role?
- Does Celestial Rescue become Unity Observatory proof or only prepare it?
- Which `.adl` historical records should move out of repo entirely versus be
  exported into public prompt packets or ObsMem ingestion bundles?

## Exit Criteria

- Decisions are reflected in WBS, sprint, demo, checklist, and issue wave docs.
- Any superseding decisions are recorded rather than implied by implementation.
