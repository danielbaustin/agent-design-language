# CSM Observatory CLI Demo

## What This Is

This is the command-driven CSM Observatory demo surface. It packages the
fixture-backed visibility packet, operator report, and visual-console reference
into one reviewable artifact directory.

Run:

```bash
bash adl/tools/demo_v0901_csm_observatory.sh
```

The default output directory is:

```text
artifacts/v0901/csm-observatory
```

## Direct CLI Command

The demo script wraps:

```bash
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- csm observatory --packet demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json --format bundle --out artifacts/v0901/csm-observatory
```

Bundle mode writes:

- visibility_packet.json
- operator_report.md
- console_reference.md
- demo_manifest.json

## Truth Boundary

Demo classification: fixture_backed.

This demo proves the CLI packaging path for CSM Observatory visibility. It does
not prove a live CSM run, live Runtime v2 capture, live mutation, snapshot/wake
completion, or v0.92 identity rebinding.

All command behavior is read-only. The CLI validates and renders the packet; it
does not mutate Runtime v2 state.

Future operator actions are governed by the v0.90.1 command packet contract.
That contract defines how pause, resume, snapshot, trace annotation, and
shepherd requests must become kernel-routed command packets with operator-event
logging before any live mutation is allowed.

## Why This Matters

The static console is the spectacular human surface. The operator report is the
reviewer notebook. This CLI demo is the reproducible packaging path that lets a
reviewer regenerate both proof surfaces from the packet without manual file
assembly.
