# Runtime v3 Soak And Rollback Proof

Issue: #5253
Target: v0.91.7

## Result

#5253 retains a production-like Runtime v3 soak and rollback proof without
changing the default runtime. The proof uses explicit Runtime v3 selection,
keeps Runtime v2 as the rollback/default target, and records that no v2 deletion
or decommission is authorized by this issue.

The machine-readable retained packet is:

```text
docs/architecture/runtime_v3_soak_rollback_5253.v1.json
```

## Proof Boundary

The retained #5175 soak remains the bounded execution source: 100 cycles, 16
items per cycle, continuity generation 100, injected component restart, fatal
child recovery, queue saturation, corrupt-continuity quarantine, degraded clock
startup, and shutdown-deadline handling.

#5253 adds the cutover-specific routing around that evidence:

- Runtime v3 is explicit opt-in only.
- Runtime v2 remains the default and rollback target.
- Control policy remains local on `127.0.0.1:20997`.
- Stdout/stderr contract truth remains separated.
- Remote multi-day soak and GPU telemetry are deferred non-cutover lanes.
- Horust 0.1.13 remains blocked for bounded restart until a fixed release is
  qualified.

## Parity Routing

This issue resolves the `guardian.packaging_soak` blocker in the retained live
black-box parity packet. Capability-specific shared-fixture questions are routed
to #5254, the final default-switch/decommission decision gate, instead of being
overclaimed as soak proof.

## Non-Claims

- This packet does not authorize Runtime v3 as the default runtime.
- This packet does not delete or decommission Runtime v2.
- This packet does not claim observed GPU telemetry.
- This packet does not claim a fixed Horust release.
