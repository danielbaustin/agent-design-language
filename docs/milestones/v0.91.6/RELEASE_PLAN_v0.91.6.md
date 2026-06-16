# v0.91.6 Release Plan

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Release date: not scheduled
- Release manager: ADL maintainers

## How To Use

Use this as the release-tail checklist once `v0.91.6` execution begins. This
planning package does not publish a release by itself.

## 0. Release-Tail Convergence

- [ ] Bridge ledger refreshed from first-tranche outcomes.
- [ ] Feature docs reviewed and updated for final truth.
- [ ] Open residuals routed to `v0.91.7`, `v0.92`, or later milestones.
- [ ] Tooling reliability issues have complete, blocked, deferred, or routed
  status.
- [ ] Public prompt record security/export status recorded.
- [ ] Provider/model reliability status recorded.
- [ ] Security/CAV status recorded.

## 1. Release Readiness

- [ ] Milestone checklist complete or exceptions documented.
- [ ] Release notes approved.
- [ ] Go/no-go decision recorded in `DECISIONS_v0.91.6.md`.

## 2. Branch And Tag Preparation

- [ ] Target branch confirmed.
- [ ] Working tree clean.
- [ ] Version strings validated if code changes occur.
- [ ] Tag created only after implementation scope, if any, lands.

## 3. GitHub Release Steps

- [ ] GitHub Release draft created if this milestone ships as a public release.
- [ ] Release body populated from approved notes.
- [ ] Links to key PRs/issues included.
- [ ] Release visibility confirmed.

## 4. Verification

- [ ] Focused docs validation recorded.
- [ ] CI status checked for merged PRs.
- [ ] Release links tested.
- [ ] Immediate regressions triaged and tracked.

## 5. Communication

- [ ] Roadmap/status updated.
- [ ] `v0.91.7` handoff visible.
- [ ] `v0.92` activation remains blocked or is explicitly opened by reviewed
  bridge truth.

## Exit Criteria

- No hidden implementation or unresolved truth-maintenance work remains in the
  ceremony phase.
- Every activation-relevant surface is complete, blocked, deferred, or routed.
