---
name: repo-diagram-planner
description: Plan source-grounded diagram tasks for CodeBuddy-style repository reviews from repo packets, specialist findings, architecture maps, dependency surfaces, and docs evidence, then hand bounded diagram briefs to diagram-author without authoring diagram source or inventing repo behavior.
---

# Repo Diagram Planner

Plan diagram work for a repository review without becoming the diagram author.
This skill turns bounded CodeBuddy packet evidence and specialist review outputs
into diagram task briefs that `diagram-author` can execute one at a time.

Use this skill after `repo-packet-builder` has created a bounded review packet
and after at least one specialist review artifact is available, or when an
operator supplies an explicit repo/path/doc packet that needs diagram planning.

## Quick Start

1. Confirm the target scope:
   - review packet
   - specialist review artifacts
   - path slice
   - issue or doc packet
2. Prefer CodeBuddy packet artifacts when available:
   - `repo_scope.md`
   - `repo_inventory.json`
   - `evidence_index.json`
   - `specialist_assignments.json`
3. Run the deterministic planning helper when local packet access is available:
   - `scripts/plan_repo_diagrams.py <packet-root> --out <artifact-root>`
4. Inspect the generated diagram task candidates and prune unsupported or
   duplicate tasks.
5. Hand selected tasks to `diagram-author` as bounded briefs. Stop before
   authoring diagram source, rendering, or publication.

## Focus

Prioritize diagram candidates that clarify:

- architecture boundaries and C4-style context/container views
- lifecycle, workflow, or orchestration paths
- state machines and long-lived runtime transitions
- data flows across trust boundaries
- module, package, skill, or dependency relationships
- issue/review handoff flows and specialist responsibility maps
- docs claims that need a visual proof or orientation aid

Prefer fewer, stronger diagrams over broad diagram inventory. Each candidate
must have evidence, a communication goal, a suggested diagram family, a suggested
backend, assumptions, unknowns, and a `diagram-author` handoff note.

## Required Inputs

At minimum, gather:

- `repo_root`
- one concrete target:
  - `target.review_packet_path`
  - `target.specialist_artifacts`
  - `target.target_path`
  - `target.issue_number`
  - `target.doc_path`

Useful additional inputs:

- `artifact_root`
- `audience`
- `diagram_goals`
- `max_tasks`
- `preferred_backends`
- `forbidden_assumptions`
- `validation_mode`

If there is no bounded source or target, stop and report `blocked`.

## Workflow

### 1. Establish Scope

Record:

- planning mode
- source packet or artifact paths
- included surfaces
- excluded surfaces
- audience and communication goals
- assumptions and known limits

Do not silently expand a path or diff planning pass into whole-repo planning.

### 2. Collect Diagram Evidence

Look for:

- architecture evidence and architecture review candidate diagram tasks
- runtime, workflow, lifecycle, state, orchestration, and scheduler surfaces
- security trust-boundary or data-flow findings
- dependency, package, and module relationship surfaces
- docs that describe user journeys, setup paths, or operational lifecycles
- existing diagrams and diagram sources

Use repo-relative or packet-relative paths only.

### 3. Select Diagram Families

Choose the smallest useful diagram family:

- `system_context` for audience orientation
- `container_or_component` for architecture boundaries
- `workflow` for process or lifecycle flows
- `sequence` for actor/service interactions over time
- `state` for lifecycle state machines
- `data_flow` for trust boundaries and data movement
- `dependency_graph` for module, package, or skill relationships
- `responsibility_map` for review or agent handoffs

Suggested backend defaults:

- Mermaid for GitHub-friendly diagrams and issue/PR review surfaces
- Structurizr DSL for C4 model families and multiple related architecture views
- D2 for polished explanatory system maps
- PlantUML for formal sequence/component/state diagrams
- Markdown outline when evidence is insufficient for diagram source

### 4. Emit Handoff Tasks

Each task should include:

- task id
- diagram family
- suggested backend
- audience
- goal
- source evidence paths
- assumptions and unknowns
- claims that must not be made
- renderer expectation
- `diagram-author` handoff brief

The planner may recommend tasks but must not execute `diagram-author`
automatically unless the operator explicitly requests that as a separate step.

## Output Expectations

Default output should include:

- findings or planning summary first
- diagram task list
- source evidence map
- diagram family/backend rationale
- assumptions and unknowns
- blocked or skipped candidates
- validation performed or not run
- handoff instructions for `diagram-author`
- residual planning risk

Use `references/output-contract.md` and the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`.

## Stop Boundary

Stop after producing the diagram plan.

Do not:

- author Mermaid, D2, PlantUML, Structurizr, SVG, or raster diagram artifacts
- render diagrams
- publish diagrams
- mutate customer repositories
- create issues or PRs
- replace `diagram-author`, architecture review, docs review, security review,
  dependency review, or synthesis
- invent architecture, runtime behavior, data flows, trust boundaries, or
  dependencies that are not backed by the provided evidence

## CodeBuddy Integration Notes

This skill consumes CodeBuddy packet artifacts and specialist review artifacts,
then produces a diagram planning artifact for downstream `diagram-author` runs.
It is a planning and routing lane, not a review lane and not a diagram rendering
lane.

Deferred automation:

- Reading specialist review artifacts directly to harvest finding-linked diagram
  tasks.
- Clustering duplicate diagram candidates across specialists.
- Producing one-click `diagram-author` invocation packets.
- Renderer availability probing through the `diagram-author` renderer setup
  contract.

