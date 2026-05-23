## Metadata

- Skill: repo-architecture-review
- Target: agent-design-language repository, WP-13 internal review lane for v0.91.3 C-SDLC first-slice architecture/process surfaces
- Date: 2026-05-23
- Artifact: .adl/reviews/v0.91.3/internal/wp-13/codebuddy/specialist_reviews/architecture_review.md
- Packet: .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet
- Role: architecture
- Validation mode: inspect_only

## Retained Correction Note

An earlier draft of this lane incorrectly reported a stale `v0.91.2` crate
version finding against `adl/Cargo.toml`. That claim was invalid at the
reviewed baseline and is preserved only as review-process error evidence in
`.adl/reviews/v0.91.3/internal/wp-13/REVIEW_LANE_ERRORS.md` (`RLE-001`). It is
not a product finding and must not be synthesized into WP-15 remediation
records.

## Findings

### P1: CodeBuddy repo packet under-captures the milestone proof surfaces needed for this architecture lane

- File: .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/specialist_assignments.json
- Role: architecture
- Scenario: A downstream synthesis reviewer trusts the architecture assignment and evidence index as the bounded source set for C-SDLC state-model review.
- Architecture boundary or layer: review packet scope boundary between WP-13 milestone/process evidence and specialist lane inputs.
- Impact: The architecture lane can miss the very artifacts that define the v0.91.3 C-SDLC state model, transition DAG, ObsMem handoff, and proof boundaries. That creates a credible path to false confidence in WP-13 synthesis or WP-14 handoff, because the packet presents a whole-repository review shape while omitting central milestone proof surfaces from the architecture assignment.
- Evidence: The tracked WP-13 scope docs in docs/milestones/v0.91.3/review/internal_review/REVIEW_PACKET.md and docs/milestones/v0.91.3/review/internal_review/WP13_INTERNAL_REVIEW_PLAN.md define the architecture lane as whole-repository review with emphasis on the v0.91.3 C-SDLC milestone docs, proof packets, lifecycle records, and review handoff truth. The retained generated architecture assignment under .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/specialist_assignments.json includes architecture docs and selected CLI/runtime files, but not the key milestone C-SDLC proof paths such as docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md, docs/milestones/v0.91.3/features/TRANSITION_DAG_AND_SHARD_COORDINATION.md, docs/milestones/v0.91.3/features/SRP_SOR_OBSMEM_HANDOFF.md, docs/milestones/v0.91.3/review/transition_dag/, docs/milestones/v0.91.3/review/obsmem_handoff/, or workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/.
- Recommended follow-up owner: repo-packet-builder / WP-13 review packet owner.

### P1: Canonical architecture narrative and diagrams still describe the pre-C-SDLC three-card lifecycle

- File: docs/architecture/ADL_ARCHITECTURE.md
- Role: architecture
- Scenario: An operator, reviewer, or future agent follows the canonical architecture package rather than the newer v0.91.3 feature docs and doctor classifier.
- Architecture boundary or layer: card lifecycle boundary between issue intent, task selection, operative planning, review result truth, and outcome truth.
- Impact: The canonical architecture package can route humans or agents around SPP and SRP, collapsing issue-local operative plan truth and review-result truth back into STP/SIP/SOR. That undercuts the v0.91.3 claim that the first slice operationalizes SIP -> STP -> SPP -> SRP -> SOR and makes stale diagrams a real process hazard rather than only documentation drift.
- Evidence: AGENTS.md and docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md declare SIP -> STP -> SPP -> SRP -> SOR as canonical. adl/src/cli/pr_cmd/doctor.rs builds the doctor lifecycle in that exact order and blocks pr run unless SIP, STP, SPP, and SRP are design-time complete. By contrast, docs/architecture/ADL_ARCHITECTURE.md still describes the authoring/control-plane packet as STP, SIP, and SOR only, and its task bundle lifecycle says STP/SIP/SOR cards are created. docs/architecture/diagrams/task_bundle_state.mmd labels BundleInitialized as STP SIP SOR created, and docs/architecture/diagrams/DIAGRAM_PACKET.md describes that diagram as showing STP/SIP/SOR bundle transitions.
- Recommended follow-up owner: architecture docs owner with docs-review and diagram-plan support.

### P2: Tracked C-SDLC card proof and local-only issue-mode tooling have no explicit migration boundary

