# Cognitive Transition Schema

## Status

Tracked schema summary for the `v0.91.3` first slice and `v0.91.4` hardening
milestone.

## Purpose

The Cognitive Transition schema describes the minimum object model needed to
make C-SDLC transitions reviewable, replayable, measurable, and governed.

## Transition Record

A transition record should include:

- transition id
- issue id and URL
- milestone/version
- branch and worktree identity
- pull request URL
- actor and role references
- card record paths for `SIP`, `STP`, `SPP`, `SRP`, and `SOR`, where `SPP`
  means Structured Plan Prompt
- transition DAG or shard plan
- evidence bundle references
- review synthesis reference
- merge-readiness gate result
- trace or signed-trace proof references
- ObsMem handoff reference

## Lifecycle States

Initial C-SDLC implementations should distinguish at least:

- `planned`
- `bound`
- `in_progress`
- `review_ready`
- `reviewed`
- `merge_ready`
- `merged`
- `closed_out`
- `blocked`
- `superseded`

These states must reflect GitHub and repo evidence. They are not aspirational
labels.

## Shard Model

A shard is a bounded work slice owned by one actor or agent. Shards should
declare:

- scope
- owner/actor
- writable paths
- dependencies
- interface-freeze constraints
- proof obligations
- merge/convergence requirements

Parallel shards are safe only when their boundaries and convergence points are
explicit.

## Evidence Bundle

The evidence bundle should collect:

- changed files
- validation commands and results
- review findings
- finding dispositions
- trace/proof references
- demo or replay results when relevant
- residual risks

Evidence must be repo-relative and durable when it is used for review,
closeout, release evidence, or memory.

## Merge-Readiness Gate

The merge-readiness gate should fail closed when:

- issue truth is stale
- branch/worktree truth is ambiguous
- validation evidence is missing or overstated
- review findings are unresolved
- signed trace proof is required but missing
- durable records remain local-only
- closeout would overclaim integration truth

## Memory Boundary

`SRP`, `SOR`, evidence bundles, and signed trace references are the primary
inputs for ObsMem. Memory should be derived from tracked evidence, not from
local lore or chat-only context.
