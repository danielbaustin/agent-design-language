# ADR 0026 Candidate: Repo Visibility Manifest And Linkage Layer

- Status: Candidate
- Target milestone: v0.91.2
- Related issue: #3124
- Related ADRs: ADR 0013, ADR 0025 after acceptance

## Context

v0.90 delivered a bounded repo-visibility prototype. v0.91.2 plans a practical
follow-on that makes canonical source authority and code-doc-test-demo linkage
more useful for review, planning, and productized CodeFriend packets.

Repo visibility is not full repo cognition. It is a manifest and linkage layer
that helps agents and reviewers understand which files, docs, tests, demos, and
evidence surfaces are relevant.

## Decision

ADL treats repo visibility as a bounded manifest/linkage layer for source,
documentation, tests, demos, proof packets, review surfaces, and milestone
evidence.

The layer supports reviewer and planner navigation. It does not replace
canonical source files, Git history, issue/PR truth, or human review.

## Requirements

- Repo visibility artifacts must identify canonical source paths and related
  proof surfaces.
- Linkage should connect code, docs, tests, demos, issue work, and review
  evidence where useful.
- The layer must preserve uncertainty and missing-evidence markers.
- The layer must not claim complete indexing, total repository cognition, or
  automatic architectural understanding.
- CodeFriend and milestone review can consume the layer as evidence routing,
  not as proof by itself.

## Consequences

### Positive

- Makes large-review and milestone-closeout work easier to navigate.
- Gives CodeFriend a better packet-building substrate.
- Reduces hidden dependency on session memory or local browsing.

### Negative

- Linkage artifacts can become stale if not maintained.
- Partial visibility can be mistaken for complete knowledge unless non-claims
  remain prominent.
- Repo visibility work needs validation against real file existence and review
  usefulness.

## Alternatives Considered

### Treat repo visibility as full repo cognition

This overclaims what the current substrate can do.

### Leave visibility as ad hoc search

This keeps work lightweight but loses repeatability and packet quality.

## Validation Notes

This candidate should be reviewed against the v0.90 baseline, v0.91.2 repo
visibility feature doc, CodeFriend packet needs, and any bounded linkage proof
created in the milestone.

## Non-Claims

- This ADR does not claim complete repository indexing.
- This ADR does not replace source review.
- This ADR does not make generated linkage authoritative without evidence.
