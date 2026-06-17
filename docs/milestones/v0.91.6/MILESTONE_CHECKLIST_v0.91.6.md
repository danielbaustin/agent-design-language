# v0.91.6 Milestone Checklist

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Setup issue: `#3800`

## Status

Forward checklist. Items are intentionally unchecked because `v0.91.6`
execution has not started.

## Planning

- [ ] `#3778` bridge ledger consumed.
- [ ] Candidate WBS reviewed and promoted or corrected.
- [ ] Candidate issue wave opened with C-SDLC card bundles.
- [ ] Every first-tranche surface has an owning issue.
- [ ] `#3801` second-tranche residuals remain explicit.
- [ ] `#3780` v0.92 activation refresh remains blocked until bridge truth is
  complete, deferred, blocked, or routed.

## Feature Docs

- [ ] Resilience, persistence, and sleep/wake feature doc completed or routed.
- [ ] Logging/tooling proof-loop reliability feature doc completed or routed.
- [ ] Public prompt records export/redaction/indexing feature doc completed or
  routed.
- [ ] Provider/model reliability and multi-agent readiness feature doc completed
  or routed.
- [ ] ACIP/A2A/provider communications decision record completed or routed.
- [ ] Security bridge and CAV feature doc completed or routed.
- [ ] Identity/continuity and capability-selector bridge record completed or
  routed.
- [ ] Observatory/Unity consumption classification completed or routed.
- [ ] AEE completion, Memory/ObsMem handoff, and ACP/cognitive profile
  accounting completed or routed.

## Tooling Reliability

- [ ] `#3802` prompt-card parallel validation behavior fixed or routed.
- [ ] `#3803` prompt-card enum diagnostics aligned or routed.
- [ ] `#3804` lifecycle-card path-leakage diagnostic precision fixed or routed.
- [ ] `#3805` octocrab token preflight diagnostics fixed or routed.

## Companion Setup Planning

- [ ] `#3902` `agent-logic.ai` AWS account setup plan completed, blocked,
  deferred, or routed.
- [ ] AWS credits/application guidance is recorded without exposing sensitive
  offer identifiers.
- [ ] Account-bound Terraform and hosting/security boundaries are explicit
  before later infrastructure consumers rely on the account.
- [ ] CodeFriend v1 / portable adapter v2 route remains visible for
  post-v0.92 / pre-v0.95 proof work.
- [ ] Guilds remain visible as an MVP-scoped governance route through v0.93 and
  v0.95 consumption.

## Scope Integrity

- [ ] No runtime feature is claimed by planning docs alone.
- [ ] No `v0.92` activation readiness claim appears without bridge evidence.
- [ ] Security remains on the activation path.
- [ ] ACIP/A2A decisions include protobuf/JSON/WebSocket/access-rule posture.
- [ ] Public prompt records preserve local editable authoring and public export
  boundaries.
- [ ] Gemma/model reliability is addressed as part of multi-agent readiness.
- [ ] AEE, Memory/ObsMem, and ACP/profile surfaces are visible before v0.92
  activation refresh.
- [ ] `#3902` is visible as v0.91.6 setup planning, not v0.92 activation proof.
- [ ] CodeFriend and guild route preservation is not treated as first-tranche
  runtime or activation proof.

## Review And Closeout

- [ ] Bounded internal review completed.
- [ ] Findings fixed or explicitly routed.
- [ ] Bridge-ledger dispositions refreshed or handed off.
- [ ] `v0.91.7` planning issue `#3801` has the residuals it needs.
- [ ] Closeout record states what `v0.92` may consume and what remains blocked.

## Exit Criteria

- First-tranche bridge surfaces are reviewable from tracked docs and issues.
- `v0.91.7` has no vague spillover.
- `v0.92` activation can tell which surfaces are complete, deferred, blocked,
  or routed.
