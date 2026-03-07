# Release Notes Template

## Metadata
- Product: `{{product_name}}`
- Version: `{{version}}`
- Release date: `{{release_date}}`
- Tag: `{{tag_name}}`

## How To Use
- Keep statements implementation-accurate and test-validated.
- Prefer concise bullets over marketing language.
- Explicitly separate shipped behavior from "What's Next."

# `{{product_name}}` `{{version}}` Release Notes

## Summary
{{summary_paragraph}}

## Highlights
- {{highlight_1}}
- {{highlight_2}}
- {{highlight_3}}

## What's New In Detail

### {{area_1}}
- {{detail_1a}}
- {{detail_1b}}

### {{area_2}}
- {{detail_2a}}
- {{detail_2b}}

### {{area_3}}
- {{detail_3a}}
- {{detail_3b}}

## Upgrade Notes
- {{upgrade_note_1}}
- {{upgrade_note_2}}

## Known Limitations
- {{limitation_1}}
- {{limitation_2}}

## Validation Notes
- {{validation_note_1}}
- {{validation_note_2}}

## What's Next
- {{next_1}}
- {{next_2}}

## Exit Criteria
- Notes reflect only shipped behavior.
- Known limitations and future work are explicitly separated.
- Final text is ready to paste into GitHub Release UI without further editing.

# ADL v0.8 Release Notes

## Metadata
- Product: Agent Design Language (ADL)
- Version: v0.8
- Release date: TBD
- Tag: v0.8

## Summary
ADL v0.8 introduces the foundations for self-improving agents using a Gödel-style scientific learning loop. This release formalizes experiment tracking, mutation formats, and agent profile schemas that allow deterministic evaluation and controlled policy evolution. The milestone also consolidates documentation around memory, experimentation, and authoring surfaces while preserving deterministic replay guarantees established in earlier releases.

## Highlights
- Gödel scientific learning loop foundations (experiment → evaluation → mutation)
- Canonical schemas for agent profiles and mutations
- Deterministic experiment and evidence structures

## What's New In Detail

### Gödel Scientific Loop Foundations
- Introduces a documented scientific loop for agents: failure detection → hypothesis → mutation → evaluation.
- Establishes structured experiment artifacts to support reproducible agent improvement cycles.

### Canonical Experiment Artifacts
- Defines ExperimentRecord schema for tracking experiments and outcomes.
- Introduces a Canonical Evidence View to normalize evaluation evidence across runs.

### Agent Policy Evolution Surface
- Adds `agent_profile.v1` schema describing prompts, routing, planner configuration, memory policy, and executor policy.
- Adds `mutation.v1` schema for bounded policy mutations applied during experiments.

## Upgrade Notes
- No breaking runtime changes expected for workflows created in v0.75.
- The new schemas are introduced as design-stage artifacts and may later move to runtime schema locations.

## Known Limitations
- Gödel experiment orchestration is defined but not fully automated yet.
- Adaptive policy learning loops remain experimental and may evolve in future milestones.

## Validation Notes
- All schema artifacts follow strict JSON Schema definitions.
- Design documents explicitly define deterministic replay and experiment trace requirements.

## What's Next
- Runtime implementation of the Gödel experiment loop.
- Adaptive policy learning and automated mutation evaluation.

## Exit Criteria
- Notes reflect only shipped behavior.
- Known limitations and future work are explicitly separated.
- Final text is ready to paste into GitHub Release UI without further editing.