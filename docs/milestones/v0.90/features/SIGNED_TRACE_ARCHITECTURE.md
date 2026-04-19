# Signed Trace Architecture

## Purpose

Define the architecture for **explicit per-run trace artifacts**, **bounded replay classification**, and later **signed trace verification** in ADL.

This document closes a gap that became visible after the introduction of:
- Structured Task Prompts (STP)
- Structured Implementation Prompts (SIP)
- Structured Output Records (SOR)
- stronger execution-record rigor
- bounded replay language in output cards
- Trace Query Language (TQL) planning

ADL already has:
- design intent artifacts
- execution intent artifacts
- execution summary artifacts

But it does **not yet** have a first-class **execution trace artifact** for each run.

This document defines that missing layer.

---

## Overview

Today, each ADL run is represented by:
- `stp.md` — design intent
- `sip.md` — execution intent
- `sor.md` — execution summary / review record

What is still missing is the **trace substrate**:
- the ordered record of what the run actually did
- the emitted event history of control-plane transitions, validations, and artifact production
- the substrate that later signed replay and TQL should query

The SOR is useful, but it is not the trace.

That distinction matters.

Without a first-class trace artifact, ADL cannot cleanly answer:
- what commands actually ran, in what order
- what control-plane events occurred
- which emitted artifacts were produced at which phase
- which events are replayable vs merely observable
- which parts of a run can be revalidated later
- what exactly would be signed in a signed-trace model

This is now a visible architecture gap.

---

## Key Capabilities

- Structured Task Prompts (STP)
- Structured Implementation Prompts (SIP)
- Structured Output Records (SOR)
- stronger execution-record rigor
- bounded replay language in output cards
- Trace Query Language (TQL) planning
- design intent artifacts
- execution intent artifacts

## How It Works

### Problem Statement

Today, each ADL run is represented by:
- `stp.md` — design intent
- `sip.md` — execution intent
- `sor.md` — execution summary / review record

What is still missing is the **trace substrate**:
- the ordered record of what the run actually did
- the emitted event history of control-plane transitions, validations, and artifact production
- the substrate that later signed replay and TQL should query

The SOR is useful, but it is not the trace.

That distinction matters.

Without a first-class trace artifact, ADL cannot cleanly answer:
- what commands actually ran, in what order
- what control-plane events occurred
- which emitted artifacts were produced at which phase
- which events are replayable vs merely observable
- which parts of a run can be revalidated later
- what exactly would be signed in a signed-trace model

This is now a visible architecture gap.

---

### Core Principle

**The SOR is the review record; the trace is the execution substrate. Replay is a bounded property of trace events, not a claim that the world is reversible.**

This is the governing principle for the entire design.

ADL should not promise impossible total replay.
It should promise:
- explicit trace artifacts
- deterministic event structure
- bounded replay or recheck classification
- later signature and verification over those explicit artifacts

---

### Why This Was Not Fully Caught Earlier

The missing trace layer was partially obscured because adjacent work landed first:
- deterministic workflow structure
- SOR rigor
- proof-surface requirements
- replay language in validation sections
- TQL planning over execution records

These made the system look more complete than it was.

In reality, ADL currently has:
- strong **summary records**
- partial **recheck / replay language**
- no first-class **trace artifact**

This sequencing was not irrational. The substrate had to mature first:
1. deterministic task artifacts
2. bounded control-plane phases
3. output-record rigor
4. proof-surface discipline

Only after those layers became real did the missing trace substrate become obvious and painful.

That is where the system is now.

---

### 1. Make every run explicitly traceable

Each ADL run should emit a durable trace artifact.

A run is normally the execution of one issue/task bundle through the bounded control-plane path.

The trace must be:
- durable
- inspectable
- queryable
- bounded
- suitable for later signing

---

### 2. Separate trace from summary

The SOR remains the execution summary and review record.

The trace becomes:
- the ordered event history
- the primary substrate for replay classification
- the future substrate for signing and query

The SOR should reference the trace, not impersonate it.

---

### 3. Support bounded replay rather than fantasy replay

Real systems are not universally idempotent.

