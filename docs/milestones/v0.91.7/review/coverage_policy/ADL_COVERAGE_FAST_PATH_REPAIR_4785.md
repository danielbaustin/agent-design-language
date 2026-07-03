# ADL Coverage Fast-Path Repair (#4785)

## Summary

Issue `#4785` repairs the `adl-coverage` PR path after the coverage workflow regressed into running the full authoritative workspace coverage lane whenever coverage was required.

The intended contract is restored:

- full authoritative coverage remains required for push-to-main, fail-closed, and full coverage surfaces
- pull requests with `coverage_required=true` and `full_coverage_required=false` use a targeted PR-fast coverage summary
- PR-fast coverage uses `cargo llvm-cov nextest` with the risk expression produced by `check_coverage_impact.sh`
- PRs with no risky changed production Rust source run an explicit coverage-impact preflight instead of pretending coverage ran
- required tools remain real and deterministic: `cargo-llvm-cov`, `cargo-nextest`, `sccache`, and `lld`

## Release-gate disposition

This change touches `.github/workflows/ci.yaml`, so the validation manager correctly classifies it as a release-gate surface.

Disposition: approved for draft PR publication with focused release-gate evidence.

Reason:

- the change narrows PR coverage execution from accidental full workspace coverage back to the bounded PR-fast path used in earlier milestones
- push-to-main and non-PR full coverage gates are not weakened
- the PR-fast path still produces a real `coverage-summary.json` when risky Rust source requires coverage evidence
- the no-risk PR path runs `check_coverage_impact.sh --require-summary-for-risk`, which fails closed if risky changed Rust source exists without summary evidence
- local contract tests cover the workflow/path-policy contract and the coverage-impact expression path
- GitHub `adl-coverage` remains the external proof gate before merge

Non-claim: this packet does not claim full workspace coverage ran locally for `#4785`.

## Focused validation

Commands run locally:

```bash
bash adl/tools/test_ci_path_policy.sh
bash adl/tools/test_check_coverage_impact.sh
bash adl/tools/test_run_authoritative_coverage_lane.sh
git diff --check
```

Results: all passed locally before PR publication.

## Review

A bounded subagent review was run before publication.

Findings fixed before publication:

- `is_pr_fast_coverage_workflow_change` initially inspected full diff context instead of changed payload lines, which could reject the intended bounded workflow repair. Fixed by evaluating added/removed payload lines only.
- The synthetic workflow fixture initially did not model the production multiline `GITHUB_OUTPUT` contract for `filter_expression`. Fixed by updating the fixture to use the same heredoc shape.
- A `grep -E` guard for disallowed selectors initially used a pattern beginning with `--` without `-e`. Fixed with `grep -E -e`.

Final review result: no blockers.

## Expected GitHub evidence

The PR should show `adl-coverage` using the restored PR-fast path for this coverage-policy repair. The expected outcome is a green required `adl-coverage` check with timing substantially below the previous 30-45 minute accidental full-workspace run.
