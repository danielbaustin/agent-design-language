# Active Issue Migration Policy

## Status

Tracked WP-11 feature packet proposed for `v0.91.4`.

## Purpose

Define how open and future ADL software-development issues move onto C-SDLC
without corrupting in-flight work.

The end state is simple: future ADL software-development issues use C-SDLC by
default. The migration path needs care because some active issues may already
have older cards, open PRs, or partially completed closeout records.

## Scope

The policy must classify active issues as:

- migrate now
- migrate at next lifecycle boundary
- leave unchanged and close out under the old contract
- no-op close or fold into another issue
- block until operator judgment

## Acceptance Criteria

- The policy names the decision criteria for each migration class.
- A sampled active-issue audit demonstrates the classification process.
- Future issue creation defaults to the canonical C-SDLC card sequence.
- New durable records created after migration use
  `docs/milestones/v0.91.4/review/evidence/csdlc/issues/` as the tracked issue-local namespace.
- In-flight PRs are not forced through unsafe card rewrites.
- Historical records keep their truth while new records stop reproducing old
  drift.

## Decision Classes

Active issues classify into one of five buckets:

- `migrate_now`
  - Use when the issue is still open, not already in a risky in-flight
    publication state, and can safely adopt the current C-SDLC lane without
    rewriting established truth.
- `defer`
  - Use when the issue belongs to a future milestone, a later sprint, or a
    lane whose migration timing should be decided closer to execution.
- `leave_unchanged`
  - Use when the issue already runs on the intended C-SDLC contract or when
    forced migration would rewrite truthful in-flight history.
- `fold_or_noop_close`
  - Use when the issue is already satisfied, duplicated, or superseded and the
    correct next move is routing through `issue-folding` rather than migrating
    execution.
- `block`
  - Use when operator judgment, unresolved prerequisites, or ambiguous
    ownership/scope make automatic migration unsafe.

## Future Defaults

Future ADL software-development issues should default to:

- upfront creation of `SIP`, `STP`, `SPP`, `SRP`, and `SOR` from the current
  prompt-template registry
- issue-specific `SIP`, `STP`, and `SPP` readiness before execution binding
- `workflow-conductor` as the front door for lifecycle routing
- editor-skill repair for card drift instead of ad hoc manual normalization
- preserving `SRP` as review truth and `SOR` as execution/integration truth
  rather than treating either as disposable chat residue

## Tracked Proof Surface

- `docs/milestones/v0.91.4/ACTIVE_ISSUE_MIGRATION_AUDIT_2026-05-27.md`
- `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`

## Landed v0.91.4 Position

WP-11 does not bulk-migrate every open issue. It lands a bounded policy and a
sampled audit process:

- issues already on the new lane should stay truthful instead of being
  rewritten for ceremony
- open issues that are safe to bring onto the current lane should migrate at
  their next sensible execution entrypoint
- future ADL software-development issues should start on the five-card C-SDLC
  contract by default
- fold/no-op close and block cases remain explicit routing decisions rather than
  silent cleanup

## Non-Goals

- This feature does not rewrite all historical issue records.
- This feature does not hide old drift by renaming it complete.
- This feature does not bypass editor skills for card normalization.
