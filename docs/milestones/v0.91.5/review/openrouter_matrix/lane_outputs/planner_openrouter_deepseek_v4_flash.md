# planner_openrouter_deepseek_v4_flash

Role: `planner`
Provider: `openrouter`
Model: `deepseek/deepseek-v4-flash`

## Output

# Plan
For issue #3723, we will finalize the provider model matrix at `docs/milestones/v0.91.5/features/PROVIDER_MODEL_MATRIX_v0.91.5.md`, mapping bounded ADL capabilities to supported providers. This includes verifying model coverage and updating the matrix with v0.91.5 constraints.

# Risks
- Incomplete provider coverage may delay release if new models are added post-freeze.
- Matrix format changes could break downstream tooling expecting the current schema.

# Non-Claims
- no universal OpenRouter compatibility claim
