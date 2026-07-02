---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth-validation-plan"
issue: 4630
task_id: "issue-4630"
run_id: "issue-4630"
version: "v0.91.7"
title: "[v0.91.7][WP-03] Consume C-SDLC integration control-plane truth"
branch: "codex/4630-v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth"
generated_at: "2026-07-02T17:35:41Z"
card_status: "ready"
status: "ready"
initial_pvf_lane: "prompt_template"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "vpp.lane.v1"
validation_runtime_class: "small"
validation_resource_profile: "local"
validation_family: "tooling_command_profile"
validation_size_split: "small_only"
expected_proof_cost: "medium"
planned_validation_seconds: "600"
planned_validation_tokens: "1000000"
issue_goal_ref: "issue-4630"
sprint_goal_ref: "unknown"
goal_metrics_rollup_ref: "unknown"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/4630"
  - kind: "stp"
    ref: ".adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/spp.md"
selected_lanes:
  - "tooling_command_tests"
  - "proof_validation_contract"
  - "diff_check"
parallel_groups:
  - "tooling_shepherd"
  - "retained_proof_lane"
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml --bin adl lifecycle_shepherd -- --nocapture"
  - "ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token cargo run --quiet --manifest-path adl/Cargo.toml --bin adl-pr-shepherd -- 4630 --version v0.91.7 --json"
  - "ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token ./adl/tools/pr.sh shepherd 4630 --version v0.91.7 --json"
  - "cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::tooling_cmd::tests::structured_prompt::tracked_csdlc_card_bundle_validates' -- --exact --nocapture"
  - "cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::pr_cmd::doctor::tests::card_lifecycle_accepts_tracked_csdlc_bundle' -- --exact --nocapture"
  - "bash adl/tools/test_run_v0913_proof_validation_lane.sh"
  - "git diff --check"
failure_policy: "fail_closed"
notes: "Planning corrected from generated docs-only profile to first-class lifecycle shepherd tooling slice. Focus proof on the new repo-native `adl-pr-shepherd` owner binary, its doctor/watch synthesis, compatibility-wrapper reachability, and refreshed-base coexistence with the upstream `pr-inventory` command surface. After PR publication, `adl-ci` failed in the retained v0.91.3 proof-validation lane because the older card-lifecycle contract commands broad-compiled enough of the tree to exhaust runner disk. Validation planning now explicitly includes the blocker-driven exact `--bin adl` retained-proof contract tests and the narrow proof-lane contract script that proves the fix."
---

Canonical Template Source: `docs/templates/prompts/1.0.3/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Validation planning prompt for [v0.91.7][WP-03] Consume C-SDLC integration control-plane truth; source issue prompt: .adl/v0.91.7/bodies/issue-4630-v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth.md.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `vpp.lane.v1`
- Initial PVF lane from issue creation: `prompt_template`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- tooling_command_tests
- proof_validation_contract
- diff_check

## Parallelization Plan

- Parallel groups: tooling_shepherd
- Validation runtime class: `small`
- Validation resource profile: `local`
- Validation family: `tooling_command_profile`
- Validation size split: `small_only`

## Goal Accounting Hooks

- Issue goal ref: `issue-4630`
- Sprint goal ref: `unknown`
- Goal metrics rollup ref: `unknown`

## Proof Cost / Runtime Expectations

- Expected proof cost: `medium`
- Planned validation seconds: `600`
- Planned validation token budget: `1000000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml --bin adl lifecycle_shepherd -- --nocapture
- ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token cargo run --quiet --manifest-path adl/Cargo.toml --bin adl-pr-shepherd -- 4630 --version v0.91.7 --json
- ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token ./adl/tools/pr.sh shepherd 4630 --version v0.91.7 --json
- cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::tooling_cmd::tests::structured_prompt::tracked_csdlc_card_bundle_validates' -- --exact --nocapture
- cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::pr_cmd::doctor::tests::card_lifecycle_accepts_tracked_csdlc_bundle' -- --exact --nocapture
- bash adl/tools/test_run_v0913_proof_validation_lane.sh
- git diff --check

## Failure Semantics

- fail_closed

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Planning corrected from generated docs-only profile to first-class lifecycle shepherd tooling slice. Focus proof on the new repo-native `adl-pr-shepherd` owner binary, its doctor/watch synthesis, compatibility-wrapper reachability, and refreshed-base coexistence with the upstream `pr-inventory` command surface. After PR publication, `adl-ci` failed in the retained v0.91.3 proof-validation lane because the older card-lifecycle contract commands broad-compiled enough of the tree to exhaust runner disk. Validation planning now explicitly includes the blocker-driven exact `--bin adl` retained-proof contract tests and the narrow proof-lane contract script that proves the fix.
