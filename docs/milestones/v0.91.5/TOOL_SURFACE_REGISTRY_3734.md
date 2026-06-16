# v0.91.5 Tool Surface Registry

Issue: #3734
Parent sprint: #3732
Captured: 2026-06-16
Status: generated_live_registry

## Summary

This registry records the current visible ADL toolkit surface for the
toolkit-simplification mini-sprint. It is a live inventory intended to
drive later simplification cuts without confusing primary workflow
entrypoints, supported compatibility shims, internal helpers, and
historical evidence artifacts.

This registry distinguishes active workflow spines from already-retired
wrapper residue so later cuts can stay evidence-bound.

## Generation Command

```bash
python3 adl/tools/generate_tool_surface_registry.py
```

## Freshness Check

```bash
python3 adl/tools/generate_tool_surface_registry.py --check
```

## Classification Counts

- `primary`: 30
- `supported shim`: 9
- `internal helper`: 471
- `historical evidence`: 4
- `archive candidate`: 0
- `remove candidate`: 2

## Owner Binaries

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `adl-csdlc` | owner binary | primary | Primary C-SDLC owner binary for workflow control-plane and prompt-card tooling. |
| `adl-runtime` | owner binary | primary | Primary runtime owner binary for execution, providers, demos, agents, and instrumentation. |
| `adl-review` | owner binary | primary | Primary review owner binary for review tooling and packet verification. |
| `adl-validate-structured-prompt` | owner binary | primary | Direct validator binary for structured prompt validation. |
| `adl-lint-prompt-spec` | owner binary | primary | Direct validator binary for Prompt Spec lint. |
| `adl-prompt-template` | owner binary | primary | Direct editor/renderer binary for prompt-template operations. |
| `adl-remote` | owner binary | primary | Primary remote workflow binary for remote execution surfaces. |
| `adl-provider-adapter` | owner binary | primary | Primary provider adapter utility for provider invocation surfaces. |

## Core Workflow Commands

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `adl/tools/pr.sh init <issue>` | core workflow command | primary | Canonical issue bootstrap command in the current governed workflow spine. |
| `adl/tools/pr.sh doctor <issue>` | core workflow command | primary | Canonical readiness/doctor command for tracked issue work. |
| `adl/tools/pr.sh run <issue>` | core workflow command | primary | Canonical issue binding command for execution-time branch/worktree setup. |
| `adl/tools/pr.sh ready <issue>` | core workflow command | primary | Canonical readiness classification command for tracked issue work. |
| `adl/tools/pr.sh finish <issue>` | core workflow command | primary | Canonical finish/publication command for issue branches. |
| `adl/tools/pr.sh closeout <issue>` | core workflow command | primary | Canonical closeout command after merge or no-PR resolution. |
| `pr-janitor skill / janitor workflow` | core workflow command | primary | Canonical in-flight PR blocker routing surface once a PR has been published. |
| `adl-csdlc tooling prompt-template ...` | core workflow command | primary | Primary direct prompt-template workflow surface after the small-binary split. |
| `adl-validate-structured-prompt --type <kind> --phase <phase> --input <card>` | core workflow command | primary | Primary direct structured-prompt validator surface after the small-binary split. |
| `adl-lint-prompt-spec --input <file>` | core workflow command | primary | Primary direct prompt-spec lint surface after the small-binary split. |

