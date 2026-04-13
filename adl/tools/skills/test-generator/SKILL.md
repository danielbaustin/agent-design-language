---
name: test-generator
description: Generate or update focused tests for a concrete issue, diff, file, or worktree without taking over broader implementation. Use when the user wants bounded regression-test authoring, missing-test backfill for a specific surface, or the smallest truthful test additions needed to cover a concrete change.
---

# Test Generator

Generate tests with a bounded, issue-scoped mindset first, not a broad refactor mindset. The goal is to add or update the smallest truthful test surface that covers a concrete behavior, regression risk, or acceptance path.

This skill is allowed to write:
- tests
- fixtures
- snapshots
- narrowly related test harness helpers

It is not allowed to:
- silently absorb product-feature work
- rewrite unrelated test suites
- overclaim validation it did not run
- replace `pr-run` or broader issue execution orchestration

## Quick Start

1. Confirm the target surface:
   - issue
   - diff or changed paths
   - specific file or subsystem
   - bound worktree
2. Read the concrete implementation surface before writing tests.
3. Identify the smallest meaningful test gap:
   - missing regression
   - missing edge case
   - missing failure-path coverage
   - drift between code and existing tests
4. Prefer extending an existing nearby test file before creating a new one, unless a new file is clearly cleaner.
5. Generate only the bounded test additions needed for the target.
6. Run the smallest truthful validation set for the touched tests.
7. Stop after the test-writing surface is complete.

## When To Use It

Use this skill when:
- an issue already has implementation context and needs focused tests
- a change or diff exists and the missing test surface is concrete
- a review comment asks for tests
- the goal is to backfill one bounded regression or edge-case test

Do not use it when:
- there is no concrete target behavior to test
- the real task is broader feature implementation
- a repo-wide test strategy or large test reorganization is being requested
- the user wants a general code review instead of test writing

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete target:
  - `issue_number`
  - `diff_base`
  - `target_path`
  - `worktree_path`

Useful additional inputs:
- `changed_paths`
- `target_behavior`
- `acceptance_surface`
- `test_framework`
- `validation_mode`
- `allow_new_test_files`
- `allow_fixture_updates`

If there is no concrete target, stop and report `blocked`.

## Workflow

### 1. Confirm The Test Target

Resolve the narrowest trustworthy target first:
1. issue plus changed surface
2. explicit diff base and changed paths
3. explicit file or subsystem path
4. bound worktree with one intended task context

If the requested test surface is too vague, stop rather than inventing one.

### 2. Read The Existing Behavior Surface

Before writing tests, inspect:
- the changed or target implementation files
- existing nearby tests
- issue acceptance criteria or concrete bug description
- relevant manifests or test-runner config when needed

The skill should understand:
- what behavior is expected
- what regression is being guarded
- what test style the repo already uses

### 3. Choose The Smallest Test Shape

Prefer this order:
1. extend an existing nearby test
2. add one new focused test file near the subsystem
3. add a fixture or snapshot only if required by the test shape

Bias toward:
- one clear regression test over many speculative cases
- the repo's existing conventions over generic patterns
- explicit assertions over broad snapshot-only coverage

### 4. Write The Tests

Allowed writes include:
- test modules and files
- fixtures and snapshots
- narrowly related test helpers

Avoid:
- unrelated cleanup
- moving large test suites around
- editing production code unless the user explicitly asked for that and the issue scope includes it

### 5. Validate Truthfully

Run the smallest meaningful validation:
- one targeted Rust test module
- one shell regression test
- one package-specific test target

If the repo has no reasonably bounded local validation command, say so explicitly.

### 6. Stop Boundary

Stop after:
- focused test additions are written
- minimal validation is recorded
- the output artifact or summary is updated truthfully

Do not:
- auto-finish the PR
- janitor CI
- expand into unrelated implementation work

## Output Expectations

Default output should include:
- what target was tested
- what test files changed
- what behavior/regression those tests cover
- what validation was run
- any residual risk or follow-up test gaps

When ADL expects a structured artifact, follow `references/output-contract.md`.

## Design Basis

Within this skill bundle, the operational details live in:
- `references/test-playbook.md`
- `references/output-contract.md`

The operator-facing invocation contract lives in:
- `/Users/daniel/git/agent-design-language/adl/tools/skills/docs/TEST_GENERATOR_SKILL_INPUT_SCHEMA.md`

Prefer the tracked repo copies of these docs over memory when the bundle evolves.
