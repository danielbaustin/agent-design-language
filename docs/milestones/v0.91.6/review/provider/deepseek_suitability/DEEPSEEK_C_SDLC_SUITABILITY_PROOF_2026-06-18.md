# DeepSeek C-SDLC Suitability Proof

Date: 2026-06-18

Issues: `#4096`, `#4095`

## Scope

This packet instantiates the reusable C-SDLC suitability panel for DeepSeek across one hosted native lane and the available local Ollama DeepSeek lanes.

It records bounded advisory-role evidence only. It does not grant merge, closeout, release, or repo-mutation authority.

## Source evidence

- `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`
- `docs/milestones/v0.91.5/review/native_deepseek_provider/DEEPSEEK_NATIVE_PROVIDER_PROOF_3549.md`

## Candidate matrix

| Candidate | Lane | Watcher | Card validator | Reviewer | Planner | Closeout checker | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `deepseek_api:deepseek-chat` | `hosted_api` / `deepseek-chat` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass_with_limits` | `useful_with_limits` |
| `ollama:deepseek-r1:8b` | `local_ollama` / `deepseek-r1:8b` | `pass` | `pass_with_limits` | `fail_truth` | `pass_with_limits` | `fail_truth` | `candidate_only_truth_repair_needed` |
| `ollama:deepseek-r1:32b` | `local_ollama` / `deepseek-r1:32b` | `pass_with_limits` | `pass` | `pass` | `pass_with_limits` | `fail_truth` | `candidate_only_truth_repair_needed` |

## Candidate descriptors

### `deepseek_api:deepseek-chat`

- Lane: `hosted_api`
- Provider profile ref: `unprofiled:hosted_api:deepseek-chat`
- Provider family: `deepseek`
- Provider spec kind: `deepseek`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/deepseek.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

### `ollama:deepseek-r1:8b`

- Lane: `local_ollama`
- Provider profile ref: `unprofiled:local_ollama:deepseek-r1:8b`
- Provider family: `deepseek`
- Provider spec kind: `ollama`
- Runtime surface: `ollama_http`
- Credential source: `none`
- Supported tasks: watcher, card_validator, planner
- Recommendation: `candidate_only_truth_repair_needed`

### `ollama:deepseek-r1:32b`

- Lane: `local_ollama`
- Provider profile ref: `unprofiled:local_ollama:deepseek-r1:32b`
- Provider family: `deepseek`
- Provider spec kind: `ollama`
- Runtime surface: `ollama_http`
- Credential source: `none`
- Supported tasks: watcher, card_validator, reviewer, planner
- Recommendation: `candidate_only_truth_repair_needed`

## Per-task evidence

| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `deepseek_api:deepseek-chat` | `watcher_state_v1` | `pass_with_limits` | 1872 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/deepseek_api:deepseek-chat__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/deepseek_api:deepseek-chat__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/deepseek_api:deepseek-chat__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `deepseek_api:deepseek-chat` | `card_validator_v1` | `pass` | 1538 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/deepseek_api:deepseek-chat__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/deepseek_api:deepseek-chat__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/deepseek_api:deepseek-chat__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `deepseek_api:deepseek-chat` | `review_findings_v1` | `pass` | 2667 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/deepseek_api:deepseek-chat__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/deepseek_api:deepseek-chat__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/deepseek_api:deepseek-chat__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `deepseek_api:deepseek-chat` | `bounded_planner_v1` | `pass` | 2521 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/deepseek_api:deepseek-chat__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/deepseek_api:deepseek-chat__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/deepseek_api:deepseek-chat__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `deepseek_api:deepseek-chat` | `closeout_checker_v1` | `pass_with_limits` | 2158 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/deepseek_api:deepseek-chat__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/deepseek_api:deepseek-chat__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/deepseek_api:deepseek-chat__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |
| `ollama:deepseek-r1:8b` | `watcher_state_v1` | `pass` | 33928 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:8b__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:8b__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:8b__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `ollama:deepseek-r1:8b` | `card_validator_v1` | `pass_with_limits` | 2463 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:8b__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:8b__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:8b__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `ollama:deepseek-r1:8b` | `review_findings_v1` | `fail_truth` | 4161 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:8b__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:8b__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:8b__review_findings_v1.jsonl` | review output did not focus on the supplied evidence-provenance problem |
| `ollama:deepseek-r1:8b` | `bounded_planner_v1` | `pass_with_limits` | 6507 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:8b__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:8b__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:8b__bounded_planner_v1.jsonl` | planner output was usable but missed one or more requested constraints |
| `ollama:deepseek-r1:8b` | `closeout_checker_v1` | `fail_truth` | 3680 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:8b__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:8b__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:8b__closeout_checker_v1.jsonl` | closeout output overclaimed closure readiness against the supplied evidence |
| `ollama:deepseek-r1:32b` | `watcher_state_v1` | `pass_with_limits` | 34158 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:32b__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:32b__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:32b__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `ollama:deepseek-r1:32b` | `card_validator_v1` | `pass` | 8929 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:32b__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:32b__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:32b__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `ollama:deepseek-r1:32b` | `review_findings_v1` | `pass` | 12865 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:32b__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:32b__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:32b__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `ollama:deepseek-r1:32b` | `bounded_planner_v1` | `pass_with_limits` | 29194 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:32b__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:32b__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:32b__bounded_planner_v1.jsonl` | planner output was usable but missed one or more requested constraints |
| `ollama:deepseek-r1:32b` | `closeout_checker_v1` | `fail_truth` | 14345 | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_outputs/ollama:deepseek-r1:32b__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_results/ollama:deepseek-r1:32b__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/deepseek_suitability/lane_logs/ollama:deepseek-r1:32b__closeout_checker_v1.jsonl` | closeout output overclaimed closure readiness against the supplied evidence |

## Findings

- `deepseek_api:deepseek-chat` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.
- `ollama:deepseek-r1:8b` is `candidate_only_truth_repair_needed` for the bounded panel, based on task scores `watcher, card_validator, planner`.
- `ollama:deepseek-r1:32b` is `candidate_only_truth_repair_needed` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner`.

## Non-claims

- This packet does not prove broad DeepSeek coding quality or benchmark superiority.
- This packet does not generalize beyond the exact hosted and local lanes named here.
- This packet does not grant external or local models workflow authority.

