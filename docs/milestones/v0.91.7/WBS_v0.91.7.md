# v0.91.7 Candidate Work Breakdown Structure

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Status: candidate WP sequence for final pre-`v0.92` bridge and readiness tranche
- Setup lineage: `#3801`, `#3825`, `#4368`
- Source capture: `PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- Release-tail handoff addendum: `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`

## Status

Candidate allocation only. `v0.91.7` issues should be opened from this WBS only after the current `v0.91.6` closeout truth is reviewed. WP-01 must consume the failed WP-15 `#3980` external-review truth, WP-16 `#3981`, WP-16 children `#4620` and `#4621`, tooling child `#4622`, and closed WP-14A remediation truth before dependent execution work begins.

WP-01 should consume this document, `PLANNING_SOURCE_CAPTURE_v0.91.7.md`, and [WP_ISSUE_WAVE_v0.91.7.yaml](WP_ISSUE_WAVE_v0.91.7.yaml), then open concrete GitHub issues with canonical C-SDLC cards.

## WBS Summary

`v0.91.7` should make the path to `v0.92` explicit. It combines residual bridge docs with the operational substrate needed for first-birthday execution: sprint execution, validation planning, goal/metrics accounting, scheduler/provider/local-agent routing, build throughput, runtime integration/soak, runtime architecture diet, security/protocol residuals, demos, and launch handoff.

## Candidate WP Sequence

