---
name: repo-review-tests
description: Specialist test reviewer for a multi-agent repository review. Use when a review packet needs a bounded test-quality role focused on missing coverage, weak assertions, brittle fixtures, validation gaps, test isolation, flaky or overbroad tests, and whether risky behavior has executable proof without editing tests.
---

# Repo Review Tests

Review the test and validation surface as one specialist in the multi-agent repo
review suite.

This skill identifies coverage and validation risks. It does not write tests;
use `test-generator` later if remediation is requested.

## Quick Start

1. Confirm the repo, branch, path, diff, or review packet scope.
2. Identify risky behavior from the target and map it to existing tests.
3. Look for missing coverage, weak assertions, brittle fixtures, and validation
   commands that overclaim proof.
4. Emit findings first, with affected behavior and the missing proof.
5. Hand the artifact to `repo-review-synthesis` for cross-role assembly.

## Focus

Prioritize:
- risky code paths without direct tests
- weak assertions that only prove the happy path ran
- brittle fixtures, hidden ordering assumptions, and flaky timing
- tests that encode stale behavior or contradict docs
- validation commands that are too broad, too narrow, or misleading
- missing negative cases around parsing, permissions, retries, recovery, and IO

Defer primary ownership of these areas to other specialists:
- implementation defects: `repo-review-code`
- security exploitability: `repo-review-security`
- documentation truth: `repo-review-docs`
- final dedupe and ordering: `repo-review-synthesis`

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete target:
  - `target.target_path`
  - `target.branch`
  - `target.diff_base`
  - `target.review_packet_path`

Useful additional inputs:
- `changed_paths`
- `risky_behaviors`
- `test_commands`
- `exclude_paths`
- `validation_mode`

If there is no concrete repo or slice target, stop and report `blocked`.

## Output Expectations

Default output should include:
- findings first
- reviewed test surfaces
- missing proof map
- validation performed or not run
- residual test risk

Use the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md` when ADL expects
a structured artifact.

## Stop Boundary

Stop after producing the test-review artifact.

Do not:
- write tests or fixtures
- claim coverage that was not executed
- block on exhaustive test strategy when a bounded missing-proof finding is enough
- downgrade code or security findings from other specialists
