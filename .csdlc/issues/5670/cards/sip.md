# Structured Intent Prompt

Template: 1.0.0

Issue: 5670

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Reduce PR latency by fanning hosted coverage work into parallel shard producers while preserving one authoritative final coverage gate.

## Required Outcome

Hosted coverage uses deterministic parallel shard producers and a fail-closed aggregation gate without weakening coverage thresholds, ownership filtering, path policy, or evidence truth.

## Scope

- GitHub Actions hosted coverage topology
- authoritative coverage runner shard controls
- focused CI/runtime/path-policy contract tests
- issue-local C-SDLC v2 lifecycle records

## Authority

- typed C-SDLC v2 remains lifecycle authority
- adl-coverage-hosted remains the final required coverage gate
- coverage shards produce evidence; aggregation owns final authority
- FastWork is required for local build and temp output

## Assumptions

- none

## Operator Constraints

- Use FastWork only for worktree, temp, and build output
- Do not write implementation changes on root main
- No AWS execution
- No threshold reduction or hidden test-scope reduction
- Move quickly and avoid broad rewrites
