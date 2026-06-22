# v0.91.7 Planning Source Capture

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Issue: `#4368`
- Status: planning-source ledger for v0.91.7 scheduling

## Purpose

Capture the source surfaces that must be considered before `v0.91.7` opens for execution so ADL does not rediscover required pre-`v0.92` work during the birthday milestone.

This document is a routing ledger, not proof of implementation. It distinguishes tracked milestone evidence, local TBD planning inputs, existing open issues, and deferred or non-authoritative scratch.

## Planning Rule

`v0.91.7` should be planned as the final pre-`v0.92` bridge and readiness tranche.

It must answer three questions before `v0.92` opens:

1. What must be completed before first birthday activation can begin?
2. What may be explicitly deferred, blocked, or routed without weakening the birthday milestone?
3. What operational substrate must be stable enough that `v0.92` can execute quickly and predictably?

## Tracked Inputs

| Source | Planning use | Required v0.91.7 handling |
| --- | --- | --- |
| `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md` | Pre-`v0.92` bridge ledger / `#3778` source | Consume as upstream bridge authority; do not let v0.91.7 contradict the ledger without an explicit decision. |
| `docs/milestones/v0.91.6/` | First bridge tranche, runtime/tooling/provider/security/observability evidence | Consume closeout truth; do not duplicate completed work. |
| `docs/milestones/v0.91.6/review/` | Sprint reviews, remediation, proof packets, and retained evidence | Convert residual findings into explicit v0.91.7 routes or deferrals. |
| `docs/milestones/v0.91.6/RUNTIME_FIRE_UP_PLAN_v0.91.6.md` | Runtime fire-up and soak continuity | Carry into runtime Soak #2 / integrated runtime proof. |
| `docs/milestones/v0.91.6/features/COGNITIVE_SCHEDULER_v0.91.6.md` | Scheduler v1 bridge | Preserve as scheduler/economics input, not just docs residue. |
| `docs/milestones/v0.91.6/review/scheduler/` | Scheduler proof and economics inputs | Feed v0.91.7 scheduler execution or closeout route. |
| `docs/milestones/v0.91.6/review/build_throughput/` | Build throughput, sccache/linker/target-dir/CodeBuild/Nessus evidence | Feed validation/build-throughput follow-ons and remote-build decisions. |
| `docs/milestones/v0.91.6/review/provider/` | Provider reliability, suitability, profiles v2, role profiles | Feed provider/scheduler/local-agent routes and v0.92 model-readiness boundaries. |
| `docs/milestones/v0.91.6/review/security/` | CAV, SSM, access-rule, security residual evidence | Feed security residual and v0.93 enterprise-security handoff. |
| `docs/milestones/v0.91.6/review/runtime_aws_signal_bridge/` | ACIP-to-SNS and heartbeat bridge proof | Feed runtime AWS/heartbeat operational route and later ObsMem/community memory. |
| `docs/milestones/v0.91.6/review/sprint_execution_packets/` | SEP, activity logs, sprint-conductor simulation | Feed sprint-execution process and nested-goal/PVF scheduling. |
| `docs/templates/planning/` | Milestone planning template authority | Use current planning template process; do not hand-roll canonical docs. |
| `docs/templates/sprints/` | Sprint Execution Packet template authority | Use for mini-sprint/sprint setup and closeout expectations. |
| `docs/templates/prompts/` | SIP/STP/SPP/SRP/SOR template authority | Preserve card lifecycle and planned VPP/template-version changes. |
| `docs/planning/ADL_FEATURE_LIST.md` | Roadmap/feature-list truth surface | Refresh, reconcile, or explicitly route stale feature-list rows before v0.92 planning consumes them. |

## Open-Issue Inputs Observed During #4368

These were open at the source-capture pass and should be explicitly closed, completed, moved, or routed before `v0.92` starts.

