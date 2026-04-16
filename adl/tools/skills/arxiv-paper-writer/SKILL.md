---
name: arxiv-paper-writer
description: Turn one bounded scholarly source packet into a reviewer-friendly arXiv-style manuscript packet without submitting, publishing, inventing citations, or adding unsupported claims. Use when the user wants arXiv paper drafting, manuscript section drafting, citation-gap review, claim-boundary review, or a pre-submission review packet that must stop before publication.
---

# arXiv Paper Writer

Draft or revise one arXiv-style manuscript packet from a bounded source packet.

This skill exists to make scholarly drafting reviewable. It does not perform
literature search, invent citations, submit to arXiv, or claim publication.

This skill is allowed to:
- inspect one concrete source packet
- draft title, abstract, outline, sections, reviewer notes, and revision notes
- produce citation-gap and claim-boundary reports
- mark unsupported claims instead of laundering them into the draft
- write one bounded manuscript or review artifact

It is not allowed to:
- submit to arXiv or any publication platform
- invent citations, author lists, affiliations, acknowledgements, or results
- treat model-written prose as final author-approved text
- broaden into an unbounded research or literature-review task
- claim novelty, correctness, benchmark wins, or peer acceptance without source support

## Quick Start

1. Confirm the concrete source packet.
2. Identify the intended manuscript mode and target artifact.
3. Read `references/arxiv-writing-playbook.md` when drafting or revising text.
4. Read `references/output-contract.md` when writing a review packet.
5. Build a source-backed claim map before drafting.
6. Produce the bounded manuscript packet.
7. Stop before submission or publication.

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete target:
  - `target.source_packet_path`
  - `target.source_packet_text`
  - `target.demo_doc_path`

Useful additional inputs:
- `artifact_root`
- `paper_title`
- `paper_domain`
- `target_sections`
- `known_citations`
- `forbidden_claims`
- `author_approval_state`
- `validation_mode`

If there is no concrete source packet, stop and report `blocked`.

## Workflow

### 1. Resolve The Source Packet

Prefer:
1. explicit source packet path
2. explicit source packet text
3. demo document plus a declared source packet

The source packet should name the facts, results, citations, examples, and
scope boundaries that the manuscript may use.

If the packet is vague, ask for a source packet rather than creating an
unsourced paper.

### 2. Build The Evidence Map

Before drafting, identify:
- supported claims
- claims that need citations
- claims that need experimental evidence
- claims that must be weakened or removed
- citation gaps and missing bibliographic details
- author or affiliation fields that require human confirmation

Treat gaps as review notes, not as permission to invent.

### 3. Draft Or Revise The Packet

Depending on the mode, produce one bounded artifact:
- manuscript outline
- title and abstract options
- section draft
- full manuscript packet
- citation-gap report
- claim-boundary report
- reviewer response or revision packet

Prefer clear technical prose, explicit limitations, and source-backed claims
over grand framing.

### 4. Apply Publication Gates

Before returning the result, check:
- no arXiv submission was attempted
- no citation was invented
- unsupported claims are listed as gaps
- human authorship and approval are required before publication
- any web or live citation research is explicitly out of scope unless a future
  issue adds that step

### 5. Stop Boundary

Stop after:
- one bounded manuscript or review packet
- one explicit citation-gap and claim-boundary status
- one note that submission/publication is out of scope

Do not:
- publish, submit, schedule, or upload anything
- claim peer review, acceptance, or arXiv availability
- silently create unrelated issues, demos, or lifecycle artifacts

## Output Expectations

Default output should include:
- target source packet
- intended manuscript mode
- packet contents produced
- source-backed claim status
- citation-gap status
- submission boundary
- recommended next review step

When ADL expects a structured artifact, follow `references/output-contract.md`.

## Design Basis

Within this skill bundle, the operational details live in:
- `references/arxiv-writing-playbook.md`
- `references/output-contract.md`

The operator-facing invocation contract lives in:
- `/Users/daniel/git/agent-design-language/adl/tools/skills/docs/ARXIV_PAPER_WRITER_SKILL_INPUT_SCHEMA.md`

Prefer the tracked repo copies of these docs over memory when the bundle evolves.
