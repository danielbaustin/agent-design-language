# Shared Observability And OTEL Contract (#3705)

Issue: #3705  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: design_contract

## Purpose

ADL already emits several useful logging and observability artifacts:

- control-plane `adl_event` terminal lines for shell and Rust CLI execution;
- runtime `logs/action_log.jsonl` action-log projections;
- provider invocation and review-provider JSONL logs;
- long-lived-agent ledgers, status files, and operator event streams;
- proof packets that record timing and execution evidence for bounded workflows.

Those surfaces are real, but they are not yet governed by one shared contract.
This document defines the common vocabulary and migration boundary so later
issues can unify behavior without inventing incompatible formats or forcing
OpenTelemetry into every local workflow.

## Problem Statement

Without a shared observability contract:

- different subsystems can log the same event with incompatible names;
- JSON consumers can be broken by stdout/stderr ambiguity;
- heartbeat and timeout behavior stays inconsistent;
- review packets can overstate OTEL or correlation maturity;
- Observatory and long-lived agents can drift into separate telemetry truth.

## Goals

- Define one bounded vocabulary for ADL observability across control-plane,
  runtime, provider, long-lived-agent, and Observatory-facing surfaces.
- Preserve existing durable artifacts and current `adl_event` usage while
  making later convergence and OTEL export explicit.

## Non-Goals

- This issue does not wire every existing call site to the new contract.
- This issue does not require a hosted OTEL collector or make OTEL a local test
  dependency.
- This issue does not authorize logging raw prompts, raw provider payloads,
  secrets, private tool arguments, or host-local absolute paths.

## Existing Surfaces Covered By This Contract

| Surface | Current shape | Shared contract role |
| --- | --- | --- |
| Shell and Rust control-plane events | `adl_event schema=adl.observability.event.v1 ...` | Human-readable stage/progress/failure events, optionally mirrored into a durable log. |
| Runtime action log | `logs/action_log.jsonl` / `adl.runtime_action_log.v1` | Durable machine-readable projection over canonical runtime trace events. |
| Provider run logs | provider invocation/result and review-provider JSONL | Durable machine-readable provider execution evidence with redacted result and failure details. |
| Long-lived-agent status, ledgers, and operator events | `status.json`, `cycle_ledger.jsonl`, `provider_binding_history.jsonl`, `operator_events.jsonl` | Domain-specific durable operational records that must map into the shared vocabulary instead of remaining isolated telemetry. |
| Review/proof packets | tracked milestone review docs and JSON packets | Reviewer-facing evidence surfaces that may cite observability fields but do not replace canonical event artifacts. |

## Contract Layers

### 1. Terminal Event Layer

The terminal layer exists so operators can answer:

- what command or subsystem started;
- what stage is currently running;
- whether the process is waiting, progressing, blocked, or failed;
- what durable artifact or issue/PR/run ref to inspect next.

Current baseline:

```text
adl_event schema=adl.observability.event.v1 command=<command> stage=<stage> result=<result> key=value ...
```

This line-oriented format remains valid. Migration must preserve compatibility
for current scripts that already recognize `adl_event`.

### 2. Durable Machine-Readable Layer

The durable layer exists for replay, review, packet assembly, and downstream
correlation. It uses JSON or JSONL artifacts, not ad hoc terminal scraping.

Examples already in repo:

- `logs/action_log.jsonl`
- provider run logs / review provider logs
- `cycle_ledger.jsonl`
- `provider_binding_history.jsonl`
- `operator_events.jsonl`

### 3. Optional OTEL Export Layer

OTEL is an export and integration boundary, not the local source of truth.

- local deterministic files remain authoritative for review;
- OTEL export must remain optional and disable-safe;
- CI and ordinary local commands must not require a collector;
- OTEL mapping must be derivable from the shared vocabulary below.

## Shared Vocabulary

The shared vocabulary is centered on one event/span model with bounded fields.

### Core Fields

The table below defines the target shared vocabulary. Present repo surfaces do
not all use these exact names yet; some still expose equivalent domain-specific
fields such as `event_type`, `status`, `final_status`, `state`, or `event`.
Follow-on implementation issues should normalize those shapes toward this table
without pretending the migration is already complete.

