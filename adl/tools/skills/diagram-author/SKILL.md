---
name: diagram-author
description: Create one source-grounded diagram packet from a bounded brief, issue, code slice, doc packet, or review surface. Use when the user wants architecture diagrams, workflow diagrams, dependency maps, sequence/state/data-flow diagrams, C4 views, Mermaid/D2/PlantUML/Structurizr sources, or diagram review notes without inventing system behavior.
---

# Diagram Author

Create or revise one source-grounded diagram packet.

This skill is a diagram-as-code and model-as-code router. It chooses the
smallest useful diagram family and source format for the communication goal,
then produces a reviewable diagram packet with assumptions and validation
notes.

This skill is allowed to:
- inspect one bounded source packet, issue, code slice, doc packet, or review surface
- choose a diagram family and backend: Mermaid, D2, PlantUML, Structurizr DSL, or no-render markdown
- draft diagram source plus rationale, assumptions, and validation commands
- render SVG and optional PNG artifacts when the requested renderer is already available locally or rendering is explicitly required
- write one bounded diagram packet or review artifact
- recommend rendering commands when the relevant tool exists locally

It is not allowed to:
- invent architecture, runtime behavior, trust boundaries, dependencies, or data flows
- treat UML as the default when a simpler diagram communicates better
- publish diagrams to external tools or docs sites without explicit separate approval
- claim a rendered artifact exists unless rendering actually ran
- silently replace architecture review, security review, or implementation proof
- broaden into repo-wide diagram inventory work without a bounded target

## Quick Start

1. Confirm the concrete source packet or target surface.
2. Identify the audience, communication goal, and required fidelity.
3. Select the diagram family and backend.
4. Read `references/diagram-playbook.md` when choosing diagram types or tools.
5. Read `references/output-contract.md` when writing a diagram packet.
6. Draft the diagram source with explicit assumptions and unknowns.
7. If rendering is requested, use `scripts/render_diagrams.sh` and record PASS, SKIP, or FAIL truthfully.
8. Add validation/render instructions.
9. Stop before publication or unbounded redesign.

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete target:
  - `target.source_packet_path`
  - `target.source_packet_text`
  - `target.issue_number`
  - `target.code_path`
  - `target.doc_path`

Useful additional inputs:
- `artifact_root`
- `diagram_goal`
- `audience`
- `preferred_backend`
- `required_diagram_family`
- `render_policy`
- `validation_mode`
- `forbidden_assumptions`

If there is no bounded source or target, stop and report `blocked`.

## Workflow

### 1. Resolve The Evidence Source

Prefer:
1. explicit source packet path
2. explicit source packet text
3. issue number plus local STP/SIP/SOR or linked docs
4. bounded code path
5. bounded doc path

Record source gaps instead of filling them with plausible architecture.

### 2. Choose Diagram Family

Choose from the communication goal:
- flowchart for process, workflow, or lifecycle
- sequence for actor/service interaction over time
- state for lifecycle state machines
- dependency graph for modules, packages, or issue relationships
- data-flow for movement of data across trust boundaries
- C4/Structurizr for architecture views that need model consistency
- UML/PlantUML for formal class, component, deployment, or strict sequence diagrams
- D2 for polished system maps, explainers, and presentation-ready visual maps
- Mermaid for GitHub-friendly markdown diagrams and lightweight docs

Do not choose formal UML just because the request says "diagram" unless the
user asks for UML or the source calls for formal modeling.

### 3. Choose Backend

Default backend order:
1. Mermaid for GitHub/Markdown-native docs and quick reviewable diagrams
2. Structurizr DSL for C4 architecture models and multiple related views
3. D2 for polished system maps and demo/presentation explainers
4. PlantUML for strict UML semantics or advanced UML diagram families
5. Markdown outline when evidence is insufficient for diagram source

Treat "renders in the current surface" as a first-class selection criterion.
If the user asks to show a diagram in GitHub Markdown, Codex chat, an issue, or
a PR body, prefer Mermaid unless another backend is explicitly required.

When the backend is chosen, explain why.

### 4. Draft The Packet

Include:
- diagram source
- backend and diagram family
- source evidence
- assumptions and unknowns
- validation/render commands
- accessibility notes such as title, description, and readable labels

### 5. Render When Requested

Rendering is optional unless `policy.render_policy` is `render_required`.

Use:

```sh
adl/tools/skills/diagram-author/scripts/render_diagrams.sh \
  --input <diagram-source-dir> \
  --out <artifact-dir> \
  --formats svg,png
```

Renderer behavior:
- Mermaid uses `mmdc`
- D2 uses `d2`
- PlantUML uses `plantuml`
- Structurizr DSL uses `structurizr validate` and `structurizr export`
- PNG derivation can use ImageMagick `convert` when SVG exists

If a renderer is missing and rendering is not required, record `SKIP` rather
than failing or inventing a rendered artifact. Prefer SVG as the durable visual
asset. Use raster formats such as PNG only for surfaces that cannot consume SVG.

### 6. Validate Boundaries

Before returning, check:
- no unsupported architecture or runtime claim was added
- assumptions are labeled separately from source-backed structure
- rendering/publishing did not happen unless explicitly requested
- selected backend matches the audience and doc surface
- the packet has a clear next review step

## Output Expectations

Default output should include:
- target source packet
- diagram goal and audience
- selected diagram family and backend
- diagram source or explicit reason no source was emitted
- assumptions and unknowns
- validation/render instructions
- publication boundary
- recommended next step

When ADL expects a structured artifact, follow `references/output-contract.md`.

## Design Basis

Within this skill bundle, the operational details live in:
- `references/diagram-playbook.md`
- `references/output-contract.md`

The operator-facing invocation contract lives in:
- `/Users/daniel/git/agent-design-language/adl/tools/skills/docs/DIAGRAM_AUTHOR_SKILL_INPUT_SCHEMA.md`

Prefer the tracked repo copies of these docs over memory when the bundle evolves.
