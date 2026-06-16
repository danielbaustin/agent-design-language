# Public Prompt Records Export

## Metadata

- Feature Name: Public Prompt Records Export
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: artifact, policy
- Proof Modes: review, schema, tests

## Purpose

Define how editable local C-SDLC prompt records become public-safe exported
records without losing authoring truth or exposing private state.

## Scope

In scope:

- local `.adl/` authoring boundary;
- public export shape;
- redaction and secret/path checks;
- validation and indexing;
- security review and evidence links.

Out of scope:

- moving authoring out of `.adl/`;
- publishing unreviewed local records;
- broad prompt-template redesign.

## Required Decisions

- Which records are public by default and which require explicit export?
- Which fields are redacted, transformed, or omitted?
- Which index owns public navigation?
- Which validation blocks export?

## Dependencies

- Public C-SDLC prompt records transition plan.
- Security bridge and CAV feature doc.
- Prompt-template renderer and card validation tooling.

## Validation And Review

- Run export dry-run checks before public publication.
- Scan for secrets, host-local paths, provider logs, and private `.adl` state.
- Validate schema/index consistency.
- Review redaction exceptions explicitly.

## v0.92 Consumption

`v0.92` may consume public prompt records only after export, redaction,
validation, indexing, and security review are complete or explicitly routed.

## Non-Goals

- No deletion of local editable records.
- No assumption that local truth is public-safe.
- No public export without validation.
