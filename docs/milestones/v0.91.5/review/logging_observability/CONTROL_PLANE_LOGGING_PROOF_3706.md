# Control-Plane Logging Proof (#3706)

Issue: #3706  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: implementation_refreshed

## Summary

This packet records the refreshed `#3706` proof slice after the logging
mini-sprint was re-audited against current repo truth.

The remaining bounded implementation gap was not "add logging everywhere from
scratch." The repo already had:

- shell `adl_event` emission in `adl/tools/observability.sh`;
- Rust dispatcher/event emission in `adl/src/cli/observability.rs`;
- Octocrab operation logs in `adl/src/cli/pr_cmd/github.rs`;
- stage-oriented `doctor`, `finish`, and `closeout` control-plane work from
  earlier logging fixes.

The refreshed gap was consistency and policy:

1. Octocrab transport still emitted raw `eprintln!` lines instead of the shared
   Rust observability helper, so it bypassed durable-log mirroring and any
   explicit stderr policy.
2. JSON-mode compatibility needed a governed escape hatch for callers that need
   stdout-only JSON *and* quiet stderr without disabling observability
   entirely.

## Implemented Slice

The refreshed implementation does three bounded things:

- Rust observability events now mirror to `ADL_OBSERVABILITY_LOG`, matching the
  shell helper, as an explicit compatibility mirror rather than a governed
  JSON/JSONL durable event store.
- Rust and shell observability now honor the documented compatibility toggle
  `ADL_OBSERVABILITY_STDERR=0`.
- Octocrab transport events now route through the shared Rust observability
  helper instead of direct `eprintln!` calls.

## JSON-Mode Policy

The current policy is now explicit and shared across the tracked contracts:

- default behavior: observability stays on stderr and machine-readable payloads
  remain on stdout;
- compatibility behavior for strict JSON consumers:
  - `ADL_OBSERVABILITY_STDERR=0`
  - `ADL_OBSERVABILITY_LOG=<compatibility-log-path>`

Under that compatibility mode, stdout remains JSON-only while observability is
still preserved on a separate explicit compatibility mirror. The governed
durable machine-readable layer remains JSON/JSONL and is not replaced by this
file sink. If the compatibility sink cannot be created or written while
`ADL_OBSERVABILITY_STDERR=0` is active, the Rust helper now emits one bounded
fallback failure event on stderr instead of silently dropping observability.

## Proof Commands

### Shell control-plane helper

```bash
bash adl/tools/test_control_plane_observability.sh
```

Expected proof:

- repo/home/tmp path redaction still works;
- `ADL_OBSERVABILITY=0` still fully disables emission;
- `ADL_OBSERVABILITY_STDERR=0` suppresses terminal output while durable logging
  continues in the compatibility mirror file;
- when the compatibility sink path is invalid under quiet-stderr mode, the
  helper emits one bounded `stage=compatibility_log result=failed` fallback
  line instead of losing the event silently.

### Rust observability helper

```bash
cargo test --manifest-path adl/Cargo.toml cli::observability::tests -- --nocapture
```

Expected proof:

- secret/path redaction still works;
- Rust helper appends to `ADL_OBSERVABILITY_LOG`;
- Rust helper remains compatible with the quiet-stderr configuration used by
  shell-level proof, without changing the governed JSON/JSONL durable-layer
  contract;
- sink-open/write failures are classified explicitly instead of being ignored.

### Octocrab transport path

```bash
cargo test --manifest-path adl/Cargo.toml \
  octocrab_transport_honors_quiet_stderr_compatibility_log \
  -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  octocrab_transport_covers_pr_and_issue_operations_against_mock_github \
  -- --nocapture
```

Expected proof:

- GitHub operation events still emit `started`/`completed` control-plane logs;
- the path mirrors events into `ADL_OBSERVABILITY_LOG` under the explicit
  quiet-stderr compatibility configuration;
- the refreshed transport path compiles and runs through the shared
  observability helper rather than direct transport-specific `eprintln!`
  formatting.

## Non-Claims

- This issue does not prove bounded heartbeat/progress for every long-running
  path; that remains `#3708`.
- This issue does not add OTEL export wiring; that remains `#3709`.
- This issue does not claim every JSON-producing tool path auto-enables quiet
  stderr. The supported compatibility mode is explicit rather than implicit.