| Issue | Planning classification | v0.91.7 relevance |
| --- | --- | --- |
| `#3974` | Observatory mini-sprint umbrella | Must complete or truthfully route Observatory readiness. |
| `#3976`-`#3984` | v0.91.6 release-tail WPs | Must close v0.91.6 before v0.91.7/v0.92 sequencing is trusted. |
| `#4030`-`#4035`, `#4341` | Observatory children | Must be resolved or explicitly carried into v0.91.7 demo/runtime readiness. |
| `#4286` | PR closing-linkage guard | Tooling residual; route with process-hardening work. |
| `#4299` | Issue resource telemetry archive | Feeds metrics/S3/ObsMem history. |
| `#4308` | VPP and externalized PVF lane registry | Required for validation planning prompt substrate. |
| `#4309` | Next prompt-template version with VPP, time, token, goal fields | Required template-version work before full SEP/VPP maturity. |
| `#4317` | Nessus remote Rust validation runner | Build/validation throughput route. |
| `#4322` | CI checks and validation cost review | Validation/test-tax route. |
| `#4324`, `#4369`-`#4376`, `#4378` | ADR mini-sprint, candidate ADR reviews, and tooling remediation | `#4324` and `#4369`-`#4376` are closed inputs; open `#4378` must be completed, blocked, deferred, or routed by WP-02 before v0.92 handoff. |
| `#4329` | Per-issue execution metrics foundation | Required for time/token prediction and issue baselines. |
| `#4331` | First-class nested goal accounting | Closed input that must be consumed by goal-state and SOR metrics planning. |
| `#4332` | VPP and PVF lane-template mini-sprint | Required validation-planning sprint; should follow template substrate as needed. |
| `#4388`-`#4398` | v0.91.6 C-SDLC integration control-plane completion sprint | Required v0.91.6 completion sprint for SEP, VPP, PVF lane configuration, prompt-card templates, GitHub/octocrab convergence, goal metrics, logging, watcher/lifecycle automation, runtime dependency routing, tooling reliability, and FastContext evaluation before v0.92 depends on sprint-scale execution. v0.91.7 should consume the closed/blocked/routed truth from this sprint rather than recreate it as new scope. |
| `#4405` | Session coordination and root checkout policy | Closed input for multi-session root-checkout safety; WP-02/WP-03 must consume it before v0.91.7 parallel execution starts. |
| `#4412` | Session ledger and cross-session coordination commands | Required input for multi-account/multi-agent continuity; must be complete, blocked, deferred, or routed before v0.92 relies on parallel session handoff. |
| `#4413` | PR lifecycle delegate stalls and command liveness | Required tooling-reliability input for long-running lifecycle commands. |
| `#4417`-`#4421` | Validation throughput and lifecycle automation mini-sprint | Required input for path ownership, `pr ready` / `pr run` session-ledger integration, SOR fact emission, and validation-manager path-profile selection. |
| `#4425` | VPP generation from ownership and validation profile facts | Required input so VPPs become generated planning artifacts rather than chat-memory expectations. |
| `#4431` | Forward authoritative workflow time/token accounting | Required forward-capture fix; must not be conflated with historical backfill. |
| `#4441` | v0.91.6 workflow metric backfill evidence | Bounded archaeology/backfill input for v0.91.6 only; token values must remain explicit or `not_collected`, never inferred. |
| `#4433`-`#4438` | Operationalize adopted C-SDLC practices after PVF2 | Required adoption sprint so watchers, VPPs, shepherding, card readiness, SOR fact capture, and full-path proof become operational defaults. |
| `#4442` | Native host-integrated goal snapshot capture | v0.91.7-facing input for host/session goal accounting; route through WP-04 unless completed before v0.91.7 starts. |
| `#4443` | Full issue-lifecycle shepherd | v0.91.7-facing lifecycle input above watcher/janitor/closeout; route through WP-03 unless completed before v0.91.7 starts. |
| `#4368` | v0.91.7 planning docs | This source-capture and planning alignment issue. |

## Local TBD Inputs To Capture Or Route

Local `.adl/docs/TBD/` files are ignored planning inputs, not tracked proof. They are cited here as source material that must be either promoted into tracked docs/issues or explicitly deferred.

