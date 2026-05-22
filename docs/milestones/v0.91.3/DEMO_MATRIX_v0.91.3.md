# v0.91.3 Demo Matrix

## Status

Active demo surface for the completed `v0.91.3` first slice and its demo wave.
Each row should point reviewers at a real front-stage artifact, executable demo
command, validator-backed packet, or explicit packet-first proof path.

| Demo | WP | Purpose | Primary Run Path | Expected Proof | Status |
| --- | --- | --- | --- | --- | --- |
| C-SDLC demo proof contract | demo WP-01 / #3220 | Define the shared claim, timebox, validation, and review contract for later demo packets. | `python3 adl/tools/validate_csdlc_demo_proof_contract_packet.py docs/milestones/v0.91.3/review/csdlc_demo_proof_contract` | `review/csdlc_demo_proof_contract/` packet, reusable packet template, and validator-backed packet proof | proven |
| Cognitive Transition manifest validation | WP-02 / #3200 | Show a transition manifest links issue, actor roles, cards, DAG, evidence, gate, and memory. | `cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture` | validator-backed valid/invalid fixture test results | proven |
| Card lifecycle integration | WP-03 / #3201 | Show a tracked public issue bundle preserves `SIP -> STP -> SPP -> SRP -> SOR` semantics. | `adl/tools/pr.sh doctor 3201 --version v0.91.3 --json` | tracked card bundle plus validator/doctor proof | proven |
| Transition DAG and shard plan | WP-04 / #3202 | Show serial work, parallel shards, and barriers are explicit. | `python3 adl/tools/validate_transition_dag_packet.py docs/milestones/v0.91.3/review/transition_dag` | DAG packet, shard plan, and validator-backed summary | proven |
| Evidence bundle and review synthesis | WP-05 / #3203 | Show review inputs and findings converge into a bounded packet. | `python3 adl/tools/validate_evidence_bundle_packet.py docs/milestones/v0.91.3/review/evidence_bundle` | evidence bundle plus synthesis output and validator-backed packet proof | proven |
| Governed merge-readiness gate | WP-06 / #3204 | Show C-SDLC preserves issue, PR, CI, branch, and human review truth. | `python3 adl/tools/validate_merge_readiness_packet.py docs/milestones/v0.91.3/review/merge_readiness` | gate result fixture and validator-backed packet proof | proven |
| SRP/SOR ObsMem handoff | WP-07 / #3205 | Show review results and outcome truth have a memory handoff shape. | `python3 adl/tools/validate_obsmem_handoff_packet.py docs/milestones/v0.91.3/review/obsmem_handoff` | tracked handoff record plus validator-backed packet proof | proven |
| Integrated process lessons and proof readiness | WP-08 / #3206 | Show the upstream proof chain is ready for the first bounded proof run. | `python3 adl/tools/validate_first_proof_readiness_packet.py docs/milestones/v0.91.3/review/first_proof_readiness` | readiness packet plus combined-lane validator/test proof | proven |
| Five-minute-sprint first proof | WP-09 / #3207 | Show one bounded transition can execute with measurable coordination behavior. | `python3 adl/tools/demo_v0913_first_proof_demo.py --timeline docs/milestones/v0.91.3/review/first_proof_demo/ct_demo_001_timeline_snapshot.json --out docs/milestones/v0.91.3/review/first_proof_demo` | `review/first_proof_demo/` packet, deterministic demo command, and metrics snapshot that separately classify process success versus literal five-minute success | proven |
| Five-minute HTML game sprint demo | demo WP-02 / #3221 | Build the first visible C-SDLC creative-production demo as a playable browser game with explicit proof notes. | `bash adl/tools/demo_v0913_starharvest_html_game.sh` | `demos/v0.91.3/starharvest_five_minute_sprint_demo.html`, `review/five_minute_html_game/` packet, and validator-backed packet proof | proven |
| Five-minute sprint console demo | demo WP-03 / #3222 | Make the Starharvest mini-sprint legible as a governed process with roles, work states, review events, and launch truth on one visual surface. | `bash adl/tools/demo_v0913_five_minute_sprint_console.sh` | `demos/v0.91.3/five_minute_sprint_console_demo.html`, `review/five_minute_sprint_console/` packet, and validator-backed packet proof | proven |
| Podcast Studio v2 demo | demo WP-04 / #3223 | Turn the older podcast pilot into a deterministic recurring production-system packet with inspectable roles, transcript, and truthful audio status. | `bash adl/tools/demo_v0913_podcast_studio_v2.sh` | `demos/v0.91.3/adl_podcast_studio_v2_episode_card.html`, `review/podcast_studio_v2/` packet, and validator-backed packet proof | proven |
| C-SDLC demo showcase package | demo WP-05 / #3224 | Package the mini-sprint into one reviewer-facing order-of-operations, claims, and artifact index surface. | `cat docs/milestones/v0.91.3/review/csdlc_demo_showcase/ct_demo_005_demo_index.md` | `review/csdlc_demo_showcase/` packet plus status/claims index for the merged demo wave | proven |

## Reviewer Shortcut

For a single pass that answers “what is the best demo for each feature?” start
with:

- `docs/milestones/v0.91.3/review/demo_coverage/DEMO_COVERAGE_PACKET_v0.91.3.md`
- `docs/milestones/v0.91.3/review/demo_coverage/ct_demo_006_feature_demo_map.md`

## Demo Rules

- Demos must be fixture-backed unless live execution is explicitly approved.
- Demos must record skipped states truthfully.
- Demos must use the shared C-SDLC demo proof contract before claiming success.
- Demos must not bypass GitHub, CI, branch protection, or human review.
- Demos must distinguish speed evidence from governance evidence.
