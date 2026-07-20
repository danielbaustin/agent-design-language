# v0.91.7 Milestone README

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Created: `2026-06-21`
- Last verified: `2026-07-19`
- Owner: ADL maintainers
- Setup lineage: `#3801`, `#3825`, `#4368`
- Source ledger: `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`
- First-tranche input: `docs/milestones/v0.91.6/`
- Source-capture ledger: `PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- v0.91.6 handoff addendum: `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`
- Required successor bridge: [`../v0.91.8/README.md`](../v0.91.8/README.md)
- Required reviewed handoff: [`../v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md`](../v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md)

## Status

Current status: closeout tail active after the implementation and demo waves.

Live issue truth was last verified on 2026-07-19. WP-01 through WP-18 are
closed. WP-19 is closed with a degraded-provider review that returned 22
findings.
WP-20 and WP-23 remain open, while WP-21, WP-21A, and WP-22 are closed
retained planning evidence. Closed issue state does not by itself
mean review-clean or release-ready; the sprint-review register and issue-local
proof packets retain findings and non-claims.

Issues #5572 / PR #5574 and #5575 are v0.91.8 follow-ons and do not block
v0.91.7. Closeout audit #5573 is closed; merged PR #5578 retains Markdown and
JSON registers classifying all 427 issues that were closed when the audit ran.
No second milestone audit is required or claimed. WP-19 completed its
replacement review against the frozen exact-revision corpus.

| Closeout gate | Issue | Live state | Consumption boundary |
| --- | ---: | --- | --- |
| WP-14 launch/birthday handoff | #4641 | closed | Routing and claim-boundary evidence only. |
| WP-15 demo convergence | #4642 | closed | Demo/proof coverage, not release approval. |
| WP-16 quality gate | #4643 | closed | Passed with downstream gates open; retained packet is `review/V0917_WP16_QUALITY_GATE_4643.md`. |
| WP-17 documentation alignment | #4644 | closed | Merged by PR #5539; alignment packet is under `review/`. |
| WP-18 internal review | #4645 | closed | Merged PR #5543; internal findings were routed through closed #5408 and #5544-#5547. |
| WP-19 external review | #4646 | closed; provider-degraded review complete | The exact 70-file corpus received one Fable 5 lane and three independent shadow lanes; 22 findings are retained. |
| WP-20 remediation/preflight | #4647 | open | Owns synthesis and remediation of the 22 WP-19 findings. |
| WP-21 next-milestone planning | #4648 | closed | Retained planning evidence; current v0.91.8 authority supersedes direct activation use. |
| WP-21A next-milestone closeout planning | #5489 | closed | Retains the canonical v0.91.8 planning and external-review handoff package. |
| WP-22 next-milestone review | #4649 | closed | Retained review evidence, not release approval. |
| WP-23 release ceremony | #4650 | open | Final gate after required review/remediation truth. |

- Initial planning package: created by earlier v0.91.7 setup work.
- Source-capture refresh: `#4368`.
- v0.91.6 release-tail handoff refresh: `#3982`.
- WP-02 v0.91.6 closeout-truth consumption: `#4661` /
  `docs/milestones/v0.91.7/review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md`.
- WP-01 planning promotion: `#4628`.
- WP-06 build-throughput sprint lane: `#4633` /
  `review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md`.
  The selected child lane and remote-builder follow-ups are merged and
  reconciled: `#4676`, `#4800`, `#4698`, `#4726`, `#4677`, `#4678`, `#4679`,
  `#4680`, `#4837`, `#4838`, `#4879`, `#4928`, and `#4945` record the current
  WP-06 implementation, operationalization, and closeout-truth path. `#4952`
  is the final records/handoff repair that closes the remaining milestone-truth
  drift before WP-08 execution starts.
  Residual `pr watch` `closeout_needed` ambiguity for already-closeouted issues
  is routed to tooling bug `#4950`, not treated as remaining WP-06 work.
- Canonical WP issue wave: `#4628` through `#4650`.
- Existing assigned v0.91.7 issues:
  - `#4603` routes into WP-06 build throughput / remote validation.
  - `#4617` routes into WP-04 goal metrics / telemetry.
  - `#4622` delivered the WP-02 release-tail PR inventory path through
    `bash adl/tools/pr.sh pr-inventory --json`.
