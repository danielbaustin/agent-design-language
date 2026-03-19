# Reviewer Surface

This document describes the first bounded repo-local review helper introduced for WP-06.

## Purpose

The reviewer surface is a deterministic helper, not a full replacement for richer GPT-based review.

It exists to provide:

- a concrete repo-local review/helper surface
- a fixed subset of checklist-aware review behavior
- a stable fixture and deterministic review run

## Current Entry Point

Use:

```bash
ruby swarm/tools/review_card_surface.rb \
  --input docs/tooling/examples/reviewer-regression/issue-661/input_661.md \
  --output docs/tooling/examples/reviewer-regression/issue-661/output_661.md
```

## Current Checks

The first slice checks:

- required output-card sections
- normalized output-card status
- non-blank execution metadata
- explicit repo-relative artifact paths
- absence of absolute host paths in the output card
- canonical Prompt Spec review-surface ordering when present

## Deterministic Fixture

Use:

```bash
bash swarm/tools/test_review_card_surface.sh
```

This fixture run compares the tool output against:

- `docs/tooling/examples/reviewer-surface/issue-661/expected_review_surface_output_661.yaml`

## Scope Notes

This is intentionally narrower than the full reviewer GPT instructions and checklist ecosystem.

It is meant to:

- make review behavior more operational than prose-only guidance
- create a bounded stable proof surface
- support later integration with richer editor and lifecycle tooling
