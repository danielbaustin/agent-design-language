# v0.91.4 Work Breakdown Structure

## Status

Planned WBS. Work package issue numbers remain `pending` until seeded through
the v0.91.4 issue wave.

## WBS Summary

v0.91.4 hardens the v0.91.3 C-SDLC vertical slice into repeatable operation.
The goal is not more theory; it is a dependable development control plane.

## Candidate WP Sequence

| WP | Title | Queue | Primary Deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Design pass and issue-wave readiness | docs | v0.91.4 issue wave, sprint umbrellas, validated cards | v0.91.3 closeout |
| WP-02 | Lifecycle validator hardening | tools | strict card and transition lifecycle validators | WP-01 |
| WP-03 | Doctor and conductor state truth | tools | `pr doctor`, workflow-conductor, and sprint-conductor aligned on C-SDLC stages | WP-02 |
| WP-04 | Editor skill repair reliability | tools | card editor skills repair drift without hand edits | WP-02 |
| WP-05 | Shard ownership and interface freeze | tools/docs | enforceable shard write boundaries and synchronization rules | WP-03 |
| WP-06 | Evidence convergence, review synthesis, and signed trace proof | tools | repeatable evidence bundle, review synthesis flow, and minimal signed trace bundle | WP-05 |
| WP-07 | Merge-readiness and PR gate hardening | tools | transition-aware merge gate preserving GitHub truth | WP-06 |
| WP-08 | ObsMem transition memory integration | tools/docs | SRP/SOR/signed-trace memory handoff write/read contract | WP-06, WP-07 |
| WP-09 | Sprint conductor default C-SDLC lane | tools | sprint flow that cannot skip child closeout or umbrella truth | WP-03, WP-04 |
| WP-10 | Five-minute-sprint repeatability metrics | demo/tools | repeated transition metrics and coordination-latency report | WP-05 through WP-09 |
| WP-11 | Active issue migration policy | docs/tools | policy for existing open cards and future issue defaults | WP-02 through WP-09 |
| WP-12 | Regression fixtures for process drift | tests/tools | fixtures for SRP drift, stale SORs, skipped closeout, and env/global-state hazards | WP-10, WP-11 |
| WP-13 | Internal review | review | code/docs/tests/process review packet | WP-12 |
| WP-14 | Review findings remediation | review | fixes and follow-on routing | WP-13 |
| WP-15 | Docs + adoption pass | docs | C-SDLC default-operation docs, tracked-source docs, and onboarding updates | WP-14 |
| WP-16 | Release ceremony | release | evidence package, signed-trace proof, and closeout record | WP-15 |

## Sequencing Notes

Validator and conductor truth must land before repeatability metrics. Otherwise
the milestone risks timing an unreliable process and calling that a proof.

The sprint-conductor lane should explicitly consume the v0.91.2 and v0.91.3
lessons:

- no next child before closeout truth
- no umbrella closeout without sprint review and state artifact truth
- no isolated-test pass when combined-lane validation is the risk
- no SRP/SOR memory handoff over stale card semantics
- no durable C-SDLC proof without tracked signed trace evidence
- no default operation while durable workflow records still live only in local
  `.adl` state
