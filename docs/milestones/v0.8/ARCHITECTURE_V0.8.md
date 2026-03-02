# ADL v0.8 Architecture (Planning)

## Scope Positioning

v0.8 is scoped to **EPIC-C + EPIC-D** only:
- Gödel-style policy evolution surfaces
- Authoring surfaces and related UX hardening

v0.8 explicitly does **not** include:
- ObsMem v1 integration (moved to v0.75 planning)
- Distributed / cluster execution (deferred to v0.85/v0.9 planning)

## Dependency Boundaries

v0.8 assumes deterministic substrate work is already complete from:
- v0.7 foundation hardening
- v0.75 deterministic substrate + ObsMem v1 planning track

This means v0.8 design docs should treat ObsMem and cluster features as
external dependencies, not in-milestone deliverables.

## Cross-References

- v0.7 release-train docs: `docs/milestones/v0.7/`
- v0.8 incubation docs: `docs/milestones/v0.8/incubation/`
- v0.8 epic mapping: `docs/milestones/v0.8/EPIC_MAPPING_v0.8.md`
