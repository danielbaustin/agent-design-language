# v0.91.6 Release Plan

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Release date: not scheduled
- Release manager: ADL maintainers

## How To Use

Use this as the release-tail checklist once `v0.91.6` execution begins. This
planning package does not publish a release by itself.

Current issue truth for release consumption must be read from:

- [review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md](review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md)
  for closed bridge umbrellas and retained evidence posture
- [CLOSEOUT_TAIL_SPRINT_v0.91.6.md](CLOSEOUT_TAIL_SPRINT_v0.91.6.md)
  for the ordered open release-tail issue wave
- [OPERATIONAL_COMPLETION_GATE_v0.91.6.md](OPERATIONAL_COMPLETION_GATE_v0.91.6.md)
  for the required completion-class boundary on product/runtime claims

Use [review/V0916_RELEASE_AND_BRIDGE_DOC_TRUTH_CONSUMPTION_REVIEW_4522.md](review/V0916_RELEASE_AND_BRIDGE_DOC_TRUTH_CONSUMPTION_REVIEW_4522.md)
as the bounded audit of this rule and the remaining manual boundary, not as a
third current-state ledger.

This release plan is a checklist surface, not the canonical per-issue status
ledger.

## 0. Release-Tail Convergence

- [ ] Bridge ledger refreshed from first-tranche outcomes, consuming the
  retained-evidence matrix for closed bridge umbrellas and the closeout-tail
  sprint surface for open ordered release-tail work.
- [ ] Every touched product/runtime surface is classified with the operational
  completion gate before release truth calls it complete.
- [ ] The operational completion gate is the required truth boundary for
  product/runtime completion claims in this release tail.
- [ ] Feature docs reviewed and updated for final truth.
- [ ] Open residuals routed to `v0.91.7`, `v0.92`, or later milestones.
- [ ] Tooling reliability issues have complete, blocked, deferred, or routed
  status.
- [ ] Public prompt record security/export status recorded.
- [ ] Provider/model reliability status recorded.
- [ ] Security/CAV status recorded.
- [x] `agent-logic.ai` AWS account/setup planning status recorded through
  `#3902` without exposing sensitive offer identifiers.
- [ ] CodeFriend v1 / portable adapter v2 and guild route preservation status
  recorded without expanding first-tranche activation scope.

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

- [ ] Focused docs validation recorded, including
  `python3 adl/tools/check_repo_quality_staleness.py --milestone v0.91.6`
  when reviewer-facing repo or milestone docs changed.
- [ ] CI status checked for merged PRs.
- [ ] Release links tested.
- [ ] Immediate regressions triaged and tracked.

## 5. Communication

- [ ] Roadmap/status updated.
- [ ] `v0.91.7` handoff visible.
- [x] `#3902` account/setup route visible for later infrastructure consumers.
- [ ] CodeFriend and guild routes remain visible for later MVP consumers.
- [ ] `v0.92` activation remains blocked or is explicitly opened by reviewed
  bridge truth.

## Exit Criteria

- No hidden implementation or unresolved truth-maintenance work remains in the
  ceremony phase.
- No product/runtime surface is described as complete from prerequisite-only
  proof.
- Every activation-relevant surface is complete, blocked, deferred, or routed.
