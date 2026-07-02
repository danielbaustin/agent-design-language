# v0.91.7 WP-04 Closeout: Goal State, Nested Goals, And Execution Metrics

Issue: `#4631`

Status: complete and merged.

## Summary

WP-04 is complete with residual metric-capture limits recorded. The child issue
set delivered goal accounting, SOR time/token/resource accounting, session
telemetry harvesting, bounded v0.91.6 metrics backfill, execution outlier
analysis, and the execution metrics prediction engine. This umbrella closeout
records the integrated proof surface and closes the WP-04 sprint wrapper
without adding new implementation scope.

## Child Issue Truth

| Issue | Result | PR | Closeout truth |
| --- | --- | --- | --- |
| `#4617` | Codex session telemetry harvesting/reporting and prediction support completed | `#4742` merged | STP/SIP/SOR validated by `pr.sh closeout` |
| `#4666` | Nested issue and sprint goal accounting completed | `#4727` merged | STP/SIP/SOR validated by `pr.sh closeout` |
| `#4667` | SOR time-token-resource accounting completed | `#4732` merged | STP/SIP/SOR validated by `pr.sh closeout` |
| `#4668` | Codex session telemetry harvesting completed | `#4733` merged | STP/SIP/SOR validated by `pr.sh closeout` |
| `#4669` | Bounded v0.91.6 metrics backfill completed | `#4734` merged | STP/SIP/SOR validated by `pr.sh closeout` |
| `#4670` | Execution outlier analysis completed | `#4735` merged | STP/SIP/SOR validated by `pr.sh closeout` |
| `#4743` | Execution metrics prediction engine completed | `#4744` merged | STP/SIP/SOR validated by `pr.sh closeout`; worktree was already pruned after restoring unrelated `#4630` VPP drift from known closeout bug `#4736` |

## Integrated Proof Surface

- `adl/tools/skills/sprint-conductor/scripts/write_issue_goal_metrics_report.py`
- `adl/tools/skills/sprint-conductor/scripts/predict_issue_execution_metrics.py`
- `adl/tools/test_sprint_conductor_helpers.sh`
- `docs/default_workflow.md`
- `docs/milestones/v0.91.7/review/V0917_WP04_CODEX_SESSION_TELEMETRY_SAMPLE_4617.*`
- `docs/milestones/v0.91.7/review/V0917_WP04_EXECUTION_METRICS_PREDICTION_SAMPLE_4743.*`

## Validation

The umbrella closeout used repo-native lifecycle tools only.

- `pr.sh watch 4617 4666 4667 4668 4669 4670 4743`: verified each child issue was closed with a merged linked PR.
- `pr.sh closeout 4617 4666 4667 4668 4669 4670`: validated each child STP/SIP/SOR root bundle.
- `pr.sh closeout 4743`: validated STP/SIP/SOR and confirmed the worktree was already absent after manual prune recovery.
- `git status --short --branch`: verified the root checkout remained clean on `main`.

## Residual Risks

- `#4736` remains the tracked closeout-tool bug for the repeated unrelated
  `#4630` VPP deletion during closeout. The affected VPP was restored before
  manual worktree pruning, and root checkout remained clean.
- WP-04 delivered the metrics fields, harvesting scripts, reports, samples, and
  prediction packet flow, but it did not fully prove automatic nested per-issue
  goal capture while an umbrella goal is active. Several child issues therefore
  retain `unknown` actual elapsed/token metrics where nested issue goal creation
  was refused by the active WP-04 tail goal. This is truthful accounting, not a
  zero or inferred metric, and it remains the residual before claiming fully
  automatic per-issue accounting in all sprint execution modes.
- `#4747` records the post-closeout review remediation: stale local SOR fields
  were normalized, `#4617` received its missing `wp:WP-04` label, and
  prediction readiness was split into minimal and full readiness so missing
  validation/PR/CI wait inputs cannot be mistaken for full prediction proof.

## Non-Claims

- This closeout does not claim the broader v0.91.7 milestone is complete.
- This closeout does not claim runtime/provider scheduling work from WP-05.
- This closeout does not claim the closeout-tool bug `#4736` is fixed.
