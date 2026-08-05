# Structured Task Prompt

Template: 1.0.0

Issue: 5763

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Recompute and update only stale digest metadata required to restore feature-crosswalk validation.

## Deliverables

- Updated crosswalk digest metadata
- Issue-local typed lifecycle/evidence records
- Ready PR closing #5763

## Acceptance

1. AC-1: Recompute and review the canonical 122-row digest without weakening the guard
2. AC-2: ruby .csdlc/prepared/issues/5594/validate_feature_crosswalk.rb passes
3. AC-3: Existing structured planning and link validators pass
4. AC-4: YAML parse and diff hygiene pass where relevant
5. AC-5: One bounded pre-PR GPT-5.5 review is clean
6. AC-6: PR includes Closes #5763

## Dependencies

- Current origin/main at 57d115741f32b945217ee3cb14188b41ebde9b3f
- Reviewed WP-14 decomposition source-row and matching crosswalk entry already present on main
- .csdlc/prepared/issues/5594/validate_feature_crosswalk.rb

## Inputs

- GitHub issue #5763
- docs/planning/ADL_FEATURE_LIST.md
- docs/milestones/v0.91.8/feature_preservation_crosswalk_5594.v1.json
- .csdlc/prepared/issues/5594/validate_feature_crosswalk.rb
- .csdlc/prepared/issues/5594/validate_structured_planning.rb
- .csdlc/prepared/issues/5594/validate_links.rb

## Non Goals

- No new feature-list rows
- No crosswalk regeneration beyond stale digest metadata
- No validator weakening
- No unrelated docs, planning, GitHub routing, or lifecycle cleanup
