# v0.91.7 Runtime Soak #2 Execution Packet

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Status: `ready_to_execute`
- Source issue: `#4549`
- Upstream runtime sprint authority: `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md`

## Purpose

Make Soak #2 executable as the full feature-list integration gate before
`v0.92` activation or birthday-readiness claims consume runtime planning as if
it were runtime proof.

This packet is planning and issue-setup truth only. It does not execute Soak #2
and does not claim `v0.92` readiness.

## Handoff Rule

`v0.92` remains blocked from runtime-coherence claims until every Soak #2 row is
one of:

- `integrated_proven`
- `blocked`
- `deferred`
- `routed_to_soak_3`

Planning-doc existence is never enough to close a row.

## Soak #1 Inputs Consumed

Tracked Soak #1 handoff truth already available in the repository:

- `docs/milestones/v0.91.6/review/runtime/V0916_INTEGRATED_RUNTIME_SOAK_PROOF_4245.md`
  - walking-skeleton integrated runtime packet with continuity, stop behavior,
    timeout, bulkhead/backpressure, degraded fallback, remote timeout, and one
    ObsMem handoff shape
- `docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md`
  - retained sprint-level review truth that explicitly keeps full feature-list
    runtime closure for later soak work
- `docs/milestones/v0.91.6/review/runtime/V0916_RUNTIME_AWS_SSM_HEALTH_4545.md`
  - live AWS account/profile truth plus bounded SSM and CloudWatch signal-path
    evidence for `wuji`, `nessus`, and `opticon`

Late v0.91.6 child proofs that should be folded into Soak #2 execution when
published:

- `#4546` ACIP + AEE + memory proof
- `#4547` integrated runtime failure-injection proof
- `#4548` Observatory/Unity live runtime-consumption proof when owned lane
  execution completes

If those late child proofs are still unpublished when Soak #2 starts, the Soak
#2 umbrella must classify them as explicit prerequisites or blockers rather than
silently assuming them.

## Standard Execution Posture

- Soak #2 is the default full feature-list integration gate.
- Soak #3 exists only if Soak #2 exposes blockers that cannot be truthfully
  closed in one pass.
- Runtime AWS/signal-bridge proof can run in parallel with the local runtime
  integration run, but the final gate still consumes both.

## Feature-List Integration Matrix

| Surface | Soak #1 handoff truth consumed | Soak #2 required proof mode | Default owner lane | If unresolved before `v0.92` |
| --- | --- | --- | --- | --- |
| Tokio runtime substrate | `#4245` walking-skeleton runtime boot/run/stop packet | one assembled long-running integrated runtime path | Soak #2 umbrella + standard run | block |
| Agent lifecycle | `#4245` long-lived-agent continuity and status snapshots | lifecycle transitions and negative copied/restart cases under the assembled runtime | Soak #2 matrix/setup | block or route to Soak #3 |
| AEE path | Soak #1 planning plus late `#4546` handoff when published | governed temporary-agent execution through the real AEE path | Soak #2 matrix/setup | block |
| ACIP/A2A path | `#4245` prerequisite consumption plus late `#4546` handoff when published | integrated governed send/receive plus malformed/denied/failed-delivery cases | Soak #2 matrix/setup | block |
| Provider/model substrate | `#4245` remote timeout and provider-failure classification | success/failure proof across the actual provider-routing path used by the assembled runtime | Soak #2 standard run | block or defer only with explicit operator approval |
| Scheduler | source bridge docs plus scheduler follow-on truth from v0.91.6 review corpus | local, cheap-remote, premium, delayed, and governor-like choices are explainable in live packets | Soak #2 matrix/setup | block |
| Resilience | `#4245` resilience traces plus late `#4547` handoff when published | retry, timeout, cancellation, degraded mode, and partial-failure proof under the assembled runtime | Soak #2 standard run | block or route to Soak #3 |
| Logging and observability | `#4245` action-log/heartbeat/progress contract | stdout/stderr contract, runtime action logs, heartbeat, timeout, and redaction remain truthful under the full run | Soak #2 standard run | block |
| Runtime AWS and signal bridge | `#4545` AWS profile, SSM, and CloudWatch truth | heartbeat publisher, ACIP-to-SNS, and runtime AWS signal route are integrated or explicitly blocked | parallel AWS/signal lane | block or defer with named owner issue |
| Observatory / Unity live consumption | `#4245` prerequisite consumption only; late `#4548` handoff when published | live runtime state is consumed, not just canned demo data | Observatory lane tied to Soak #2 closeout | block or route to Soak #3 |
| ObsMem and memory handoff | `#4245` transition-memory request shape; late `#4546` handoff when published | checkpoint or handoff survives the long-running context boundary with redaction-safe evidence | Soak #2 standard run | block |
| Identity and continuity | `#4245` continuity snapshots and lease-overlap boundary | startup, wake, copied state, stop, and true continuity cases are distinguishable | Soak #2 matrix/setup | block |
| Capability envelope | feature-list and handoff docs only | capability/authority limits are explicit for the assembled runtime and temporary agents | Soak #2 matrix/setup | routed with named follow-on if not required for the standard run |
| Security / CAV boundary | residual security docs and v0.91.6 review corpus | malformed output, unauthorized access, and trust-boundary failures fail closed under integrated conditions | security/runtime cross-lane review | block |
| Curiosity / Constructability, if promoted before run | v0.91.7 bridge docs only | one governed discovery/admissibility cycle if those surfaces land in time; otherwise explicit deferral | explicit optional Soak #2 row | deferred or routed |

## Soak #2 Wave Ready For Execution

1. Soak #2 umbrella and execution packet
   - this packet plus issue `#4549`
2. Feature-list integration matrix implementation and fixture/setup packet
   - produce or update the issue-local proof matrix for every row above
3. Standard Soak #2 run
   - assembled runtime proof plus bounded negative cases
4. Soak #2 review and blocker register
   - classify every row as `integrated_proven`, `blocked`, `deferred`, or
     `routed_to_soak_3`
5. Soak #3 remediation run, only if needed
6. Final pre-`v0.92` runtime-coherence disposition

## Blocker / Defer Policy

- `blocked`
  - the row is required for `v0.92` and lacks integrated proof
- `deferred`
  - the row is explicitly outside `v0.92` activation scope and has approved
    non-claim language
- `routed_to_soak_3`
  - the row is required, Soak #2 found a concrete blocker, and one more bounded
    pass is justified
- `integrated_proven`
  - the row has assembled-runtime proof, not just component or docs-only proof

Rows may not exit as `assumed`, `implied`, or `planning_complete`.

## Validation For This Packet

This issue should use focused docs/planning checks only:

- `git diff --check`
- bounded planning-doc review for cross-link truth and overclaim avoidance
- truthful `SRP` / `SOR` updates once this packet and linked milestone docs are
  updated

## Non-Goals

- Do not execute Soak #2 inside `v0.91.6`.
- Do not claim `v0.92` activation readiness from this packet.
- Do not silently convert unpublished late v0.91.6 child proofs into assumed
  Soak #2 prerequisites.
