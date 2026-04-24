# Issue Splitter Playbook

Use `issue-splitter` when an issue looks too broad, mixes multiple concern
families, or contains explicit follow-on planning hints.

## Evidence Order

Prefer these surfaces:

1. source issue prompt
2. `stp.md`
3. `sip.md`
4. `sor.md` when it already records deferred work

## Concern Buckets

Prefer explicit bucket prefixes when they exist, such as:

- `runtime:`
- `tooling:`
- `docs:`
- `tests:`
- `review:`
- `release:`
- `security:`
- `process:`

Fallback keyword bucketing is acceptable, but if the packet is too vague,
classify `blocked` rather than inventing clean split boundaries.

## Classification Heuristics

- `keep_as_is`
  - one dominant bucket
  - supporting docs/tests remain normal proof work
- `split_now`
  - multiple strong buckets and explicit split/follow-on language
  - clean child issue titles can be proposed without losing traceability
- `defer`
  - mixed buckets exist but the split should wait
- `blocked`
  - packet says both “must stay together” and “split now”
  - current issue scope cannot be narrowed safely

## Handoff

- `keep_as_is` -> normal workflow execution
- `split_now` or `defer` -> `finding-to-issue-planner`, `pr-init`, or operator approval path
- `blocked` -> operator review
