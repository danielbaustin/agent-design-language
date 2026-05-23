# Transition DAG And Shard Coordination

## Status

Proven `v0.91.3` feature under `WP-04` / `#3202`.

## Purpose

Define the first C-SDLC transition graph: the serial steps, bounded shards,
synchronization barriers, and interface-freeze rules needed to execute one
Cognitive State Transition without hidden coordination.

The five-minute sprint becomes convincing only if speed is produced by clear
division of work, not by skipping review or allowing agents to collide.

## Scope

The first slice must define:

- a transition DAG fixture for one bounded issue
- serial phases that cannot be parallelized safely
- shardable work areas with explicit ownership
- synchronization barriers before review, merge readiness, and closeout
- interface-freeze rules that prevent overlapping writes unless an explicit
  synchronization step exists

## Acceptance Criteria

- The transition manifest links to a DAG artifact.
- The DAG identifies serial nodes, shard nodes, and barrier nodes.
- Each shard has a named owner, allowed write surface, and handoff contract.
- Review can determine whether parallel work respected shard boundaries.
- The first proof reports coordination latency separately from implementation
  time.

## Current WP-04 Proof Surface

`WP-04` now anchors the first tracked DAG/shard packet at:

- `docs/milestones/v0.91.3/review/transition_dag/`

That packet contains:

- one transition DAG fixture with explicit serial, shard, and barrier nodes
- one shard plan with bounded ownership and interface-freeze rules
- one validator/test lane proving the packet contract is present and stable
- one schema-level proof that the `WP-02` manifest fixture still points at the
  tracked `WP-04` DAG and shard-plan paths

## Non-Goals

- This feature does not attempt broad autonomous parallel engineering.
- This feature does not remove the need for human review.
- This feature does not allow write conflicts to be resolved by silent
  overwrites.
