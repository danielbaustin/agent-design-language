---
name: srp-editor
description: Normalize and correct an SRP review card so it preserves Structured Review Prompt semantics, review-result truth, finding dispositions, and residual-risk boundaries. Use when an `srp.md` is missing, still uses legacy Structured Review Policy scaffolding, lacks review results, or overclaims review completion.
---

# SRP Editor

This skill owns bounded editing of `srp.md` review cards.

Its job is to:
- normalize `SRP` structure as a Structured Review Prompt
- preserve review instructions and policy while also recording review results
- keep findings, dispositions, reviewer scope, and residual risk truthful
- retire legacy Structured Review Policy wording when final SRP truth is needed
- stop before implementation, finish publication, merge, or output-card authoring

This is a helper skill, not a lifecycle orchestrator.

## Required Inputs

At minimum, gather:
- repository root
- `srp_path`
- one explicit editing mode

Useful additional inputs:
- issue number
- source prompt path
- linked `stp.md`, `sip.md`, `spp.md`, or `sor.md`
- review findings or reviewer notes
- review outcome and finding dispositions

## Quick Start

1. Read the SRP and the linked source prompt if available.
2. Determine whether the SRP is still a legacy policy scaffold or a final
   Structured Review Prompt record.
3. Preserve the issue-local review policy and reviewer instructions.
4. Normalize review-result truth, findings, dispositions, and residual risk to
   the evidence supplied by the caller.
5. Remove unsupported completion claims and stale Structured Review Policy-only
   language when final SRP truth is required.
6. Emit a structured edit result and stop.

## Allowed Edits

This skill may:
- normalize `artifact_type` to `structured_review_prompt`
- update headings and wording from legacy Structured Review Policy scaffolding
  to Structured Review Prompt semantics
- add or tighten sections for review scope, review instructions, findings,
  dispositions, reviewer notes, residual risks, and recommended outcome
- record review findings only when explicit review evidence is supplied
- mark no-findings review results only when an actual review was performed
- preserve unresolved findings and route them back to implementation or
  follow-on issue creation
- remove placeholders and stale review-completion claims

This skill must not:
- invent review results, reviewer coverage, or validation evidence
- resolve findings that have not been fixed or explicitly accepted
- rewrite `STP`, `SIP`, `SPP`, or `SOR` instead of handing off
- publish or merge a PR
- widen issue scope

## Handoff

Typical callers are:
- `workflow-conductor` when doctor or card evidence reports an incomplete SRP
- `sprint-conductor` during sprint-wide structured prompt preflight
- `pr-run` or human review after bounded subagent review results are available
- `pr-finish` when final review truth blocks publication

## Output

Return a concise structured result including:
- target `SRP` path
- review prompt semantics normalized
- review results recorded or explicitly absent
- finding dispositions updated
- unresolved blockers
- recommended next handoff
