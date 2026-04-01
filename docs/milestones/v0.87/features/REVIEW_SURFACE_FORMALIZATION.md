

# REVIEW SURFACE FORMALIZATION v1

## Metadata
- Owner: `adl`
- Status: `draft`
- Target milestone: `v0.87`
- Work package: `WP-11`
- Purpose: Define the canonical review surface for ADL so findings, evidence, and action guidance are emitted in a bounded, deterministic, and reviewer-usable form.

## Purpose

Define the **review surface** as a first-class output substrate in ADL.

This document specifies:
- what a review surface is
- what sections it must contain
- how review outputs relate to trace, artifacts, skills, and cards
- how automated and human review surfaces should stay structurally compatible

This is the **feature-owner doc** for review-surface formalization in `v0.87`.

## Core Principle

> A review surface is not commentary. It is a structured, bounded record that turns execution and inspection into findings, evidence, and next-step guidance.

A valid review surface must be:
- truthful
- bounded
- evidence-bearing
- reviewable by a human without hidden context
- stable enough for automated consumption

## Why This Matters in `v0.87`

`v0.87` is a substrate milestone focused on coherence and reviewer-facing credibility.

Without formalized review surfaces, the system risks:
- ad hoc narrative reviews
- inconsistent findings formats
- unclear severity meaning
- missing evidence or trigger conditions
- reviews that cannot be compared across issues or milestones

A canonical review surface is required so that:
- trace can be interpreted consistently
- skills can emit usable findings
- issue output cards can embed or link structured review results
- internal and 3rd-party review can inspect the same truth surface

## Scope

### In scope
- canonical structure for review outputs
- severity model
- required fields for findings
- system-level assessment section
- action-plan section
- relationships to trace, artifacts, cards, and reports
- compatibility between manual review and skill-generated review

### Out of scope
- full UI for review browsing
- policy/governance adjudication beyond bounded findings/output
- later aptitude/governance/social review models

## Review Surface Definition

A review surface is the canonical bounded output of a review operation.

A review operation may be:
- human-driven
- skill-driven
- CLI/report-driven
- CI-driven

All of these must converge on the same structural output family.

## Design Goals

1. **Stable structure**
   - The required sections and finding fields must remain stable.

2. **Evidence first**
   - Every nontrivial finding must point to concrete evidence.

3. **Deterministic shape**
   - Wording may vary, but shape and required sections must not drift.

4. **Human + machine compatibility**
   - A human should be able to read it.
   - A tool should be able to parse it.

5. **Boundedness**
   - A review should end with explicit status and next-step guidance.

## Canonical Review Surface Structure

Every review surface should contain the following major sections in order:

1. `Metadata`
2. `Scope`
3. `Findings`
4. `System-Level Assessment`
5. `Recommended Action Plan`
6. `Follow-ups / Deferred Work`
7. `Final Assessment`

These may be represented in:
- markdown with stable headings
- structured YAML/JSON plus a rendered markdown view
- issue output cards embedding the same sections

## Metadata Section

Required metadata fields:
- `review_type`
- `subject`
- `scope`
- `reviewer`
- `date`
- `input_surfaces`
- `output_location` (if persisted)

Examples of `review_type`:
- `repo_review`
- `card_review`
- `trace_review`
- `demo_review`
- `release_review`

## Scope Section

The scope section must state:
- what was reviewed
- what was not reviewed
- whether the review is code-focused, docs-focused, trace-focused, or mixed
- whether the review is pre-merge, post-merge, or release-tail

This prevents findings from being overinterpreted.

## Findings Section

The findings section is mandatory.

Each finding must include:
- `id`
- `severity`
- `title`
- `location`
- `description`
- `impact`
- `trigger`
- `evidence`
- `fix_direction`

### Severity Model

Canonical severity levels:
- `P1` — correctness / contract violation / release-blocking defect
- `P2` — trust / security / integrity / serious design flaw
- `P3` — architectural risk / maintainability / non-blocking design debt
- `P4` — hygiene / portability / lower-severity cleanup

