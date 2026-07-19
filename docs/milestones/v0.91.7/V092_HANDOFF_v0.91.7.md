# v0.91.7 to v0.92 Handoff

## Metadata

- Source milestone: `v0.91.7`
- Target milestone: `v0.92`
- Version: `v0.91.7`
- Created: `2026-06-21`
- Last verified: `2026-07-18`
- Owner: ADL maintainers
- Related issues: `#3825`, `#4368`, `#3982`, `#3780`, `#5383`, `#5384`

## Purpose

Record the second-tranche implementation/proof surfaces that `#3780` / `v0.92` may consume
after `v0.91.7` docs-package completion. This is a handoff record, not runtime
implementation proof.

Update for `#5383`: `v0.91.8` is now the planned bridge prerequisite between
this handoff and `v0.92`. `v0.92` should not consume `v0.91.7` launch/birthday
handoff rows directly as activation approval. It must first consume the
reviewed `v0.91.8` exact-revision handoff, including ADL v2, Runtime v3,
C-SDLC v2, selector/rollback, WP-14A child disposition, and explicit non-claim
truth.

Before `v0.92` consumes this handoff, `v0.91.7` WP-01 must consume
`V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md`, the failed-but-closed v0.91.6
WP-15 external-review truth, closed WP-16 remediation/preflight truth, closed
`#4620` / `#4621`, and closed v0.91.7 PR-inventory fix `#4622`, plus closed WP-14A `#4582`
and `#4609`-`#4612` remediation truth. If those inputs are missing, this
handoff remains planning-only.

## Handoff Rule

`v0.92` may consume an activation-path surface only as one of:

- `integrated_proven`: implementation runs in the integrated path with retained evidence;
- `operator_scoped_out`: implementation proof is explicitly outside `v0.92` activation scope, with evidence, risk, and operator approval recorded;
- `blocked_with_evidence`: named missing evidence or decision prevents activation use;
- `implementation_required`: owned by a named v0.91.7/v0.92 issue, but not consumable until integrated proof exists.

For product/runtime surfaces inherited from `v0.91.6`, `doc-ready` is not
runtime completion. Those surfaces require an explicit completion class from
[`../v0.91.6/OPERATIONAL_COMPLETION_GATE_v0.91.6.md`](../v0.91.6/OPERATIONAL_COMPLETION_GATE_v0.91.6.md),
and only `integrated_proven` counts as operational completion.

## v0.91.7 Closeout-Tail Truth

As last verified on 2026-07-18, WP-01 through WP-17 are closed; WP-17 closed
through #4644 and merged PR #5539. WP-18, WP-19, WP-20, and WP-23 are open;
WP-21 and WP-22 are closed retained planning/review evidence. This handoff is
therefore not release or activation approval. Its current role is to route
bounded evidence into the reviewed v0.91.8 bridge.

## v0.91.8 Bridge Routing

| Bridge item | Required v0.91.8 source | v0.92 consumption rule |
| --- | --- | --- |
| Active v0.91.8 setup package | `#5383`, `docs/milestones/v0.91.8/` | Consume as planning source only until merged and reviewed. |
| Integrated platform acceptance | `#5384` / WP-14A | Required before birthday-facing platform readiness claims. |
| ADL v2 acceptance | `#5336`-`#5350`, `#5343`, `#5344` | Required before ADL v2 language/compiler/CLI claims. |
| Runtime v3 acceptance | `#5341`, `#5361` | Required before runtime execution claims. |
| C-SDLC v2 acceptance | `#5358` | Required before lifecycle-governance claims. |
| v0.92 handoff | `#5352`, `#5362`, `NEXT_MILESTONE_HANDOFF_v0.91.8.md` | Required before opening birthday implementation issues. |

## WP-14 Launch / Birthday Handoff Refresh

`#4641` / WP-14 now exits as `routed_with_evidence` through
`review/V0917_WP14_LAUNCH_BIRTHDAY_HANDOFF_4641.md` and the machine-readable
ledger at `review/wp14_launch_birthday_4641/ledger.yaml`.

This refresh does not make `v0.92` activation-ready. It records that the
launch, activation, Memory Palace, capability envelope, witness/receipt, and
birthday-doc surfaces are owned by the open v0.91.8 WP-14 child issues
`#4758`-`#4763`. `v0.92` may consume this WP-14 packet only as routing,
blocker, and public-claim-boundary truth until those child issues close with
integrated evidence and review.

## Surface Dispositions

