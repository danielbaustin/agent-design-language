# Work Breakdown Structure (WBS) — v0.86

## Metadata
- Milestone: `v0.86`
- Version: `0.86`
- Date: `2026-03-27`
- Owner: `adl`

## How To Use
- Break work into independently-mergeable issues.
- Keep each item measurable and testable.
- Include deliverables, dependencies, issue links, and proof surfaces.
- `WP-01` is **always** the milestone **design pass**.
- Final WPs are reserved for demos, quality gates, docs/review convergence, release, and next-milestone planning.
- No work package is complete unless it produces code, artifacts, demos, validated docs, or a bounded review surface.

## WBS Summary
v0.86 delivers the first **working bounded cognitive system** for ADL, centered on:
- the canonical cognitive stack
- the canonical cognitive loop
- cognitive signals (instinct and affect)
- cognitive arbitration
- fast/slow reasoning paths
- bounded execution (AEE-lite)
- evaluation signals and termination conditions
- minimal frame adequacy and reframing
- bounded agency via candidate selection
- Freedom Gate decision control
- initial memory participation (ObsMem-lite)
- local demos that prove the integrated system is real
- documentation and proof surfaces strong enough to support implementation and review

This milestone is not satisfied by concept alignment alone. The milestone-defining planning docs under .adl/docs/v0.86planning/ must be implemented in at least one coherent execution path and aligned with the tracked milestone docs.

## Work Packages

| ID | Work Package | Description | Deliverable | Validation / proof surface | Dependencies | Issue |
|---|---|---|---|---|---|---|
| WP-01 | Design pass (canonical docs + planning) | Align DESIGN, VISION, WBS, SPRINT, CHECKLIST, RELEASE PLAN, RELEASE NOTES, README, and the corrected v0.86 planning set | All milestone docs coherent and complete | Canonical milestone docs no longer contradict each other or the planning source docs | None | #882 |
| WP-02 | Cognitive Stack Canonicalization | Establish one authoritative cognitive stack for the milestone | Canonical stack definition in docs and implementation notes | No competing stack definitions remain | WP-01 | |
| WP-03 | Cognitive Loop Canonicalization | Establish one authoritative loop / flow model for the milestone and bind it to the stack and control boundaries | Canonical loop definition with step order, control boundaries, and stage semantics | No competing loop definitions remain; loop and stack agree in docs and implementation notes | WP-01, WP-02 | |
| WP-04 | Cognitive Signals (Instinct + Affect) | Implement instinct and affect as bounded cognitive signals feeding the loop | Structured signal inputs and artifacts | Signals are emitted and visible in at least one run | WP-02, WP-03 | |
| WP-05 | Cognitive Arbitration | Implement arbitration as a real routing and control surface using signals and context | Arbitration outputs and routing behavior | Routing decisions are visible and reviewable in at least two scenarios | WP-03, WP-04 | |
| WP-06 | Fast / Slow Thinking Paths | Implement explicit fast-path and slow-path execution modes under arbitration control | Fast/slow path selection and structured outputs | Both paths are demonstrable and meaningfully different | WP-05 | |
| WP-07 | Agency and Candidate Selection | Implement bounded candidate generation / candidate selection so agency is operational rather than rhetorical | Candidate selection path with inspectable alternatives | At least one run shows candidate generation and selection before execution | WP-03, WP-04, WP-05 | |
| WP-08 | Bounded Execution (AEE-lite) | Implement bounded iterative execution with observable per-iteration behavior | Bounded execution loop and iteration artifacts | At least one run performs a visible bounded iteration | WP-05, WP-06, WP-07 | |
| WP-09 | Evaluation Signals and Termination | Implement evaluation signals (progress, contradiction, failure) and explicit termination conditions | Evaluation outputs and termination artifacts | Evaluation affects behavior or termination; termination condition is always inspectable | WP-08 | |
| WP-10 | Frame Adequacy and Reframing | Implement minimal frame adequacy assessment and bounded reframing behavior | Frame adequacy and reframing artifacts | At least one run demonstrates inadequate frame detection and bounded reframing/adaptation | WP-09 | |
| WP-11 | Memory Participation (ObsMem-lite) | Implement initial memory participation so outcomes or state transitions can be observed in the loop | Minimal memory read/write participation and artifacts | Memory participation is visible in outputs or subsequent control behavior | WP-09, WP-10 | |
| WP-12 | Freedom Gate (v0.86 minimal) | Implement bounded Freedom Gate with allow / defer / refuse decisions and artifacts | Freedom Gate event / decision output | Freedom Gate blocks or permits at least one case and produces an inspectable event | WP-07, WP-09, WP-10 | |
| WP-13 | Canonical Bounded Cognitive Path | Wire signals, candidate selection, arbitration, fast/slow reasoning, bounded execution, evaluation, reframing, memory participation, and Freedom Gate into one authoritative path | End-to-end integrated cognitive flow | One run traverses the canonical bounded cognitive path and emits structured outputs | WP-06, WP-08, WP-10, WP-11, WP-12 | |
| WP-14 | Artifact Schema Enforcement | Ensure all major stages emit required artifact shapes consistently | Schema-compliant outputs across the integrated path | Stage artifacts are stable, reviewable, and consistently named | WP-13 | |
| WP-15 | Local Agent Demo Program | Build bounded local-model demos exercising the entire v0.86 cognitive system | Working local demo(s) + proof surfaces | End-to-end demo runs locally and exercises signals, loop, arbitration, fast/slow, execution, evaluation, reframing, memory participation, agency, and Freedom Gate | WP-14 | |
| WP-16 | Demo Matrix and Review Surface | Update canonical demo matrix / proof-surface docs for v0.86 | Demo matrix + demo guide + reviewer entry point | Reviewers can identify what to run and what each demo proves | WP-15 | |
| WP-17 | Coverage / Quality Gate | Run tests, coverage, and bounded quality checks for the integrated cognitive system | Green coverage + passing tests/demos | Quality-gate evidence recorded and non-red | WP-15, WP-16 | |
| WP-18 | Docs + Review Pass | Align all docs with implementation and confirm feature docs remain truthful | Docs consistent with actual behavior | Canonical docs match implementation and proof surfaces | WP-17 | |
| WP-19 | Internal Review | Perform bounded internal review of milestone truth, behavior, and proof surfaces | Internal review record | Findings recorded with remediation pointers | WP-18 | |
| WP-20 | External / 3rd-Party Review Preparation | Prepare milestone for external review legibility and proof-surface clarity | External-review-ready package | Milestone docs, demos, and artifact surfaces are externally legible | WP-19 | |
| WP-21 | Review Findings Remediation | Fix or explicitly defer review findings | Remediation changes and/or explicit deferrals | Findings resolved or tracked with clear ownership | WP-19, WP-20 | |
| WP-22 | Release Ceremony | Final validation, tag, release notes, repo cleanup | Tagged release + clean repo state | Auditable release record and clean repository state | WP-21 | |
| WP-23 | Next Milestone Planning | Prepare next milestone planning package before closure | Planning package for next milestone | Next milestone materials exist before v0.86 closure | WP-22 | |

