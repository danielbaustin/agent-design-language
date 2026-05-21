# Moderne / OpenRewrite LST Modernization Demo

## Metadata

- Feature Name: Moderne / OpenRewrite LST Modernization Demo
- Milestone Target: `v0.91.2`
- Status: implemented bounded packet
- Planned WP Home: WP-10
- Source Docs:
  - `.adl/docs/TBD/code_modernization/ADL_MODERNE_INTERACTION_PLAN.md`
  - `.adl/docs/TBD/code_modernization/FIRST_RECIPE_SHORTLIST_2026-05-10.md`
  - `.adl/docs/TBD/code_modernization/V0912_MODERNE_DEMO_PREP_PACKET_2026-05-10.md`
  - `.adl/docs/TBD/code_modernization/PROOF_PACKET_TEMPLATE_2026-05-10.md`
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

## Current State

`WP-10` currently carries a bounded modernization packet with a real
OpenRewrite dry-run against a tracked Java fixture rather than pretending the
Rust ADL repository itself is already a live Java modernization target.

What is now present:

- one tracked interaction plan for ADL as the governed modernization layer
- one explicit executed dry-run command and captured execution log
- one reversibility and review policy
- one top-level demo packet tying those surfaces together
- one real OpenRewrite patch artifact copied into the tracked packet

Why the packet stops at this boundary:

- this repository does not contain a Java/Maven modernization target
- the bounded fixture avoids fake claims about modernizing the Rust source tree
- the packet still proves the real ADL claim: governed planning,
  pinned tool boundary, review gate, residual honesty, and a real deterministic
  rewrite event

## Proving Surface

- `docs/milestones/v0.91.2/review/code_modernization/modernization_interaction_plan.md`
- `docs/milestones/v0.91.2/review/code_modernization/modernization_dry_run_evidence.md`
- `docs/milestones/v0.91.2/review/code_modernization/modernization_reversibility_and_review_policy.md`
- `docs/milestones/v0.91.2/review/code_modernization/modernization_demo_packet.md`
- `docs/milestones/v0.91.2/review/code_modernization/modernization_execution_command.md`
- `docs/milestones/v0.91.2/review/code_modernization/modernization_execution_log.txt`
- `docs/milestones/v0.91.2/review/code_modernization/modernization_rewrite.patch`