ADL must distinguish between:
- events that are replayable
- events that are recheckable
- events that are observational only

This is more honest and more powerful than pretending everything can be rerun identically.

---

### 4. Stay repo-local and artifact-first

The trace should live inside the task bundle and remain repo-relative.

It must not depend on hidden machine-local state or privileged external storage.

---

### 5. Enable future signing and TQL

The trace format must be designed so that later work can add:
- digest / hashing
- signing
- signature verification
- TQL queries over trace events

without redesigning the entire run model.

---

### Architectural Model

For each run, ADL should produce four primary artifact layers:

1. **STP** — design intent  
2. **SIP** — execution intent  
3. **TRACE** — ordered execution substrate  
4. **SOR** — review / summary record  

This yields the correct separation of concerns:

- STP = what should be done
- SIP = how this run should do it
- TRACE = what actually happened
- SOR = what reviewers should conclude

---

### Proposed Task-Bundle Layout

Minimum target layout:

- `stp.md`
- `sip.md`
- `sor.md`
- `trace.jsonl`
- `replay_manifest.json`

Later expansion:

- `trace_manifest.json`
- `trace.signature.json`
- `trace_hashes.json`

### Why `trace.jsonl`

`JSONL` is the recommended first format because it is:
- append-friendly
- event-oriented
- easy to stream
- easy to hash later
- easy to inspect with shell tools
- suitable for TQL later

A single-record JSON document is less suitable because runs are naturally event streams.

---

### Run Identity

Each trace must be tied to a concrete run identity.

Minimum run identity fields:
- `task_id`
- `run_id`
- `issue_number` when available
- `bundle_slug`
- `branch`
- `started_at`
- `finished_at` when available

If GitHub issue numbers are absent, the task bundle identity remains canonical.

This aligns with the broader rule:
- GitHub issues are projections
- task/bundle identity is canonical

---

### Trace Event Model

Each line in `trace.jsonl` should be a structured event.

Minimum event fields:
- `trace_version`
- `task_id`
- `run_id`
- `sequence`
- `timestamp`
- `phase`
- `event_type`
- `actor`
- `status`
- `details`
- `artifact_refs`
- `replay_class`

Recommended optional fields:
- `tool`
- `command`
- `command_digest`
- `input_refs`
- `output_refs`
- `notes`
- `parent_event_sequence`

### Event Types

The first version should keep event types small and practical.

Recommended initial event types:
- `run_started`
- `run_finished`
- `phase_started`
- `phase_finished`
- `command_executed`
- `validator_executed`
- `artifact_emitted`
- `artifact_updated`
- `demo_executed`
- `review_checkpoint`
- `deviation_recorded`
- `failure_recorded`

This is enough to make traces useful without overbuilding a complete process ontology.

---

### Phases

Recommended phase vocabulary:
- `init`
- `authoring`
- `execution`
- `validation`
- `demo`
- `review`
- `finish`

Not every run must emit every phase. The important property is that phase names are normalized and machine-readable.

---

### Replay Classification

Every trace event should carry a replay classification.

This is central to the architecture.

### 1. Replayable

The event can be rerun deterministically.

Examples:
- local validator execution
- deterministic graph ranking fixture
- hypothesis generator test against fixed input
- policy comparison artifact generation from fixed fixtures

### 2. Recheckable

The event itself may not be rerun as an identical historical act, but its effect can be revalidated.

Examples:
- `git diff` over bounded files
- schema validation over emitted artifact
- path-hygiene check
- coverage gate verification after code exists

### 3. Observational-only

The event is a record of something that occurred but is not meaningfully replayable or recheckable as the same historical act.

Examples:
- human intervention note
- temporary sandbox refusal
- transient timing data
- non-deterministic environment observation

This classification allows ADL to be both honest and useful.

---

### Replay Manifest

Each run should emit a `replay_manifest.json` alongside the trace.

The replay manifest is not the trace itself. It is the summary of replay semantics.

Minimum fields:
- `trace_path`
- `task_id`
- `run_id`
- `replay_scope`
- `events_by_replay_class`
- `replay_commands`
- `recheck_commands`
- `notes`

