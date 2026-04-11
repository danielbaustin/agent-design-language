

# TRACE REVIEW PIPELINE v1

## Metadata
- Owner: `adl`
- Status: `promoted`
- Target milestone: `v0.87`
- Parent: `docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md`
- Depends on:
  - `TRACE_SCHEMA_V1.md`
  - `TRACE_RUNTIME_EMISSION.md`
  - `TRACE_ARTIFACT_MODEL.md`
  - `TRACE_VALIDATION_AND_REVIEW.md`

## Purpose

Define the **review pipeline** for ADL traces.

This document specifies:
- how trace data flows through validation and review stages
- how automated and human review are composed
- how findings are produced and surfaced
- how the pipeline integrates with demos, CI, and future control-plane features

This is the **feature-owner doc** for trace review pipeline in `v0.87`.

## Core Principle

> Review is not a single step; it is a pipeline that transforms execution into understanding.

The pipeline must:
- validate correctness
- expose decisions
- surface failures
- enable reconstruction

Boundary note:
- this doc owns orchestration, staging, and report generation
- `TRACE_VALIDATION_AND_REVIEW.md` owns what counts as valid trace and what
  reviewers must be able to see
- `TRACE_SCHEMA_V1.md` owns event/field requirements

## Pipeline Overview

The trace review pipeline consists of ordered stages:

1. **Ingestion**
2. **Validation**
3. **Enrichment (optional v1-light)**
4. **Analysis**
5. **Review Output Generation**

Each stage MUST be deterministic given the same trace + artifacts.

## Stage 1: Ingestion

Inputs:
- trace event stream
- artifact directory

Responsibilities:
- load trace events
- index artifacts
- build initial in-memory representation

### Requirements

- ingestion MUST fail if required artifacts are missing
- ingestion MUST preserve event ordering

## Stage 2: Validation

Per `TRACE_VALIDATION_AND_REVIEW.md`:

- schema validation
- structural validation
- semantic validation

### Output

- validation report
- list of errors/warnings

### Behavior

- hard failures MAY halt pipeline (configurable in future)
- soft failures MUST be recorded

## Stage 3: Enrichment (v1-light)

Optional but recommended minimal enrichment:

- derive step summaries
- annotate spans with duration
- attach simple derived metadata

### Constraints

- enrichment MUST NOT alter original trace data
- enrichment MUST be deterministic

## Stage 4: Analysis

Analysis extracts meaning from validated trace.

### Responsibilities

- identify decision points
- map execution flow
- correlate inputs → outputs
- detect anomalies (missing events, unexpected paths)

### Outputs

- structured findings
- anomaly list
- execution summary

## Stage 5: Review Output Generation

Generate human- and machine-consumable outputs.

### Outputs

- review report (markdown or structured)
- summary of execution
- list of issues
- trace-to-artifact navigation hints

This stage feeds:
- output cards
- CI reports
- `.adl/reports/` artifacts

## Pipeline Interfaces

### Input Interface

- trace file(s)
- artifact root path

### Output Interface

- structured report object
- human-readable report file

## Determinism Requirements

- same input MUST produce identical review outputs (excluding timestamps)
- no nondeterministic ordering
- no hidden state

## Integration Points

### Demo Matrix (v0.87)

- each demo MUST produce trace
- pipeline MUST validate and review demo traces
- output MUST be inspectable in demo artifacts

### Output Cards

- output cards SHOULD reference review findings
- review summaries SHOULD be embedded or linked

### CI / Automation

- pipeline MAY be run in CI
- failures MAY fail builds (future tightening)

### `.adl/reports/`

- review outputs SHOULD be persisted here

## Decision Traceability

Pipeline MUST preserve and expose:

- DECISION events
- APPROVAL / REJECTION / REVISION
- CONTRACT_VALIDATION outcomes

Reviewer must see:
- what was decided
- why (if available)
- what changed as a result

## Error Handling

- pipeline MUST handle ERROR events explicitly
- errors MUST appear in final report
- upstream context MUST be included

## Acceptance Criteria

- pipeline runs end-to-end on demo traces
- validation issues are correctly surfaced
- decisions are clearly visible in output
- artifact references are navigable
- reviewer can reconstruct execution without inference

## Non-Goals (v1)

- full graphical UI
- distributed review pipelines
- advanced statistical analysis

## Open Questions

- standard report schema vs markdown-only
- CLI tooling vs embedded library first
- thresholds for failing CI

## Implementation Notes

Initial implementation may be:

- CLI tool (`adl trace review`)
- library used by demos and tests

Core components:
- ingestion loader
- validator
- analyzer
- report generator

## Next Steps

Future extensions:
- Freedom Gate integration (policy review stage)
- ObsMem ingestion hooks
- replay-assisted review

The review pipeline is the bridge between execution and understanding in ADL.