## Sequencing
- Phase 1: WP-01 → WP-06 (planning, stack, loop, signals, arbitration, fast/slow)
- Phase 2: WP-07 → WP-14 (agency, bounded execution, evaluation, reframing, memory, Freedom Gate, integrated path, artifacts)
- Phase 3: WP-15 → WP-23 (demos, quality, docs/review, release, handoff)

## Current Execution Interpretation
The milestone should be executed as one integrated cognitive-system build, not as isolated concept papers.

There must be exactly one authoritative bounded cognitive path at runtime for this milestone.

The intended order is:
1. Establish canonical milestone truth (`WP-01`)
2. Build the stack / loop / signal / routing substrate (`WP-02` through `WP-06`)
3. Add bounded agency, execution, evaluation, reframing, memory, and Freedom Gate control (`WP-07` through `WP-13`)
4. Enforce artifact truth (`WP-14`)
5. Prove the milestone with local demos and review surfaces (`WP-15` through `WP-16`)
6. Close with quality, docs, review, release, and next-milestone handoff (`WP-17` through `WP-23`)


If implementation and docs disagree, treat the disagreement as a defect to fix immediately.

The tracked milestone docs in docs/milestones/v0.86/ together with the milestone-defining planning docs under .adl/docs/v0.86planning/ form the implementation contract for v0.86. Any divergence between them is a defect and must be resolved before release.

## Acceptance Mapping
- WP-01 → All planning docs aligned with the corrected v0.86 planning set
- WP-02 → Stack agreement exists in docs and implementation
- WP-03 → Loop agreement exists in docs and implementation and binds the stack to execution
- WP-04 → Signals are structured, visible, and feed the loop
- WP-05 → Arbitration decisions are observable and reviewable
- WP-06 → Fast and slow paths are both demonstrable
- WP-07 → Candidate selection / agency path is observable
- WP-08 → Bounded execution performs at least one inspectable iteration
- WP-09 → Evaluation signals and termination conditions are emitted and used
- WP-10 → Minimal reframing / adaptation occurs in at least one scenario
- WP-11 → Memory participation is visible in outputs or subsequent behavior
- WP-12 → Freedom Gate blocks or permits at least one case
- WP-13 → One canonical bounded cognitive path exists and can be demonstrated
- WP-14 → All required artifact schemas are present and consistent
- WP-15 → Local demo shows the full bounded cognitive system with artifacts
- WP-16 → Demo matrix and proof-surface docs are reviewable
- WP-17 → Tests and demos pass with coverage targets
- WP-18 → Docs match implementation with no conceptual drift
- WP-19 → Internal review findings are recorded and actionable
- WP-20 → External review package is legible and runnable
- WP-21 → Review findings are resolved or explicitly deferred
- WP-22 → Repo is clean and release is tagged
- WP-23 → Next milestone plan is ready before closure

## Exit Criteria
- All v0.86 milestone-defining planning docs are implemented in at least one execution path and aligned with the tracked milestone docs.
- End-to-end local demo proves the bounded cognitive system, not partial components.
- Artifacts are complete, inspectable, and consistent across all major stages.
- Signals, arbitration, fast/slow routing, agency, bounded execution, evaluation, reframing, memory participation, and Freedom Gate behavior are observable.
- There is exactly one authoritative bounded cognitive path for the milestone.
- Review surfaces are strong enough for internal and external inspection.
- Repo is clean and releasable.
