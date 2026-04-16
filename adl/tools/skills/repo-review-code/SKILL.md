---
name: repo-review-code
description: Specialist code reviewer for a multi-agent repository review. Use when a review packet needs a bounded code/correctness reviewer focused on behavioral bugs, regressions, maintainability risks, API misuse, state transitions, concurrency, parsing, serialization, and implementation drift without taking over security, docs, tests, or synthesis roles.
---

# Repo Review Code

Review executable code for correctness and maintainability findings as one
specialist in the multi-agent repo review suite.

This skill is findings-only. It may inspect code and run bounded local tests,
but it must not edit code or claim merge approval.

## Quick Start

1. Confirm the repo, branch, path, diff, or review packet scope.
2. Identify entrypoints, core runtime modules, stateful logic, manifests, and
   the highest-risk implementation surfaces.
3. Review behavior before style.
4. Emit findings first, with file references, severity, and trigger scenario.
5. Hand the artifact to `repo-review-synthesis` when a multi-agent review is
   being assembled.

## Focus

Prioritize:
- correctness bugs and behavioral regressions
- API misuse and mismatched assumptions
- partial refactors and stale call sites
- state-machine, retry, cancellation, and recovery holes
- parsing, serialization, path handling, and external I/O behavior
- maintainability risks that materially increase review or change hazards

Defer primary ownership of these areas to other specialists:
- security threat and abuse analysis: `repo-review-security`
- missing or weak test coverage: `repo-review-tests`
- misleading docs and onboarding drift: `repo-review-docs`
- cross-role dedupe and final ordering: `repo-review-synthesis`

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
- `review_depth`
- `artifact_root`
- `exclude_paths`
- `validation_mode`

If there is no concrete repo or slice target, stop and report `blocked`.

## Output Expectations

Default output should include:
- findings first
- assumptions
- reviewed code surfaces
- validation performed or not run
- residual code-review risk

Use the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md` when ADL expects
a structured artifact.

## Stop Boundary

Stop after producing the code-review artifact.

Do not:
- edit implementation files
- silently run the whole multi-agent workflow
- downgrade security, docs, or test findings from other specialists
- claim approval, merge readiness, or remediation completion
