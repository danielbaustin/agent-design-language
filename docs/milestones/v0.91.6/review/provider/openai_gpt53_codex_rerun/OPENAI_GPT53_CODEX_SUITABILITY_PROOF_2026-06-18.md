# OpenAI GPT-5.3-Codex Direct-Hosted Suitability Proof

Date: 2026-06-18

Issues: `#4155`, `#4154`

## Scope

This packet reruns the reusable C-SDLC suitability panel against direct-hosted OpenAI gpt-5.3-codex.

It records bounded advisory-role evidence only. It does not grant merge, closeout, release, or repo-mutation authority.

## Source evidence

- `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`

## Candidate matrix

| Candidate | Lane | Watcher | Card validator | Reviewer | Planner | Closeout checker | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `openai:gpt-5.3-codex` | `openai` / `gpt-5.3-codex` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass_with_limits` | `useful_with_limits` |

## Candidate descriptors

### `openai:gpt-5.3-codex`

- Lane: `openai`
- Provider profile ref: `unprofiled:openai:gpt-5.3-codex`
- Provider family: `openai`
- Provider spec kind: `openai`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/openai2.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

## Per-task evidence

| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `openai:gpt-5.3-codex` | `watcher_state_v1` | `pass_with_limits` | 6520 | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_outputs/openai_gpt-5.3-codex__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_results/openai_gpt-5.3-codex__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_logs/openai_gpt-5.3-codex__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `openai:gpt-5.3-codex` | `card_validator_v1` | `pass` | 2463 | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_outputs/openai_gpt-5.3-codex__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_results/openai_gpt-5.3-codex__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_logs/openai_gpt-5.3-codex__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `openai:gpt-5.3-codex` | `review_findings_v1` | `pass` | 4549 | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_outputs/openai_gpt-5.3-codex__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_results/openai_gpt-5.3-codex__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_logs/openai_gpt-5.3-codex__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `openai:gpt-5.3-codex` | `bounded_planner_v1` | `pass` | 4665 | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_outputs/openai_gpt-5.3-codex__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_results/openai_gpt-5.3-codex__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_logs/openai_gpt-5.3-codex__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `openai:gpt-5.3-codex` | `closeout_checker_v1` | `pass_with_limits` | 3439 | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_outputs/openai_gpt-5.3-codex__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_results/openai_gpt-5.3-codex__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/lane_logs/openai_gpt-5.3-codex__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |

## Findings

- `openai:gpt-5.3-codex` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.

## Non-claims

- This packet does not prove general OpenAI model quality outside the bounded panel tasks.
- This packet does not equate direct-hosted OpenAI success with Codex shell authority.
- This packet does not grant any tested lane merge, closeout, release, or repo-mutation authority.

