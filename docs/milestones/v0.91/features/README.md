# v0.91 Feature Package

This directory holds the tracked first-class feature docs for the v0.91 core
cognitive-being milestone. These docs now feed the active v0.91 issue wave, WP
readiness source, opened issue-card bundles, demo matrix, feature-proof
coverage map, and `v0.91.0` architecture decisions.

## Tracked Feature Docs

| Feature surface | Tracked doc | Status |
| --- | --- | --- |
| Moral event contract | [MORAL_EVENT_CONTRACT.md](MORAL_EVENT_CONTRACT.md) | Implemented and proof-mapped |
| Moral trace schema | [MORAL_TRACE_SCHEMA.md](MORAL_TRACE_SCHEMA.md) | Implemented and proof-mapped |
| Outcome linkage and attribution | [OUTCOME_LINKAGE_AND_ATTRIBUTION.md](OUTCOME_LINKAGE_AND_ATTRIBUTION.md) | Implemented and proof-mapped |
| Moral metrics | [MORAL_METRICS.md](MORAL_METRICS.md) | Implemented and proof-mapped |
| Moral trajectory review | [MORAL_TRAJECTORY_REVIEW.md](MORAL_TRAJECTORY_REVIEW.md) | Implemented and proof-mapped |
| Anti-harm trajectory constraints | [ANTI_HARM_TRAJECTORY_CONSTRAINTS.md](ANTI_HARM_TRAJECTORY_CONSTRAINTS.md) | Implemented and proof-mapped |
| Wellbeing and happiness | [WELLBEING_AND_HAPPINESS.md](WELLBEING_AND_HAPPINESS.md) | Implemented and proof-mapped |
| Kindness | [KINDNESS.md](KINDNESS.md) | Implemented and proof-mapped |
| Humor and absurdity | [HUMOR_AND_ABSURDITY.md](HUMOR_AND_ABSURDITY.md) | Implemented and proof-mapped |
| Affect reasoning-control | [AFFECT_REASONING_CONTROL.md](AFFECT_REASONING_CONTROL.md) | Implemented and proof-mapped |
| Cultivating intelligence | [CULTIVATING_INTELLIGENCE.md](CULTIVATING_INTELLIGENCE.md) | Implemented and proof-mapped |
| Moral resources | [MORAL_RESOURCES.md](MORAL_RESOURCES.md) | Implemented and proof-mapped |
| Structured planning and plan review | [STRUCTURED_PLANNING_AND_PLAN_REVIEW.md](STRUCTURED_PLANNING_AND_PLAN_REVIEW.md) | Implemented baseline and proof-mapped |
| Structured review policy and SRP | [STRUCTURED_REVIEW_POLICY_AND_SRP.md](STRUCTURED_REVIEW_POLICY_AND_SRP.md) | Implemented baseline and proof-mapped |
| A2A external agent adapter | [A2A_EXTERNAL_AGENT_ADAPTER.md](A2A_EXTERNAL_AGENT_ADAPTER.md) | Bounded adapter boundary proof-mapped |

## Cross-Cutting v0.91 Planning Sources

| Feature surface | Current tracked source | Status |
| --- | --- | --- |
| Moral governance allocation | [../MORAL_GOVERNANCE_ALLOCATION_v0.91.md](../MORAL_GOVERNANCE_ALLOCATION_v0.91.md) | Allocation map for moral-event, trace, validation, attribution, metrics, trajectory review, and anti-harm |
| Agent Communication and Invocation Protocol | [../AGENT_COMMS_SPLIT_PLAN_v0.91.md](../AGENT_COMMS_SPLIT_PLAN_v0.91.md) | Split across v0.90.5, v0.91, v0.91.1 hardening, and v0.92 prerequisites |
| Cognitive-being allocation | [../COGNITIVE_BEING_FEATURES_v0.91.md](../COGNITIVE_BEING_FEATURES_v0.91.md) | Milestone allocation and v0.91/v0.91.1 boundary doc |
| Active issue wave | [../WP_ISSUE_WAVE_v0.91.yaml](../WP_ISSUE_WAVE_v0.91.yaml) | Active WP sequence; issue numbers assigned as #2735-#2759 |
| WP execution readiness | [../WP_EXECUTION_READINESS_v0.91.md](../WP_EXECUTION_READINESS_v0.91.md) | Card-authoring source for required outputs, validation, boundaries, and proof expectations |
| Architecture decisions | [../../adr/0016-moral-evidence-and-cognitive-being-substrate.md](../../adr/0016-moral-evidence-and-cognitive-being-substrate.md), [../../adr/0017-secure-local-agent-comms-and-a2a-boundary.md](../../adr/0017-secure-local-agent-comms-and-a2a-boundary.md), [../../adr/0018-structured-planning-and-review-policy-artifacts.md](../../adr/0018-structured-planning-and-review-policy-artifacts.md) | Accepted `v0.91.0` decision records |

## Execution Coverage Map

