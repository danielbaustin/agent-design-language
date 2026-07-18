# v0.91.7 Feature-Doc Index

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-21`
- Setup lineage: `#3801`, `#3825`, `#4368`

## Status

Feature-doc package aligned for the v0.91.7 closeout tail. Owning WP-10 through
WP-13 umbrellas and their declared child issues are closed, with retained
proof, boundary, and remediation evidence. These docs define consumption
limits; they are not standalone runtime proof or release approval.

## Required Feature Docs And Implementation Records

| Feature doc | Surface | Required questions | Exit state before v0.92 |
| --- | --- | --- | --- |
| [`CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md`](features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md) | Curiosity Engine / Discovery Substrate | What artifacts, hooks, hypotheses, budgets, governance, Freedom Gate, ObsMem/reasoning-graph updates, and proof are required? | Closed `#4692` retains bounded governed-cycle proof; broader autonomous discovery remains unclaimed. |
| [`CONSTRUCTABILITY_GATE_v0.91.7.md`](features/CONSTRUCTABILITY_GATE_v0.91.7.md) | Constructability Gate | What construction events, external anchors, validators, and shared-reality boundaries are required? | Closed `#4693` retains bounded validator and fail-closed proof. |
| [`REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md`](features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md) | Reasoning graph / loop runtime / `adl.skill.v1` | How do prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1` connect before v0.92? | Closed `#4694`-`#4697` retain bounded implementation/proof; full standard and adaptive-learning convergence remain later work. |
| [`SECURITY_RESIDUAL_READINESS_v0.91.7.md`](features/SECURITY_RESIDUAL_READINESS_v0.91.7.md) | Security implementation readiness | What remains after v0.91.6 security/CAV, and what blocks activation? | Closed `#4656`-`#4660` plus `#5404`/`#5406` retain bounded proof and remediation; broader release security remains review-gated. |
| [`ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md`](features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md) | ACIP/A2A/protobuf implementation decisions | Which JSON/protobuf/WebSocket/access-rule choices remain, and what can v0.92 consume? | Closed `#4658`-`#4660` retain bounded schema/projection, loopback WebSocket, and access-rule proof; production federation is not claimed. |
| [`AFFECT_HAPPINESS_BRIDGE_v0.91.7.md`](features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md) | Affect and happiness surfaces | What safe tests and public claim boundaries govern affect, humor, happiness, and wellbeing evidence? | Closed `#4752` retains the operational reasoning-control boundary; subjective affect, consciousness, and wellbeing claims remain prohibited. |
| [`GODEL_MECHANICS_BRIDGE_v0.91.7.md`](features/GODEL_MECHANICS_BRIDGE_v0.91.7.md) | Godel mechanics | What experiment, hypothesis, mutation, evaluation, and promotion mechanics can birthday evidence consume? | Closed `#4753` and remediation `#5405` retain the bounded Godel/constructability boundary; adaptive-DAG completion remains unclaimed. |
| [`ECONOMICS_CONTEXT_DECISION_v0.91.7.md`](features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md) | Economics context | Is economics context-only for v0.92, or does it require explicit tests? | Closed `#4754` retains the bounded economics context and non-claim boundary; payment, market, and product authority remain unclaimed. |
| [`GUILD_FOUNDATION_BOUNDARY_v0.91.7.md`](features/GUILD_FOUNDATION_BOUNDARY_v0.91.7.md) | Guild foundation | Which MVP guild surfaces can v0.92 consume as governance handoff context, and which v0.93 governance claims remain unsupported? | Closed `#4755` retains the declarative guild-governance handoff boundary; implemented membership, citizenship, and collective authority remain unclaimed. |
| [`AWS_SPOT_REMOTE_VALIDATION_LANE_v0.91.7.md`](features/AWS_SPOT_REMOTE_VALIDATION_LANE_v0.91.7.md) | Historical remote-validation lane | What bounded implementation and account-bound proof were retained during v0.91.7? | Historical implementation/proof remains retained. Current operator posture forbids AWS execution; this record is not an active validation route or authorization. |

## Additional Planning Dispositions

The refreshed source-capture pass also assigned operational substrate that is
not represented by a primary feature document. The current closeout
dispositions are:

