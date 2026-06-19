# Anthropic Current Direct-Hosted Suitability Proof

Date: 2026-06-18

Issues: `#4156`, `#4154`

## Scope

This packet runs the reusable C-SDLC suitability panel against current direct-hosted Anthropic models through the native hosted provider path using the ADL demo Anthropic key source.

It records bounded advisory-role evidence only. It does not grant merge, closeout, release, or repo-mutation authority.

## Source evidence

- `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`

## Candidate matrix

| Candidate | Lane | Watcher | Card validator | Reviewer | Planner | Closeout checker | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `anthropic:claude-opus-4-8` | `anthropic` / `claude-opus-4-8` | `pass_with_limits` | `pass` | `pass` | `fail_format` | `pass` | `candidate_only_format_repair_needed` |
| `anthropic:claude-sonnet-4-6` | `anthropic` / `claude-sonnet-4-6` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass` | `useful_with_limits` |
| `anthropic:claude-haiku-4-5` | `anthropic` / `claude-haiku-4-5` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass_with_limits` | `useful_with_limits` |

## Candidate descriptors

### `anthropic:claude-opus-4-8`

- Lane: `anthropic`
- Provider profile ref: `unprofiled:anthropic:claude-opus-4-8`
- Provider family: `anthropic`
- Provider spec kind: `anthropic`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/ADL_demo_ref_04.txt`
- Supported tasks: watcher, card_validator, reviewer, closeout_checker
- Recommendation: `candidate_only_format_repair_needed`

### `anthropic:claude-sonnet-4-6`

- Lane: `anthropic`
- Provider profile ref: `unprofiled:anthropic:claude-sonnet-4-6`
- Provider family: `anthropic`
- Provider spec kind: `anthropic`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/ADL_demo_ref_04.txt`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

### `anthropic:claude-haiku-4-5`

- Lane: `anthropic`
- Provider profile ref: `unprofiled:anthropic:claude-haiku-4-5`
- Provider family: `anthropic`
- Provider spec kind: `anthropic`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/ADL_demo_ref_04.txt`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

## Per-task evidence

| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `anthropic:claude-opus-4-8` | `watcher_state_v1` | `pass_with_limits` | 2475 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-opus-4-8__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-opus-4-8__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-opus-4-8__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `anthropic:claude-opus-4-8` | `card_validator_v1` | `pass` | 3752 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-opus-4-8__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-opus-4-8__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-opus-4-8__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `anthropic:claude-opus-4-8` | `review_findings_v1` | `pass` | 4147 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-opus-4-8__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-opus-4-8__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-opus-4-8__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `anthropic:claude-opus-4-8` | `bounded_planner_v1` | `fail_format` | 4100 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-opus-4-8__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-opus-4-8__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-opus-4-8__bounded_planner_v1.jsonl` | planner output missed the required headings |
| `anthropic:claude-opus-4-8` | `closeout_checker_v1` | `pass` | 2971 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-opus-4-8__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-opus-4-8__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-opus-4-8__closeout_checker_v1.jsonl` | closeout output correctly withheld closure until merge/proof evidence exists |
| `anthropic:claude-sonnet-4-6` | `watcher_state_v1` | `pass_with_limits` | 3539 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-sonnet-4-6__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-sonnet-4-6__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-sonnet-4-6__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `anthropic:claude-sonnet-4-6` | `card_validator_v1` | `pass` | 3082 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-sonnet-4-6__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-sonnet-4-6__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-sonnet-4-6__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `anthropic:claude-sonnet-4-6` | `review_findings_v1` | `pass` | 5627 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-sonnet-4-6__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-sonnet-4-6__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-sonnet-4-6__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `anthropic:claude-sonnet-4-6` | `bounded_planner_v1` | `pass` | 5848 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-sonnet-4-6__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-sonnet-4-6__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-sonnet-4-6__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `anthropic:claude-sonnet-4-6` | `closeout_checker_v1` | `pass` | 3210 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-sonnet-4-6__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-sonnet-4-6__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-sonnet-4-6__closeout_checker_v1.jsonl` | closeout output correctly withheld closure until merge/proof evidence exists |
| `anthropic:claude-haiku-4-5` | `watcher_state_v1` | `pass_with_limits` | 1433 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-haiku-4-5__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-haiku-4-5__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-haiku-4-5__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `anthropic:claude-haiku-4-5` | `card_validator_v1` | `pass` | 1897 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-haiku-4-5__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-haiku-4-5__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-haiku-4-5__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `anthropic:claude-haiku-4-5` | `review_findings_v1` | `pass` | 2875 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-haiku-4-5__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-haiku-4-5__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-haiku-4-5__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `anthropic:claude-haiku-4-5` | `bounded_planner_v1` | `pass` | 3003 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-haiku-4-5__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-haiku-4-5__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-haiku-4-5__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `anthropic:claude-haiku-4-5` | `closeout_checker_v1` | `pass_with_limits` | 1699 | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_outputs/anthropic:claude-haiku-4-5__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_results/anthropic:claude-haiku-4-5__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/lane_logs/anthropic:claude-haiku-4-5__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |

## Findings

- `anthropic:claude-opus-4-8` is `candidate_only_format_repair_needed` for the bounded panel, based on task scores `watcher, card_validator, reviewer, closeout_checker`.
- `anthropic:claude-sonnet-4-6` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.
- `anthropic:claude-haiku-4-5` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.

## Non-claims

- This packet does not prove general Anthropic model quality outside the bounded panel tasks.
- This packet does not promote any Claude lane to default authority-bearing use.
- This packet does not grant any tested lane merge, closeout, release, or repo-mutation authority.