| Source | Disposition for v0.91.7 planning |
| --- | --- |
| `.adl/docs/TBD/ADL_GOAL_STATE.md` | Schedule as goal-state/nested-goal substrate input; connect to issue metrics, SOR accounting, and v0.92 continuity. |
| `.adl/docs/TBD/ADL_COGNITIVE_SCHEDULER_v1.md` | Schedule as scheduler execution route; connect provider profiles, aptitude, local agents, quota/cost, and sprint orchestration. |
| `.adl/docs/TBD/ADL_COGNITIVE_ECONOMICS.md` | Use as scheduler/economics rationale; default to context and routing unless a bounded test is promoted. |
| `.adl/docs/TBD/ADL_BUILD_IMPROVEMENTS.md` | Use for build-throughput, remote validation, Nessus, CodeBuild, sccache/linker, and validation-DAG convergence routes. |
| `.adl/docs/TBD/LAUNCH_PLAN_JULY_2026.md` | Route into v0.91.7/v0.92 launch-readiness planning; do not let launch work silently expand birthday scope. |
| `.adl/docs/TBD/LOCAL_BACKLOG.md` | Use as the local backlog source for queued work that is not already represented as a public GitHub issue; promote only bounded rows with clear milestone fit. |
| `.adl/docs/TBD/ADL_AND_GUILDS.md` | Route into v0.91.7/v0.93 governance planning; do not make guilds a v0.92 implementation blocker unless explicitly promoted. |
| `.adl/docs/TBD/workflow_tooling/PARALLEL_EXECUTION_LANES_AND_COMPRESSION_MODEL.md` | Account in PVF/VPP/validation-lane planning. |
| `.adl/docs/TBD/workflow_tooling/planning/SPRINT_CYCLE_TIME_REDUCTION_PLAN.md` | Account in SEP, sprint-conductor, VPP, and validation manager routes. |
| `.adl/docs/TBD/tools/VALIDATION_MANAGER_TEST_TAX_RECOVERY_PLAN.md` | Account in validation manager / test-tax / CI lane planning. |
| `.adl/docs/TBD/csm_observatory/UNITY_OBSERVATORY_DEMO.md` | Align with Observatory route; do not claim Unity completion until issue evidence exists. |
| `.adl/docs/TBD/runtime_v2/RUNTIME_V2_MINIMAL_PROTOTYPE.md` | Reconcile with current runtime fire-up/soak plan; preserve minimal runtime proof requirements. |
| `.adl/docs/TBD/Test_Tax_Prompt_2.md` | Retire as scratch after captured validation/test-tax facts are represented elsewhere. |
| `.adl/docs/TBD/RUSTDOC_GAP_ANALYSIS.md` | Leave as standing Rust refactoring documentation; do not duplicate unless refactoring sprint consumes it. |

## Issue-Draft Inputs

| Source draft | Planning use |
| --- | --- |
| `issue-v0917-required-bridge-tranche.md` | Original v0.91.7 bridge-scope statement; keep as boundary source. |
| `issue-pre-v092-bridge-feature-doc-production.md` | Cross-milestone feature-doc production source; ensure v0.91.6/v0.91.7 split remains explicit. |
| `issue-v092-activation-birthday-feature-doc-refresh.md` | v0.92 WP-01 input; v0.91.7 should produce what this needs. |
| `issue-feature-list-roadmap-sync.md` | Feature-list/roadmap sync candidate; route to planning/feature-list work. |
| `issue-test-cycle-architecture-split.md` | Validation architecture split candidate; route to validation/PVF/build-throughput planning. |
| `issue-codefriend-v1-proof-and-adapter-v2-acceptance.md` | Pre-v0.95 proof-planning input; preserve but do not expand v0.91.7 unless launch readiness needs it. |
| `issue-ci-runtime-budget-observability.md` | CI/runtime budget and observability route; connect with resource telemetry and build logs. |
| `issue-memory-palace-v092-bridge-feature-doc.md` | v0.92 memory bridge input; ensure v0.91.7 handoff names it. |
| `issue-v093-v095-mvp-feature-doc-production.md` | Later feature-doc wave source; route, do not implement in v0.91.7. |

