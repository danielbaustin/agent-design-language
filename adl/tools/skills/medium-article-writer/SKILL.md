---
name: medium-article-writer
description: Turn one concrete article brief into a reviewer-friendly Medium article packet without directly publishing. Use when the user wants bounded Medium-style drafting that enforces title quality, strong lead, section clarity, readability, and editorial notes while stopping before publication, platform posting, or scheduling.
---

# Medium Article Writer

Write one Medium-style article packet with a bounded editorial mindset.

This skill exists to turn one concrete article brief into a reviewable article
packet, not to run a publication program or publish to Medium.

This skill is allowed to:
- inspect one concrete article brief
- reuse the existing bounded Medium article writing demo surface
- draft one article packet with title options, outline, article draft, and editorial notes
- enforce Medium-oriented writing rules and reviewer-facing checks
- write one bounded review artifact

It is not allowed to:
- publish directly to Medium
- schedule posts or manage an editorial calendar
- silently widen into a content strategy project
- claim guaranteed reach, virality, or business outcomes

## Quick Start

1. Confirm the concrete article brief.
2. Read the existing Medium-writing demo surface first.
3. Identify the audience, claim, and reviewer expectations.
4. Produce one bounded article packet.
5. Record what was written and stop before publication.

## When To Use It

Use this skill when:
- one concrete article brief should become a reviewer-friendly Medium packet
- the operator wants Medium-style writing rules enforced consistently
- the output should be draft-oriented rather than publish-oriented

Do not use it when:
- there is no concrete brief
- the user wants direct platform publishing
- the task is a broad content program or editorial calendar
- the real task is just running the existing demo without creating a reusable skill

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete target:
  - `target.article_brief_path`
  - `target.article_brief_text`
  - `target.demo_doc_path`

Useful additional inputs:
- `artifact_root`
- `audience`
- `house_style`
- `forbidden_claims`
- `expected_sections`
- `validation_mode`
- `reviewer_mode`

If there is no concrete article brief, stop and report `blocked`.

## Workflow

### 1. Resolve The Writing Target

Prefer:
1. explicit brief path
2. explicit brief text
3. documented demo packet plus explicit brief override

If the brief is vague, stop rather than inventing a strategy document.

### 2. Inspect The Existing Writing Surface

Read:
- the Medium article writing demo doc
- the Medium article writing demo entrypoint
- the concrete brief

The skill should understand:
- who the article is for
- what claim the article makes
- what proof or examples the article should contain
- what would make the packet reviewable

### 3. Enforce Medium-Oriented Writing Rules

Bias toward:
- one sharp headline family, not many weak options
- a strong opening that earns the reader's attention quickly
- section clarity and readable pacing
- concrete examples over abstract filler
- editorial notes that call out risks, not just polish

Avoid:
- clickbait certainty
- vague “thought leadership” padding
- corporate boilerplate
- fake certainty about performance or audience reaction

### 4. Produce The Packet

The packet should normally include:
- article premise or angle
- title and subtitle options
- section outline
- article draft
- editorial notes
- publication caveats or reviewer notes

Prefer reusing the existing demo's bounded packet shape rather than inventing a
different hidden workflow engine.

### 5. Stop Boundary

Stop after:
- one bounded article packet
- one review artifact
- one explicit note that publication is out of scope

Do not:
- publish to Medium
- schedule the article
- claim the article is final without reviewer approval

## Output Expectations

Default output should include:
- target brief
- intended audience and angle
- packet contents produced
- Medium-rule checks applied
- publication boundary
- follow-up recommendation

When ADL expects a structured artifact, follow `references/output-contract.md`.

## Design Basis

Within this skill bundle, the operational details live in:
- `references/medium-writing-playbook.md`
- `references/output-contract.md`

The operator-facing invocation contract lives in:
- `/Users/daniel/git/agent-design-language/adl/tools/skills/docs/MEDIUM_ARTICLE_WRITER_SKILL_INPUT_SCHEMA.md`

Prefer the tracked repo copies of these docs over memory when the bundle evolves.
