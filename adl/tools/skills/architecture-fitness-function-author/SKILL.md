---
name: architecture-fitness-function-author
description: Author bounded architecture fitness-function plans from CodeBuddy review packets, architecture reviews, findings, or repo evidence by separating machine-checkable invariants from human judgment and proposing dependency rules, forbidden imports, contract tests, docs checks, CI gates, validation commands, expected failure modes, and deferred automation without mutating repositories.
---

# Architecture Fitness Function Author

Author reviewable architecture fitness-function plans from architecture rules
and review findings. This skill turns recurring architecture risks into bounded checks where practical.

It is an authoring lane for fitness-function specifications and handoffs, not a
repo mutation lane. It may propose check logic, validation commands, failure
modes, and CI placement, but it must not edit customer repositories, CI files,
tests, or production code unless a separate implementation issue explicitly
authorizes that work.

## Quick Start

1. Confirm the bounded target:
   - CodeBuddy review packet
   - architecture review artifact
   - synthesis artifact
   - findings file
   - repo path with architecture policy evidence
2. Prefer packet artifacts when available:
   - `evidence_index.json`
   - `repo_inventory.json`
   - `run_manifest.json`
   - architecture review artifact
3. Run the deterministic authoring scaffold when local access is available:
   - `scripts/author_architecture_fitness_functions.py <source-root> --out <artifact-root>`
4. Inspect the emitted candidate checks and tighten the specs.
5. Hand implementation-ready checks to the appropriate downstream lane. Stop
   before editing tests, CI, docs, policies, issues, PRs, or customer repo files.

## Focus

Prioritize:

- dependency direction and layering rules
- forbidden imports or forbidden path relationships
- runtime lifecycle and state-transition contracts
- docs truth checks for architecture claims
- contract tests for architecture boundaries
- CI gate candidates with clear validation commands
- expected failure messages and false-positive risks
- separation of machine-checkable invariants from human-judgment review items
- deferred automation boundaries when a rule cannot be safely automated yet

Defer primary ownership of these areas:

- finding original architecture defects: `repo-architecture-review`
- writing tests: `test-generator`
- planning test work from general findings: `review-to-test-planner`
- creating follow-up issues: `finding-to-issue-planner`
- writing ADRs: `adr-curator`
- editing CI or repo policy files: implementation issue workflow
- final report synthesis: `repo-review-synthesis` or report writer skills

## Required Inputs

At minimum, gather:

- `repo_root`
- one concrete target:
  - `target.review_packet_path`
  - `target.architecture_review_artifact`
  - `target.findings_file`
  - `target.target_path`

Useful additional inputs:

- `artifact_root`
- `allowed_check_types`
- `ci_system`
- `validation_mode`
- `policy_scope`
- `implementation_allowed`
- `risk_tolerance`

If there is no bounded architecture rule, finding, or evidence source, stop and
report `blocked`.

## Workflow

### 1. Establish Scope

Record:

- source artifacts reviewed
- architecture surfaces consulted
- target repo/path scope
- assumed CI or validation environment
- whether implementation is allowed

Do not widen a single architecture finding into a whole-repo policy program.

### 2. Extract Candidate Invariants

Look for:

- boundary terms such as layer, adapter, runtime, CLI, persistence, state, and
  provider
- dependency terms such as import, dependency, manifest, package, module, and
  ownership
- contract terms such as schema, validation, policy, lifecycle, and state
- docs-truth terms such as command, README, guide, milestone, and architecture
  claim

For each candidate, decide whether it is:

- `machine_checkable`: can be expressed as a deterministic local check
- `human_judgment`: needs reviewer judgment or architectural decision first
- `deferred`: lacks source evidence, stable rule shape, or safe implementation

### 3. Author Fitness Function Specs

For machine-checkable candidates, include:

- rule id
- invariant statement
- source evidence
- check type
- suggested implementation surface
- validation command
- expected failure mode
- false-positive risks
- downstream owner

For human-judgment candidates, include:

- decision needed
- evidence gap
- suggested follow-up owner
- automation boundary

### 4. Emit Handoffs

Recommended handoffs:

- `test-generator` for contract-test implementation
- implementation issue workflow for CI gates or repo policy scripts
- `repo-architecture-review` for unresolved architecture evidence
- `adr-curator` for durable decisions that need an ADR before automation

Do not invoke downstream skills automatically unless the operator explicitly
asks for that follow-on execution.

## Output Expectations

Default output should include:

- fitness-function catalog
- machine-checkable invariants
- human-judgment candidates
- deferred automation boundaries
- validation command plan
- expected failure modes
- implementation handoffs
- validation performed or not run
- residual architecture risk

Use `references/output-contract.md` and the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`.

## Stop Boundary

Stop after producing the architecture fitness-function plan.

Do not:

- edit tests, CI, docs, production code, policy files, or issue state
- mutate customer repositories
- create issues or PRs
- claim a fitness function is installed when only a plan exists
- replace architecture review, ADR writing, test generation, or implementation
  workflow
- use network or paid services

## CodeBuddy Integration Notes

This skill consumes CodeBuddy packet artifacts and architecture review outputs.
It produces fitness-function plans that can feed test generation, implementation
issues, CI hardening, ADR work, or final synthesis/report writing.

Deferred automation:

- language-specific dependency graph extraction
- project-specific import-rule generation
- CI provider-specific gate patching
- coverage-linked confidence scoring
- batch execution of approved check implementations
