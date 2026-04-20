# Runtime v2 Foundation Demo

## Purpose

This is the WP-12 reviewer-facing demo for v0.90.1. It integrates the Runtime
v2 foundation artifacts from WP-05 through WP-11 into one bounded proof packet.

## Run

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-l-v0901-runtime-v2-foundation --run --trace --out artifacts/v0901 --no-open
```

The primary proof surface is:

```text
artifacts/v0901/demo-l-v0901-runtime-v2-foundation/runtime_v2/proof_packet.json
```

For a direct contract-only generation path, use:

```bash
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 foundation-demo --out artifacts/v0901/demo-l-v0901-runtime-v2-foundation
```

## Expected Artifacts

- `runtime_v2/proof_packet.json`
- `runtime_v2/manifold.json`
- `runtime_v2/kernel/service_registry.json`
- `runtime_v2/kernel/service_state.json`
- `runtime_v2/kernel/service_loop.jsonl`
- `runtime_v2/citizens/proto-citizen-alpha.json`
- `runtime_v2/citizens/proto-citizen-beta.json`
- `runtime_v2/citizens/active_index.json`
- `runtime_v2/citizens/pending_index.json`
- `runtime_v2/snapshots/snapshot-0001.json`
- `runtime_v2/rehydration_report.json`
- `runtime_v2/invariants/violation-0001.json`
- `runtime_v2/operator/control_report.json`
- `runtime_v2/security_boundary/proof_packet.json`
- `runtime_v2/reviewer_boundary_notes.md`
- `trace.jsonl`

## Classification

`proving` for the bounded v0.90.1 claim that a reviewer can inspect the Runtime
v2 foundation prototype end to end.

## Non-Claims

- This does not prove first true Gödel-agent birth.
- This does not prove full moral, emotional, or polis governance.
- This does not prove live scheduling, migration, or full red/blue/purple
  defense ecology.
