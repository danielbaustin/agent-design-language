# v0.91.6 Milestone Checklist

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Setup issue: `#3800`

## Status

Forward checklist. `v0.91.6` execution is now in progress, so checked rows
represent surfaces already completed or routed while unchecked rows remain open
milestone work.

Use [`OPERATIONAL_COMPLETION_GATE_v0.91.6.md`](OPERATIONAL_COMPLETION_GATE_v0.91.6.md)
when a checklist row could otherwise mistake prerequisite proof for integrated
runtime or product completion.

## Planning

- [ ] `#3778` bridge ledger consumed.
- [ ] Candidate WBS reviewed and promoted or corrected.
- [ ] Candidate issue wave opened with C-SDLC card bundles.
- [ ] Every first-tranche surface has an owning issue.
- [ ] `#3801` second-tranche residuals remain explicit.
- [ ] `#3780` v0.92 activation refresh remains blocked until bridge truth is
  complete, deferred, blocked, or routed.

## Feature Docs

- [x] Resilience, persistence, and sleep/wake feature doc completed or routed.
- [x] Logging/tooling proof-loop reliability feature doc completed or routed.
- [x] Public prompt records export/redaction/indexing feature doc completed or
  routed.
- [x] Provider/model reliability and multi-agent readiness feature doc completed
  or routed.
- [x] ACIP/A2A/provider communications decision record completed or routed.
- [x] Security bridge and CAV feature doc completed or routed.
- [x] Identity/continuity and capability-selector bridge record completed or
  routed.
- [x] Observatory/Unity consumption classification completed or routed.
- [x] AEE completion, Memory/ObsMem handoff, and ACP/cognitive profile
  accounting completed or routed.

## Tooling Reliability

- [ ] `#3802` prompt-card parallel validation behavior fixed or routed.
- [ ] `#3803` prompt-card enum diagnostics aligned or routed.
- [ ] `#3804` lifecycle-card path-leakage diagnostic precision fixed or routed.
- [ ] `#3805` octocrab token preflight diagnostics fixed or routed.

## Companion Setup Planning

- [x] `#3902` `agent-logic.ai` AWS account setup plan completed, blocked,
  deferred, or routed.
- [x] AWS credits/application guidance is recorded without exposing sensitive
  offer identifiers.
- [x] Account-bound Terraform and hosting/security boundaries are explicit
  before later infrastructure consumers rely on the account.
- [x] Runtime integration soak plan `#4185` distinguishes Soak #1 in `v0.91.6`
  from Soak #2/#3 full feature-list readiness in `v0.91.7`.
- [x] CodeFriend v1 / portable adapter v2 route remains visible for
  post-v0.92 / pre-v0.95 proof work.
- [x] Guilds remain visible as an MVP-scoped governance route through v0.93 and
  v0.95 consumption.

## Scope Integrity

- [x] Every product/runtime surface consumed by closeout is classified with the
  operational completion gate as a manual release-evidence boundary.
- [x] No runtime feature is claimed by planning docs alone.
- [x] No runtime/product surface is treated as `done` from `docs_ready`,
  `seam_ready`, `mock_proven`, `component_proven`, `local_slice_proven`, or
  `demo_scaffold` evidence alone.
- [x] No `v0.92` activation readiness claim appears without bridge evidence.
- [x] Security remains on the activation path.
- [x] ACIP/A2A decisions include protobuf/JSON/WebSocket/access-rule posture.
- [x] Public prompt records preserve local editable authoring and public export
  boundaries.
- [x] Gemma/model reliability is addressed as part of multi-agent readiness.
- [x] AEE, Memory/ObsMem, and ACP/profile surfaces are visible before v0.92
  activation refresh.
- [x] Runtime coherence is not claimed from component completion or Soak #1
  alone; Soak #2, or Soak #3 if needed, remains a named pre-`v0.92` gate.
- [x] `#3902` is visible as v0.91.6 setup planning, not v0.92 activation proof.
- [x] CodeFriend and guild route preservation is not treated as first-tranche
  runtime or activation proof.

## Review And Closeout

- [x] WP-14A `#4582` completed the internal-review and pre-`v0.92` burn-down
  step; closed `#3979` remains retained planning/source evidence only.
- [x] Bounded internal review completed.
- [x] Findings fixed or explicitly routed.
- [x] WP-12 quality gate consumed WP-11 proof convergence. Current retained
  packet:
  [review/V0916_WP12_QUALITY_GATE_3977.md](review/V0916_WP12_QUALITY_GATE_3977.md)
  consumes merged `#3976` / PR `#4605` demo/proof truth and cleared WP-13 to
  start after the WP-12 PR landed. WP-11 closeout normalization remains routed
  as release-tail hygiene rather than a proof blocker.
- [x] WP-13 docs/review-surface alignment completed through merged `#3978` /
  PR `#4608`. Follow-on release-tail doc-truth cleanup landed through closed
  `#4609`, and the internal-review owner `#4582` has closed.
- [x] WP-15 external review failure is recorded truthfully and does not read as
  pending-send or approved. Current retained packet:
  [review/external_review/V0916_EXTERNAL_REVIEW_FINDINGS_2026-06-28.md](review/external_review/V0916_EXTERNAL_REVIEW_FINDINGS_2026-06-28.md)
  records the failed review; `#3980` is closed and WP-16 `#3981` remediated the
  accepted findings.
- [x] WP-17 `#3982` refreshed the v0.91.7 handoff and closed.
- [x] WP-18 `#3983` reviewed v0.91.7 readiness and closed.
- [x] Bridge-ledger dispositions refreshed or handed off.
- [x] Runtime/product closeout rows preserve explicit completion class, evidence,
  and blocker or defer routes when not `integrated_proven`; mechanical
  Rust/PVF enforcement of the closed-issue SOR bundle truth gate remains routed
  residual work.
- [x] `v0.91.7` planning issue `#3801` has the residuals it needs.
- [x] Closeout record states what `v0.92` may consume and what remains blocked.

## Exit Criteria

- First-tranche bridge surfaces are reviewable from tracked docs and issues.
- `v0.91.7` has no vague spillover.
- `v0.92` activation can tell which surfaces are complete, deferred, blocked,
  or routed.
