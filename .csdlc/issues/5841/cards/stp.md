# Structured Task Prompt

Template: 1.0.0

Issue: 5841

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver behavior-preserving simplification of active Rust ownership boundaries, duplication, and maintainability hotspots before review.

## Deliverables

- behavior-preserving simplification of active Rust ownership boundaries, duplication, and maintainability hotspots before review
- exact refactoring inventory, focused behavior parity, Clippy and test proof, before/after LoC, and bounded review

## Acceptance

1. AC-1: WP-20 and WP-21 are merged, terminal, claim-free, ancestral, and the post-deletion hotspot inventory is pinned to the exact execution SHA.
2. AC-2: Every selected refactor names exact files, current and target owner, behavior invariants, duplication/boundary defect, before/after LoC, and rollback.
3. AC-3: Characterization and focused positive/negative tests prove no supported language, compiler, engine, runtime, lifecycle, artifact, or error-contract change.
4. AC-4: The change reduces meaningful responsibility mixing or duplication without widening public APIs, creating an unowned utility, or hiding feature/deletion work.
5. AC-5: Touched-workspace tests, strict Clippy, formatting, and applicable macOS/Linux CI pass at the exact candidate head.
6. AC-6: Exact-head independent review has no actionable findings and the closing PR reports unresolved hotspots without claiming broader milestone completion.

## Dependencies

- WP-20
- WP-21

## Inputs

- Live issue #5841 and WP-21A rows in docs/milestones/v0.92/WBS_v0.92.md and WP_ISSUE_WAVE_v0.92.yaml
- Exact terminal outputs and retained inventory from WP-20 and WP-21
- Current manifests, source boundaries, tests, and public APIs in adl-v2, adl-runtime-kernel, and csdlc-v2

## Non Goals

- Legacy deletion already owned by #5786 or any new v0.92 feature behavior
- Broad workspace rewrite, dependency-upgrade campaign, public API redesign, or aesthetic churn
- Moving code without reducing responsibility mixing or meaningful duplication
