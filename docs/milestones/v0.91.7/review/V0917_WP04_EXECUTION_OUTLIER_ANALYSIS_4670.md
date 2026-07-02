# V0.91.7 WP-04 Execution Outlier Analysis

Issue: `#4670`

## Summary

This packet is the deterministic execution-outlier analysis for WP-04.
It consumes the bounded v0.91.6 workflow metric backfill and keeps
`unknown` values out of numeric thresholds instead of treating them as zero.

## Source

- Input CSV: `docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_INVENTORY_4441.csv`
- Historical backfill issue: `#4441`
- v0.91.7 consumption issue: `#4669`
- Baseline freshness: this report reflects the input CSV at runtime.

## Coverage

- Surveyed issues: `348`
- Closed issues: `348`
- Row contract counts: `{"complete": 348}`
- Metric availability counts: `{"cycle_only_recovered": 24, "full_metrics_known": 32, "timing_recovered_token_gap": 292}`
- Row confidence counts: `{"high": 32, "low": 316}`

## Metric Thresholds

| Metric | Known | Unknown | Invalid | Median | P90 | P95 | Max | Outliers |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `actual_session_elapsed_seconds` | 323 | 24 | 1 | 875 | 30560 | 64218 | 162409 | 17 |
| `github_cycle_time_seconds` | 348 | 0 | 0 | 44640 | 165664 | 261493 | 955044 | 18 |
| `actual_total_tokens` | 32 | 316 | 0 | 264239 | 1021648 | 1937899 | 9333614 | 2 |

## Top Outliers

### `actual_session_elapsed_seconds`

| Issue | Value | Status | Confidence | Title |
| --- | ---: | --- | --- | --- |
| `#3972` | 162409 | derived | low | [v0.91.6][WP-07][security] Complete security bridge readiness and CAV route |
| `#3973` | 162409 | derived | low | [v0.91.6][WP-08][identity] Complete identity continuity and capability-selector bridge |
| `#3975` | 162408 | derived | low | [v0.91.6][WP-10][memory] Account for AEE, Memory/ObsMem, Memory Palace, and ACP |
| `#4241` | 93744 | derived | low | [v0.91.6][runtime][sprint] Runtime resilience follow-on sprint |
| `#3966` | 93496 | derived | low | [v0.91.6][WP-01][planning] Promote v0.91.6 issue wave and schedule existing work |
| `#4012` | 90990 | derived | low | [v0.91.6][WP-05][provider][M-05] Complete provider/model reliability closeout matrix |
| `#3967` | 90574 | derived | low | [v0.91.6][WP-02][resilience] Resilience layer mini-sprint umbrella |
| `#3970` | 90425 | derived | low | [v0.91.6][WP-05][provider] Complete provider/model reliability and multi-agent readiness |
| `#4341` | 87946 | derived | low | [v0.91.6][WP-09][observatory][O-06] Rebuild the HTML Observatory as a mobile-capable governed surface |
| `#4286` | 79593 | derived | low | [v0.91.6][tools][projection] Move PR closing-linkage guard into Rust/PVF |
| `#4167` | 78192 | derived | low | [v0.91.6][acip][runtime][R-04] Prove the first local multi-agent ACIP runtime slice |
| `#4438` | 77602 | derived | low | [v0.91.6][csdlc][adoption] Prove full operational C-SDLC path on a fresh issue |
| `#4543` | 76501 | derived | low | [v0.91.6][runtime][soak] Execute integrated runtime/ops Soak #1 |
| `#4394` | 71842 | derived | low | [v0.91.6][tools][templates] Repair prompt-card template edge cases |
| `#4390` | 68568 | derived | low | [v0.91.6][tools][pvf] Externalize PVF lane selection and configuration |

### `github_cycle_time_seconds`

| Issue | Value | Status | Confidence | Title |
| --- | ---: | --- | --- | --- |
| `#3984` | 955044 | derived | high | [v0.91.6][WP-19][release] Release ceremony |
| `#3983` | 948361 | derived | low | [v0.91.6][WP-18][review] Next milestone review pass |
| `#3982` | 946330 | derived | low | [v0.91.6][WP-17][planning] Next milestone planning and v0.91.7 handoff |
| `#3981` | 943815 | derived | low | [v0.91.6][WP-16][review] Findings remediation and final preflight |
| `#3980` | 943780 | derived | low | [v0.91.6][WP-15][review] External and third-party review |
| `#3978` | 854168 | derived | low | [v0.91.6][WP-13][docs] Docs and review-surface alignment |
| `#3977` | 850474 | derived | low | [v0.91.6][WP-12][quality] Coverage and quality gate |
| `#3976` | 849703 | derived | low | [v0.91.6][WP-11][demo] Demo matrix and proof convergence |
| `#3979` | 536616 | derived | low | [v0.91.6][WP-14][review] Internal review and pre-v0.92 burn-down checklist |
| `#3974` | 362254 | derived | low | [v0.91.6][WP-09][observatory] Implement Observatory and Unity demo readiness |
| `#4035` | 349869 | derived | low | [v0.91.6][WP-09][observatory][O-05] Complete working Unity Observatory closeout proof |
| `#4034` | 349204 | derived | low | [v0.91.6][WP-09][observatory][O-04] Complete logging OTel and security consumption proof |
| `#4033` | 320804 | derived | low | [v0.91.6][WP-09][observatory][O-03] Implement inhabitant-readiness surfaces |
| `#4032` | 318713 | derived | low | [v0.91.6][WP-09][observatory][O-02] Implement ADL evidence data contract for Observatory |
| `#4622` | 315288 | derived | low | [v0.91.6][tools][github] Add repo-native PR inventory for release-tail review |

### `actual_total_tokens`

| Issue | Value | Status | Confidence | Title |
| --- | ---: | --- | --- | --- |
| `#4417` | 9333614 | explicit | high | [v0.91.6][tools][mini-sprint] Validation throughput and lifecycle automation |
| `#4529` | 1937899 | explicit | high | [v0.91.6][demo] Migrate Unity Observatory to Unity 6.5 baseline |

## Non-Claims

- Unknown values are excluded from percentile and outlier thresholds, not treated as zero.
- Invalid numeric values are excluded from thresholds and counted separately from unknown values.
- Historical token totals remain sparse; token outliers cover only rows with explicit token evidence.
- This analysis is descriptive over the retained backfill artifact and is not a predictive model.
- The report reflects the input CSV at runtime.

## v0.91.7 Consumption

- WP-04 closeout should report timing and token outliers separately.
- Future validation-manager work should compare forward v0.91.7 issue metrics against these descriptive baselines.
- Token outlier analysis should remain explicitly incomplete until forward capture substantially reduces the historical token gap.
