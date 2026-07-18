# Tools Reliability Remediation Validation (#5407)

Observed: 2026-07-16

## Typed PVF Result

The declared documentation lanes ran through `csdlc-validate` using the
retained request at `.csdlc/5407-validation.json`. Its lane executables use
`/usr/bin/env` so repository prerequisites resolve from the operator's `PATH`.
The first
performance-boundary probe failed because the explicit non-claim crossed a
Markdown line break. The wording was corrected to state, "No material hosted
wall-clock speedup is claimed," and the complete five-lane rerun passed.

| Lane | Command surface | Result |
| --- | --- | --- |
| patch-integrity | `git diff --check` | passed |
| logging-scope-truth | `rg` over `BUILD_ACTION_LOGS.md` | passed |
| typed-v2-authority | `rg` over `ADL_PLATFORM_CLI_BINARY_TAXONOMY.md` | passed |
| closeout coverage | `rg` for #5037 and #4938 | passed |
| performance-boundary | `rg` for the explicit non-claim | passed |

## Strengthened Closeout Proof

After independent review, AC-3 was strengthened from a two-child presence
check to a complete machine-readable snapshot check:

```text
jq -e '.entries | length == 11 and all(.[];
  .issue_state == "CLOSED" and
  .pr_state == "MERGED" and
  (.checks | length > 0))'
  docs/reviews/v0.91.7/tools-5407/github-closeout-snapshot-5036.json
```

Result: `true` (passed).

The snapshot was collected with `gh issue view` and `gh pr view` for every
declared #5036 child/PR pair. It retains issue closure timestamps, PR merge
timestamps, exact head and merge revisions, and the observed check-rollup
conclusions. It intentionally does not infer branch-protection requirements.

## Evidence Location

The first SOR entries cite lane logs under `.csdlc/issues/5407/evidence/`.
Those six referenced logs are tracked and resolve in the reviewed revision.
Because typed card updates atomically replace the canonical issue directory,
operators must restore these tracked evidence files after later lifecycle-only
card projections. This authored validation record and the retained request are
the durable, reproducible summary for the passing rerun and strengthened matrix
proof.
