# Structured Task Prompt

Template: 1.0.0

Issue: 5653

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Update and validate only the root README.

## Deliverables

- updated README
- design and diagram
- focused Markdown/link proof

## Acceptance

1. AC-1: README names the current v0.91.8 release-tail posture without claiming an unpublished release.
2. AC-2: README visibly links to https://agent-logic.ai.
3. AC-3: CI and coverage badges remain pointed at the canonical main branch.
4. AC-4: No stale v0.91.5-only status claim remains in the README.
5. AC-5: Focused Markdown/link checks pass and the exact head receives review.

## Dependencies

- current v0.91.8 release-tail and CI evidence

## Inputs

- README.md
- docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/RELEASE_NOTES_v0.91.8.md

## Non Goals

- creating a release
- creating a tag
- changing CI
- runtime/product changes
- release approval
