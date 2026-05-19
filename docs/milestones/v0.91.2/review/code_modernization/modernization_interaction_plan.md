# ADL Moderne / OpenRewrite Modernization Interaction Plan

## Purpose

Define the bounded `WP-10` interaction shape for using ADL as the governed
intelligence layer around Moderne / OpenRewrite modernization.

This plan is intentionally about the interaction boundary, not about claiming
that ADL itself performs direct source rewriting.

## Core Thesis

The demonstration should make one separation obvious:

- `Moderne` / `OpenRewrite` provide deterministic transformation hands.
- ADL provides the planning, scoping, authority, and review mind.

The intended value is not "AI edits code automatically."
The intended value is "ADL governs deterministic modernization more
intelligently and more reviewably than naïve tool calling."

## Terms

- `Moderne`: multi-repository orchestration and execution surface
- `OpenRewrite`: deterministic transformation framework
- `LST`: Lossless Semantic Tree used for precise source transformation
- `Recipe`: deterministic transformation unit executed over the LST

## Intended Operator Flow

1. Declare one modernization objective.
2. Record non-goals and blast-radius limits.
3. Search candidate recipe families.
4. Inspect recipe metadata and likely scope.
5. Select one bounded recipe path and reject broader alternatives explicitly.
6. Run a scan or dry-run posture first.
7. Review the prospective or resulting diff before any mutation acceptance.
8. Route residuals into follow-on work rather than hiding partiality.

## First Demonstration Boundary

The first demonstration should stay in the safest proof band:

- bounded repository objective
- deterministic recipe family
- reviewer-comprehensible diff size
- explicit dry-run-first posture
- reversible workflow with review gate before acceptance

The first demo should avoid:

- broad framework migration
- mass dependency churn
- multi-repo orchestration theater without reviewable proof
- any claim that generated patches should merge automatically

## Recommended First Recipe Category

The strongest first proof category remains:

1. static-analysis remediation
2. code-quality cleanup
3. simple API modernization

These categories maximize:

- deterministic behavior
- easy reviewer comprehension
- smaller blast radius
- cleaner accepted / partial / blocked classification

## UTS + ACC Role

The modernization lane should preserve the same ADL governance split as other
tooling demos:

- UTS describes the modernization tool surface and its side-effect class.
- ACC records authority scope, approval posture, and visibility posture.
- The model may choose and justify a recipe path, but it does not directly edit
  source files as a substitute for the deterministic transformation engine.

## Expected Proof Surfaces

`WP-10` should be reviewable through:

- this interaction plan
- a dry-run evidence packet
- a reversibility and review policy
- a top-level demo packet tying the pieces together

## Non-Claims

- This plan does not claim a live Moderne platform integration.
- This plan does not claim repo-local Java modernization was executed in this
  repository.
- This plan does not authorize mass rewrite.
- This plan does not claim ADL replaces OpenRewrite.