## Required v0.91.7 Scheduling Themes

| Theme | Why it matters before v0.92 | Scheduling posture |
| --- | --- | --- |
| v0.91.6 closeout and release-tail truth | v0.92 cannot consume incomplete or stale bridge truth. | First gate. |
| SEP/VPP/PVF, session-ledger, lifecycle automation, and template-version work | Sprint execution, validation planning, and time/token accounting must be predictable. `#4388`-`#4398`, `#4405`, `#4412`-`#4413`, `#4417`-`#4425`, and `#4433`-`#4438` must exit complete, blocked, deferred, or routed before v0.92 relies on sprint-scale execution. | v0.91.6 closeout input consumed by v0.91.7 WP-02/WP-03. |
| Goal state, nested goals, time/token/resource metrics | v0.92 needs continuity and issue/sprint accounting. `#4431` owns forward capture; `#4441` owns bounded v0.91.6 backfill; `#4442` owns host-integrated goal snapshots. | Early process/runtime bridge. |
| Cognitive scheduler and local-agent acceleration | Premium cognition is now a bottleneck; local/deepseek/hosted agent suitability must route work. | Early scheduler/provider sprint. |
| Capability envelope and capability-testing boundary | v0.92 memory/identity/birthday evidence depends on knowing what capability envelope, witnesses/receipt, and Aptitude Atlas evidence may or may not claim. | Scheduler/provider and handoff bridge. |
| Build throughput and validation cost | C-SDLC speed is limited by build/validation tail. | Parallel validation/build sprint. |
| Runtime integration / Soak #2 / fire-up | v0.92 birthday needs one assembled runtime path, not disconnected components or docs-only confidence. | Runtime sprint after substrate readiness. |
| Runtime architecture diet | Integration will expose bloat, seams, duplicate abstractions, and premature surfaces that should be routed without blocking the first assembled runtime proof. | Runtime sprint output plus bounded follow-on route. |
| Observatory and demo readiness | First birthday evidence needs visible runtime/workflow surfaces. | Demo/runtime visibility sprint. |
| Curiosity and constructability | Major conceptual bridge surfaces need proof expectations before public consumption. | Bridge feature sprint. |
| Reasoning graph / loops / `adl.skill.v1` | Skills, prompts, traces, ObsMem, and runtime loops must have a bridge map. | Bridge feature sprint. |
| Security, CAV, SSM, ACIP/A2A/protobuf | Activation cannot hide governance, protocol, or security residuals. | Security/protocol sprint. |
| Affect, happiness, Godel mechanics, economics, guilds | Public claims need safe boundaries and future governance routes. | Boundary/decision sprint. |
| CodeFriend, adapter v2, papers, and publication surfaces | Launch and birthday docs must not silently inherit product or publication commitments from later roadmap rows. | Boundary/decision and launch handoff sprint. |
| GitHub convergence/control-plane tooling | Sprint execution should know whether octocrab/tooling convergence is complete, blocked, deferred, or routed before relying on it. The v0.91.6 control-plane stream should consume GitHub/octocrab, watcher, lifecycle automation, logging, template-edge, FastContext, session-ledger, and operational-adoption defects; `#4443` is the v0.91.7-facing shepherd route if lifecycle orchestration remains incomplete. | v0.91.6 process/tooling bridge consumed by v0.91.7 planning. |
| Launch/birthday planning | July launch and v0.92 birthday must align without scope explosion. | Closeout/handoff sprint. |

## Explicit Non-Claims

- This ledger does not make local TBD drafts public proof.
- This ledger does not implement v0.91.7 work.
- This ledger does not approve v0.92 activation.
- This ledger does not require every cited future idea to become v0.91.7 implementation.
- This ledger does require every cited pre-v0.92 input to be complete, blocked, deferred, or routed before `v0.92` starts.
