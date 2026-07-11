# v0.91.7 to v0.92 Handoff

## Metadata

- Source milestone: `v0.91.7`
- Target milestone: `v0.92`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Owner: ADL maintainers
- Related issues: `#3825`, `#4368`, `#3982`, `#3780`

## Purpose

Record the second-tranche implementation/proof surfaces that `#3780` / `v0.92` may consume
after `v0.91.7` docs-package completion. This is a handoff record, not runtime
implementation proof.

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

## Surface Dispositions

| Surface | Handoff state | v0.92 consumption limit | Source doc |
| --- | --- | --- | --- |
| Curiosity Engine / Discovery Substrate | implementation_required | `v0.92` may consume this surface only after governed discovery-cycle proof is integrated/proven or blocked with evidence and operator approval. | `features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md` |
| Constructability Gate | implementation_required | `v0.92` may consume this surface only after the shared-reality boundary and validator proof are integrated/proven or blocked with evidence and operator approval. | `features/CONSTRUCTABILITY_GATE_v0.91.7.md` |
| Reasoning graph / loop runtime / `adl.skill.v1` | implementation_required | `v0.92` may consume this surface only after producer/consumer or runtime proof exists for the required graph/loop/skill-standard path. | `features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md` |
| Security readiness | blocked_with_evidence until resolved | Unresolved activation blockers must be resolved or blocked with evidence and operator approval before launch. | `features/SECURITY_RESIDUAL_READINESS_v0.91.7.md` |
| ACIP/A2A/protobuf implementation | blocked_with_evidence until resolved | `v0.92` must choose JSON projection, protobuf, or another implemented carrier before claiming protocol readiness. | `features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md` |
| Affect and happiness | integrated_proven for operational reasoning-control; subjective affect not_claimed | `v0.92` may consume the #4752 safe-test boundary and existing `affect_reasoning_control_packet.v1` proof only as operational reasoning-control evidence. Birthday, launch, demo, or publication copy must not imply hidden emotion, subjective happiness, wellbeing, suffering, consciousness, scalar happiness scores, reward channels, or public reputation. | `features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md`; `review/wp13_affect_happiness_boundary_4752.md` |
| Godel mechanics | integrated_proven for CSM-supervised launch admission and claim-boundary consumption; live hosted invocation and adaptive DAG completion not_claimed | `v0.92` may consume the Runtime v2 Godel/constructability boundary for reviewed birthday claims only when retained Godel runtime evidence, launch-plan provider-request admission, and constructability anchors are cited. | `features/GODEL_MECHANICS_BRIDGE_v0.91.7.md`; `review/wp13_godel_constructability_boundary_4753.md` |
| Economics context | operator_scoped_out unless reopened | Default `v0.92` posture is context-only unless an explicit operator decision promotes and proves a bounded test. | `features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md` |
| Guild foundation | integrated_proven for governance handoff context; v0.93 governance not_claimed | `v0.92` may consume the Runtime v2 guild foundation boundary only as birthday governance context, identity witness evidence routing, community-memory boundary language, and future governance issue inputs. It must not imply constitutional citizenship, polis authority, delegated governance authority, binding collective decision-making, public guild product readiness, or completed governance. | `features/GUILD_FOUNDATION_BOUNDARY_v0.91.7.md`; `review/wp13_guild_foundation_boundary_4755.md` |

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
| Integrated logging, OTel boundary, and Observatory consumption | `#4718` is the current pre-v0.92 proof issue. `v0.92` remains blocked from relying on runtime/provider/control-plane logging, OTel compatibility, AWS/signal observability, or Observatory/Unity consumption until current integrated evidence exists for events, stdout/stderr separation, redaction/path hygiene, OTel boundary truth, and consumer samples. |
| Runtime integration, Soak #2, and AWS signal integration | `RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md` is the planning gate. `v0.92` remains blocked until that packet's required rows exit as `integrated_proven` or `blocked_with_evidence`; Soak #3 risk requires explicit operator approval. |
| Runtime architecture diet | Runtime module map and keep/merge/postpone/retire follow-ons explicit enough to reduce bloat without counting refactoring plans as integration proof. |
| Observatory and demo readiness | Visible proof surfaces integrated/proven or blocked with evidence and operator approval. |
| CodeFriend, adapter v2, paper/publication surfaces | CodeFriend/adapter obligations are bounded by `#4756`, `docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md`, and `runtime_v2.codefriend_adapter_obligations.v1`: v0.92 may consume the complete v1 plan and tracked handoff truth only, not CodeFriend product readiness or external-repo execution. CodeFriend v1 / adapter v2 external-repo proof packaging remains a v0.95 MVP-convergence obligation; paper/publication surfaces are operator-scoped-out unless launch readiness promotes a bounded, evidence-backed slice. |

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
