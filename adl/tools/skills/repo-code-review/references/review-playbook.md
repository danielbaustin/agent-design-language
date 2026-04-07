# Repo Review Playbook

Use this file after the main skill triggers and you are ready to perform a repository-wide review.

## Review Priorities

Review in this order unless the user provides a narrower target:
1. top-level manifests, workspace manifests, and dependency/build/toolchain config
2. executable entrypoints and core code paths
3. trust boundaries and security-sensitive code
4. state transitions and persistence
5. failure handling and recovery
6. concurrency and ordering
7. API and serialization edges
8. tests for the above
9. artifact, export, logging, and resume surfaces
10. lower-severity operational and maintainability issues
11. docs as supporting context only

## High-Value Questions

Ask these questions while reading:
- Can invalid input cross a trust boundary?
- Do manifests, feature flags, dependency versions, and build settings match the code's assumptions?
- Can a partial failure leave state inconsistent?
- Can retries duplicate work or corrupt data?
- Can cancellation or restart violate invariants?
- Do docs and tests encode the same behavior as the code?
- Is there a critical branch with no test coverage?
- Is the error classification accurate, or does it hide the true failure mode?
- Does this path leak host details, reduce portability, or trust mutable state too early?
- Even if low severity, is the issue concrete and worth recording as P4 or P5?
- Is this source file or test file overlarge enough to create review and maintenance risk?

## Findings Threshold

A finding is worth reporting when it is:
- behaviorally wrong
- likely to regress in production
- security-relevant
- likely to break portability, upgrades, or rollback
- risky enough that a missing test is itself material
- operationally misleading in a way that will waste debugging time
- a concrete privacy, observability, or maintainability problem with real downside
- a file or module is so large that it materially raises change risk or hides responsibility boundaries

Avoid reporting low-signal nits unless requested.

Use P4 and P5 sparingly:
- `P4` for low-severity but concrete issues with a real operational or maintenance downside
- `P5` for very small but still actionable issues that are not merely stylistic

Do not report speculative, aesthetic, or preference-only comments as P4/P5.

## Output Shape

Write the review using the canonical section order from:
- `docs/tooling/review-surface-format.md`

For repo review, always include:
- `Metadata`
- `Scope`
- `Findings`
- `System-Level Assessment`
- `Recommended Action Plan`
- `Follow-ups / Deferred Work`
- `Final Assessment`

Findings remain first in substance, but the artifact must still include the metadata and scope sections ahead of them.

## Evidence Standard

Prefer:
- a specific path and line
- the runtime condition that exposes the issue
- the resulting wrong behavior

For manifest/config findings, prefer:
- the specific manifest or config path
- the dependency, feature, build, or CI setting at issue
- the concrete behavioral or operational downside

Avoid vague findings like "this may be confusing" unless the confusion creates a real operational risk.

## When No Findings Appear

If the review is clean:
- say explicitly that no significant findings were identified
- state what was reviewed
- state what was not validated at runtime

## Test Policy

Run tests when they are:
- local
- targeted to the reviewed subsystem
- reasonably fast
- not dependent on external services or extra approvals

Prefer a narrow command over a broad suite.

If no appropriate test was run, say that explicitly in the review artifact.

## Suggested Commands

Use fast repo-local commands such as:
- `python3 scripts/repo_inventory.py <repo-root>`
- `rg --files <repo-root>`
- `rg -n "<pattern>" <repo-root>`
- `wc -l <file>`

Prefer targeted reads after inventory instead of scanning every file in full.

Use the inventory output to find:
- top-level manifests and workspace config before deep code review
- likely code roots before docs
- `largest_code_files` for maintainability review
- `largest_files` to catch oversized non-code artifacts that may still affect reviewability

When available, add one targeted validation command such as:
- `cargo test <target> -- --nocapture`
- `pytest path/to/test_file.py`
- `npm test -- <pattern>`
