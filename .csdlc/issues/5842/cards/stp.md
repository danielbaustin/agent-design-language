# Structured Task Prompt

Template: 1.0.0

Issue: 5842

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver quality gate that blocks internal review until every indexed v0.92 feature is landed with accepted exact-revision proof.

## Deliverables

- quality gate that blocks internal review until every indexed v0.92 feature is landed with accepted exact-revision proof
- feature-completion matrix, quality-gate record, platform proof, and blocker report

## Acceptance

1. The declared required outcome is complete at the exact reviewed revision
2. Declared dependencies are verified from current evidence
3. The named proof surface is reproducible and retained
4. Applicable negative, failure, security, privacy, portability, and claim boundaries are tested or dispositioned
5. One bounded pre-PR review has no unresolved actionable findings

## Dependencies

- WP-04
- WP-05
- WP-06
- WP-07
- WP-13A
- WP-20
- WP-21
- WP-21A

## Inputs

- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/WBS_v0.92.md

## Non Goals

- Adjacent work packages
- Historical evidence rewriting
- Unsupported downstream milestone claims
