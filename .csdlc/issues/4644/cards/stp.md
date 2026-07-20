# Structured Task Prompt

Template: 1.0.0

Issue: 4644

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare and later execute only WP-17; do not absorb sibling WPs or release-readiness claims.

## Deliverables

- Docs alignment packet
- Repaired planning surfaces

## Acceptance

1. AC-1: WP-17 executes only within its declared scope and protected output paths
2. AC-2: Deliverables are retained on tracked paths with evidence-bound claims
3. AC-3: Validation records distinguish fresh checks, retained proof, skipped checks, and unproven surfaces
4. AC-4: Sibling WP work, v0.91.7 release readiness, and v0.92 activation readiness are not silently claimed

## Dependencies

- WP-16 quality-gate output

## Inputs

- docs/milestones/v0.91.7/README.md
- docs/milestones/v0.91.7/WBS_v0.91.7.md
- docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
- docs/planning/ADL_FEATURE_LIST.md

## Non Goals

- Do not execute this work during preparation
- Do not implement sibling WP work
- Do not use AWS or paid remote validation lanes for preparation
- Do not claim release readiness from setup cards
