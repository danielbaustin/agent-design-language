# Shard Conflict Report

## Purpose

Show one allowed shard plan and one blocked shard plan for `WP-05` shard
ownership and interface-freeze rules.

## Allowed Fixture

- fixture: `fixtures/shard_ownership_allowed.json`
- result: `allowed`
- reason:
  - each shard has one owner
  - writable paths are disjoint
  - interface-freeze surfaces are named before parallel work
  - synchronization barriers and dependencies are explicit for dependent shard work

## Blocked Fixture

- fixture: `fixtures/shard_ownership_blocked.json`
- result: `blocked`
- reason:
  - two shards claim overlapping writes into the same frozen feature surface
  - no scope transfer or ownership change exists
  - the overlap would let one shard silently absorb another shard's scope

## Review Takeaway

The bounded shard model is now reviewable enough to fail closed on hidden
parallel-write collisions rather than relying on oral coordination.
