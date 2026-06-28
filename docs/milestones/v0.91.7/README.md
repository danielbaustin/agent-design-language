# v0.91.7 Milestone README

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Owner: ADL maintainers
- Setup lineage: `#3801`, `#3825`, `#4368`
- Source bridge ledger: `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`
- First-tranche input: `docs/milestones/v0.91.6/`
- Source-capture ledger: `PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- v0.91.6 handoff addendum: `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`

## Status

Current status: planning refresh for the final pre-`v0.92` bridge and readiness tranche.

- Initial planning package: created by earlier v0.91.7 setup work.
- Source-capture refresh: `#4368`.
- v0.91.6 release-tail handoff refresh: `#3982`.
- Execution: not started beyond planning/feature docs and `#4549` Soak #2
  execution-packet setup; no Soak #2 runtime execution has run.
- Validation: docs-readiness validation only until implementation issues run.
- Release readiness: not applicable until `v0.91.7` executes.
- Dependency gate: v0.91.7 execution must consume the failed-but-closed WP-15
  `#3980` external-review truth, closed WP-16 `#3981`
  remediation/final-preflight truth, closed WP-16 children `#4620` and
  `#4621`, the open v0.91.7 tooling route `#4622`, and closed WP-14A
  remediation truth before opening dependent execution work.

This package does not implement runtime features and does not claim `v0.92` activation readiness. It exists so every required pre-birthday surface is visible, scheduled, deferred, blocked, or routed before `v0.92` opens.

## Purpose

`v0.91.7` is the final bridge/readiness tranche before `v0.92`.

It must convert the remaining major pre-birthday surfaces into reviewable issue routes, sprint structure, feature docs, and handoff truth:

- v0.91.6 closeout truth, ADR release-tail decisions, and release-tail cleanup;
- the v0.91.6 C-SDLC integration/control-plane completion stream: `#4388`-`#4398`, session coordination `#4405`, session-ledger and lifecycle liveness `#4412`-`#4413`, validation-throughput/lifecycle automation and generated VPP inputs `#4417`-`#4421` plus `#4425`, forward metric capture `#4431`, bounded v0.91.6 metric backfill `#4441`, closed operational-adoption sprint `#4433`-`#4438`, closed release/docs follow-ons `#4520`-`#4522`, and any surviving v0.91.7-facing goal snapshot/lifecycle shepherd work `#4442`-`#4443`;
- goal state, nested goals, per-issue time/token/resource metrics, and predictable execution baselines;
- cognitive scheduler, cognitive economics, provider suitability, and local-agent acceleration;
- build throughput, validation manager, remote/local build runners, and CI/test-tax reduction;
- runtime fire-up, Soak #2, runtime heartbeat/AWS/ACIP signal bridge, and runtime minimal-prototype reconciliation;
- Observatory/Unity/demo readiness;
- Curiosity Engine, Constructability Gate, reasoning graph, loop runtime, and `adl.skill.v1` bridge;
- residual security/CAV/SSM and ACIP/A2A/protobuf decisions;
- affect/happiness, Godel mechanics, economics-context, and guild/civilization boundaries;
- launch/birthday planning and `v0.92` handoff.

`v0.91.7` is not vague spillover. It is the final place to make the work to reach `v0.92` explicit before the first-birthday milestone begins.

## Bridge Boundary

`v0.91.7` consumes:

- the `#3778` pre-`v0.92` bridge ledger;
- the `#3800` / `v0.91.6` first-tranche planning and evidence package;
- residuals explicitly left by `v0.91.6` sprint reviews, late control-plane issues, and closeout;
- local backlog routing from `.adl/docs/TBD/LOCAL_BACKLOG.md`;
- local TBD source material captured in `PLANNING_SOURCE_CAPTURE_v0.91.7.md`.

Every surface must exit as one of:

- `complete`: reviewed doc, issue, proof, or implementation evidence exists;
- `deferred`: explicitly not required for `v0.92`, with risk accepted;
- `blocked`: named missing evidence or operator decision prevents completion;
- `routed`: owned by a named follow-on issue, sprint, or milestone with a clear exit condition.

## Required Work Streams

