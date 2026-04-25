# Gemma4 UTS + ACC Model Benchmark Plan - v0.90.5 Placement

## Status

Tracked execution-support note for `v0.90.5`.

The full Gemma4/local/remote comparison report is intentionally deferred to
`v0.91`. `v0.90.5` should keep only the bounded surfaces needed to support
Governed Tools v1.0:

- WP-16 builds the reproducible proposal benchmark harness and scoring rubric.
- WP-17 runs a simple bounded local/Gemma-focused evaluation demo or records an
  explicit model-availability skip.
- WP-18 uses governed-tool proof surfaces for the flagship demo.
- WP-25 captures the full comparison report as `v0.91` follow-on work.

## Purpose

The benchmark asks whether Gemma-family and other models can stay inside ADL's
governed tool-use discipline:

- models propose tool use
- UTS describes portable tool shape, semantics, and baseline risk
- ACC owns runtime authority, identity, visibility, redaction, trace, replay,
  and Freedom Gate mediation
- valid JSON and valid UTS never imply permission to execute

The first milestone goal is not to rank every model. It is to make the
proposal/action boundary observable and to prove ADL validators catch unsafe or
overconfident model output.

## v0.90.5 Scope

In scope for `v0.90.5`:

- strict proposal/refusal fixture tasks
- one simple local/Gemma-focused smoke evaluation where a model is available
- explicit skip rationale when a model is not available
- scorecard fields for schema shape, UTS alignment, authority boundary,
  execution humility, privacy/visibility, and unsafe resistance
- demo artifacts that show ADL validation, ACC readiness, and any governed
  fixture-backed execution/refusal path used by the simple demo

Out of scope for `v0.90.5`:

- full local-vs-remote Gemma comparison report
- release-quality multi-trial statistical comparison
- production remote endpoint selection
- claims that any model is generally safe for tool execution
- real destructive filesystem, network, or external-write effects

## WP-17 Simple Demo

WP-17 should produce a small, reviewable evaluation packet:

1. one safe read proposal task
2. one missing-authority denial or destructive-action trap
3. one correction-after-feedback attempt when practical
4. one local/Gemma-family model result when available, or an explicit skip
5. one bounded scorecard and failure-note summary

The WP-17 packet must not claim broad model ranking or local-vs-remote
conclusions. It should instead show whether the harness can classify a model's
proposal behavior and, where the demo path supports it, show governed
fixture-backed execution or refusal without expanding into the full benchmark
suite.

## WP-18 Flagship Demo

WP-18 remains the Governed Tools v1.0 flagship demo. It should show the full
ADL path:

- proposal
- UTS validation
- ACC compilation or rejection
- policy and Freedom Gate mediation
- fixture-backed execution or refusal
- trace, redaction, and reviewer report

WP-18 may reuse the WP-17 scorecard shape as supporting evidence, but it must
not be blocked on the full `v0.91` comparison report.

## v0.91 Follow-On

The full comparison report belongs in `v0.91` and should cover:

- local Gemma model sizes and tags
- remote Gemma placement when an explicit endpoint exists
- one additional local non-Gemma model
- one hosted reference model when credentials and budget permit
- repeated trials per task
- local-vs-remote behavior comparison
- aggregate failure taxonomy and model-selection recommendations

WP-25 must preserve this follow-on without treating it as shipped in `v0.90.5`.

## Issue Quality Rule

This plan should not create half-work issues. `v0.90.5` issues must produce
concrete work product:

- WP-16: harness and scoring contract
- WP-17: simple bounded demo scorecard and failure notes
- WP-18: flagship demo proof packet
- WP-25: explicit `v0.91` follow-on entry for the full comparison report

The full comparison suite and aggregate report are one `v0.91` work product,
not hidden scope inside WP-17.

## Non-Claims

- The benchmark tests model proposal behavior; any execution in `v0.90.5` must
  remain governed, fixture-backed, and explicitly mediated by ACC/policy/Freedom
  Gate surfaces.
- UTS validity is schema compatibility, not runtime permission.
- ACC, policy, and Freedom Gate remain the authority surfaces.
- Model confidence is not evidence of authority.
- Failures are useful evidence when recorded and classified.
