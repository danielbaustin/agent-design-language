# ADL Review Surface Format

Status: Draft (v0.87)
Applies to: human-authored and skill-generated review artifacts
Primary milestone feature: `docs/milestones/v0.87/features/REVIEW_SURFACE_FORMALIZATION.md`

## Purpose

Define the canonical review-surface contract for ADL review artifacts.

This document turns the feature-level review-surface design into an operational format that:
- humans can read quickly
- skills can emit consistently
- validators can check deterministically

This is the shared contract for review artifacts such as:
- repository code reviews
- card reviews
- trace or demo reviews
- internal and external readiness reviews

## Canonical Major Sections

Every review artifact in the `adl.review_surface.v1` family must contain these sections in this order:

1. `Metadata`
2. `Scope`
3. `Findings`
4. `System-Level Assessment`
5. `Recommended Action Plan`
6. `Follow-ups / Deferred Work`
7. `Final Assessment`

## Metadata

The `Metadata` section must state:
- review type
- review subject
- reviewer identity
- review date
- input surfaces
- output location if persisted

## Scope

The `Scope` section must state:
- what was reviewed
- what was not reviewed
- whether the review is code-focused, docs-focused, trace-focused, or mixed
- whether the review is pre-merge, post-merge, or release-tail

## Findings

The `Findings` section is mandatory.

If findings exist, each finding must include:
- priority (`P1` through `P4`; `P5` allowed only when justified)
- title
- location
- impact
- trigger
- evidence
- fix direction

If no material findings exist, the section must say so explicitly.

## System-Level Assessment

This section must summarize:
- the dominant risk themes
- what the finding pattern says about subsystem maturity
- whether the reviewed surface is trustworthy enough for the next gate

## Recommended Action Plan

This section must separate action into bands:
- fix now
- fix before milestone closeout
- defer

## Follow-ups / Deferred Work

This section records:
- explicit deferrals
- ownership or follow-up references
- known non-blocking debt

## Final Assessment

This section must answer:
- Is the reviewed surface trustworthy?
- Is it ready for the next gate?
- What remains before approval?

## Markdown Profile

The default human-readable profile is markdown with the exact section headings above.

For findings-first reviews such as repository code review, use this shape:

```md
## Metadata
- Review Type: repo_review
- Subject: <repo root or target>
- Reviewer: <human or skill id>
- Date: <UTC timestamp or calendar date>
- Input Surfaces:
  - <path or command>
- Output Location: <path or none>

## Scope
- Reviewed: <paths or subsystem summary>
- Not Reviewed: <explicit exclusions>
- Review Mode: code | docs | trace | mixed
- Gate: pre-merge | post-merge | release-tail

## Findings
1. [P1] Short title
Location: <path:line or artifact>
Impact: <behavioral consequence>
Trigger: <how it appears>
Evidence: <concrete evidence>
Fix Direction: <bounded repair direction>

## System-Level Assessment
<short synthesis paragraph>

## Recommended Action Plan
- Fix now: <items>
- Fix before milestone closeout: <items>
- Defer: <items>

## Follow-ups / Deferred Work
- <follow-up or explicit none>

## Final Assessment
<bounded conclusion>
```

## Determinism Rules

- section order is fixed
- section headings are fixed
- findings must be ordered by severity, then stable local ordering
- if a review contains no material findings, that absence must be explicit
- no absolute host paths
- no secrets, raw prompts, or raw tool arguments

## First v0.87 Implementation Surface

The first bounded implementation surface for this contract is:
- the `repo-code-review` skill output contract
- plus the repo-local validator and fixture test for repo review markdown artifacts

Later review surfaces may serialize the same contract differently, but must preserve the same major structure and finding semantics.
