# Long-Lived Agent Runtime Feature Set

## Status

v0.90 sprint-ready feature-doc set.

This package defines the missing runtime pieces needed to move from bounded
fixture demos to long-lived agent demos that can run for hours, days, or weeks
under explicit operator control.

It does not move or replace the `v0.92` identity/capability substrate plan.
Everything here uses a pre-identity continuity handle and must remain
compatible with a later migration into the full `v0.92` identity model.

## Problem Statement

ADL currently has strong bounded execution primitives:

- deterministic run execution
- persisted run artifacts
- strict step-boundary pause/resume
- steering at resume boundaries
- trace and runtime-state artifacts
- ObsMem contract surfaces for cross-run memory

Those are necessary, but they are not sufficient for long-lived agents.

The missing runtime layer is an explicit supervisor that can:

1. wake an agent on a schedule or manual tick
2. gather observations
3. run a bounded ADL workflow
4. persist cycle outputs and memory
5. enforce leases, stop conditions, and safety gates
6. expose truthful status without pretending this is the full `v0.92` identity
   system

## Core Claim

ADL can support long-lived agents by treating each agent as a sequence of
bounded, replayable ADL cycles managed by a durable supervisor.

The agent is "long-lived" because its continuity record, memory references,
cycle ledger, and operator controls persist across cycles. The individual ADL
workflow execution remains bounded and replayable.

## Feature Docs In This Package

- `FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md`
  - native supervisor loop, tick scheduling, leases, and lifecycle commands
- `FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md`
  - observe -> decide -> persist cycle contract and artifact schema
- `FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md`
  - pre-v0.92 continuity handles, ledgers, memory links, and migration boundary
- `FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md`
  - start/status/stop controls, guardrails, failure policy, and safety truth

## Non-Goals

These are explicitly out of scope for this sprint package:

- implementing the full `v0.92` identity tuple substrate
- model/provider capability governance beyond what the provider substrate
  already exposes
- mid-step checkpointing
- distributed checkpoint recovery
- autonomous execution without operator-configured schedules or limits
- broker integration, real trading, or financial advice in the stock demo
- hidden background agents without durable status artifacts

## Sprint Implementation Slice

The sprint should implement a minimal but real long-lived runtime surface:

1. A supervisor command or tool that supports `tick`, `run`, `status`, and
   `stop`.
2. A long-lived agent spec file that binds:
   - `agent_instance_id`
   - display name
   - workflow path
   - state root
   - heartbeat interval
   - max cycle count or stop time
   - provider/model preferences
   - safety policy
3. A cycle ledger under a durable state root.
4. One lease file that prevents overlapping cycles for the same agent.
5. One status artifact that a reviewer can read without tailing logs.
6. A stock-league demo mode that runs at least multiple consecutive cycles
   without deleting prior journals.
7. Tests proving stop/resume/restart behavior and overlap protection.

## Recommended Minimal CLI

The final command shape can be adjusted during implementation, but the feature
docs assume this operator model:

```bash
adl agent tick --spec <agent-spec.yaml>
adl agent run --spec <agent-spec.yaml> --max-cycles <n>
adl agent status --spec <agent-spec.yaml>
adl agent stop --spec <agent-spec.yaml> --reason <text>
```

If we do not want to add a Rust CLI subcommand in the first slice, implement the
same contract as:

```bash
python3 adl/tools/long_lived_agent_supervisor.py tick --spec <agent-spec.yaml>
python3 adl/tools/long_lived_agent_supervisor.py run --spec <agent-spec.yaml> --max-cycles <n>
python3 adl/tools/long_lived_agent_supervisor.py status --spec <agent-spec.yaml>
python3 adl/tools/long_lived_agent_supervisor.py stop --spec <agent-spec.yaml> --reason <text>
```

The artifact contract must be the same either way.

## Artifact Root

Default state layout:

```text
.adl/long_lived_agents/<agent_instance_id>/
  agent_spec.locked.json
  continuity.json
  status.json
  stop.json
  lease.json
  cycles/
    cycle-000001/
      cycle_manifest.json
      observations.json
      decision_request.json
      decision_result.json
      run_ref.json
      memory_writes.jsonl
      guardrail_report.json
      cycle_summary.md
    cycle-000002/
      ...
```

For demo artifacts, the same structure may be mirrored under:

```text
artifacts/v090/<demo_name>/long_lived_agents/<agent_instance_id>/
```

## Truth Model

Every long-lived run must answer these questions from artifacts alone:

- Is the agent running, stopped, paused, failed, or completed?
- What was the last completed cycle?
- What workflow was run for each cycle?
- What observations were available before each decision?
- What decision was made?
- What memory was written?
- Was any guardrail violated?
- Is another cycle currently leased or in progress?
- What operator configured the run?
- What stop condition applies?

## Acceptance Tests

Minimum sprint tests:

1. `tick` creates cycle `000001` and writes status.
2. A second `tick` creates cycle `000002` and preserves cycle `000001`.
3. `run --max-cycles 3` writes exactly three cycles and exits `completed`.
4. An existing active lease blocks another concurrent tick.
5. A stale lease older than policy is reported as recoverable and not silently
   ignored.
6. `stop` prevents new cycles and records an operator reason.
7. Status can be read after process exit.
8. No artifact contains secrets or absolute host paths where public demo
   artifacts are expected.
9. The stock-league demo can run multiple cycles using fixture observations and
   shows continuity across cycles.

## Implementation Order

1. Implement state layout and schema helpers.
2. Implement `tick` with a mock workflow or stock-league fixture cycle.
3. Add lease and status handling.
4. Add `run --max-cycles`.
5. Add stop file handling.
6. Wire the stock-league demo into multi-cycle mode.
7. Add regression tests.
8. Add one demo-operator run that proves multi-cycle behavior.

## Relationship To v0.92

This package introduces only a `continuity_handle`, not the full identity
system.

The handle is intentionally small:

- `agent_instance_id`
- `continuity_schema`
- creation metadata
- workflow binding
- memory namespace
- cycle ledger pointer
- future `v0.92_identity_ref` placeholder

When v0.92 lands, it can adopt or migrate these handles. Until then, the runtime
must avoid claiming full identity semantics.
