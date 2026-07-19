# v0.91.7 Work Breakdown Structure

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Created: `2026-06-21`
- Last verified: `2026-07-18`
- Status: closeout tail active; WP-01 through WP-17 closed, WP-18, WP-19, WP-20, WP-21A, and WP-23 open
- Setup lineage: `#3801`, `#3825`, `#4368`
- Source capture: `PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- Release-tail handoff addendum: `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`

## Status

WP allocation is promoted into the v0.91.7 issue wave. WP-01 is `#4628`; WP-02 through WP-23 are `#4629` through `#4650`. Existing v0.91.7 issues are assigned rather than duplicated: `#4603` belongs to WP-06, `#4617` belongs to WP-04, `#4622` belongs to WP-02, and integrated logging/OTel proof `#4718` belongs to WP-07 with WP-08/WP-09 consumers. `#4622` is closed and delivered the repo-native PR inventory command required for release-tail review.

Live WP truth last verified on 2026-07-18: WP-01 through WP-17 are closed;
WP-17 closed through issue #4644 and merged PR #5539. WP-18, WP-19, WP-20,
WP-21A, and WP-23 are open; WP-21 and WP-22 are closed retained planning evidence.
This sequence snapshot is issue-state truth only.
Review cleanliness and release consumption remain governed by
`review/V0917_SPRINT_REVIEW_REGISTER.md` and issue-local proof packets.

WP-01 consumes this document, `PLANNING_SOURCE_CAPTURE_v0.91.7.md`, and [WP_ISSUE_WAVE_v0.91.7.yaml](WP_ISSUE_WAVE_v0.91.7.yaml), then keeps the opened issue wave and planning truth aligned before dependent execution begins.

## WBS Summary

`v0.91.7` should make its inputs to the required `v0.91.8` bridge explicit.
The reviewed v0.91.8 exact-revision handoff, not this WBS directly, is the
consumable platform prerequisite for `v0.92`. v0.91.7 combines the operational
substrate needed for that bridge: sprint execution, validation planning,
goal/metrics accounting, scheduler/provider/local-agent execution, build
throughput, integrated logging/OTel proof, runtime integration/soak, runtime
architecture diet, signal operations, security/protocol implementation,
demos, and launch handoff.

Completion standard: planned, documented, mocked, component-proven, assigned, or merely owned work does not count as done for a product, runtime, or release-gating surface. Any activation-path surface must exit v0.91.7 as `integrated_proven`, `already_closed_with_evidence`, `operator_scoped_out`, or `blocked_with_evidence`. Scoped-out exits require evidence, risk, and explicit operator approval; blocked exits require owner, evidence, risk, and explicit operator approval. Assignment to another issue or later milestone is scheduling truth only, not completion truth.

## WP Sequence