| Surface | Handoff state | v0.92 consumption limit | Source doc |
| --- | --- | --- | --- |
| Curiosity Engine / Discovery Substrate | integrated_proven for the bounded Runtime v2 cycle | `v0.92` may consume only the governed discovery-cycle behavior and non-claims retained by closed #4692 and the WP-10 review packet. | `features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md`; `review/V0917_WP10_CURIOSITY_CONSTRUCTABILITY_REVIEW_4637.md` |
| Constructability Gate | integrated_proven for the bounded Runtime v2 validator | `v0.92` may consume only the construction-event, anchor, validator, and fail-closed proof retained by closed #4693. | `features/CONSTRUCTABILITY_GATE_v0.91.7.md`; `review/V0917_WP10_CURIOSITY_CONSTRUCTABILITY_REVIEW_4637.md` |
| Reasoning graph / loop runtime / `adl.skill.v1` | integrated_proven for bounded WP-11 slices | Closed #4694-#4697 and follow-ons retain producer/consumer and runtime proof. Full adaptive-learning or final-standard convergence is not claimed. | `features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md`; `review/V0917_WP11_RUNTIME_V2_COGNITIVE_CONTROL_EVIDENCE_4638.md` |
| Security readiness | bounded proof and remediation retained | Closed #4656-#4660 and #5404/#5406 support only their retained CAV, SSM, access, protocol, and validation claims. Broader release security remains review-gated. | `features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`; `review/security/WP12_ACCESS_ACTIVATION_GATE_4660.md`; `.csdlc/issues/5404/` |
| ACIP/A2A/protobuf implementation | bounded projection, WebSocket, and access proof retained | `v0.92` may consume the retained schema/projection, loopback WebSocket, and access-rule evidence only; broader federation or production transport readiness is not claimed. | `features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md`; `review/runtime/wp12_acip_websocket_transport_4659/README.md` |
| Affect and happiness | integrated_proven for operational reasoning-control; subjective affect not_claimed | `v0.92` may consume the #4752 safe-test boundary and existing `affect_reasoning_control_packet.v1` proof only as operational reasoning-control evidence. Birthday, launch, demo, or publication copy must not imply hidden emotion, subjective happiness, wellbeing, suffering, consciousness, scalar happiness scores, reward channels, or public reputation. | `features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md`; `review/wp13_affect_happiness_boundary_4752.md` |
| Godel mechanics | boundary_proven for CSM-supervised admission readiness and claim-boundary consumption; provider requests remain resolved_not_invoked and adaptive DAG completion not_claimed | `v0.92` may consume the Runtime v2 Godel/constructability boundary for reviewed birthday claims only when retained Godel plan evidence, non-invoked provider-request admission, and constructability anchors are cited. | `features/GODEL_MECHANICS_BRIDGE_v0.91.7.md`; `review/wp13_godel_constructability_boundary_4753.md` |
| Economics context | operator_scoped_out unless reopened | Default `v0.92` posture is context-only unless an explicit operator decision promotes and proves a bounded test. | `features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md` |
| Guild foundation | boundary_proven for declarative governance handoff context; guild record and hook producer/consumer behavior and v0.93 governance not_claimed | `v0.92` may consume only the Runtime v2 guild foundation vocabulary, allowlists, deferrals, and promotion gates as birthday governance context. It must not imply implemented identity/witness routing, membership events, moderation hooks, constitutional citizenship, polis authority, delegated governance authority, binding collective decision-making, public guild product readiness, or completed governance. | `features/GUILD_FOUNDATION_BOUNDARY_v0.91.7.md`; `review/wp13_guild_foundation_boundary_4755.md` |
| Paper and publication surfaces | scoped_out_for `v0.92` birthday activation by tracked #4757 boundary; external publication not approved | `v0.92` may consume the #4757 publication boundary only as non-claim and promotion-gate truth. It must not infer a published paper, public launch approval, customer-facing CodeFriend/report readiness, autonomous review authority, subjective affect/consciousness claims, economic/governance authority, v0.93 governance completion, or v0.95 CodeFriend MVP completion. | `review/wp13_publication_boundary_4757.md`; `review/wp13_publication_boundary_4757/boundary_packet.json` |
| WP-13 parent closeout | reconciled for `#4640` | `v0.92` may consume WP-13 only through the child packet limits summarized in the parent closeout. The closeout does not strengthen any claim beyond the child evidence and does not approve public launch, paper publication, CodeFriend product readiness, subjective affect, economic authority, or completed governance. | `review/wp13_closeout_4640.md`; `review/wp13_closeout_4640/closeout_packet.json` |

## Operational Substrate Handoff

`v0.92` should also consume the operational substrate dispositions from `PLANNING_SOURCE_CAPTURE_v0.91.7.md`:

