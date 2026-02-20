## Metadata
- Milestone: `v0.5`
- Version: `0.5`
- Date: `2026-02-18`
- Owner: Daniel Austin

## Scope Freeze (Unit 0 Output)

v0.5 scope is frozen to the following categories:

1. Language Surface Completion
   - Explicit schemas for all 6 primitives:
     Agents, Runs, Providers, Tasks, Tools, Workflows
   - Deterministic composition semantics

2. Pattern + Compilation Layer
   - PatternSchema → ExecutionPlan compiler
   - Deterministic multi-agent pattern expansion

3. Runtime Controls
   - Configurable scheduler limits
   - Stable deterministic execution guarantees

4. Distributed + Security Completion (original v0.3 goals)
   - Remote execution MVP
   - Workflow signing + enforcement
   - Structured trace + replay guarantees

Anything not listed above is explicitly deferred to v0.6+.

---

## WBS Summary

v0.5 completes the ADL language surface, introduces deterministic multi-agent pattern compilation, exposes configurable scheduler controls, and formalizes distributed + signing capabilities originally planned for v0.3.

The milestone follows a disciplined execution structure:

0) Milestone Init  
1) Work Unit 1 — Tooling Stabilization  
2) Work Unit 2 — Primitive Schema Completion  
3) Work Unit 3 — Composition + Pattern Compiler  
4) Work Unit 4 — Scheduler Configurability  
5) Work Unit 5 — Remote Execution MVP  
6) Work Unit 6 — Signing + Enforcement  
7) Demo Generation Pass  
8) Documentation Pass  
9) Review Pass  
10) Closing Ceremony  

---

## Work Packages

| ID    | Work Package             | Description                                                 | Deliverable                         | Dependencies                     | Issue |
|-------|--------------------------|-------------------------------------------------------------|-----------------------------------|---------------------------------|-------|
| WP-00 | Milestone Init           | Finalize milestone docs, freeze scope                        | DESIGN + WBS + DECISIONS + SPRINT + CHECKLIST finalized | None                            | #330  |
| WP-01 | Tooling Stabilization    | Fix pr.sh start bug, stabilize nightly automations          | pr.sh fix + CI stable              | WP-00                           | #342  |
| WP-02 | Primitive Schema Completion | Explicit schemas for Agents, Runs, Providers, Tasks, Tools, Workflows | Schema definitions + validation tests | WP-00                           | #343  |
| WP-03 | Composition Layer        | Implement include/call + hierarchical workflow composition  | Deterministic multi-file workflow demo | WP-02                       | #344  |
| WP-04 | Pattern Compiler v0.1    | PatternSchema → ExecutionPlan compiler (linear, fork_join; optional map_reduce) | Deterministic pattern demo        | WP-03                           | #345  |
| WP-05 | Scheduler Configurability | Expose max_parallel + minimal policy surface                | Configurable concurrency demo     | WP-04                           | #357  |
| WP-06 | Remote Execution MVP     | Define remote protocol + reference server + placement rules | Mixed local/remote demo            | WP-03                           | #346  |
| WP-07 | Signing + Enforcement    | Implement sign/verify + enforcement in run                   | Signed workflow demo + rejection test | WP-02, WP-03                   | #347  |
| WP-08 | Demo Generation Pass     | Systematic demo sweep covering all primitives and patterns  | Demo matrix complete               | WP-04, WP-05, WP-06, WP-07      | #348  |
| WP-09 | Documentation Pass       | README + spec + milestone docs updated                       | Docs aligned with shipped behavior | WP-08                          | #349  |
| WP-10 | Review Pass              | Coverage audit + doc audit + regression audit                | CI green + nightly automation stable | WP-09                         | #350  |
| WP-11 | Closing Ceremony         | Release notes + tag + retrospective                          | v0.5.0 tagged and published       | WP-10                           | #351  |

---

## Sequencing

### Phase 1 — Structural Foundation (Language Lock-In)
- WP-00
- WP-01
- WP-02

### Phase 2 — Pattern + Compiler Layer
- WP-03
- WP-04
- WP-05

### Phase 3 — Distributed + Trust Model
- WP-06
- WP-07

These work packages may proceed in parallel with Phase 2 after WP-03 is complete or stable.

### Phase 4 — Demo + Documentation Hardening
- WP-08
- WP-09
- WP-10

### Phase 5 — Release + Retrospective
- WP-11

---

## Demo Matrix (v0.5)

Each primitive must have:

- Solo demo
- Composed demo

Structural demos must include:

- Linear workflow
- Multi-step chain
- Fork/join
- Hierarchical workflow
- Pattern-based debate
- Planner-executor
- Mixed local/remote placement
- Signed workflow execution
- Deterministic replay

---

## Acceptance Mapping

- Primitive schemas complete → WP-02
- Deterministic composition model → WP-03
- Pattern compilation deterministic → WP-04
- Configurable scheduler working → WP-05
- Remote execution functional → WP-06
- Signing enforced → WP-07
- All demos runnable locally → WP-08
- Docs fully aligned → WP-09
- CI and nightly automation stable → WP-10
- Tag + release published → WP-11

---

## Exit Criteria

- Every in-scope requirement maps to at least one WBS item.
- Every WBS item has an issue reference.
- Demo matrix fully satisfied.
- Signing + remote execution verified.
- Deterministic replay preserved.
- v0.5 milestone checklist fully satisfied.
- Release `v0.5.0` tagged and published.
- All demos execute with readable timestamps and user-visible progress output.