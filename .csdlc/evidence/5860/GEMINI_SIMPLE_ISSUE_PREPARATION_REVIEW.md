# Gemini Review: Simple Issue Preparation And Binding

Model: `gemini-3.1-pro-preview`

Disposition: four actionable findings, all incorporated into issue #5861.

## Findings

### P1: Preparation catch-22

The first proposal required preparation to fail atomically on placeholders
without leaving partial cards, but operators need an editable generated packet
in order to replace those placeholders. A single strict operation made the
`prepared` state unreachable as a useful resting state.

Disposition: split preparation into iterative `sync` and strict `seal`.
`sync` may preserve visible placeholders; only `seal` can emit readiness.

### P1: Multi-file and Git atomicity was underspecified

Updating design plus six cards cannot be one filesystem transaction, and Git
worktree creation cannot be atomic with claim persistence. A crash could leave
mixed cards or an orphan worktree.

Disposition: use staged immutable generations plus one atomic current-manifest
replacement for preparation, and a write-ahead `binding -> bound` recovery
record for Git worktree binding.

### P1: Concurrent readiness simulation races

Two claim-free prepared issues can both appear bindable while declaring the
same product path. A readiness receipt cannot promise future resource
availability.

Disposition: define readiness as semantic proof at one revision. Recheck
overlap and every volatile predicate at bind; concurrent overlapping binds
must produce exactly one winner and no loser artifacts.

### P1: Legacy preparation claims can become permanent blockers

Existing initialized records can carry preparation claims. A naive importer
would make them unbindable under the new state model.

Disposition: migrate preparation-only initialized claims to claim-free
`prepared` with an explicit audit event. Preserve valid active execution claims
for bound/implemented records. Fail ambiguous records closed with one repair
operation.

## Required Adversarial Tests

1. Sync leaves editable placeholder cards but no readiness receipt.
2. Seal rejects placeholders without deleting the prepared generation.
3. A crash during staged generation leaves the prior current generation intact.
4. A crash after worktree creation resumes safely from `binding` intent.
5. Legacy preparation claims import to claim-free prepared state with audit.
6. Concurrent overlapping binds produce one winner and a clean loser.
7. Forging `Status: ready` without the digest-pinned receipt cannot satisfy
   doctor or bind.