- File: adl/src/cli/pr_cmd/finish_support.rs
- Role: architecture
- Scenario: v0.91.4 or a later milestone attempts to make workflow/c-sdlc/ the default durable card home while the existing pr finish/closeout path still treats canonical issue surfaces as local-only .adl state.
- Architecture boundary or layer: durable workflow-state ownership between local execution cache, ignored .adl cards, tracked workflow/c-sdlc proof records, and PR publication.
- Impact: Operators can end up with two plausible sources of card truth: tracked workflow/c-sdlc records for C-SDLC proof and local-only .adl task bundles for issue-mode execution. Without an explicit migration boundary, future work can either accidentally publish local execution records that were supposed to remain ephemeral or continue leaving durable governance truth on one machine while claiming tracked C-SDLC operation.
- Evidence: docs/architecture/adr/0028-c-sdlc-tracked-workflow-state-and-signed-trace.md says durable C-SDLC records should move to tracked Git state, while local .adl state should shrink toward cache and scratch. docs/milestones/v0.91.3/C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md defines workflow/c-sdlc/v0.91.3/ as the provisional durable namespace and says local .adl must not be the only authoritative home once a record is part of the public proof packet. docs/milestones/v0.91.3/review/card_lifecycle_integration/CARD_LIFECYCLE_PROOF_PACKET_v0.91.3.md proves a tracked public bundle but explicitly does not claim default-operation rollout. Meanwhile adl/src/cli/pr_cmd/finish_support.rs validates SPP/SRP/SOR from issue task-bundle paths and calls ensure_issue_surfaces_are_local_only, and adl/src/cli/pr_cmd/lifecycle/reconciliation.rs enforces canonical .adl output surfaces as local-only during closed-issue reconciliation.
- Recommended follow-up owner: workflow control-plane owner with ADR curator and architecture-fitness-function author.

### P2: Transition DAG and shard boundaries are reviewable documents, not yet enforceable process state

- File: docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md
- Role: architecture
- Scenario: Multiple agents execute a future C-SDLC transition in parallel and rely on the shard plan to prevent overlapping writes or hidden coordination.
- Architecture boundary or layer: transition DAG/shard coordination boundary between declared ownership, allowed write surfaces, synchronization barriers, and actual repository changes.
- Impact: The current packet is a strong first proof of vocabulary, but it does not yet prevent shard collision or out-of-scope writes before review. If treated as operational enforcement, the process can discover violations only after the fact, at review or merge-readiness time.
- Evidence: docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_transition_dag.md declares serial nodes, shard nodes, barrier nodes, and interface-freeze rules. docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md assigns allowed and forbidden write surfaces. docs/milestones/v0.91.3/review/transition_dag/TRANSITION_DAG_PROOF_PACKET_v0.91.3.md explicitly says the packet does not claim live multi-agent parallel execution or already-measured timing. adl/tools/validate_transition_dag_packet.py validates the presence of required files and snippets, but does not compare a PR or SOR changed-path list against shard ownership or barrier state.
- Recommended follow-up owner: architecture-fitness-function author with transition-DAG tooling owner.

### P2: ObsMem handoff summarizes final SRP/SOR truth without anchoring the exact final card sources

- File: docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json
- Role: architecture
- Scenario: A later ObsMem ingestion job imports the v0.91.3 handoff and needs to reconstruct the exact final SRP review truth and SOR outcome truth that produced the memory candidates.
- Architecture boundary or layer: evidence/ObsMem handoff boundary between final review/output cards, tracked supporting artifacts, summarized memory entries, and future signed trace ingestion.
- Impact: The handoff can preserve useful memory candidates while losing precise card provenance. That weakens replay and auditability because the memory record says it is derived from final SRP/SOR truth, but its canonical citations point to supporting review/evidence artifacts rather than exact final card paths, revisions, or digests.
- Evidence: docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.md says the handoff is derived from final SRP and SOR truth and that local .adl card files remain derivation inputs only. docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json records srp_memory_entry.source_truth as derived_from_final_srp and sor_memory_entry.source_truth as derived_from_final_sor, but the citations and tracked_supporting_artifacts are evidence bundle, review synthesis, and merge-readiness gate paths. adl/tools/validate_obsmem_handoff_packet.py requires citations to be repo-relative and not .adl paths, but it does not require a tracked final SRP/SOR path, revision, digest, or signed trace reference.
- Recommended follow-up owner: ObsMem handoff owner with ADR curator and fitness-function author.