| WP | Work Package | Description | Primary deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Planning promotion and issue-wave readiness | Promote the refreshed planning package, consume `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`, classify every source in the source-capture ledger, sync the feature-list/roadmap truth, and open the issue wave. | Opened issue wave, C-SDLC card bundles, and source/feature-list disposition ledger. | Pre-v0.92 source ledger, v0.91.6 closeout truth, source-capture ledger, and feature-list/roadmap truth. |
| WP-02 | Closeout truth, ADR disposition, and carryover cleanup | Consume release-tail, ADR, Observatory, C-SDLC, and late-control-plane carryovers from the source-capture ledger; classify each as integrated/proven, already closed with evidence, or explicitly blocked with evidence and operator approval. | Closeout truth ledger, ADR disposition, tooling-remediation disposition, C-SDLC control-plane disposition, late-input disposition, and carryover proof/blocker updates. | WP-01. |
| WP-03 | Consume C-SDLC integration control-plane truth | Consume the v0.91.6 C-SDLC/control-plane completion stream named in the source-capture ledger. Do not recreate completed v0.91.6 work as fresh v0.91.7 implementation scope unless WP-02 records a blocker that must be implemented before v0.92. | Process/tooling truth-consumption gate with explicit implementation owner or evidence-backed blocker only where v0.91.6 did not complete. | WP-01, WP-02, source-capture ledger. |
| WP-04 | Goal state, nested goals, and execution metrics | Implement first-class goal-state consumption, issue/sprint goal nesting, SOR time/token/resource accounting, forward capture, bounded archaeology/backfill, host goal snapshots, and outlier analysis, or record an evidence-backed blocker with operator approval. | Goal/metrics implementation proof and template-field updates. | WP-03 and source-capture ledger. |
| WP-05 | Cognitive scheduler and provider/local-agent routing | Implement scheduler v1, provider profiles, model suitability, local/hosted agent routing, capability-envelope inputs, cheapest-validated-outcome policy, and local-agent delegation readiness, or block v0.92 with evidence and operator approval. | Scheduler/provider execution proof, role suitability matrix, and capability-envelope proof. | WP-03, WP-04, provider sprint outputs. |
| WP-06 | Build throughput and validation-cost reduction | Implement validation manager, long-test fanout, CI log archive/S3, Nessus, CodeBuild runner evaluation, EC2 Spot or alternate remote-builder proof, `sccache`/linker/target-dir cleanup, and validation DAG/build graph convergence where required before v0.92; otherwise record explicit blockers with evidence and operator approval. | Build/validation throughput sprint, remote-builder proof, and cost/time evidence. | WP-03, validation sprint outputs. |
| WP-07 | Runtime integration, logging/OTel proof, fire-up, and Soak #2 | Assemble the runtime substrate into one minimal end-to-end path, reconcile Runtime v2 minimal prototype with current Tokio/runtime substrate, consume the prerequisite logging/OTel proof from `#4718`, run integrated runtime Soak #2, identify bloat/seam pain, and preserve Soak #3 only as an operator-approved risk if Soak #2 cannot prove the activation path. | Runtime integration proof or evidence-backed blocker list, logging/OTel prerequisite-consumption proof, runtime module map, and architecture-diet follow-on. | WP-02, WP-05, WP-06. |
| WP-08 | Runtime AWS and signal operations | Integrate heartbeat publisher, ACIP-to-SNS, AWS signal integration, local polis SSM, and future S3/ObsMem/community-memory archive policy enough to produce runtime AWS/local operations evidence, or block v0.92 with evidence and operator approval. | Runtime AWS/local operations proof and proof expectations. | WP-07, security inputs. |
| WP-09 | Observatory, demos, and birthday-visible proof | Finish Unity/HTML Observatory, demo matrix convergence, and first-birthday-visible proof surfaces with retained evidence; unsupported runtime claims become public claim boundaries or blockers, not scheduled completion. | Demo/Observatory readiness packet. | WP-07, WP-08. |
| WP-10 | Curiosity and Constructability | Implement governed discovery-cycle proof and shared-reality/anchor/validator boundaries required before v0.92, or block activation with evidence and operator approval. | Curiosity and Constructability integrated proof records or evidence-backed blockers. | WP-01, WP-07. |
| WP-11 | Reasoning graph, loops, and `adl.skill.v1` | Implement prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1` enough for v0.92 activation, or record evidence-backed blockers. | Reasoning graph / skill-standard integrated proof. | WP-03, WP-07, WP-10. |
| WP-12 | Security, CAV, SSM, and ACIP/A2A/protobuf implementation | Implement security/CAV, SSM readiness, ACIP/A2A/protobuf/JSON/WebSocket/access-rule choices, and activation-path blockers; anything unresolved must block v0.92 with evidence and operator approval. | Security/protocol implementation packet. | WP-08, WP-10. |
| WP-13 | Affect, happiness, Godel mechanics, economics, guild, CodeFriend, and publication boundaries | Implement MVP-scope affect-model, Godel, guild, CodeFriend/adapter v2, and required boundary behavior before v0.92; paper/publication surfaces may be scoped out only with evidence and operator approval. | Boundary decision packet, MVP-scope proof/blocker rows, handoff rows, and parent closeout packet `review/wp13_closeout_4640.md`. | WP-05, WP-10, WP-11. |
| WP-14 | Launch and v0.92 birthday handoff | Align July launch planning, v0.92 activation refresh, Memory Palace/context, capability envelope, birth witnesses/receipt, first birthday docs, and public claim boundaries. | `V092_HANDOFF_v0.91.7.md` refresh and launch/birthday readiness map. | WP-02 through WP-13. |
| WP-15 | Demo convergence | Confirm Observatory/demo matrix truth, visible proof status, public demo claim boundaries, and any evidence-backed demo blockers before review. | Demo convergence packet and demo matrix update. | WP-09, WP-14. |
| WP-16 | Quality gate | Run focused repo-quality, checklist, stale-doc, validation-plan, and release-readiness checks appropriate for an implementation/proof milestone. | Quality-gate packet and blocker list. | WP-14, WP-15. |
| WP-17 | Documentation alignment | Align README, feature docs, WBS, sprint plan, checklist, handoff, issue wave, and feature-list/roadmap truth before formal review. | Docs alignment packet and repaired planning surfaces. | WP-16. |
| WP-18 | Internal review | Review docs, code paths, feature coverage, source capture, sprint/issue plans, and release-tail packets for missing surfaces, stale claims, overclaims, or unowned blockers. | Internal review packet and finding register. | WP-17. |
| WP-19 | External review | Prepare and run the external/third-party review handoff after internal review remediation is ready enough for outside scrutiny. | External review handoff and finding register. | WP-18. |
| WP-20 | Remediation and preflight | Fix internal/external review findings, rerun focused checks, update checklists, and record only evidence-backed blockers explicitly approved by the operator. | Remediation PRs, preflight packet, final checklist updates. | WP-19. |
| WP-21 | Feature-list, TBD, and v0.92 planning truth alignment | Prepare v0.92 planning inputs from reviewed v0.91.7 implementation/proof truth without reopening v0.91.7 scope; consume observability `#4718` and resilience `#4778` / `#4780`-`#4783` truth explicitly. | v0.92 planning seed, source-capture handoff, and feature-list/TBD disposition update. | WP-20. |
| WP-21A | Next milestone closeout planning | Open as issue #5489 to carry the canonical next-milestone closeout role that WP-21 previously held. | v0.92 closeout-planning packet and review-ready handoff. | WP-21. |
| WP-22 | Next milestone review | Review v0.92 planning inputs for missing activation blockers, overclaims, and stale assumptions before v0.92 opens. | v0.92 planning review packet. | WP-21A. |
| WP-23 | Release ceremony | Finalize release evidence, closeout truth, release notes/checklist state, and ceremony packet after all review findings are fixed or explicitly blocked with evidence and operator approval. | Release ceremony packet and final closeout record. | WP-22. |