### Evidence Requirements

Every significant finding must cite concrete evidence such as:
- file path + function or block
- trace event IDs
- artifact references
- validator output
- command output

### Trigger Requirements

Every significant finding should include a concrete trigger or reproduction condition.

This is especially important for:
- correctness defects
- trust-boundary issues
- workflow/control-plane issues

## System-Level Assessment

This section is required.

It must summarize:
- dominant risk themes
- clustering of findings
- implications for system maturity
- what the findings say about the subsystem being reviewed

This prevents the review surface from becoming only a list of isolated defects.

Examples of acceptable themes:
- control-plane contract correctness
- provider attribution drift
- trace completeness gaps
- shell ownership fragility

## Recommended Action Plan

This section is required.

It must separate findings into action bands such as:
- fix immediately
- fix before milestone closeout
- defer to later milestone

This section should make execution sequencing clear without rewriting the roadmap.

## Follow-ups / Deferred Work

This section captures:
- explicit deferrals
- ownership or follow-up location
- known non-blocking debt

Deferrals must not disappear into prose.

## Final Assessment

This section should answer:
- Is the reviewed surface trustworthy?
- Is it ready for the next gate (merge, internal review, 3rd-party review, release)?
- What remains before approval?

The output should end in a bounded conclusion, not trail off.

## Review Surface Variants

The same canonical structure should support several bounded variants.

### 1. Skill-generated review

A skill-generated review must:
- follow the canonical structure
- preserve severity/evidence/trigger fields
- remain bounded and explicit

### 2. Human-authored review

A human-authored review may use freer wording, but must preserve:
- the same major sections
- the same finding fields
- the same severity model

### 3. Card-embedded review

Issue output cards may embed a compact version of the surface if they still preserve:
- findings
- evidence
- final assessment

### 4. Report/CLI review

CLI or report-generated reviews may serialize the canonical fields into JSON/YAML and render markdown from them.

## Integration with `v0.87` Substrates

### Trace
- reviews should cite trace events or artifact references when available
- trace review surfaces should be structurally compatible with repo/code reviews

### Operational Skills
- review-oriented skills must emit this surface shape
- examples include `card-review` and `repo-review`

### Issue Cards
- input/output cards may link or embed review surfaces
- output cards should summarize how findings were handled

### Demo Matrix
- demos that produce findings or inspection outputs should reference the same review structure

### `.adl/reports/`
- persisted review reports should use this canonical structure so they remain comparable across issues and milestones

## Determinism Requirements

A review surface is deterministic when:
- required sections are always present
- finding fields are always present when a finding exists
- severity model is stable
- ordering is stable enough for comparison

Allowed variability:
- exact prose wording
- human-authored explanation depth

Not allowed:
- silently missing sections
- missing evidence for serious findings
- changing severity semantics between reviews

## Acceptance Criteria

The review-surface formalization is acceptable for `v0.87` when:
- a canonical review structure is defined
- the P1–P4 severity model is defined and used consistently
- every major review surface can emit findings with evidence and triggers
- system-level assessment and action-plan sections are required
- the same structure works for both skill-generated and human-authored review
- output cards and `.adl/reports/` can reference or embed the same review family without contradiction

## Open Questions

- Should the canonical source of truth be markdown-first, schema-first, or dual-format?
- What is the first required structured serialization format, if any?
- Which `v0.87` reviews are mandatory to claim review-surface success?

## Non-Goals (v1)

- advanced statistical review analytics
- governance/policy adjudication engine
- a full review UI
- general-purpose external reporting platform

## Next Steps

Derive or align the following from this doc:
- concrete review skill docs (`card-review`, `repo-review`)
- review output template/schema doc
- trace review report surfaces
- issue output-card conventions for embedding or linking findings

Review surface formalization makes ADL review outputs consistent, comparable, and credible enough for real internal and external evaluation.