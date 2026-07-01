# v0.91.7 Work Breakdown Structure

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Status: WP issue wave opened for final pre-`v0.92` bridge and readiness tranche
- Setup lineage: `#3801`, `#3825`, `#4368`
- Source capture: `PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- Release-tail handoff addendum: `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`

## Status

WP allocation is promoted into the v0.91.7 issue wave. WP-01 is `#4628`; WP-02 through WP-23 are `#4629` through `#4650`. Existing v0.91.7 issues are assigned rather than duplicated: `#4603` belongs to WP-06, `#4617` belongs to WP-04, and `#4622` belongs to WP-02. `#4622` is closed and delivered the repo-native PR inventory command required for release-tail review.

WP-01 consumes this document, `PLANNING_SOURCE_CAPTURE_v0.91.7.md`, and [WP_ISSUE_WAVE_v0.91.7.yaml](WP_ISSUE_WAVE_v0.91.7.yaml), then keeps the opened issue wave and planning truth aligned before dependent execution begins.

## WBS Summary

`v0.91.7` should make the path to `v0.92` explicit. It combines residual bridge docs with the operational substrate needed for first-birthday execution: sprint execution, validation planning, goal/metrics accounting, scheduler/provider/local-agent routing, build throughput, runtime integration/soak, runtime architecture diet, security/protocol residuals, demos, and launch handoff.

Completion standard: planned, documented, mocked, component-proven, assigned, or routed work does not count as done for a product, runtime, or release-gating surface. Any activation-path surface must exit v0.91.7 as `integrated_proven` or `blocked_with_evidence`. Blocked exits require owner, evidence, residual risk, and explicit operator approval. Assignment to another issue or later milestone is scheduling truth only, not completion truth.

## WP Sequence

