# ADL Decision Log — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Date: `2026-03-11`
- Owner: `Daniel Austin / Agent Logic`

## Purpose
Capture significant architectural, scope, and process decisions made during the v0.85 milestone so they are reviewable later.

This log intentionally records *why* decisions were made, not only the final outcome.

## How To Use
- Add one row per decision.
- Prefer links to issues or PRs instead of long prose.
- Keep status current: `accepted`, `rejected`, `deferred`, `superseded`.

---

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Treat v0.85 as a **strengthening milestone** rather than expanding scope. | accepted | ADL needs stronger operational maturity before expanding feature scope toward v0.9. | Expand feature scope now. | Keeps milestone disciplined and release‑focused. | `DESIGN_v0.85.md` |
| D-02 | Make **Adaptive Execution Engine (AEE)** a named theme of the milestone. | accepted | AEE had been repeatedly deferred; surfacing it clarifies the cognitive direction of ADL. | Continue deferring AEE to later milestones. | Establishes AEE as part of the core roadmap. | Issue #559 |
| D-03 | Introduce a **bounded affect / emotion model** as a reasoning control surface. | accepted | Emotion signals help prioritize reasoning and evaluation without introducing anthropomorphic claims. | Ignore affect entirely or implement unconstrained cognitive features. | Provides a structured foundation for later reasoning systems. | `AFFECT_MODEL_v0.85.md` |
| D-04 | Use v0.85 to define the **reasoning graph schema direction**. | accepted | Early schema alignment avoids later incompatibility between hypothesis engines and memory structures. | Wait until v0.9 implementation stage. | Creates a clean interface boundary for future reasoning work. | `REASONING_GRAPH_SCHEMA_V0.85.md` |
| D-05 | Tie **dependable execution** and **verifiable inference** to explicit artifacts rather than positioning language. | accepted | Trust claims must be grounded in observable system behavior. | Treat trust primarily as messaging or documentation. | Strengthens enterprise credibility of ADL. | Issue #729 |
| D-06 | Temporarily treat the planning workspace as the planning anchor even while docs remain split. | accepted | Avoid blocking work while milestone docs are reorganized. | Pause planning until doc layout cleanup finishes. | Allows milestone planning to progress immediately. | WBS_v0.85.md |
| D-07 | Allow a **bounded coverage threshold (≈90%)** for milestone release with documented rationale. | accepted | Prevents the release from being blocked by diminishing‑return test work. | Require higher coverage before release. | Keeps schedule realistic while maintaining quality visibility. | Issue #705 |
| D-08 | Require **internal review and external review** before the release ceremony. | accepted | Ensures milestone planning and architecture receive independent scrutiny. | Internal review only. | Improves trustworthiness and architectural discipline. | WBS_v0.85.md |

---

## Open Questions

- What demonstration artifacts should be required to validate **verifiable inference** before v0.9?  
  Owner: Daniel Austin  
  Tracking: Issue #729

- What is the minimum affect‑signal set that improves reasoning without unnecessary complexity?  
  Owner: Daniel Austin  
  Tracking: AFFECT_MODEL_v0.85.md

- Should milestone planning documents be consolidated into a single directory after v0.85?  
  Owner: Daniel Austin  
  Tracking: follow‑up cleanup

---

## Exit Criteria

- All milestone‑critical architectural decisions are logged with rationale.
- Deferred or rejected alternatives are recorded where relevant.
- Open questions have owners and tracking references.
- The decision log can explain the milestone design during internal or external review.
