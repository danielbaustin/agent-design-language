# OpenRouter

- Issue `#3723` is status `supported_with_limits`.
- Five native OpenRouter provider invocations were executed through `adl-provider-adapter` using requested model IDs: `planner_openrouter_deepseek_v4_flash`, `worker_openrouter_gpt4o_mini`, `reviewer_openrouter_claude_3_5_haiku`, `watcher_openrouter_gemini_2_5_flash_lite`, `worker_openrouter_qwen3_6_flash`.
- All five requested route IDs completed successfully and were recorded in the state packet.
- A negative control omitting the required credential produced `provider_auth_missing`, proving fail-closed auth behavior.
- No flaky retry or timeout behavior was observed in this bounded run.
- Prior evidence from `#3415` recorded OpenRouter planner output as useful but generic/off-target in places.

# Remote-Gemma

- The historical empty watcher lane `watcher_remote_gemma4_e2b` is a true historical fact from the `#3415` workcell packet.
- The strongest proving lane is `adapter_gemma4_31b`, which returned reviewer-usable markdown with required watcher headings and the exact phrase `route probe completed` through the real ADL provider adapter surface.
- Secondary useful routes `gemma4:31b`, `gemma4:26b`, and `gemma4:e4b` also returned structured watcher text.
- Remote Gemma watcher usefulness is now proven in a bounded way for short, structured watcher prompts on larger Gemma4 routes.
- This packet does not prove `gemma4:e2b` is universally recovered for the original historical workcell prompt.

# Overhead

- Status is `single_agent_preferred_for_tiny_docs_audit`.
- Multi-agent was `faster only for raw lane execution` (54.495s) compared to single-agent (139s), but single-agent was simpler and produced a focused comparison.
- The multi-agent path produced mixed results: one useful worker, one partial planner, one empty watcher.
- Multi-agent execution is not yet a net speedup claim for tiny docs audits; the `#3415` proof should remain `useful_with_limits`.
- Remote Gemma watcher availability is proven, but role usefulness is not proven for this prompt shape because the output was empty.

# Verdict

**mixed** — Multi-agent now appears stronger for disjoint multi-surface evidence review than for the tiny docs audit. The OpenRouter packet proves `supported_with_limits` for five route IDs with structured execution, and the Remote-Gemma packet proves `adapter_gemma4_31b` as the strongest proving lane for bounded watcher output. However, the overhead comparison shows `single_agent_preferred_for_tiny_docs_audit` for the small audit task, with multi-agent being `faster only for raw lane execution` but requiring human synthesis and producing an empty watcher lane. The evidence is mixed because multi-agent provides better provider/model diversity and reusable substrate, but single-agent remains simpler and more focused for tiny docs audits.
