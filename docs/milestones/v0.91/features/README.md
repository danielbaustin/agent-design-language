# v0.91 Feature Planning

This directory now holds the tracked first-class feature docs for the v0.91
core cognitive-being milestone. These docs are still planning-phase artifacts,
not a final issue wave, but they are the canonical tracked feature surfaces for
later WP authoring and demo-matrix compression.

## Tracked Feature Docs

| Feature surface | Tracked doc | Status |
| --- | --- | --- |
| Wellbeing and happiness | [WELLBEING_AND_HAPPINESS.md](WELLBEING_AND_HAPPINESS.md) | Tracked feature doc |
| Kindness | [KINDNESS.md](KINDNESS.md) | Tracked feature doc |
| Humor and absurdity | [HUMOR_AND_ABSURDITY.md](HUMOR_AND_ABSURDITY.md) | Tracked feature doc |
| Affect reasoning-control | [AFFECT_REASONING_CONTROL.md](AFFECT_REASONING_CONTROL.md) | Tracked feature doc |
| Cultivating intelligence | [CULTIVATING_INTELLIGENCE.md](CULTIVATING_INTELLIGENCE.md) | Tracked feature doc |
| Moral resources | [MORAL_RESOURCES.md](MORAL_RESOURCES.md) | Tracked feature doc |
| Structured planning and plan review | [STRUCTURED_PLANNING_AND_PLAN_REVIEW.md](STRUCTURED_PLANNING_AND_PLAN_REVIEW.md) | Tracked workflow feature doc |
| Structured review policy and SRP | [STRUCTURED_REVIEW_POLICY_AND_SRP.md](STRUCTURED_REVIEW_POLICY_AND_SRP.md) | Tracked workflow feature doc |
| A2A external agent adapter | [A2A_EXTERNAL_AGENT_ADAPTER.md](A2A_EXTERNAL_AGENT_ADAPTER.md) | Tracked comms-adapter feature doc |

## Cross-Cutting v0.91 Planning Sources

| Feature surface | Current tracked source | Status |
| --- | --- | --- |
| Moral governance allocation | [../MORAL_GOVERNANCE_ALLOCATION_v0.91.md](../MORAL_GOVERNANCE_ALLOCATION_v0.91.md) | Allocation map for moral-event, trace, validation, attribution, metrics, trajectory review, and anti-harm |
| Agent Communication and Invocation Protocol | [../AGENT_COMMS_SPLIT_PLAN_v0.91.md](../AGENT_COMMS_SPLIT_PLAN_v0.91.md) | Split across v0.90.5, v0.91, v0.91.1 hardening, and v0.92 prerequisites |
| Cognitive-being allocation | [../COGNITIVE_BEING_FEATURES_v0.91.md](../COGNITIVE_BEING_FEATURES_v0.91.md) | Milestone allocation and v0.91/v0.91.1 boundary doc |

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

## Planning Status

The feature docs in this directory define a real implementation bar rather than
concept-only placeholders. The remaining gap is execution planning: the later
v0.91 WP-01 pass still needs to turn these tracked feature surfaces into the
final issue wave and proof matrix.

That wave should start from an already-promoted package rather than leaving key
workflow or comms-adapter features stranded in TBD or side worktrees.
