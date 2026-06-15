# Logging And Observability Gap Map

Issue: #3704  
Parent: #3703  
Captured: 2026-06-14  
Status: inventory_complete

## Summary

ADL now has several real logging surfaces, but they are still fragmented. The
repo has shell and Rust `adl_event` terminal events, runtime
`logs/action_log.jsonl`, provider run JSONL logs, long-lived-agent ledgers,
multi-agent duration packets, and Octocrab operation logs. Those are useful
baselines, not a complete observability system.

The remaining gap is coherence and coverage: one shared event/span contract,
complete C-SDLC stage coverage, runtime/provider integration, heartbeat and
timeout diagnostics for long-running work, an OpenTelemetry boundary, and
Observatory consumption rules.

## Existing Baselines

| Surface | Evidence | Current status | Notes |
| --- | --- | --- | --- |
| Shell control-plane events | `adl/tools/observability.sh` | implemented_baseline | Emits `adl_event schema=adl.observability.event.v1` lines, sanitizes secrets/paths, supports optional `ADL_OBSERVABILITY_LOG`. |
| Rust CLI events | `adl/src/cli/observability.rs` | implemented_baseline | Emits sanitized `adl_event` lines. Used by dispatcher and selected subcommands. |
| C-SDLC control-plane contract | `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md` and #3609 | implemented_baseline | Defines the `adl_event` vocabulary and an OTEL-ready mapping, but does not complete OTEL export or all command coverage. |
| Closeout silent-hang fix | #3698 | implemented_followup | Closed a reproduced post-merge closeout silence case. This is evidence that baseline logging needed follow-up hardening. |
| Octocrab GitHub operation logs | `adl/src/cli/pr_cmd/github.rs` and #3697 | implemented_baseline | Covered GitHub operations emit `github_octocrab` started/completed/failed events. Useful and recently dogfooded. |
| Runtime action log | `docs/milestones/v0.91.5/RUNTIME_ACTION_LOG_CONTRACT_3556.md`, `adl/src/instrumentation/action_log.rs`, #3556 | implemented_first_slice | Generates `logs/action_log.jsonl` from trace events and keeps it as a projection, not a second truth source. |
| Provider run log | `adl/src/provider_communication.rs`, `adl/src/provider_adapter_cli.rs`, #3480 | implemented_baseline | Provider JSONL logs exist with redacted event fields and duration/failure metadata. |
| Long-lived agent ledgers | `adl/src/long_lived_agent/`, `adl/src/long_lived_agent/storage.rs` | implemented_domain_specific | Has heartbeat spec, cycle ledger, provider binding history, and operator events, but not unified with C-SDLC/runtime observability. |
| Multi-agent timing packets | `docs/milestones/v0.91.5/review/multi_agent_*` | implemented_evidence | Sprint 2 packets record wall durations and lane timing; they are evidence artifacts, not a general logging framework. |

## Gap Map

