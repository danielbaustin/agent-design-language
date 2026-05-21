# Cognitive Transition Manifest

## Metadata

- Feature Name: Cognitive Transition Manifest
- Milestone Target: `v0.91.3`
- Status: planned
- Planned WP Home: WP-02 through WP-07 / #3200 through #3205

## Purpose

Define the first machine-checkable manifest for a Cognitive State Transition.

The manifest is the bridge between issue cards, execution shards, review
evidence, merge readiness, outcome truth, and memory handoff.

## Expected Shape

The initial manifest should record:

- transition identity
- issue and branch/worktree identity
- actor and role references for material participants
- base and candidate repository state
- SIP, STP, SPP, SRP, and SOR paths
- DAG and shard plan paths
- evidence bundle path
- trace/proof manifest path
- review synthesis path
- merge-readiness gate result
- ObsMem handoff boundary

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
