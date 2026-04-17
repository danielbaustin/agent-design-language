# Diagram Playbook

This skill stops at a reviewable diagram packet unless rendering is explicitly
requested by policy. Rendering still stops before external publication.

## Backend Selection

Use this default decision table:

| Goal | Preferred backend |
| --- | --- |
| GitHub/Markdown-native flow, sequence, state, ER, timeline, or simple architecture docs | Mermaid |
| C4 architecture model with multiple related views | Structurizr DSL |
| Polished explainer, demo visual, system map, or presentation-friendly diagram | D2 |
| Formal UML class, component, deployment, strict sequence, or timing diagram | PlantUML |
| Evidence is insufficient for precise structure | Markdown outline with gaps |

Mermaid is the default for ADL docs because it is easy to review in Markdown.
Structurizr DSL is preferred when the durable artifact is a C4 model rather
than a single picture. D2 is preferred when visual quality and layout matter.
PlantUML is a specialist path for formal UML, not the default.

## Render Policy

Use three output tiers:

1. Inline preview: Mermaid source for GitHub, PRs, issues, docs, and Codex chat.
2. Review packet: source files plus commands and truth-boundary notes.
3. Rendered asset packet: source files plus generated SVG/PNG artifacts when
   local renderers exist or rendering is explicitly required.

Prefer SVG as the durable rendered artifact. SVG scales cleanly and is suitable
for docs and review. Produce PNG only when the target surface cannot use SVG or
when the user explicitly asks for raster output.

Do not produce raster screenshots as the canonical artifact when source or SVG
would be reviewable.

## Diagram Families

- Flowchart: process, workflow, lifecycle, branching decisions.
- Sequence: interactions over time between agents, services, users, or tools.
- State: lifecycle states and allowed transitions.
- Dependency graph: modules, packages, skills, issues, or build relationships.
- Data-flow: data movement, storage, processing, and trust boundaries.
- C4: system context, container, component, dynamic, and deployment views.
- UML: class, component, deployment, timing, object, or strict sequence models.
- Concept map: explanatory relationships when a formal model is too heavy.

## Evidence Rules

- Source-backed nodes and edges should map to text, code, issue cards, docs, or runtime artifacts.
- Assumed nodes and edges must be labeled as assumptions.
- Unknown relationships should be called out in notes instead of drawn as fact.
- Security or trust-boundary edges require explicit evidence or a security-review caveat.
- Do not collapse two different concepts into one node just to simplify layout.

## Render And Validation Commands

Use commands only when the tool exists locally or the user explicitly asks to install/use it.

The bundled renderer harness is:

```sh
adl/tools/skills/diagram-author/scripts/render_diagrams.sh \
  --input <diagram-source-dir> \
  --out <artifact-dir> \
  --formats svg,png
```

Check local availability:

```sh
adl/tools/skills/diagram-author/scripts/render_diagrams.sh --check-tools
```

Examples:

```sh
mmdc -i diagram.mmd -o diagram.svg
d2 diagram.d2 diagram.svg
java -jar plantuml.jar -tsvg diagram.puml
structurizr validate -workspace workspace.dsl
```

If no renderer is available, provide the source and mark rendering as `not_run`.
If a renderer is unavailable and rendering is optional, mark the individual
source as `SKIP` rather than failing the whole packet.

## Accessibility And Review

- Include a short title and description in the packet.
- Prefer readable labels over compact internal names.
- Keep diagrams small enough to review; split large diagrams by view.
- Add legend/key notes when colors, shapes, or line styles have meaning.
- Prefer SVG for review and publication handoff.
