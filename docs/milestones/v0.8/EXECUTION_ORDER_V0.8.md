# v0.8 Execution Order and Dependency Map

This document defines the canonical implementation sequence for v0.8 work.

It is a planning/control artifact only. It does not change architecture or milestone scope.

## Scope Baseline

This execution map is anchored to the frozen v0.8 milestone docs baseline under:

- `docs/milestones/v0.8/`

## Deterministic Ordering Rules

Use these rules whenever multiple items appear runnable at the same time:

1. Respect explicit dependency edges first.
2. Prefer lower work package IDs (`WP-02` before `WP-03`, etc.).
3. If still tied, prefer lower issue number.
4. If still tied, use lexicographic branch slug.

## Workstreams

- `WS-A` Gödel experiment substrate: `WP-02` to `WP-06`
- `WS-B` Memory and evidence integration: `WP-07` and `WP-08`
- `WS-C` Authoring and automation: `WP-09` and `WP-10`
- `WS-D` Flagship demo implementation: `WP-11` and `WP-12`
- `WS-E` Release tail and convergence: `WP-13` to `WP-16`

## Canonical Execution Sequence

### Phase 0: Milestone Doc Baseline (Completed)

| Order | Item | Issue | Depends on |
|---|---|---|---|
| 0.1 | Promote canonical v0.8 docs | #659 | v0.75 release convergence |
| 0.2 | Reconcile canonical v0.8 doc inconsistencies | #660 | #659 |
| 0.3 | Canonical index and navigation pass | #661 | #660 |
| 0.4 | Freeze canonical docs baseline | #662 | #661 |

### Phase 1: Schema Spine

| Order | Item | Issue | Depends on |
|---|---|---|---|
| 1.1 | ExperimentRecord schema v1 (`WP-02`) | #609 | #662 |
| 1.2 | Canonical Evidence View (`WP-03`) | #610 | #662 |
| 1.3 | Mutation format v1 (`WP-04`) | #611 | #662 |
| 1.4 | EvaluationPlan v1 (`WP-05`) | #612 | #609, #610, #611 |
| 1.5 | Gödel experiment workflow template (`WP-06`) | #613 | #609, #610, #611, #612 |

### Phase 2: Memory + Runtime Contract Hardening

| Order | Item | Issue | Depends on |
|---|---|---|---|
| 2.1 | ObsMem indexing for run summaries + experiment records (`WP-07`) | #614 | #609, #610, #613 |
| 2.2 | ToolResult contract hardening (`WP-08`) | #618 | #610 |

### Phase 3: Authoring + Prompt Automation

| Order | Item | Issue | Depends on |
|---|---|---|---|
| 3.1 | Authoring surfaces v1 (`WP-09`) | #517 | #662 |
| 3.2 | Prompt automation + reviewer-ready flow (`WP-10`) | TBD | #517, #618 |

### Phase 4: Flagship Rust Transpiler Demo

| Order | Item | Issue | Depends on |
|---|---|---|---|
| 4.1 | Rust transpiler fixture + scaffold (`WP-11`) | TBD | #613, #517 |
| 4.2 | Rust transpiler verification + adaptive evidence (`WP-12`) | TBD | #612, #618, WP-10, WP-11 |

### Phase 5: Release Tail

| Order | Item | Issue | Depends on |
|---|---|---|---|
| 5.1 | Demo matrix + integration demos (`WP-13`) | TBD | WP-06, WP-07, WP-10, WP-12 |
| 5.2 | Coverage / quality gate (`WP-14`) | TBD | WP-08 through WP-13 |
| 5.3 | Docs pass + review convergence (`WP-15`) | TBD | WP-13, WP-14 |
| 5.4 | 3rd party review pass | TBD | WP-15 docs freeze |
| 5.5 | Review fixes / explicit deferrals | TBD | 3rd party review findings |
| 5.6 | Release ceremony (`WP-16`) | TBD | WP-15, review convergence |

## Cross-Workstream Dependency Notes

- `WP-10` is an integration hinge: it depends on both authoring (`WP-09`) and runtime contract hardening (`WP-08`).
- `WP-12` should not begin until `WP-10` and `WP-11` are stable; this avoids rework in demo evidence wiring.
- Release-tail items (`WP-13` onward) should not start before Phase 1 and Phase 2 core surfaces are merged.

## Contributor Guidance

When starting a v0.8 implementation issue:

1. Confirm its dependency row in this file is satisfied.
2. If a dependency issue is `TBD`, create/assign that issue before implementation.
3. Do not reorder release-tail sequence (`WP-13` -> `WP-14` -> `WP-15` -> review -> `WP-16`).

