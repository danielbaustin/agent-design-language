# ADL v0.7 Work Breakdown Structure (WBS)

## Metadata
- Milestone: `v0.7`
- Version: `v0.7.x (release train)`
- Date: 2026-02-24
- Owner: Daniel Austin

---

## WBS Summary

v0.7 executes as a **two-phase release train**:

- **Phase 1: v0.7.0 Foundation** — runtime hardening + policy surfaces + security envelope + delegation. No full learning loop.
- **Phase 2: v0.7.x Learning Train** — incremental, overlay-based learning features (observe → score → suggest → apply → export), independent of ObsMem.


Explicit deferrals beyond v0.7:
- v0.75: ObsMem integration (and memory-backed learning substrate)
- v0.85/v0.9: Distributed / cluster execution and durable checkpoint / recovery engine

Release tail: after core feature work (including WP-12), v0.7 concludes with a set of parallelizable “tail” WPs (demos, quality gates, docs review, and release ceremony). These are intentionally structured to run in parallel where possible and converge on a final release checklist.

## Taxonomy (EPIC vs WP vs Task)

- **EPIC-\***: umbrella / grouping only (not directly executed as a single PR).
- **WP-\***: executable work package (drives PR/worktree flow via `pr.sh`).
- **Task**: smaller scoped issue that is pulled into a WP as a dependency or sub-issue.

EPICs organize the roadmap; WPs are the unit of execution.

---

## EPIC Map

| EPIC | Theme | Primary WPs | Related tasks |
|---|---|---|---|
| EPIC-A (#412) | Dynamic learning (release train) | WP-08, WP-09, WP-10 | — |
| EPIC-B (#413) | Delegation runtime | WP-04 | — |
| EPIC-C (#414) | Learning surfaces (ObsMem deferred to v0.75 planning) | WP-08 | — |
| EPIC-D (#415) | Cleanup + deferred hard systems work | WP-11 | — |
| EPIC-E (#429) | Security envelope + trust hardening | WP-02, WP-03 | #472, #370, #371, #386 |
| EPIC-F (#430) | Runtime resilience + checkpointing surfaces | WP-06 | — |
| EPIC-G (#478) | Release tail | WP-13, WP-14, WP-15, WP-16 | — |
| EPIC-H (#479) | Runtime identity migration (swarm → adl) | WP-12 | — |

---

## Work Packages

| ID | Work Package | Description | Deliverable | Dependencies | Issue(s) |
|---|---|---|---|---|---|
| WP-01 | v0.7 Milestone Docs Bootstrap | Create canonical milestone docs under `docs/milestones/v0.7/` and a safe script to regenerate/check them. | Canonical v0.7 milestone docs + `.adl/scripts/bootstrap_milestone_docs.sh` | None | #473 |
| WP-02 | Security Envelope Foundation | Harden trust boundaries for local/remote execution, including sandbox boundaries and symlink escape prevention. | Hardened sandbox rules + tests; trust envelope invariants documented | WP-01 | #429 (EPIC-E), #472 |
| WP-03 | Remote Signing + Trust Policy | Enforce signing and trust policy requirements for remote execution requests, with deterministic verification and clear failure semantics. | Remote signing enforcement + trust policy docs/tests | WP-02 | #370, #371, #386 |
| WP-04 | Delegation Runtime (Paper-driven) | Implement/solidify delegation runtime semantics and policy-driven delegation patterns (paper-driven / DeepMind-style). | Delegation runtime features + docs + demos | WP-01, WP-02 | #413 (EPIC-B) |
| WP-05 | Scheduler Policy Surface | Add explicit scheduler policy configuration (per-workflow concurrency / policy surface) without breaking determinism. | Scheduler policy surface + tests + docs | WP-01 | #369 |
| WP-06 | Runtime Resilience Surfaces | Deliver v0.7-level resilience surfaces (structured state, replay affordances, initial checkpointing surface) without promising durable recovery engine (v0.85/v0.9). | Resilience surfaces + docs; clear deferred-cluster note | WP-01, WP-05 | #430 (EPIC-F) |
| WP-07 | Canonical Execution Path Cleanup | Remove duplication and consolidate step execution semantics into canonical helpers without behavior changes. | Simplified execution code paths + regression tests | WP-01 | #383 |
| WP-08 | Learning Surfaces (No ObsMem) | Stabilize IDs, artifact layout, trace schema, and strict JSON schemas needed for learning train; explicitly independent of ObsMem. | Stable surfaces + `run_summary.json` schema stub + docs | WP-01, WP-02 | #412 (EPIC-A), #414 (EPIC-C) |
| WP-09 | Learning Train: Observe → Score → Suggest | Incremental learning features across v0.7.x minors: summaries, scoring hooks, explainable suggestions (no automatic application). | `run_summary.json`, scoring hooks, `suggestions.json` + tests | WP-08 | #412 (EPIC-A) |
| WP-10 | Learning Train: Apply Overlays + Export | Overlay application (opt-in) and dataset export; no silent mutation; reversible by deleting overlay artifacts. | Overlay mechanism + export format + CLI modes | WP-09, WP-02 | #412 (EPIC-A) |
| WP-11 | Cleanup / Deferred Systems Work | General deferred hard systems cleanup in v0.7, excluding high-churn rename which is scheduled last (WP-12). | Targeted cleanup PRs + docs updates | WP-01 | #415 (EPIC-D) |
| WP-12 | Runtime Identity Rename (Do Last) | Rename crate/package + binaries to `adl` while keeping `swarm/` directory path stable in v0.7; provide one-release compatibility shims. | `adl` binaries + compat shims + migration notes; no directory rename | All foundation WPs complete | #336 (EPIC-H) |
| WP-13 | Demo Matrix + Integration Demos | Standardize v0.7 demos (foundation + learning train) and ensure they are runnable and documented. | `docs/milestones/v0.7/DEMOS_v0.7.md` + runnable demo commands | WP-02–WP-07 for foundation demos; WP-08–WP-10 for learning demos | #474 (EPIC-G) |
| WP-14 | Coverage / Quality Gate | Establish v0.7 quality ratchet (coverage audit + exclusions documented) and ensure tests cover new security + delegation + scheduler surfaces. | Coverage report + exclusions + follow-up issues | WP-02–WP-11 (and WP-12 after rename, if applicable) | #475 (EPIC-G) |
| WP-15 | Docs + Review Pass (repo-wide alignment) | Final documentation alignment for v0.7 (READMEs, milestone docs consistency, threat model links, migration notes for rename). | Updated docs + review checklist completion | WP-13, WP-14, WP-12 | #476 (EPIC-G) |
| WP-16 | Release Ceremony | Final validation, tag, publish release notes, and cleanup/close milestone. | Tag + GitHub Release + milestone close | WP-15 | #477 (EPIC-G) |

---

## Sequencing

### Phase 1 — v0.7.0 Foundation

Execute and merge in roughly this order:

1) WP-01 (docs bootstrap)
2) WP-02 → WP-03 (security + signing)
3) WP-05 (scheduler surface)
4) WP-04 (delegation runtime)
5) WP-06 (resilience surfaces)
6) WP-07 (execution path cleanup)
7) WP-11 (cleanup)

**Exit for v0.7.0:** foundation runtime shipped with security envelope hardened and replayable/deterministic semantics intact.

### Phase 2 — v0.7.x Learning Train

1) WP-08 (learning surfaces)
2) WP-09 (observe/score/suggest)
3) WP-10 (apply/export)

