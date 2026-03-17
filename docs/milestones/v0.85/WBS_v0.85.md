# ADL Work Breakdown Structure — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Date: `2026-03-16`
- Owner: `Daniel Austin / Agent Logic`

## Summary
v0.85 is a strengthening milestone focused on bounded operational maturity, stronger trust surfaces, clearer authoring/review workflows, first practical Gödel runtime progress, and a minimal working affective-reasoning substrate.

This WBS aligns the milestone around a four-sprint, twenty-five-work-package structure that maps design intent to canonical issues, demos, validation, and release readiness. It should be treated as part of the canonical v0.85 planning set under `docs/milestones/v0.85/`.

## Work Packages

| WP | Package | Description | Deliverable(s) | Dependencies | Related issue(s) |
|---|---|---|---|---|---|
| WP-01 | Milestone reorganization and docs alignment | Align the milestone docs, issue graph, sprint structure, and process expectations under the `#886` umbrella issue. | Updated canonical docs, revised milestone structure, issue-reconciliation guidance. | None | #886, old scaffold issue #866 to absorb/supersede |
| WP-02 | Deterministic queue, checkpoint, and steering substrate | Strengthen queue/checkpoint/resume semantics and make steering replay-compatible without weakening determinism. | Runtime changes, tests, validation commands, and explicit steering/checkpoint semantics. | WP-01 | #674, superseding placeholder #867 |
| WP-03 | Cluster / distributed execution groundwork | Advance cluster execution planning and initial substrate behavior while preserving explicit ownership, claims, leases, and replay semantics. | Cluster execution updates, bounded implementation/prototype work, explicit ownership/claim/lease/replay semantics. | WP-01, WP-02 | #868 provisional remap issue |
| WP-04 | Prompt Spec completeness for editors | Improve structured prompt contracts so prompts/cards can be reviewed, generated, and edited more reliably. | Prompt Spec updates, validation/linting expectations, and editor-facing field clarity. | WP-01 | #716, #869 provisional remap issue |
| WP-05 | First authoring/editor surfaces | Ship real issue/input-card editor surfaces rather than only design notes. | HTML/HPA editor artifacts, preview/validation flow, save/export path compatible with ADL artifacts. | WP-01, WP-04 | #870 provisional remap issue |
| WP-06 | Editing and review GPT/tooling surfaces | Improve editing/review reliability and repeatability through reusable GPT/prompt assets and workflow integration. | Issue Editor, Card Editor, Card Reviewer GPT/prompt assets, and tighter review workflow integration. | WP-04, WP-05 | #871 provisional remap issue |
| WP-07 | Dependable execution runtime surfaces | Turn dependable execution into explicit runtime and artifact behavior rather than positioning language. | Runtime artifacts, traces, validations, and docs tied to dependable execution claims. | WP-02, WP-03 | #872 provisional remap issue |
| WP-08 | Verifiable inference runtime surfaces | Strengthen evidence-linked outputs, provenance, replay/report structures, and reviewability. | Provenance/evidence artifacts, replay/validation support, and concrete trust surfaces. | WP-02, WP-06, WP-07 | #873 provisional remap issue |
| WP-09 | Adaptive Execution Engine bounded progress | Advance AEE from deferred concept to concrete milestone work with bounded policy hooks and strategy-loop progress. | AEE traces, policy hooks, strategy-loop/adaptation artifacts, and demoable behavior where possible. | WP-02, WP-07, WP-08 | #874 provisional remap issue |
| WP-10 | Deterministic hypothesis generation engine | Make the first practical Gödel hypothesis engine real and explicit inside the milestone. | Deterministic hypothesis behavior and/or artifacts, code/tests/docs/demos. | WP-09 | #748 |
| WP-11 | Policy-learning and adaptive Gödel loop | Make bounded policy-learning and adaptive Gödel behavior explicit and testable. | Policy-learning traces, bounded adaptation behavior, and supporting tests/docs/demos. | WP-09, WP-10 | #749 |
| WP-12 | Experiment prioritization and strategy confidence | Make prioritization, novelty handling, and strategy confidence inspectable and demoable. | Prioritization logic/artifacts, confidence traces, and tests/docs/demos. | WP-10, WP-11 | #750 |
| WP-13 | Cross-workflow learning and recursive improvement | Make cross-workflow learning and recursive improvement explicit, bounded, and reviewable. | Learning artifacts/behavior, bounded recursive-improvement surfaces, and demos/traces/docs/tests. | WP-10, WP-11, WP-12 | #751 |
| WP-14 | Promotion and eval-report artifact loop | Turn promotion and evaluation reporting into concrete artifact-bearing workflow surfaces. | Promotion artifacts, evaluation-report artifacts, and tests/docs/demos tied to those outputs. | WP-10 through WP-13 | #752 |
| WP-15 | Affect engine core | Deliver a minimal working affect engine that is concrete enough to demo and strong enough to evolve later. | Minimal working affect-engine code, affect state model, update rules, artifacts/traces, and a proof path. | WP-09, WP-10 through WP-14 | #875 provisional remap issue |
| WP-16 | Reasoning graph and affect integration | Integrate reasoning graphs with affect so the system can represent tensions, salience, priorities, contradictions, and lineage in a behaviorally meaningful way. | Reasoning-graph schema docs, artifact examples, affect-linked reasoning graph artifacts/examples, and legible integration surfaces. | WP-15 | #876 provisional remap issue |
| WP-17 | Affect-plus-Gödel vertical slice | Connect affect, reasoning graphs, and Gödel hypothesis work into a runnable bounded vertical slice. | Runnable demo showing affect changing ranking/hypothesis/evaluation behavior plus emitted artifacts/traces. | WP-10 through WP-16 | #877 provisional remap issue |
| WP-18 | Demo program for v0.85 features | Prove the milestone through multiple runnable bounded demos rather than only checklists. | Demo matrix/playbook, runnable demos, and explicit linkage between demos and major new features. | WP-02 through WP-17 | #743, #878 provisional remap issue |
| WP-19 | Coverage / quality gate | Reach an acceptable quality gate for the milestone with documented exceptions if needed. | Coverage status, test notes, ratchet/exclusion decisions, release-quality evidence. | WP-02 through WP-18 | #879 provisional remap issue |
| WP-20 | Documentation consistency pass | Reduce contradictions across milestone docs and canonical issue bodies before review and release. | Consistent milestone docs and aligned issue/body text. | WP-01 through WP-19 | #880 provisional remap issue |
| WP-21 | Internal review | Perform internal review as a distinct milestone gate. | Internal review notes/findings and explicit action items. | WP-18 through WP-20 | new follow-on issue under #886 |
| WP-22 | External review | Perform external review as a distinct milestone gate. | External review notes/findings and explicit action items. | WP-18 through WP-21 | new follow-on issue under #886 |
| WP-23 | Review findings remediation | Resolve or explicitly defer findings from internal and external review. | Remediation notes, changed artifacts, and tracked deferrals. | WP-21, WP-22 | new follow-on issue under #886 |
| WP-24 | Release ceremony | Execute the bounded release process for v0.85. | Final validation evidence, release notes, release-plan completion, tag, and follow-up cleanup notes. | WP-19, WP-20, WP-23 | new follow-on issue under #886 |
| WP-25 | Next milestone planning | Prepare the next milestone before closing v0.85. | Next milestone planning docs/templates and explicit process closeout evidence. | WP-24 | new follow-on issue under #886 |

