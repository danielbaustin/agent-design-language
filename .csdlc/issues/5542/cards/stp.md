# Structured Task Prompt

Template: 1.0.0

Issue: 5542

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only the three documented post-merge findings; do not execute sibling work packages or runtime/cloud work.

## Deliverables

- Repaired canonical closeout and bridge-precedence entrypoints
- Explicit creation/verification date semantics
- Extended executable WP-17 validator and fresh receipt

## Acceptance

1. AC-1: #4644 is closed and PR #5539 is merged in every touched canonical entrypoint
2. AC-2: WP-17 is absent from open work-package sets while WP-18, WP-19, WP-20, and WP-23 remain gates
3. AC-3: v0.92 consumption is routed through the reviewed v0.91.8 bridge with canonical links
4. AC-4: canonical date metadata distinguishes creation from last verification
5. AC-5: focused validator, links, structured docs, diff hygiene, and exact-revision review pass

## Dependencies

- Merged PR #5539
- Active #4645 ownership boundary for V0917_SPRINT_REVIEW_REGISTER.md

## Inputs

- README.md
- REVIEW.md
- docs/milestones/v0.91.7/README.md
- docs/milestones/v0.91.7/WBS_v0.91.7.md
- docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
- docs/planning/ADL_FEATURE_LIST.md
- docs/milestones/v0.91.8/README.md

## Non Goals

- Do not execute WP-18, WP-19, WP-20, or WP-23
- Do not modify Runtime, provider, cloud, AWS, Unity, GPU, or v0.92 activation behavior
- Do not rewrite retained historical evidence
- Do not edit the sprint-review register while #4645 owns it
