
# Decisions — v0.8

## Metadata
- Milestone: `v0.8`
- Version: `0.8`
- Date: `2026-03-07`
- Owner: `Daniel Austin / Agent Logic`

## Purpose
Capture significant v0.8 decisions while they are still fresh, including architecture, scope, sequencing, schema placement, demo strategy, and release process.

## How To Use
- Add one row per milestone-significant decision.
- Prefer explicit status values: `accepted`, `deferred`, `rejected`, `superseded`.
- Link to issues/PRs/docs where possible.
- Record both the rationale and the alternatives considered.
- Keep backlog items separate from accepted milestone scope.

## Decision Log
| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | v0.8 is the milestone for **controlled experimentation and authoring**, built on the deterministic substrate delivered in v0.75. | accepted | Provides a clear milestone identity: Gödel experiment artifacts, authoring surfaces, and a flagship adaptive-execution demo. | Keep v0.8 as a generic continuation of v0.75; defer experimentation further. | Aligns design, WBS, sprint plan, and demo narrative. | `docs/milestones/v0.8/DESIGN_V0.8.md` |
| D-02 | The **Rust transpiler / migration demo** belongs in v0.8, not v0.75. | accepted | The demo is a flagship capability showcase and depends on adaptive execution, evidence, and authoring surfaces; adding it to v0.75 would destabilize the release tail. | Implement it under v0.75 WP-13 demo matrix; defer it beyond v0.8. | Keeps v0.75 focused on stabilization and makes the demo a headline v0.8 artifact. | `docs/milestones/v0.8/DESIGN_V0.8.md`, `docs/milestones/v0.8/WBS_V0.8.md` |
| D-03 | Gödel experiment work should be implemented as **ordinary ADL workflows operating over deterministic artifacts**, not as a separate learning runtime. | accepted | Preserves the design principle that ADL “speaks ADL,” reduces architectural complexity, and keeps experiments replayable and auditable. | Build a separate learning runtime / agent subsystem. | Simplifies execution semantics and unifies workflows, replay, evidence, and experiments. | `docs/milestones/v0.8/DESIGN_V0.8.md`, `docs/milestones/v0.8/GODEL_SCIENTIFIC_METHOD.md` |
| D-04 | Design-stage Gödel schemas remain canonical under **milestone docs first**, with later promotion to runtime schemas only when code is consuming them. | accepted | Avoids premature runtime commitment while still preserving canonical machine-readable artifacts. | Keep schemas only in `.adl/docs/`; move immediately to `swarm/schemas/`. | Supports clean promotion path: docs/milestones/v0.8 now, runtime schema location later. | Issue #638, `docs/milestones/v0.8/GODEL_SCIENTIFIC_METHOD.md` |
| D-05 | Structured cards are first-class execution contracts, and prompt generation should be driven by **machine-readable prompt schema blocks** rather than markdown heuristics. | accepted | Improves determinism, automation reliability, and future reviewer/CI tooling. | Continue relying on markdown heading parsing and ad hoc prompt writing. | Enables Card Automation Pipeline and provider-agnostic prompt generation. | Issues #629, #630, #633; `docs/milestones/v0.75/STRUCTURED_PROMPTS_DESIGN.md` |
| D-06 | Output cards should include a **machine-readable Verification Summary / verification surfaces** so reviewer tooling can validate results without depending on prose interpretation. | accepted | Makes review automation and CI validation feasible and consistent. | Keep verification evidence entirely in prose sections. | Strengthens reviewer GPT / CI pipeline and formalizes validation outputs. | Issue #634, `docs/milestones/v0.75/STRUCTURED_PROMPTS_DESIGN.md` |
| D-07 | v0.8 release tail must include **documentation freeze before review**, an explicit **3rd party review** step, and only then the **release ceremony**. | accepted | Preserves review integrity and avoids silent doc drift during formal review. | Blend docs + review into one pass; skip 3rd party review. | Shapes WP-15/WP-16 sequencing and milestone checklist requirements. | `docs/milestones/v0.8/WBS_V0.8.md`, `docs/milestones/v0.8/SPRINT_V0.8.md`, `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md` |
| D-08 | v0.8 scope should stay focused on a narrow backbone: experiment artifacts, deterministic authoring flow, one flagship demo, and release discipline. | accepted | Prevents the milestone from ballooning into full online learning, broad GUI work, or unconstrained autonomy. | Pull in wider authoring UI, distributed execution, or full NL→ADL compiler work now. | Keeps the milestone ship-able and technically coherent. | `docs/milestones/v0.8/DESIGN_V0.8.md`, `docs/milestones/v0.8/WBS_V0.8.md` |
| D-09 | The **Hypothesis Engine for the Gödel agent** is a tracked future backlog item, not a committed v0.8 deliverable. | accepted | Keeps the idea visible without silently enlarging v0.8 scope. v0.8 provides the substrate the future engine will require. | Pull the hypothesis engine into current core scope; omit it entirely from planning docs. | Preserves backlog continuity and prevents the concept from being forgotten. | `docs/milestones/v0.8/DESIGN_V0.8.md` |
| D-10 | ToolResult hardening is part of the v0.8 experiment/evidence spine because evidence-oriented workflows need richer machine-readable success/error surfaces. | accepted | Experiment, repair, and reviewer flows require stronger output contracts than prose/log-only tool responses. | Leave ToolResult unchanged until later refactor. | Justifies WP-08 and ties ToolResult work into Gödel and authoring surfaces. | Issue #618, `docs/milestones/v0.8/WBS_V0.8.md` |

## Open Questions
- What is the exact canonical non-`.adl` location for design-stage Gödel schemas before any runtime promotion? (Owner: Daniel Austin) (Issue: #638)
- Should Prompt Spec and Verification Summary remain markdown-embedded YAML blocks, or gain a parallel JSON export path in v0.8? (Owner: Daniel Austin) (Issue: #630 / #634)
- What is the smallest stable fixture crate that makes the Rust transpiler demo compelling without introducing toolchain flakiness? (Owner: Daniel Austin) (Issue: TBD)
- Which v0.8 WPs should be pre-created as issues now versus left as TBD until v0.75 release completion? (Owner: Daniel Austin) (Issue: TBD)

## Exit Criteria
- All milestone-critical decisions are logged with rationale and status.
- Deferred / rejected / follow-on items are explicitly separated from accepted v0.8 scope.
- Open questions have owners and at least one tracking reference or explicit TBD.
- The decision log is consistent with the v0.8 design, WBS, sprint plan, and checklist.
