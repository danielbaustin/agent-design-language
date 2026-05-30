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

## Exit Criteria

- No bridge issue starts from generic or stale design-time cards.
- The issue wave can be reviewed without reconstructing sequencing from chat.

