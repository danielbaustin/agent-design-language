# Nessus Validation Manager Consumption Proof for `#4678`

Status: `implemented_local_contract_proven`
Issue: `#4678`
Date: 2026-07-04

## Scope

This packet records the v0.91.7 WP-06 follow-up that consumes the existing
Nessus remote validation lane through a first-class validation-manager wrapper.

This issue proves:

- a repo-local command can derive the single selected local lane from
  `adl/tools/validation_manager.sh`
- the derived command is routed back through the validation manager with
  `--remote-runner nessus`
- local `--changed-files` manifests are recreated inside the remote checkout
  before the consumed lane runs
- operators can pin the remote checkout with `--remote-git-ref`
- the remote report preserves the consumed `local_run` evidence under the
  `nessus_remote_validation` lane
- in local-executor contract mode, the fetched Nessus summary and
  validation-manager report both record a passed run
- docs-only remote requests still fail closed through the manager eligibility
  gates

This issue does not prove:

- a fresh live SSH run on `nessus.local` for this branch
- that Nessus is the default lane for all ADL validation
- that provider-credentialed or network-bound validation is safe to run on
  Nessus
- that GitHub CI has migrated to Nessus

## Implemented Surfaces

- `adl/tools/run_validation_manager_nessus_lane.sh`
- `adl/tools/test_run_validation_manager_nessus_lane.sh`
- `adl/config/validation_lane_selector.v0.91.6.json`
- `docs/tooling/NESSUS_VALIDATION_MANAGER_LANE.md`
- `adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md`

## Prior Evidence Consumed

The underlying remote runner and validation-manager remote selection were proven
before this issue:

- `docs/milestones/v0.91.6/review/build_throughput/NESSUS_REMOTE_VALIDATION_LANE_4553.md`
- `docs/milestones/v0.91.6/review/build_throughput/REMOTE_BUILD_LANES_4587.md`

Those packets establish the operational Nessus runner, retained summary/log
artifact contract, and the v0.91.6 decision to use Nessus as the immediate
Phase 1 remote validation lane.

## Local Contract Proof

Focused proving command:

```bash
bash adl/tools/test_run_validation_manager_nessus_lane.sh
```

Observed result:

```text
PASS test_run_validation_manager_nessus_lane
```

The test fixture runs `run_validation_manager_nessus_lane.sh` with:

- `ADL_NESSUS_REMOTE_EXECUTOR=local`
- a changed-file set that selects the focused Rust PR-fast lane
- no explicit `--remote-command`, forcing the wrapper to consume the manager's
  selected local command
- `--remote-git-ref origin/main`, proving the wrapper forwards the remote ref
  to the underlying runner
- `--remote-artifact-dir` and `--report-out`, proving both artifact surfaces

Assertions covered:

- `profile["run_status"] == "passed"`
- `profile["remote_runner"]["requested"] == "nessus"`
- `profile["remote_runner"]["decision"] == "selected"`
- `profile["run"][0]["lane_id"] == "nessus_remote_validation"`
- `profile["run"][0]["local_run"]` is retained
- fetched `summary.json` records `runner=nessus` and `status=passed`
- fetched `summary.json` records `.adl/tmp/validation-manager-nessus-changed-files.txt`
  instead of the caller's local temporary changed-files path
- fetched `summary.json` records `git_ref=origin/main`
- docs-only changed files fail closed with
  `requested remote runner is not eligible`

## Operator Command

For an eligible single-lane changed surface:

```bash
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --changed-files <changed-files.txt> \
  --remote-artifact-dir <artifact-dir> \
  --remote-git-ref <branch-or-ref> \
  --report-out <validation-manager-report.json> \
  --run \
  --json
```

The wrapper may also accept an explicit `--remote-command`, but that path still
uses the validation-manager remote eligibility gates.

## Residual Risk

This issue intentionally uses local-executor contract proof plus prior live
Nessus evidence. A fresh SSH run against `nessus.local` can be recorded by a
later operator proof when the machine is available and a branch-specific remote
checkout is desired.
