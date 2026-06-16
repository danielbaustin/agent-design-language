# Docs-Only Validation Bundle And Lifecycle Policy (#3736)

Issue: #3736
Status: implementation policy

## Purpose

Define the bounded docs-only validation lane inside the normal ADL C-SDLC
workflow.

This policy does not create a parallel process. Docs-only issues still use:

- `workflow-conductor`
- `pr-init`
- `pr-ready`
- `pr-run`
- `pr-finish`
- `pr-closeout`
- the normal `SIP -> STP -> SPP -> SRP -> SOR` card lifecycle

The change is only in the proving bundle and the wording standard for truthful
publication.

## Eligible Surface

Use the docs-only bundle when every tracked change stays inside documentation,
planning, policy, review packets, prompt-card artifacts, or other non-runtime
surfaces and the issue does not change:

- Rust behavior
- shell wrapper behavior
- CI runner behavior
- GitHub transport behavior
- validation selector logic
- coverage-impact logic
- runtime, provider, demo, or observability implementation code

Typical eligible paths include:

- `docs/`
- root Markdown policy docs such as `AGENTS.md`
- `.adl/` card, review, and sprint truth surfaces

Prompt-card edits remain docs-only only when the issue changes card truth or
rendered prompt content without changing the renderer, schemas, or validator
implementation.

Read-only helper scripts or queries may still participate in docs-only proof,
but changing shell/tooling execution behavior is not docs-only work.

## Local Validation Bundle

The docs-only bundle should be the smallest proving set that matches the touched
surface. Use the applicable subset of:

1. `git diff --check`
2. focused Markdown/path/link checks for touched docs
3. YAML parsing or schema checks for touched YAML artifacts
4. live GitHub issue, label, title, or PR-state verification when the touched
   docs make claims about current GitHub truth
5. prompt-template values, render, structure, and schema checks when prompt
   cards or template artifacts changed
6. redaction/path-hygiene checks when publication-facing docs or observability
   packets changed
7. focused contract or read-only helper checks only when they verify the
   touched docs or truth surface without changing workflow behavior

The docs-only bundle does not imply:

- `cargo fmt`
- `cargo clippy`
- broad `cargo nextest`
- full `cargo llvm-cov`
- demo smoke
- release-only or nightly proof

Those lanes remain CI or release-gate concerns unless the issue actually edits a
surface that owns them.

## Widening Rules

Docs-only work must widen out of the docs-only bundle when any tracked change
touches:

- `adl/src/`
- behavioral shell tooling under `adl/tools/` that changes execution semantics
- CI/workflow runners that determine validation or publication outcomes
- prompt-template Rust binaries, validators, or schemas
- logging/observability implementation code
- provider/runtime/demo behavior

When widening is required:

- choose the smallest truthful focused lane first
- use the owner lane or targeted proof surface when available
- fall back to broad Rust validation only when the changed surface is broad,
  ambiguous, or unclassified

## SOR Wording Expectations

Docs-only `SOR` validation text must distinguish local proof from deferred CI
and from proof that was intentionally not run.

Required truth pattern:

```text
Local preflight: PASS (<focused docs/path/card/redaction commands>).
CI integration: deferred to GitHub `adl-ci` / `adl-coverage`.
Release-gate proof: not required for this docs-only change.
Validation not run locally: broad Rust validation was not run because no tracked
runtime/tooling behavior changed.
```

Do not write:

- "full validation passed" when only docs/path/card checks ran
- "coverage passed" when CI skipped coverage by path policy
- "runtime unchanged" unless the changed paths actually support that claim

When prompt cards are involved, the `SOR` should name the exact renderer and
structure checks that ran. When redaction- or observability-facing docs are
involved, the `SOR` should say whether the proof was contract-only or whether a
real command/log proof was run.

## Non-Closing Lifecycle PR Policy

A docs-only PR may complete issue-local execution without closing a larger
lifecycle surface.

Examples:

- a child issue under an open sprint umbrella
- a review-hold closeout packet for an umbrella that intentionally remains open
- a milestone truth update that records deferred follow-ons rather than closing
  the milestone

The policy is:

- `pr-finish` may publish a docs-only PR when the issue-local execution is done
- publication must not silently close an umbrella, sprint, milestone, or review
  surface that still has active follow-ons
- `pr-closeout` happens only after merge or explicit no-PR closure truth is
  settled
- open umbrellas must say why they remain open and which follow-ons still govern
  closure

So a docs-only PR is allowed to finish locally while a parent lifecycle record
remains intentionally open.

## Examples

### Allowed Docs-Only Bundle

- milestone README truth updates
- sprint closeout packet updates
- review packet wording fixes
- prompt-card truth repair without renderer changes

### Must Widen

- changing `pr finish` selection logic
- changing prompt-template Rust code
- changing structured prompt validator behavior
- changing observability emission code

## Non-Claims

- This policy does not bypass `workflow-conductor`.
- This policy does not let docs-only work skip truthful `SRP`/`SOR` updates.
- This policy does not turn a green stable CI check into release evidence.
- This policy does not allow hidden runtime/tooling edits to ride a docs-only
  lane.
