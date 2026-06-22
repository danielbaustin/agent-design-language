# v0.91.7 to v0.92 Handoff

## Metadata

- Source milestone: `v0.91.7`
- Target milestone: `v0.92`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Owner: ADL maintainers
- Related issues: `#3825`, `#4368`, `#3780`

## Purpose

Record the second-tranche bridge surfaces that `#3780` / `v0.92` may consume
after `v0.91.7` docs-package completion. This is a handoff record, not runtime
implementation proof.

## Handoff Rule

`v0.92` may consume a surface only as one of:

- `doc-ready`: reviewed bridge doc exists, but implementation proof remains
  future issue work;
- `blocked`: named missing evidence or decision prevents activation use;
- `deferred`: explicitly outside `v0.92` activation scope;
- `routed`: owned by a named follow-on issue or milestone.

## Surface Dispositions

| Surface | Handoff state | v0.92 consumption limit | Source doc |
| --- | --- | --- | --- |
| Curiosity Engine / Discovery Substrate | doc-ready | `v0.92` may consume governance, artifact, budget, and proof expectations only; governed discovery-cycle proof remains future work. | `features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md` |
| Constructability Gate | doc-ready | `v0.92` may consume the shared-reality boundary and validator expectations only; runtime validator proof remains future work. | `features/CONSTRUCTABILITY_GATE_v0.91.7.md` |
| Reasoning graph / loop runtime / `adl.skill.v1` | doc-ready | `v0.92` may consume the bridge map; full skill-standard ratification and graph runtime remain later work. | `features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md` |
| Residual security readiness | doc-ready | `v0.92` may consume named residual categories; unresolved activation blockers must be complete, blocked, deferred, or routed before launch. | `features/SECURITY_RESIDUAL_READINESS_v0.91.7.md` |
| ACIP/A2A/protobuf residuals | doc-ready | `v0.92` must choose JSON projection, protobuf, mock carrier, or explicit deferral before claiming protocol readiness. | `features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md` |
| Affect and happiness | doc-ready | `v0.92` may consume safe-test and non-claim boundaries only; no inner-state, wellbeing, or consciousness claim is supported. | `features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md` |
| Godel mechanics | doc-ready | `v0.92` may consume a reviewed mechanics map only; autonomous self-improvement and runtime completion are unsupported. | `features/GODEL_MECHANICS_BRIDGE_v0.91.7.md` |
| Economics context | doc-ready | Default `v0.92` posture is context-only unless an explicit operator decision promotes a bounded test. | `features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md` |

## Operational Substrate Handoff

`v0.92` should also consume the operational substrate dispositions from `PLANNING_SOURCE_CAPTURE_v0.91.7.md`:

| Surface | Required state before v0.92 |
| --- | --- |
| C-SDLC integration control plane | v0.91.6 `#4388`-`#4398`, `#4405`, `#4412`-`#4413`, `#4417`-`#4425`, `#4433`-`#4438`, and `#4443` / SEP / VPP / PVF / templates / GitHub-octocrab convergence / session ledger / logging / watcher-lifecycle automation / operational adoption / shepherding / FastContext complete, blocked, deferred, or routed with clear sprint-execution consequences. |
| Goal and metrics accounting | Time/token/resource, nested-goal, forward capture `#4431`, v0.91.6-only backfill `#4441`, and host goal snapshot `#4442` route explicit enough for v0.92 issue planning. |
| Scheduler/provider/local agents | Routing policy and suitability path explicit enough to protect premium cognition. |
| Capability envelope and capability testing | Memory grounding, capability envelope, birth witnesses/receipt, and Aptitude Atlas boundaries explicitly consumed, deferred, blocked, or routed before birthday evidence relies on them. |
| Build/validation throughput | Validation-cost, path ownership, SOR fact capture, validation manager, VPP generation, and remote/local build routes clear enough to avoid rediscovery during birthday work. |
| GitHub convergence and control-plane tooling | GitHub/octocrab/tooling convergence, session coordination, lifecycle liveness, and shepherd state explicit enough that v0.92 sprint execution does not depend on ambiguous `gh` fallback, stale control-plane assumptions, or chat-only session memory. |
| Runtime integration, Soak #2, and AWS signal bridge | One minimal assembled runtime path proven or blocked; operational signal blockers named before birthday claims. |
| Runtime architecture diet | Runtime module/seam map and keep/merge/defer/retire route explicit enough to reduce bloat without blocking the integration proof. |
| Observatory and demo readiness | Visible proof surfaces complete, blocked, deferred, or routed. |
| CodeFriend, adapter v2, paper/publication surfaces | Explicitly deferred or routed unless launch readiness promotes a bounded, evidence-backed slice. |

## Activation Blockers To Preserve

- No `v0.92` activation claim may cite these docs as runtime proof.
- Security and ACIP/A2A residuals remain activation-path work until complete,
  blocked, deferred, or routed with evidence.
- Curiosity and Constructability require proof issues before public claims.
- Affect, happiness, and Godel mechanics require non-claim language in birthday
  evidence.
- Capability envelope, witnesses/receipt, and publication-facing narratives must
  not be inferred from launch language without tracked evidence.

## `#3780` Consumption Checklist

- [ ] Read `FEATURE_DOCS_v0.91.7.md` and this handoff together.
- [ ] For each second-tranche surface, record whether `#3780` consumes,
  blocks, defers, or routes it.
- [ ] Preserve runtime non-claims in `v0.92` activation docs.
- [ ] Do not reopen `v0.91.7` scope inside `v0.92` without a tracked issue.

## Non-Goals

- This handoff does not implement runtime behavior.
- This handoff does not approve `v0.92` activation.
- This handoff does not replace the `v0.92` activation bridge ledger.
