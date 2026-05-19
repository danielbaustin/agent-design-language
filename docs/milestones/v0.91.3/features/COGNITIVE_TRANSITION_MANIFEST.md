# Cognitive Transition Manifest

## Metadata

- Feature Name: Cognitive Transition Manifest
- Milestone Target: `v0.91.3`
- Status: planned
- Planned WP Home: WP-02 through WP-07

## Purpose

Define the first machine-checkable manifest for a Cognitive State Transition.

The manifest is the bridge between issue cards, execution shards, review
evidence, merge readiness, outcome truth, and memory handoff.

## Expected Shape

The initial manifest should record:

- transition identity
- issue and branch/worktree identity
- base and candidate repository state
- SIP, STP, SPP, SRP, and SOR paths
- DAG and shard plan paths
- evidence bundle path
- review synthesis path
- merge-readiness gate result
- ObsMem handoff boundary

## Acceptance Criteria

- Valid and invalid fixtures exist.
- Repo-relative paths are used.
- Missing cards, stale branch state, and missing review results are rejected or
  classified clearly.
- The manifest does not replace GitHub PRs; it describes and governs the
  transition around them.

