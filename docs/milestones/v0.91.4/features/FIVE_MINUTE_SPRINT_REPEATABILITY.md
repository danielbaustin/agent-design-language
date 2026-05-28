# Five-Minute Sprint Repeatability

## Metadata

- Feature Name: Five-Minute Sprint Repeatability
- Milestone Target: `v0.91.4`
- Status: landed
- Planned WP Home: WP-10

## Purpose

Move the five-minute sprint from a first proof into a repeatable operating
surface. v0.91.4 must also address the newly visible validation-tail bottleneck:
a five-minute sprint is not operationally convincing if every transition still
waits on a monolithic 20-25 minute validation cycle.

## Acceptance Criteria

- More than one bounded transition records timing and coordination metrics.
- Metrics distinguish implementation time from planning, review, merge
  readiness, and closeout latency.
- Speed claims remain coupled to governance, replay, review, and merge truth.
- Repeated runs expose process bottlenecks without hiding failures.
- Metrics distinguish immediate issue-local proof from deferred or broader
  validation gates.
- Validation-tail evidence records which proof ran synchronously, which proof
  ran asynchronously, and which proof blocked continuation.
- Repeatability evidence includes a Parallel Validation Fabric plan or first
  bounded proof showing how validation shards can run without hiding pending
  or failed proof.
- Repeated runs produce tracked proof packets; when WP-10 lands on a narrower
  measurement/reporting surface instead of originating new signed-trace or
  ObsMem-derived handoff artifacts, the packet must say so explicitly and defer
  those dependencies to the milestone-level review/release evidence path.
- ObsMem and signed-trace dependencies remain visible and must not be
  overclaimed as WP-10-local outputs.

## Source Notes

- `.adl/docs/TBD/cognitive-sdlc/FIVE_MINUTE_SPRINT_TEST_CYCLE_NOTE_2026-05-22.md`
- `.adl/docs/TBD/cognitive-sdlc/C_SDLC_AND_LONG_TESTING_TAIL.md`

## Tracked Proof Surface

- `docs/milestones/v0.91.4/FIVE_MINUTE_SPRINT_REPEATABILITY_REPORT_2026-05-27.md`

## Landed v0.91.4 Position

WP-10 does not prove that all end-to-end ADL transitions complete in five
minutes. It proves a narrower and more truthful claim:

- bounded issue coordination and publication setup can be consistently fast
- validation-tail and proof-latency must be measured separately from execution
- a five-minute sprint claim is only honest when long blocking proof tails are
  called out explicitly instead of being absorbed into success language

Signed-trace and ObsMem-derived evidence remain milestone-level dependencies
that WP-10 must acknowledge but does not need to originate locally in order to
land this first repeatability packet.

The tracked report records a three-transition sample across:

- a substantive infrastructure/product-sidecar lane
- a docs/process lane
- a core C-SDLC tools lane that exposed a long validation tail

That sample is enough for v0.91.4 to move this feature from planned to landed
as a first measurement/reporting surface rather than a full signed-trace-backed
repeatability proof system.
