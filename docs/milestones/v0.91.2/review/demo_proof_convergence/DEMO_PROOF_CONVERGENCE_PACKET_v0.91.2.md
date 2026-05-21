# Demo / Proof Convergence Packet - v0.91.2

## Status

Tracked `WP-17` convergence packet for Sprint 4 release-tail review.

This packet does not create new implementation proof. It consolidates the
strongest truthful demo or proof route for each landed `WP-02` through `WP-16`
slice so reviewers can quickly see what the milestone can actually show.

## Purpose

The milestone accumulated real proof surfaces, but they are spread across
benchmark reports, JSON review artifacts, bounded demo packets, docs/product
packets, and operator runbooks.

`WP-17` exists to make that body of evidence legible:

- which slices have a strong live or fixture-backed demo
- which slices have a strong packetized proof path
- which slices should be showcased as flagship demos
- which slices still rely on bounded packet truth rather than a fresh live
  demonstration

## Showcase Classes

| Class | Meaning | Expected use |
| --- | --- | --- |
| Flagship demo | One of the milestone's strongest reviewer-facing proofs with direct operational or executable evidence. | Use in review, release evidence, and external explanation of what `v0.91.2` actually accomplished. |
| Strong bounded demo | A clear fixture-backed or bounded demo path with retained evidence. | Use when a reviewer needs believable proof but not necessarily the flagship story. |
| Packet-first proof | A docs/product/review packet is the truthful proving surface. | Use for publication, productization, and planning slices where a new live demo would overclaim. |
| Explicit deferral | The work landed, but the next stronger demo should happen in a follow-on rather than being faked now. | Keep the milestone honest about what is and is not already demonstrated. |

## Converged Feature Map

| WP | Feature Slice | Strongest current route | Showcase class | Why it is compelling now | Explicit boundary or deferral |
| --- | --- | --- | --- | --- | --- |
| WP-02 / WP-03 | UTS + ACC benchmark and provider-native comparison | `review/uts_acc_multi_model_benchmark_report.json` plus `review/provider_native_tool_call_comparison_report.json` | Flagship demo | This is the milestone's clearest evidence that ADL can compare proposal discipline against provider-native tool-call behavior without conflating either with authority. | Benchmark/report proof is strong; it is not a live execution-authority demo. |
| WP-04 / WP-05 | Runtime/test-cycle recovery and coverage ergonomics | `review/runtime_test_cycle_recovery_report.md`, `review/runtime_test_cycle_recovery_changed_files.txt`, and `review/coverage_gate_ergonomics_report.md` | Flagship demo | This slice directly addresses one of the repo's biggest practical bottlenecks and shows concrete recovery rather than abstract planning. | The proof is operational and report-based, not a flashy product demo. |
| WP-06 | CodeFriend productization | `review/codefriend_productization/` | Strong bounded demo | The packet shows repeatable packet-to-report workflow surfaces and turns review artifacts into a real product lane. | It remains evidence/product proof, not autonomous customer delivery. |
| WP-07 | Review heuristics and demos | `review/review_heuristics_demo/` | Strong bounded demo | This is a substantial retained review packet with fixtures, outputs, and acceptance checks rather than a vague heuristics note. | It demonstrates bounded review behavior, not replacement of human judgment. |
| WP-08 / WP-09 / #3091 / #3092 / #3093 / #3094 | Google Workspace CMS bridge stack | `review/google_workspace_cms_bridge/` | Flagship demo | This is the milestone's richest end-to-end collaboration lane: fixture-backed bridge, native adapter boundary, live safety package, bounded live reads, bounded content-card roundtrip, and reusable operational package. | Strongest bounded live-collaboration demo in the milestone, but Workspace is still not canonical repo truth. |
| WP-10 | Moderne / OpenRewrite modernization | `review/code_modernization/modernization_demo_packet.md`, `modernization_dry_run_evidence.md`, `modernization_execution_command.md`, `modernization_execution_log.txt`, and `modernization_rewrite.patch` | Flagship demo | This is one of the milestone's most concrete proofs because it includes a real dry-run command, execution log, and retained patch rather than policy-only discussion. | The bounded recipe run is real; broad production modernization remains a later lane. |
| WP-11 | Speculative decoding prototype | `bash adl/tools/demo_v0912_speculative_decoding_showcase.sh`, `review/speculative_decoding/speculative_decoding_prototype_report.json`, and `review/speculative_decoding/speculative_decoding_prototype_packet.md` | Strong bounded demo | The prototype gives a truthful answer about conditional value without pretending universal speedups, and now has a clean operator-facing showcase command. | Explicit deferral remains: a stronger same-family serving-stack backend trial is future work. |
| WP-12 | Repo visibility follow-on | `review/repo_visibility/` | Strong bounded demo | The manifest, linkage report, and navigation packet make reviewer/planner navigation materially easier and are directly inspectable. | This remains navigation proof, not full repo cognition. |
| WP-13 | Publication program | `review/publication_program/` | Packet-first proof | The packet makes the backlog, gates, and non-publication posture explicit and reviewable. | This is intentionally not a live publication demo. |
| WP-14 | General intelligence paper packet | `review/general_intelligence_paper/` plus the canonical separate `general-intelligence-paper` repo | Packet-first proof | The packet creates a serious reviewer handoff and makes the separate paper repo's canonical role explicit. | Public release timing and deeper manuscript hardening remain outside this milestone. |
| WP-15 | Rustdoc/doc cleanup | `features/RUSTDOC_DOC_CLEANUP.md` plus the tracked docs cleanup landed by WP-15 | Packet-first proof | The proof is that stale milestone/workflow/doc surfaces were actually corrected on tracked docs, not just cataloged. | This slice is real but not inherently a showpiece live demo; its value is trust and hygiene. |
| WP-16 | Workflow guardrails | `bash adl/tools/demo_v0912_workflow_guardrails_showcase.sh`, `adl/tools/workflow_guardrails.sh`, `adl/tools/test_workflow_guardrails.sh`, and `review/workflow_guardrails/` | Flagship demo | This is a strong operator-safety demo because it now has one coherent showcase command on top of the guardrail script, retained test surface, and runbook/proof packet. | It demonstrates failure-closed guardrails, not the elimination of all operator error. |

## Best Demo Story For The Milestone

If the milestone needs a short, high-signal showcase, the strongest sequence is:

1. UTS + ACC benchmark and provider-native comparison.
2. Runtime/test-cycle recovery and coverage ergonomics.
3. Google Workspace CMS bridge stack.
4. Moderne / OpenRewrite bounded dry-run modernization.
5. Workflow guardrails hardening.

That sequence is the best combined answer to:

- does ADL produce serious runtime/tooling evidence?
- did this milestone solve practical operator pain?
- are the collaboration and modernization ideas real?
- did the workflow itself get safer?

## Explicit Non-Claims

- This packet does not upgrade packet-first proof into live execution proof.
- This packet does not declare the milestone released.
- This packet does not replace issue-local implementation proof, PR evidence, or later review.
- This packet does not claim every feature should now have a new bespoke live demo if its truthful route is a bounded packet.

## Recommended WP-17 Outcome

`WP-17` is complete when:

- `DEMO_MATRIX_v0.91.2.md` names the strongest truthful proving route for each slice
- `FEATURE_PROOF_COVERAGE_v0.91.2.md` reflects landed status instead of stale planning language
- this packet gives Sprint 4 review one stable convergence surface for the milestone's showcase story
