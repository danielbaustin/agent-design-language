# Structured Planning And Plan Review

## Milestone Boundary

This `v0.91` feature makes saved execution planning a first-class workflow
surface rather than a temporary chat behavior.

It sits downstream of the existing issue-bundle artifacts (`STP`, `SIP`,
`SOR`) and upstream of tracked execution. It does not replace implementation,
review, or closeout. It improves them by making the intended execution plan
durable, reviewable, and auditable.

## Purpose

ADL already treats source prompts, execution context, and output truth as
structured artifacts. Planning should reach the same quality bar.

The purpose of this feature is to:

- preserve the chosen execution plan inside the issue bundle
- make assumptions and stop conditions explicit before implementation starts
- require review of the plan before tracked execution where policy demands it
- improve delegation quality, validation quality, and replanning discipline

## Core Thesis

`/plan` is helpful, but it is too easy to lose when it exists only in chat.

ADL should therefore introduce a durable saved planning artifact plus a
planning skill and a review gate, so plan quality becomes operational rather
than accidental.

## Intended Artifact

The canonical planning artifact should be:

- `SPP`: Structured Plan Prompt

Primary location:

- `.adl/<version>/tasks/issue-<n>__<slug>/spp.md`

Compatibility surface:

- `.adl/cards/<issue>/plan_<issue>.md`

## What The SPP Should Capture

The first `v0.91` implementation should make the plan artifact capture:

- the execution goal
- the intended implementation sequence
- assumptions
- touched files and surfaces
- validation commands
- proof or demo expectations
- delegation boundaries
- risks, stop conditions, and replan triggers

## Planning Skill Requirement

This feature should also introduce a first-class planning skill:

- `pr-plan`

The skill should:

- read the issue prompt and existing task bundle
- generate or revise `spp.md`
- keep the plan bounded and execution-oriented
- stop before implementation begins

Later hardening may add:

- `pr-plan-review`
- `spp-editor`

## Review Requirement

Tracked execution should not depend only on the presence of an `SPP`.

The plan should also be reviewed.

The review should check:

- scope truthfulness
- touched-file truthfulness
- validation sufficiency
- delegation safety
- hidden assumptions
- stop and replan quality

## Architectural Placement

The planning workflow belongs between bootstrap and execution:

1. issue bundle exists
2. `SPP` is generated or revised
3. plan review runs
4. execution starts only if plan policy is satisfied

This keeps planning separate from:

- source intent (`STP`)
- execution context (`SIP`)
- execution result (`SOR`)

## Implementation Placement

The first `v0.91` implementation should land:

- an `SPP` template or schema
- a canonical issue-bundle save location
- `pr-plan`
- a bounded plan review result model
- workflow awareness of missing, stale, or unreviewed plans
- `SOR` truth fields recording whether execution matched the reviewed plan

## Evidence Expectations

The proof surface should show that:

- plans are saved with the issue bundle
- reviewed plans improve execution quality
- stale plans are detectable
- execution can record whether it followed or deviated from plan
- delegation and validation are clearer than in the plan-less workflow

## Non-Claims

This feature does not claim that planning eliminates all mistakes, removes the
need for judgment, or turns execution into a rigid script.

It claims a narrower result:

ADL should have a durable, reviewable, and policy-aware planning surface that
improves implementation discipline before tracked execution begins.
