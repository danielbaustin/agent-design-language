# ADL v0.8 Architecture Slice

This document defines the tracked v0.8 architecture slice used during v0.7 closeout planning.

## Milestone Slicing (Canonical)

- `v0.75`: EPIC-A + EPIC-B (Deterministic Substrate + ObsMem v1)
- `v0.8`: EPIC-C + EPIC-D (Godel + Authoring)
- `v0.85+`: Cluster / distributed execution

## Scope for v0.8

v0.8 focuses on the authoring and adaptive workflow surface:

- Godel-style artifacted self-improvement pattern design
- Authoring ergonomics and workflow composition improvements
- Deterministic interfaces and replay-safe artifact boundaries

## Explicitly Out of Scope for v0.8

- ObsMem v1 integration work (owned by v0.75)
- Cluster/distributed execution runtime work (owned by v0.85+)

## Cross-References

- ObsMem planning source (to be moved into tracked docs at milestone start): `.adl/docs/v075planning/OBSMEM_BAYES.md`
- Cluster planning source (to be moved into tracked docs at milestone start): `.adl/docs/v085planning/CLUSTER_EXECUTION.md`
- Current incubation material: `docs/milestones/v0.8/incubation/GODEL_AGENT.md`
