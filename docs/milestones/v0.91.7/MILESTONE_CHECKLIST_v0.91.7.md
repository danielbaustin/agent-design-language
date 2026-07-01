# v0.91.7 Milestone Checklist

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Setup lineage: `#3801`, `#3825`, `#4368`
- Source capture: `PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- Release-tail handoff addendum: `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`

## Status

Forward checklist. Items are intentionally unchecked because `v0.91.7` execution has not started.

## Planning

- [ ] `#3778` bridge ledger consumed.
- [ ] v0.91.5 pre-v0.92 bridge ledger source row consumed.
- [ ] `v0.91.6` first-tranche package consumed.
- [ ] `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md` consumed with failed-but-closed WP-15 `#3980` review truth, closed WP-16 `#3981` remediation truth, closed `#4620` / `#4621`, closed v0.91.7 `#4622` PR-inventory proof, and closed WP-14A remediation truth.
- [ ] Closed ADR release-tail issues `#4324` and `#4369`-`#4376` consumed, and open tooling remediation `#4378` closed with evidence or explicitly blocked with evidence and operator approval.
- [ ] `PLANNING_SOURCE_CAPTURE_v0.91.7.md` reviewed.
- [ ] Candidate WBS reviewed and promoted or corrected.
- [ ] Candidate issue wave opened with C-SDLC card bundles.
- [ ] Every source-capture row is integrated/proven, already closed with evidence, explicitly non-claimed with operator approval, or blocked with evidence and operator approval.
- [ ] `#3780` activation refresh remains blocked until bridge truth is integrated/proven, explicitly non-claimed with operator approval, or blocked with evidence and operator approval.

## Process And Validation Substrate

- [ ] v0.91.6 `#4388`-`#4398` C-SDLC integration control-plane sprint is consumed as integrated/proven, already closed with evidence, or explicitly blocked with evidence and operator approval, including SEP, VPP, PVF/template-version work, GitHub/octocrab convergence, goal metrics, logging, watcher/lifecycle automation, runtime dependency routing, tooling reliability, and FastContext evaluation.
- [ ] Late control-plane inputs `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, `#4433`-`#4438`, and `#4442`-`#4443` are integrated/proven, already closed with evidence, explicitly non-claimed with operator approval, or blocked with evidence and operator approval before v0.91.7 execution relies on them.
- [ ] Sprint `/goal`, issue goal, session ledger, watcher, activity log, shepherd, and closeout rules are implemented in templates/skills/tools or blocked with evidence and operator approval.
- [ ] PVF/VPP lane assignment and generated VPP creation can be made during planning or are explicitly blocked with evidence and operator approval.
- [ ] Forward time/token/resource accounting, v0.91.6-only backfill, and host goal snapshots are implemented for cards/SORs or blocked with evidence and operator approval.
- [ ] Closed dependency `#4331` is consumed by goal-state and nested-goal planning.
- [ ] Validation-manager/test-tax/build-throughput work is implemented/proven where v0.92 depends on it or blocked with evidence and operator approval.

## Runtime, Scheduler, Provider, And Build Readiness

- [ ] Goal-state and nested-goal continuity inputs are implemented/proven or blocked with evidence and operator approval.
- [ ] Cognitive scheduler and economics inputs are implemented/proven or blocked with evidence and operator approval.
- [ ] Provider/local-agent suitability and model-routing proof is current.
- [x] Runtime fire-up / Soak #2 route is concrete.
  - Tracked execution packet: `RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md`
- [ ] Runtime AWS/heartbeat/ACIP-SNS/SSM/S3 archive proof is concrete or blocked with evidence and operator approval.
- [ ] Nessus/CodeBuild/EC2 Spot or alternate remote validation and local build-throughput decisions are proven or explicitly blocked with evidence and operator approval.
- [ ] Remote-builder proof records time, cost, region/instance class, cache posture, interruption behavior, and cleanup/termination evidence before it becomes a release-critical validation lane.

## Feature And Bridge Docs

- [ ] Curiosity Engine / Discovery Substrate implemented/proven or blocked with evidence and operator approval.
- [ ] Constructability Gate implemented/proven or blocked with evidence and operator approval.
- [ ] Reasoning graph / loop runtime / `adl.skill.v1` bridge implemented/proven or blocked with evidence and operator approval.
- [ ] Residual security readiness resolved or blocked with evidence and operator approval.
- [ ] Residual ACIP/A2A/protobuf decision record resolved or blocked with evidence and operator approval.
- [ ] Affect/happiness bridge completed as safe non-claim/proof boundary or blocked with evidence and operator approval.
- [ ] Godel mechanics bridge completed as safe non-claim/proof boundary or blocked with evidence and operator approval.
- [ ] Economics-context decision completed or blocked with evidence and operator approval.
- [ ] Guilds/civilization-model boundary explicitly de-scoped or implemented with evidence, without becoming an accidental v0.92 blocker.
- [ ] Memory Palace/context problem status is visible for v0.92 as proof, non-claim, or evidence-backed blocker.

## Demo And Launch Readiness

- [ ] Observatory/Unity/HTML demo proof status is current.
- [ ] Demo matrix distinguishes proof, demo, and non-claim surfaces.
- [ ] July launch plan has v0.91.7/v0.92 proof, non-claim, or evidence-backed blocker status.
- [ ] First birthday evidence boundaries are explicit.

## Scope Integrity

- [ ] No runtime feature is claimed by planning docs alone.
- [ ] No `v0.92` activation readiness claim appears without evidence.
- [ ] Security and ACIP/A2A residuals remain on the activation path unless explicitly non-claimed with evidence, risk, and operator approval.
- [ ] Affect/happiness/Godel claims preserve safe-test and non-claim boundaries.
- [ ] Launch/product/guild work does not silently expand v0.92 birthday scope.

## Review And Closeout

- [ ] Demo convergence completed or explicitly blocked/non-claimed with evidence and operator approval.
- [ ] Quality gate completed with blockers recorded.
- [ ] Documentation alignment completed before formal review.
- [ ] Bounded internal review completed.
- [ ] External review completed or explicitly blocked with evidence according to release policy and operator approval.
- [ ] Internal/external findings fixed or recorded as evidence-backed residual risks with operator approval.
- [ ] Remediation/preflight packet completed.
- [ ] v0.92 next-milestone planning completed.
- [ ] v0.92 next-milestone planning reviewed.
- [ ] Release ceremony packet completed.
- [ ] Bridge-ledger dispositions refreshed or handed off.
- [ ] `#3780` has the tracked activation inputs it needs.
- [ ] Closeout record states what `v0.92` may consume and what remains blocked.

## Exit Criteria

- Final pre-`v0.92` bridge surfaces are reviewable from tracked docs and issues.
- `#3780` can begin v0.92 activation refresh from tracked bridge truth.
- `v0.92` activation can tell which surfaces are integrated/proven, explicitly non-claimed with operator approval, or blocked with evidence and operator approval.
