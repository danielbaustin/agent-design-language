# arXiv Paper Writer Skill Input Schema

Schema id: `arxiv_paper_writer.v1`

## Purpose

Provide one structured invocation shape for the bounded `arxiv-paper-writer`
skill.

The skill should turn one concrete scholarly source packet into a
reviewer-friendly arXiv-style manuscript packet while stopping before
submission or publication.

## Supported Modes

- `draft_from_source_packet`
- `draft_section`
- `review_claims_and_citations`
- `revise_from_review_notes`

## Top-Level Shape

```yaml
skill_input_schema: arxiv_paper_writer.v1
mode: draft_from_source_packet | draft_section | review_claims_and_citations | revise_from_review_notes
repo_root: /absolute/path
target:
  source_packet_path: <path or null>
  source_packet_text: <string or null>
  demo_doc_path: <path or null>
  review_notes_path: <path or null>
  artifact_root: <path or null>
  paper_title: <string or null>
  paper_domain: <string or null>
  target_sections:
    - <section name>
  known_citations:
    - <citation supplied by the source packet>
  forbidden_claims:
    - <string>
  author_approval_state: unknown | draft_reviewed | approved_for_submission
policy:
  citation_policy: source_packet_only | mark_gaps
  claim_policy: strict_source_bound | mark_unsupported
  validation_mode: artifact_only | demo_aligned | none
  stop_before_submission: true
```

## Mode Requirements

### `draft_from_source_packet`

Requires:

- `target.source_packet_path`

Use when:

- the operator wants a bounded manuscript packet from a concrete local source packet

### `draft_section`

Requires:

- `target.source_packet_path`
- `target.target_sections`

Use when:

- only selected manuscript sections should be drafted or revised

### `review_claims_and_citations`

Requires:

- `target.source_packet_path`

Use when:

- the operator wants a citation-gap or unsupported-claim review before drafting

### `revise_from_review_notes`

Requires:

- `target.source_packet_path`
- `target.review_notes_path`

Use when:

- an existing manuscript packet should be revised against bounded review notes

## Policy Fields

- `citation_policy`
  - required
  - one of `source_packet_only` or `mark_gaps`
- `claim_policy`
  - required
  - one of `strict_source_bound` or `mark_unsupported`
- `validation_mode`
  - required
  - one of `artifact_only`, `demo_aligned`, or `none`
- `stop_before_submission`
  - must be `true`

## Example Invocation

```yaml
Use $arxiv-paper-writer at /Users/daniel/git/agent-design-language/adl/tools/skills/arxiv-paper-writer/SKILL.md with this validated input:

skill_input_schema: arxiv_paper_writer.v1
mode: draft_from_source_packet
repo_root: /Users/daniel/git/agent-design-language
target:
  source_packet_path: demos/v0.89.1/arxiv_manuscript_workflow_demo.md
  source_packet_text: null
  demo_doc_path: demos/v0.89.1/arxiv_manuscript_workflow_demo.md
  review_notes_path: null
  artifact_root: null
  paper_title: Agent Design Language manuscript packet
  paper_domain: agentic software engineering
  target_sections:
    - abstract
    - introduction
    - limitations
  known_citations: []
  forbidden_claims:
    - peer reviewed
    - submitted to arXiv
    - benchmark superiority without evidence
  author_approval_state: unknown
policy:
  citation_policy: mark_gaps
  claim_policy: mark_unsupported
  validation_mode: artifact_only
  stop_before_submission: true
```

## Notes

- prefer a smaller source-backed draft over a polished but unsupported paper
- keep arXiv submission outside the skill boundary
- use citation gaps and claim-boundary labels as first-class review output