## Acceptance Mapping

- Pre-v0.92 required surface coverage:

| Required surface before v0.92 | v0.91.7 issue coverage | Completion bar |
| --- | --- | --- |
| C-SDLC shepherd, watcher, session ledger, predictable execution | `#4630`, `#4713`, `#4709`, `#4721`, `#4953`, WP-03 `#4630` and source-capture inputs | Repo-native tool path works in normal issue/PR flow, or v0.92 remains blocked with evidence and operator approval. |
| Goal/time/token/resource metrics and backfill | `#4631`, `#4666`-`#4670`, `#4617` | Forward capture works from current SPP/VPP/SOR/goal surfaces; backfill gaps are explicit. |
| Scheduler/provider/local-agent execution | `#4632`, `#4671`-`#4675`, `#4653`, `#4654` | Scheduler/provider decisions are executable and protect premium cognition with model-suitability evidence. |
| Build throughput, validation-cost reduction, and Rust simplification | `#4633`, `#4676`-`#4680`, `#4698`, `#4603`, `#4651`, `#4892`-`#4900` | Tail validation and remote/local build lanes are measured, bounded, and operational enough for the milestone; Rust simplification reduces hand-rolled commodity mechanics without generic file splitting. |
| Runtime integration and Soak #2 | `#4634`, `#4681`-`#4683` | One minimal runtime path runs with retained soak evidence or named blockers. |
| Integrated logging, observability, and OTel-compatible boundary | `#4718`, `#4682`, WP-07/WP-09 consumers | `#4718` proves parse-safe JSON, stderr `adl_event` behavior, redaction hygiene, and an OTel-compatible mapping boundary. Runtime/Unity/production telemetry consumption requires Soak #2 and Observatory consumers before v0.92 relies on it. |
| AWS/signal operations | `#4635`, `#4684`-`#4688` | Heartbeat, ACIP-to-SNS, AWS signal, local SSM, and archive policy have working evidence or blockers. |
| Observatory/Unity/HTML birthday-visible proof | `#4636`, `#4689`-`#4691`, `#4652`, `#4702`-`#4704` | Visible proof surfaces run, carry public claim boundaries, are operator-scoped-out with evidence, or remain blocked. |
| Curiosity and Constructability | `#4637`, `#4692`, `#4693` | Governed discovery and constructability-anchor proof surfaces are implemented/proven or blocked. |
| Reasoning graph, loops, `adl.skill.v1`, AEE/ObsMem/PVF | `#4638`, `#4694`-`#4697` | Required behavior has at least one producer/consumer or runtime proof path. |
| Security/CAV/SSM/ACIP/A2A/protobuf/WebSocket/access implementation | `#4639`, `#4656`-`#4660`, `#4914`, `#4917`, `#4920` | `docs/milestones/v0.91.7/review/security/WP12_ACCESS_ACTIVATION_GATE_4660.md` is the access/activation gate. Rows marked `integrated_proven` may support scoped claims; closed `#4659` and merged PR `#5146` support a bounded loopback WebSocket transport-path claim, `#4914` supports bounded retained red/blue CAV claims, and live WebSocket runtime API integration is backlog-only until promoted in the next milestone. |
| Affect/happiness/Godel/economics/guild/CodeFriend/publication boundaries | `#4640`, `#4752`-`#4757` | MVP-scope affect/Godel/guild/CodeFriend obligations are reconciled by `review/wp13_closeout_4640.md`; publication and inner-state claims are claim-bounded by retained WP-13 packets, including the #4757 publication non-claim and promotion-gate packet. |