## Tool Surfaces Under `adl/tools`

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `adl/tools/BURST_FINAL_SUMMARY_TEMPLATE.md` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/README.md` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/adl_provider_adapter.rs` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/archive_run_artifacts.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/attach_post_merge_closeout.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/attach_pr_janitor.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/batched_checks.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/benchmark` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |
| `adl/tools/branch_hygiene.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/build_v0911_anrm_trace_dataset.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/burst_continue.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/burst_worktree.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/card_paths.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/card_prompt.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/check_compression_finish_profiles.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_coverage_impact.sh` | tool surface | primary | Primary local coverage-impact gate for risky changed files. |
| `adl/tools/check_csdlc_prompt_editor_browser.js` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_issue_metadata_parity.sh` | tool surface | primary | Primary metadata parity guard for issue records. |
| `adl/tools/check_milestone_closed_issue_sor_truth.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_no_new_legacy_swarm_refs.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_no_tracked_adl_issue_record_residue.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_pr_closing_linkage.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_release_notes_commands.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_release_version_surfaces.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_v090_milestone_state.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/ci_path_policy.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/clean_coverage_artifacts.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/closeout_completed_issue_wave.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/codex_pr.sh` | tool surface | remove candidate | Retired fail-closed wrapper kept only for migration guidance until final deletion. |
| `adl/tools/codexw.sh` | tool surface | remove candidate | Retired fail-closed wrapper kept only for migration guidance until final deletion. |
| `adl/tools/compute_ccc_v0.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/coverage.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/demo_adaptive_godel_loop.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_aee_bounded_adaptation.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_affect_engine.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_affect_godel_vertical_slice.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_codex_ollama_operational_skills.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_cross_workflow_learning.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_d11_signed_remote.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_experiment_prioritization.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_five_command_editing.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_five_command_run.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_godel_hypothesis_engine.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_hitl_editor_review.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_one_command.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_promotion_eval_loop.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_reasoning_graph_affect.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_smoke_v07_story.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_steering_queue_checkpoint.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v086_candidate_selection.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v086_control_path.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v086_fast_slow.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v086_freedom_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v086_review_surface.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_docs_review.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_integrated_runtime.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_lifecycle.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_multi_agent_discussion.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_operator_surface.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_provider_chatgpt.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_provider_http.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_provider_local_ollama.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_provider_mock.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_real_multi_agent_discussion.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_release_review_package.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_resilience_failure.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_restartability.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_review_surface.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_runtime_environment.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_runtime_state.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_shepherd_recovery.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_suite.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0871_trace_runtime.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v087_control_plane.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v087_operational_skills.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v087_provider_portability.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v087_reviewer_package.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v087_shared_obsmem.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v087_suite.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v087_trace_truth.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v088_deep_agents_comparative_proof.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v088_instinct_review_surface.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v088_paper_sonata.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v088_phi_review_surface.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v088_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v088_real_multi_agent_discussion.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v088_review_surface.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v088_temporal_review_surface.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0891_arxiv_manuscript_workflow.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0891_deep_agents_comparative_wave.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0891_five_agent_hey_jude.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0891_gemini_provider_harmony_roundtable.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0891_long_lived_stock_league.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0891_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0891_wp13_demo_integration.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_deep_agents_comparative_wave.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_gemini_in_the_loop.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_gemini_provider_harmony_and_economics.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_gemma4_issue_clerk.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_medium_article_writing.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_multi_agent_repo_code_review.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_proof_entrypoints.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v089_review_surface.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0901_csm_observatory.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0901_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0902_multi_agent_repo_review_proof.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0902_paper_sonata_expansion.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0902_review_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v090_architecture_document_generation.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v090_codebuddy_review_showcase.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0911_five_agent_hey_jude_audio.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0911_multiagent_podcast_audio.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0911_multiagent_podcast_pilot.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0912_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0912_speculative_decoding_showcase.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0912_workflow_guardrails_showcase.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_first_proof_demo.py` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_five_minute_sprint_console.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_podcast_studio_v2.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_starharvest_html_game.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0914_codex_only_complete_issue_workcell.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0914_multi_agent_repo_review_serious_proof.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v091_chatgpt_gemini_claude_review_panel.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v091_chatgpt_gemini_claude_triad_conversation.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v091_chatgpt_gemini_direct_conversation.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v091_chatgpt_gemini_task_handoff.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/diagnose_browser_routes.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/editor_action.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/enforce_coverage_gates.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/fill_planning_template.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/fix_git_main_sync.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/fix_git_main_sync_preserve_local_adl.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/generate_active_command_reference_scan.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/generate_podcast_studio_v2_packet.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/generate_tool_surface_registry.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/install_adl_operational_skills.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/install_adl_pr_cycle_skill.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/install_local_authoritative_coverage_prereqs.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/lint_prompt_spec.sh` | tool surface | supported shim | Compatibility shim over the direct prompt-spec lint binary. |
| `adl/tools/local_model_capabilities.v1.json` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/mcp` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |
| `adl/tools/mock_deep_agents_follow_on_provider.py` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/mock_deep_agents_wave_provider.py` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/mock_gemini_http_provider.py` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/mock_gemini_provider_roundtable.py` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/mock_hey_jude_ensemble_provider.py` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/mock_multi_agent_discussion_provider.py` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/mock_ollama_fail_once.sh` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/mock_ollama_v0_4.sh` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/normalize_adl_cards.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/observability.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/open_artifact.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/plan_multi_agent_workcell.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/pr.sh` | tool surface | supported shim | Canonical agent-facing issue workflow wrapper; implementation owner may move but wrapper remains public workflow spine. |
| `adl/tools/preflight_review.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/provider_demo_common.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/real_chatgpt_gemini_claude_provider_adapter.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/real_chatgpt_gemini_provider_adapter.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/real_multi_agent_provider_adapter.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/release_ceremony.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/render_csm_observatory_report.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/render_v0904_contract_market_summary.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/report_large_rust_modules.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/report_module_navigability.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/review_card_surface.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/run_anrm_gemma_shepherd_trials.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_authoritative_coverage_lane.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_local_authoritative_coverage_gate.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_owner_validation_lane.sh` | tool surface | primary | Primary focused owner-lane runner for bounded validation. |
| `adl/tools/run_pr_fast_test_lane.sh` | tool surface | primary | Primary focused PR-fast local test runner. |
| `adl/tools/run_pvf_ci_release_policy_fixture.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_pvf_validation_lane.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_uts_benchmark.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_v0904_contract_market_runner.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_v0913_proof_validation_lane.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_v0914_multi_agent_workcell_proof.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_v0915_multi_agent_quality_comparison.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_v0915_openrouter_matrix.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_v0915_remote_gemma_watcher_probe.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/stock_league_demo.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/summarize_ci_runtime.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/summarize_nextest_timings.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/sync_task_bundle_prompts.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/test_adl_milestone_creator_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_adl_review_compatibility.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_adl_runtime_compatibility.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_adr_curator_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_architecture_diagram_reviewer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_architecture_fitness_function_author_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_archive_run_artifacts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_arxiv_paper_writer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_attach_post_merge_closeout.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_batched_checks_no_codexpr_usage_banner.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_build_v0911_anrm_trace_dataset.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_card_editor_repair_examples.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_card_editor_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_card_paths.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_card_prompt.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_card_reviewer_fixture.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ccc_v0_instrumentation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_coverage_impact.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_issue_metadata_parity.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_milestone_closed_issue_sor_truth.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_no_tracked_adl_issue_record_residue.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_v090_milestone_state.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ci_cache_linker_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ci_path_policy.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ci_runtime_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_cli_owner_command_guidance.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_cli_wrapper_migration_contract.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_closeout_completed_issue_wave.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_completed_output_validation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_control_plane_observability.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_csdlc_demo_proof_contract_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_csdlc_prompt_editor.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_csm_operator_command_packets.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_csm_visibility_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_codex_ollama_operational_skills.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_codex_ollama_semantic_fallback.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_five_command_editing.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_operator_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_multi_agent_discussion.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_operator_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_provider_chatgpt.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_provider_http.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_provider_local_ollama.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_provider_mock.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_provider_parallel.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_real_multi_agent_discussion.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_review_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_review_tail_rows.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_runtime_rows.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_runtime_state.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0871_suite.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v088_deep_agents_comparative_proof.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v088_instinct_review_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v088_paper_sonata.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v088_phi_review_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v088_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v088_real_multi_agent_discussion.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v088_review_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v088_temporal_review_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0891_deep_agents_comparative_wave.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0891_five_agent_hey_jude.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0891_gemini_provider_harmony_roundtable.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0891_long_lived_stock_league.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0891_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0891_wp13_demo_integration.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_deep_agents_comparative_wave.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_gemini_in_the_loop.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_gemini_provider_harmony_and_economics.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_gemma4_issue_clerk.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_medium_article_writing.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_multi_agent_repo_code_review.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_proof_entrypoints.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v089_review_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0901_csm_observatory_operator_report.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0901_csm_observatory_static_console.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0901_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0902_arxiv_writer_field_test.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0902_multi_agent_repo_review_proof.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0902_paper_sonata_expansion.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0902_review_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v090_architecture_document_generation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v090_codebuddy_review_showcase.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0911_five_agent_hey_jude_audio.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0912_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0913_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0914_codex_only_complete_issue_workcell.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0914_multi_agent_repo_review_serious_proof.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_diagram_author_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_documentation_specialist_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_editor_action.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_evidence_bundle_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_finding_to_issue_planner_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_first_proof_demo_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_first_proof_readiness_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_five_command_editor_truth.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_five_command_regression_suite.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_five_minute_html_game_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_five_minute_sprint_console_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_fix_git_main_sync_preserves_local_adl_cards.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_gap_analysis_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_generate_active_command_reference_scan.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_generate_tool_surface_registry.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_hosted_benchmark_hardening.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_install_adl_operational_skills.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_install_adl_pr_cycle_skill.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_issue_folding_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_issue_splitter_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_issue_watcher_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_medium_article_writer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_merge_readiness_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_milestone_dashboard.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_module_navigability_report.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_multi_agent_repo_review_specialist_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_multiagent_podcast_audio_policy.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_normalize_adl_cards.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_obsmem_handoff_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_owner_validation_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_plan_multi_agent_workcell.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_planning_doc_editor_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_podcast_studio_v2_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_portable_contract_normalizer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_cards_context_defaults.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_cards_primary_root.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_closeout_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_closing_linkage.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_create.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_delegate_exit_status.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_doctor_prefers_built_binary.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_finish_default_stage_root.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_finish_delegates_to_rust.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_finish_relative_card_paths.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_help_and_card_open.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_init.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_init_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_issue_version_inference.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_json_observability.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_locking.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_no_fetch_offline.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_ready_prefers_built_binary.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_run.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_run_ambiguity_policy.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_run_issue_mode.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_run_materializes_worktree_cards.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_stack_manager_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_start_template_validation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pr_start_worktree_safe.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_process_drift_regressions.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_product_report_writer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_prompt_spec_lint.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_prompt_template_structure_schemas.py` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_prompt_template_workflow_integration.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_prompt_templates_1_0_0.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_provider_demo_common.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pvf_ci_release_policy.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_records_hygiene_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_redaction_and_evidence_auditor_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_refactoring_helper_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_release_ceremony.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_release_evidence_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_repo_architecture_review_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_repo_code_review_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_repo_dependency_review_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_repo_diagram_planner_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_repo_packet_builder_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_repo_review_contract.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_card_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_comment_triage_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_output_provenance.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_quality_evaluator_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_readiness_cleanup_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_to_test_planner_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_authoritative_coverage_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_local_authoritative_coverage_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_pr_fast_test_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_pvf_validation_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_v0913_proof_validation_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_shell_wrapper_inventory.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_skill_documentation_completeness.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_slow_proof_lane_contract.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_software_development_polis_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_sprint_conductor_helpers.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_structured_prompt_validation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_summarize_ci_runtime.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_summarize_nextest_timings.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_sync_task_bundle_prompts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_test_generator_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_transition_dag_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_use_case_writer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_uts_benchmark_entrypoint_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_uts_benchmark_runner_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v086_demo_review_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0904_contract_market_fixture_set.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0904_contract_market_runner.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0904_contract_market_summary.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0914_csdlc_evidence_bundle.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0914_merge_readiness_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0914_obsmem_transition_memory.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0915_multi_agent_quality_comparison.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0915_openrouter_matrix.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0915_remote_gemma_watcher_probe.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_validate_multi_agent_workcell_state.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_validate_structured_prompt_parallel.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_workflow_conductor_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_workflow_guardrails.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_worktree_doctor.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_worktree_prune.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_wp17a_demo_follow_ons.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/update_latest_reports.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/update_reports_index.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/uts_benchmark_runner.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/v0871_demo_common.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/validate_architecture_docs.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_codebuddy_review_showcase_demo.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csdlc_demo_proof_contract_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csm_governed_observatory.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csm_operator_command_packets.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csm_static_console.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csm_visibility_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_evidence_bundle_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_first_proof_demo_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_first_proof_readiness_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_five_agent_hey_jude_audio_demo.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_five_agent_music_demo.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_five_minute_html_game_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_five_minute_sprint_console_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_merge_readiness_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_multi_agent_repo_review_demo.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_multi_agent_transcript.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_multi_agent_workcell_state.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_obsmem_handoff_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_planning_template.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_podcast_studio_v2_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_pvf_manifest.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_skill_frontmatter.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_software_development_polis_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_structured_prompt.sh` | tool surface | supported shim | Compatibility shim over the direct structured-prompt validator binary. |
| `adl/tools/validate_transition_dag_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v086_control_path.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0902_multi_agent_repo_review_proof.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0902_paper_sonata_expansion.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0913_quality_gate_review_surfaces.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0914_csdlc_evidence_bundle.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0914_merge_readiness_gate.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0914_multi_agent_repo_review_serious_proof.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0914_obsmem_transition_memory.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0915_multi_agent_quality_comparison.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0915_openrouter_matrix.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0915_remote_gemma_watcher_probe.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/verify_repo_review_contract.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/verify_review_output_provenance.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/workflow_guardrails.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/worktree_doctor.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/worktree_prune.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |

