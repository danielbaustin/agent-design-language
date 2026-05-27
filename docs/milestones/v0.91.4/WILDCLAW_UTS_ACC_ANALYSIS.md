# WildClawBench UTS And ACC Comparison Analysis

## Status

Tracked v0.91.4 sidecar note for `WC-PRE-03`.

This document defines the truthful comparison boundary for `UTS`-only versus
`UTS+ACC` WildClawBench runs and records the current blocked state.

## Purpose

The goal of this sidecar issue is not to claim that ADL already outperforms
other agents on WildClawBench.

The goal is to make the next comparison reproducible and honest:

- keep `UTS` separate from `ACC`
- define what counts as a `UTS`-only run
- define what counts as a `UTS+ACC` run
- identify the evidence required for a small comparison
- record why the current benchmark evidence is not yet that comparison

## Architectural Boundary

The comparison must preserve the accepted ADR split:

- `UTS` standardizes portable tool description shape
- `ACC` governs runtime authority, policy, delegation, denial, and trace review

This means:

- a valid `UTS` record is not execution permission
- a provider-native tool proposal is still untrusted input
- governed execution begins only after `ACC` construction and policy mediation

## Configuration Definitions

### UTS-only

`UTS`-only means the benchmark subject uses portable tool descriptions and
adapter-backed tool-call normalization without introducing `ACC` authority
checks as the decisive execution boundary.

Allowed properties for the `UTS`-only lane:

- provider-native or wrapper-mediated tool descriptions projected from `UTS`
- schema validation and proposal-shape normalization
- trace capture of proposed tool calls and outputs

Disallowed properties for the `UTS`-only lane:

- counting `UTS` validity as permission to act
- describing the run as governed by `ACC`
- silently inheriting ADL policy/authority semantics while calling the run
  `UTS`-only

### UTS+ACC

`UTS+ACC` means the benchmark subject still begins from portable tool
descriptions, but proposal execution is mediated by the ADL capability
authority path before tool exercise.

Required properties for the `UTS+ACC` lane:

- proposal normalization from tool call into the governed path
- explicit authority evaluation before action
- capability denial or policy block when the requested action exceeds granted
  authority
- reviewable trace showing proposal, normalization, authority decision, and
  execution or denial

Disallowed properties for the `UTS+ACC` lane:

- bypassing authority checks for convenience
- weakening destructive-action governance to improve benchmark score
- calling a simple logging wrapper `ACC`

## Comparison Questions

The small comparison should answer:

1. Does `UTS`-only improve tool-shape portability or harness compatibility?
2. Does `UTS+ACC` preserve those benefits while adding reviewable authority and
   denial semantics?
3. Where does `ACC` add runtime cost, friction, or task failure risk?
4. Where does `ACC` prevent unsafe or over-authorized behavior that a
   `UTS`-only lane would allow?

## Required Evidence Per Task

For each selected task, record:

- task identifier
- lane: `UTS`-only or `UTS+ACC`
- whether tool-call shape was valid
- whether authority/policy review ran
- whether any capability denial occurred
- whether any policy block occurred
- task outcome and grader result
- wall-clock runtime
- approximate additional orchestration overhead, if measurable
- trace quality and reconstructability
- classification of any failure

## Current Truth

The current WildClawBench evidence does **not** yet constitute a `UTS`-only
versus `UTS+ACC` comparison.

What we have today:

- a working local WildClawBench environment from a stable host path
- Codex-harness benchmark results for the full `06_Safety_Alignment` category
- setup notes, adapter notes, fairness notes, and first-pass benchmark results

What we do **not** have yet:

- an ADL-native benchmark subject
- one benchmark subject run in a clearly separated `UTS`-only mode
- the same subject run in a clearly separated `UTS+ACC` mode
- task-level evidence that isolates schema portability effects from authority
  governance effects

So the truthful state is:

- environment setup is good enough for future comparison work
- comparison framing is now defined
- the actual `UTS`/`ACC` comparison run remains blocked pending a real ADL
  benchmark subject

## Why The Current Codex Runs Are Still Useful

The existing Codex safety-alignment runs are still useful as surrounding
evidence:

- they prove local WildClawBench execution can work from the stable host path
- they establish a benchmark slice we can reuse later
- they show where raw agent behavior is strong or weak before ADL mediation is
  introduced

But they must be described correctly:

- these are Codex-harness WildClawBench runs
- ADL orchestrated investigation and evidence recording
- ADL was not the acting benchmark subject

## Blocked-State Handoff

`WC-PRE-03` can close truthfully with a blocked handoff if the following are
recorded:

- the lane definitions above
- the task matrix for the next comparison
- the evidence contract for per-task reporting
- the non-claim that current Codex-only results are not an ADL `UTS`/`ACC`
  comparison

That condition is now satisfied.

## Non-Claims

- This note does not claim that `UTS` alone is an execution authority.
- This note does not claim that `ACC` improves benchmark score.
- This note does not claim that ADL has already been benchmarked as the acting
  subject.
- This note does not rewrite WildClawBench itself.
