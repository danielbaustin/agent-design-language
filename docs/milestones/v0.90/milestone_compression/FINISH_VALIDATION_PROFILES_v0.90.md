# Finish Validation Profiles - v0.90

## Status

Refinement issue: #2080

This document refines the v0.90 milestone compression pilot after the #2053
editor-refresh run.

## Purpose

Milestone compression has two separate concerns:

- execution compression: keep issue selection, scope, worktree binding, and drift
  checks small enough to move quickly.
- validation compression: choose the smallest truthful local validation set that
  proves the changed surface before handing the PR to CI and review.

The #2053 run showed that execution compression can work while finish-time
validation still pays the heavy local Rust suite cost. This document defines the
safe boundary for using focused local validation on low-risk docs/static-tooling
work without weakening review truth or CI requirements.

## Non-Negotiable Boundaries

- Focused local validation is not full local validation.
- Focused local validation does not remove CI.
- Focused local validation does not approve merge or release.
- Focused local validation does not apply to runtime, schema, security, release,
  or broad tooling changes unless a human explicitly escalates and records why.
- The output record must state exactly which validation profile was used and
  which checks actually ran.
- Root main must remain clean and untouched by issue implementation work.

## Profiles

### FULL_LOCAL

Use this profile for:

- runtime behavior changes
- schema or artifact contract changes
- security, signing, sandbox, provider, or remote execution changes
- release or ceremony truth changes
- broad tooling changes with unclear blast radius
- any issue where the operator is unsure whether focused validation is enough

Required local evidence:

- formatter / linter checks appropriate to the changed language
- relevant unit and integration tests
- full repo validation when the changed surface touches runtime or release gates
- SOR contract validation
- clean issue worktree before publication
- CI must pass before merge

### FOCUSED_LOCAL_CI_GATED

Use this profile only for low-risk docs/static-tooling work where the changed
surface is narrow and easy to validate directly.

Eligible examples:

- milestone planning docs
- static editor docs
- static browser copy or validation text
- small shell helper docs/tests where behavior is covered by a focused script
- review packet or process docs with deterministic grep-style guardrails

Required local evidence:

- explicit changed path list
- focused tests or grep-style proof for the changed claim
- parser or syntax checks for touched scripts or JavaScript when applicable
- SOR contract validation
- root checkout cleanliness check
- no tracked local `.adl` issue bundle residue
- PR remains gated on CI before merge

Required SOR wording:

- validation profile: FOCUSED_LOCAL_CI_GATED
- full local validation: not run
- CI requirement: required before merge
- rationale: one sentence explaining why focused validation is safe for this
  issue

### ESCALATE_TO_FULL

Use this profile when a low-risk issue starts drifting into risky territory.

Escalation triggers:

- touched code moves from static tooling into runtime behavior
- tests or docs reveal uncertainty about blast radius
- security, privacy, signing, provider, sandbox, or schema language appears
- release truth is changed rather than only checked
- the issue adds a new automation capability rather than documenting or checking
  an existing boundary

Required action:

- stop and either run FULL_LOCAL validation or split the risky work into a new
  issue.

## #2053 Pilot Lesson

#2053 was a good compression pilot because:

- the work scope was narrow
- pre/post milestone drift checking stayed passing
- root main remained clean
- focused editor tests proved the changed surface

It was not yet a fast closeout because `pr finish` still ran the heavy local
Rust validation suite before PR publication. That was safe, but it is not the
speed profile we want for future low-risk docs/static-tooling issues.

The refinement is therefore not "skip checks." The refinement is:

- run the smallest truthful local checks for low-risk changes
- record that the full local suite did not run
- require CI before merge
- escalate to FULL_LOCAL when risk is unclear

## Operator Checklist

Before using FOCUSED_LOCAL_CI_GATED, confirm:

- The issue is docs/static-tooling/process only.
- The changed paths are explicit.
- The focused validation commands directly cover the changed claims.
- The SOR says full local validation was not run.
- The PR is not merged until CI passes.
- Root main is clean and untouched.
- No hidden lifecycle or browser-direct execution claim was added.

If any answer is uncertain, use FULL_LOCAL or stop for review.
