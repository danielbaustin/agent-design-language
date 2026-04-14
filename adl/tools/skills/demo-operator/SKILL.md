---
name: demo-operator
description: Run one named demo in a bounded way and classify the result as proving, non-proving, skipped, or failed without absorbing the demo's own logic. Use when the user wants consistent demo execution and proof classification rather than manual after-the-fact interpretation.
---

# Demo Operator

Run one named demo with a bounded, evidence-first mindset.

This skill exists to run one named demo and classify its result truthfully.

This skill is allowed to:
- inspect one named demo entrypoint
- check prerequisites and operator-gated requirements
- run one bounded demo command or dry-run surface
- classify the outcome as proving, non-proving, skipped, or failed
- write one reviewable artifact describing the result

It is not allowed to:
- rewrite the underlying demo into a different workflow
- silently absorb release-review or milestone-closeout logic
- overclaim proof when the demo was skipped or operator-gated
- widen into unrelated implementation work

## Quick Start

1. Confirm the concrete demo target.
2. Read the demo doc or entrypoint first.
3. Identify prerequisites and operator-gated requirements.
4. Run the smallest truthful demo command.
5. Classify the result.
6. Record the outcome and stop.

## When To Use It

Use this skill when:
- one named demo should be executed end-to-end in a reviewable way
- the operator wants consistent proof classification
- a demo run should yield a bounded artifact rather than only console output

Do not use it when:
- the real task is building or changing the demo itself
- the target is a release package rather than an ordinary demo surface
- the operator wants repo-wide release evidence instead of one demo outcome

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete target:
  - `target.demo_name`
  - `target.demo_command`
  - `target.demo_doc_path`

Useful additional inputs:
- `artifact_root`
- `expected_artifacts`
- `provider_requirements`
- `operator_gate_reason`
- `validation_mode`
- `allow_live_provider`

If there is no concrete demo target, stop and report `blocked`.

## Workflow

### 1. Resolve The Demo Target

Prefer:
1. explicit named demo with known command
2. explicit demo command
3. explicit demo doc path plus its documented command

If multiple demos match ambiguously, stop.

### 2. Check Prerequisites

Inspect:
- whether the demo command exists
- whether required docs or fixtures exist
- whether credentials, providers, or remote hosts are required
- whether the demo is operator-gated or intentionally dry-run only

### 3. Run The Smallest Truthful Demo Surface

Prefer:
- documented dry-run or fixture-backed demo path
- bounded smoke command
- one live-provider path only when explicitly allowed

### 4. Classify The Result

Use one of:
- `proving`
- `non_proving`
- `skipped`
- `failed`

Interpretation guidance:
- `proving`: the demo ran and produced the intended proof surface
- `non_proving`: the demo ran but did not produce the expected proof
- `skipped`: the demo was intentionally not run because of an explicit gate or missing allowed prerequisites
- `failed`: the demo should have run but the command or proof surface failed

### 5. Stop Boundary

Stop after:
- one bounded demo execution surface
- one evidence-backed classification
- one recorded artifact

Do not:
- fix the demo implementation in the same skill pass
- build release-evidence packages
- janitor unrelated CI or milestone state

## Output Expectations

Default output should include:
- demo target
- command run
- prerequisite state
- classification
- produced artifacts
- follow-up recommendation

When ADL expects a structured artifact, follow `references/output-contract.md`.

## Design Basis

Within this skill bundle, the operational details live in:
- `references/demo-playbook.md`
- `references/output-contract.md`

The operator-facing invocation contract lives in:
- `/Users/daniel/git/agent-design-language/adl/tools/skills/docs/DEMO_OPERATOR_SKILL_INPUT_SCHEMA.md`

Prefer the tracked repo copies of these docs over memory when the bundle evolves.
