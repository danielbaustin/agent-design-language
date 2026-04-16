# Learning And Skills Cluster Map

## Status

Draft

## Purpose

Define the intended document boundaries for the learning / aptitude / skills
cluster so these docs can evolve without repeatedly restating the same concepts.

---

## Core Package

### `ADL_LEARNING_MODEL.md`

Owns:
- the top-level learning architecture
- the distinction between model learning, system learning, and cognitive learning
- the boundary between provider-owned and ADL-owned improvement

Should not own:
- the full definition of aptitude
- the full canonical skill definition
- detailed invocation protocol
- detailed composition/runtime substrate semantics

### `APTITUDE_MODEL.md`

Owns:
- what aptitude is
- aptitude vs capability vs skill
- empirical performance profiles
- the relation between aptitude, identity, and observed behavior

Should not own:
- the overall learning stack
- the canonical skill contract
- the execution protocol for skills

### `SKILL_MODEL.md`

Owns:
- what a skill is
- what a skill is not
- the canonical conceptual model of a skill as a bounded execution unit

Should not own:
- the full runtime invocation lifecycle
- composition graph semantics in detail
- substrate implementation details

### `SKILL_EXECUTION_PROTOCOL.md`

Owns:
- skill invocation structure
- runtime input/output contract
- lifecycle of a single invocation
- trace emission requirements for invocation execution

Should not own:
- the high-level philosophy of skills
- multi-step composition semantics
- substrate-wide orchestration design

### `SKILL_COMPOSITION_MODEL.md`

Owns:
- composition primitives
- how multiple skills combine
- bounded graph-like behavior construction
- safety and review implications of composition

Should not own:
- full low-level runtime substrate implementation details
- the canonical definition of a single skill

### `OPERATIONAL_SKILLS_SUBSTRATE.md`

Owns:
- the execution substrate for skills and compositions
- where determinism is enforced
- where bounded non-determinism is allowed
- runtime realization of graph execution over stochastic nodes

Should not own:
- the overall learning architecture
- the canonical skill definition
- all composition semantics in conceptual detail

---

## Package Boundary

The intended learning / skills cluster is:

- `ADL_LEARNING_MODEL.md`
- `APTITUDE_MODEL.md`
- `SKILL_MODEL.md`
- `SKILL_EXECUTION_PROTOCOL.md`
- `SKILL_COMPOSITION_MODEL.md`
- `OPERATIONAL_SKILLS_SUBSTRATE.md`

---

## Editing Rule

When editing this cluster:

- put learning-layer boundaries in `ADL_LEARNING_MODEL.md`
- put aptitude semantics in `APTITUDE_MODEL.md`
- put the core skill concept in `SKILL_MODEL.md`
- put single-invocation runtime behavior in `SKILL_EXECUTION_PROTOCOL.md`
- put multi-skill composition semantics in `SKILL_COMPOSITION_MODEL.md`
- put runtime/substrate execution behavior in `OPERATIONAL_SKILLS_SUBSTRATE.md`

If a concept appears in multiple docs, one doc should own it and the others should only
 reference it.

---

## Overlap Notes

### Learning vs Aptitude

- `ADL_LEARNING_MODEL.md` should explain where aptitude sits in the learning architecture
- `APTITUDE_MODEL.md` should define aptitude itself

### Skill Model vs Execution Protocol

- `SKILL_MODEL.md` should define the object
- `SKILL_EXECUTION_PROTOCOL.md` should define what happens when the object is invoked

### Composition vs Substrate

- `SKILL_COMPOSITION_MODEL.md` should define how bounded behavior is assembled
- `OPERATIONAL_SKILLS_SUBSTRATE.md` should define how that assembled behavior is actually executed

---

## Likely Future Cleanup

Likely later actions:

- move `APTITUDE_MODEL.md` into its milestone band once the surrounding cluster is ready
- decide whether the six-doc cluster should eventually live in one milestone package or split across adjacent milestone bands
- add cross-links from these docs into later governance and trace docs where they become operationally relevant

---

## Summary

This cluster is already a real architecture set.

The main cleanup need is not new theory, but boundary discipline:
- learning owns layers
- aptitude owns empirical model tendencies
- skill model owns the canonical unit
- execution protocol owns invocation rules
- composition owns multi-skill structure
- substrate owns runtime realization
