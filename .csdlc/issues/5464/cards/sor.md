# Structured Output Record

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Classify every install-action tool token independently and reject nextest-bearing multi-tool steps.

## Artifacts

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- Pin all four nextest install steps to install-action v2.82.10 commit 50414676f9f5d50a65992c6dd2ed02641263226c
- Set fallback to none on every nextest install step
- Require the canonical installer, nextest version, fallback policy, and complete four-step inventory in the CI runtime contract
- Require exactly four nextest selections and exactly four 0.9.140 selections across the whole workflow
- Reject the alternate cargo-nextest alias
- Add negative fixtures for unnamed, quoted, inline-map, floating-installer, and fallback-enabled forms
- Require each install-action use to occupy its own named step
- Normalize block-style tool scalars and classify versioned and unversioned nextest aliases
- Add negative fixtures for unversioned nextest and cargo-nextest aliases
- Parse ci.yaml with YAML.safe_load instead of regex for nextest installer enforcement
- Traverse every job step and normalize quoted, block, and inline mappings
- Add quoted installer and fully inline step bypass fixtures
- Split tool inputs on commas and whitespace
- Require nextest-bearing steps to select only nextest@0.9.140
- Add comma-list, whitespace-list, and cargo-nextest multi-tool fixtures

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove every hosted nextest install uses the supported immutable manifest, pins 0.9.140, and disables fallback",
    "outcome": "passed",
    "evidence_ref": "local:5464-nextest-install-contract-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove unnamed, quoted, inline, floating-installer, and fallback-enabled nextest forms cannot escape enforcement",
    "outcome": "passed",
    "evidence_ref": "local:5464-nextest-bypass-regression-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove every install-action step is inventoried and both unversioned nextest aliases fail closed",
    "outcome": "passed",
    "evidence_ref": "local:5464-unversioned-alias-regression-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove structural nextest enforcement covers quoted installer scalars and fully inline workflow steps",
    "outcome": "passed",
    "evidence_ref": "local:5464-structural-yaml-regression-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove comma- and whitespace-separated tool lists cannot hide nextest or cargo-nextest",
    "outcome": "passed",
    "evidence_ref": "local:5464-multitool-regression-pass"
  },
  {
    "command": [
      "gh",
      "api",
      "repos/danielbaustin/agent-design-language/actions/jobs/88059440377/logs"
    ],
    "purpose": "Prove nextest 0.9.140 downloads directly from the supported release asset, verifies SHA-256, and emits no unsupported-binary or cargo-binstall fallback warning",
    "outcome": "passed",
    "evidence_ref": "github-actions:run-29636408536:job-88059440377:direct-nextest-install:annotations-empty:spot-skipped"
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
