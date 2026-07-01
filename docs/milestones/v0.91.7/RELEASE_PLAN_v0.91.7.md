# v0.91.7 Release Plan

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Release date: not scheduled
- Release manager: ADL maintainers

## How To Use

Use this as the release-tail checklist once `v0.91.7` execution begins. This
planning package does not publish a release by itself.

## 0. Release-Tail Convergence

- [ ] Bridge ledger refreshed from second-tranche outcomes.
- [ ] Feature docs reviewed and updated for final truth.
- [ ] Open residuals resolved, explicitly non-claimed with operator approval,
  or blocked with evidence and operator approval.
- [ ] Security and ACIP/A2A residual status recorded.
- [ ] Curiosity and Constructability proof or blocker status recorded.
- [ ] Reasoning graph / `adl.skill.v1` proof or blocker status recorded.

## 1. Release Readiness

- [ ] Milestone checklist complete or exceptions documented.
- [ ] Release notes approved.
- [ ] Go/no-go decision recorded in `DECISIONS_v0.91.7.md`.

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
- [ ] `#3780` activation handoff visible.
- [ ] `v0.92` activation remains blocked or is explicitly opened by reviewed
  bridge truth.

## Exit Criteria

- No hidden implementation or unresolved truth-maintenance work remains in the
  ceremony phase.
- Every activation-relevant residual is integrated/proven, explicitly non-claimed with operator approval, or blocked with evidence and operator approval.
