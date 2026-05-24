# Cognitive Transition Schema

## Status

Tracked schema summary for the `v0.91.3` first slice and `v0.91.4` hardening
milestone. `WP-02` now anchors the first machine-checkable manifest surface in
`adl/src/cognitive_transition_schema.rs`.

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

Canonical first-slice runtime surface:

- `adl/src/cognitive_transition_schema.rs`
- schema id: `cognitive_transition_manifest.v1`

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

The first machine-checkable validator currently accepts the same state list:

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

## Actor-Role Seed

The first slice needs a bounded seed, not the full `v0.91.4` standing model.

The current validator requires these material seed roles:

- `operator`
- `lifecycle_router`
- `implementation_owner`

Later slices may add reviewer, verifier, shard owner, and closeout owner
requirements without redesigning the manifest.

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

## Fixtures And Validation Plan

`WP-02` now proves the schema with:

- valid fixture helper:
  `wp02_cognitive_transition_manifest_valid_fixture()`
- valid JSON fixture:
  `docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`
- invalid JSON fixture:
  `docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`
- durable tracked card proof bundle:
  `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3200-card-schema-proof/`
- validator:
  `validate_cognitive_transition_manifest_v1(...)`
- JSON-schema export:
  `cognitive_transition_manifest_v1_schema_json()`

Focused proof command:

```bash
cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture
```

This proof currently checks:

- schema version gating
- allowed lifecycle states
- repo-relative path rules
- required actor-role seed presence
- JSON-schema surface generation
- durable tracked card references instead of local-only `.adl` card paths

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
