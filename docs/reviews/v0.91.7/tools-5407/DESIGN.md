# Tools Reliability Review Fix (#5407)

## Decision

Resolve the four #5036 review findings by correcting current repository truth.
The supported `adl.build_action_log.v1` producer remains
`validation_manager.py --run`; removed v1 lifecycle wrappers and unimplemented
CI, remote-builder, watcher, and closeout consumers are not claimed. Future
producer or consumer expansion requires a separately reviewed issue.

## Changes

1. Narrow build-action-log documentation to the implemented producer and list
   the original #5032 surfaces as explicit non-claims.
2. Replace the stale CLI taxonomy with Gate 10D2 typed C-SDLC v2 authority.
3. Retain a #5036 closeout synthesis covering every declared child, including
   #5037 and #4938, with issue and PR closure evidence.
4. Record #5037 as a focused CI contract split only. No material wall-clock
   speedup is claimed without comparable hosted before/after runs.

## Invariants

- Gate 10D2 `v1_sunset` remains authoritative.
- Documentation must distinguish implemented behavior from future work.
- Existing validation-manager packet behavior and tests remain unchanged.
- Closeout evidence must cover the complete declared sprint wave.
- Local timing cannot prove hosted CI speedup.

## Validation

- focused searches reject active v1 wrapper guidance and material-speedup claims
- shell/tooling documentation checks remain green
- all retained child/PR rows are present in the closeout synthesis
- `git diff --check` passes

## Non-Goals

- implementing new build-action-log producers or consumers
- restoring `pr.sh`, compatibility binaries, or v1 lifecycle commands
- claiming hosted performance improvement from local timings
- changing CI routing or validation-manager runtime behavior
