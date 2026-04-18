---
name: review-to-test-planner
description: Plan bounded test-generation tasks from CodeBuddy review findings, specialist review artifacts, or review packets by mapping findings to behavior under test, suggested test locations, fixtures, assertions, validation commands, and safe test-generator handoffs without writing tests or mutating repositories.
---

# Review To Test Planner

Plan follow-up test work from review findings. This skill sits between review
artifacts and `test-generator`: it translates findings into bounded test briefs,
but it does not write tests. In short, it is the bridge between review artifacts and `test-generator`.

Use this skill when CodeBuddy or an operator has findings from repo review,
security review, architecture review, dependency review, docs review, synthesis,
or third-party review and wants a source-grounded plan for what should be tested
next.

## Quick Start

1. Confirm the bounded review target:
   - review packet
   - specialist artifact directory
   - synthesis artifact
   - single review file
2. Prefer CodeBuddy packet artifacts when available:
   - `evidence_index.json`
   - `repo_inventory.json`
   - `run_manifest.json`
   - specialist review artifacts
3. Run the deterministic planner when local access is available:
   - `scripts/plan_review_tests.py <review-root> --out <artifact-root>`
4. Inspect the generated plan and tighten any findings that need human judgment.
5. Hand only safe, concrete tasks to `test-generator`. Stop before writing tests,
   fixtures, issues, PRs, or customer-repo changes.

## Focus

Prioritize:

- behavior under test, not generic coverage advice
- the smallest meaningful test target for each finding
- suggested test location and framework based on source path evidence
- fixture needs and negative cases
- expected assertions and validation commands
- whether the finding is safe for automated test generation
- explicit `test-generator` handoff blocks for safe tasks

Classify each task with one generation status:

- `generated`: a complete handoff brief already exists and can be passed to
  `test-generator` without additional planning.
- `recommended`: enough evidence exists to recommend a bounded test-generation
  task, but a human/operator should still decide whether to run it.
- `deferred`: more evidence, implementation context, or product decision is
  needed before test generation.
- `unsafe`: automated test generation should not proceed because the task would
  risk secrets, production systems, destructive behavior, privacy, or unbounded
  repo mutation.

Defer primary ownership of these areas:

- writing tests or fixtures: `test-generator`
- identifying original findings: review specialist skills
- fixing production code: implementation issue workflow
- creating issues from findings: finding-to-issue planner
- final synthesis/report writing: synthesis or report writer skills

## Required Inputs

At minimum, gather:

- `repo_root`
- one concrete target:
  - `target.review_packet_path`
  - `target.review_artifact_path`
  - `target.specialist_artifacts`
  - `target.findings_file`

Useful additional inputs:

- `artifact_root`
- `diff_base`
- `changed_paths`
- `test_framework_hint`
- `validation_mode`
- `generation_policy`
- `allowed_test_roots`
- `blocked_test_roots`

If there is no bounded review artifact or finding source, stop and report
`blocked`.

## Workflow

### 1. Establish Scope

Record:

- reviewed artifact paths
- finding sources considered
- evidence packet consulted
- test framework hints
- blocked paths or unsafe domains

Do not widen a single review artifact into a whole-repo test strategy unless the
input is explicitly a whole-repo review packet.

### 2. Map Findings To Behavior

For each finding, identify:

- priority and title
- affected file or subsystem
- behavior under test
- risk or regression scenario
- source evidence
- likely test framework and test location
- fixture/setup needs
- core assertions
- validation command

If the behavior or target file cannot be named concretely, mark the task
`deferred`.

### 3. Classify Generation Safety

Mark a task `unsafe` when test generation would require:

- real credentials, secrets, or production accounts
- destructive filesystem, database, network, billing, or deployment actions
- unbounded customer-repo mutation
- broad refactors rather than focused tests
- guessing behavior not supported by review evidence

Only safe tasks should include a ready `test-generator` handoff.

### 4. Emit Handoffs

For `generated` and `recommended` tasks, include a structured handoff that can
be reviewed before invoking `test-generator`.

The handoff should specify:

- `skill_input_schema: test_generator.v1`
- mode
- repo root
- target file/path/worktree/diff
- target behavior
- acceptance surface
- test depth
- fixture policy
- validation mode
- `stop_after_generation: true`

Do not invoke `test-generator` unless the operator explicitly asks for the
follow-on execution.

## Output Expectations

Default output should include:

- findings-to-test map
- generation-status summary
- test task briefs
- fixture and assertion map
- validation command plan
- safe `test-generator` handoffs
- deferred and unsafe tasks
- validation performed or not run
- residual test-planning risk

Use `references/output-contract.md` and the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`.

## Stop Boundary

Stop after producing the test planning artifact.

Do not:

- write tests, fixtures, snapshots, or production code
- mutate customer repositories
- create issues or PRs
- call `test-generator` automatically
- claim tests are generated when only a plan exists
- replace review specialists, synthesis, finding-to-issue planning, or
  implementation workflow

## CodeBuddy Integration Notes

This skill consumes CodeBuddy review packets and specialist artifacts. It
produces a review-to-test plan that can feed `test-generator`, synthesis,
product reports, or milestone follow-up triage.

Deferred automation:

- richer parsing for third-party PDF review extracts
- repository-specific framework discovery from package manifests
- direct confidence scoring from executed coverage data
- optional batch handoff execution through a separate conductor-approved flow
