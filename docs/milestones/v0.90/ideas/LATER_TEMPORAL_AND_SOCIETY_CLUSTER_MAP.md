# Later Temporal And Society Cluster Map

## Status

Draft

## Purpose

Define the intended document boundaries for the later-band temporal / identity /
 accountability / social-coordination feature surfaces still living in `local planning backlog/`.

These docs were intentionally kept out of the bounded `v0.88` temporal package because
 they depend on stronger identity, social, and multi-agent substrate maturity.

---

## Core Package

### `CROSS_AGENT_TEMPORAL_ALIGNMENT.md`

Owns:
- temporal alignment across agents
- drift, uncertain ordering, and partial observability between agents
- multi-agent event reconciliation and shared temporal coherence

Should not own:
- single-agent chronosense fundamentals
- the full accountability layer
- full branch/counterfactual identity semantics

### `TIMELINE_FORKS_AND_COUNTERFACTUALS.md`

Owns:
- canonical vs hypothetical timelines
- branch-local reasoning
- replay/counterfactual branch distinctions
- identity risks introduced by forks

Should not own:
- the full social-accountability layer
- cross-agent temporal reconciliation in general

### `TEMPORAL_ACCOUNTABILITY.md`

Owns:
- responsibility over time
- historical interpretation and hindsight boundaries
- relation of continuity, signed history, delegation, and later review

Should not own:
- the full trust/reputation/citizenship system
- all temporal-query mechanics
- the entire branch/fork semantics stack

---

## Package Boundary

The intended later temporal / society cluster is:

- `CROSS_AGENT_TEMPORAL_ALIGNMENT.md`
- `TIMELINE_FORKS_AND_COUNTERFACTUALS.md`
- `TEMPORAL_ACCOUNTABILITY.md`

This cluster is explicitly later than the core `v0.88` temporal package.

---

## Editing Rule

When editing this cluster:

- put multi-agent temporal coherence in `CROSS_AGENT_TEMPORAL_ALIGNMENT.md`
- put branch/counterfactual reasoning in `TIMELINE_FORKS_AND_COUNTERFACTUALS.md`
- put accountability-over-time semantics in `TEMPORAL_ACCOUNTABILITY.md`

If a concept appears in multiple docs, one doc should own it and the others should only
 reference it.

---

## Overlap Notes

### Alignment vs Forks

- `CROSS_AGENT_TEMPORAL_ALIGNMENT.md` should own shared-world temporal ordering across agents
- `TIMELINE_FORKS_AND_COUNTERFACTUALS.md` should own hypothetical branching and non-canonical timelines

### Forks vs Accountability

- `TIMELINE_FORKS_AND_COUNTERFACTUALS.md` should explain how branch structure complicates continuity
- `TEMPORAL_ACCOUNTABILITY.md` should explain how responsibility should be interpreted across time and review

### Accountability vs Governance

`TEMPORAL_ACCOUNTABILITY.md` overlaps with later trust / identity / constitutional docs,
but it is still useful as a bounded extraction of the accountability-over-time problem.

---

## Likely Future Cleanup

Likely later actions:

- decide the eventual milestone band for this cluster
- merge parts of `TIMELINE_FORKS_AND_COUNTERFACTUALS.md` into stronger identity/fork docs if that cluster stabilizes first
- merge parts of `TEMPORAL_ACCOUNTABILITY.md` into later trust/ethics/governance docs if accountability remains cross-cutting instead of feature-shaped

---

## Summary

This cluster captures the temporal features that become important only after bounded
 single-agent chronosense is in place:
- multi-agent temporal coherence
- hypothetical branching and counterfactual reasoning
- accountability over time

