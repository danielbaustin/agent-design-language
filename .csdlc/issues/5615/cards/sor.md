# Structured Output Record

Template: 1.0.0

Issue: 5615

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Initialize the standalone C-SDLC v2 external Cargo root inside a run step using RUNNER_TEMP and export it through GITHUB_ENV.

## Artifacts

- .github/workflows/ci.yaml
- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/ci_path_policy.sh
- adl/tools/run_cargo_validation.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_run_cargo_validation.sh
- adl/tools/test_select_validation_lanes.sh
- adl/tools/ci_path_policy.sh
- adl/tools/run_cargo_validation.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_run_cargo_validation.sh
- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- Split .csdlc metadata and csdlc-v2 Rust selectors in validation-manager authority
- Add an explicit standalone C-SDLC v2 selector output and hosted test/format/strict-Clippy job
- Fail closed on malformed selector output and required-but-skipped standalone proof
- Preserve Runtime v3 focused routing when issue lifecycle metadata is present
- Add an external Cargo build-root wrapper with FastWork fallback and compatibility symlink
- Derive standalone C-SDLC v2 proof from the selector's complete csdlc-v2 surface
- Compose standalone C-SDLC v2 and Runtime v3 focused lanes for mixed changes
- Reject pre-created Cargo child symlinks and require canonical child containment
- Add exact operator-path, mixed-lane, and symlink-escape regressions
- Remove the invalid runner.temp expression from job-level env
- Create the external Cargo root in the preparation step from RUNNER_TEMP
- Export ADL_CARGO_BUILD_ROOT through GITHUB_ENV for later steps
- Add a contract rejecting runner.temp at standalone job scope

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh",
      "&&",
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh",
      "&&",
      "bash",
      "adl/tools/test_run_cargo_validation.sh"
    ],
    "purpose": "Prove metadata-only, standalone C-SDLC v2, Runtime-plus-lifecycle, stable aggregate, selector, and external build-root behavior.",
    "outcome": "passed",
    "evidence_ref": "issue-5615:focused-ci-routing-and-wrapper-contracts:pass"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5615/run_csdlc_v2_standalone.sh"
    ],
    "purpose": "Prove all C-SDLC v2 targets with locked tests, formatting, and strict Clippy while Cargo state remains on the external SSD.",
    "outcome": "passed",
    "evidence_ref": "issue-5615:csdlc-v2-standalone-fastwork:test-fmt-clippy-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh",
      "&&",
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh",
      "&&",
      "bash",
      "adl/tools/test_run_cargo_validation.sh",
      "&&",
      "bash",
      "adl/tools/test_select_validation_lanes.sh",
      "&&",
      "bash",
      ".csdlc/prepared/issues/5615/run_csdlc_v2_standalone.sh"
    ],
    "purpose": "Prove selector/classifier agreement, compositional Runtime routing, child-symlink rejection, and the complete C-SDLC v2 test/fmt/strict-Clippy lane on FastWork.",
    "outcome": "passed",
    "evidence_ref": "issue-5615:exact-review-remediation:focused-and-standalone-pass"
  },
  {
    "command": [
      "ruby",
      "-e",
      "YAML.safe_load(File.read('.github/workflows/ci.yaml'), permitted_classes: [Date], aliases: true)",
      "&&",
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh",
      "&&",
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "purpose": "Prove the workflow parses, the standalone build root is initialized through RUNNER_TEMP and GITHUB_ENV, and the stable CI contract remains fail closed.",
    "outcome": "passed",
    "evidence_ref": "issue-5615:workflow-expression-repair:yaml-and-contracts-pass"
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
