# v0.91.7 Milestone Checklist

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Created: `2026-06-21`
- Last verified: `2026-07-20`
- Setup lineage: `#3801`, `#3825`, `#4368`
- Source capture: `PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- Release-tail handoff addendum: `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`

## Status

Live checklist refreshed on 2026-07-20. Checked items below have direct
issue/packet evidence; unchecked items remain unverified or belong to open
quality/review/release gates. This checklist does not infer completion merely
from a closed issue.

## Planning

- [ ] `#3778` pre-v0.92 source ledger consumed.
- [ ] v0.91.5 pre-v0.92 source-ledger row consumed.
- [ ] `v0.91.6` first-tranche package consumed.
  - WP-02 closeout-truth child packet: `docs/milestones/v0.91.7/review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md`.
- [ ] `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md` consumed with failed-but-closed WP-15 `#3980` review truth, closed WP-16 `#3981` remediation truth, closed `#4620` / `#4621`, closed v0.91.7 `#4622` PR-inventory proof, and closed WP-14A remediation truth.
- [ ] Closed ADR release-tail issues `#4324` and `#4369`-`#4376` consumed, and open tooling remediation `#4378` closed with evidence or explicitly blocked with evidence and operator approval.
- [ ] `PLANNING_SOURCE_CAPTURE_v0.91.7.md` reviewed.
- [ ] Candidate WBS reviewed and promoted or corrected.
- [x] Candidate issue wave opened with C-SDLC card bundles (`#4628`).
- [ ] Every source-capture row is integrated/proven, already closed with evidence, operator-scoped-out with evidence and approval, or blocked with evidence and operator approval.
- [ ] `#3780` activation refresh remains blocked until required surfaces are integrated/proven, operator-scoped-out with evidence, or blocked with evidence and operator approval.

## Process And Validation Substrate

- [ ] v0.91.6 `#4388`-`#4398` C-SDLC integration control-plane sprint is consumed as integrated/proven, already closed with evidence, or explicitly blocked with evidence and operator approval, including SEP, VPP, PVF/template-version work, GitHub/octocrab convergence, goal metrics, logging, watcher/lifecycle automation, runtime dependency disposition, tooling reliability, and FastContext evaluation.
- [ ] Late control-plane inputs `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, `#4433`-`#4438`, and `#4442`-`#4443` are integrated/proven, already closed with evidence, operator-scoped-out with evidence and approval, or blocked with evidence and operator approval before v0.91.7 execution relies on them.
- [ ] Sprint `/goal`, issue goal, session ledger, watcher, activity log, shepherd, and closeout rules are implemented in templates/skills/tools or blocked with evidence and operator approval.
- [ ] PVF/VPP lane assignment and generated VPP creation can be made during planning or are explicitly blocked with evidence and operator approval.
- [ ] Forward time/token/resource accounting, v0.91.6-only backfill, and host goal snapshots are implemented for cards/SORs or blocked with evidence and operator approval.
- [ ] Closed dependency `#4331` is consumed by goal-state and nested-goal planning.
- [ ] Validation-manager/test-tax/build-throughput work is implemented/proven where v0.92 depends on it or blocked with evidence and operator approval.

## Runtime, Scheduler, Provider, And Build Readiness

- [ ] Goal-state and nested-goal continuity inputs are implemented/proven or blocked with evidence and operator approval.
- [ ] Cognitive scheduler and economics inputs are implemented/proven or blocked with evidence and operator approval.
- [ ] Provider/local-agent suitability and model-routing proof is current.
- [x] Runtime fire-up / Soak #2 execution and review evidence is retained by closed `#4682`.
  - Tracked execution packet: `RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md`
- [x] Integrated logging/OTel proof `#4718` is closed with bounded retained evidence; broader production telemetry and Unity readiness are not inferred.
  - Required proof: current integrated runtime/provider/control-plane events, stdout/stderr separation, redaction/path hygiene, OTel-compatible boundary truth, and Observatory/Unity event consumption.
- [ ] Runtime AWS/heartbeat/ACIP-SNS/SSM/S3 archive proof is concrete or blocked with evidence and operator approval.
- [ ] Nessus/CodeBuild/EC2 Spot or alternate remote validation and local build-throughput decisions are proven or explicitly blocked with evidence and operator approval.
- [ ] Remote-builder proof records time, cost, region/instance class, cache posture, interruption behavior, and cleanup/termination evidence before it becomes a release-critical validation lane.

## Feature And Bridge Docs

