# v0.91.3 Demo Matrix

## Status

Active demo surface. Demo rows remain `planned` until their owning work
packages execute.

| Demo | WP | Purpose | Expected Proof | Status |
| --- | --- | --- | --- | --- |
| C-SDLC demo proof contract | demo WP-01 / #3220 | Define the shared claim, timebox, validation, and review contract for later demo packets. | `review/csdlc_demo_proof_contract/` packet, reusable packet template, and validator-backed packet proof | proven |
| Cognitive Transition manifest validation | WP-02 / #3200 | Show a transition manifest links issue, actor roles, cards, DAG, evidence, gate, and memory. | validator-backed valid/invalid fixture test results | proven |
| Card lifecycle integration | WP-03 / #3201 | Show a tracked public issue bundle preserves `SIP -> STP -> SPP -> SRP -> SOR` semantics. | tracked card bundle plus validator/doctor proof | proven |
| Transition DAG and shard plan | WP-04 / #3202 | Show serial work, parallel shards, and barriers are explicit. | DAG packet, shard plan, and validator-backed summary | proven |
| Evidence bundle and review synthesis | WP-05 / #3203 | Show review inputs and findings converge into a bounded packet. | evidence bundle plus synthesis output and validator-backed packet proof | proven |
| Governed merge-readiness gate | WP-06 / #3204 | Show C-SDLC preserves issue, PR, CI, branch, and human review truth. | gate result fixture and validator-backed packet proof | proven |
| SRP/SOR ObsMem handoff | WP-07 / #3205 | Show review results and outcome truth have a memory handoff shape. | tracked handoff record plus validator-backed packet proof | proven |
| Integrated process lessons and proof readiness | WP-08 / #3206 | Show the upstream proof chain is ready for the first bounded proof run. | readiness packet plus combined-lane validator/test proof | proven |
| Five-minute-sprint first proof | WP-09 / #3207 | Show one bounded transition can execute with measurable coordination behavior. | `review/first_proof_demo/` packet, deterministic demo command, and metrics snapshot that separately classify process success versus literal five-minute success | in_flight |
| Five-minute HTML game sprint demo | demo WP-02 / #3221 | Build the first visible C-SDLC creative-production demo as a playable browser game with explicit proof notes. | `demos/v0.91.3/starharvest_five_minute_sprint_demo.html`, `review/five_minute_html_game/` packet, and validator-backed packet proof | in_flight |

## Demo Rules

- Demos must be fixture-backed unless live execution is explicitly approved.
- Demos must record skipped states truthfully.
- Demos must use the shared C-SDLC demo proof contract before claiming success.
- Demos must not bypass GitHub, CI, branch protection, or human review.
- Demos must distinguish speed evidence from governance evidence.
