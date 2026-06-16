# Logging Validation Checklist (#3711)

Issue: #3711  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: docs_validation_contract

## Purpose

This checklist defines the minimum proof bar for future logging or
observability-affecting changes in ADL. It is not a blanket requirement to run
every logging test on every issue. It is a scoped checklist for deciding what
must be proven when a change touches the logging contract.

## When To Use This Checklist

Use this checklist when a change touches any of the following:

- workflow or control-plane command logging
- machine-readable command output that coexists with observability events
- runtime action-log behavior
- provider or review-provider durable logs
- heartbeat, timeout, or long-running progress diagnostics
- OTEL boundary or exporter policy
- Observatory or downstream log-consumer contracts
- docs or skill guidance that teaches operators how those surfaces work

## Core Questions

Answer these questions before publication:

1. What logging surface changed?
2. What is the claimed operator-visible behavior?
3. What machine-readable or durable artifact is authoritative?
4. What larger logging claim is explicitly *not* being made?

If any answer is unclear, the issue is not review-ready.

## Required Validation Categories

### 1. Channel Policy

If the change touches JSON or other machine-readable command payloads:

- prove whether stdout remains machine-readable
- prove whether observability remains on stderr, a compatibility log, or both
- record any explicit compatibility mode such as:
  - `ADL_OBSERVABILITY_STDERR=0`
  - `ADL_OBSERVABILITY_LOG=<path>`

Minimum proof:

- one command transcript or focused test showing the channel behavior that is
  being claimed

### 2. Redaction And Path Hygiene

If the change touches emitted logs or durable artifacts:

- verify no raw secrets, raw prompts, private tool arguments, or unjustified
  host-local absolute paths are newly exposed
- record what surface was checked

Minimum proof:

- focused redaction/path-hygiene test, validator, or bounded transcript review

### 3. Durable Artifact Truth

If the change touches runtime/provider/Observatory/durable log artifacts:

- name the authoritative artifact
- prove the artifact is written or interpreted as claimed
- distinguish projection artifacts from canonical truth

Minimum proof:

- focused artifact-level test, validator, or documented replay/readback command

### 4. Progress / Heartbeat / Timeout Truth

If the change touches long-running execution:

- prove what visible progress signal exists
- prove timeout or heartbeat reason codes if they are part of the claim
- distinguish “pending but healthy” from “blocked or failed”

Minimum proof:

- one focused slow-path or fixture proving the claimed progress/timeout behavior

### 5. Non-Claims

Every logging-affecting issue must say what it does **not** prove. Examples:

- full OpenTelemetry implementation
- repo-wide runtime/provider correlation
- complete progress coverage for every command
- machine-readable cleanliness for every existing command path

If the issue does not have non-claims, the review packet is incomplete.

## Validation Selection Matrix

| Change class | Expected local proof |
| --- | --- |
| Docs/skill guidance only | file existence, Markdown/path hygiene, claim-evidence alignment |
| Control-plane logging | focused shell/Rust command proof, stderr/stdout policy proof |
| Runtime/provider durable logs | focused Rust/unit/integration proof on the named artifact |
| Heartbeat/timeout diagnostics | focused slow-path or fixture proving bounded progress behavior |
| OTEL boundary docs | contract/proof packet validation, no overclaim review |
| Observatory consumption docs | example stream validation, contract/path truth, no runtime overclaim |

## Publication Rule

Do not publish a logging-affecting issue with only a generic “tests passed”
claim. The SOR and review packet must say:

- what exact logging surface changed
- what exact proof ran
- what larger logging work remains deferred

## Follow-On Routing

If the issue uncovers a tooling defect outside its scope, route a follow-on
instead of widening the current slice. Typical examples:

- queue gating on unrelated PRs
- machine-readable output pollution
- overly generic cards emitted during issue bootstrap
- sprint-state closeout drift

## Related Proof Surfaces

- [Shared observability contract](SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md)
- [Control-plane observability contract](CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md)
- [Runtime action-log contract](RUNTIME_ACTION_LOG_CONTRACT_3556.md)
- [Logging gap map](review/logging_observability/LOGGING_OBSERVABILITY_GAP_MAP_3704.md)
