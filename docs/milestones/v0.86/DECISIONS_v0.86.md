# Decisions — v0.86

## Metadata
- Milestone: `v0.86`
- Version: `0.86`
- Date: `2026-03-27`
- Owner: `Daniel Austin`

## Purpose
Capture the critical architectural and scope decisions that define v0.86 as the first executable **bounded cognitive system**.

This log is authoritative for what v0.86 *is* and what it explicitly *is not*.

---

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | v0.86 is scoped as the **first bounded cognitive system** (not a thin control layer) | accepted | The milestone must prove integrated cognition, not partial routing or gating | Ship only a control layer and defer cognition | Forces inclusion of signals, execution, evaluation, reframing, and memory participation | #882 |
| D-02 | **Demos are the primary proof surface**, not tests or docs | accepted | Cognitive systems must be demonstrated end-to-end, not inferred from components | Rely on unit tests or doc alignment | Elevates demo program to first-class deliverable | DEMO_MATRIX_v0.86.md |
| D-03 | The **canonical bounded cognitive path demo (D1)** is the primary milestone proof | accepted | One undeniable integrated proof prevents fragmented validation | Multiple equal demos with no anchor | Anchors release and review around a single system-level execution | DEMO_MATRIX_v0.86.md |
| D-04 | **Signals, bounded execution, evaluation, reframing, and memory participation are IN scope** (minimal bounded form) | accepted | These are required for real cognition and cannot be deferred without invalidating the milestone | Defer these to later milestones | Expands milestone to full loop while keeping implementations minimal and bounded | docs/milestones/v0.86/features/ |
| D-05 | **Agency must be implemented as candidate selection**, not rhetorical output | accepted | Agency must be observable and inspectable in artifacts | Implicit decision-making in text output | Forces real decision structures in runtime | AGENCY_AND_AGENTS.md |
| D-06 | **Freedom Gate is required in minimal form** (allow / defer / refuse) | accepted | Establishes early decision boundary and moral control surface | Omit gate until later milestones | Introduces real constraint layer into cognition | FREEDOM_GATE.md |
| D-07 | **Artifacts are the system of record** for behavior | accepted | Behavior must be inspectable and reviewable independent of output text | Logging-only or implicit reasoning | Enables deterministic inspection and review surfaces | DESIGN_v0.86.md |
| D-08 | **Determinism is structural, not textual** | accepted | LLM outputs vary; cognitive path must be stable instead | Require exact output matching | Defines how replay and validation are judged | DEMO_MATRIX_v0.86.md |
| D-09 | **Docs must match runtime exactly** (no aspirational content) | accepted | Prevents drift between architecture and implementation | Allow forward-looking or speculative doc content | Enforces discipline across all milestone artifacts | WBS_v0.86.md |
| D-10 | **Local-first execution is required for demos** | accepted | Ensures reproducibility and independence from external providers | Cloud-only demos | Guarantees demos are runnable by reviewers | LOCAL_AGENT_DEMOS.md |
| D-11 | **Release is blocked unless the full cognitive loop is proven** | accepted | Prevents shipping partial cognition disguised as progress | Release based on CI or partial feature completion | Enforces GO/NO-GO discipline tied to real behavior | RELEASE_PLAN_v0.86.md |
| D-12 | **There must be exactly one canonical bounded cognitive path** | accepted | Multiple competing paths would fragment behavior and invalidate proof | Allow parallel or experimental paths in milestone | Forces architectural clarity and reviewability | DESIGN_v0.86.md |
| D-13 | **Bounded implementations are required for all cognitive components** | accepted | Scope must remain executable in a short window while still proving the full loop | Fully general or unbounded implementations | Keeps milestone realistic while preserving correctness of architecture | docs/milestones/v0.86/features/ |

---

## Source of Truth Model

v0.86 is defined by the coordinated tracked documentation under `docs/milestones/v0.86/`, including the promoted feature-defining docs under `docs/milestones/v0.86/features/`.

If tracked milestone docs and promoted feature docs diverge, that divergence is a defect and must be resolved before release.

---

## Open Questions

- What is the minimal artifact schema contract for long-term stability? (Owner: Daniel Austin) (Issue: to be assigned during implementation)
- How should arbitration confidence be normalized across different models? (Owner: Daniel Austin) (Issue: to be assigned during implementation)

---

## Exit Criteria

- All decisions defining v0.86 scope and behavior are explicitly recorded
- In-scope cognitive components (signals, execution, evaluation, reframing, memory participation) are clearly included
- Out-of-scope features are clearly excluded and linked to future milestones
- No conflicting architectural interpretations remain
- The source-of-truth model (tracked docs + planning docs) is consistent and non-contradictory
