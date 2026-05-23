# CodeFriend Reference Inventory

## Status

Planning inventory for issue `#3238`.

This inventory classifies observed `CodeFriend`, `CodeBuddy`, `codefriend`, and
`codebuddy` references so later migration work can be precise instead of a
repo-wide string replacement.

## Current Product Name

- Current product name: `CodeFriend`
- Current domain name: `CodeFriend.ai`
- Legacy working name: `CodeBuddy`
- Candidate alpha milestone band: `v0.93.x`

The naming migration should support a future CodeFriend alpha milestone whose
exit bar is a fully working alpha ready for testing.

The alpha milestone is not the end state. Future CodeFriend planning must also
cover the remaining structural-intelligence, executable-governance,
architectural-memory, and product-delivery features after `v0.93.x`.

## Current Tracked CodeFriend Surfaces

These surfaces already use current product naming and should remain primary
references:

- `docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md`
- `docs/milestones/v0.91.2/review/codefriend_productization/`
- `docs/adr/0025-codefriend-review-packet-product-boundary.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md`

## Local Drafting Source

Operator-provided local input used in this planning pass:

- `.adl/docs/TBD/codebuddy_ai/CODEFRIEND_NOTES.md`

That local note is provenance only for this issue; this tracked inventory and
setup plan summarize the relevant content so future readers do not need the
ignored local note.

Ignored in this planning pass:

- `.adl/docs/TBD/codebuddy_ai/ADL_MULTIAGENT_INVESTIGATE_ENHANCE.md`

## Reference Classes

### Class A: Current Product-Facing References

These should use `CodeFriend` unless a document is explicitly historical.

Examples:

- current planning docs
- product setup docs
- public-facing README or landing-page copy
- current feature-list entries
- current review-product documentation

### Class B: Historical Milestone Evidence

These may preserve `CodeBuddy` when the text is describing the historical state
of an earlier milestone or demo.

Examples:

- `docs/milestones/v0.90/DEMO_MATRIX_v0.90.md`
- `demos/v0.90/codebuddy_multi_agent_review_showcase_demo.md`
- older release notes or milestone handoffs that recorded the working name used
  at that time

Potential action:

- add a short note that CodeBuddy was the predecessor working name for
  CodeFriend, but do not rewrite history unless the doc is actively reused for
  current product copy.

### Class C: Internal Skill And Schema Compatibility

These require a dedicated migration decision because they may affect artifact
paths, schema names, or fixture compatibility.

Examples:

- `.adl/reviews/codebuddy/<run_id>/`
- `codebuddy.repo_packet`
- `codebuddy.product_report`
- `codebuddy_review_engine`
- skill descriptions that say `CodeBuddy-style`
- generated filenames such as `codebuddy_product_report.md`

Potential action:

- keep temporarily as compatibility identifiers
- create a future versioned migration plan for artifact roots and schema names
- avoid silent renames that break existing review artifacts

### Class D: Current CodeFriend Adjacent Surfaces

These mention CodeFriend but are not the canonical product docs.

Examples:

- Google Workspace bridge docs that mention future CodeFriend projects
- Moderne/code-modernization docs that mention CodeFriend packets
- repo-visibility docs that route reviewers to CodeFriend proof surfaces

Potential action:

- leave in place when they are accurate
- link back to `docs/planning/codefriend/` when they become active planning
  inputs

## Recommended Migration Order

1. Update current product-facing docs and glossary entries.
2. Add historical-name signposts where older milestone/demo docs are still
   likely to be read.
3. Decide whether internal schema and artifact roots keep `codebuddy` as a
   compatibility name or receive a versioned `codefriend` successor.
4. Only then rename scripts, fixtures, generated filenames, or artifact paths.

## Known High-Value Cleanup Targets

Targets likely worth addressing in the first naming-migration issue:

- `docs/GLOSSARY.md`
- current docs under `docs/planning/`
- current architecture docs that describe the active review-product lane
- current demo indexes that mention the old name without historical context

Targets that should be handled more carefully:

- `adl/tools/skills/**`
- `adl/tools/skills/**/scripts/**`
- artifact path patterns under `.adl/reviews/codebuddy/`
- schema names beginning with `codebuddy.`
- historical milestone docs from v0.90 and v0.91

## Validation For Migration Issues

Any naming migration issue should run:

- `rg -n "CodeBuddy|codebuddy|CodeFriend|codefriend" ...`
- Markdown relative-link validation for changed docs
- `git diff --check`
- targeted skill/schema tests if internal names or scripts change

Do not run broad code tests for a docs-only naming pass unless scripts or schema
paths are changed.