### SOR Integration

The SOR should stop implying that it is the trace.

It should explicitly reference trace surfaces.

Recommended new SOR metadata fields later:
- `trace_present`
- `trace_path`
- `replay_manifest_path`
- `trace_signing_status`
- `replay_scope`

This makes the layers clear:
- SOR = summary / review
- TRACE = execution substrate
- replay manifest = replay semantics

---

### Control-Plane Integration

The control plane should emit trace events automatically.

Minimum integration points:
- `pr start`
- `pr run`
- `pr finish`

Later integration points:
- `pr init`
- `pr create`

### Minimum requirement

At minimum, the control plane should record:
- run start
- major phase transitions
- validation executions
- artifact emissions
- run finish
- deviations or failures

This must be automatic. The trace cannot depend on manual narrative reconstruction after the fact.

---

### Determinism and Ordering

The trace format itself must be deterministic enough to be useful.

Requirements:
- events must carry monotonically increasing `sequence`
- timestamps should be recorded, but ordering must not rely on timestamps alone
- event schema must be normalized
- emitted artifact references should be repo-relative
- trace generation should not depend on absolute host paths

This is especially important because ADL already has strong path-hygiene requirements elsewhere.

---

### Signing Strategy

Signed traces should be introduced in phases.

### Phase 1 — explicit trace artifact

- emit `trace.jsonl`
- emit `replay_manifest.json`
- no signatures yet

### Phase 2 — digest / hashing

- compute per-trace or per-event digests
- add `trace_hashes.json` or digest fields

### Phase 3 — signed trace bundle

- sign the trace bundle or manifest
- emit `trace.signature.json`

### Phase 4 — verification tooling

- add verification commands
- integrate signature checks into review / policy surfaces

This sequencing avoids premature complexity while keeping the architecture honest.

---

### What Should Be Signed Later

The most practical later target is to sign the **trace manifest + digests**, not arbitrary raw tool output.

Likely signed bundle components:
- trace metadata
- digest of `trace.jsonl`
- digest of replay manifest
- key artifact digests
- signer identity / method
- verification metadata

This is more stable than trying to sign every raw event as an isolated act in the first version.

---

### Relationship to Git

Git remains the canonical mechanism for code state transition and rollback.

The trace does not replace Git.

Instead:
- Git captures repository state
- the trace captures execution history
- the replay manifest captures rerun/recheck semantics
- the SOR captures reviewable conclusions

This is an important conceptual distinction.

Git can often restore file state. It does not by itself explain what happened during the run.

---

### Relationship to TQL

TQL becomes much more powerful once trace artifacts exist.

Without traces, TQL can only query:
- SOR summaries
- task-bundle metadata
- emitted static artifacts

With traces, TQL can later query things like:
- runs where validation failed before artifact emission
- runs with observational-only deviations
- runs where replay scope is only partial
- runs that emitted a specific artifact class
- runs that followed the same stage pattern but diverged in output

This is why signed trace architecture and TQL planning belong to the same larger direction, even if they should be implemented in separate bounded steps.

---

### Security and Privacy Requirements

Trace artifacts must obey the same hygiene rules as other ADL artifacts.

They must not contain:
- secrets
- raw tokens
- hidden credentials
- absolute host paths
- uncontrolled prompt dumps unless explicitly approved

The trace should reference artifacts and commands in bounded, reviewable ways rather than capturing everything indiscriminately.

This is a trace substrate, not a surveillance log.

---

### Minimum v0 Implementation Slice

The minimum credible first implementation should do the following:

1. Emit a repo-local `trace.jsonl` per run  
2. Emit a `replay_manifest.json` per run  
3. Record normalized control-plane and validation events  
4. Classify events as `replayable`, `recheckable`, or `observational_only`  
5. Reference the trace and replay manifest from the SOR  
6. Keep all paths repo-relative  
7. Avoid signing in the first slice unless it lands cheaply and honestly  

This is sufficient to make the trace promise real without destabilizing the rest of the system.

---

### Roadmap Position