| Field | Meaning | Requirement |
| --- | --- | --- |
| `schema` or `schema_version` | Stable event or artifact schema identifier. | Required |
| `command` | Top-level command family such as `pr.sh`, `adl`, `adl-runtime`, `adl-provider-adapter`. | Required for control-plane events; optional elsewhere when `component` is more specific |
| `component` | Bounded emitting subsystem such as `runtime`, `provider`, `long_lived_agent`, `observability`, `github_octocrab`, `polis`. | Required for records authored directly to the shared contract; recommended for terminal events; equivalent legacy emitter fields may persist during migration |
| `stage` | Bounded stage name such as `dispatch`, `doctor`, `provider_call`, `heartbeat`, `artifact_write`, `closeout`. | Required |
| `operation` | Named operation within a stage such as `issue.view.state`, `provider.invoke`, `status.recover`, `trace.project`. | Optional but recommended when the stage covers multiple remote or internal operations |
| `result` | Bounded status such as `started`, `progress`, `heartbeat`, `completed`, `failed`, `blocked`, `skipped`, `timeout`, `denied`, `recorded`. | Required |
| `reason_code` | Safe compact explanation such as `runtime_failure`, `generated_run_artifact`, `provider_auth_missing`, `stale_lease_recovered`. | Optional but preferred for failures, denials, and policy-driven outcomes |
| `elapsed_ms` or `duration_ms` | Millisecond timing for a stage, attempt, or completed operation. | Required on durable completion/failure records when timing is available; optional on start/progress events |
| `severity` | `debug`, `info`, `warning`, `error`, `critical` or bounded equivalent. | Optional for local text emission; recommended for structured event families |
| `artifact_ref` / `artifact_refs` | Repo-relative or run-relative durable artifact references. | Optional |
| `issue_ref`, `pr_ref`, `run_id`, `request_id`, `review_request_id`, `cycle_id`, `agent_instance_id` | Correlation references that connect events to ADL lifecycle or runtime execution. | Optional but recommended whenever a stable ref exists |
| `provider_ref`, `provider_model_id`, `model_ref`, `runtime_surface` | Provider/model correlation fields. | Optional outside provider/runtime lanes; required where provider behavior is the subject |
| `trace_ref` | Ref to the canonical trace or governed execution envelope when a durable trace artifact exists. | Optional but recommended for provider and runtime evidence surfaces |

### Result Vocabulary

The shared result vocabulary is:

- `started`
- `progress`
- `heartbeat`
- `completed`
- `failed`
- `blocked`
- `skipped`
- `timeout`
- `denied`
- `recorded`

Existing fields such as `ok`, `exec`, `allow`, `success`, or `wrote` remain
permitted during migration. So do currently-shipped domain-specific values such
as `retry`, `failure`, provider attempt `error`, provider attempt `timeout`,
provider `final_status`, and long-lived-agent `state` / `last_cycle_status`.
Later implementation issues should map those values toward the shared
vocabulary, or explicitly document why a domain-specific result is still
needed.

### Span Lifecycle

Long-running operations should emit a bounded lifecycle:

1. `started`
2. zero or more `progress` or `heartbeat`
3. terminal `completed`, `failed`, `blocked`, `skipped`, or `timeout`

This applies to:

- long-running control-plane workflows such as `doctor`, `finish`, and
  `closeout`;
- provider invocations or reviewer-provider runs;
- long-lived-agent cycles and lease recovery paths;
- runtime steps or projections when a bounded long-running span is visible to
  operators.

## Channel Policy

### Terminal Channel

- Human-readable observability belongs on stderr by default.
- Machine-readable command output on stdout must remain parse-safe.
- If a command advertises JSON mode, stdout must contain only the declared JSON
  payload unless the interface explicitly says otherwise.
- When a caller needs quiet stderr for JSON consumers, the supported
  compatibility mode is `ADL_OBSERVABILITY_STDERR=0` together with
  `ADL_OBSERVABILITY_LOG=<compatibility-log-path>`, so the payload stays
  stdout-only while the event stream remains available on a separate explicit
  compatibility mirror.

This compatibility mirror is not the durable machine-readable observability
layer. The governed durable layer still requires JSON or JSONL with a declared
schema.

### Durable Log Channel

- Durable logs must use JSON or JSONL with a declared schema.
- Durable logs must flush one complete record at a time.
- Durable paths should be repo-relative, run-relative, or otherwise stable for
  review surfaces.

### Compatibility Boundary

`adl_event` remains a supported compatibility layer. Later implementation work
may move machine-readable observability to a separate sink or explicit file, but
it must not silently break existing scripts that depend on current `adl_event`
semantics.

## Correlation Model

Observability records should connect across subsystems by carrying the strongest
safe refs available.

Preferred refs include:

- `issue_ref`
- `pr_ref`
- `run_id`
- `request_id`
- `review_request_id`
- `cycle_id`
- `agent_instance_id`
- `artifact_ref`
- `trace_ref`
- `provider_model_id`

Correlation guidance:

- control-plane events should emit issue/PR refs when operating on tracked
  issue work;