**Guardrails:** learning is overlay-based, opt-in, auditable, and reversible; no dependency on ObsMem in v0.7.

### Convergence — High-churn Rename + Tail WPs

- WP-12 (rename; EPIC-H) runs **late** in v0.7 once functional surfaces stabilize.
- After WP-12, execute the release tail:
  - WP-13 (demos) and WP-14 (quality gate) run in parallel.
  - WP-15 (docs/review) converges results (including migration notes).
  - WP-16 (release ceremony) ships the release.

---

## Acceptance Mapping

- Security envelope hardening (WP-02/03) → no sandbox escapes; remote signing enforced; deterministic failure semantics.
- Delegation runtime (WP-04) → delegation patterns execute deterministically and are policy-driven.
- Scheduler/resilience (WP-05/06) → explicit policy surface + resilience affordances without breaking determinism.
- Canonical execution path (WP-07) → reduced duplication, stable behavior, better maintainability.
- Learning train (WP-08/09/10) → artifacted, overlay-only learning: observe → score → suggest → apply → export.
- Rename (WP-12) → `adl` identity shipped with one-release compat window; no `swarm/` directory rename in v0.7.

- Demos + quality gates (WP-13/14) → runnable integration demos and measurable quality ratchet.
- Docs + ceremony (WP-15/16) → consistent public docs, migration guidance, and a clean tagged release.

---

## Exit Criteria

- Every in-scope requirement maps to at least one WBS item and issue reference.
- v0.7.0 foundation can be released independently of v0.7.x learning minors.
- Learning features remain opt-in and reversible with strict artifact schemas.
- Deferred follow-on work is explicitly tracked (v0.75 ObsMem integration; v0.85/v0.9 cluster execution + durable checkpoint engine).
