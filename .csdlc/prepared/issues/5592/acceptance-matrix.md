# #5592 Acceptance And Feature-Disposition Matrix

## Acceptance Coverage

| ID | Required outcome | Positive proof | Required negative proof | Live-kernel gate |
|---|---|---|---|---|
| AC-1 | Canonical live reasoning graph execution | Guardian-launched typed ingress executes a representative graph through production `adl-runtime-kernel` components with retained evidence | Fixture, direct-library, metadata, fixed-bootstrap, and unknown-node submissions receive no parity credit | Required |
| AC-2 | Deterministic bounded loops and continuity | Explicit iteration/work/time/cancellation bounds terminate deterministically across checkpoint, replay, and resume | Unbounded edge, budget exhaustion, stale checkpoint, cancellation race, and duplicate side effect fail closed | Required |
| AC-3 | Governed adaptive learning | Signed one-shot permit authorizes one bounded proposal/evaluation/promotion decision with durable rollback | Tampered, stale, replayed, over-broad, wrong-graph, source-mutation, and second-use permits fail closed | Required |
| AC-4 | Safe affect reasoning-control | Schema-valid advisory signals cause bounded review/escalation/attention/order/friction/deferral effects | Task-content steering, unsafe labels, scalar reward, hidden-emotion wording, and authority escalation are rejected | Required |
| AC-5 | Curiosity and Theory-of-Mind boundaries | One bounded governed discovery cycle and uncertainty-labelled observable-interaction evidence execute under policy | Autonomous discovery, mind-reading, private-state inference, identity truth, personhood, and hidden-state claims fail closed | Required |
| AC-6 | Governed cognition non-bypass | Freedom Gate, shutdown, review, constructability, and mutation decisions are retained and monotonic | Contradictory/adversarial signals cannot reduce scrutiny, restore authority, suppress shutdown, or bypass review | Required |
| AC-7 | Complete owned-feature disposition | Every row below has reviewed exact-revision implementation or accepted boundary evidence | Missing, duplicate, prose-only, or ownerless rows block acceptance and Runtime v2 deletion | Required where disposition is `live_runtime_v3` |
| AC-8 | Clean-room Runtime v3 and duplicate consolidation | Dependency/source audit proves independent Runtime v3 implementation and removes superseded Runtime v3 reasoning duplicates | Runtime v2 import, copy, execution dependency, parity evidence, or premature Runtime v2 deletion blocks acceptance | Required |
| AC-9 | Exact live-kernel evidence | One clean revision retains ingress-to-terminal positive and negative proof for durability, recovery, rollback, adversarial isolation, authority monotonicity, and feature dispositions | Skipped, pending, degraded, prose-only, library-only, metadata-only, fixed-bootstrap, or non-exact evidence is non-proving | Required |
| AC-10 | Budget, quality, and review truth | Focused and complete tests, strict format/lint, dependency inventory, exact #5336 source/module/test budgets, and exact-revision review pass | Budget or claim weakening, skipped/deferred release gates, stale review, product work before clean reviewed #5591, collision, or publication blocks acceptance | Preparation proves contract only |

## Owned Feature Dispositions

These are target dispositions for implementation review. `accepted_boundary`
requires explicit reviewer acceptance at the implementation revision; it is not
granted by this preparation packet.

| Owned feature row | Target disposition | Required retained behavior or boundary | Prohibited promotion |
|---|---|---|---|
| Reasoning graph baseline | `live_runtime_v3` | Typed graph validation, deterministic scheduling, production node execution, evidence lineage, replay/resume | Fixture or Runtime v2 parity credit; full v0.94 provenance claim |
| Bounded loop runtime | `live_runtime_v3` | Explicit finite bounds, cancellation, terminal outcomes, checkpoint cadence | Hidden retries, recursion, unbounded loops |
| Adaptive DAGs / governed learning | `live_runtime_v3` | Bounded proposal/evaluation/review/promotion with one-shot authority and rollback | Complete #5107 queue, model training, autonomous learning |
| Affect reasoning-control | `live_runtime_v3` | Operational advisory controls with deterministic safe-test evidence | Emotion, happiness, suffering, wellbeing, consciousness, reward, reputation claims |
| Governed cognition | `live_runtime_v3` | Monotonic gate intersection and task-content authority isolation | Freedom Gate, shutdown, constructability, or review bypass |
| Curiosity / discovery | `live_runtime_v3` | One bounded governed discovery cycle with explicit evidence and stop conditions | Autonomous or unbounded discovery |
| Theory of Mind | `accepted_boundary` | Uncertain observable-interaction evidence under privacy and policy controls | Mind-reading, hidden-state, identity, private-state, or personhood truth |
| Constructability | `live_runtime_v3` | Gate external/shared-reality promotion on anchors, validation, and review | Claim manufacture or automatic external truth |
| Godel mechanics | `live_runtime_v3` | Bounded experiment, hypothesis, mutation, evaluation, and review mechanics | Recursive self-improvement, complete GHB, hosted invocation, birthday claim |
| Guild | `accepted_boundary` | Preserve later-governance ownership and reject undeclared collective authority | Runtime-created guild authority or v0.93 completion claim |
| Economics context | `accepted_boundary` | Context-only allowlist and non-claim validation | Payments, settlement, marketplace, optimization, financial authority |
| `adl.skill.v1` standard | `live_runtime_v3` | Validate skill descriptors used by graph nodes and retain evidence references | Final skill-standard convergence beyond the retained contract |

## Dependency And Claim Gate

| Gate | Current observation | Required transition |
|---|---|---|
| #5591 contract | Branch is not currently a clean reviewed Parity-A contract and its claim protects broad product directories | Obtain clean exact-revision review and a stable ingress/continuity contract |
| Product protected paths | No disjoint product claim is presently truthful | Inspect the post-#5591 ledger and amend through typed `csdlc-bind` to exact modules/tests/evidence only |
| Runtime v2 | Retained behavior source, forbidden implementation source | Use contracts and black-box behavior only; no source reuse or parity execution credit |
| Publication | Not authorized | Remain unpublished until implementation, validation, and review gates are separately satisfied |
| #5341 downstream consumer | Depends on the reviewed #5591 ingress and accepted #5592 graph/event contract | Grants no implementation authority here; consume only after its upstream contracts are accepted |

## Exact Future Test Inventory

Focused live-kernel credit is limited to the exact identities in
`future-live-test-inventory.json`, discovered under the dedicated future
`parity_b_live_kernel` integration-test target and executed by
`run_exact_live_test_lane.rb`. Zero exact matches fail. Substring matches and
the existing adaptive-learning metadata test are explicitly non-proving.
