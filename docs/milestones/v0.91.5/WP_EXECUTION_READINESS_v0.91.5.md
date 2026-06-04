# v0.91.5 WP Execution Readiness

## Status

Planned readiness surface.

## Readiness Rules

- Every opened issue must have `SIP`, `STP`, `SPP`, `SRP`, and `SOR`.
- `SIP`, `STP`, and `SPP` must be issue-specific and design-time ready before
  execution.
- Cards must come from `docs/templates/prompts/current.json`.
- Card edits must use editor skills.
- Work must execute in a bound worktree, never on `main`.
- Docs-only issues should run focused docs/YAML/link/template validation, not
  broad Rust tests by reflex.
- Runtime/tooling issues should use PVF lane classification.

## v0.91.5 Specific Readiness

- Moved issues must carry `version:v0.91.5`.
- v0.91.4 docs must show bridge work moved, not abandoned.
- v0.92 docs must consume v0.91.5 closeout and `#3377`.
- Multi-agent issues must record role, shard, provider/model, and closeout
  expectations before execution.
- Sprint umbrella issues `#3571` through `#3574` must receive qualitative
  card review before they are used for sprint execution/closeout.
- Downstream issue-card rewrite/normalization is intentionally routed through
  `#3582` after the prompt-template renderer, structure schemas, and
  field-level values editor land. The `#3582` audit validates all downstream
  v0.91.5 cards phase-aware and records the disposition in
  `PROMPT_CARD_REWRITE_AUDIT_3582.md`.
- Closeout-tail issues `#3575`, `#3579`, `#3576`, `#3580`, `#3577`,
  `#3581`, and `#3578` must remain ordered and must not start before their
  dependency WPs are truthfully complete or blocked.

## Exit Criteria

- No bridge issue execution starts from generic or stale design-time cards;
  downstream card truth is validated or repaired through `#3582` before later
  Sprint 1 work consumes it.
- The issue wave can be reviewed without reconstructing sequencing from chat.
- Sprint and release-tail issue routes are visible from tracked docs and
  GitHub issue state.
