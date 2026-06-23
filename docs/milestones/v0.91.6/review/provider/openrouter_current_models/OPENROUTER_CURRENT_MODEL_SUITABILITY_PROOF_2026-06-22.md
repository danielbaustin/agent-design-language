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
| `openrouter:claude-opus-4.8` | `openrouter` / `anthropic/claude-opus-4.8` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass_with_limits` | `pass` | `useful_with_limits` |
| `openrouter:gpt-5.4` | `openrouter` / `openai/gpt-5.4` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass_with_limits` | `pass` | `useful_with_limits` |
| `openrouter:glm-5.2` | `openrouter` / `z-ai/glm-5.2` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass_with_limits` | `pass` | `useful_with_limits` |
| `openrouter:kimi-k2.7-code` | `openrouter` / `moonshotai/kimi-k2.7-code` | `pass_with_limits` | `pass` | `fail_format` | `fail_format` | `pass_with_limits` | `pass` | `candidate_only_format_repair_needed` |
| `openrouter:gemini-3.5-flash` | `openrouter` / `google/gemini-3.5-flash` | `pass_with_limits` | `pass_with_limits` | `pass` | `pass` | `pass_with_limits` | `pass` | `useful_with_limits` |

## Candidate descriptors

### `openrouter:claude-opus-4.8`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:anthropic/claude-opus-4.8`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker, worker
- Recommendation: `useful_with_limits`

### `openrouter:gpt-5.4`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:openai/gpt-5.4`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker, worker
- Recommendation: `useful_with_limits`

### `openrouter:glm-5.2`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:z-ai/glm-5.2`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker, worker
- Recommendation: `useful_with_limits`

### `openrouter:kimi-k2.7-code`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:moonshotai/kimi-k2.7-code`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: watcher, card_validator, closeout_checker, worker
- Recommendation: `candidate_only_format_repair_needed`

### `openrouter:gemini-3.5-flash`

- Lane: `openrouter`
- Provider profile ref: `unprofiled:openrouter:google/gemini-3.5-flash`
- Provider family: `openrouter`
- Provider spec kind: `openrouter`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openrouter.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker, worker
- Recommendation: `useful_with_limits`

## Per-task evidence

| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `openrouter:claude-opus-4.8` | `watcher_state_v1` | `pass_with_limits` | 3726 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-opus-4.8__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-opus-4.8__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-opus-4.8__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openrouter:claude-opus-4.8` | `card_validator_v1` | `pass` | 4415 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-opus-4.8__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-opus-4.8__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-opus-4.8__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openrouter:claude-opus-4.8` | `review_findings_v1` | `pass` | 8324 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-opus-4.8__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-opus-4.8__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-opus-4.8__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openrouter:claude-opus-4.8` | `bounded_planner_v1` | `pass` | 7972 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-opus-4.8__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-opus-4.8__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-opus-4.8__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openrouter:claude-opus-4.8` | `closeout_checker_v1` | `pass_with_limits` | 5489 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-opus-4.8__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-opus-4.8__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-opus-4.8__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |
| `openrouter:claude-opus-4.8` | `worker_contract_v1` | `pass` | 4220 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:claude-opus-4.8__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:claude-opus-4.8__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:claude-opus-4.8__worker_contract_v1.jsonl` | worker output returned a bounded structured task contract |
| `openrouter:gpt-5.4` | `watcher_state_v1` | `pass_with_limits` | 2063 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openrouter:gpt-5.4` | `card_validator_v1` | `pass` | 2162 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openrouter:gpt-5.4` | `review_findings_v1` | `pass` | 2888 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openrouter:gpt-5.4` | `bounded_planner_v1` | `pass` | 3191 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openrouter:gpt-5.4` | `closeout_checker_v1` | `pass_with_limits` | 1937 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |
| `openrouter:gpt-5.4` | `worker_contract_v1` | `pass` | 1434 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gpt-5.4__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gpt-5.4__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gpt-5.4__worker_contract_v1.jsonl` | worker output returned a bounded structured task contract |
| `openrouter:glm-5.2` | `watcher_state_v1` | `pass_with_limits` | 32584 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openrouter:glm-5.2` | `card_validator_v1` | `pass` | 26317 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openrouter:glm-5.2` | `review_findings_v1` | `pass` | 10540 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openrouter:glm-5.2` | `bounded_planner_v1` | `pass` | 11295 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openrouter:glm-5.2` | `closeout_checker_v1` | `pass_with_limits` | 6336 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |
| `openrouter:glm-5.2` | `worker_contract_v1` | `pass` | 10036 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:glm-5.2__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:glm-5.2__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:glm-5.2__worker_contract_v1.jsonl` | worker output returned a bounded structured task contract |
| `openrouter:kimi-k2.7-code` | `watcher_state_v1` | `pass_with_limits` | 20390 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openrouter:kimi-k2.7-code` | `card_validator_v1` | `pass` | 9776 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openrouter:kimi-k2.7-code` | `review_findings_v1` | `fail_format` | 40614 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__review_findings_v1.jsonl` | review output missed the required headings |
| `openrouter:kimi-k2.7-code` | `bounded_planner_v1` | `fail_format` | 14564 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__bounded_planner_v1.jsonl` | planner output missed the required headings |
| `openrouter:kimi-k2.7-code` | `closeout_checker_v1` | `pass_with_limits` | 3754 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |
| `openrouter:kimi-k2.7-code` | `worker_contract_v1` | `pass` | 1942 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:kimi-k2.7-code__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:kimi-k2.7-code__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:kimi-k2.7-code__worker_contract_v1.jsonl` | worker output returned a bounded structured task contract |
| `openrouter:gemini-3.5-flash` | `watcher_state_v1` | `pass_with_limits` | 8284 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openrouter:gemini-3.5-flash` | `card_validator_v1` | `pass_with_limits` | 5217 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openrouter:gemini-3.5-flash` | `review_findings_v1` | `pass` | 9079 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openrouter:gemini-3.5-flash` | `bounded_planner_v1` | `pass` | 8338 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openrouter:gemini-3.5-flash` | `closeout_checker_v1` | `pass_with_limits` | 4699 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |
| `openrouter:gemini-3.5-flash` | `worker_contract_v1` | `pass` | 6108 | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_outputs/openrouter:gemini-3.5-flash__worker_contract_v1.md` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_results/openrouter:gemini-3.5-flash__worker_contract_v1.json` | `docs/milestones/v0.91.6/review/provider/openrouter_current_models/lane_logs/openrouter:gemini-3.5-flash__worker_contract_v1.jsonl` | worker output returned a bounded structured task contract |

## Findings

- `openrouter:claude-opus-4.8` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker, worker`.
- `openrouter:gpt-5.4` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker, worker`.
- `openrouter:glm-5.2` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker, worker`.
- `openrouter:kimi-k2.7-code` is `candidate_only_format_repair_needed` for the bounded panel, based on task scores `watcher, card_validator, closeout_checker, worker`.
- `openrouter:gemini-3.5-flash` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker, worker`.

## Non-claims

- This packet does not prove native OpenAI, Anthropic, Gemini, Moonshot, or Z.ai suitability outside the OpenRouter route.
- This packet does not prove broad model superiority or general intelligence.
- This packet does not grant any tested OpenRouter lane merge, closeout, release, or repo-mutation authority.

