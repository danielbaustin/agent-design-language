# Structured Output Record

Template: 1.0.0

Issue: 5670

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented hosted full coverage with four workspace profraw shard producers and one authoritative aggregation gate, while keeping non-full PR-fast coverage in a separate single-run hosted producer.

## Artifacts

- .github/workflows/ci.yaml
- adl/tools/run_authoritative_coverage_lane.sh
- adl/tools/test_run_authoritative_coverage_lane.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- .github/workflows/ci.yaml
- adl/tools/run_authoritative_coverage_lane.sh
- adl/tools/test_run_authoritative_coverage_lane.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- .csdlc/evidence/5670/test_run_authoritative_coverage_lane.log
- .csdlc/evidence/5670/test_ci_runtime_contracts.log
- .csdlc/evidence/5670/test_ci_path_policy.log
- .csdlc/evidence/5670/ci-yaml-parse.log
- .csdlc/evidence/5670/git-diff-check.log

## Execution

- Added collect/report modes, shard selection, run-scoped profile roots, profraw import, and input validation to adl/tools/run_authoritative_coverage_lane.sh.
- Fanned adl-coverage-workspace-hosted across four workspace profraw shard producers while keeping non-full PR-fast coverage on shard 1 only.
- Moved authoritative full workspace summary rendering, lcov/text release artifact generation, provenance verification, and workspace/runtime summary merge into adl-coverage-hosted.
- Updated coverage runner, CI runtime, and path-policy contracts to prove the new shard/aggregation topology and guard against stale workflow expectations.
- Added collect/report modes, shard selection, run-scoped profile roots, compile-only profile cleanup, profraw import, and input validation to adl/tools/run_authoritative_coverage_lane.sh.
- Split non-full PR-fast coverage into adl_coverage_workspace_fast_hosted so it runs once and records non-sharded provenance.
- Limited adl_coverage_workspace_hosted to full coverage and fanned it across four profraw shard producers.
- Moved authoritative full workspace summary rendering, lcov/text release artifact generation, provenance verification, and workspace/runtime summary merge into adl_coverage_hosted.
- Updated coverage runner, CI runtime, and path-policy contracts to prove the fast/full split, shard topology, aggregation gate, and regression fixes.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_authoritative_coverage_lane.sh"
    ],
    "purpose": "Prove coverage collect/report modes, shard partition selection, unsafe run-id rejection, compile-only profile cleanup, profraw import, isolated outputs, and release artifact behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5670/test_run_authoritative_coverage_lane.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove CI topology contracts for separate PR-fast and full shard producers, hosted aggregation, provenance, artifacts, and required toolchain setup.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5670/test_ci_runtime_contracts.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove path-policy and coverage workflow contract integration for PR-fast, full coverage, and validation manager routing.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5670/test_ci_path_policy.log"
  },
  {
    "command": [
      "ruby",
      "-e",
      "require 'yaml'; YAML.safe_load(File.read('.github/workflows/ci.yaml'), aliases: true)"
    ],
    "purpose": "Verify the updated GitHub Actions workflow parses as YAML.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5670/ci-yaml-parse.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify patch whitespace and diff hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5670/git-diff-check.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