| Work stream | Required output before v0.92 |
| --- | --- |
| Closeout truth | v0.91.6 release-tail and ADR issues closed or routed, with v0.91.7 not inheriting stale truth. |
| C-SDLC integration control plane | v0.91.6 `#4388`-`#4398` plus late `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, closed adoption sprint `#4433`-`#4438`, closed release/docs follow-ons `#4520`-`#4522`, and any remaining `#4442` / `#4443` carryforward are consumed, blocked, deferred, or routed before v0.91.7 depends on them. |
| Goal and metrics | Goal state, nested goals, SOR time/token/resource fields, forward metric capture `#4431`, bounded backfill `#4441`, and host snapshot capture `#4442` are scheduled or routed. |
| Scheduler and providers | Cognitive scheduler, provider profiles, local/hosted model suitability, and local-agent delegation routes scheduled. |
| Build and validation throughput | Validation manager, path ownership, SOR fact capture, VPP generation, long-test fanout, CI log archive/S3, Nessus/CodeBuild, sccache/linker/target-dir work scheduled. |
| Runtime | Runtime Soak #2/fire-up, runtime heartbeat/AWS signal bridge, ACIP-to-SNS, and minimal prototype reconciliation routed. |
| Observatory and demos | Unity/HTML Observatory and flagship demo readiness routed with proof expectations. |
| Conceptual bridge docs | Curiosity, Constructability, reasoning graph/loop/skill standard, affect/happiness, Godel, economics, and guilds bounded. |
| Security and protocol | Security/CAV/SSM and ACIP/A2A/protobuf residuals complete, blocked, deferred, or routed. |
| Launch and birthday handoff | July launch planning and `v0.92` activation handoff aligned without absorbing birthday implementation. |

## Document Map

- Source capture: [PLANNING_SOURCE_CAPTURE_v0.91.7.md](PLANNING_SOURCE_CAPTURE_v0.91.7.md)
- Work breakdown: [WBS_v0.91.7.md](WBS_v0.91.7.md)
- Vision: [VISION_v0.91.7.md](VISION_v0.91.7.md)
- Design: [DESIGN_v0.91.7.md](DESIGN_v0.91.7.md)
- Decisions: [DECISIONS_v0.91.7.md](DECISIONS_v0.91.7.md)
- Sprint plan: [SPRINT_PLAN_v0.91.7.md](SPRINT_PLAN_v0.91.7.md)
- Runtime Soak #2 packet: [RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md](RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md)
- v0.91.6-to-v0.91.7 handoff addendum: [V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md](V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md)
- Demo matrix: [DEMO_MATRIX_v0.91.7.md](DEMO_MATRIX_v0.91.7.md)
- Feature-doc index: [FEATURE_DOCS_v0.91.7.md](FEATURE_DOCS_v0.91.7.md)
- v0.92 handoff: [V092_HANDOFF_v0.91.7.md](V092_HANDOFF_v0.91.7.md)
- Candidate issue wave: [WP_ISSUE_WAVE_v0.91.7.yaml](WP_ISSUE_WAVE_v0.91.7.yaml)
- Checklist: [MILESTONE_CHECKLIST_v0.91.7.md](MILESTONE_CHECKLIST_v0.91.7.md)
- Release plan: [RELEASE_PLAN_v0.91.7.md](RELEASE_PLAN_v0.91.7.md)
- Release notes: [RELEASE_NOTES_v0.91.7.md](RELEASE_NOTES_v0.91.7.md)
- Review and validation checklist: [REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md](REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md)
- Feature directory index: [features/README.md](features/README.md)

## Non-Goals

- Do not implement `v0.92` birthday work in `v0.91.7`.
- Do not claim `v0.92` activation readiness from planning docs alone.
- Do not move every long-term ADL idea into `v0.91.7`.
- Do not make launch, product, or governance ambition silently expand the milestone.
- Do not move or delete ignored local TBD files in this planning package.

## Exit Criteria

- Every required source in `PLANNING_SOURCE_CAPTURE_v0.91.7.md` is complete, blocked, deferred, or routed.
- WP-01 consumes `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md` plus failed-but-closed WP-15 truth, final WP-16 closeout truth, closed `#4620` / `#4621`, and the v0.91.7-routed `#4622`, before starting dependent execution work.
- Every v0.91.6 carryover issue has a truthful disposition before `v0.92` opens.
- `#3780` can refresh `v0.92` activation docs from tracked bridge truth without reconstructing scope from chat.
- The first-birthday milestone starts with a clear runtime/demo/security/protocol/process substrate and known residual risks.
