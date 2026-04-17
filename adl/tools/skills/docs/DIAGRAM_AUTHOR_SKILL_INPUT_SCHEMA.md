# Diagram Author Skill Input Schema

Schema id: `diagram_author.v1`

## Purpose

Provide one structured invocation shape for the bounded `diagram-author` skill.

The skill should turn one concrete source packet, issue, code slice, doc packet,
or existing diagram into a reviewable diagram-as-code packet while preserving
source truth and stopping before external publication.

## Supported Modes

- `draft_from_source_packet`
- `draft_from_issue`
- `draft_from_code`
- `review_or_revise_diagram`

## Top-Level Shape

```yaml
skill_input_schema: diagram_author.v1
mode: draft_from_source_packet | draft_from_issue | draft_from_code | review_or_revise_diagram
repo_root: /absolute/path
target:
  source_packet_path: <path or null>
  source_packet_text: <string or null>
  issue_number: <integer or null>
  code_path: <path or null>
  doc_path: <path or null>
  diagram_path: <path or null>
  artifact_root: <path or null>
  diagram_goal: explain | prove | compare | debug | document | review | unknown
  audience: engineer | reviewer | user | stakeholder | unknown
  preferred_backend: mermaid | d2 | plantuml | structurizr | auto
  required_diagram_family: flowchart | sequence | state | dependency | data_flow | c4 | uml | concept_map | auto
  forbidden_assumptions:
    - <string>
policy:
  backend_policy: auto_select | prefer_requested | require_requested
  truth_policy: strict_source_bound | mark_assumptions
  render_policy: source_only | render_if_tool_available | render_required
  validation_mode: syntax_only | render_check | artifact_only | none
  output_formats:
    - svg
    - png
  stop_before_publication: true
```

## Mode Requirements

### `draft_from_source_packet`

Requires:

- `target.source_packet_path`

Use when:

- a bounded packet of facts should become a diagram packet

### `draft_from_issue`

Requires:

- `target.issue_number`

Use when:

- an issue and its local ADL cards should become a diagram packet

### `draft_from_code`

Requires:

- `target.code_path`

Use when:

- one bounded code surface should become a dependency, flow, state, or interaction diagram

### `review_or_revise_diagram`

Requires:

- `target.diagram_path`

Use when:

- an existing diagram source should be reviewed, corrected, or revised against bounded evidence

## Policy Fields

- `backend_policy`
  - required
  - one of `auto_select`, `prefer_requested`, or `require_requested`
- `truth_policy`
  - required
  - one of `strict_source_bound` or `mark_assumptions`
- `render_policy`
  - required
  - one of `source_only`, `render_if_tool_available`, or `render_required`
- `output_formats`
  - optional
  - list containing `svg`, `png`, or both
  - default is `svg`
  - prefer `svg`; use `png` only when raster output is explicitly useful
- `validation_mode`
  - required
  - one of `syntax_only`, `render_check`, `artifact_only`, or `none`
- `stop_before_publication`
  - must be `true`

## Example Invocation

```yaml
Use $diagram-author at /Users/daniel/git/agent-design-language/adl/tools/skills/diagram-author/SKILL.md with this validated input:

skill_input_schema: diagram_author.v1
mode: draft_from_issue
repo_root: /Users/daniel/git/agent-design-language
target:
  source_packet_path: null
  source_packet_text: null
  issue_number: 2041
  code_path: null
  doc_path: null
  diagram_path: null
  artifact_root: .adl/reviews
  diagram_goal: document
  audience: engineer
  preferred_backend: auto
  required_diagram_family: auto
  forbidden_assumptions:
    - hidden runtime behavior
    - undocumented trust boundaries
policy:
  backend_policy: auto_select
  truth_policy: mark_assumptions
  render_policy: render_if_tool_available
  validation_mode: render_check
  output_formats:
    - svg
  stop_before_publication: true
```

## Notes

- prefer reviewable diagram source over image-only artifacts
- choose Mermaid for lightweight docs, Structurizr DSL for C4 model consistency,
  D2 for polished explainers, and PlantUML for formal UML
- use `adl/tools/skills/diagram-author/scripts/render_diagrams.sh` for local
  SVG/PNG generation when renderers are available
- keep assumptions, unknowns, and unsupported claims out of the diagram body or
  label them explicitly in the packet
