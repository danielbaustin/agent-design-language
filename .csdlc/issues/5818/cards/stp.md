# Structured Task Prompt

Template: 1.0.0

Issue: 5818

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver v0.92 current-version truth across docs/planning/ADL_FEATURE_LIST.md, canonical docs, READMEs, manifests, Cargo metadata, skills, and runbooks.

## Deliverables

- Machine-readable canonical-surface inventory with update, already_current, historical_preserve, or not_authoritative disposition
- Current docs and docs/planning/ADL_FEATURE_LIST.md aligned to active v0.92 without unlanded completion claims
- Authoritative 0.92.0 package and workspace version declarations with inspected lockfile regeneration
- Current link and C-SDLC v2 language repairs with historical records preserved
- Retained focused validation and exact-revision review packet

## Acceptance

1. Canonical inventory classifies every checked current or historical surface and names its owner and disposition
2. docs/planning/ADL_FEATURE_LIST.md and current entrypoints identify v0.92 as active without claiming planned features complete
3. All authoritative current version declarations agree on v0.92 or 0.92.0 and Cargo metadata is internally consistent
4. Historical milestone, release, review, migration, and evidence records retain their original version and claims
5. Current Markdown links plus YAML and JSON surfaces parse and resolve within their declared boundary
6. Current AGENTS.md, REVIEW.md, skills, and runbooks agree with final C-SDLC v2 authority
7. Focused diff, lockfile, placeholder, stale-reference, and exact-revision review checks pass

## Dependencies

- WP-01 issue #5817 and PR #5859 merged
- The WP-01 merge commit is ancestral to the implementation base
- The v0.92 issue wave and canonical milestone package remain current

## Inputs

- README.md
- docs/README.md
- docs/planning/ADL_FEATURE_LIST.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/WBS_v0.92.md
- docs/milestones/v0.92/QUALITY_GATE_v0.92.md
- docs/templates/prompts/current.json
- AGENTS.md
- REVIEW.md

## Non Goals

- v0.92 product implementation
- Historical evidence rewriting
- Repository migration or release ceremony
- Child issue execution or closeout
- Broad runtime validation unless executable version behavior changes
