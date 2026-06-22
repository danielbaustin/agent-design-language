# OpenRouter Current-Model Suitability Proof

Date: 2026-06-22

Issues: `#4429`

## Scope

This packet instantiates the reusable C-SDLC suitability panel for five current OpenRouter routes selected from the live catalog on 2026-06-22.

It records bounded advisory-role evidence only. It does not grant merge, closeout, release, or repo-mutation authority.

## Source evidence

- `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`
- `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md`

## Candidate matrix

| Candidate | Lane | Watcher | Card validator | Reviewer | Planner | Closeout checker | Worker | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `openrouter:claude-fable-5` | `openrouter` / `anthropic/claude-fable-5` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `runtime_unsuitable_for_this_panel` |
| `openrouter:gpt-5.4` | `openrouter` / `openai/gpt-5.4` | `pass_with_limits` | `pass` | `pass` | `fail_format` | `pass` | `pass` | `candidate_only_format_repair_needed` |
| `openrouter:glm-5.2` | `openrouter` / `z-ai/glm-5.2` | `fail_format` | `pass` | `timeout_or_empty` | `pass` | `timeout_or_empty` | `fail_format` | `candidate_only_format_repair_needed` |
| `openrouter:kimi-k2.7-code` | `openrouter` / `moonshotai/kimi-k2.7-code` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `pass` | `runtime_unsuitable_for_this_panel` |
| `openrouter:gemini-3.5-flash` | `openrouter` / `google/gemini-3.5-flash` | `fail_truth` | `fail_format` | `fail_format` | `fail_format` | `fail_format` | `fail_format` | `candidate_only_truth_repair_needed` |

## Candidate descriptors

### `openrouter:claude-fable-5`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:anthropic/claude-fable-5`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: none
- Recommendation: `runtime_unsuitable_for_this_panel`

### `openrouter:gpt-5.4`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:openai/gpt-5.4`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: watcher, card_validator, reviewer, closeout_checker, worker
- Recommendation: `candidate_only_format_repair_needed`

### `openrouter:glm-5.2`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:z-ai/glm-5.2`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: card_validator, planner
- Recommendation: `candidate_only_format_repair_needed`

### `openrouter:kimi-k2.7-code`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:moonshotai/kimi-k2.7-code`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: worker
- Recommendation: `runtime_unsuitable_for_this_panel`

### `openrouter:gemini-3.5-flash`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:google/gemini-3.5-flash`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: none
- Recommendation: `candidate_only_truth_repair_needed`

## Per-task evidence

| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `openrouter:claude-fable-5` | `watcher_state_v1` | `timeout_or_empty` | 2388 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-fable-5__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-fable-5__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-fable-5__watcher_state_v1.jsonl` | provider returned empty output |
| `openrouter:claude-fable-5` | `card_validator_v1` | `timeout_or_empty` | 2190 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-fable-5__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-fable-5__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-fable-5__card_validator_v1.jsonl` | provider returned empty output |
| `openrouter:claude-fable-5` | `review_findings_v1` | `timeout_or_empty` | 1804 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-fable-5__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-fable-5__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-fable-5__review_findings_v1.jsonl` | provider returned empty output |
| `openrouter:claude-fable-5` | `bounded_planner_v1` | `timeout_or_empty` | 1415 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-fable-5__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-fable-5__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-fable-5__bounded_planner_v1.jsonl` | provider returned empty output |
| `openrouter:claude-fable-5` | `closeout_checker_v1` | `timeout_or_empty` | 1843 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-fable-5__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-fable-5__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-fable-5__closeout_checker_v1.jsonl` | provider returned empty output |
| `openrouter:claude-fable-5` | `worker_contract_v1` | `timeout_or_empty` | 1516 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-fable-5__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-fable-5__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-fable-5__worker_contract_v1.jsonl` | provider returned empty output |
| `openrouter:gpt-5.4` | `watcher_state_v1` | `pass_with_limits` | 2705 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openrouter:gpt-5.4` | `card_validator_v1` | `pass` | 1924 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openrouter:gpt-5.4` | `review_findings_v1` | `pass` | 2700 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openrouter:gpt-5.4` | `bounded_planner_v1` | `fail_format` | 3945 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__bounded_planner_v1.jsonl` | planner output missed the required headings |
| `openrouter:gpt-5.4` | `closeout_checker_v1` | `pass` | 1538 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__closeout_checker_v1.jsonl` | closeout output correctly withheld closure until merge/proof evidence exists |
| `openrouter:gpt-5.4` | `worker_contract_v1` | `pass` | 1612 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__worker_contract_v1.jsonl` | worker output returned a bounded structured task contract |
| `openrouter:glm-5.2` | `watcher_state_v1` | `fail_format` | 3326 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__watcher_state_v1.jsonl` | watcher output missed the bounded status contract |
| `openrouter:glm-5.2` | `card_validator_v1` | `pass` | 8292 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openrouter:glm-5.2` | `review_findings_v1` | `timeout_or_empty` | 5407 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__review_findings_v1.jsonl` | provider returned empty output |
| `openrouter:glm-5.2` | `bounded_planner_v1` | `pass` | 19895 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openrouter:glm-5.2` | `closeout_checker_v1` | `timeout_or_empty` | 5229 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__closeout_checker_v1.jsonl` | provider returned empty output |
| `openrouter:glm-5.2` | `worker_contract_v1` | `fail_format` | 8330 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__worker_contract_v1.jsonl` | worker output did not return parseable JSON |
| `openrouter:kimi-k2.7-code` | `watcher_state_v1` | `timeout_or_empty` | 6179 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__watcher_state_v1.jsonl` | provider returned empty output |
| `openrouter:kimi-k2.7-code` | `card_validator_v1` | `timeout_or_empty` | 2886 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__card_validator_v1.jsonl` | provider returned empty output |
| `openrouter:kimi-k2.7-code` | `review_findings_v1` | `timeout_or_empty` | 9459 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__review_findings_v1.jsonl` | provider returned empty output |
| `openrouter:kimi-k2.7-code` | `bounded_planner_v1` | `timeout_or_empty` | 4857 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__bounded_planner_v1.jsonl` | provider returned empty output |
| `openrouter:kimi-k2.7-code` | `closeout_checker_v1` | `timeout_or_empty` | 11736 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__closeout_checker_v1.jsonl` | provider returned empty output |
| `openrouter:kimi-k2.7-code` | `worker_contract_v1` | `pass` | 3894 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__worker_contract_v1.jsonl` | worker output returned a bounded structured task contract |
| `openrouter:gemini-3.5-flash` | `watcher_state_v1` | `fail_truth` | 1977 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__watcher_state_v1.jsonl` | watcher output did not anchor itself in the supplied issue/worktree facts |
| `openrouter:gemini-3.5-flash` | `card_validator_v1` | `fail_format` | 2645 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__card_validator_v1.jsonl` | card-validator output missed the required findings structure |
| `openrouter:gemini-3.5-flash` | `review_findings_v1` | `fail_format` | 2123 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__review_findings_v1.jsonl` | review output missed the required headings |
| `openrouter:gemini-3.5-flash` | `bounded_planner_v1` | `fail_format` | 2148 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__bounded_planner_v1.jsonl` | planner output missed the required headings |
| `openrouter:gemini-3.5-flash` | `closeout_checker_v1` | `fail_format` | 2187 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__closeout_checker_v1.jsonl` | closeout output missed the required headings |
| `openrouter:gemini-3.5-flash` | `worker_contract_v1` | `fail_format` | 2265 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__worker_contract_v1.jsonl` | worker output did not return parseable JSON |

## Findings

- `openrouter:claude-fable-5` is `runtime_unsuitable_for_this_panel` for the bounded panel, based on task scores `none`.
- `openrouter:gpt-5.4` is `candidate_only_format_repair_needed` for the bounded panel, based on task scores `watcher, card_validator, reviewer, closeout_checker, worker`.
- `openrouter:glm-5.2` is `candidate_only_format_repair_needed` for the bounded panel, based on task scores `card_validator, planner`.
- `openrouter:kimi-k2.7-code` is `runtime_unsuitable_for_this_panel` for the bounded panel, based on task scores `worker`.
- `openrouter:gemini-3.5-flash` is `candidate_only_truth_repair_needed` for the bounded panel, based on task scores `none`.

## Non-claims

- This packet does not prove native OpenAI, Anthropic, Gemini, Moonshot, or Z.ai suitability outside the OpenRouter route.
- This packet does not prove broad model superiority or general intelligence.
- This packet does not grant any tested OpenRouter lane merge, closeout, release, or repo-mutation authority.