- Non-WP v0.91.7 issues:
  - `#4651` covers sensible Rust refactoring by ownership and validation cost.
    It is now split into executable child issues `#4892`-`#4900` for enum
    typing, `strum`/third-party-library simplification, tracing-backed
    observability, provider HTTP mechanics, secret/digest hygiene, canonical
    JSON signing, and ACIP runtime streaming substrate proof.
  - `#4652` covers Unity demo surfaces.
  - `#4653` covers dspark speculative decoding evaluation with Qwen and Gemma.
  - `#4654` covers deepseek-v4-flash-dspark smoke testing on ephemeral 2xH100 EC2 with teardown/cost proof.
- Execution: WP-01 through WP-19 are complete with issue-local evidence and
  retained limitations. The remaining active work is WP-20
  remediation/preflight and WP-23 release ceremony.
  This package is a living
  milestone surface, not the original pre-execution planning snapshot.
- Validation: validation truth is issue-local; completed child issues retain
  their focused proof surfaces, while remaining open issues must provide their
  own validation before closeout.
- Release readiness: not claimed until all required v0.91.7 execution,
  validation, review, and closeout surfaces converge.
- Dependency gate: v0.91.7 execution must consume the failed-but-closed WP-15
  `#3980` external-review truth, closed WP-16 `#3981`
  remediation/final-preflight truth, closed WP-16 children `#4620` and
  `#4621`, the WP-02 PR inventory command delivered by `#4622`, and closed
  WP-14A remediation truth before opening dependent execution work.

This package does not by itself claim `v0.92` activation readiness. Its reviewed
outputs feed the required [v0.91.8 bridge](../v0.91.8/README.md), and `v0.92`
may consume only the reviewed v0.91.8 exact-revision handoff. Every required
surface must still exit as integrated/proven, already closed with evidence,
operator-scoped-out with evidence and approval, or blocked with evidence and
operator approval.

## Purpose

`v0.91.7` is the implementation/readiness tranche that feeds the required
`v0.91.8` bridge before `v0.92`.

It must convert the remaining major pre-birthday surfaces into reviewable issue work, sprint structure, feature docs, and handoff truth:

- v0.91.6 closeout truth, ADR release-tail decisions, and release-tail cleanup;
- the v0.91.6 C-SDLC integration/control-plane completion stream: `#4388`-`#4398`, session coordination `#4405`, session-ledger and lifecycle liveness `#4412`-`#4413`, validation-throughput/lifecycle automation and generated VPP inputs `#4417`-`#4421` plus `#4425`, forward metric capture `#4431`, bounded v0.91.6 metric backfill `#4441`, closed operational-adoption sprint `#4433`-`#4438`, closed release/docs follow-ons `#4520`-`#4522`, and any surviving v0.91.7-facing goal snapshot/lifecycle shepherd work `#4442`-`#4443`;
- goal state, nested goals, per-issue time/token/resource metrics, and predictable execution baselines;
- cognitive scheduler, cognitive economics, provider suitability, and local-agent acceleration;
- build throughput, validation manager, remote/local build runners, and CI/test-tax reduction;
- runtime fire-up, Soak #2, runtime heartbeat/AWS/ACIP signal integration, and runtime minimal-prototype reconciliation;
- Observatory/Unity/demo readiness;
- Curiosity Engine, Constructability Gate, reasoning graph, loop runtime, and `adl.skill.v1`;
- security/CAV/SSM and ACIP/A2A/protobuf implementation decisions;
- affect/happiness, Godel mechanics, economics-context, and guild/civilization boundaries;
- launch/birthday planning and `v0.92` handoff.

`v0.91.7` is not vague spillover. It makes its implementation and evidence
truth explicit for reviewed v0.91.8 platform acceptance and handoff before the
first-birthday milestone begins.

## Activation Boundary

`v0.91.7` consumes:

- the `#3778` pre-`v0.92` source ledger;
- the `#3800` / `v0.91.6` first-tranche planning and evidence package;
- blockers explicitly left by `v0.91.6` sprint reviews, late control-plane issues, and closeout;
- local backlog ownership from `.adl/docs/TBD/LOCAL_BACKLOG.md`;
- local TBD source material captured in `PLANNING_SOURCE_CAPTURE_v0.91.7.md`.

Every activation-path surface must exit as one of:

- `integrated_proven`: implementation runs in the integrated path with retained evidence;
- `already_closed_with_evidence`: the source issue is closed and its retained evidence is current;
- `operator_scoped_out`: explicitly not required for `v0.92`, with evidence, risk, and operator approval recorded;
- `blocked_with_evidence`: named missing evidence or operator decision prevents completion.

