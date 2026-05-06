# Runtime And Polis Architecture

## Metadata

- Feature Name: Runtime and Polis Architecture Alignment
- Milestone Target: `v0.91.1`
- Status: planned
- Planned WP Home: WP-02
- Source Docs: `.adl/docs/TBD/runtime_v2/`
- Proof Modes: review, docs, source inventory

## Purpose

Align the tracked Runtime v2 and polis architecture story with the code and
artifact surfaces that exist by the start of v0.91.1. This feature prevents
identity, citizen state, communication, and Observatory work from building on
stale runtime assumptions.

## Scope

In scope:

- Runtime v2 architecture refresh.
- Polis, kernel, manifold, snapshot, lifecycle, and control-plane inventory.
- Drift report for older source docs that no longer match repo truth.
- Reviewable architecture package that later WPs can cite.

Out of scope:

- Full identity or birthday semantics.
- Replacing Runtime v2 implementation in a docs-only pass.
- External federation or cross-polis transport.

## Acceptance Criteria

- Architecture claims are source-grounded or clearly marked planned.
- Runtime/polis terminology is consistent across v0.91.1 docs.
- No feature WP depends on an unstated runtime assumption.
- Reviewers can trace runtime claims back to code, artifacts, or source docs.
