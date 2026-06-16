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

## Prompt-Template Tooling Boundary

When creating a new SRP or fully re-rendering one, prefer the active
prompt-template values renderer and structure/schema validators before using
Markdown as lifecycle state:

```sh
adl-csdlc tooling prompt-template validate-values --kind srp --values <path>
adl-csdlc tooling prompt-template edit-values --kind srp --values <path> --set <field=value> --out <path>
adl-csdlc tooling prompt-template render --kind srp --values <path> --out <path>
adl-csdlc tooling prompt-template validate-structure --kind srp --input <path>
```

If `adl-csdlc` is not already on `PATH`, run the same commands from a fresh
checkout through `cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- ...`.

Use this skill for SRP truth repairs: review scope, review prompts, findings,
dispositions, reviewer notes, residual risks, and recommended outcome. Do not
use it to bypass locked template prose or schema validation. When a supported
declared values field is the only change needed, prefer `edit-values` before
rendering instead of patching rendered Markdown.

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
- set `card_status` to `draft`, `ready`, `approved`, `completed`, `blocked`,
  or `superseded` according to observed review truth
- update headings and wording from legacy Structured Review Policy scaffolding
  to Structured Review Prompt semantics
- prepare a complete review prompt before review while explicitly preserving
  that findings, dispositions, and recommended outcome are absent until review
  actually runs
- add or tighten sections for review scope, review instructions, findings,
  dispositions, reviewer notes, residual risks, and recommended outcome
- record review findings only when explicit review evidence is supplied
- mark no-findings review results only when an actual review was performed
- set `card_status: "completed"` only after review findings, dispositions,
  reviewer notes, residual risks, or an explicit final policy exception are
  recorded
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
