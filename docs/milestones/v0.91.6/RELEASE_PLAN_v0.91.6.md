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
- [CONTROL_PLANE_RESCUE_SPRINT_v0.91.6.md](CONTROL_PLANE_RESCUE_SPRINT_v0.91.6.md)
  for the control-plane rescue gate that must clear before release-tail work
  resumes through the standard issue wave

Use [review/V0916_RELEASE_AND_BRIDGE_DOC_TRUTH_CONSUMPTION_REVIEW_4522.md](review/V0916_RELEASE_AND_BRIDGE_DOC_TRUTH_CONSUMPTION_REVIEW_4522.md)
as the bounded audit of this rule and the remaining manual boundary, not as a
third current-state ledger.

This release plan is a checklist surface, not the canonical per-issue status
ledger.

## 0. Release-Tail Convergence

- [x] Control-plane rescue sprint `#4588` completed before ordered release-tail
  execution resumes. Any incomplete child, blocked follow-on, or retained
  watcher residue requires an explicit operator waiver in `#4588` with
  release-tail impact, retained watcher evidence, and follow-on owner.
- [ ] Bridge ledger refreshed from first-tranche outcomes, consuming the
  retained-evidence matrix for closed bridge umbrellas and the closeout-tail
  sprint surface for open ordered release-tail work.
- [ ] Every release-tail issue that entered a wait state records a retained
  watcher packet, watcher-packet summary, or explicit not-applicable reason
  before it is counted as cleanly advanced or complete.
- [ ] WP-14A internal review and pre-`v0.92` burn-down executes under `#4582`;
  closed `#3979` is consumed only as retained planning/source evidence.
- [ ] Every touched product/runtime surface is classified with the operational
  completion gate before release truth calls it complete.
- [ ] The operational completion gate is the required truth boundary for
  product/runtime completion claims in this release tail.
- [ ] Feature docs reviewed and updated for final truth.
- [ ] Open residuals routed to `v0.91.7`, `v0.92`, or later milestones.
- [ ] Tooling reliability issues have complete, blocked, deferred, or routed
  status.
- [x] Workflow-critical ADL commands use independent binaries rather than
  implicit Cargo execution during normal finish, validation, watcher, and
  closeout paths through merged issue `#4590`.
- [x] Deterministic `finish --ready` publication and Git push authentication
  repaired through merged issue `#4598`.
- [x] Shared C-SDLC operating docs and installed skill guidance reflect the
  current watcher, prep-scout, scheduler, binary-first, and closeout contract
  through merged issue `#4591`.
- [ ] Public prompt record security/export status recorded.
- [ ] Provider/model reliability status recorded.
- [ ] Security/CAV status recorded.
- [x] `agent-logic.ai` AWS account/setup planning status recorded through
  `#3902` without exposing sensitive offer identifiers.
- [ ] CodeFriend v1 / portable adapter v2 and guild route preservation status
  recorded without expanding first-tranche activation scope.
- [x] WP-12 quality gate has consumed WP-11 proof convergence. Current retained
  packet:
  [review/V0916_WP12_QUALITY_GATE_3977.md](review/V0916_WP12_QUALITY_GATE_3977.md)
  consumes merged `#3976` / PR `#4605` demo/proof truth, clears WP-13 to start
  after the WP-12 PR lands, and routes WP-11 closeout normalization as
  release-tail hygiene rather than a proof blocker.
- [ ] WP-13 docs/review-surface alignment has landed. Current owner: `#3978`.
  It must leave README, changelog, milestone planning docs, release docs,
  feature-list references, and review entrypoints aligned before `#4582`
  internal review begins.

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