- v0.91.6 closeout truth and ADR release-tail decisions must be consumed before `v0.92` opens.
- WP-02 closeout truth is split across child issues. `#4661` owns only the
  v0.91.6 closeout-truth consumption packet
  `docs/milestones/v0.91.7/review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md`;
  `#4662`-`#4665` own ADR, Observatory, C-SDLC control-plane, and PR-inventory
  child dispositions.
- SEP/VPP/PVF/template/session-ledger/workflow-adoption work must make sprint execution predictable rather than chat-memory driven, with the v0.91.6 C-SDLC completion stream named in the source-capture ledger serving as the integrated-control-plane input.
- Goal and metrics work must preserve issue/sprint token/time/resource accounting, separating forward capture from bounded v0.91.6 backfill.
- Scheduler/provider work must protect premium capacity and support local/hosted model routing.
- Capability-envelope, capability-testing, and Aptitude Atlas boundaries must be explicit before v0.92 consumes memory/identity/birthday evidence.
- Build/validation work must reduce the validation tail without weakening proof; EC2 Spot or another disposable remote-builder path must be proven before it becomes a release-critical lane.
- WP-06 selected sprint-lane truth is retained in
  `review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md`.
  Merged/closed children `#4676`, `#4800`, `#4698`, `#4726`, `#4677`, and
  `#4678` have settled PR truth. Remote-builder follow-ups `#4837`, `#4838`,
  `#4879`, `#4928`, `#4680`, and umbrella `#4679` are merged/reconciled with
  retained Spot, CodeBuild, builder-image, cache, routing, and benchmark proof.
  The remaining `pr watch` `closeout_needed` ambiguity on already-closeouted
  closed issues is tracked as tooling bug `#4950`, not as WP-06 implementation
  residue.
- GitHub convergence/control-plane work must be reliable enough for sprint execution or explicitly recorded as a v0.92 blocker with evidence and operator approval. The `#4622` repo-native PR inventory command removes the `missing_owner_binary_cargo_fallback_disabled` failure from release-tail issue/PR inventory.
- Runtime integration/Soak #2 must prove one assembled minimal runtime path or name blockers before birthday activation.
- Logging/observability is not optional polis infrastructure. `#4718` is the prerequisite proof for parse-safe JSON, stderr `adl_event` behavior, redaction hygiene, and OTel-compatible mapping; Soak #2 and Observatory/Unity consumers must consume that proof before v0.92 may rely on logging readiness.
- Runtime architecture diet must identify keep/merge/postpone/retire boundaries without hiding speculative refactoring inside the integration sprint.
- Security and ACIP/A2A requirements must not silently move out of activation.
- Curiosity, Constructability, reasoning graphs, affect/happiness, Godel mechanics, economics, guilds, CodeFriend/adapter work, and publication surfaces must be bounded by evidence and public claim rules.
- Launch planning must inform v0.92 sequencing without expanding v0.92 implementation scope.

## Exit Criteria

- WP-01 can open concrete issues without reconstructing the plan from chat.
- Every source in `PLANNING_SOURCE_CAPTURE_v0.91.7.md` is integrated/proven, already closed with evidence, operator-scoped-out with evidence and approval, or blocked with evidence and operator approval.
- `#3780` can refresh v0.92 activation docs from tracked implementation/proof truth.