| Assignment | Source | Current closeout disposition |
| --- | --- | --- |
| SEP / VPP / PVF / prompt-template next version | `#4308`, `#4309`, `#4332`, `#4388`-`#4398`, `#4417`-`#4421` plus `#4425`, sprint execution packets | The v0.91.7 control-plane and validation packets are retained as bounded evidence. Current C-SDLC v2 authority remains typed and generated; no chat-memory VPP policy is promoted by this document. |
| Goal state and execution metrics | `.adl/docs/TBD/ADL_GOAL_STATE.md`, `#4329`, `#4331`, `#4431`, `#4441`, `#4442`, WP-04 `#4631` | WP-04 is closed with retained time/token/resource and nested-goal evidence. Broader historical backfill and product analytics are not inferred. |
| Scheduler/provider/local-agent routing | scheduler/provider v0.91.6 docs, WP-05 `#4632`, retained provider review packets | WP-05 is closed and review-remediated. v0.92 may consume only the retained suitability, delegation, and fail-closed identity proof; cheapest-outcome or universal model-selection claims remain unproven. |
| Build and validation throughput | WP-06 `#4633`, build-throughput reviews, validation-manager/test-tax docs | WP-06 is closed with bounded local/remote throughput evidence. Remote validation is not a release-critical default, and the current operator direction prohibits AWS execution. |
| Runtime integration, Soak #2, and signal integration | WP-07 `#4634`, WP-08 `#4635`, closed `#4682`, closed `#4718`, retained runtime packets | The umbrellas and Soak/observability proof issues are closed with bounded evidence. Runtime hardening remediation `#5408` remains open; broad production resilience and telemetry remain unclaimed. No new AWS execution is authorized. |
| Observatory and launch/birthday evidence | WP-09 `#4636`, WP-14 `#4641`, WP-15 `#4642`, Observatory docs and demo matrix | The three umbrellas are closed with bounded visible-proof and handoff packets. Runtime v3 remains explicit opt-in, Unity limitations remain visible, and neither release nor public-launch approval is inferred. |
| Feature-list and roadmap truth | `docs/planning/ADL_FEATURE_LIST.md`, `issue-feature-list-roadmap-sync.md`, WP-17 `#4644` | This issue aligns the roadmap and feature index to live closeout truth; later review gates remain independent. |
| Capability envelope and Aptitude Atlas boundary | WP-14 handoff and v0.91.8 issues `#4758`-`#4763` | WP-14 is closed as routing-with-evidence. Capability, Memory Palace, witness/receipt, and birthday activation proof remain owned by the open v0.91.8 bridge issues and are not v0.91.7 completion claims. |
| CodeFriend v1 and portable adapter v2 | closed `#4756`, CodeFriend v1 plan, WP-13 obligation packet | The bounded Runtime v2 obligation/handoff is retained. External-repository CodeFriend product proof and portable adapter v2 completion remain v0.95 work unless separately promoted. |
| GitHub convergence and tooling control plane | v0.91.6 convergence work, closed WP-03 `#4630`, retained C-SDLC v2 records | Bounded lifecycle/shepherd/tooling evidence is retained under typed C-SDLC v2 authority. This docs pass does not claim universal control-plane reliability or revive sunset v1 wrappers. |
| Paper and publication surfaces | closed `#4757`, `review/wp13_publication_boundary_4757.md` | Paper/publication work remains scoped out of v0.92 activation and is not externally approved. Any publication still requires a tracked promotion, evidence/redaction review, and human approval. |
| WP-13 parent closeout | closed `#4640`, `review/wp13_closeout_4640.md` | WP-13 reconciles closed children `#4752`-`#4757`; every child packet's non-claims and promotion gates remain authoritative. |

## Cross-Doc Requirements

- Every doc must name non-goals and unsupported claims.
- Every doc must include validation and review expectations.
- Every doc must say what `#3780` / `v0.92` may consume.
- Security, ACIP/A2A, Curiosity, Constructability, and reasoning graphs must
  not be collapsed into generic future-work language.
- `#3780` consumption truth is summarized in `V092_HANDOFF_v0.91.7.md`.

## Validation

When this index is consumed:

- verify each planned implementation/proof surface has an owning issue and
  exits as proof, operator-scoped-out with evidence, or evidence-backed blocker
- scan for `v0.92` readiness overclaims
- scan for local authoring-workspace links or host-local paths
- verify all second-tranche surfaces remain visible
