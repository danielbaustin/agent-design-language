# anthropic:claude-opus-4-8 / bounded_planner_v1

Candidate: `anthropic:claude-opus-4-8`
Lane: `anthropic`
Model: `claude-opus-4-8`

## Output

# Plan

1. Provision direct-hosted Anthropic access for claude-opus-4-8, claude-sonnet-4-6, and claude-haiku-4-5, verifying connectivity per model lane.
2. Run all three models against one shared five-task suitability panel under identical inputs.
3. Score each model per task; record any skipped or blocked lanes explicitly with their reasons.
4. Compile comparative results and recommend a model selection for issue #4156.

# Blockers

- Direct-hosted Anthropic credentials or quota may be unavailable for any model lane.
- Suitability panel task definitions must be finalized before execution.

# Assumptions

- The five-task suitability panel is fixed and shared across all three models.
- Direct-hosted access (not prox
