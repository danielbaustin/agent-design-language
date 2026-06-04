# Runtime Action Log Contract (#3556)

Issue: #3556
Captured: 2026-06-04
Status: first_slice_implemented

## Purpose

Runtime and Software Development Polis actions need a deterministic,
reviewer-facing log that answers:

- what happened;
- which runtime or polis component acted;
- which bounded input or output references were involved;
- what decision or result was reached;
- why the decision was made, when a safe reason code exists.

The first slice adds `logs/action_log.jsonl` as a generated run artifact. It is
derived from the canonical trace event stream, so it improves observability
without creating a competing truth source.

## Artifact

Path:

```text
.adl/runs/RUN_ID/logs/action_log.jsonl
```

Manifest reference:

```text
logs/action_log.jsonl
```

Schema:

```text
adl.runtime_action_log.v1
```

Each JSONL line is one `RuntimeActionLogEvent` in trace emission order.

## Event Fields

| Field | Meaning |
| --- | --- |
| `schema_version` | Stable action-log schema identifier. |
| `sequence` | Trace-order sequence number. |
| `stage` | Bounded runtime stage such as `step`, `policy`, `provider_call`, `freedom_gate`, or `artifact_write`. |
| `component` | Runtime or polis component such as `runtime`, `delegation`, or `polis`. |
| `result` | Bounded result such as `started`, `ok`, `success`, `failure`, `denied`, `allow`, or `recorded`. |
| `step_id` | Step identifier when the event is step-scoped. |
| `actor` | Agent or bounded actor reference when safe. |
| `provider_ref` | Provider, tool, adapter, or target reference when safe. |
| `action_ref` | Proposal, action, workflow, or delegation action reference when safe. |
| `input_refs` | Bounded input references such as prompt hashes, task ids, delegation ids, namespaces, or redacted argument refs. |
| `output_refs` | Bounded output references such as artifact refs, evidence refs, output byte counts, or result refs. |
| `reason_code` | Safe reason code or policy/routing code when available. |
| `elapsed_ms` | Runtime elapsed milliseconds copied from trace event timing. |

## Redaction Rules

The first slice does not include raw prompts, raw model output, raw provider
payloads, private tool arguments, credentials, private keys, or absolute
host-local paths.

For failures, the log records `reason_code=runtime_failure` rather than copying
the full error message. Full diagnostic detail remains governed by existing
trace/review artifacts and redaction policy.

## Relationship To Existing Evidence

`action_log.jsonl` is a projection over trace events:

- `logs/activation_log.json` remains the normalized trace artifact.
- `logs/trace_v1.json` remains the trace-v1 replay/review envelope.
- `run_manifest.json` advertises `logs/action_log.jsonl` as a generated run
  artifact.
- Control-path and cognitive artifacts remain their existing domain-specific
  proof surfaces.

This keeps action logs inspectable without making them an independent source of
runtime truth.

## First Slice Coverage

The first implementation projects these trace surfaces:

- runtime lifecycle and execution boundaries;
- scheduler policy selection;
- run success/failure;
- step start, prompt assembly, output chunks, and step finish;
- delegation policy/request/dispatch/result/completion events;
- nested workflow call entry/exit;
- governed polis proposal, policy, visibility, freedom-gate, action,
  execution-result, refusal, and redaction events.
- core generated run-artifact writes advertised by `run_manifest.json`.

## Deferred Work

The following remains intentionally out of scope for this first slice:

- direct emission of action-log events from every runtime validation branch;
- `artifact_write` events for every low-level `atomic_write` call beyond the
  core generated run-artifact manifest;
- OpenTelemetry exporter wiring;
- long-running runtime span lifecycle;
- hosted collector setup;
- external observability dashboards;
- trace-query indexing over action logs.

Those should be split into follow-ons if needed after this artifact is proven in
normal runtime runs.

## Validation

Focused validation for this slice:

```bash
cargo test --manifest-path adl/Cargo.toml instrumentation::action_log -- --nocapture
cargo test --manifest-path adl/Cargo.toml artifacts::tests::path_accessors_cover_all_canonical_artifact_locations -- --nocapture
```

These tests prove:

- action-log projection is deterministic;
- raw failure messages are not copied into the log;
- JSONL writer emits one valid event per line;
- `RunArtifactPaths` exposes the canonical `logs/action_log.jsonl` path.
