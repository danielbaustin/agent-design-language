# v0.91.7 Release Plan

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Target release closeout date: `2026-07-20`
- Release manager: ADL maintainers

## How To Use

Use this as the active release-tail checklist. WP-01 through WP-22, including
WP-21A, are closed. WP-20 fixed all 22 WP-19 findings through merged PR #5588.
WP-23 #4650 is the sole open v0.91.7 issue before ceremony integration.
This document
does not publish a release by itself, and its output must pass through the
reviewed [v0.91.8 bridge](../v0.91.8/README.md) before v0.92 consumption.

## 0. Release-Tail Convergence

- [ ] Bridge ledger refreshed from second-tranche outcomes.
- [ ] Feature docs reviewed and updated for final truth.
- [ ] Open requirements resolved, operator-scoped-out with evidence and approval,
  or blocked with evidence and operator approval.
- [ ] Security and ACIP/A2A implementation/blocker status recorded.
- [ ] Curiosity and Constructability proof or blocker status recorded.
- [ ] Reasoning graph / `adl.skill.v1` proof or blocker status recorded.
- [ ] Paper/publication boundary recorded through #4757 before release notes,
  launch copy, reports, or website copy imply external publication readiness.

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
- [ ] Any publication-facing copy cites retained proof packets and passes the
  #4757 redaction/public-claim/human-approval promotion gates before external
  publication.

## 4. Verification

- [ ] Focused docs validation recorded.
- [ ] CI status checked for merged PRs.
- [ ] Release links tested.
- [ ] Immediate regressions triaged and tracked.

## 5. Communication

- [ ] Roadmap/status updated.
- [ ] `#3780` activation handoff visible.
- [ ] `v0.92` activation remains blocked or is explicitly opened by reviewed
  implementation/proof truth.

## Exit Criteria

- No hidden implementation or unresolved truth-maintenance work remains in the
  ceremony phase.
- Every activation-relevant surface is integrated/proven, operator-scoped-out with evidence, or blocked with evidence and operator approval.

Unchecked items above remain bounded release/publication or successor-bridge
work. WP-23 closes the v0.91.7 milestone evidence boundary; it does not create
a tag, publish a hosted release, deploy code, or activate v0.92.