| WP | Work Package | Description | Primary deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Planning promotion and issue-wave readiness | Promote the refreshed planning package, consume `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`, classify every source in the source-capture ledger, sync the feature-list/roadmap truth, and open the issue wave. | Opened issue wave, C-SDLC card bundles, and source/feature-list disposition ledger. | Pre-v0.92 bridge ledger, v0.91.6 closeout truth, source-capture ledger, and feature-list/roadmap truth. |
| WP-02 | Closeout truth, ADR route, and carryover cleanup | Consume release-tail, ADR, Observatory, C-SDLC, and late-control-plane carryovers from the source-capture ledger; classify each as integrated/proven, already closed with evidence, or explicitly blocked with evidence and operator approval. | Closeout truth ledger, ADR disposition, tooling-remediation disposition, C-SDLC control-plane disposition, late-input disposition, and carryover proof/blocker updates. | WP-01. |
| WP-03 | Consume C-SDLC integration control-plane truth | Consume the v0.91.6 C-SDLC/control-plane completion stream named in the source-capture ledger. Do not recreate completed v0.91.6 work as fresh v0.91.7 implementation scope unless WP-02 records a blocker that must be implemented before v0.92. | Process/tooling truth-consumption gate with explicit implementation owner or evidence-backed blocker only where v0.91.6 did not complete. | WP-01, WP-02, source-capture ledger. |
| WP-04 | Goal state, nested goals, and execution metrics | Implement first-class goal-state consumption, issue/sprint goal nesting, SOR time/token/resource accounting, forward capture, bounded archaeology/backfill, host goal snapshots, and outlier analysis, or record an evidence-backed blocker with operator approval. | Goal/metrics implementation proof and template-field updates. | WP-03 and source-capture ledger. |
| WP-05 | Cognitive scheduler and provider/local-agent routing | Implement scheduler v1, provider profiles, model suitability, local/hosted agent routing, capability-envelope inputs, cheapest-validated-outcome policy, and local-agent delegation readiness, or block v0.92 with evidence and operator approval. | Scheduler/provider execution proof, role suitability matrix, and capability-envelope proof. | WP-03, WP-04, provider sprint outputs. |
| WP-06 | Build throughput and validation-cost reduction | Implement validation manager, long-test fanout, CI log archive/S3, Nessus, CodeBuild runner evaluation, EC2 Spot or alternate remote-builder proof, `sccache`/linker/target-dir cleanup, and validation DAG/build graph convergence where required before v0.92; otherwise record explicit blockers with evidence and operator approval. | Build/validation throughput sprint, remote-builder proof, and cost/time evidence. | WP-03, validation sprint outputs. |
| WP-07 | Runtime integration, fire-up, and Soak #2 | Assemble the runtime substrate into one minimal end-to-end path, reconcile Runtime v2 minimal prototype with current Tokio/runtime substrate, run integrated runtime Soak #2, identify bloat/seam pain, and preserve Soak #3 only as an operator-approved risk if Soak #2 cannot prove the activation path. | Runtime integration proof or evidence-backed blocker list, runtime module map, and architecture-diet follow-on. | WP-02, WP-05, WP-06. |
| WP-08 | Runtime AWS and signal bridge operations | Integrate heartbeat publisher, ACIP-to-SNS, AWS signal bridge, local polis SSM, and future S3/ObsMem/community-memory archive policy enough to produce runtime AWS/local operations evidence, or block v0.92 with evidence and operator approval. | Runtime AWS/local operations proof and proof expectations. | WP-07, security inputs. |
| WP-09 | Observatory, demos, and birthday-visible proof | Finish Unity/HTML Observatory, demo matrix convergence, and first-birthday-visible proof surfaces with retained evidence; unsupported runtime claims become explicit non-claims or blockers, not scheduled completion. | Demo/Observatory readiness packet. | WP-07, WP-08. |
| WP-10 | Curiosity and Constructability bridge | Implement or explicitly block governed discovery-cycle proof expectations and shared-reality/anchor/validator boundaries required before v0.92. | Curiosity and Constructability issue-ready docs, proof records, or evidence-backed blockers. | WP-01, WP-07. |
| WP-11 | Reasoning graph, loops, and `adl.skill.v1` bridge | Implement the bridge among prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1` enough for v0.92 activation, or record evidence-backed blockers. | Reasoning graph / skill-standard bridge proof. | WP-03, WP-07, WP-10. |
| WP-12 | Security, CAV, SSM, and ACIP/A2A/protobuf residuals | Resolve security/CAV residuals, SSM readiness, ACIP/A2A/protobuf/JSON/WebSocket/access-rule choices, and activation-path blockers; anything unresolved must block v0.92 unless the operator explicitly approves a non-claim with evidence. | Security/protocol residual decision packet. | WP-08, WP-10. |
| WP-13 | Affect, happiness, Godel mechanics, economics, guild, CodeFriend, and publication boundaries | Define safe-test/non-claim language and governance/civilization/economics boundaries; implement MVP-scope guild and CodeFriend/adapter v2 obligations required before v0.92; explicitly de-scope paper/publication surfaces with operator approval where they are not birthday blockers. | Boundary decision packet, MVP-scope proof/blocker rows, and handoff rows. | WP-05, WP-10, WP-11. |
| WP-14 | Launch and v0.92 birthday handoff | Align July launch planning, v0.92 activation refresh, Memory Palace/context, capability envelope, birth witnesses/receipt, first birthday docs, and external-facing non-claims. | `V092_HANDOFF_v0.91.7.md` refresh and launch/birthday readiness map. | WP-02 through WP-13. |
| WP-15 | Demo convergence | Confirm Observatory/demo matrix truth, visible proof status, demo non-claims, and any evidence-backed demo blockers before review. | Demo convergence packet and demo matrix update. | WP-09, WP-14. |
| WP-16 | Quality gate | Run focused repo-quality, checklist, stale-doc, validation-plan, and release-readiness checks appropriate for a planning/bridge milestone. | Quality-gate packet and blocker list. | WP-14, WP-15. |
| WP-17 | Documentation alignment | Align README, feature docs, WBS, sprint plan, checklist, handoff, issue wave, and feature-list/roadmap truth before formal review. | Docs alignment packet and repaired planning surfaces. | WP-16. |
| WP-18 | Internal review | Review docs, code routes, feature routing, source capture, sprint/issue plans, and release-tail packets for missing surfaces, stale claims, overclaims, or unowned blockers. | Internal review packet and finding register. | WP-17. |
| WP-19 | External review | Prepare and run the external/third-party review handoff after internal review remediation is ready enough for outside scrutiny. | External review handoff and finding register. | WP-18. |
| WP-20 | Remediation and preflight | Fix internal/external review findings, rerun focused checks, update checklists, and record only evidence-backed residual risks explicitly approved by the operator. | Remediation PRs, preflight packet, final checklist updates. | WP-19. |
| WP-21 | Next milestone planning | Prepare v0.92 planning inputs from reviewed v0.91.7 bridge truth without reopening v0.91.7 scope. | v0.92 planning seed and source-capture handoff. | WP-20. |
| WP-22 | Next milestone review | Review v0.92 planning inputs for missing activation blockers, overclaims, and stale bridge assumptions before v0.92 opens. | v0.92 planning review packet. | WP-21. |
| WP-23 | Release ceremony | Finalize release evidence, closeout truth, release notes/checklist state, and ceremony packet after all review findings are fixed or explicitly blocked with evidence and operator approval. | Release ceremony packet and final closeout record. | WP-22. |

## Acceptance Mapping

- v0.91.6 closeout truth and ADR release-tail decisions must be consumed before `v0.92` opens.
- SEP/VPP/PVF/template/session-ledger/workflow-adoption work must make sprint execution predictable rather than chat-memory driven, with the v0.91.6 C-SDLC completion stream named in the source-capture ledger serving as the integrated-control-plane input.
- Goal and metrics work must preserve issue/sprint token/time/resource accounting, separating forward capture from bounded v0.91.6 backfill.
- Scheduler/provider work must protect premium capacity and support local/hosted model routing.
- Capability-envelope, capability-testing, and Aptitude Atlas boundaries must be explicit before v0.92 consumes memory/identity/birthday evidence.
- Build/validation work must reduce the validation tail without weakening proof; EC2 Spot or another disposable remote-builder path must be proven before it becomes a release-critical lane.
- GitHub convergence/control-plane work must be reliable enough for sprint execution or explicitly recorded as a v0.92 blocker with evidence and operator approval. The `#4622` repo-native PR inventory command removes the `missing_owner_binary_cargo_fallback_disabled` failure from release-tail issue/PR inventory.
- Runtime integration/Soak #2 must prove one assembled minimal runtime path or name blockers before birthday activation.
- Runtime architecture diet must identify keep/merge/defer/retire boundaries without hiding speculative refactoring inside the integration sprint.
- Security and ACIP/A2A residuals must not silently defer out of activation.
- Curiosity, Constructability, reasoning graphs, affect/happiness, Godel mechanics, economics, guilds, CodeFriend/adapter work, and publication surfaces must be bounded by evidence and non-claims.
- Launch planning must inform v0.92 sequencing without expanding v0.92 implementation scope.

## Exit Criteria

- WP-01 can open concrete issues without reconstructing the plan from chat.
- Every source in `PLANNING_SOURCE_CAPTURE_v0.91.7.md` is integrated/proven, already closed with evidence, explicitly non-claimed with operator approval, or blocked with evidence and operator approval.
- `#3780` can refresh v0.92 activation docs from tracked bridge truth.