This work is correctly placed **after** the current v0.85 milestone band.

It should have been visible in the broader road-to-v0.95 thinking, but it is not realistic to force it into v0.85 now.

That is acceptable.

The correct next-roadmap interpretation is:
- v0.85 builds stronger artifacts, review surfaces, and bounded runtime loops
- v0.86+ introduces the explicit trace substrate
- later milestones can add:
  - signing
  - trace verification
  - TQL over trace events

So this is not a v0.85 omission in execution. It is a v0.86+ architecture layer becoming visible at the right time.

---

### Recommended New Issues

This document naturally decomposes into a small bounded issue train.

### Issue A — Explicit per-run trace artifact

- emit `trace.jsonl`
- normalized event schema
- repo-relative pathing
- control-plane integration at `pr run` / `pr finish`

### Issue B — Replay manifest

- emit `replay_manifest.json`
- classify events by replayability
- add replay scope to SOR metadata

### Issue C — Trace/SOR integration

- SOR references trace presence and replay manifest
- review surfaces become trace-aware

### Issue D — Signed trace bundle

- add digests
- add signature manifest
- add verification command

### Issue E — TQL over traces

- query trace events and replay classes
- integrate with broader TQL work

This is the right bounded rollout path.

---

### Summary

ADL currently has strong task artifacts and increasingly strong execution records, but it still lacks one critical layer: the first-class execution trace.

This document defines that missing layer.

The architecture is based on a simple distinction:
- STP = design intent
- SIP = execution intent
- TRACE = execution substrate
- SOR = review summary

Replay is treated as a bounded property of trace events, not as an unrealistic promise that the world is reversible.

That makes the model:
- more honest
- more auditable
- more extensible
- and suitable for later signing and TQL

The result will be a significantly stronger ADL platform:
- more trustworthy
- more queryable
- more replay-aware
- and much closer to the signed, reviewable, shared-reality substrate the project has been aiming toward.

### Non-Goals

This architecture explicitly does **not** require in its first slice:
- total replay of arbitrary real-world actions
- full distributed tracing infrastructure
- signing of every event immediately
- natural-language querying over arbitrary trace prose
- replacement of Git as the state substrate
- a full database layer before repo-local artifacts are proven useful

The goal is a strong first trace substrate, not a complete observability platform.

---

## Example / Demo

```json
{
  "trace_version": "adl.trace.v1",
  "task_id": "issue-0876",
  "run_id": "issue-0876",
  "sequence": 12,
  "timestamp": "2026-03-20T18:12:11Z",
  "phase": "validation",
  "event_type": "command_executed",
  "actor": "execution_agent",
  "status": "passed",
  "tool": "shell",
  "command": "cargo test graph_affect::tests::ranking_changes",
  "artifact_refs": [
    "artifact://adl/tasks/issue-0876/sor.md"
  ],
  "replay_class": "replayable",
  "details": {
    "purpose": "Prove affect changes graph ranking via before/after fixture"
  }
}
```

The example above is illustrative; the architecture does not require this exact event set, but it does require event structure of this kind.

---

A realistic early lifecycle for one ADL run:

1. `pr run` begins  
2. trace emits `run_started`  
3. validation command emits `validator_executed`  
4. runtime logic emits `artifact_emitted`  
5. demo step emits `demo_executed`  
6. `pr finish` emits `run_finished`  
7. `sor.md` records the summary  
8. `replay_manifest.json` classifies replayability  

This is enough to turn the run into a real auditable substrate.

---

## Why It Matters

This feature matters because it contributes to ADL's bounded, reviewable, and explicit system design. See Purpose and How It Works for the preserved rationale from the original document.

## Current Status

- Milestone: v0.93
- Status: Draft
- Notes: No additional status notes recorded.

## Related Documents

- stp.md
- sip.md
- sor.md

## Future Work

- `full`
- `partial`
- `recheck_only`
- `none`

For early ADL runs, `partial` or `recheck_only` will be the normal truthful value.

---


## Notes

- This document was reformatted to the shared feature-doc structure as part of #1009 without intentionally removing original content.
