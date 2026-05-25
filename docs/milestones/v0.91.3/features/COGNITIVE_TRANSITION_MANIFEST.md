# Cognitive Transition Manifest

## Metadata

- Feature Name: Cognitive Transition Manifest
- Milestone Target: `v0.91.3`
- Status: proven first-slice schema surface
- Planned WP Home: WP-02 through WP-07 / #3200 through #3205

## Purpose

Define the first machine-checkable manifest for a Cognitive State Transition.

The manifest is the bridge between issue cards, execution shards, review
evidence, merge readiness, outcome truth, and memory handoff.

## Implemented Shape

The initial manifest records or links:

- transition identity
- issue and branch/worktree identity
- actor and role references for material participants
- SIP, STP, SPP, SRP, and SOR paths, where SPP means Structured Plan Prompt
- DAG and shard plan paths
- evidence bundle path
- trace/proof manifest path
- review synthesis path
- merge-readiness gate result
- ObsMem handoff boundary

`WP-02` now establishes the first machine-checkable contract surface at:

- `adl/src/cognitive_transition_schema.rs`

## Acceptance Criteria

- Valid and invalid fixtures exist.
- Repo-relative paths are used.
- Actor and role references identify the operator, conductor/lifecycle router,
  implementation or shard owner, reviewer, verifier, and closeout owner when
  those roles are present.
- Missing cards, stale branch state, and missing review results are rejected or
  classified clearly.
- Trace/proof references are repo-relative and not local-only.
- The manifest does not replace GitHub PRs; it describes and governs the
  transition around them.
- The manifest seeds, but does not complete, the v0.91.4 Software Development
  Polis and actor-standing model.

## Current WP-02 Proof Surface

`WP-02` owns the first bounded schema slice:

- schema id: `cognitive_transition_manifest.v1`
- required seed roles:
  - `operator`
  - `lifecycle_router`
  - `implementation_owner`
- validator:
  - `validate_cognitive_transition_manifest_v1(...)`
- fixture helper:
  - `wp02_cognitive_transition_manifest_valid_fixture()`
- tracked JSON fixtures:
  - `docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`
  - `docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`
- durable tracked card bundle:
  - `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3200-card-schema-proof/`
- focused proof command:
  - `cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture`

This is intentionally bounded. DAG convergence, evidence-bundle richness,
merge-readiness enforcement, and ObsMem handoff hardening stay with later WPs.

The `WP-02` manifest now points at tracked workflow card snapshots for
reviewer-facing proof surfaces. Local `.adl` issue-card copies remain
derivation and execution-history inputs, and the promoted tracked snapshots may
still preserve bounded historical local references while the broader
tracked-workflow migration remains incomplete.
