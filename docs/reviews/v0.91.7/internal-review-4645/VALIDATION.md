# v0.91.7 Internal Review Validation (#4645)

Status: local_validation_passed

Issue: #4645

Captured: 2026-07-18

## Commands

```bash
git diff --check
python3 -m json.tool docs/reviews/v0.91.7/internal-review-4645/live-state/summary.json
python3 -m json.tool docs/reviews/v0.91.7/internal-review-4645/live-state/dependency_5408_5419.json
python3 -m json.tool docs/reviews/v0.91.7/internal-review-4645/live-state/github_issue_summary.json
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/adl-wp-4645-csdlc-v2-target cargo run --manifest-path csdlc-v2/Cargo.toml --bin csdlc-validate -- --request docs/reviews/v0.91.7/internal-review-4645/live-state/validate-request.json
```

## Typed PVF Result

`csdlc-validate` returned:

```json
{
  "schema": "csdlc.pvf.report.v1",
  "disposition": "local_pass",
  "selected_waves": [["dependency-json", "diff-check", "summary-json"]]
}
```

Retained logs:

- `docs/reviews/v0.91.7/internal-review-4645/validation/dependency-json.log`
- `docs/reviews/v0.91.7/internal-review-4645/validation/diff-check.log`
- `docs/reviews/v0.91.7/internal-review-4645/validation/summary-json.log`

## Retrospective committed-range correction

The original bare `git diff --check` command and empty retained log only proved
that the worktree had no unstaged whitespace errors at execution time. They did
not inspect the committed PR range and therefore do not prove the committed
bytes of PR #5543.

Retrospective exact-head review found a trailing blank line at EOF in
`SPECIALIST_LANE_RESULTS.md` with `git diff --check HEAD^..HEAD`. Issue #5572
records that finding. The remediation removes the defect and validates the
complete remediation range with:

```bash
bash adl/tools/test_retained_diff_proof_contract.sh \
  1adb842f8d71506e9eb95de132761cf96eeea55b \
  246be119d085a647e848b425713a7386c5fb32f4
```

After rebasing the remediation branch onto current `main`, the pinned endpoints
identify that immutable rebase base and the exact first remediation commit.
They isolate the remediation change and do not absorb unrelated later `main`
changes. The helper fails closed unless both a base and head revision are named.
This addendum preserves the original limitation instead of rewriting the old
log as proving evidence.

## Validation Boundaries

- `/Volumes/home/builds` was not mounted in this session, so the Rust build
  target was placed under `/Volumes/FastWork/adl-builds/adl-wp-4645-csdlc-v2-target`.
- No AWS validation was run.
- No broad Rust/runtime suites were run for this docs/review-only packet.
- The #5408/#5419 live-state snapshot is a point-in-time dependency record, not
  a claim that #5408 passed or closed.