- runtime events should emit `run_id`, step ids, and artifact refs;
- provider events should emit provider/model/runtime-surface refs plus request
  ids where available;
- long-lived-agent records should emit `agent_instance_id`, `cycle_id`, and
  continuity refs.

## Redaction And Privacy Rules

Forbidden in shared observability output:

- raw prompts;
- raw model output unless a separate artifact explicitly governs it;
- raw provider payloads;
- credentials, tokens, API keys, secret markers, private keys;
- private tool arguments;
- host-local absolute paths;
- unbounded stderr/stdout excerpts that can leak operator-private context.

Required normalization:

- repo paths -> `<repo>/...` or repo-relative paths;
- home paths -> `<home>/...`;
- temp paths -> `<tmp>` or bounded temp refs;
- failures -> safe `reason_code` plus a redacted excerpt only when already
  governed by an existing provider/result contract.

## Mapping To Current Implementations

| Current surface | Shared contract mapping |
| --- | --- |
| `adl.observability.event.v1` | Terminal event layer with `command`, `stage`, `result`, optional `operation`, refs, and timing. |
| `adl.runtime_action_log.v1` | Already close to the target durable layer: it emits `component`, `stage`, `result`, refs, `reason_code`, and `elapsed_ms`. |
| `provider_communication.v1` result/log events | Partial mapping only today: provider run logs currently center on `event_type` plus optional `status`, while invocation results use `final_status`, attempt statuses, route/model identity, durations, artifact refs, and trace refs. Follow-on work should normalize these toward the shared `stage` / `result` vocabulary without losing current provider semantics. |
| `adl.long_lived_agent_*` schemas | Partial mapping only today: status records use `state` / `last_cycle_status`, and operator events use `event` plus `details`. Follow-on work should map these operational records into the shared stage/result/reason model while preserving current ledger and continuity authority. |

## OTEL Mapping Plan

The OTEL bridge should use standard Rust `tracing` / OpenTelemetry concepts
without making them mandatory for local execution.

| Shared field | OTEL-style mapping |
| --- | --- |
| `command` / `component` | `service.name`, `service.namespace`, or span naming prefix |
| `stage` / `operation` | span name or event name |
| `result` | span status or event attribute |
| `reason_code` | event attribute |
| `issue_ref`, `pr_ref`, `run_id`, `cycle_id`, `request_id` | span attributes |
| `elapsed_ms` / `duration_ms` | span duration or event timing |
| `artifact_ref`, `provider_model_id`, `runtime_surface` | span attributes |

OTEL non-claims for this issue:

- no collector requirement;
- no promise that every internal function becomes a span;
- no change to local exit-code or replay authority;
- no claim that OTEL output becomes the canonical ADL truth surface.

## Example Records

Example terminal event:

```text
adl_event schema=adl.observability.event.v1 command=pr.sh component=control_plane stage=doctor operation=issue.view.state result=started issue=3705
```

Example durable runtime/provider-style record:

```json
{
  "schema_version": "adl.runtime_action_log.v1",
  "sequence": 17,
  "stage": "provider_call",
  "component": "runtime",
  "result": "completed",
  "provider_ref": "openrouter",
  "action_ref": "delegation:reviewer",
  "input_refs": ["request:review-0001"],
  "output_refs": ["artifact:review/logs/review-run-1.jsonl"],
  "reason_code": "provider_completed",
  "elapsed_ms": 1422
}
```

Example long-lived-agent operator event shape:

```json
{
  "schema": "adl.long_lived_agent_operator_event.v1",
  "agent_instance_id": "agent-001",
  "event": "stale_lease_recovered",
  "at": "2026-06-15T12:00:00Z",
  "operator": "local",
  "details": {
    "cycle_id": "cycle-000004",
    "reason_code": "stale_lease_recovered"
  }
}
```

## Migration Guidance

Follow-on issues should use this contract as the migration target:

1. `#3706` completes C-SDLC control-plane coverage and makes JSON output
   channel-safe.
2. `#3707` correlates runtime and provider observability.
3. `#3708` standardizes heartbeat, progress, and timeout behavior.
4. `#3709` implements or finalizes the OTEL boundary behind an optional export.
5. `#3710` defines Observatory consumption against this vocabulary.
6. `#3711` updates docs, skills, AGENTS guidance, and closeout proof.

## Validation Notes

This issue defines the shared contract only. It does not claim:

- full OTEL implementation;
- full call-site coverage;
- JSON/stdout channel fixes for all control-plane commands;
- complete runtime/provider/agent unification today.

It does establish the authoritative vocabulary those follow-on issues must use.
