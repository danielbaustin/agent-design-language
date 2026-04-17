---
name: repo-dependency-review
description: Specialist dependency and supply-chain reviewer for CodeBuddy-style repository reviews focused on manifests, lockfiles, package manager configuration, toolchains, Docker and CI dependency setup, license-sensitive cues, dependency drift, and dependency-related test gaps without performing upgrades or remediation.
---

# Repo Dependency Review

Review repository dependency and supply-chain surfaces as one specialist in the
CodeBuddy multi-agent review suite. This skill is findings-first and
source-grounded. It may inspect code, manifests, lockfiles, Dockerfiles, CI
configuration, package-manager configuration, and a CodeBuddy review packet, but
it must not edit the repository, upgrade dependencies, or claim merge approval.

Use this skill after `repo-packet-builder` has created a bounded review packet,
or when an operator gives an explicit repo/path/diff scope.

## Quick Start

1. Confirm the target scope:
   - repository
   - path slice
   - branch
   - diff
   - existing review packet
2. Prefer a `repo-packet-builder` packet when available, especially
   `repo_scope.md`, `repo_inventory.json`, `evidence_index.json`, and
   `specialist_assignments.json`.
3. Run the deterministic scaffold helper when local packet access is available:
   - `scripts/prepare_dependency_review.py <packet-root> --out <artifact-root>`
4. Inspect dependency and supply-chain surfaces and write a findings-first
   specialist review artifact.
5. Hand final dedupe to `repo-review-synthesis`. Hand remediation ideas to a
   future issue or maintainer-owned upgrade plan, not to this skill.

## Focus

Prioritize:

- manifest and lockfile consistency
- unsupported, stale, overbroad, or risky dependency declarations
- package manager configuration and install determinism
- Docker, compose, devcontainer, and runtime image dependency setup
- CI workflow dependency bootstrap and cache behavior
- toolchain version drift across docs, manifests, CI, and lockfiles
- dependency-related test gaps, such as missing install or import smoke tests
- license-sensitive dependency cues that require human or legal follow-up
- supply-chain trust boundaries, including scripts, registries, vendored code,
  generated dependency files, and pinned versus floating versions

Defer primary ownership of these areas to other specialists:

- implementation correctness: `repo-review-code`
- security exploitation and abuse paths: `repo-review-security`
- general missing coverage: `repo-review-tests`
- documentation truth and onboarding drift: `repo-review-docs`
- architecture boundaries and layering: `repo-architecture-review`
- final cross-role dedupe and severity ordering: `repo-review-synthesis`

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
- `dependency_focus`

If there is no concrete repo or packet target, stop and report `blocked`.

## Workflow

### 1. Establish Scope

Record:

- review mode
- included dependency surfaces
- excluded dependency surfaces
- assumptions and known limits
- whether the review is dependency-only or part of a multi-agent review

Do not silently expand a path or diff review into a whole-repo review.

### 2. Map Dependency Surfaces

Look for:

- language manifests and lockfiles
- package manager configuration
- runtime images, Dockerfiles, compose files, and devcontainers
- CI setup, install, cache, and dependency audit steps
- generated or vendored dependency directories
- dependency metadata in docs, demos, examples, and setup scripts
- license files, notices, third-party attributions, and copied code markers
- tests that prove installability, imports, CLI startup, or packaging behavior

### 3. Review For Dependency Findings

Findings should be behaviorally meaningful. Avoid style-only comments.

Use this priority scale:

- `P0`: dependency flaw can enable severe compromise, unsafe execution, or
  broken release artifacts in normal use
- `P1`: install, packaging, lockfile, or supply-chain drift can block users,
  CI, releases, or trusted review results
- `P2`: dependency policy, pinning, cache, or test gaps are likely to cause
  recurring regressions or unreviewed supply-chain exposure
- `P3`: useful dependency hygiene issue with bounded follow-up value

Each finding should include:

- trigger scenario
- affected dependency surface
- file/path evidence
- impact
- recommended follow-up owner

### 4. Emit Follow-Up Candidates

Include candidate follow-ups, but do not execute them:

- upgrade or pinning issue candidates
- dependency policy candidates
- install or packaging test candidates
- license review candidates

These are handoff notes, not created issues or performed remediation.

## Output Expectations

Default output should include:

- findings first
- assumptions
- reviewed dependency surfaces
- dependency surface map
- candidate supply-chain findings
- candidate dependency test gaps
- candidate license review notes
- validation performed or not run
- residual dependency risk

Use `references/output-contract.md` and the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`.

## Stop Boundary

Stop after producing the dependency-review artifact.

Do not:

- edit code, docs, tests, configs, lockfiles, manifests, Dockerfiles, or CI
- install, upgrade, downgrade, pin, unpin, vendor, or remove dependencies
- run external vulnerability feeds or paid data sources
- silently run the whole multi-agent review workflow
- create issues or PRs
- perform license/legal determinations
- downgrade findings from other specialist roles
- claim approval, merge readiness, or remediation completion

## CodeBuddy Integration Notes

This skill consumes CodeBuddy packet artifacts and produces a specialist
dependency review artifact for synthesis. It is compatible with
`repo-packet-builder` and should run before `repo-review-synthesis` when the
operator wants dependency and supply-chain coverage as a first-class review
lane.

Deferred automation:

- Optional SBOM generation and parsing.
- Optional ecosystem-specific vulnerability database integrations.
- License policy allow/deny-list integration.
- Package-manager-specific lockfile consistency analyzers.
