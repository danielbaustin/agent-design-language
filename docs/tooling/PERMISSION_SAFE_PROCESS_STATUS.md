# Permission-Safe Process Status

Issue: #4136
Promotes: LB-121
Version seed: v0.91.6

## Purpose

ADL agents need a safe way to answer narrow questions such as "is the server
PID I started still alive?" or "is this local port bound?" without asking macOS
for broad process-inspection permissions or dumping host process tables.

The first supported surface is:

```sh
adl process status --pid-file .adl/runs/demo/server.pid --json
adl process status --pid 12345 --json
adl process status --port 8787 --json
adl process status --name adl-demo-server --json
```

The command is status-only. It does not stop, restart, signal, or supervise
processes.

## Safe Order

1. Prefer ADL-owned run metadata written when the process starts.
2. Check an exact known PID with a bounded liveness probe.
3. Check a local port with a bind probe when the workflow only needs to know
   whether an address is available or already in use.
4. Treat a process name without ADL metadata as `unknown` unless a future issue
   adds a documented, narrow, capped, platform-specific fallback.
5. Do not use broad process listings as normal workflow control.

Normal ADL checks must not default to:

- `ps aux`
- `ps -ef`
- broad `pgrep` scans
- broad `lsof` process dumps

## Run Metadata

ADL-started long-lived helpers should write metadata near their run artifact,
for example:

```text
.adl/runs/<run-id>/server.pid
.adl/runs/<run-id>/server.status.json
```

The PID file must be a regular file containing only the decimal PID and a
trailing newline. Richer metadata should avoid secret-bearing command arguments.
Recommended fields are:

- schema
- run id
- pid
- host
- port
- started_at
- component name

The status helper intentionally does not echo the PID file path in JSON output,
so machine consumers do not accidentally persist host-local absolute paths. PID
files are read as bounded tiny metadata files rather than unbounded text inputs.

## Status Classifications

The JSON schema is `adl.process_status.v1`.

`--pid` and `--pid-file` may report:

- `live_pid`
- `stale_pid`
- `missing_metadata`
- `invalid_metadata`
- `unknown`

`--port` may report:

- `bound_port`
- `unbound_port`
- `unknown`

`--name` reports `unknown` in this slice. That is intentional: without ADL-owned
metadata, a name lookup would otherwise push agents toward host-wide process
inspection.

Every report includes:

- `broad_process_scan: false`
- `uses_ps: false`

Machine-readable JSON is emitted on stdout. Human diagnostics and normal ADL
observability, when present, belong on stderr under the existing logging
contract.

## Platform Notes

PID liveness uses an in-process exact `kill(pid, 0)` probe on Unix platforms.
This checks only the declared PID and does not inspect command lines,
environment variables, or other user processes. Permission-denied responses mean
the PID exists but is not signalable by the current user, so they classify as
`live_pid`.

Port status uses a local `TcpListener` bind probe against the requested host and
port. The default host is `127.0.0.1`. Port probes are limited to loopback
targets: `127.0.0.1`, `::1`, or `localhost`.

Any future platform fallback must be issue-scoped, optional, narrow, capped, and
documented before use. It must preserve the no-`ps aux` and no-`ps -ef` rule.
