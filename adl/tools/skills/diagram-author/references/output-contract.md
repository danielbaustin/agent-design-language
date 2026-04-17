# Output Contract

The default `diagram-author` artifact is markdown with these sections in this order:

```md
## Metadata
- Skill: diagram-author
- Subject: <source packet, issue, code path, or doc path>
- Date: <UTC timestamp or calendar date>
- Output Location: <path or none>

## Target
- Mode: draft_from_source_packet | draft_from_issue | draft_from_code | review_or_revise_diagram
- Source: <path, issue, doc, code path, or inline target>
- Audience: <engineer | reviewer | user | stakeholder | unknown>
- Diagram Goal: <explain | prove | compare | debug | document | review>

## Diagram Decision
- Diagram Family: flowchart | sequence | state | dependency | data_flow | c4 | uml | concept_map | other
- Backend: mermaid | d2 | plantuml | structurizr | markdown_outline
- Rationale: <why this backend/family fits>

## Diagram Packet
- Primary Diagram Source: <path or inline fenced source>
- Rendered Artifact: <path or none>
- Result: PASS | FAIL | PARTIAL | NOT_RUN

## Truth Boundary
- Source-Backed Elements:
  - <bounded list or none>
- Assumptions:
  - <bounded list or none>
- Unknowns:
  - <bounded list or none>
- Unsupported Claims Added: true | false

## Render Validation
- Render Attempted: true | false
- Validation Command: <command or none>
- Validation Result: PASS | FAIL | NOT_RUN
- Notes: <bounded summary>

## Rendered Artifacts
- SVG Artifacts:
  - <path or none>
- Raster Artifacts:
  - <path or none>
- Source-Only Artifacts:
  - <path or none>
- Skipped Renderers:
  - <backend and reason or none>

## Publication Boundary
- Publication Attempted: true | false
- External Tool Upload Attempted: true | false
- Human Review Required: true | false
- Reason: <must explain why publication/upload is out of scope unless explicitly requested>

## Follow-up
- Recommended Next Step: <one bounded next action or explicit none>
```

## Rules

- Do not invent architecture, dependencies, data flows, or runtime behavior.
- Do not make UML the default when a simpler diagram communicates better.
- Do not hide assumptions; list them in the truth-boundary section.
- Do not claim a rendered artifact exists unless rendering actually ran.
- Prefer SVG for durable rendered artifacts and PNG only for raster-only surfaces.
- Do not emit raw secrets, raw prompts, raw tool arguments, or unjustified absolute host paths.
