# Gemini Current Direct-Hosted Suitability Proof

Date: 2026-06-18

Issues: `#4157`, `#4154`

## Scope

This packet runs the reusable C-SDLC suitability panel against current direct-hosted Gemini models through the native hosted provider path.

It records bounded advisory-role evidence only. It does not grant merge, closeout, release, or repo-mutation authority.

The panel tasks intentionally reuse shared calibration fixtures from the
tracked suitability-panel contract, including the legacy merge-truth and
legacy-`gh` evidence excerpts. Those references are prompt fixtures for
cross-model comparison, not claims about live `#4157` repository state.

## Source evidence

- `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`

## Candidate matrix

| Candidate | Lane | Watcher | Card validator | Reviewer | Planner | Closeout checker | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `gemini:gemini-2.5-pro` | `gemini` / `gemini-2.5-pro` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass_with_limits` | `useful_with_limits` |
| `gemini:gemini-2.5-flash` | `gemini` / `gemini-2.5-flash` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass` | `useful_with_limits` |
| `gemini:gemini-2.5-flash-lite` | `gemini` / `gemini-2.5-flash-lite` | `pass_with_limits` | `pass` | `pass` | `pass` | `pass` | `useful_with_limits` |

## Candidate descriptors

### `gemini:gemini-2.5-pro`

- Lane: `gemini`
- Provider profile ref: `unprofiled:gemini:gemini-2.5-pro`
- Provider family: `google`
- Provider spec kind: `gemini`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/gcp-ace-2023.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

### `gemini:gemini-2.5-flash`

- Lane: `gemini`
- Provider profile ref: `unprofiled:gemini:gemini-2.5-flash`
- Provider family: `google`
- Provider spec kind: `gemini`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/gcp-ace-2023.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

### `gemini:gemini-2.5-flash-lite`

- Lane: `gemini`
- Provider profile ref: `unprofiled:gemini:gemini-2.5-flash-lite`
- Provider family: `google`
- Provider spec kind: `gemini`
- Runtime surface: `hosted_api`
- Credential source: `$HOME/keys/gcp-ace-2023.key`
- Supported tasks: watcher, card_validator, reviewer, planner, closeout_checker
- Recommendation: `useful_with_limits`

## Per-task evidence

| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `gemini:gemini-2.5-pro` | `watcher_state_v1` | `pass_with_limits` | 11886 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-pro__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-pro__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-pro__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `gemini:gemini-2.5-pro` | `card_validator_v1` | `pass` | 10234 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-pro__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-pro__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-pro__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `gemini:gemini-2.5-pro` | `review_findings_v1` | `pass` | 12156 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-pro__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-pro__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-pro__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `gemini:gemini-2.5-pro` | `bounded_planner_v1` | `pass` | 9162 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-pro__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-pro__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-pro__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `gemini:gemini-2.5-pro` | `closeout_checker_v1` | `pass_with_limits` | 9584 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-pro__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-pro__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-pro__closeout_checker_v1.jsonl` | closeout output remained conservative even if it missed one specific supplied gap |
| `gemini:gemini-2.5-flash` | `watcher_state_v1` | `pass_with_limits` | 4555 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `gemini:gemini-2.5-flash` | `card_validator_v1` | `pass` | 3475 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `gemini:gemini-2.5-flash` | `review_findings_v1` | `pass` | 4888 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `gemini:gemini-2.5-flash` | `bounded_planner_v1` | `pass` | 3382 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `gemini:gemini-2.5-flash` | `closeout_checker_v1` | `pass` | 3299 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash__closeout_checker_v1.jsonl` | closeout output correctly withheld closure until merge/proof evidence exists |
| `gemini:gemini-2.5-flash-lite` | `watcher_state_v1` | `pass_with_limits` | 688 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash-lite__watcher_state_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash-lite__watcher_state_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash-lite__watcher_state_v1.jsonl` | watcher output stayed bounded and cited the supplied workflow facts |
| `gemini:gemini-2.5-flash-lite` | `card_validator_v1` | `pass` | 749 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash-lite__card_validator_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash-lite__card_validator_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash-lite__card_validator_v1.jsonl` | card-validator output identified the supplied lifecycle-truth contradiction |
| `gemini:gemini-2.5-flash-lite` | `review_findings_v1` | `pass` | 1054 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash-lite__review_findings_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash-lite__review_findings_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash-lite__review_findings_v1.jsonl` | review output identified the legacy-gh evidence drift and routed to ADL-native proof |
| `gemini:gemini-2.5-flash-lite` | `bounded_planner_v1` | `pass` | 1243 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash-lite__bounded_planner_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash-lite__bounded_planner_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash-lite__bounded_planner_v1.jsonl` | planner output stayed bounded and covered the required lane/proof constraints |
| `gemini:gemini-2.5-flash-lite` | `closeout_checker_v1` | `pass` | 566 | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_outputs/gemini:gemini-2.5-flash-lite__closeout_checker_v1.md` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_results/gemini:gemini-2.5-flash-lite__closeout_checker_v1.json` | `docs/milestones/v0.91.6/review/provider/gemini_current_models/lane_logs/gemini:gemini-2.5-flash-lite__closeout_checker_v1.jsonl` | closeout output correctly withheld closure until merge/proof evidence exists |

## Findings

- `gemini:gemini-2.5-pro` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.
- `gemini:gemini-2.5-flash` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.
- `gemini:gemini-2.5-flash-lite` is `useful_with_limits` for the bounded panel, based on task scores `watcher, card_validator, reviewer, planner, closeout_checker`.

## Non-claims

- This packet does not prove general Gemini model quality outside the bounded panel tasks.
- This packet does not equate prior OpenRouter Gemini route proof with native Gemini proof.
- This packet does not grant any tested lane merge, closeout, release, or repo-mutation authority.
