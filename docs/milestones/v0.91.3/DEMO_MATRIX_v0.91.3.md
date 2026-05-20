# v0.91.3 Demo Matrix

## Status

Planned demo surface.

| Demo | WP | Purpose | Expected Proof | Status |
| --- | --- | --- | --- | --- |
| Cognitive Transition manifest validation | WP-02 | Show a transition manifest links issue, cards, DAG, evidence, gate, and memory. | valid/invalid fixture results | planned |
| Card lifecycle integration | WP-03 | Show the issue bundle preserves `SIP -> STP -> SPP -> SRP -> SOR` semantics. | card validation fixture/report | planned |
| Transition DAG and shard plan | WP-04 | Show serial work, parallel shards, and barriers are explicit. | DAG fixture and rendered/reviewable summary | planned |
| Evidence bundle and review synthesis | WP-05 | Show review inputs and findings converge into a bounded packet. | evidence bundle plus synthesis output | planned |
| Governed merge-readiness gate | WP-06 | Show C-SDLC preserves issue, PR, CI, branch, and human review truth. | gate result fixture | planned |
| SRP/SOR ObsMem handoff | WP-07 | Show review results and outcome truth have a memory handoff shape. | memory handoff fixture | planned |
| Five-minute-sprint first proof | WP-09 | Show one bounded transition can execute with measurable coordination behavior. | demo report and metrics snapshot | planned |

## Demo Rules

- Demos must be fixture-backed unless live execution is explicitly approved.
- Demos must record skipped states truthfully.
- Demos must not bypass GitHub, CI, branch protection, or human review.
- Demos must distinguish speed evidence from governance evidence.