Assignment to a follow-on issue, sprint, or milestone is scheduling truth only. It does not count as completion.

## Required Work Streams

| Work stream | Required output before v0.92 |
| --- | --- |
| Closeout truth | v0.91.6 release-tail and ADR issues closed with evidence or blocked with evidence and operator approval, with v0.91.7 not inheriting stale truth. |
| C-SDLC integration control plane | v0.91.6 `#4388`-`#4398` plus late `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, closed adoption sprint `#4433`-`#4438`, closed release/docs follow-ons `#4520`-`#4522`, and any remaining `#4442` / `#4443` carryforward are consumed as integrated/proven, already closed with evidence, operator-scoped-out with evidence and operator approval, or blocked with evidence and operator approval before v0.91.7 depends on them. |
| Goal and metrics | Goal state, nested goals, SOR time/token/resource fields, forward metric capture `#4431`, bounded backfill `#4441`, and host snapshot capture `#4442` are implemented/proven or blocked with evidence and operator approval. |
| Scheduler and providers | Cognitive scheduler, provider profiles, local/hosted model suitability, and local-agent delegation are implemented/proven or blocked with evidence and operator approval. |
| Build and validation throughput | Validation manager, path ownership, SOR fact capture, VPP generation, long-test fanout, CI log archive/S3, Nessus/CodeBuild, sccache/linker/target-dir work are implemented/proven where v0.92 depends on them or blocked with evidence and operator approval. |
| Runtime | Runtime Soak #2/fire-up, runtime heartbeat/AWS signal integration, ACIP-to-SNS, and minimal prototype reconciliation are integrated/proven or blocked with evidence and operator approval. |
| Observatory and demos | Unity/HTML Observatory and flagship demo readiness are proven with retained evidence, public-claim-bounded/operator-scoped-out with evidence and operator approval, or blocked with evidence and operator approval. |
| Cognitive/protocol implementation surfaces | Curiosity, Constructability, reasoning graph/loop/skill standard, affect/happiness, Godel, economics, and guilds are implemented/proven or blocked with evidence and operator approval. |
| Security and protocol | Security/CAV/SSM and ACIP/A2A/protobuf requirements are resolved or blocked with evidence and operator approval. |
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
- Opened issue wave: [WP_ISSUE_WAVE_v0.91.7.yaml](WP_ISSUE_WAVE_v0.91.7.yaml)
- Checklist: [MILESTONE_CHECKLIST_v0.91.7.md](MILESTONE_CHECKLIST_v0.91.7.md)
- Release plan: [RELEASE_PLAN_v0.91.7.md](RELEASE_PLAN_v0.91.7.md)
- Release notes: [RELEASE_NOTES_v0.91.7.md](RELEASE_NOTES_v0.91.7.md)
- Review and validation checklist: [REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md](REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md)
- Feature directory index: [features/README.md](features/README.md)
- WP-02 closeout-truth consumption packet:
  [docs/milestones/v0.91.7/review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md](review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md)
- WP-06 build-throughput sprint packet:
  [docs/milestones/v0.91.7/review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md](review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md)
- WP-12 access and activation gate:
  [docs/milestones/v0.91.7/review/security/WP12_ACCESS_ACTIVATION_GATE_4660.md](review/security/WP12_ACCESS_ACTIVATION_GATE_4660.md)

## Non-Goals

- Do not implement `v0.92` birthday work in `v0.91.7`.
- Do not claim `v0.92` activation readiness from planning docs alone.
- Do not move every long-term ADL idea into `v0.91.7`.
- Do not make launch, product, or governance ambition silently expand the milestone.
- Do not move or delete ignored local TBD files in this planning package.

## Exit Criteria

- Every required source in `PLANNING_SOURCE_CAPTURE_v0.91.7.md` is integrated/proven, already closed with evidence, operator-scoped-out with evidence and operator approval, or blocked with evidence and operator approval.
- WP-01 consumes `V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md` plus failed-but-closed WP-15 truth, final WP-16 closeout truth, closed `#4620` / `#4621`, and closed `#4622` PR-inventory proof, before starting dependent execution work.
- Every v0.91.6 carryover issue has a truthful disposition before `v0.92` opens.
- `#3780` can refresh `v0.92` activation docs from tracked implementation/proof truth without reconstructing scope from chat.
- The first-birthday milestone starts with a clear runtime/demo/security/protocol/process substrate and known evidence-backed blockers.
