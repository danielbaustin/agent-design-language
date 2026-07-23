# Structured Task Prompt

Template: 1.0.0

Issue: 5592

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare and validate the complete Parity-B execution contract now; do not implement product code, amend into product paths, publish, or claim runtime acceptance until #5591 has a clean reviewed Parity-A contract and a typed disjoint claim can be established.

## Deliverables

- complete issue-specific SIP, STP, SPP, VPP, SRP, and SOR generated from the current typed v2 native templates
- Runtime v3 Parity-B design and dependency/control-flow diagram
- acceptance and owned-feature disposition matrix preserving every safe claim and boundary
- typed disjoint preparation-only protected-path claim plus a non-authoritative deferred product-scope procedure
- future live graph, loop, adaptation, affect-isolation, monotonic-governance, rollback, and feature-preservation proof contract
- typed preparation validation and doctor evidence at the committed revision

## Acceptance

1. AC-1: A guardian-launched initialized adl-runtime-kernel process accepts a versioned submitted reasoning graph through #5591 canonical ingress, executes production nodes over bounded channels, and retains deterministic transition and output evidence; fixture, direct-library, metadata, fixed-bootstrap, or degraded execution is insufficient
2. AC-2: Explicit loop state machines terminate deterministically under immutable iteration, deadline, cancellation, resource, and evidence budgets, and checkpoint/replay/resume preserves accepted sequence and remaining budgets without duplicate effects or limit reset
3. AC-3: Provenance-bound observations create inert adaptation proposals and only an exact signed one-shot grant can atomically produce a policy-bounded mutation with durable before/after hashes and deterministic rollback; forged, stale, replayed, wrong-policy, wrong-sequence, excessive, self-issued, interrupted, or reused grants fail closed
4. AC-4: Affect is a bounded typed reasoning-control signal with explicit subjective non-claims; adversarial task/tool/retrieval/model content cannot directly set signals, policy, review, grants, budgets, or authority, and signal influence can only preserve or reduce authority, reorder pre-authorized work, escalate review, or refuse
5. AC-5: Curiosity, discovery, intelligence confidence, and theory-of-mind task models remain bounded non-authoritative surfaces with no subjective-state, tool, network, disclosure, mutation, or policy authority; low confidence or provenance failure can only hold, escalate, or refuse
6. AC-6: Governed cognition, review records, adaptation, replay, and restart cannot bypass or widen Freedom Gate, shutdown, cancellation, resource limits, or human review authority, with deterministic adversarial monotonicity evidence
7. AC-7: Reasoning/adaptation, affect, curiosity/theory-of-mind, Constructability, Godel mechanics, guild, economics context, and skill standard each receive exactly one reviewed feature disposition with named proof or owner, and metadata/schema/fixture/context-only evidence never receives live_runtime_v3 credit
8. AC-8: Runtime v3 implementation and proof contain no Runtime v2 source import, copy, link, execution, modification, or parity credit; duplicate reasoning implementations are inventoried and deleted only after live replacement proof, with no AWS, cutover, default switch, Runtime v2 deletion, or new product claim
9. AC-9: One clean exact revision retains live-kernel positive and negative evidence for ingress-to-terminal execution, durability, deterministic recovery, rollback, adversarial signal isolation, authority monotonicity, and feature dispositions; skipped, pending, degraded, prose-only, library-only, metadata-only, fixed-bootstrap, or non-exact evidence is non-proving
10. AC-10: Focused and complete tests, strict format/lint, dependency inventory, source LoC, module growth, test count, and exact-revision bounded review pass the integrated #5336 budget without removing required behavior, negative proof, safe non-claims, or review findings

## Dependencies

- #5361 Runtime v3 acceptance umbrella
- #5336 reviewed architecture, feature-ledger, and budget authority
- clean reviewed #5591 Parity-A canonical ingress and continuity contract before any product implementation
- typed active-claim ledger showing a collision-free narrow Parity-B product scope
- #5341 is downstream of the reviewed #5591 canonical ingress and accepted #5592 graph/event contract and grants no implementation authority here
- #5107 downstream adaptive-learning DAG queue boundary
- retained v0.91.7 reasoning, affect, curiosity, Constructability, Godel, economics-context, and skill-standard contracts

## Inputs

- .csdlc/prepared/issues/5592/source-authority.json operator-directed canonical title authority
- .adl/local-artifacts/wp5594/live-v0918-issues.json mutable operator snapshot for issue 5592 body provenance only; not canonical live truth
- .csdlc/prepared/issues/5592/design.md
- .csdlc/prepared/issues/5592/acceptance-matrix.md
- docs/milestones/v0.91.8/features/RUNTIME_V3_ADAPTER_v0.91.8.md
- docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md
- docs/planning/ADL_FEATURE_LIST.md
- docs/milestones/v0.91.7/features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md
- docs/milestones/v0.91.7/features/GODEL_MECHANICS_BRIDGE_v0.91.7.md
- docs/milestones/v0.91.7/features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md
- docs/milestones/v0.94/features/REASONING_GRAPH_BASELINE_v0.94.md
- adl-runtime-kernel/Cargo.toml and current production module/test inventory

## Non Goals

- product implementation, product-path binding, publication, or acceptance before clean reviewed #5591
- Runtime v2 source reuse, modification, execution credit, defaulting, cutover, deletion, or replacement claim
- AWS, provider deployment, hosted-provider proof, model training, or new product scope
- subjective affect, wellbeing, suffering, consciousness, mind-reading, personhood, scalar reward, or reputation claims
- autonomous or recursive self-improvement, complete GHB runtime, birthday, guild authority, payment, settlement, marketplace, or broader v0.94/v0.95 completion
