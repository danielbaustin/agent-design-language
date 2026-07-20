# Issue 5542 design

## Purpose

Repair the post-merge WP-17 documentation truth defects found after PR #5539.

## Boundaries

- Update only canonical documentation entrypoints and the WP-17 validator/audit packet.
- Represent issue #4644 as closed and PR #5539 as merged.
- Route v0.92 consumption through the reviewed v0.91.8 bridge.
- Distinguish document creation dates from current verification dates.
- Preserve WP-18, WP-19, WP-20, and WP-23 as independent release gates.
- Do not modify Runtime, provider, cloud, AWS, Unity, GPU, or v0.92 implementation surfaces.

## Coordination

WP-18 #4645 currently owns the sprint-review register. This issue must not edit
that file until its active claim is released or an explicit handoff is recorded.

## Proof

Extend the executable WP-17 validator with assertions for closed/merged truth,
the remaining gate set, v0.91.8 bridge precedence, and date semantics. Retain a
fresh machine-readable receipt and exact-revision subagent review.
