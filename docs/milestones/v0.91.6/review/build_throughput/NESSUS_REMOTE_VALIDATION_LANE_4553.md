# Nessus Remote Validation Lane Proof for `#4553`

Status: `integrated_proven_for_issue_scope`
Issue: `#4553`
Date: 2026-06-26

## Scope

This packet records the bounded proof that ADL now has an operational
Nessus-backed remote validation lane rather than only prerequisite evidence.

This issue proves:

- a repo-native command can run one declared validation command on
  `nessus.local` through the proven `danie@nessus.local` plus explicit
  `wsl.exe -u root` path
- the validation manager can choose the Nessus runner for an eligible
  single-lane deterministic Rust profile
- the remote runner fails closed when preflight fails
- the remote runner emits a stable summary artifact plus retained remote log
  paths
- repeated runs against the same remote root record warm-path behavior

This issue does not prove:

- that every validation lane should run remotely
- that provider/model credentialed lanes are remote-safe
- that GitHub CI should move to Nessus
- that CodeBuild is no longer deferred

## Implemented surfaces

- `adl/tools/run_nessus_remote_validation.sh`
- `adl/tools/validation_manager.py --remote-runner nessus --remote-command ...`
- selector classification for the new remote-runner contract surfaces in
  `adl/config/validation_lane_selector.v0.91.6.json`

## Manager-routed proof

The first live run used the validation-manager route for the eligible focused
Rust profile produced by the changed-path set:

- changed path: `adl/src/provider_communication.rs`
- selected profile: `rust_pr_fast_profile`
- remote runner decision: `selected`
- remote command:
  `cargo test --manifest-path adl/Cargo.toml --locked provider_communication -- --nocapture`

Observed result:

- status: `passed`
- run id: `20260626-174605`
- commit: `abf3945ab363c54e43437b0124826b9fcafe916a`
- elapsed seconds: `181`
- remote run root: `/root/adl-remote-runner/logs/20260626-174605`

The remote command log shows the focused lane passed on Nessus:

- `21 passed; 0 failed`
- filtered binary harnesses remained `0 failed`

## Warm-path proof

The same remote command was run again against the same remote root without
clearing the retained target/cache directories.

Observed result:

- status: `passed`
- run id: `20260626-175020`
- commit: `abf3945ab363c54e43437b0124826b9fcafe916a`
- elapsed seconds: `8`
- remote run root: `/root/adl-remote-runner/logs/20260626-175020`

Interpretation:

- the first run paid the expected compile/setup cost
- the second run completed on the warm path with no additional compile work
  recorded by `sccache`
- this is sufficient to classify the Nessus runner as operational for this
  bounded focused Rust lane

## Cache and artifact truth

First run cache facts:

- compile requests: `134`
- compile requests executed: `33`
- cache misses: `32`
- cache hits: `0`
- `sccache` stats log:
  `/root/adl-remote-runner/logs/20260626-174605/sccache-stats.log`

Second run cache facts:

- compile requests: `0`
- compile requests executed: `0`
- cache misses: `0`
- cache hits: `0`
- `sccache` stats log:
  `/root/adl-remote-runner/logs/20260626-175020/sccache-stats.log`

Stable remote summary artifacts:

- `/root/adl-remote-runner/logs/20260626-174605/summary.json`
- `/root/adl-remote-runner/logs/20260626-175020/summary.json`

Each summary records:

- host/runner identity
- requested Git ref and resolved commit
- declared command
- exit status and elapsed seconds
- stable remote log paths
- retained target and `sccache` roots

## Failure-mode proof

The issue-local contract tests prove fail-closed behavior for:

- apt-source hygiene failure
- invalid remote-runner selection requests from the validation manager
- unmapped selector surfaces and ordinary validation-manager escalation gates

Focused proving commands:

- `bash adl/tools/test_run_nessus_remote_validation.sh`
- `bash adl/tools/test_select_validation_lanes.sh`
- `bash adl/tools/test_validation_manager.sh`

## CodeBuild disposition

`#4316` remains the truthful CodeBuild position:

- CodeBuild stays deferred
- Nessus is the current operational remote-runner path
- a future CodeBuild pilot still needs its own bounded issue with AWS/OIDC and
  security truth

## Non-claims

- This packet does not claim release-wide remote validation routing.
- This packet does not claim provider-credentialed or network-bound validation
  commands are eligible for Nessus by default.
- This packet does not claim the remote runner replaces local focused proof for
  normal narrow tooling issues.
