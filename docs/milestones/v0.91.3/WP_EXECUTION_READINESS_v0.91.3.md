# v0.91.3 WP Execution Readiness

## Status

Active readiness surface. The v0.91.3 issue wave is open as `#3199` through
`#3214`; child execution remains blocked until `WP-01` / `#3199` is merged and
closed out with validated issue cards.

## Readiness Requirements

Before any v0.91.3 WP starts:

- the issue has SIP, STP, SPP, SRP, and SOR cards
- `workflow-conductor` identifies the next lifecycle stage
- execution is bound with `adl/tools/pr.sh run <issue>`
- the work happens in the bound worktree
- card repairs use the matching editor skill
- pre-PR review is run and findings are fixed or routed
- closeout is performed after merge or intentional closure

## C-SDLC-Specific Readiness Gates

The first-slice work must additionally prove:

- transition identity is stable and repo-relative
- transition manifest links all five cards
- transition manifest or evidence identifies material actor roles without
  claiming the full v0.91.4 actor-standing model
- transition DAG identifies serial steps, shards, and barriers
- shard ownership prevents overlapping writes unless explicitly synchronized
- evidence bundle records commands, artifacts, review results, and residual risk
- merge-readiness gate preserves GitHub issue, PR, CI, branch, and human review
  truth
- SRP review results and SOR outcome truth have an ObsMem handoff boundary
- the tracked C-SDLC source package is used instead of local-only TBD notes as
  milestone planning evidence
- trace/proof references are repo-relative and shaped so v0.91.4 can add
  signed trace bundles without redesigning the first-slice manifest

## Lessons From v0.91.2 Process Mini-Sprints

The v0.91.2 process mini-sprints showed that expansion work can be real while
still leaving workflow risks. v0.91.3 must therefore treat these as readiness
requirements:

- combined-lane validation must be run when multiple modules share process
  state, env vars, fixtures, or global resources
- sprint umbrella closeout truth must be reviewed, not inferred from closed
  child issues
- issue SORs must be normalized after merge, not left in `worktree_only` or
  `pr_open` language
- dry-run proof is acceptable only when the docs state exactly what it proves
  and what it does not prove