- [ ] Curiosity Engine / Discovery Substrate implemented/proven or blocked with evidence and operator approval.
- [ ] Constructability Gate implemented/proven or blocked with evidence and operator approval.
- [ ] Reasoning graph / loop runtime / `adl.skill.v1` implemented/proven or blocked with evidence and operator approval.
- [ ] Residual security readiness resolved or blocked with evidence and operator approval.
- [ ] Residual ACIP/A2A/protobuf decision record resolved or blocked with evidence and operator approval.
- [ ] Affect/happiness model completed as proof-bound operational reasoning-control implementation through `#4752`, with subjective affect/happiness/wellbeing claims explicitly not claimed.
- [ ] Godel mechanics boundary implemented/proven for v0.92 claim consumption; live hosted invocation and adaptive DAG completion remain non-claimed.
- [ ] Economics-context decision completed or blocked with evidence and operator approval.
- [ ] Guild foundation boundary implemented/proven through `#4755`, with v0.93 constitutional governance, polis authority, delegated authority, and public product readiness explicitly not claimed.
- [ ] Paper/publication boundary recorded through `#4757`, with public launch approval, papers, customer-facing CodeFriend/report readiness, autonomous review authority, and unbounded WP-13 claims explicitly not claimed unless a later tracked issue promotes a bounded artifact with evidence, redaction/public-claim review, and human approval.
- [x] WP-13 parent closeout reconciled through `#4640` and `docs/milestones/v0.91.7/review/wp13_closeout_4640.md`.
- [ ] Memory Palace/context problem status is visible for v0.92 as proof or evidence-backed blocker.

## Demo And Launch Readiness

- [ ] Observatory/Unity/HTML demo proof status is current.
- [ ] Demo matrix distinguishes proof, demo, and evidence-backed blocker surfaces.
- [ ] July launch plan has v0.91.7/v0.92 proof or evidence-backed blocker status.
- [ ] First birthday evidence boundaries are explicit.
- [ ] Publication-facing narratives cite retained proof packets and preserve the #4757 non-claim boundary.

## Scope Integrity

- [ ] No runtime feature is claimed by planning docs alone.
- [ ] No logging, observability, OTel, runtime, or Observatory readiness claim is made without current integrated evidence.
- [ ] No `v0.92` activation readiness claim appears without evidence.
- [ ] Security and ACIP/A2A requirements remain on the activation path unless operator-scoped-out with evidence, risk, and operator approval.
- [ ] Affect/happiness/Godel claims preserve proof boundaries and avoid unsupported public claims.
- [ ] Launch/product/guild work does not silently expand v0.92 birthday scope.

## Review And Closeout

- [x] Demo convergence completed with explicit public-claim boundaries through `#4642` and `review/V0917_WP15_DEMO_CONVERGENCE_4642.md`.
- [x] Quality gate completed through WP-16 #4643 with downstream boundaries retained.
- [x] Documentation alignment completed through WP-17 #4644 / merged PR #5539.
- [x] Bounded internal review completed through WP-18 #4645 and retained remediation evidence.
- [x] External review completed through WP-19 #4646 with provider limitations retained explicitly.
- [x] All 22 WP-19 findings fixed through WP-20 #4647 / merged PR #5588.
- [x] Remediation/preflight packet completed under `review/wp20_remediation_4647/`.
- [x] Next-milestone planning retained through closed `#4648`; current v0.91.8 planning supersedes direct activation use.
- [x] Next-milestone planning review retained through closed `#4649`; it is not release approval.
- [x] Release ceremony packet completed under `review/wp23_release_ceremony_4650/` for #4650 integration.
- [x] Bridge-ledger dispositions handed to the reviewed v0.91.8 planning package through WP-21A #5489.
- [ ] `#3780` has the tracked activation inputs it needs.
- [x] WP-23 closeout record states what `v0.92` may consume and what remains blocked.

Unchecked historical planning and feature rows above remain bounded proof or
handoff questions; this ceremony does not silently convert them into product
completion. The WP-23 packet records what v0.91.7 closes and what the required
v0.91.8 bridge must still review before v0.92. The unchecked #3780 activation
input is a successor-bridge obligation, not permission to claim v0.92 ready.

## Exit Criteria

- Final pre-`v0.92` implementation/proof surfaces are reviewable from tracked docs and issues.
- `#3780` can begin v0.92 activation refresh from tracked implementation/proof truth.
- `v0.92` activation can tell which surfaces are integrated/proven, already closed with evidence, operator-scoped-out with evidence and approval, or blocked with evidence and operator approval.
