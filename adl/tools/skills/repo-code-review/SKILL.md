---
name: repo-code-review
description: Review an entire repository or large repo slice for bugs, regressions, security risks, correctness issues, maintainability risks, misleading diagnostics, and missing tests across all severity levels. Use when the user asks for a code review of a whole repo, wants repo-wide risk assessment before merge or release, or needs findings-first review output across multiple files.
---

# Repo Code Review

Review repositories with a code-review mindset first, not an implementation mindset. Optimize for finding correctness, regression, security, operability, and maintainability issues across the full severity range before suggesting polish.

Bias the review toward the actual executable codebase first. In mixed repos, treat docs as supporting context unless the user explicitly asks for doc review or the docs define critical invariants that the code claims to implement.

Always include build, dependency, and package configuration in the review surface. Do not treat files such as `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`, workspace manifests, toolchain files, Dockerfiles, CI workflow files, or lockfiles as incidental just because they are not executable source.

## Quick Start

1. Confirm the review target:
   - current repo
   - specific path
   - branch or diff if one is provided
2. Build a deterministic inventory before reading deeply:
   - run `scripts/repo_inventory.py` against the repo root
   - identify likely product code, manifests, dependency/build config, tests, generated code, and vendor code
   - identify the largest code files and largest files overall
3. Prioritize high-signal surfaces:
   - top-level manifests, workspace manifests, dependency declarations, and build/toolchain config before deep code reads
   - executable source files before docs
   - entrypoints and routing
   - auth, permissions, secrets, signing, or trust boundaries
   - persistence and migrations
   - concurrency, retries, cancellation, and state transitions
   - serialization, parsing, and external I/O
   - tests covering critical paths
4. After the high-risk pass, perform a second sweep for lower-severity but real issues:
   - misleading error classification
   - unsafe or leaky artifact paths
   - resume/recovery inconsistencies
   - weak diagnostics or observability gaps
   - portability and privacy footguns
   - stale or contradictory tests
   - overlarge files, modules, or test files that create maintenance risk
5. Run targeted tests when they are repo-local, reasonably bounded, and directly validate reviewed behavior.
6. Emit findings first, ordered by severity, with file references and concrete reasoning.

## Review Standard

Default to a production code review standard:
- look for behavioral bugs
- look for security or trust-boundary mistakes
- look for failure-mode gaps
- look for missing validation or missing tests around risky code
- look for regressions caused by partial refactors or drift between code and docs
- look for lower-severity but still real issues such as misleading diagnostics, privacy leaks, portability hazards, and path-handling drift

Do not spend most of the review on style nits unless the user explicitly asks for style feedback.

## Workflow

### 1. Scope the Review

Determine whether the user wants:
- the whole repository
- a subtree
- a branch or PR-style review
- a release-readiness scan

If no narrower scope is provided, review the repository root as a whole.

### 2. Build the Repo Inventory

Run `scripts/repo_inventory.py <repo-root>` to get a stable summary of:
- dominant file types
- likely application roots
- likely code roots
- test directories
- docs-only areas
- ignored/generated/vendor-heavy areas
- largest files overall
- largest code files by line count

Use the inventory to avoid spending too much time in:
- `node_modules`
- lockfiles
- vendored dependencies
- generated artifacts
- build outputs

Use the inventory to focus first on:
- top-level manifests and workspace/package config
- `largest_code_files`
- likely code roots
- entrypoint-bearing modules

### 3. Choose Reading Order

Prefer this order when the repo is large:
1. top-level manifests and executable entrypoints
2. dependency, build, toolchain, packaging, and CI configuration
3. core runtime modules
4. largest code files and most central modules
5. stateful/storage/integration code
6. security-sensitive code
7. tests for the above
8. lower-risk support code
9. artifact, logging, export, and recovery surfaces that often hide P3-P5 issues
10. docs only as support for intended invariants

If the repo has architecture docs or a security doc, skim those early to understand intended invariants.

### 4. Evaluate by Risk

For each important area, ask:
- What invariant is this code trying to preserve?
- What input or state transition can violate that invariant?
- What happens on malformed input, retries, cancellation, partial failure, or restart?
- Is there a missing test for the risky path?
- Does the implementation still match the documented behavior?

