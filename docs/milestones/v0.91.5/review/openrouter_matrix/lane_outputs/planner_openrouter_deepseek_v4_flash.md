# planner_openrouter_deepseek_v4_flash

Role: `planner`
Provider: `openrouter`
Model: `deepseek/deepseek-v4-flash`

## Output

# Plan
For issue #3723, we will integrate the provider model matrix from `docs/milestones/v0.91.5/features/PROVIDER_MODEL_MATRIX_v0.91.5.md` into the ADL pipeline. This involves mapping all listed providers and models to bounded action definitions, ensuring consistent naming and parameter constraints.

# Risks
- Incomplete or outdated matrix entries may cause mismatches with live API endpoints.
- Performance overhead if matrix size grows beyond current bounded limits.

# Non-Claims
- no universal OpenRouter compatibility claim
