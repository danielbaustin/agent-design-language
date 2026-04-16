# Milestone README - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`

## Purpose

Provide the canonical entry point for the `v0.89.1` milestone package.

`v0.89.1` is the explicit follow-on band to `v0.89`. It takes the adversarial, exploit-replay, and security-proof work that `v0.89` intentionally did not absorb and turns it into a real milestone package rather than an implied carry-forward note.

This package should be strong enough to:
- explain why the band exists
- show exactly what belongs in it
- seed a clean issue wave without reconstructing intent from local notes
- survive review as a mechanically issueizable milestone rather than another exploratory planning pass

## Overview

`v0.89.1` represents the stage where ADL moves from:
- governed adaptive execution with explicit judgment, action, and security posture surfaces

into:
- adversarial runtime operation, exploit/replay evidence, red-blue execution structure, self-attack patterns, and stronger security-proof surfaces

This milestone focuses on:
- adversarial runtime architecture
- red/blue/purple execution structure
- exploit artifacts and replay manifests
- continuous verification and exploit generation
- self-attacking system patterns
- adversarial and security demo surfaces
- operational skill substrate and composition surfaces needed to run these behaviors cleanly
- a bounded manuscript/publication workflow strong enough to write the initial ADL three-paper arXiv program inside the milestone

Key outcomes:
- a real tracked feature package for the `v0.89.1` adversarial/runtime band
- a bounded `arxiv-paper-writer` operational skill grounded in `Paper Sonata` and the existing writing-skill surfaces
- reviewer-legible manuscript packets for:
  - What Is ADL?
  - Gödel Agents and ADL
  - Cognitive Spacetime Manifold
- a coherent WBS and sprint plan that map the source planning corpus to executable work
- a clean boundary between what belongs to `v0.89` and what belongs to this follow-on milestone

## Scope Summary

### In scope
- adversarial runtime model and execution architecture
- exploit artifact schema family and replay manifest surfaces
- continuous verification / exploit-generation execution patterns
- self-attacking systems as a bounded architectural pattern
- flagship adversarial demo planning and supporting security-proof demo surfaces
- operational skills substrate and skill-composition runtime framing as they relate to adversarial/governed execution
- a bounded `arxiv-paper-writer` skill plus execution of the initial three-paper arXiv program

### Out of scope
- reopening or redefining the settled `v0.89` core milestone
- the later identity, moral-governance, and broader constitutional bands (`v0.91+`)
- seeding the official `v0.89.1` issue wave in this docs issue

## Document Map

Canonical milestone documents:
- Vision: `VISION_v0.89.1.md`
- Design: `DESIGN_v0.89.1.md`
- Work Breakdown Structure (WBS): `WBS_v0.89.1.md`
- Feature index: `FEATURE_DOCS_v0.89.1.md`
- Sprint plan: `SPRINT_v0.89.1.md`
- Decisions log: `DECISIONS_v0.89.1.md`
- Demo matrix: `DEMO_MATRIX_v0.89.1.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.89.1.md`
- Release plan / process: `RELEASE_PLAN_v0.89.1.md`
- Release notes: `RELEASE_NOTES_v0.89.1.md`
- Issue wave: `WP_ISSUE_WAVE_v0.89.1.yaml`

Tracked feature docs:
- `features/ADL_ADVERSARIAL_RUNTIME_MODEL.md`
- `features/RED_BLUE_AGENT_ARCHITECTURE.md`
- `features/ADVERSARIAL_EXECUTION_RUNNER.md`
- `features/EXPLOIT_ARTIFACT_SCHEMA.md`
- `features/ADVERSARIAL_REPLAY_MANIFEST.md`
- `features/CONTINUOUS_VERIFICATION_AND_EXPLOIT_GENERATION.md`
- `features/SELF_ATTACKING_SYSTEMS.md`
- `features/ADL_ADVERSARIAL_DEMO.md`
- `features/OPERATIONAL_SKILLS_SUBSTRATE.md`
- `features/SKILL_COMPOSITION_MODEL.md`