For manifest and config files, also ask:
- Do dependency versions, feature flags, and workspace wiring match the code's assumptions?
- Are build, release, CI, and toolchain settings likely to change runtime behavior or security posture?
- Are there risky defaults, missing hardening flags, surprising optional features, or stale dependency declarations?
- Does the lockfile or resolved dependency story contradict the intended dependency policy?

Then ask a second-pass question:
- Even if this is not a release-blocking bug, does it create misleading behavior, poor diagnostics, portability risk, privacy leakage, confusing policy behavior, or future maintenance risk?

Then ask a maintainability question:
- Is this file or module large enough that size alone is increasing review risk, coupling, or change hazard?

### 5. Produce Findings

Each finding should include:
- priority level
- affected file and line
- what is wrong
- why it matters in behavior terms
- what scenario triggers it

Prefer concrete findings such as:
- incorrect conditionals
- stale assumptions after refactor
- state-machine holes
- missing error handling
- inconsistent validation
- risky manifest or feature-flag configuration
- dependency or build configuration drift
- unsafe path or shell handling
- race-prone logic
- incorrect serialization or parsing
- missing rollback or cleanup

Treat these as valid lower-severity findings when they are concrete:
- incorrect or misleading error taxonomy
- unnecessary retries on deterministic failure
- host-path leakage in durable artifacts
- unsafe or insufficient identifier/path normalization
- recovery/resume behavior that trusts mutable state too early
- documentation or test cases that encode contradictory behavior
- overlarge source files or tests with enough breadth that they impair safe review and change isolation

Use the full priority range when warranted:
- `P0`: critical security/data-loss/correctness breakage
- `P1`: high-impact bug or policy failure
- `P2`: meaningful correctness, safety, or operational issue
- `P3`: moderate reliability, observability, or maintainability issue
- `P4`: low-severity but real issue with concrete downside
- `P5`: very low-severity but still actionable issue; use sparingly and only when non-speculative

Do not invent P4/P5 filler. Report them only when they are real, concrete, and still worth fixing.

File-size findings are valid when the size creates a real engineering downside, for example:
- too large to review safely in one pass
- mixes unrelated responsibilities
- encourages hidden coupling
- makes targeted testing difficult
- repeatedly acts as a dumping ground for unrelated logic

### 6. Validate with Tests When Feasible

Prefer running a small, targeted validation step when all of the following are true:
- the repo has a clear local test command
- the command is scoped to reviewed behavior
- runtime cost is reasonable
- no extra approvals or external systems are required

Examples:
- one Rust test module related to the reviewed subsystem
- one package test target
- one focused shell test script

If tests are too broad, too slow, flaky, unavailable, or require external services, do not fake coverage. State that they were not run and why.

### 7. State Residual Risk

If no significant findings are found, say so explicitly and mention residual review limits such as:
- not all paths were executed
- no runtime validation was performed
- generated code was skipped
- integration behavior remains unverified

## Output Format

Default output order:
1. findings
2. open questions or assumptions
3. short summary of review coverage
4. validation performed

Use the output contract in `references/output-contract.md` when ADL expects a structured artifact.

## Artifact Path

When the review should be written to disk, store it at:
- `.adl/reviews/<timestamp>-repo-review.md`

Use a filesystem-safe timestamp such as `YYYYMMDD-HHMMSS`.

If ADL provides a more specific output target, follow that target instead.

## Boundaries

This skill may:
- inspect repository files
- run bounded read-only discovery commands
- run the bundled inventory script
- compare code, tests, and documentation
- compare manifests, lockfiles, CI, and build configuration against code assumptions
- run targeted repo-local tests when they are relevant and bounded
- produce findings-first review output

This skill must not:
- silently rewrite code during review
- broaden into implementation unless the user asks
- claim execution coverage that was not actually performed
- treat generated or vendored code as first-class review targets unless the user asks
- spend most of the review budget on docs when executable code is present

## ADL Compatibility

This skill is Codex-compatible through `name` and `description` in frontmatter.

For stricter ADL execution, also use:
- `adl-skill.yaml` for machine-readable admission and output policy
- `references/review-playbook.md` for the expanded review procedure
- `references/output-contract.md` for structured result shape

## Resources

- Inventory helper: `scripts/repo_inventory.py`
- Detailed procedure: `references/review-playbook.md`
- Structured output contract: `references/output-contract.md`