| WP | Work Package | Description | Primary deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Planning promotion and issue-wave readiness | Promote the refreshed planning package, reconcile v0.91.6 closeout truth, consume `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`, classify every source in the source-capture ledger, sync the feature-list/roadmap truth, and open the issue wave. | Opened issue wave, C-SDLC card bundles, and source/feature-list disposition ledger. | `#3778`, `#3800`, `#3801`, `#4368`, failed-review truth from `#3980`, final WP-16 `#3981` / `#4620` / `#4621`, PR inventory route `#4622`, and v0.91.6 closeout truth. |
| WP-02 | v0.91.6 closeout truth, ADR release-tail, C-SDLC control-plane sprint, and cleanup | Ensure open v0.91.6 closeout/release-tail WPs, Observatory carryover, closed ADR release-tail issues `#4324` / `#4369`-`#4376`, open tooling remediation `#4378`, the `#4388`-`#4398` C-SDLC control-plane sprint, and late control-plane issues `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, `#4433`-`#4438`, and `#4442`-`#4443` are consumed, closed, blocked, deferred, or routed. | Closeout truth ledger, ADR route, tooling-remediation route, C-SDLC control-plane disposition, late-input disposition, and carryover routing updates. | WP-01. |
| WP-03 | Consume C-SDLC integration control-plane truth | Consume the v0.91.6 completion truth for Sprint Execution Packets, VPP, external PVF lane registry, prompt-template next version, GitHub/octocrab convergence, `/goal` rules, session ledger, issue/sprint metrics, metric backfill, logging, watchers, closeout, validation-manager/SOR-fact routing, AST/template integration routes, operational adoption, lifecycle shepherding, and FastContext evaluation. Do not recreate this as fresh v0.91.7 implementation scope unless WP-02 records a blocker or routed follow-on. | Process/tooling truth-consumption gate with explicit blocker/follow-on routes only where v0.91.6 did not complete. | WP-01, WP-02, `#4308`, `#4309`, `#4332`, `#4388`-`#4398`, `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, `#4433`-`#4438`, `#4442`-`#4443`. |
| WP-04 | Goal state, nested goals, and execution metrics | Define first-class goal-state consumption, issue/sprint goal nesting, SOR time/token/resource accounting, forward capture, bounded archaeology/backfill, host goal snapshots, and outlier analysis route. | Goal/metrics feature route and template-field plan. | WP-03, `#4329`, `#4331`, `#4431`, `#4441`, `#4442`. |
| WP-05 | Cognitive scheduler and provider/local-agent routing | Build or route scheduler v1, provider profiles, model suitability, local/hosted agent routing, capability-envelope inputs, and cheapest-validated-outcome policy. | Scheduler/provider execution route, role suitability matrix, and capability-envelope routing. | WP-03, WP-04, provider sprint outputs. |
| WP-06 | Build throughput and validation-cost reduction | Route validation manager, long-test fanout, CI log archive/S3, Nessus, CodeBuild runner evaluation, EC2 Spot or alternate remote-builder proof, `sccache`/linker/target-dir cleanup, and validation DAG/build graph convergence. | Build/validation throughput sprint, remote-builder proof plan, and cost/time evidence route. | WP-03, validation sprint outputs. |
| WP-07 | Runtime integration, fire-up, and Soak #2 | Assemble the runtime substrate into one minimal end-to-end path, reconcile Runtime v2 minimal prototype with current Tokio/runtime substrate, run or route integrated runtime Soak #2, identify bloat/seam pain, and preserve Soak #3 contingency. | Runtime integration proof or blocker list, runtime module map, and architecture-diet follow-on route. | WP-02, WP-05, WP-06. |
| WP-08 | Runtime AWS and signal bridge operations | Carry heartbeat publisher, ACIP-to-SNS, AWS signal bridge, local polis SSM, and future S3/ObsMem/community-memory archive policy into one operational route. | Runtime AWS/local operations route and proof expectations. | WP-07, security inputs. |
| WP-09 | Observatory, demos, and birthday-visible proof | Finish or route Unity/HTML Observatory, demo matrix convergence, and first-birthday-visible proof surfaces without claiming unsupported runtime completion. | Demo/Observatory readiness packet. | WP-07, WP-08. |
| WP-10 | Curiosity and Constructability bridge | Preserve governed discovery-cycle proof expectations and shared-reality/anchor/validator boundaries. | Curiosity and Constructability issue-ready docs or proof routes. | WP-01, WP-07. |
| WP-11 | Reasoning graph, loops, and `adl.skill.v1` bridge | Define the bridge among prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1`. | Reasoning graph / skill-standard bridge route. | WP-03, WP-07, WP-10. |
| WP-12 | Security, CAV, SSM, and ACIP/A2A/protobuf residuals | Account for security/CAV residuals, SSM readiness, ACIP/A2A/protobuf/JSON/WebSocket/access-rule choices, and activation-path blockers. | Security/protocol residual decision packet. | WP-08, WP-10. |
| WP-13 | Affect, happiness, Godel mechanics, economics, guild, CodeFriend, and publication boundaries | Define safe-test/non-claim language and governance/civilization/economics boundaries; route guilds, CodeFriend/adapter v2, and paper/publication surfaces without making them birthday implementation blockers. | Boundary decision packet and handoff rows. | WP-05, WP-10, WP-11. |
| WP-14 | Launch and v0.92 birthday handoff | Align July launch planning, v0.92 activation refresh, Memory Palace/context, capability envelope, birth witnesses/receipt, first birthday docs, and external-facing non-claims. | `V092_HANDOFF_v0.91.7.md` refresh and launch/birthday readiness map. | WP-02 through WP-13. |
| WP-15 | Demo convergence | Confirm Observatory/demo matrix truth, visible proof status, demo non-claims, and any blocked/deferred demo surfaces before review. | Demo convergence packet and demo matrix update. | WP-09, WP-14. |
| WP-16 | Quality gate | Run focused repo-quality, checklist, stale-doc, validation-plan, and release-readiness checks appropriate for a planning/bridge milestone. | Quality-gate packet and blocker list. | WP-14, WP-15. |
| WP-17 | Documentation alignment | Align README, feature docs, WBS, sprint plan, checklist, handoff, issue wave, and feature-list/roadmap truth before formal review. | Docs alignment packet and repaired planning surfaces. | WP-16. |
| WP-18 | Internal review | Review docs, code routes, feature routing, source capture, sprint/issue plans, and release-tail packets for missing surfaces, stale claims, overclaims, or unowned blockers. | Internal review packet and finding register. | WP-17. |
| WP-19 | External review | Prepare and run the external/third-party review handoff after internal review remediation is ready enough for outside scrutiny. | External review handoff and finding register. | WP-18. |
| WP-20 | Remediation and preflight | Fix or route internal/external review findings, rerun focused checks, update checklists, and record residual risks. | Remediation PRs, preflight packet, final checklist updates. | WP-19. |
| WP-21 | Next milestone planning | Prepare v0.92 planning inputs from reviewed v0.91.7 bridge truth without reopening v0.91.7 scope. | v0.92 planning seed and source-capture handoff. | WP-20. |
| WP-22 | Next milestone review | Review v0.92 planning inputs for missing activation blockers, overclaims, and stale bridge assumptions before v0.92 opens. | v0.92 planning review packet. | WP-21. |
| WP-23 | Release ceremony | Finalize release evidence, closeout truth, release notes/checklist state, and ceremony packet after all review findings are complete, blocked, deferred, or routed. | Release ceremony packet and final closeout record. | WP-22. |

## Acceptance Mapping

- v0.91.6 closeout truth and ADR release-tail decisions must be consumed before `v0.92` opens.
- SEP/VPP/PVF/template/session-ledger/workflow-adoption work must make sprint execution predictable rather than chat-memory driven, with `#4388`-`#4398` plus late inputs `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, and `#4433`-`#4438` serving as the v0.91.6 completion stream for the integrated control plane.
- Goal and metrics work must preserve issue/sprint token/time/resource accounting, separating forward capture `#4431` from bounded v0.91.6 backfill `#4441`.
- Scheduler/provider work must protect premium capacity and support local/hosted model routing.
- Capability-envelope, capability-testing, and Aptitude Atlas boundaries must be explicit before v0.92 consumes memory/identity/birthday evidence.
- Build/validation work must reduce the validation tail without weakening proof; EC2 Spot or another disposable remote-builder path must be proven before it becomes a release-critical lane.
- GitHub convergence/control-plane work must be either reliable enough for sprint execution or explicitly routed as a blocker/follow-on through the v0.91.6 control-plane completion stream and the v0.91.7-facing lifecycle shepherd `#4443`.
- Runtime integration/Soak #2 must prove one assembled minimal runtime path or name blockers before birthday activation.
- Runtime architecture diet must identify keep/merge/defer/retire boundaries without hiding speculative refactoring inside the integration sprint.
- Security and ACIP/A2A residuals must not silently defer out of activation.
- Curiosity, Constructability, reasoning graphs, affect/happiness, Godel mechanics, economics, guilds, CodeFriend/adapter work, and publication surfaces must be bounded by evidence and non-claims.
- Launch planning must inform v0.92 sequencing without expanding v0.92 implementation scope.

## Exit Criteria

- WP-01 can open concrete issues without reconstructing the plan from chat.
- Every source in `PLANNING_SOURCE_CAPTURE_v0.91.7.md` is complete, blocked, deferred, or routed.
- `#3780` can refresh v0.92 activation docs from tracked bridge truth.