| Gap | Risk | Owner issue | Target surface | Proof needed |
| --- | --- | --- | --- | --- |
| No single shared observability contract across shell, Rust CLI, runtime, providers, long-lived agents, and Observatory. | Different subsystems keep adding compatible-looking but divergent logs. | #3705 | Shared docs/schema/contract | Contract maps existing `adl_event`, runtime action logs, provider logs, heartbeat events, and OTEL attributes. |
| C-SDLC control-plane logging remains uneven beyond the fixed high-pain paths. | `doctor`, `finish`, `closeout`, validators, and watchers can still be hard to diagnose when a new stage lacks progress output. | #3706 | `adl/src/cli/pr_cmd/`, `adl/tools/pr.sh`, validators | Focused tests or transcripts show stage/progress/failure logs for success, fail-closed GitHub, validation failure, and waiting states. |
| Runtime/provider logging is split between trace-derived action logs and provider-specific JSONL. | Multi-agent/provider failures can be misread as model-quality failures if runtime/provider logs are not correlated. | #3707 | `adl/src/execute/`, `adl/src/provider*`, `adl/src/instrumentation/`, `adl/src/trace.rs` | Local success/failure fixtures prove provider/model/result/retry/artifact events are correlated and redacted. |
| Heartbeat, timeout, and progress policy is not unified. | Operators still ask “is it hung?” for long commands or long-lived processes. | #3708 | `finish`, `closeout`, validation subprocesses, provider calls, long-lived agents | Slow/hanging fixture proves bounded heartbeats/progress with stable timeout reason codes. |
| OpenTelemetry is only planned/OTEL-ready, not implemented. | Claims of standard observability can overstate reality; future exporters may be bolted on inconsistently. | #3709 | Cargo/dependency plan and exporter boundary | No-op/stdout subscriber proof or design review; CI must not require a collector. |
| Observatory consumption is not defined against the current event model. | Unity/Observatory could invent a separate telemetry truth instead of consuming ADL runtime/C-SDLC events. | #3710 | v0.92 Observatory docs and event examples | Example event stream and requirements for ingestion, display, retention, redaction, and correlation. |
| Docs, skills, AGENTS, and validation do not yet enforce the completed logging model. | Implementation can land but future agents keep using old/silent paths. | #3711 | `AGENTS.md`, skills, docs, validation checklist | Skills and docs teach required logs; closeout packet records complete/deferred truth. |

## Not Missing After This Inventory

- The repo does not need a brand-new logging concept from nothing. It already
  has `adl_event`, runtime action logs, provider run logs, and long-lived-agent
  ledgers.
- Sprint 2 multi-agent child work appears closed; the logging sprint should be
  treated as hardening before further reliance, not as a prerequisite to
  starting Sprint 2.
- `#3697` is closed and provides operational Octocrab logging evidence for
  GitHub transport operations.
- `#3698` is closed and provides a direct fix for one reproduced closeout
  silent-hang class.
- `#3609` is the implemented C-SDLC control-plane logging baseline and shared
  `adl_event` vocabulary, not a full OpenTelemetry implementation.
- `#3556` is the implemented first slice of runtime action-log projection,
  with remaining branch/span/dashboard coverage explicitly deferred.
- `#3480` is the implemented provider run-log baseline, not a unified
  runtime/provider/C-SDLC correlation model.

## Still Missing Or Partial

- OpenTelemetry export is not implemented.
- Runtime action logs are a first slice; direct emission from every validation
  branch, low-level artifact write, long-running span, and external dashboard
  remains deferred in `RUNTIME_ACTION_LOG_CONTRACT_3556.md`.
- Control-plane logging has a contract and important implementation, but not a
  complete proof that every long-running command path emits progress.
- Provider logs exist, but provider/runtime/C-SDLC correlation is not unified.
- Long-lived-agent heartbeat and ledgers exist, but they are domain-specific and
  not mapped into a shared observability contract.
- Observatory/Unity consumption requirements are not yet tied to the ADL event
  model.
- Skills and repo guidance do not yet make logging proof a standard part of
  future issue work.

## Recommended Execution Order

1. `#3705` defines the shared contract and OTEL mapping.
2. `#3706` completes C-SDLC control-plane logging against that contract.
3. `#3707` correlates runtime/provider logging.
4. `#3708` adds heartbeats and timeouts for long-running paths.
5. `#3709` decides and/or implements the OpenTelemetry integration boundary.
6. `#3710` defines Observatory consumption.
7. `#3711` updates docs, skills, AGENTS, validation, and sprint closeout truth.

## Validation Notes

This issue is an inventory and routing issue. It intentionally does not modify
runtime behavior. Focused validation should therefore check:

- the audit file exists;
- markdown has no obvious formatting/hygiene errors;
- the child issue routing covers every gap in this map;
- no raw secrets or absolute host-local paths were introduced.
