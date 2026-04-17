# Diagram Playbook

This skill stops at a reviewable diagram packet.

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

Examples:

```sh
mmdc -i diagram.mmd -o diagram.svg
d2 diagram.d2 diagram.svg
java -jar plantuml.jar -tsvg diagram.puml
structurizr validate -workspace workspace.dsl
```

If no renderer is available, provide the source and mark rendering as `not_run`.

## Accessibility And Review

- Include a short title and description in the packet.
- Prefer readable labels over compact internal names.
- Keep diagrams small enough to review; split large diagrams by view.
- Add legend/key notes when colors, shapes, or line styles have meaning.
- Prefer SVG for review and publication handoff.
