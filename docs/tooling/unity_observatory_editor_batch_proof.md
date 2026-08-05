# Unity Observatory editor and batch proof

Issue: #4741

## Purpose

The Observatory wrapper chooses and prints one safe mode before Unity work:

- `open_editor` uses a live exact-project owner only when an
  operator-configured editor-mediated proof command exists.
- `fresh_batch` stages one writable copy when the intended project has no live
  owner metadata and no Unity lock.
- `skipped_fail_closed` stops when project ownership is ambiguous or no safe
  proof route exists.

This wrapper proves editor and batch liveness only. Unity-MCP identity belongs
to #4739, and IL post-processor diagnosis belongs to #5332.

## Required inputs

Use the repository-installed owner binary and the configured Unity editor:

```sh
export ADL_UNITY_OBSERVATORY_ADL_BIN=/Users/daniel/git/agent-design-language/.adl/bin/adl
export ADL_UNITY_OBSERVATORY_ADL_SOURCE_ROOT=/Users/daniel/git/agent-design-language
export UNITY_EDITOR_BIN=/Applications/Unity/Hub/Editor/6000.5.1f1/Unity.app/Contents/MacOS/Unity
```

The owner binary must have a matching
`.adl/bin/.provenance/adl.sha256` receipt for the declared source root. The
wrapper does not search other worktrees for an arbitrary installed binary.

Set `ADL_UNITY_OBSERVATORY_PROJECT_PATH` when the intended project is not the
repository default. Staged data uses
`/Volumes/FastWork/adl-unity-observatory` by default. Overrides are accepted
only under `/Volumes/FastWork` or the issue worktree's `.adl` directory.

## Exact project ownership

An editor owner may publish its exact PID to a dedicated metadata file:

```sh
export ADL_UNITY_OBSERVATORY_PROJECT_OWNER_PID_FILE=/path/to/unity-editor.pid
```

The wrapper checks that file only through:

```sh
adl process status --pid-file /path/to/unity-editor.pid --json
```

It does not use broad `ps`, `pgrep`, or `lsof` output. A live PID without
`ADL_UNITY_OBSERVATORY_OPEN_EDITOR_PROOF_COMMAND`, or a Unity lock without
verified live-owner metadata, produces `skipped_fail_closed`.

An editor-mediated command receives the canonical project, log, and result
paths through environment variables. It must write
`adl.unity_editor_liveness.open_editor_result.v1` JSON with the matching
project and log paths, `terminal_outcome=passed`, and
`semantic_progress=true`. The wrapper then revalidates the exact owner PID.
A successful command without that structured result fails closed.

## Progress and terminal truth

Fresh batch mode advances its idle watchdog only when a previously unseen
import, compile, validation, or terminal stage appears. Repeating semantic or
unrelated log text is not progress, and there is no arbitrary total runtime
ceiling.

Readonly-database text is a blocker only when no later import marker proves
progress. Crash and licensing markers have their own outcomes. An optional
external classifier may return `blocked:<bounded_reason>` through
`ADL_UNITY_OBSERVATORY_EXTERNAL_CLASSIFIER_COMMAND`; the wrapper records the
reason without embedding #5332-owned signatures or thresholds.

Every terminal result prints:

- editor version;
- canonical project;
- selected mode;
- permission-safe process evidence;
- progress classifier;
- repository-relative log reference when applicable;
- exact success or blocker outcome.

## Focused proof

Run deterministic proof first:

```sh
bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption_unit.sh
bash adl/tools/test_v0916_unity_observatory_contract.sh
bash adl/tools/test_select_validation_lanes.sh
```

Then run one live or staged attempt:

```sh
bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh
```

The live attempt may succeed or retain a precise fail-closed blocker. Neither a
fixture nor a mode-classification result alone proves the Unity demo.
