# Build Action Logs

ADL build and validation commands can retain durable build-action packets when
the command surface implements this contract.

## Implemented Surface

The only integrated producer is `adl/tools/validation_manager.py --run`.
It writes one packet per selected validation lane and a manifest for the run.
This is the complete current implementation boundary, not an assertion that
all ADL build or lifecycle commands emit `adl.build_action_log.v1` packets.

## Schema

Each packet uses `schema_version: adl.build_action_log.v1` and records:

- `runner`: command surface that created the packet, such as
  `validation_manager`
- `lane_id` and `reason`: validation-lane identity and selection reason
- `command` and `command_sha256`: command text plus a stable digest
- `cwd`, `binary_path`, and `cache_posture`: execution context
- `started_at`, `ended_at`, `elapsed_ms`, `exit_code`, and `status`
- `stdout_ref`, `stderr_ref`, and `packet_ref`: durable evidence refs
- `redaction_status` and `retention`: truth about publication posture

The manifest uses `schema_version: adl.build_action_log_manifest.v1` and lists
the packet refs emitted by one validation-manager run.

## Validation Manager

`validation_manager.py --run` writes build-action logs by default under:

```text
.adl/logs/build-actions/validation-manager/<timestamp>/
```

The default run directory includes a UTC timestamp and process id to avoid
overwriting packets from another validation-manager run started in the same
second. Use `--build-action-log-dir <path>` or
`ADL_BUILD_ACTION_LOG_DIR=<path>` for a bounded proof directory. When the
directory is inside the repository, refs are repo-relative. Explicit external
directories may produce absolute local refs and should stay out of tracked
artifacts.

The command replays captured stdout/stderr after each lane exits so existing
human-facing behavior remains available while durable logs are retained.

## Explicit Non-Claims

The original #5032 proposal named additional producers and consumers. They are
not implemented by this contract and are not current claims:

- the removed v1 `adl/tools/pr.sh finish` lifecycle wrapper
- owner-lane commands run outside `validation_manager.py --run`
- remote builders
- CI ingestion of build-action packets
- watcher or shepherd reporting
- closeout fail-closed behavior based on build-action packets

Gate 10D2 sunset the v1 lifecycle wrappers, so documentation does not preserve
those removed commands as pending integration surfaces. Any expansion to a
live typed C-SDLC v2 binary, CI, remote execution, or closeout must be delivered
by a separate reviewed issue with producer/consumer tests and retained proof.

## Evidence Boundaries

This surface records private workflow evidence. It does not redact raw command
logs, upload logs, or claim hosted observability. CI log archive manifests remain
the CI raw-log evidence surface. Current build-action packets do not imply that
CI, remote builders, lifecycle owners, or closeout consumed the packet.
