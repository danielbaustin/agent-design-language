---
name: repo-architecture-review
description: Specialist architecture reviewer for CodeBuddy-style repository reviews focused on boundaries, layering, coupling, runtime/state models, architecture drift, and follow-up architecture tasks without taking over code, security, tests, docs, diagrams, ADR writing, fitness-function authoring, or synthesis roles.
---

# Repo Architecture Review

Review repository architecture as one specialist in the CodeBuddy multi-agent
review suite. This skill is findings-first and source-grounded. It may inspect
code, manifests, docs, and a CodeBuddy review packet, but it must not edit the
repository or claim merge approval.

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
   - `scripts/prepare_architecture_review.py <packet-root> --out <artifact-root>`
4. Inspect the high-signal architecture surfaces and write a findings-first
   review artifact.
5. Hand candidate diagram tasks to `diagram-author`, ADR candidates to
   `adr-curator` when available, fitness-function candidates to
   `architecture-fitness-function-author` when available, and final dedupe to
   `repo-review-synthesis`.

## Focus

Prioritize:

- module and package boundaries
- layering and dependency direction
- coupling between runtime, CLI, tests, demos, docs, and tooling
- state ownership, lifecycle transitions, and persistence boundaries
- concurrency, orchestration, and long-lived runtime boundaries
- architecture drift between code, docs, diagrams, and issue cards
- missing ADRs or implicit architecture decisions
- candidate architecture fitness checks that would prevent repeat drift

Defer primary ownership of these areas to other specialists:

- implementation correctness: `repo-review-code`
- security trust boundaries and abuse paths: `repo-review-security`
- missing test coverage: `repo-review-tests`
- documentation truth and onboarding drift: `repo-review-docs`
- diagram rendering or diagram source authoring: `diagram-author`
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
- `architecture_focus`

If there is no concrete repo or packet target, stop and report `blocked`.

## Workflow

### 1. Establish Scope

Record:

- review mode
- included architecture surfaces
- excluded surfaces
- assumptions and known limits
- whether the review is architecture-only or part of a multi-agent review

Do not silently expand a path or diff review into a whole-repo review.

### 2. Map Architecture Surfaces

Look for:

- top-level manifests and package boundaries
- CLI and runtime entrypoints
- service/module directories
- state, persistence, and artifact layout modules
- orchestration, workflow, scheduler, and lifecycle code
- docs that describe architecture, demos, roadmap, or milestones
- tests that encode architecture contracts
- existing diagrams and ADR-like records

### 3. Review For Architecture Findings

Findings should be behaviorally meaningful. Avoid style-only comments.

Use this priority scale:

- `P0`: architecture defect can cause data loss, unsafe execution, or severe
  trust-boundary failure
- `P1`: architecture drift or boundary failure can send operators or agents into
  the wrong lifecycle, persistence model, or integration path
- `P2`: coupling or missing architecture contract is likely to create recurring
  implementation or review defects
- `P3`: useful architecture hygiene issue with bounded follow-up value

Each finding should include:

- trigger scenario
- affected boundary or layer
- file/path evidence
- impact
- recommended follow-up owner

### 4. Emit Follow-Up Candidates

Include candidate follow-ups, but do not execute them:

- diagram task candidates
- ADR candidates
- architecture fitness-function candidates
- issue candidates

These are handoff notes, not created issues or authored diagrams.

## Output Expectations

Default output should include:

- findings first
- assumptions
- reviewed architecture surfaces
- architecture map summary
- candidate diagram tasks
- candidate ADRs
- candidate fitness functions
- validation performed or not run
- residual architecture risk

Use `references/output-contract.md` and the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`.

## Stop Boundary

Stop after producing the architecture-review artifact.

Do not:

- edit code, docs, tests, configs, diagrams, or ADRs
- silently run the whole multi-agent review workflow
- author diagrams directly
- author ADRs directly
- author fitness-function code directly
- create issues or PRs
- downgrade findings from other specialist roles
- claim approval, merge readiness, or remediation completion

## CodeBuddy Integration Notes

This skill consumes CodeBuddy packet artifacts and produces a specialist
architecture review artifact for synthesis. It is compatible with
`repo-packet-builder` and should run before `repo-review-synthesis` when the
operator wants architecture coverage as a first-class review lane.

Deferred automation:

- Architecture graph extraction from language-specific dependency analyzers.
- C4/UML/Structurizr generation handoff to `diagram-author`.
- Executable architecture fitness-function implementation by a dedicated
  follow-up skill.

