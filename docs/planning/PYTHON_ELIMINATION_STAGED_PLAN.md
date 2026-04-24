# Python Elimination Staged Plan

Date: 2026-04-24

## Decision

ADL should not turn Python elimination into a single dedicated milestone gate.

That shape would:

- stall milestone delivery too hard
- consume too much issue and review ceremony
- amplify CI frustration and rate-limit burn during a period when the roadmap
  still needs to ship meaningful work
- increase the risk that the migration itself becomes destabilizing

Instead, `v0.90.5` should remain Governed Tools v1.0, and Python elimination
should become a managed cross-milestone burn-down program.

## Program Rule

From `v0.90.5` forward:

- every milestone should carry a small bounded Python-reduction tranche
- every tranche should delete or replace a coherent family of Python surfaces
- no new tracked Python should be added unless explicitly waived
- the tracked Python file and LOC counts should trend downward every milestone

This keeps product momentum while still reducing structural risk.

## Budget Rule

Recommended budget:

- normal milestone: `2` to `4` Python-reduction WPs
- heavy feature milestone: `1` to `2` Python-reduction WPs
- lighter planning or docs milestone: up to `4` or `5` if capacity exists

Python elimination should normally stay under about `15%` to `25%` of a
milestone unless a specific emergency requires more.

## Migration Rule

For each Python tranche:

- prefer porting or deleting a whole surface family, not random files
- prefer delete-or-collapse over literal transliteration
- require parity proof only for live behavior that still matters
- if a Python file is stale, remove it instead of honoring it with a Rust twin

## Milestone Allocation

### v0.90.5

Primary milestone:

- Governed Tools v1.0

Python tranche:

- freeze and no-new-Python rule
- inventory and disposition truth surface
- one high-leverage tooling family

Goal:

- start the burn-down without letting it dominate the milestone

### v0.90.6

Primary milestone:

- follow-on governed-tools or adjacent runtime work

Python tranche:

- biggest control-plane Python families that still remain

Goal:

- remove the most structurally dangerous Python first

### v0.91

Primary milestone:

- moral governance and wellbeing foundations

Python tranche:

- validators, renderers, and proof surfaces used by milestone and review flow

Goal:

- stop relying on Python for proof and release truth

### v0.92

Primary milestone:

- identity, memory grounding, and birthday

Python tranche:

- provider mocks, adapters, bridge cleanup, and odd edge surfaces

Goal:

- finish the weird families that are easy to leave behind

### v0.93

Primary milestone:

- constitutional citizenship, ToM, reputation, and polis governance

Python tranche:

- remaining low-count leftovers
- CI zero-Python gate if the footprint is already near zero

Goal:

- make zero tracked Python realistic without delaying the social stack

### v0.94

Primary milestone:

- publication, review hardening, and broader polish bands

Python tranche:

- final cleanup only if needed

Goal:

- Python count should be zero by or before here

## Surface Priority

Migrate by structural risk, not by raw file count alone:

1. workflow and control-plane skill scripts
2. repo packet and review planning
3. review-quality and report assembly
4. validators and renderers used in milestone or release truth
5. demo runners and benchmark helpers
6. provider mocks and adapters
7. fixture protocols and tiny leftovers

## Milestone-Level Success Metric

Each milestone should record:

- tracked Python file count at start
- tracked Python file count at end
- tracked Python LOC at start
- tracked Python LOC at end
- which Python families were eliminated or reduced
- which families remain

If a milestone does not reduce the Python footprint, that should be explicit and
justified.

## Working Policy

Effective policy recommendation:

- do not add new tracked Python by default
- if a temporary Python exception is truly needed, it must name:
  - owner
  - rationale
  - removal milestone
- default answer remains Rust first

## Bottom Line

Python elimination is important, but it should behave like a managed
debt-retirement program inside the roadmap, not like a milestone that eclipses
the roadmap.
