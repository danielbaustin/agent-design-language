# Coverage Authority And Release Proof

The repository has two intentionally different coverage surfaces:

The nightly coverage is release-authoritative when the scheduled watchdog
completes successfully.

| Surface | Purpose | Authority |
| --- | --- | --- |
| `adl/tools/run_pr_fast_coverage_lane.sh` | Fast, changed-surface feedback for a pull request | Non-authoritative advisory feedback |
| `adl/tools/run_authoritative_coverage_lane.sh` | Full workspace and companion `adl-runtime` instrumentation | Authoritative when selected by the merge/release policy |
| `.github/workflows/nightly-coverage-ratchet.yaml` | Scheduled full workspace report with the 90% workspace and 80% per-file floors | Release-authoritative nightly watchdog |

PR-fast coverage is non-authoritative and may be incomplete by design. It must not be used to claim
release coverage, and a passing PR-fast result does not waive the authoritative
merge or release lane. The authoritative runner distinguishes the full
`full_authoritative_default_features` mode from the bounded
`bounded_policy_surface_pr` mode; the latter is still not a release claim.

The nightly workflow currently sets `EXCLUDE_FROM_FILE_FLOOR_REGEX` to `^$`.
That means the report does not silently exempt active source files from the
80% per-file floor. If a future exception is required, it must be a reviewed
policy change with a named path and retained rationale, not an ad hoc workflow
edit.

## Proof boundary

`bash adl/tools/test_coverage_authority_contract.sh` proves the routing and
claim boundary without running instrumented coverage. The actual release claim
requires the corresponding authoritative workflow result and retained
`coverage-summary.json`; this contract test is not a substitute for that run.