## Assumptions And Limits

- This was an architecture/process review only, not a code correctness, security, dependency, or test-quality review.
- I treated docs/milestones/v0.91.3/review/internal_review/REVIEW_PACKET.md and WP13_INTERNAL_REVIEW_PLAN.md as the controlling tracked WP-13 scope docs, and treated .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/ as the retained generated packet surface under review.
- I did not run validation commands, GitHub queries, cargo tests, or packet validators. Findings are inspect-only and source-grounded in tracked files available in the worktree.
- I did not inspect every file in the 2,930-file repository inventory. I focused on C-SDLC state model, card lifecycle, issue/worktree/review state, transition DAG/shards, evidence/ObsMem handoff, and architecture drift.

## Architecture Map

- Top-level runtime and control-plane split: docs/architecture/ADL_ARCHITECTURE.md describes ADL as a repository-first agent orchestration system with a runtime layer, authoring/control-plane layer, workflow-skill layer, trace/artifact truth, and review/release surfaces.
- Card lifecycle/control plane: AGENTS.md, docs/architecture/adr/0024-workflow-guardrails-issue-lifecycle-control-plane.md, docs/architecture/adr/0028-c-sdlc-tracked-workflow-state-and-signed-trace.md, adl/src/cli/pr_cmd/doctor.rs, adl/src/cli/pr_cmd_cards/cards.rs, adl/src/cli/pr_cmd/finish_support.rs, and adl/src/cli/pr_cmd/lifecycle/reconciliation.rs define the active issue-card, worktree, PR, and closeout truth boundaries.
- C-SDLC first-slice proof layer: docs/milestones/v0.91.3/C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md plus docs/milestones/v0.91.3/features/ define the milestone state model for tracked cards, transition DAG/shards, evidence bundles, merge readiness, review/output truth, and ObsMem handoff.
- Transition DAG/shard layer: docs/milestones/v0.91.3/review/transition_dag/ defines the first serial/shard/barrier vocabulary and validator-backed packet proof.
- Evidence and memory handoff layer: docs/milestones/v0.91.3/review/evidence_bundle/ and docs/milestones/v0.91.3/review/obsmem_handoff/ define the handoff from review/evidence packets into memory candidates, with live ObsMem and signed trace deferred.
- Review packet layer: docs/milestones/v0.91.3/review/internal_review/ plus .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/ define WP-13 review orchestration, packet scope, specialist assignments, and residual packet limits.

## Reviewed Surfaces

- AGENTS.md
- docs/architecture/ADL_ARCHITECTURE.md
- docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md
- docs/architecture/ARCHITECTURE_REVIEW_AUTOMATION.md
- docs/architecture/adr/0024-workflow-guardrails-issue-lifecycle-control-plane.md
- docs/architecture/adr/0028-c-sdlc-tracked-workflow-state-and-signed-trace.md
- docs/architecture/diagrams/task_bundle_state.mmd
- docs/architecture/diagrams/task_bundle_and_pr_lifecycle.mmd
- docs/architecture/diagrams/DIAGRAM_PACKET.md
- docs/milestones/v0.91.3/C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md
- docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md
- docs/milestones/v0.91.3/features/TRANSITION_DAG_AND_SHARD_COORDINATION.md
- docs/milestones/v0.91.3/features/SRP_SOR_OBSMEM_HANDOFF.md
- docs/milestones/v0.91.3/review/card_lifecycle_integration/CARD_LIFECYCLE_PROOF_PACKET_v0.91.3.md
- docs/milestones/v0.91.3/review/transition_dag/TRANSITION_DAG_PROOF_PACKET_v0.91.3.md
- docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_transition_dag.md
- docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md
- docs/milestones/v0.91.3/review/evidence_bundle/EVIDENCE_BUNDLE_PROOF_PACKET_v0.91.3.md
- docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_review_synthesis.md
- docs/milestones/v0.91.3/review/obsmem_handoff/OBSMEM_HANDOFF_PROOF_PACKET_v0.91.3.md
- docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.md
- docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json
- docs/milestones/v0.91.3/review/internal_review/REVIEW_PACKET.md
- docs/milestones/v0.91.3/review/internal_review/WP13_INTERNAL_REVIEW_PLAN.md
- docs/milestones/v0.91.3/review/internal_review/READINESS_GATE.md
- .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/repo_scope.md
- .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/evidence_index.json
- .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/repo_inventory.json
- .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/specialist_assignments.json
- .adl/reviews/v0.91.3/internal/wp-13/codebuddy/repo-packet/run_manifest.json
- workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/
- adl/src/cli/pr_cmd/doctor.rs
- adl/src/cli/pr_cmd_cards/cards.rs
- adl/src/cli/pr_cmd/finish_support.rs
- adl/src/cli/pr_cmd/lifecycle/reconciliation.rs
- adl/src/cli/pr_cmd/lifecycle/transitions.rs
- adl/tools/validate_transition_dag_packet.py
- adl/tools/validate_obsmem_handoff_packet.py