| Surface | Required state before v0.92 |
| --- | --- |
| Product and runtime completion truth | `v0.91.6` product/runtime surfaces must carry an explicit operational completion class. `doc-ready`, `seam_ready`, `mock_proven`, `component_proven`, `local_slice_proven`, and `demo_scaffold` remain prerequisite evidence only; `v0.92` may treat a surface as operationally complete only when `integrated_proven` evidence is recorded. |
| C-SDLC integration control plane | v0.91.6 `#4388`-`#4398`, `#4405`, `#4412`-`#4413`, `#4417`-`#4421` plus `#4425`, `#4431`, `#4441`, closed adoption sprint `#4433`-`#4438`, closed release/docs follow-ons `#4520`-`#4522`, and any remaining `#4442` / `#4443` carryforward / SEP / VPP / PVF / templates / GitHub-octocrab convergence / session ledger / logging / watcher-lifecycle automation / operational adoption / shepherding / FastContext work are consumed as integrated/proven, already closed with evidence, or blocked with evidence and operator approval before v0.92 relies on them. |
| Goal and metrics accounting | Time/token/resource, nested-goal, forward capture `#4431`, v0.91.6-only backfill `#4441`, and host goal snapshot `#4442` are implemented/proven or blocked with evidence and operator approval before v0.92 issue planning relies on them. |
| Scheduler/provider/local agents | Routing policy and suitability path implemented/proven enough to protect premium cognition, or blocked with evidence and operator approval. |
| Capability envelope and capability testing | Memory grounding, capability envelope, birth witnesses/receipt, and Aptitude Atlas boundaries explicitly proven, operator-scoped-out with evidence, or blocked with evidence before birthday evidence relies on them. |
| Build/validation throughput | Validation-cost, path ownership, SOR fact capture, validation manager, VPP generation, and remote/local build paths implemented/proven enough to avoid rediscovery during birthday work. EC2 Spot or an alternate disposable remote-builder path must have time/cost/cache/cleanup evidence before it is treated as a release-critical lane. |
| GitHub convergence and control-plane tooling | GitHub/octocrab/tooling convergence, session coordination, lifecycle liveness, and shepherd state explicit enough that v0.92 sprint execution does not depend on ambiguous `gh` fallback, stale control-plane assumptions, or chat-only session memory. |
| Integrated logging, OTel boundary, and Observatory consumption | Closed `#4718` retains bounded integrated runtime/provider/control-plane event, stdout/stderr, redaction/path-hygiene, OTel-boundary, and consumer proof. `v0.92` may consume only those recorded surfaces; broad production telemetry or Unity readiness is not inferred. |
| Runtime integration, Soak #2, and signal integration | Closed `#4682` and `RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md` retain the reviewed Soak #2 result. `v0.92` may consume only rows classified by that evidence; Soak #3 and broader production resilience remain unclaimed unless separately approved and proven. |
| Runtime architecture diet | Runtime module map and keep/merge/postpone/retire follow-ons explicit enough to reduce bloat without counting refactoring plans as integration proof. |
| Observatory and demo readiness | Visible proof surfaces integrated/proven or blocked with evidence and operator approval. |
| CodeFriend, adapter v2, paper/publication surfaces | CodeFriend/adapter obligations are bounded by `#4756`, `docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md`, and `runtime_v2.codefriend_adapter_obligations.v1`: v0.92 may consume the complete v1 plan and tracked handoff truth only, not CodeFriend product readiness or external-repo execution. CodeFriend v1 / adapter v2 external-repo proof packaging remains a v0.95 MVP-convergence obligation. Paper/publication surfaces are bounded by #4757 and `review/wp13_publication_boundary_4757.md`: they are non-claim/promotion-gate truth only unless a later tracked issue promotes a specific artifact with evidence, redaction/public-claim review, and human approval. |

## Activation Blockers To Preserve

- No `v0.92` activation claim may cite these docs as runtime proof.
- Logging and observability are activation-path infrastructure, not optional garnish; no polis/runtime/Observatory claim may proceed without current integrated logging evidence.
- Security and ACIP/A2A implementation remains activation-path work until resolved
  or blocked with evidence and operator approval.
- Curiosity and Constructability require proof issues before public claims.
- Affect and happiness may use #4752 proof-bound operational
  reasoning-control language only; subjective affect, happiness, wellbeing, and
  consciousness remain explicit non-claims. Godel mechanics still require
  proof-bound public claim language in birthday evidence.
- Capability envelope, witnesses/receipt, and publication-facing narratives must
  not be inferred from launch language without tracked evidence.
- Papers, public launch copy, customer-facing reports, and CodeFriend
  publication claims must not be inferred from WP-13 closure. They require the
  promotion gates in `review/wp13_publication_boundary_4757.md`.

## `#3780` Consumption Checklist

- [ ] Read `FEATURE_DOCS_v0.91.7.md` and this handoff together.
- [ ] For each second-tranche surface, record whether `#3780` consumes it as
  integrated proof, blocks with evidence, or scopes it out with operator approval.
- [ ] Preserve runtime claim boundaries in `v0.92` activation docs.
- [ ] Do not reopen `v0.91.7` scope inside `v0.92` without a tracked issue.

## Non-Goals

- This handoff does not implement runtime behavior.
- This handoff does not approve `v0.92` activation.
- This handoff does not replace the `v0.92` activation consumption ledger.
