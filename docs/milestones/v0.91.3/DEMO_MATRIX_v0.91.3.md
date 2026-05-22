# v0.91.3 Demo Matrix

## Status

Active demo surface. Demo rows remain `planned` until their owning work
packages execute.

| Demo | WP | Purpose | Expected Proof | Status |
| --- | --- | --- | --- | --- |
| Cognitive Transition manifest validation | WP-02 / #3200 | Show a transition manifest links issue, actor roles, cards, DAG, evidence, gate, and memory. | validator-backed valid/invalid fixture test results | in_flight |
| Card lifecycle integration | WP-03 / #3201 | Show a tracked public issue bundle preserves `SIP -> STP -> SPP -> SRP -> SOR` semantics. | tracked card bundle plus validator/doctor proof | in_flight |
| Transition DAG and shard plan | WP-04 / #3202 | Show serial work, parallel shards, and barriers are explicit. | DAG fixture and rendered/reviewable summary | planned |
| Evidence bundle and review synthesis | WP-05 / #3203 | Show review inputs and findings converge into a bounded packet. | evidence bundle plus synthesis output | planned |
| Governed merge-readiness gate | WP-06 / #3204 | Show C-SDLC preserves issue, PR, CI, branch, and human review truth. | gate result fixture | planned |
| SRP/SOR ObsMem handoff | WP-07 / #3205 | Show review results and outcome truth have a memory handoff shape. | memory handoff fixture | planned |
| Five-minute-sprint first proof | WP-09 / #3207 | Show one bounded transition can execute with measurable coordination behavior. | planned demo report and metrics snapshot | planned |

## Demo Rules

- Demos must be fixture-backed unless live execution is explicitly approved.
- Demos must record skipped states truthfully.
- Demos must not bypass GitHub, CI, branch protection, or human review.
- Demos must distinguish speed evidence from governance evidence.
