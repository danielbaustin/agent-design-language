# OpenAI Codex and Spark Direct-Hosted Suitability Proof

Date: 2026-06-18

Issues: `#4155`, `#4154`

## Scope

This packet reruns the reusable C-SDLC suitability panel against direct-hosted OpenAI Codex-family model IDs requested by the operator.

It records bounded advisory-role evidence only. It does not grant merge, closeout, release, or repo-mutation authority.

## Source evidence

- `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`

## Candidate matrix

| Candidate | Lane | Watcher | Card validator | Reviewer | Planner | Closeout checker | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `openai:gpt-5-codex` | `openai` / `gpt-5-codex` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass` | `useful_with_limits` |
| `openai:gpt-5.3-codex-spark` | `openai` / `gpt-5.3-codex-spark` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `timeout_or_empty` | `runtime_unsuitable_for_this_panel` |

## Candidate descriptors

### `openai:gpt-5-codex`

- Lane: `openai`
- Provider profile ref: `unprofiled:openai:gpt-5-codex`
- Provider family: `openai`
- Provider spec kind: `openai`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openai2.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

### `openai:gpt-5.3-codex-spark`

- Lane: `openai`
- Provider profile ref: `unprofiled:openai:gpt-5.3-codex-spark`
- Provider family: `openai`
- Provider spec kind: `openai`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openai2.key`
- Supported tasks: none
- Recommendation: `runtime_unsuitable_for_this_panel`

## Per-task evidence

| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `openai:gpt-5-codex` | `watcher_state_v1` | `pass_with_limits` | 2790 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5-codex__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5-codex__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5-codex__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openai:gpt-5-codex` | `card_validator_v1` | `pass` | 1603 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5-codex__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5-codex__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5-codex__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openai:gpt-5-codex` | `review_findings_v1` | `pass` | 6562 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5-codex__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5-codex__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5-codex__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openai:gpt-5-codex` | `bounded_planner_v1` | `pass` | 3112 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5-codex__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5-codex__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5-codex__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openai:gpt-5-codex` | `closeout_checker_v1` | `pass` | 1697 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5-codex__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5-codex__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5-codex__closeout_checker_v1.jsonl` | closeout output correctly withheld closure until merge/proof evidence exists |
| `openai:gpt-5.3-codex-spark` | `watcher_state_v1` | `timeout_or_empty` | 251 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5.3-codex-spark__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5.3-codex-spark__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5.3-codex-spark__watcher_state_v1.jsonl` | provider returned `model_not_found` for the requested model id |
| `openai:gpt-5.3-codex-spark` | `card_validator_v1` | `timeout_or_empty` | 5255 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5.3-codex-spark__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5.3-codex-spark__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5.3-codex-spark__card_validator_v1.jsonl` | provider returned `model_not_found` for the requested model id |
| `openai:gpt-5.3-codex-spark` | `review_findings_v1` | `timeout_or_empty` | 5253 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5.3-codex-spark__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5.3-codex-spark__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5.3-codex-spark__review_findings_v1.jsonl` | provider returned `model_not_found` for the requested model id |
| `openai:gpt-5.3-codex-spark` | `bounded_planner_v1` | `timeout_or_empty` | 1308 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5.3-codex-spark__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5.3-codex-spark__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5.3-codex-spark__bounded_planner_v1.jsonl` | provider returned `model_not_found` for the requested model id |
| `openai:gpt-5.3-codex-spark` | `closeout_checker_v1` | `timeout_or_empty` | 5213 | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_outputs/openai_gpt-5.3-codex-spark__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_results/openai_gpt-5.3-codex-spark__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/lane_logs/openai_gpt-5.3-codex-spark__closeout_checker_v1.jsonl` | provider returned `model_not_found` for the requested model id |

## Findings

- `openai:gpt-5-codex` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.
- `openai:gpt-5.3-codex-spark` is `runtime_unsuitable_for_this_panel` for the bounded panel because the direct-hosted API returned `model_not_found` for every requested task.

## Non-claims

- This packet does not prove general OpenAI model quality outside the bounded panel tasks.
- This packet does not equate direct-hosted OpenAI success with Codex shell authority.
- This packet does not grant any tested lane merge, closeout, release, or repo-mutation authority.
