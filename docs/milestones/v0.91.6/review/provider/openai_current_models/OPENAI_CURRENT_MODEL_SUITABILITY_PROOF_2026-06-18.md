# OpenAI Current Direct-Hosted Suitability Proof

Date: 2026-06-18

Issues: `#4155`, `#4154`

## Scope

This packet runs the reusable C-SDLC suitability panel against current direct-hosted OpenAI models through the native hosted provider path.

It records bounded advisory-role evidence only. It does not grant merge, closeout, release, or repo-mutation authority.

## Source evidence

- `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`

## Candidate matrix

| Candidate | Lane | Watcher | Card validator | Reviewer | Planner | Closeout checker | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `openai:gpt-5.5` | `openai` / `gpt-5.5` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass_with_limits` | `useful_with_limits` |
| `openai:gpt-5.4` | `openai` / `gpt-5.4` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass` | `useful_with_limits` |

## Candidate descriptors

### `openai:gpt-5.5`

- Lane: `openai`
- Provider profile ref: `unprofiled:openai:gpt-5.5`
- Provider family: `openai`
- Provider spec kind: `openai`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openai.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

### `openai:gpt-5.4`

- Lane: `openai`
- Provider profile ref: `unprofiled:openai:gpt-5.4`
- Provider family: `openai`
- Provider spec kind: `openai`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openai.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

## Per-task evidence

| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `openai:gpt-5.5` | `watcher_state_v1` | `pass_with_limits` | 5197 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.5__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.5__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.5__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openai:gpt-5.5` | `card_validator_v1` | `pass` | 3873 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.5__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.5__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.5__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openai:gpt-5.5` | `review_findings_v1` | `pass` | 5257 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.5__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.5__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.5__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openai:gpt-5.5` | `bounded_planner_v1` | `pass` | 6002 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.5__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.5__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.5__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openai:gpt-5.5` | `closeout_checker_v1` | `pass_with_limits` | 3434 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.5__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.5__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.5__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |
| `openai:gpt-5.4` | `watcher_state_v1` | `pass_with_limits` | 1632 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.4__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.4__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.4__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openai:gpt-5.4` | `card_validator_v1` | `pass` | 1701 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.4__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.4__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.4__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openai:gpt-5.4` | `review_findings_v1` | `pass` | 3255 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.4__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.4__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.4__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openai:gpt-5.4` | `bounded_planner_v1` | `pass` | 3381 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.4__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.4__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.4__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openai:gpt-5.4` | `closeout_checker_v1` | `pass` | 1752 | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_outputs/openai_gpt-5.4__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_results/openai_gpt-5.4__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_current_models/lane_logs/openai_gpt-5.4__closeout_checker_v1.jsonl` | closeout output correctly withheld closure until merge/proof evidence exists |

## Findings

- `openai:gpt-5.5` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.
- `openai:gpt-5.4` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.

## Non-claims

- This packet does not prove general OpenAI model quality outside the bounded panel tasks.
- This packet does not equate direct-hosted OpenAI success with Codex shell authority.
- This packet does not grant any tested lane merge, closeout, release, or repo-mutation authority.

