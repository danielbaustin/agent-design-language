# C-SDLC Private Paper Repository Plan

Status: executed
Issue: #3218
Date: 2026-05-21

## Purpose

Create a private paper repository for **The Cognitive Software Development
Lifecycle** before the manuscript moves into TeX/PDF production and external
review.

The goal is to treat the C-SDLC paper as a serious scholarly project with its
own tracked history, claim/citation packet, manuscript sources, review artifacts,
and release gates.

## Pattern To Reuse

Use the same staged workflow as the private SSC/general-intelligence paper repo:

1. claim and citation hardening
2. arXiv-style manuscript shaping
3. formalization hardening
4. LaTeX conversion and compile QA
5. human review and submission decision

Reference local pattern:

- `<operator-local-papers-workspace>/general-intelligence-paper/GENERAL_INTELLIGENCE_PAPER_WORKFLOW.md`

## Proposed Private Repo

Working repo name:

```text
cognitive-sdlc-paper
```

Suggested local path:

```text
<operator-local-csdlc-paper-checkout>
```

GitHub repository:

```text
https://github.com/danielbaustin/cognitive-sdlc-paper
```

Suggested visibility:

```text
private
```

Suggested initial branch:

```text
main
```

## Initial Repository Layout

```text
cognitive-sdlc-paper/
  README.md
  C_SDLC_PAPER_WORKFLOW.md
  COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_DRAFT.md
  COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_CLAIM_AND_CITATION_PACKET.md
  COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_FORMAL_MODEL.md
  COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_ARXIV_REVIEW.md
  refs/
    README.md
    sources.bib
  figures/
    README.md
  tex/
    README.md
  reviews/
    README.md
```

## Initial Source Migration

Move or copy the current draft from:

```text
docs/cognitive-sdlc/c_sdlc_theory_paper_draft.md
```

into the private repo as:

```text
COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_DRAFT.md
```

The ADL repository should keep only a pointer or handoff note after migration,
not the authoritative evolving paper draft.

## Required First Commit Contents

The first private-repo commit should include:

- `README.md`
- `C_SDLC_PAPER_WORKFLOW.md`
- `COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_DRAFT.md`
- `COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_CLAIM_AND_CITATION_PACKET.md`
- `refs/sources.bib`
- `refs/README.md`
- `figures/README.md`
- `tex/README.md`
- `reviews/README.md`

## Workflow Phases

### Phase 1: Claim And Citation Packet

Before TeX work starts, create:

```text
COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_CLAIM_AND_CITATION_PACKET.md
```

It should classify each claim as:

- `DEFINITION`
- `FORMAL_MODEL`
- `THEORY_CLAIM`
- `SUPPORTED_BY_REFERENCE`
- `NEEDS_CITATION`
- `NEEDS_EVIDENCE`
- `AUTHOR_DECISION`
- `REMOVE_OR_WEAKEN`

Required claim areas:

- C-SDLC state model
- typed transition operator
- work-shard DAG
- read/write-set parallel safety
- serial coordination fraction
- Amdahl-style speedup with coordination overhead
- critical-path bound
- scheduler objective
- review gates as lifecycle control
- artifact contracts and mergeability
- cost-collapse hypothesis

### Phase 2: Formal Model Hardening

Create:

```text
COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_FORMAL_MODEL.md
```

It should refine:

- notation
- assumptions
- state tuple
- transition validity
- shard DAG semantics
- dependency relation
- merge operator
- invariants
- Amdahl extension
- serial coordination fraction
- scheduler objective
- measurable quantities

This phase should decide whether the formal model belongs in the main paper or
an appendix.

### Phase 3: Manuscript Shaping

Revise:

```text
COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_DRAFT.md
```

Goals:

- remove implementation-specific framing
- strengthen abstract and introduction
- make formal contribution crisp
- build a stronger related-work section
- move unproven claims into hypotheses or evaluation agenda
- convert diagrams into publication-ready figure plans

### Phase 4: TeX And PDF Production

Only after claim/citation and formal-model hardening, create:

```text
tex/COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE.tex
tex/references.bib
```

Then produce:

```text
tex/COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE.pdf
```

Compile success proves document mechanics only. It does not prove public
readiness.

### Phase 5: Review

Create:

```text
COGNITIVE_SOFTWARE_DEVELOPMENT_LIFECYCLE_ARXIV_REVIEW.md
```

Review should cover:

- claim strength
- citation adequacy
- mathematical clarity
- formal notation
- related-work positioning
- unsupported novelty claims
- diagram quality
- submission readiness

### Phase 6: Human Submission Gate

Before any public release:

- author approves title
- author approves claims
- author approves citations
- author approves final PDF
- author decides whether to submit, circulate privately, or hold

No automation should submit the paper or imply approval.

## ADL Repository Boundary

The ADL repository may keep:

- the original issue record for #3218
- a pointer to the private paper repo
- a migration note
- closeout truth for the issue

The ADL repository should not remain the canonical home for the evolving
manuscript once the private repo exists.

## Proposed #3218 Acceptance Update

#3218 should be considered complete when:

- private repo exists
- initial paper workflow exists in that repo
- current Markdown draft has been migrated into that repo
- claim/citation packet scaffold exists
- formal-model scaffold exists
- ADL issue record points to the private repo
- ADL no longer treats the in-repo draft as authoritative

## Non-Goals

- Do not create TeX before the claim/citation packet exists.
- Do not create PDF before the formal model has had a review pass.
- Do not submit to arXiv.
- Do not make ADL the subject of the paper.
- Do not keep serious paper review in chat-only context.

## Execution Result

Completed:

- private GitHub repository created
- local checkout initialized at `<operator-local-csdlc-paper-checkout>`
- initial paper workflow added
- current Markdown draft migrated into the private repo
- claim/citation packet scaffold added
- formal-model packet scaffold added
- bibliography scaffold added
- TeX, figures, and reviews placeholders added
- initial commit pushed to the private repo

## Next Step

Continue paper development in the private repository. The next substantive work
is claim/citation hardening and formal-model review before TeX/PDF production.
