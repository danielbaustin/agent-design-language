# Codex-Only Parallel Worker Slice (2026-05-29)

## Status

Ad hoc follow-on evidence for issue `#3419`.

## Purpose

Record the first successful bounded slice where two hosted Codex worker lanes
executed disjoint shard edits in parallel and a hosted Codex reviewer lane
reviewed the resulting diffs without material findings.

This note is additive evidence. It is not the canonical output of
`adl/tools/run_v0914_multi_agent_workcell_proof.sh`.

## Setup

Because the earlier nested demo path introduced Codex sandbox and patch-context
friction, this slice was run in a flatter temporary repository with explicit
worker prompts and disjoint worktrees:

- worker A worktree: `worker_a`
- worker B worktree: `worker_b`
- reviewer used embedded diffs from both worker worktrees

The two worker prompts each owned exactly two files:

- Worker A
  - `docs/worker_a_summary.md`
  - `.adl/v0.91.4/tasks/issue-6001__demo-codex-worker-a/sor.md`
- Worker B
  - `docs/worker_b_contract.md`
  - `.adl/v0.91.4/tasks/issue-6002__demo-codex-worker-b/sor.md`

## Result

Observed outcome:

- two hosted Codex worker lanes completed in parallel
- both workers stayed within their declared write sets
- both workers produced the requested normalized publication-ready content
- one hosted Codex reviewer lane reviewed the two diffs
- reviewer result: no material findings

## Worker A Output

- Worker A updated only the summary file assigned to its local worker lane.
- Worker A ran in parallel with Worker B while write ownership stayed separated by file path.
- Reviewer, janitor, and closeout decisions remained serialized outside the worker lane.

Worker A SOR:

```text
status: draft
summary: normalized worker_a_summary.md for serialized reviewer publication in the demo proof
```

## Worker B Output

1. Worker B may edit only its assigned contract file and its own local SOR record.
2. Worker B must not modify Worker A paths or shared proof packet surfaces.
3. Worker B completes after recording its bounded file-local update for reviewer publication.

Worker B SOR:

```text
status: draft
summary: normalized worker_b_contract.md for serialized reviewer publication in the demo proof
```

## Reviewer Output

Reviewer disposition:

- no material findings
- write ownership separation preserved
- `status: draft` remained appropriate because reviewer/janitor/closeout stayed serialized

Residual risk called out by the reviewer:

- review covered supplied diffs only
- surrounding repository state and extra unreviewed changes were not independently verified

## What This Proves

- hosted Codex can execute bounded disjoint worker shards in parallel
- hosted Codex can preserve narrow write ownership under explicit prompts
- hosted Codex reviewer output is useful on the resulting shard diffs

## What This Does Not Prove

- production-grade autonomous merge or closeout
- absence of provider/runtime friction in every workspace topology
- that raw Codex-only shard publication should bypass serialized downstream gates

## Truth Boundary

This was an ad hoc temporary-repository slice run to answer whether multiple
hosted Codex workers could succeed in parallel on disjoint shards.

It is valuable evidence for `#3419`, but it is not a replacement for the
tracked runner-based workcell proof packet.
