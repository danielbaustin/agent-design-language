# v0.91.3 Feature Demo Map

This is the quickest truthful answer to:

> What should I run or open for each `v0.91.3` feature?

## Strongest Front-Stage Demos

| Feature | Best reviewer path | Why it matters |
| --- | --- | --- |
| Five-Minute HTML Game Sprint Demo | `bash adl/tools/demo_v0913_starharvest_html_game.sh` | The most visible proof that the C-SDLC can yield a real browser artifact. |
| Five-Minute Sprint Console Demo | `bash adl/tools/demo_v0913_five_minute_sprint_console.sh` | Makes the governed process legible instead of hiding it behind PR history. |
| Podcast Studio v2 Demo | `bash adl/tools/demo_v0913_podcast_studio_v2.sh` | Shows a richer recurring production-system packet with inspectable roles and outputs. |

## Strongest Executable Proof Demos

| Feature | Best reviewer path | Why it matters |
| --- | --- | --- |
| Five-Minute Sprint First Proof | `python3 adl/tools/demo_v0913_first_proof_demo.py --timeline docs/milestones/v0.91.3/review/first_proof_demo/ct_demo_001_timeline_snapshot.json --out docs/milestones/v0.91.3/review/first_proof_demo` | The clearest executable proof that the bounded C-SDLC transition chain works, while still recording that the literal five-minute target remains non-proving. |
| C-SDLC Demo Proof Contract | `python3 adl/tools/validate_csdlc_demo_proof_contract_packet.py docs/milestones/v0.91.3/review/csdlc_demo_proof_contract` | Establishes the shared claims/non-claims and proof discipline that keep later demos honest. |

## Packet-First Proof Features

| Feature | Best reviewer path | Why it matters |
| --- | --- | --- |
| Cognitive Transition Manifest | `cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture` | Best proof is schema and fixture truth, not a front-stage UI. |
| Card Lifecycle Integration | `adl/tools/pr.sh doctor 3201 --version v0.91.3 --json` | Best proof is preserved lifecycle semantics and doctor-visible bundle truth. |
| Transition DAG And Shard Coordination | `python3 adl/tools/validate_transition_dag_packet.py docs/milestones/v0.91.3/review/transition_dag` | Best proof is explicit DAG/shard/barrier structure. |
| Evidence Bundle And Review Synthesis | `python3 adl/tools/validate_evidence_bundle_packet.py docs/milestones/v0.91.3/review/evidence_bundle` | Best proof is converged packet/review structure. |
| Governed Merge-Readiness Gate | `python3 adl/tools/validate_merge_readiness_packet.py docs/milestones/v0.91.3/review/merge_readiness` | Best proof is governance shape and truth preservation. |
| SRP/SOR ObsMem Handoff | `python3 adl/tools/validate_obsmem_handoff_packet.py docs/milestones/v0.91.3/review/obsmem_handoff` | Best proof is the memory handoff contract, not a UI surface. |
| Integrated Process Lessons And Proof Readiness | `python3 adl/tools/validate_first_proof_readiness_packet.py docs/milestones/v0.91.3/review/first_proof_readiness` | Best proof is readiness convergence before the first proof run. |

## Showcase / Index Surface

| Feature | Best reviewer path | Why it matters |
| --- | --- | --- |
| C-SDLC Demo Showcase | `cat docs/milestones/v0.91.3/review/csdlc_demo_showcase/ct_demo_005_demo_index.md` | One reviewer-facing path through the demo wave, with explicit claims and non-claims. |
| Cognitive SDLC First Slice | `cat docs/milestones/v0.91.3/review/demo_coverage/ct_demo_006_feature_demo_map.md` then follow the feature-specific path above | The milestone itself is best reviewed as a composed proof stack, not as one single artifact. |

## Coverage Verdict

- Strong front-stage demo coverage: present
- Strong executable proof coverage: present
- Packet-first proof coverage: present
- Missing feature with no truthful demo/proof route: none found in the current `v0.91.3` feature set

## Important Non-Claim

This map does **not** claim that every feature is equally impressive to a general audience.
It claims something narrower and more important for Sprint 4 review:

- every current `v0.91.3` feature has a reviewer-usable demo or proof route
- the strongest paths are now easy to find
