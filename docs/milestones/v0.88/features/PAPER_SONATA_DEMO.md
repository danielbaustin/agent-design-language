# Paper Sonata Demo

## Purpose

Define `Paper Sonata` as a flagship `v0.88` demo for ADL.

`Paper Sonata` is a bounded multi-agent manuscript-assembly workflow inspired by
PaperOrchestra, but the ADL value proposition is different:

- explicit workflow state
- traceable handoffs between roles
- durable intermediate artifacts
- bounded replay / re-run behavior
- truthful runtime evidence instead of opaque “paper in, paper out” magic

This demo is meant to be strong enough to show to investors, reviewers, and public audiences
without pretending to solve general scientific autonomy.

## Demo Thesis

`Paper Sonata` should demonstrate this claim:

ADL can orchestrate a bounded research-writing team that converts messy research materials
into a structured manuscript package while preserving execution traceability and reviewable
intermediate artifacts.

The point is not “AI writes papers by itself.”

The point is:
- ADL can coordinate a realistic high-value cognitive workflow
- every major stage stays inspectable
- the runtime tells the truth about what happened

## Why It Matters In v0.88

This is a particularly good `v0.88` demo because it exercises exactly the milestone’s themes:

- temporal continuity across a long artifact chain
- explicit execution posture and cost tradeoffs
- inspectable multi-step coupling and integration
- bounded role-based cognition
- reviewer-legible proof surfaces

It gives the milestone one flagship storyline that is more memorable than isolated substrate demos.

## Bounded MVP

The `v0.88` version should stay intentionally bounded.

### In scope
- one synthetic research packet
- one fixed workflow shape
- 4-5 bounded roles
- markdown-first artifact outputs
- deterministic or review-safe primary proof path
- explicit intermediate artifact retention
- normal ADL proof surfaces alongside the manuscript artifacts

### Out of scope
- arbitrary paper domains
- live open-web literature trust claims
- publication submission automation
- journal-ready bibliographic guarantees
- autonomous scientific discovery
- open-ended agent-society orchestration

## Input Contract

Minimum packet:
- `idea_summary.md`
- `lab_notes.md`
- `experiment_results.json`
- `target_venue.md`

Optional:
- `citations_seed.json`
- `paper_constraints.md`

Rules:
- inputs should be synthetic, fixture-based, or safely shareable
- the first version should not depend on private research material
- the demo should be runnable from tracked repo fixtures

## Output Contract

Minimum outputs:
- `plan.json`
- `outline.md`
- `literature_review.md`
- `results_summary.md`
- `review_notes.md`
- `paper_draft.md`
- `demo_manifest.json`

Likely section outputs:
- `sections/intro.md`
- `sections/method.md`
- `sections/results.md`
- `sections/discussion.md`

Optional outputs:
- `citations.json`
- `figures_spec.json`
- `figures/`
- `revision_requests.json`

Rules:
- output layout must be fixed and reviewable
- role outputs should remain attributable by stage
- the final package must not erase intermediate artifacts

## Recommended Role Set

### Conductor
Purpose:
- read the packet
- define the workflow order
- produce the outline and section briefs

### Scholar
Purpose:
- synthesize bounded related work from provided notes or citation seeds

### Analyst
Purpose:
- interpret experiment logs and identify what claims are actually supported

### Composer
Purpose:
- draft the paper package from the outline, literature review, and results summary

### Editor
Purpose:
- critique coherence, unsupported claims, structure, duplication, and clarity

## Review Surface

The demo should give reviewers:
- one obvious command to run
- one obvious artifact directory to inspect
- a manifest showing stage outputs and their relationships
- normal ADL trace / run evidence

## Success Criteria

The bounded `v0.88` version succeeds if:
- the same packet produces the same workflow structure and artifact layout
- the manuscript package is coherent enough to inspect seriously
- intermediate artifacts remain visible
- the editor role performs real critique rather than ceremonial approval
- the runtime proof surface is strong enough to compare favorably against opaque big-company demos

## Non-Goals

This demo should not be framed as:
- one-click publication-ready paper generation
- a trustworthy live-web literature engine
- human-free scientific authorship

The right framing is:

bounded multi-agent manuscript assembly with truthful runtime evidence.