Supporting local planning inputs:
- `DELEGATION_AND_REFUSAL.md`
- `MULTI_AGENT_NEGOTIATION.md`
- `PROPOSED_OPERATIONAL_SKILLS.md`
- `ADL_SECURITY_DEMOS.md`
- `PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md`
- local arXiv paper-program planning doc (supports the committed `v0.89.1` publication/skills slice under `WP-08` and `WP-13`)

## Execution Model

This milestone is designed to execute as a standard ADL issue wave once officially opened:
- `WP-01`: milestone design pass and canonical package completion
- `WP-02` - `WP-10`: core adversarial/runtime feature band
- `WP-11` - `WP-13`: demo scaffolding, milestone convergence, integration demos, and the initial three-paper publication packet
- `WP-14`: quality gate
- `WP-15`: docs + review convergence
- `WP-16` - `WP-18`: internal review, 3rd-party review, and findings remediation
- `WP-19`: next milestone planning
- `WP-20`: release ceremony

Execution expectations after kickoff:
- each substantive WP gets a bounded issue and PR
- promoted feature docs resolve to implementation, proofs, or explicit defer records
- any remaining `v0.89` carry-forward ambiguity is eliminated by explicit issue ownership
- issue creation should be mechanical from the WBS, sprint plan, and issue-wave YAML rather than requiring another design rewrite

## Demo and Validation Surface

Primary validation is defined in:
- `DEMO_MATRIX_v0.89.1.md`

Additional validation surfaces:
- replayable exploit artifacts
- adversarial runtime traces and reviewer-facing demo packets
- quality-gate and review issue outputs

Success criteria:
- the milestone package tells one consistent story across README, design, WBS, sprint, and feature docs
- every promoted feature doc has an implementation home in the WBS
- every non-promoted source planning doc has an explicit supporting or later-band home

## Determinism and Reproducibility

The milestone should demonstrate:
- replayable exploit and adversarial-run evidence
- explicit artifact schemas for exploit, replay, and verification surfaces
- bounded adversarial execution loops that remain reviewer-legible

Evidence locations:
- `DEMO_MATRIX_v0.89.1.md`
- issue outputs and run artifacts in the local ADL control plane

## Risks and Open Questions

Known risks:
- `v0.89.1` can sprawl into a vague "security everything" band if the exploit/runtime focus is not maintained
- negotiation, refusal, and provider-capability extensions are conceptually relevant but can easily over-expand the milestone if promoted too early
- the publication track can starve the core runtime band if the writer-skill and manuscript work are not kept bounded and proof-oriented

Open questions:
- how much of the operational-skill substrate should land as code in `v0.89.1` versus remain design-contract work
- whether provider-capability extension belongs in this band or should remain a later security-extension slice
- which demo shapes are sufficient to prove adversarial/runtime behavior before heavier follow-on bands
- whether the three-paper program should aim for fully submission-ready manuscripts inside `v0.89.1` or stop at review-ready manuscript packets with explicit post-milestone submission cleanup

## Status

Current status: canonical planning package complete; official issue wave opened; Sprint 1 execution can proceed from the settled package

- Planning: canonical package completed in `#1860`, with visible `WP-01` anchor issue `#1922` and `#1806` landing the tracked next-milestone package from the `v0.89` side
- Execution: issue wave opened through `#1921`; `WP-01` is visible as `#1922` and `WP-02` - `WP-20` are queued as `#1923` - `#1941`
- Validation: package-level checks passed; milestone execution validation still pending
- Release readiness: not started; milestone docs are ready for review, not release

## Kickoff Posture

When `v0.89.1` opens, the expected motion should be:
- review the package, not redesign it
- seed the official issue wave directly from `WP_ISSUE_WAVE_v0.89.1.yaml`
- preserve the settled `v0.89` / `v0.89.1` boundary rather than reabsorbing adversarial/runtime work back into `v0.89`

What should not happen at kickoff:
- reopening the core milestone interpretation
- promoting weak supporting inputs as if they were mature tracked commitments
- using issue creation as an excuse to rediscover milestone scope by hand

## Exit Criteria

- all canonical milestone documents are complete and internally consistent
- every in-scope feature doc is mapped to a WBS item
- every out-of-scope source doc has an explicit later home or supporting-input role
- the issue wave can be seeded directly from this package without reconstructing milestone intent
