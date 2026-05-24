# v0.91.3 WP Execution Readiness

## Status

Active readiness surface. The v0.91.3 issue wave is open. The initial wave
opened `#3199` through `#3214`; correction `#3225` added the missing closeout
tail as `#3226` through `#3231`. Sprint 1 starts at `WP-02` / `#3200` because
`WP-01` / `#3199` is already merged and recorded as closed out in Sprint 1
state. Sprint 4 has completed `WP-10` proof coverage, `WP-11` quality gate,
this `WP-12` docs review pass, and the second internal-review remediation wave.
`WP-13` / `#3208` opened the internal-review cycle, and `#3321` closed the
second internal review after its remediation issues landed. The next release
tail gate is third-party review handoff.

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
