# V0.91.7 WP-04 v0.91.6 Metrics Backfill

Issue: `#4669`

## Summary

This packet records the v0.91.7 WP-04 consumption pass over the bounded
v0.91.6 workflow-metrics backfill. It does not rewrite closed v0.91.6 issue
truth. It refreshes and consumes the retained #4441 evidence so WP-04 can feed
outlier analysis, sprint metrics, and v0.92 readiness planning from a complete
historical inventory.

## Source Evidence

- Historical backfill generator: `adl/tools/build_v0916_workflow_metric_backfill_inventory.py`
- CSV inventory: `docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_INVENTORY_4441.csv`
- JSON summary: `docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.json`
- Historical review note: `docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.md`

## Refresh Command

```bash
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
ADL_PR_RUST_BIN=adl/target/debug/adl \
python3 adl/tools/build_v0916_workflow_metric_backfill_inventory.py
```

The command uses repo-native ADL GitHub transport through `adl/tools/pr.sh
issue view` for issue metadata and reads the primary checkout local
`.adl/v0.91.6/tasks/issue-*` corpus as historical evidence.
Use an existing repo binary on `PATH` or set `ADL_PR_RUST_BIN` to the
repo-relative owner binary shown above; do not rebuild ADL just to replay this
backfill.

## Refreshed Counts

| Metric | Count |
| --- | ---: |
| Surveyed issues | 348 |
| Closed issues | 348 |
| Open issues | 0 |
| Row-contract complete rows | 348 |
| Row-contract partial rows | 0 |
| Row-contract incomplete rows | 0 |
| Actual elapsed explicit | 49 |
| Actual elapsed derived from SOR execution window | 275 |
| Actual elapsed unknown | 24 |
| GitHub cycle time reconstructed | 348 |
| Actual total tokens explicit | 32 |
| Actual total tokens unknown | 316 |
| Actual total tokens not collected | 0 |
| Full metrics known rows | 32 |
| Timing recovered but token gap rows | 292 |
| Cycle-only recovered rows | 24 |
| Open-issue local-timing-only rows | 0 |

## Coverage And Gaps

- Coverage: every surveyed v0.91.6 issue row satisfies the row contract.
- Coverage: every surveyed issue is now closed, so GitHub cycle time is
  reconstructable for all 348 rows.
- Gap: token totals remain sparse historically. Only 32 rows have explicit
  token totals; 316 rows remain `unknown`.
- Gap: 24 rows have cycle-time-only recovery because issue-local elapsed
  execution time is still unavailable.
- Non-claim: missing historical token values are not inferred, estimated, or
  treated as zero.

## v0.91.7 Consumption

- `#4670` should use this packet and
  `V0916_WORKFLOW_METRIC_BACKFILL_4441.json` as its historical baseline for
  execution outlier analysis.
- WP-04 sprint closeout should report the historical token gap explicitly
  instead of presenting v0.91.6 metrics as complete.
- Forward v0.91.7 capture remains owned by `#4667` and `#4668`; this packet is
  historical coverage/gap evidence only.
