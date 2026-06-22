# v0.91.6 Milestone README

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Owner: ADL maintainers
- Setup issue: `#3800`
- Source bridge ledger: `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`

## Status

Current status: the first pre-`v0.92` bridge tranche now includes multiple
completed or routed bridge waves, with WP-03 through WP-08 and WP-10 carrying
retained closeout truth while a smaller set of downstream runtime and
Observatory work remains open.

- Planning: created by `#3800`
- Documentation completion: `#3824`
- Issue wave: first-tranche bridge execution opened through WP-10, with
  retained closeout truth now available for WP-03 `#3968`, WP-04 `#3969`,
  WP-05 `#3970`, WP-06 `#3971`, WP-07 `#3972`, WP-08 `#3973`, and WP-10
  `#3975`
- Execution: WP-03 through WP-10 and ACIP runtime `#4160` now have bounded
  merged/closed bridge truth; Tokio integrated soak `#4185` and the ordered
  closeout-tail issue wave remain explicit downstream work rather than hidden
  blockers
- Validation: docs-readiness validation plus focused issue/PR and retained
  closeout proof for the completed bridge waves
- Release readiness: not applicable until `v0.91.6` executes

This package does not implement runtime features and does not claim `v0.92`
activation readiness. It exists so the first bridge tranche can open concrete
issues without reconstructing requirements from chat or local notes.

## Purpose

`v0.91.6` is the first required bridge/readiness tranche before `v0.92`.

Its job is to make the load-bearing pre-`v0.92` surfaces reviewable:

- resilience, citizen persistence, sleep/wake, and continuity proof
- logging/tooling proof-loop fixes and observability consumption
- public prompt records export, redaction, validation, and indexing
- provider/model reliability and multi-agent readiness
- first ACIP/A2A/provider-communications decisions
- first security bridge readiness and Continuous Adversarial Verification
- identity/continuity and capability-selector bridge accounting
- AEE completion, Memory/ObsMem handoff, and ACP/cognitive profile accounting
  so `v0.92` knows whether each surface is complete, deferred, blocked, or
  routed before activation
- strategic account and infrastructure setup planning that must be ready for
  `v0.91.6` execution, including `#3902` for the `agent-logic.ai` AWS account
- MVP route preservation for CodeFriend v1 / portable adapter v2 and guilds,
  so those later surfaces remain scheduled without expanding `v0.92`

`v0.91.6` should leave `v0.91.7` with explicit second-tranche work, not vague
spillover.

Closed umbrella truth in this milestone should not be read as proof that every
runtime-integrated downstream consumer is complete. Open runtime, demo-
convergence, and soak lanes remain explicit.

## Bridge Boundary

`v0.91.6` consumes the `#3778` bridge ledger. A surface may exit this milestone
only as one of:

- `complete`: reviewed feature doc and proof/review evidence exist
- `deferred`: explicitly not required for `v0.92`, with risk accepted
- `blocked`: named missing evidence or operator decision prevents completion
- `routed`: owned by a named follow-on issue/tranche with a clear exit condition

The milestone is successful only if `v0.92` can consume first-tranche bridge
truth without rediscovering the plan.

## First-Tranche Feature Docs

| Surface | Required v0.91.6 output | v0.92 consumption rule |
| --- | --- | --- |
| Resilience, citizen persistence, sleep/wake | Feature doc covering retry/fault classes, provider/tool/workflow resilience, health persistence, checkpoint/restore, sleep/wake, hibernation, simulation, in-transit custody, migration, replay, and continuity proof. | No half-work closure; each sub-surface must be complete, blocked, deferred, or routed. |
| Logging/tooling proof-loop fixes | Feature doc or bridge record covering validation architecture split, CI runtime-budget observability, logging/Otel consumption, and bounded PR proof-loop reliability. | Must improve bounded PR flow without weakening release confidence. |
| Public prompt records | Feature doc covering local editable authoring, public export, redaction, validation, indexing, evidence, and security review boundaries. | No publication claim without redaction/security/index posture. |
| Provider/model reliability | Feature doc covering hosted/local/remote/OpenRouter/Gemma expectations, role suitability, known failure modes, and multi-agent proof limits. | Reliability proof stays separate from training/product claims. |
| ACIP/A2A/provider communications | First decision record for schema catalog, access rules, external-agent posture, provider communications, WebSocket boundary, deterministic JSON projection, and protobuf decision point. | Residual protocol decisions may route to `v0.91.7`, but security and constructability boundaries must be visible. |
| Security bridge and CAV | Feature doc covering threat-model refresh, CAV, provider/model trust, public-record security, ACIP access/security, and adversarial/malformed-output expectations. | Security cannot be silently deferred out of the activation path. |
| Identity/continuity and capability selector | Bridge record connecting capability evidence, identity continuity, negative cases, and resilience/citizen persistence. | `v0.92` may consume evidence only; Aptitude Atlas construction remains post-MVP. |
| AEE completion, Memory/ObsMem, and ACP accounting | Bridge accounting record distinguishing AEE completion boundaries, residual runtime/provider action work, ObsMem handoff, Memory Palace planning, cognitive profile scope, privacy boundaries, and what `v0.92` may consume. | `v0.92` must not rediscover or overclaim these handoff surfaces. |

