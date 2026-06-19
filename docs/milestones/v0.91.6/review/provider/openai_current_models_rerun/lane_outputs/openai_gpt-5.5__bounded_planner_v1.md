# openai:gpt-5.5 / bounded_planner_v1

Candidate: `openai:gpt-5.5`
Lane: `openai`
Model: `gpt-5.5`

## Output

# Plan
1. Scope issue #4155 to direct-hosted OpenAI lanes covering gpt-5.5 and gpt-5.4 only.
2. Define one shared five-task suitability panel, reused unchanged across both model lanes.
3. Execute/record each lane against the panel; skipped or blocked lanes must be recorded explicitly.
4. Summarize results, deltas, and lane status without changing repository state.

# Blockers
- Missing credentials, endpoint access, or model availability blocks execution.
- Ambiguous issue criteria blocks final suitability judgment.

# Assumptions
- Issue #4155 concerns evaluation/planning only.
- The same prompts, scoring, and environment apply to both lanes.

# Non-Claims
- No repo-mutation authority.