## Phasing
- Sprint 1: milestone reorganization and execution substrate (WP-01 through WP-04)
- Sprint 2: authoring surfaces and review tooling (WP-05 through WP-08)
- Sprint 3: Gödel, affect, reasoning graphs, and AEE/runtime progress (WP-09 through WP-17)
- Sprint 4: demos, quality gate, review, release, and next-milestone planning (WP-18 through WP-25)

## Issue-Graph Notes

- `#886` is the umbrella reorganization issue for the milestone until the issue graph fully matches this WBS.
- `#674` is the canonical queue/checkpoint/steering issue; `#867` is duplicate/placeholder material to absorb, supersede, or close.
- Gödel issues `#748` through `#752` are the canonical Sprint 3 runtime/cognitive track and should not be treated as side issues.
- The provisional generated issue set `#866` through `#882` must be renamed, remapped, split, merged, or closed so each work package has one canonical issue and each canonical issue belongs to one work package.

## Acceptance Criteria by Work Package
- WP-01 -> The core v0.85 planning docs and issue graph no longer materially contradict one another on scope, sequencing, demos, or issue ownership.
- WP-02 -> Deterministic queue/checkpoint/steering behavior is documented and validated with explicit commands, tests, or artifacts, and steering remains replay-compatible.
- WP-03 -> Cluster/distributed execution direction is explicit and bounded; no hidden weakening of replay or ownership semantics.
- WP-04 -> Prompt Spec expectations are explicit enough to support consistent authoring, editing, and review.
- WP-05 -> Real editor surfaces exist in-repo and materially improve authoring flow.
- WP-06 -> Editing/review tools are materially more reliable and structurally consistent.
- WP-07 -> Dependable execution claims are tied to concrete runtime/artifact behavior.
- WP-08 -> Verifiable inference claims are tied to provenance, replay, and evidence-linked artifacts.
- WP-09 -> AEE shows concrete bounded progress rather than remaining a deferred concept.
- WP-10 -> Deterministic hypothesis-generation behavior is concrete enough to inspect, test, or demo.
- WP-11 -> Bounded policy-learning/adaptive Gödel behavior is explicit and inspectable.
- WP-12 -> Prioritization and strategy-confidence behavior is concrete enough to inspect and demo.
- WP-13 -> Cross-workflow learning/recursive-improvement behavior is bounded and reviewable.
- WP-14 -> Promotion and evaluation reporting emit concrete artifacts or strongly specified implementation surfaces.
- WP-15 -> A minimal working affect engine exists with explicit state, update rules, and observable traces.
- WP-16 -> Reasoning graphs are explicitly integrated with affect in a legible artifact-bearing way.
- WP-17 -> A runnable bounded affect-plus-Gödel demo slice exists and shows affect changing reasoning or hypothesis behavior.
- WP-18 -> Multiple runnable demos exist, including steering/queueing, HITL/editor/review flow, and affect-plus-Gödel behavior.
- WP-19 -> Coverage and validation evidence are acceptable for release, or documented exceptions are explicitly justified.
- WP-20 -> The milestone docs and canonical issue bodies are internally consistent.
- WP-21 -> Internal review is completed and findings are recorded.
- WP-22 -> External review is completed and findings are recorded.
- WP-23 -> Review findings are resolved or explicitly deferred with ownership.
- WP-24 -> Final validation evidence, release notes, and release/tag mechanics are complete and auditable.
- WP-25 -> Next milestone planning materials are ready before v0.85 is considered fully closed.
