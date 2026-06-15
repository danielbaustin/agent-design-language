# Observatory Log Consumption Contract (#3710)

Issue: #3710  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: design_contract

## Purpose

The logging mini-sprint now gives ADL a truthful local observability baseline:

- control-plane `adl_event` stage diagnostics;
- runtime action-log projections;
- provider invocation and review-provider logs;
- long-lived-agent ledgers and status artifacts;
- explicit OTEL boundary and redaction rules.

What is still missing is the consumer-side contract for Unity Observatory and
other operator-facing long-running runtime surfaces. This document defines how
Observatory should ingest, classify, retain, redact, and correlate ADL logs so
it consumes the same governed truth instead of inventing a parallel telemetry
model.

## Scope

This issue defines:

- which existing ADL observability artifacts Observatory may consume;
- the minimum normalized fields Observatory should project;
- audience/redaction boundaries for operator, reviewer, `public_report_view`,
  and `observatory_projection` surfaces;
- retention and correlation requirements for long-running runtime use;
- a small example event stream and expected display treatment.

## Non-Goals

- This issue does not build the Unity app.
- This issue does not require a live OTEL collector.
- This issue does not create a new canonical runtime truth source.
- This issue does not authorize raw prompt, secret, host-path, or private-state
  exposure in public or reviewer-facing Observatory views.

## Source Inputs

Primary contract inputs:

- `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md`
- `docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md`
- `docs/milestones/v0.91.5/OPEN_TELEMETRY_INTEGRATION_BOUNDARY_3709.md`
- `docs/milestones/v0.91.5/features/DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md`
- `docs/milestones/v0.91.5/review/logging_observability/LOGGING_OBSERVABILITY_GAP_MAP_3704.md`

Implementation/reference inputs:

- `adl/src/csm_observatory.rs`
- `adl/src/runtime_v2/contracts.rs`
- `adl/src/trace_schema_v1.rs`
- `adl/src/acc/*`
- `adl/src/agent_comms.rs`

Planning inputs:

- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/DESIGN_v0.92.md`

## Consumer Model

Observatory is a consumer and projector of governed ADL artifacts.

It must not:

- become the authority for run truth;
- replace runtime traces, action logs, provider logs, or issue/PR records;
- infer additional state that the source artifacts do not prove.

It may:

- ingest bounded event streams from governed local artifacts;
- classify those records for operators by stage/result/severity;
- project redacted views for operators, reviewers, `public_report_view`, and
  `observatory_projection` audiences;
- correlate related records across issue, run, provider, and agent contexts;
- surface missing evidence or stale telemetry as explicit gaps instead of
  fabricating health.

## Ingestion Sources

Observatory should consume only declared, schema-bearing or contract-governed
local artifacts:

| Source | Role in Observatory |
| --- | --- |
| `adl_event` / compatibility mirror | Operator-facing stage/progress heartbeat for control-plane and CLI operations. |
| `logs/action_log.jsonl` | Runtime action summaries and execution outcomes. |
| Provider invocation / review-provider logs | Provider-side execution status, failure reasons, and model/provider identity. |
| Long-lived-agent status and ledgers | Continuity, cycle state, lease recovery, and operator event history. |
| Runtime/CSM/Observatory packets | Redacted higher-order projections and reviewer/operator reports. |
| Trace / governed execution artifacts | Evidence linkage and downstream drill-through, not raw public display. |

Observatory must not require unsupported sources such as:

- hosted collectors;
- undocumented stdout scraping beyond the governed compatibility layer;
- machine-local absolute paths;
- raw provider payload dumps;
- raw prompt bodies.

## Required Projection Fields

Observatory should normalize consumed events to one bounded projection shape,
even when the source artifact still uses older field names.

Minimum projection fields:

| Field | Meaning | Source guidance |
| --- | --- | --- |
| `schema` | Source schema/version. | Required |
| `source_kind` | `control_plane`, `runtime_action_log`, `provider_log`, `long_lived_agent`, `trace_projection`, `review_packet`. | Required |
| `component` | Emitting subsystem such as `control_plane`, `runtime`, `provider`, `long_lived_agent`, `observability`, `polis`. | Required |
| `stage` | Bounded operational stage such as `doctor`, `provider_call`, `heartbeat`, `closeout`, `cycle`, `projection`. | Required |
| `result` | Bounded normalized status such as `started`, `progress`, `heartbeat`, `completed`, `failed`, `blocked`, `skipped`, `timeout`, `recorded`. | Required |
| `severity` | `info`, `warning`, `error`, or bounded equivalent. | Recommended |
| `reason_code` | Safe compact failure/policy explanation. | Recommended for non-success |
| `timestamp` | Event or artifact timestamp. | Required |
| `elapsed_ms` / `duration_ms` | Timing when available. | Recommended |
| `issue_ref` | Issue identifier when issue-bound execution exists. | Recommended |
| `pr_ref` | PR identifier for review/publication phases. | Optional |
| `run_id` | Runtime run correlation id. | Recommended when present |
| `cycle_id` | Long-lived cycle correlation id. | Recommended when present |
| `agent_instance_id` | Long-lived or delegated agent identity. | Recommended when present |
| `request_id` / `review_request_id` | Provider/review-provider invocation correlation. | Recommended when present |
| `provider_ref` / `provider_model_id` / `runtime_surface` | Provider/model execution context. | Required for provider-facing events |
| `artifact_refs` | Repo-relative or governed artifact references. | Recommended |
| `redaction_view` | `operator`, `reviewer`, `public_report_view`, `observatory_projection`, or a stricter governed visibility alias already modeled by ACC/trace artifacts. | Required |

Legacy/source-specific names such as `event_type`, `final_status`, `state`,
`event`, `artifact_ref`, `correlation_id`, or `observatory_projection` may be
carried through in a source-details block, but Observatory should render the
normalized fields above as the primary operator surface.

## Correlation Requirements

Observatory must support drill-through across the following reference classes
when they exist:

- issue / PR (`issue_ref`, `pr_ref`);
- run / cycle (`run_id`, `cycle_id`);
- agent identity (`agent_instance_id`);
- provider execution (`provider_ref`, `provider_model_id`, `runtime_surface`,
  `request_id`, `review_request_id`);
- artifact and trace linkage (`artifact_ref`, `artifact_refs`, `trace_ref`);
- ACIP / multi-agent conversation linkage (`correlation_id`, conversation or
  causal message refs) where already present in governed artifacts.

Required behavior:

- if a ref is absent in the source artifact, Observatory must display it as
  unavailable rather than synthesizing one;
- if multiple sources disagree on a ref, Observatory should display the mismatch
  as a correlation warning, not silently collapse them;
- if a public/reviewer view cannot safely show a ref, it must present a redacted
  stable placeholder or omit it with rationale.

## Audience And Redaction Policy

Observatory must inherit the repo’s fail-closed visibility and redaction rules.

Audience tiers:

- `operator`
  - may see the richest local operational view available under existing source
    contracts;
  - still must not expose secrets, raw prompts, raw provider payloads, or
    machine-local absolute paths.
- `reviewer`
  - may see bounded evidence refs, redacted projections, and issue/PR/runtime
    correlation needed for review;
  - must not depend on private-state-only data.
- `public_report_view`
  - must use redacted, bounded projections only;
  - must not show host paths, secrets, raw prompts, private payloads, or
    personally identifying operator environment state.
- `observatory_projection`
  - may exist as a separate governed audience when the source artifact already
    distinguishes Observatory-facing projections from broader public-report
    surfaces;
  - must remain at least as strict as the corresponding public/reviewer
    redaction posture.

Specific projection rules:

- host paths -> repo-relative, `<repo>/...`, `<home>/...`, or `<tmp>/...`
- raw prompts -> never shown
- raw provider payloads -> never shown
- private-state markers -> reduced to governed redaction projections
- reviewer/public-report/observatory-projection views -> use the same
  fail-closed logic modeled by ACC visibility matrices and redaction examples

## Retention Policy

Observatory should distinguish between:

- ephemeral terminal progress;
- durable local operational logs;
- tracked review packets;
- public-report / Observatory projections.

Required retention behavior:

- control-plane and runtime event projections may be cached for operator
  sessions, but the canonical source remains the governed artifact, not the
  rendered Observatory cache;
- long-lived-agent ledgers and status views must preserve append-only or
  state-authoritative semantics from their source artifacts;
- reviewer/public-report/observatory-projection packets should reference durable
  tracked artifacts rather than becoming the only copy of evidence;
- stale or pruned source artifacts should surface as an explicit missing-source
  state in Observatory.

## Display Classification

Observatory should classify events into at least these operator buckets:

| Bucket | Typical source/result patterns |
| --- | --- |
| `progress` | `started`, `progress`, `heartbeat` |
| `attention` | `blocked`, `timeout`, correlation mismatch, missing artifact |
| `failure` | `failed`, provider/runtime refusal or unrecoverable error |
| `completed` | `completed`, `recorded`, bounded successful closeout |
| `deferred` | `skipped`, policy-deferred, follow-on-required |

Suggested display rules:

- show newest active progress items first for live operator workflows;
- group by strongest available correlation key (`issue_ref`, then `run_id`,
  then `cycle_id`, then `request_id`);
- allow drill-through from projection to canonical artifact refs;
- if a source artifact is missing or only partially trusted, display that as
  state rather than silently omitting the event.

## Example Event Stream Contract

The tracked example packet for this issue is:

- `docs/milestones/v0.91.5/review/logging_observability/OBSERVATORY_EVENT_STREAM_EXAMPLE_3710.json`

That example proves:

- mixed control-plane, provider, and long-lived/runtime-oriented events can be
  shown in one normalized stream;
- issue/run/request/agent correlation can be expressed without inventing new
  runtime truth;
- public-report / Observatory projection classification can remain redacted and
  bounded.

## v0.92 Dependency Boundary

This issue is a readiness and routing surface for v0.92 Unity/Observatory work.

It proves:

- what Observatory must consume;
- how it should classify and redact that consumption;
- which refs are needed for drill-through and operator use.

It does not prove:

- Unity app implementation;
- OTEL collector export;
- production-grade dashboard persistence;
- all multi-agent / ACIP transport work is complete.

Those remain implementation or later-milestone concerns and must be cited as
such in downstream planning.

## Acceptance Mapping

This issue is complete when:

- Observatory requirements explicitly consume the shared observability contract;
- long-running-process needs are distinguished from one-shot CLI needs;
- the projection model supports local development without external services;
- public-report / Observatory projection surfaces are fail-closed and redacted;
- remaining implementation work is routed to v0.92 or a specific follow-on.

## Non-Claims

- This document does not claim Unity Observatory is already built.
- This document does not claim OTEL exporter support exists locally.
- This document does not claim every existing artifact already carries every
  preferred correlation field.
- This document does not authorize public exposure of private runtime state.
