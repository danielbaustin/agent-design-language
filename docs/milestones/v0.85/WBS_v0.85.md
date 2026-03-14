
# ADL Work Breakdown Structure — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Date: `2026-03-11`
- Owner: `Daniel Austin / Agent Logic`

## Summary
v0.85 is a strengthening milestone focused on bounded operational maturity, stronger trust surfaces, clearer authoring/review workflows, and concrete progress on the Adaptive Execution Engine (AEE) and affective reasoning substrate.

This WBS aligns the milestone around a small number of work packages that map cleanly to design intent, issue flow, validation, and release readiness. It should be treated as part of the canonical v0.85 planning set under `docs/milestones/v0.85/`.

## Work Packages

| WP | Package | Description | Deliverable(s) | Dependencies | Related issue(s) |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | Align milestone design, scope, planning language, and cross-document intent for v0.85. | Updated design/WBS/decision/checklist/release-plan docs with materially consistent scope and validation language. | None | #716 |
| WP-02 | Deterministic queueing + checkpoint substrate | Strengthen queue/checkpoint/resume semantics so workflows can be paused, resumed, and reasoned about without weakening determinism. | Runtime changes, validation commands, and design notes for deterministic execution state transitions. | WP-01 | #674 |
| WP-03 | Cluster / distributed execution groundwork | Advance cluster execution planning and initial substrate behavior while preserving explicit ownership, claims, and replay semantics. | Cluster execution doc updates and bounded implementation/prototype work. | WP-01, WP-02 | #730 |
| WP-04 | Prompt Spec completeness | Improve structured prompt contracts and authoring completeness so prompts/cards can be reviewed and generated more reliably. | Prompt Spec design/doc updates, validation/linting expectations, and linked issue/card work. | WP-01 | #735 |
| WP-05 | Authoring surfaces + structured prompt editor direction | Clarify the intended authoring experience for cards/prompts and reduce ambiguity in authoring/review artifacts. | Design notes and implementation cards for stronger authoring surfaces and artifact structure. | WP-01, WP-04 | #735 |
| WP-06 | Card Reviewer GPT stabilization | Improve review reliability, YAML/output consistency, and repeatability in card review workflows. | Reviewer prompt/process updates, validation expectations, and output quality improvements. | WP-01, WP-04 | #559 |
| WP-07 | Dependable execution surfaces | Turn dependable execution from positioning language into explicit runtime/artifact/trust surfaces. | Design and implementation work that ties dependable execution to observable workflow behavior and release validation. | WP-02, WP-03 | #681 |
| WP-08 | Verifiable inference surfaces | Strengthen evidence-linked outputs, provenance, and replay/report structures so ADL can make serious trust claims. | Reviewable artifacts and documentation for provenance, replay, and inference verification. | WP-02, WP-06, WP-07 | #681 |
| WP-09 | Adaptive Execution Engine (AEE) bounded progress | Advance AEE from deferred concept to concrete milestone work with bounded policy hooks and strategy-loop progress. | AEE design updates, issue/card work, explicit bounded-scope deliverables, and the implementation-slice breakdown anchored by `docs/milestones/v0.8/STICKTOITTIVENESS.md`. | WP-02, WP-07 | #716 |
| WP-10 | Emotion / affect model design | Define a bounded affective reasoning surface that can later influence evaluation, prioritization, and adaptive behavior. | Affective reasoning doc set, emotion/affect design notes, and alignment with Gödel–Hadamard–Bayes work. | WP-01, WP-09 | #716 |
| WP-11 | Reasoning graph schema direction | Define the schema direction for reasoning graphs, hypothesis records, tension records, and revision-friendly belief lineage. | Reasoning graph schema doc and linked planning material for later v0.9 work. | WP-10 | #716 |
| WP-12 | Hypothesis engine planning linkage | Connect affective reasoning and reasoning graphs to the planned Gödel hypothesis engine direction. | Planning docs that clarify the v0.85 conceptual role and the v0.9 implementation path. | WP-10, WP-11 | #716 |
| WP-13 | Demo matrix + integration demos | Validate that major v0.85 themes can be demonstrated coherently rather than only described independently. | Demo checklist, integration scenarios, and supporting artifacts. | WP-02 through WP-12 | #716 |
| WP-14 | Coverage / quality gate | Reach an acceptable quality gate for the milestone with documented exceptions if needed. | Coverage status, test notes, ratchet/exclusion decisions, and release-quality evidence. | WP-02 through WP-13 | #674 |
| WP-15 | Docs + review pass (3-step alignment) | Reduce contradictions across milestone docs and complete a bounded review sequence: (1) docs consistent, (2) internal review, (3) external review. | Cleaned milestone docs, internal review notes/findings, external review notes/findings, and aligned release-facing planning artifacts. | WP-01 through WP-14 | #716 |
| WP-16 | Release ceremony (final validation + tag + notes + cleanup) | Execute the bounded release process for v0.85. | Final validation evidence, release notes, release plan completion, tag, and follow-up cleanup notes. | WP-13, WP-14, WP-15 | #716 |

## Phasing
- Phase 1: Planning alignment and design clarity (WP-01, WP-10, WP-11, WP-12)
- Phase 2: Runtime / trust / authoring execution (WP-02 through WP-09)
- Phase 3: Demonstration, quality gate, doc alignment, and release (WP-13 through WP-16)

## Acceptance Criteria by Work Package
- WP-01 (Design pass) -> The core v0.85 planning docs no longer materially contradict one another on scope, trust, AEE, or affect-model intent.
- WP-02 -> Deterministic queue/checkpoint/resume behavior is documented and validated with explicit commands or test evidence.
- WP-03 -> Cluster/distributed execution direction is explicit and bounded; no hidden weakening of replay or ownership semantics.
- WP-04 -> Prompt Spec expectations are explicit enough to support consistent authoring and review.
- WP-05 -> Authoring-surface direction is clearer and produces less ambiguous artifacts.
- WP-06 -> Reviewer outputs are materially more reliable and structurally consistent.
- WP-07 -> Dependable execution claims are tied to concrete runtime/artifact behavior.
- WP-08 -> Verifiable inference claims are tied to provenance, replay, and evidence-linked artifacts.
- WP-09 -> AEE shows concrete bounded progress rather than remaining a deferred concept, and future follow-on work is decomposed into bounded slices rather than tracked as one umbrella subsystem.
- WP-10 -> The affective reasoning / emotion model is captured as a disciplined design surface, not as vague aspiration.
- WP-11 -> Reasoning graph schema direction is explicit enough to guide later implementation.
- WP-12 -> Hypothesis engine linkage is clear across the v0.85 conceptual layer and v0.9 implementation planning.
- WP-13 (Demos) -> At least one coherent integration/demo path exists across major milestone themes.
- WP-14 (Quality gate) -> Coverage and validation evidence are acceptable for release, or documented exceptions are explicitly justified.
