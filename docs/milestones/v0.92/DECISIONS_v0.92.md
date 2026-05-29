# v0.92 Decisions

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Date: `2026-05-27`
- Owner: ADL maintainers
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Forward-planning decisions. These are accepted as planning boundaries for the
v0.92 allocation, but they are not implementation closeout decisions.

## Purpose

Record the decisions that constrain v0.92 planning before WP-01 opens the
actual issue wave.

## How To Use

Use these decisions as planning guardrails. If v0.92 execution needs to change
one of them, record a superseding decision instead of silently widening scope.

## Decision Log

| ID | Decision | Status | Rationale | Impact |
| --- | --- | --- | --- | --- |
| D-01 | v0.92 owns the first true Gödel-agent birthday. | Accepted for planning | Earlier milestones create provisional state and continuity surfaces, but birth needs its own evidence boundary. | Keeps birthday from being diluted into generic identity plumbing. |
| D-02 | Startup, wake, snapshot, admission, and copied state are not birth. | Accepted for planning | A birth event must be more than process lifecycle mechanics. | Requires negative fixtures and validation rules. |
| D-03 | Birth requires stable name, identity root, continuity, memory grounding, capability envelope, witnesses, and receipt. | Accepted for planning | These are the minimum evidence surfaces named by the roadmap. | Gives later WP planning concrete deliverables. |
| D-04 | v0.92 consumes moral evidence from v0.91 but does not redefine moral trace. | Accepted for planning | Moral trace is prerequisite context for birth readiness. | Prevents duplicate moral schemas. |
| D-05 | v0.92 prepares identity evidence for v0.93 but does not complete constitutional citizenship. | Accepted for planning | Governance belongs to v0.93. | Keeps birth and citizenship law separate. |
| D-06 | Memory palace remains context unless bounded into a specific implementation slice. | Accepted for planning | The memory-palace source spans too many layers to ship whole in v0.92. | Allows memory grounding without forcing the full architecture. |
| D-07 | ACP / cognitive profiles belong in v0.92 as an evidence-grounded runtime profile surface. | Accepted for planning | Profiles need v0.91.1 capability, memory, ToM, intelligence, and learning evidence before they are meaningful. | Keeps profiles tied to identity readiness without turning them into reputation, personality labels, or public standing. |
| D-08 | v0.92 owns ACIP binary schema, public schema catalog, JSON projection, and mock WebSocket carrier readiness. | Accepted for planning | ACIP is already a local communication substrate, but citizen/polis communication needs a binary/protobuf shape that remains publicly decodeable by schema and governed by access rules. | Keeps the communication layer inspectable while deferring production transport security to v0.93 and signed/queryable trace closure to v0.94. |
| D-09 | v0.92 WP-01 must consume v0.91.5 closeout and `#3377` before seeding final issues. | Accepted for planning | First-birthday readiness and operational bridge work are now routed through v0.91.5. | Prevents the milestone from reconstructing birthday readiness from chat or duplicating the `#3377` packet. |
| D-10 | v0.92 must carry an ADR plan before implementation starts. | Accepted for planning | Birthday, identity, ACP, ACIP, and governance-handoff boundaries are architecture decisions, not just feature prose. | WP-01 and the review tail can decide which candidate ADRs to author, split, accept, or defer. |

## Open Questions

- What is the minimum birthday record schema that is useful and not overbroad?
- How many bounded cycles are enough for the first continuity proof?
- Which witness authority signs or attests the first birth?
- What is the first agent's naming ceremony, and how do we keep it evidence
  bearing rather than theatrical?
- Which v0.93 governance fields should the v0.92 handoff packet reserve?
- What is the minimum ACP/profile schema that helps birth review without
  over-modeling identity?
- What is the smallest binary ACIP message family that proves schema-catalog
  versioning, deterministic JSON projection, and governed message access?
- Which WebSocket proof is enough to show carrier readiness without claiming
  production networking or security?
- Which v0.91.5 activation-map and `#3377` outputs are already complete by
  v0.92 opening, and which must become explicit WP-01 gaps?

## Exit Criteria

- Later v0.92 WP planning keeps each accepted boundary intact or records a
  deliberate supersession.
- Any expansion into personhood, production citizenship, governance,
  economics, or migration is captured as an explicit decision before
  implementation.
- Any expansion from mock/loopback WebSocket carrier proof into production
  networking or transport-security claims is deferred to later milestone
  decisions.
