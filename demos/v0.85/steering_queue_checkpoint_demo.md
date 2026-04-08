# Steering / Queueing / Checkpoint Demo

This bounded demo is the milestone-facing proof surface for ADL's current
queue, checkpoint, resume, and steering substrate.

It demonstrates one deterministic human-in-the-loop control cycle:

1. a run executes until an explicit pause boundary
2. checkpoint state is emitted to a pause-state artifact
3. a steering patch is applied at the resume boundary
4. the resumed run produces output changed by the steered state
5. the run record captures the steering history explicitly

It does not claim a full durable distributed checkpoint engine. It proves the
currently landed bounded control surface.

## One-command demo

From repository root:

```bash
adl/tools/demo_steering_queue_checkpoint.sh
```

## Primary proof artifacts

- `.adl/runs/v0-85-hitl-steering-demo/pause_state.json`
- `.adl/runs/v0-85-hitl-steering-demo/run.json`
- `.adl/reports/demo-steering-queue-checkpoint/steer.json`
- `.adl/reports/demo-steering-queue-checkpoint/out/s2.txt`

## What to inspect

- `pause_state.json`
  - `status` should be `paused`
  - `pause.paused_step_id` should be `s1`
  - `pause.remaining_step_ids` should include `s2`
- `steer.json`
  - should use schema `steering_patch.v1`
  - should set `inputs.topic` at `resume_boundary`
- `out/s2.txt`
  - should contain `steered-topic`
- `run.json`
  - should include one `steering_history` entry
  - `set_state_keys` should include `inputs.topic`

## Why this matters

This is the concrete bounded proof that ADL can expose execution control at a
checkpoint boundary while keeping the steering action explicit, auditable, and
replay-compatible.
