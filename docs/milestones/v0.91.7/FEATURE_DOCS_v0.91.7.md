# v0.91.7 Feature-Doc Index

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Setup lineage: `#3801`, `#3825`, `#4368`

## Status

Feature-doc package created for the final pre-`v0.92` bridge/readiness tranche. These
docs define planning, decisions, validation expectations, and `v0.92`
consumption limits; they do not implement runtime behavior.

## Required Feature Docs And Bridge Records

| Feature doc | Surface | Required questions | Exit state before v0.92 |
| --- | --- | --- | --- |
| [`CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md`](features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md) | Curiosity Engine / Discovery Substrate | What artifacts, hooks, hypotheses, budgets, governance, Freedom Gate, ObsMem/reasoning-graph updates, and proof are required? | Bridge doc exists; governed proof remains future issue work. |
| [`CONSTRUCTABILITY_GATE_v0.91.7.md`](features/CONSTRUCTABILITY_GATE_v0.91.7.md) | Constructability Gate | What construction events, external anchors, validators, and shared-reality boundaries are required? | Bridge doc exists; validator proof remains future issue work. |
| [`REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md`](features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md) | Reasoning graph / loop runtime / `adl.skill.v1` | How do prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1` connect before v0.92? | Bridge doc exists; full standard remains later work. |
| [`SECURITY_RESIDUAL_READINESS_v0.91.7.md`](features/SECURITY_RESIDUAL_READINESS_v0.91.7.md) | Residual security readiness | What remains after v0.91.6 security/CAV, and what blocks activation? | Bridge doc exists; blockers must be named by implementation issues. |
| [`ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md`](features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md) | Residual ACIP/A2A/protobuf decisions | Which JSON/protobuf/WebSocket/access-rule choices remain, and what can v0.92 consume? | Bridge doc exists; ambiguous protocol posture remains a blocker. |
| [`AFFECT_HAPPINESS_BRIDGE_v0.91.7.md`](features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md) | Affect and happiness surfaces | What safe tests and non-claims govern affect, humor, happiness, and wellbeing evidence? | Bridge doc exists; public evidence remains bounded by non-claims. |
| [`GODEL_MECHANICS_BRIDGE_v0.91.7.md`](features/GODEL_MECHANICS_BRIDGE_v0.91.7.md) | Godel mechanics | What experiment, hypothesis, mutation, evaluation, and promotion mechanics can birthday evidence consume? | Bridge doc exists; runtime mechanics remain future issue work. |
| [`ECONOMICS_CONTEXT_DECISION_v0.91.7.md`](features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md) | Economics context | Is economics context-only for v0.92, or does it require explicit tests? | Bridge doc exists; default posture is context-only unless promoted. |

## Additional Required Planning Routes

The refreshed source-capture pass also requires explicit routes for operational substrate that is not represented as one of the eight original feature docs:

| Route | Source | Required disposition before v0.92 |
| --- | --- | --- |
| SEP / VPP / PVF / prompt-template next version | `#4308`, `#4309`, `#4332`, `#4388`-`#4398`, `#4417`-`#4421` plus `#4425`, sprint execution packets | Complete, blocked, deferred, or routed before sprint-scale v0.92 execution relies on it; generated VPPs must not remain chat-memory policy. |
| Goal state and execution metrics | `.adl/docs/TBD/ADL_GOAL_STATE.md`, `#4329`, `#4331`, `#4431`, `#4441`, `#4442` | Route SOR time/token/resource, nested goal accounting, forward metric capture, bounded v0.91.6 backfill, and host goal snapshots. |
| Scheduler/provider/local-agent routing | scheduler/provider v0.91.6 docs and TBD scheduler/economics notes | Route cheapest-validated-outcome scheduling and local/hosted model suitability. |
| Build and validation throughput | build-throughput reviews, validation-manager/test-tax docs, `#4417`-`#4421`, Nessus/CodeBuild candidates | Route validation-cost, path ownership, SOR fact capture, validation manager, and remote/local build decisions. |
| Runtime integration, Soak #2, and AWS signal bridge | runtime fire-up, runtime AWS signal bridge, ACIP-to-SNS, heartbeat, SSM docs | Route one minimal assembled runtime proof, operational signal surfaces, and architecture-diet follow-on boundaries. |
| Observatory and launch/birthday evidence | Observatory docs, launch plan, demo matrix | Route visible proof and public non-claim boundaries. |
| Feature-list and roadmap truth | `docs/planning/ADL_FEATURE_LIST.md`, `issue-feature-list-roadmap-sync.md` | Refresh or explicitly route stale feature-list/roadmap claims before v0.92 planning consumes them. |
| Capability envelope and Aptitude Atlas boundary | feature-list rows for ACP, capability testing, capability envelope, memory grounding, birth witnesses/receipt | Route capability envelope and capability-testing evidence into the v0.92 handoff without turning Aptitude Atlas productization into a v0.92 blocker. |
| CodeFriend v1 and portable adapter v2 | `issue-codefriend-v1-proof-and-adapter-v2-acceptance.md`, feature-list CodeFriend/adapter rows | Preserve as post-v0.92/pre-v0.95 proof-planning work unless launch readiness explicitly promotes a bounded slice. |
| GitHub convergence and octocrab/tooling control plane | v0.91.6 tooling/octocrab convergence work, `#4405`, `#4412`-`#4413`, `#4433`-`#4438`, `#4443`, feature-list control-plane Rust migration rows | Ensure sprint/process execution knows whether GitHub convergence, session coordination, lifecycle liveness, operational adoption, and shepherding are complete, blocked, deferred, or routed before relying on them. |
| Paper and publication surfaces | feature-list publication/paper rows, launch planning | Explicitly defer or route paper/publication work so first-birthday launch evidence does not silently inherit publication claims. |

## Cross-Doc Requirements

- Every doc must name non-goals and unsupported claims.
- Every doc must include validation and review expectations.
- Every doc must say what `#3780` / `v0.92` may consume.
- Security, ACIP/A2A, Curiosity, Constructability, and reasoning graphs must
  not be collapsed into generic future-work language.
- `#3780` consumption truth is summarized in `V092_HANDOFF_v0.91.7.md`.

## Validation

When this index is consumed:

- verify each planned implementation/proof route has an owning issue or
  explicit blocked/deferred route
- scan for `v0.92` readiness overclaims
- scan for local authoring-workspace links or host-local paths
- verify all second-tranche surfaces remain visible
