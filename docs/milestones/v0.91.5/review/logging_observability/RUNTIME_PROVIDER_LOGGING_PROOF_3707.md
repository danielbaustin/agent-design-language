# Runtime And Provider Logging Correlation Proof (#3707)

Issue: #3707  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: implementation_refreshed

## Summary

This packet records the refreshed `#3707` implementation slice after the
logging mini-sprint was re-audited against current repo truth.

The repo already had two real observability baselines:

- runtime `logs/action_log.jsonl` projection from canonical trace events; and
- provider invocation JSONL / result artifacts from the Rust provider adapter.

The remaining gap was not "build a second runtime log system." It was provider
correlation hygiene:

1. provider results did not carry the stable `request_id` that downstream
   reviewers and tooling need to correlate repeated invocations;
2. provider run-log rows did not carry stable request or artifact references;
3. the provider adapter CLI printed result/log paths for humans, but the JSON
   evidence itself did not preserve those refs in a safe repo-relative or
   explicitly redacted form.

## Implemented Slice

This issue makes one bounded correlation improvement on the provider side while
keeping runtime action logs as the canonical runtime projection:

- `ProviderInvocationResultV1` now preserves optional `request_id`.
- `ProviderRunLogEventV1` now preserves optional `request_id` and
  `artifact_ref`.
- `ProviderRunLoggerV1` can inject request/log correlation context into every
  emitted provider event.
- `adl-provider-adapter` now writes a safe `artifact_ref` for the result
  artifact and emits a safe `artifact_ref` on the provider run log.
- absolute temp/home paths are normalized to bounded refs such as
  `<tmp>/file.json` and `<home>/file.json`; repo files remain repo-relative.

## Correlation Model

The intended operator/reviewer read path is now:

1. use runtime `logs/action_log.jsonl` for stage/result/reason chronology;
2. use provider JSONL for attempt-by-attempt provider behavior;
3. correlate by `run_id`, `request_id`, provider/model identity, and bounded
   artifact refs;
4. consult canonical trace/replay artifacts for deeper governed detail rather
   than trying to turn the provider log into a second truth source.

This keeps the runtime action log authoritative for runtime chronology while
making provider artifacts much easier to inspect without raw prompts, payloads,
credentials, or host-local absolute paths.

## Proof Commands

### Provider communication and log context

```bash
cargo test --manifest-path adl/Cargo.toml provider_communication -- --nocapture
```

Expected proof:

- provider result validation still works transitively;
- provider-run logger flushes JSONL rows safely;
- request/artifact context is injected into emitted provider log rows;
- redaction still strips secrets and prompt bodies.

### Provider adapter CLI correlation and path hygiene

```bash
cargo test --manifest-path adl/Cargo.toml provider_adapter_cli -- --nocapture
```

Expected proof:

- failure results still normalize to stable failure kinds;
- `request_id` survives into `ProviderInvocationResultV1` on the CLI failure
  path and on direct adapter success/failure paths;
- result and log artifact refs are preserved in bounded form;
- repo files serialize repo-relative refs when possible;
- temp or home paths are redacted instead of leaking absolute host-local paths.

### Runtime action-log baseline

```bash
cargo test --manifest-path adl/Cargo.toml instrumentation::action_log -- --nocapture
```

Expected proof:

- runtime action-log projection still emits deterministic `stage`, `component`,
  `result`, `provider_ref`, `reason_code`, and `artifact_write` surfaces;
- raw runtime failure messages are still not copied into the action log;
- runtime evidence remains a projection over trace, not a competing truth
  source.

## Non-Claims

- This issue does not claim that every runtime event now embeds provider
  `request_id`; runtime chronology and provider invocation evidence remain
  separate surfaces by design.
- This issue does not redesign the canonical trace format.
- This issue does not add hosted provider spend or live hosted proof.
- This issue does not complete heartbeat/timeout observability; that remains
  `#3708`.