## Candidate Diagram Tasks

- Update docs/architecture/diagrams/task_bundle_state.mmd to show SIP -> STP -> SPP -> SRP -> SOR, including design-time readiness, final review truth, final SOR truth, and closeout reconciliation.
- Add or update a C-SDLC transition diagram that connects transition manifest, DAG, shard plan, evidence bundle, SRP, SOR, merge-readiness gate, and ObsMem handoff.
- Add a review-packet architecture diagram showing WP-13 packet scope, generated repo-packet scope, specialist assignments, milestone proof packets, and synthesis handoff so packet-undercoverage is visible.

## Candidate ADRs

- Accept or update ADR 0024 to reflect the current five-card lifecycle and distinguish architecture policy from older STP/SIP/SOR diagrams.
- Promote ADR 0028 or create its successor for the tracked workflow/c-sdlc namespace, explicitly defining the migration boundary between local .adl execution cache and tracked durable C-SDLC truth.
- Create an ADR for transition DAG/shard enforcement that decides whether shard boundaries remain review-only, become PR changed-path fitness checks, or become lifecycle-gated execution state.
- Create an ADR for SRP/SOR ObsMem source provenance requiring exact final card source references, revision/digest fields, and signed-trace linkage before memory entries become durable governance knowledge.
- Create an ADR or packet-builder design note for CodeBuddy review packet scope selection when milestone proof packets are the actual architecture under review.

## Candidate Fitness Functions

- Packet scope fitness: fail WP-13 architecture packet generation if docs/milestones/v0.91.3/features/, docs/milestones/v0.91.3/review/transition_dag/, docs/milestones/v0.91.3/review/obsmem_handoff/, and workflow/c-sdlc/v0.91.3/ are absent from architecture evidence or explicitly excluded with rationale.
- Lifecycle vocabulary fitness: fail architecture docs/diagrams that describe the current control-plane lifecycle as only STP/SIP/SOR without an explicit legacy or v0.90 qualifier.
- Card-state migration fitness: fail publication or closeout paths that claim durable C-SDLC truth without either tracked workflow/c-sdlc records or a documented local-only exception.
- Shard boundary fitness: compare changed paths from SOR/PR metadata against the transition shard plan's allowed write surfaces and require an explicit replan for out-of-shard writes.
- ObsMem provenance fitness: require srp_memory_entry and sor_memory_entry to cite exact final SRP/SOR source records or digests, in addition to supporting evidence/review/merge artifacts.
- Signed trace readiness fitness: require C-SDLC memory handoff records to carry a deferred signed-trace field with explicit status until signed trace proof lands.

## Validation Performed

- Inspect-only review of tracked files and generated packet metadata.
- No tests, validators, cargo commands, GitHub commands, or browser checks were run.
- No runtime behavior, live GitHub issue/PR state, or untracked local .adl cards were validated.

## Residual Architecture Risk

- GitHub issue/PR truth for WP-01 through WP-13 was not queried live, so issue closure and PR merge claims in milestone artifacts were not independently verified by this lane.
- The repo-packet evidence index is metadata-heavy and does not include source excerpts; this review supplemented it with targeted source inspection, but did not rebuild the packet.
- I did not inspect every architecture-assigned Rust file, so additional coupling may exist in runtime, provider, or trace code outside the C-SDLC/process focus.
- Because validation was not run, validator behavior is inferred from source inspection, not execution output.
- The tracked workflow/c-sdlc proof namespace appears intentionally first-slice only; this review treats default-operation rollout as a follow-up architecture boundary, not as a v0.91.3 delivery failure.
