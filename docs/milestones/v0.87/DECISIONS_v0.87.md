# Decisions: v0.87

## Metadata
- Milestone: `v0.87`
- Version: `0.87`
- Date: `2026`
- Owner: `adl`

## Purpose
Capture milestone-critical architecture, scope, and process decisions for `v0.87` as they are made.

This milestone is a substrate/consolidation milestone. The decisions recorded here should explain:
- why `v0.87` is focused on coherence rather than scope expansion
- how trace, provider portability, shared memory, and operational/control-plane work are prioritized
- what was intentionally deferred to later milestones

## Decision Log
| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Treat `v0.87` as a substrate/consolidation milestone rather than a new capability milestone. | accepted | `v0.86` expanded the system substantially, especially in tooling, validation, and bounded cognition. `v0.87` must make the existing system coherent, deterministic, and externally credible before later milestones deepen identity, governance, and higher-order cognition. | Continue direct capability expansion into `v0.88+` themes immediately. | Keeps milestone scope disciplined and makes external review more credible. | `docs/milestones/v0.87/VISION_v0.87.md` |
| D-02 | Define the canonical `v0.87` substrate spine as `contracts -> execution -> trace -> review -> documentation`. | accepted | The milestone needs one explicit execution truth model so contracts, trace, review, and docs do not drift independently. | Improve surfaces independently without a unifying model. | Gives the design, checklist, and review surfaces a common architectural center. | `docs/milestones/v0.87/DESIGN_v0.87.md` |
| D-03 | Pull trace v1 forward into `v0.87` as foundational substrate work. | accepted | Later work depends on reconstruction-oriented execution truth. Signed trace can remain later, but base trace vocabulary and event structure must land early. | Delay all trace work until the later provenance/signing band. | Makes trace the ground-truth substrate for demos, review, and later governance/identity work. | `docs/milestones/v0.87/WBS_v0.87.md` |
| D-04 | Treat provider / transport redesign as a first-class `v0.87` work band. | accepted | Real Gödel/AEE/provider portability later will fail if provider handling remains brittle or string-based. The substrate must separate vendor, transport, and model identity now. | Leave provider handling ad hoc and defer redesign until later routing/capability milestones. | Enables portable configs, cleaner trace attribution, and a credible common-provider story. | `.adl/docs/roadmaps/ROAD_TO_v0.95.md` |
| D-05 | Place shared ObsMem foundation work in `v0.87`, but keep later social/governance memory out of scope. | accepted | Shared memory is needed as a substrate layer, but full social memory and governance-aware memory belong later. | Delay all shared-memory work, or over-expand `v0.87` into social/governance memory systems. | Keeps the milestone realistic while still landing the shared-memory base later milestones require. | `docs/milestones/v0.87/WBS_v0.87.md` |
| D-06 | Treat operational skills as a real substrate surface in `v0.87`. | accepted | Skills are not just convenience wrappers; they are reusable, bounded operational surfaces that help make review, workflow, and later cognition more structured and deterministic. | Keep workflow logic ad hoc or defer skills until later feature milestones. | Supports reproducible workflow behavior and canonical review/operational output surfaces. | `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md` |
| D-07 | Continue moving workflow/control-plane ownership out of shell scripts and into the canonical control plane. | accepted | Recent review findings show shell-heavy workflow ownership is fragile and can drift from public contracts. | Leave shell wrappers as the long-term source of workflow behavior. | Improves determinism, worktree safety, and the credibility of the tooling substrate. | `#1192` |
| D-08 | Reserve PR Demo work in `v0.87` for planning/preparation only, not real social/identity execution. | accepted | `v0.87` is not yet the milestone for identity-bearing persistent agents or governed multi-agent social behavior. Demo work here should prepare later PR Demo execution without inflating milestone claims. | Claim early PR Demo execution in `v0.87`. | Keeps demo claims honest and aligns roadmap expectations with actual substrate readiness. | `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md` |
| D-09 | Keep `v0.87` bounded and do not silently pull `v0.88+` systems forward. | accepted | The roadmap now has explicit homes for chronosense, aptitudes, AEE 1.0, Freedom Gate v2, identity, governance, and PR Demo execution. Pulling them forward would collapse milestone discipline. | Opportunistically absorb later systems during implementation. | Protects roadmap coherence and makes handoff into later milestones cleaner. | `.adl/docs/roadmaps/ROAD_TO_v0.95.md` |
| D-10 | Require reviewer-facing proof surfaces and artifact-backed demos for milestone credibility. | accepted | `v0.87` is intended to support internal and external evaluation. Claims must be inspectable through real artifact roots, demo surfaces, and review outputs. | Rely on prose descriptions or partially specified proof surfaces. | Raises the bar for milestone truthfulness and improves reviewer experience. | `docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md` |

## Open Questions
- Which exact issue sequence should be used for the first substrate implementation slice after the canonical docs land? (Owner: `Daniel Austin`) (Issue: `TBD`)
- Should trace v1 or control-plane consolidation be the very first implementation-heavy slice once sprint 1 planning is complete? (Owner: `Daniel Austin`) (Issue: `TBD`)
- What is the minimum provider set required to claim provider portability credibly in `v0.87` demos? (Owner: `Daniel Austin`) (Issue: `TBD`)
- Which initial operational skills are mandatory for the first bounded substrate proof in `v0.87`? (Owner: `Daniel Austin`) (Issue: `TBD`)

## Exit Criteria
- All milestone-critical architectural and scope decisions are logged with rationale.
- Deferred alternatives and intentionally later-milestone work are explicitly represented.
- Open questions that affect execution order or milestone truth have owners and tracking hooks.
- The recorded decisions align with the roadmap, design, WBS, sprint plan, demo matrix, and release plan.
