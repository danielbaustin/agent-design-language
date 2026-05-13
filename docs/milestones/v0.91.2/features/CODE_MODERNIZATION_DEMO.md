# Moderne / OpenRewrite LST Modernization Demo

## Metadata

- Feature Name: Moderne / OpenRewrite LST Modernization Demo
- Milestone Target: `v0.91.2`
- Status: planned
- Planned WP Home: WP-10
- Source Docs: `.adl/docs/TBD/code_modernization/`
- Proof Modes: dry run, review packet, demo

## Purpose

Define and prove a bounded modernization workflow where ADL plans, constrains,
and reviews deterministic code transformation around the Moderne platform and
the OpenRewrite ecosystem, without turning modernization into automatic mass
rewrite.

The key terminology should stay explicit:

- `Moderne` is the platform and multi-repo orchestration surface.
- `OpenRewrite` is the transformation framework.
- `LST` means `Lossless Semantic Tree`, the semantic code model that makes
  transformations precise and repeatable.
- `Recipes` are the deterministic transformation units that search and rewrite
  code over the LST.

## Scope

In scope:

- Moderne/OpenRewrite interaction plan.
- Explicit LST and recipe framing in the demo packet.
- Dry-run evidence.
- Reversibility and review policy.
- Relationship to CodeFriend and Google Workspace surfaces.

Out of scope:

- Unreviewed bulk rewrite.
- Production service launch.
- Automatic acceptance of generated patches.
- vague “AI rewrites code automatically” positioning.

## Acceptance Criteria

- Demo changes are source-grounded and reversible.
- The packet names Moderne, OpenRewrite, LST, and recipes accurately.
- Review policy is explicit.
- Mass rewrite requires explicit follow-on approval.