## Tooling Reliability Inputs

The first logging/tooling tranche should include the observed C-SDLC reliability
issues created during the feature-doc wave:

- `#3802`: parallel prompt-card validation hang diagnostics
- `#3803`: prompt-card enum diagnostics and lifecycle-state alignment
- `#3804`: lifecycle-card absolute-path leakage diagnostic precision
- `#3805`: octocrab token preflight diagnostics
- `#3935`: `SOR`-driven PR-body convergence and generalized card-to-GitHub
  projection policy

These are not blockers for creating this planning package. They are required
inputs to the `v0.91.6` tooling proof-loop work, and `#3935` is expected to
complete its first-tranche `SOR` convergence slice within this milestone.

## Companion Setup Inputs

`#3902` is the tracked route for creating and planning the `agent-logic.ai` AWS
account. It is not a `v0.92` activation surface and should not expand the
birthday milestone. The account setup is operationally complete and the
sanitized decision record is tracked at
[review/AGENT_LOGIC_AWS_ACCOUNT_DECISION_RECORD_3902.md](review/AGENT_LOGIC_AWS_ACCOUNT_DECISION_RECORD_3902.md).
AWS Activate review and private credit visibility remain post-close external
follow-up rather than blockers for `#3902` closeout.

CodeFriend v1 / portable adapter v2 and guilds are also companion planning
routes for v0.91.6. They are not first-tranche activation proof, but their
routes must remain visible so CodeFriend v1 can land after v0.92 and before
v0.95, and guilds can stay in MVP scope through the v0.93 governance feature
route and v0.95 MVP consumption.

## v0.91.7 Handoff

`v0.91.7` remains required for second-tranche bridge work unless `v0.91.6`
explicitly pulls work forward without weakening first-tranche outputs.

Expected `v0.91.7` residuals:

- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graph, loop runtime, and `adl.skill.v1`
- residual security readiness
- residual ACIP/A2A/protobuf/JSON projection decisions
- affect/happiness, Godel mechanics, and economics-context bridge accounting

## Source Map

