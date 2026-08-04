# Structured Output Record

Template: 1.0.0

Issue: 5610

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Coalesce lexical aliases for one canonical coverage file conservatively from summary-only evidence.

## Artifacts

- adl/tools/merge_coverage_summaries.py
- adl/tools/test_merge_coverage_summaries.sh
- adl/tools/merge_coverage_summaries.py
- adl/tools/test_merge_coverage_summaries.sh
- adl/tools/merge_coverage_summaries.py
- adl/tools/test_merge_coverage_summaries.sh

## Execution

- Normalize slash-unified filenames with POSIX lexical semantics
- Permit bounded parent traversal only beneath the owned source root
- Add exact safe-path and repository/owned-root escape regressions
- Deduplicate identical complete records after ownership canonicalization
- Reject conflicting records for the same canonical owned filename
- Add exact identical-alias acceptance and conflicting-alias rejection regressions
- Require identical metric-name and per-metric field schemas plus identical non-summary fields
- Use per-metric maximum count and covered values without summing
- Recompute notcovered and percent for every coalesced metric
- Add exact artifact-derived and fail-closed regressions

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_merge_coverage_summaries.sh"
    ],
    "purpose": "Prove exact safe lexical normalization, repository and owned-root escape rejection, unchanged merge semantics, and coupled authoritative coverage contracts.",
    "outcome": "passed",
    "evidence_ref": "FastWork: test_merge_coverage_summaries, test_ci_runtime_contracts, and test_run_authoritative_coverage_lane all passed; py_compile and git diff --check passed."
  },
  {
    "command": [
      "bash",
      "adl/tools/test_merge_coverage_summaries.sh"
    ],
    "purpose": "Prove complete-record equality is required before canonical alias coalescing and conflicts remain fail-closed.",
    "outcome": "passed",
    "evidence_ref": "FastWork: py_compile, test_merge_coverage_summaries, test_ci_runtime_contracts, test_run_authoritative_coverage_lane, and git diff --check all passed."
  },
  {
    "command": [
      "bash",
      "adl/tools/test_merge_coverage_summaries.sh"
    ],
    "purpose": "Prove conservative maxima coalescing, exact hosted artifact compatibility, malformed/schema/non-summary rejection, coupled CI contracts, and unchanged authoritative coverage behavior.",
    "outcome": "passed",
    "evidence_ref": "FastWork: synthetic merger regressions passed; exact run 29813137619 artifacts merged with agent_cmd.rs instantiations 246/61/185; py_compile, test_ci_runtime_contracts, test_run_authoritative_coverage_lane, test_coverage_authority_contract, and git diff --check passed."
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