## ADL Skills

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `adl/tools/skills/adl-milestone-creator` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/adr-curator` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/architecture-diagram-reviewer` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/architecture-fitness-function-author` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/arxiv-paper-writer` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/demo-operator` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/diagram-author` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/documentation-specialist` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/finding-to-issue-planner` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/gap-analysis` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/issue-folding` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/issue-splitter` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/issue-watcher` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/medium-article-writer` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/planning-doc-editor` | skill | supported shim | Lifecycle editor skill used to normalize durable card truth. |
| `adl/tools/skills/portable-contract-normalizer` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/pr-closeout` | skill | primary | Primary skill in the governed issue/sprint lifecycle. |
| `adl/tools/skills/pr-finish` | skill | primary | Primary skill in the governed issue/sprint lifecycle. |
| `adl/tools/skills/pr-init` | skill | primary | Primary skill in the governed issue/sprint lifecycle. |
| `adl/tools/skills/pr-janitor` | skill | primary | Primary skill in the governed issue/sprint lifecycle. |
| `adl/tools/skills/pr-ready` | skill | primary | Primary skill in the governed issue/sprint lifecycle. |
| `adl/tools/skills/pr-run` | skill | primary | Primary skill in the governed issue/sprint lifecycle. |
| `adl/tools/skills/pr-stack-manager` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/product-report-writer` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/records-hygiene` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/redaction-and-evidence-auditor` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/refactoring-helper` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/release-evidence` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-architecture-review` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-code-review` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-dependency-review` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-diagram-planner` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-packet-builder` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-review-code` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-review-docs` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-review-security` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-review-synthesis` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/repo-review-tests` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/review-comment-triage` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/review-quality-evaluator` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/review-readiness-cleanup` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/review-to-test-planner` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/sip-editor` | skill | supported shim | Lifecycle editor skill used to normalize durable card truth. |
| `adl/tools/skills/sor-editor` | skill | supported shim | Lifecycle editor skill used to normalize durable card truth. |
| `adl/tools/skills/spp-editor` | skill | supported shim | Lifecycle editor skill used to normalize durable card truth. |
| `adl/tools/skills/sprint-conductor` | skill | primary | Primary skill in the governed issue/sprint lifecycle. |
| `adl/tools/skills/srp-editor` | skill | supported shim | Lifecycle editor skill used to normalize durable card truth. |
| `adl/tools/skills/stp-editor` | skill | supported shim | Lifecycle editor skill used to normalize durable card truth. |
| `adl/tools/skills/test-generator` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/use-case-writer` | skill | internal helper | Specialized skill available for bounded tasks but not part of the primary issue workflow spine. |
| `adl/tools/skills/workflow-conductor` | skill | primary | Primary skill in the governed issue/sprint lifecycle. |

## Historical Evidence Artifacts

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | historical evidence | historical evidence | Historical command-inventory evidence and prior ownership table. |
| `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md` | historical evidence | historical evidence | Historical migration contract for wrapper/public workflow spine truth. |
| `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | historical evidence | historical evidence | Historical compatibility-sunset policy and active-bundle scan gate. |
| `docs/milestones/v0.91.5/review/tooling_adoption/CSDLC_SMALL_BINARIES_PROOF_3832.md` | historical evidence | historical evidence | Historical proof packet for the validator/editor small-binary slice. |

## Non-Claims

- This registry does not approve final deletion of retired wrappers.
- This registry does not prove active-reference absence for any shim.
- This registry does not replace the later simplification review and scan issues.