| Planned workstream | Primary source docs | Active WP placement |
| --- | --- | --- |
| Milestone setup and card authoring | [../WP_ISSUE_WAVE_v0.91.yaml](../WP_ISSUE_WAVE_v0.91.yaml), [../WP_EXECUTION_READINESS_v0.91.md](../WP_EXECUTION_READINESS_v0.91.md) | WP-01 |
| Moral event, validation, trace, attribution, metrics, trajectory, and anti-harm | [MORAL_EVENT_CONTRACT.md](MORAL_EVENT_CONTRACT.md), [MORAL_TRACE_SCHEMA.md](MORAL_TRACE_SCHEMA.md), [OUTCOME_LINKAGE_AND_ATTRIBUTION.md](OUTCOME_LINKAGE_AND_ATTRIBUTION.md), [MORAL_METRICS.md](MORAL_METRICS.md), [MORAL_TRAJECTORY_REVIEW.md](MORAL_TRAJECTORY_REVIEW.md), [ANTI_HARM_TRAJECTORY_CONSTRAINTS.md](ANTI_HARM_TRAJECTORY_CONSTRAINTS.md), [../MORAL_GOVERNANCE_ALLOCATION_v0.91.md](../MORAL_GOVERNANCE_ALLOCATION_v0.91.md), [../WP_EXECUTION_READINESS_v0.91.md](../WP_EXECUTION_READINESS_v0.91.md) | WP-02 through WP-08 |
| Wellbeing and happiness | [WELLBEING_AND_HAPPINESS.md](WELLBEING_AND_HAPPINESS.md), [../COGNITIVE_BEING_FEATURES_v0.91.md](../COGNITIVE_BEING_FEATURES_v0.91.md) | WP-09 |
| Moral resources | [MORAL_RESOURCES.md](MORAL_RESOURCES.md), [../COGNITIVE_BEING_FEATURES_v0.91.md](../COGNITIVE_BEING_FEATURES_v0.91.md) | WP-10 |
| Kindness | [KINDNESS.md](KINDNESS.md), [../COGNITIVE_BEING_FEATURES_v0.91.md](../COGNITIVE_BEING_FEATURES_v0.91.md) | WP-11 |
| Humor and absurdity | [HUMOR_AND_ABSURDITY.md](HUMOR_AND_ABSURDITY.md), [../COGNITIVE_BEING_FEATURES_v0.91.md](../COGNITIVE_BEING_FEATURES_v0.91.md) | WP-12 |
| Affect reasoning-control | [AFFECT_REASONING_CONTROL.md](AFFECT_REASONING_CONTROL.md), [../COGNITIVE_BEING_FEATURES_v0.91.md](../COGNITIVE_BEING_FEATURES_v0.91.md) | WP-13 |
| Cultivating intelligence | [CULTIVATING_INTELLIGENCE.md](CULTIVATING_INTELLIGENCE.md), [../COGNITIVE_BEING_FEATURES_v0.91.md](../COGNITIVE_BEING_FEATURES_v0.91.md) | WP-14 |
| Structured planning, plan review, and SRP | [STRUCTURED_PLANNING_AND_PLAN_REVIEW.md](STRUCTURED_PLANNING_AND_PLAN_REVIEW.md), [STRUCTURED_REVIEW_POLICY_AND_SRP.md](STRUCTURED_REVIEW_POLICY_AND_SRP.md) | WP-15 |
| Secure Agent Comms and A2A boundary | [../AGENT_COMMS_SPLIT_PLAN_v0.91.md](../AGENT_COMMS_SPLIT_PLAN_v0.91.md), [A2A_EXTERNAL_AGENT_ADAPTER.md](A2A_EXTERNAL_AGENT_ADAPTER.md) | WP-16 |
| Flagship demo, proof coverage, quality, docs, review, remediation, next planning, and release | [../DEMO_MATRIX_v0.91.md](../DEMO_MATRIX_v0.91.md), [../FEATURE_PROOF_COVERAGE_v0.91.md](../FEATURE_PROOF_COVERAGE_v0.91.md), [../RELEASE_PLAN_v0.91.md](../RELEASE_PLAN_v0.91.md), [../WP_EXECUTION_READINESS_v0.91.md](../WP_EXECUTION_READINESS_v0.91.md) | WP-17 through WP-25 |

## Boundary Notes

v0.91.1 should separately absorb adjacent source groups:

- capability and aptitude testing
- intelligence metric architecture
- ANRM/Gemma
- Theory of Mind
- memory/identity alignment
- runtime-v2/polis documentation
- ACIP hardening that does not fit safely in v0.91

Those should not be silently folded into the v0.91 core feature list.

## Implementation And Proof Status

The feature docs in this directory define the landed `v0.91.0` implementation
bar rather than concept-only placeholders. The demo matrix and feature-proof
coverage record now map each tracked feature surface to concrete proof routes,
fixture-backed validation, integrated flagship evidence, or explicit deferral.

The remaining milestone work is the final release ceremony. Internal review,
third-party review, accepted-finding remediation, and next-milestone planning
are complete or explicitly dispositioned.