- `#3778`: pre-`v0.92` bridge ledger and issue route
- `#3800`: this planning package
- `#3801`: `v0.91.7` second-tranche planning package
- `#3825`: `v0.91.7` planning and feature-doc completion package
- `#3780`: later `v0.92` activation and birthday refresh
- `#3779`: feature-doc production wave setup
- `#3802`-`#3805`: tooling reliability findings raised during this wave
- `#3935`: card/GitHub projection convergence for PR and lifecycle truth
- `#3902`: `agent-logic.ai` AWS account setup planning
- `docs/milestones/v0.93/features/GUILDS_AND_COLLECTIVE_ORGANIZATION_v0.93.md`
- `docs/milestones/v0.95/features/CODEFRIEND_V1_PORTABLE_ADAPTER_V2_PROOF_v0.95.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/FEATURE_DOC_PRODUCTION_MINI_SPRINT_v0.91.5.md`
- `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`
- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`

## Document Map

- Work breakdown: [WBS_v0.91.6.md](WBS_v0.91.6.md)
- Vision: [VISION_v0.91.6.md](VISION_v0.91.6.md)
- Design: [DESIGN_v0.91.6.md](DESIGN_v0.91.6.md)
- Decisions: [DECISIONS_v0.91.6.md](DECISIONS_v0.91.6.md)
- Sprint plan: [SPRINT_PLAN_v0.91.6.md](SPRINT_PLAN_v0.91.6.md)
- Demo matrix: [DEMO_MATRIX_v0.91.6.md](DEMO_MATRIX_v0.91.6.md)
- Feature-doc index: [FEATURE_DOCS_v0.91.6.md](FEATURE_DOCS_v0.91.6.md)
- Candidate issue wave: [WP_ISSUE_WAVE_v0.91.6.yaml](WP_ISSUE_WAVE_v0.91.6.yaml)
- Checklist: [MILESTONE_CHECKLIST_v0.91.6.md](MILESTONE_CHECKLIST_v0.91.6.md)
- Release plan: [RELEASE_PLAN_v0.91.6.md](RELEASE_PLAN_v0.91.6.md)
- Release notes: [RELEASE_NOTES_v0.91.6.md](RELEASE_NOTES_v0.91.6.md)
- Review and validation checklist:
  [REVIEW_AND_VALIDATION_CHECKLIST_v0.91.6.md](REVIEW_AND_VALIDATION_CHECKLIST_v0.91.6.md)
- Feature directory index: [features/README.md](features/README.md)
- Review packet:
  [review/CSDLC_GITHUB_PROJECTION_CONVERGENCE_REVIEW_3935.md](review/CSDLC_GITHUB_PROJECTION_CONVERGENCE_REVIEW_3935.md)
- Runtime AWS signal bridge design:
  [review/runtime_aws_signal_bridge/RUNTIME_AWS_SIGNAL_BRIDGE_DESIGN_4294.md](review/runtime_aws_signal_bridge/RUNTIME_AWS_SIGNAL_BRIDGE_DESIGN_4294.md)
- Runtime AWS heartbeat publisher proof:
  [review/runtime_aws_signal_bridge/RUNTIME_AWS_HEARTBEAT_PUBLISHER_PROOF_4295.md](review/runtime_aws_signal_bridge/RUNTIME_AWS_HEARTBEAT_PUBLISHER_PROOF_4295.md)
- Runtime ACIP-to-SNS bridge proof:
  [review/runtime_aws_signal_bridge/RUNTIME_ACIP_SNS_BRIDGE_PROOF_4296.md](review/runtime_aws_signal_bridge/RUNTIME_ACIP_SNS_BRIDGE_PROOF_4296.md)
- Runtime AWS signal bridge mini-sprint closeout:
  [review/runtime_aws_signal_bridge/RUNTIME_AWS_SIGNAL_BRIDGE_MINI_SPRINT_CLOSEOUT_4325.md](review/runtime_aws_signal_bridge/RUNTIME_AWS_SIGNAL_BRIDGE_MINI_SPRINT_CLOSEOUT_4325.md)
- Validation-tail A/B index:
  [review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md](review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md)

## Non-Goals

- Do not implement runtime features in the planning package.
- Do not claim `v0.92` activation readiness.
- Do not close or supersede `#3801`.
- Do not move or delete local planning files.
- Do not collapse security, ACIP/A2A, resilience, or tooling reliability into
  generic follow-up language.

## Exit Criteria

- Every first-tranche surface has a feature-doc route, issue-wave route, and
  review gate.
- `v0.91.7` residuals remain explicit.
- `v0.92` activation remains blocked until the bridge ledger can say each
  activation surface is complete, deferred, blocked, or routed with evidence.

## Closeout-tail sprint

The standard milestone closeout tail for `v0.91.6` is tracked in [CLOSEOUT_TAIL_SPRINT_v0.91.6.md](CLOSEOUT_TAIL_SPRINT_v0.91.6.md). Use that file as the canonical ordered sprint surface for demo convergence, quality gate, review, remediation/preflight, next-milestone planning, and release ceremony sequencing.

Document-map addendum: [CLOSEOUT_TAIL_SPRINT_v0.91.6.md](CLOSEOUT_TAIL_SPRINT_v0.91.6.md) is part of the milestone planning and closeout packet set and should be read alongside the sprint plan, release plan, and review surfaces.
