# Paper Sonata Implementation Plan

## Purpose

Translate `Paper Sonata` into a bounded `v0.88` implementation slice.

This plan exists so the demo can be executed as real work rather than admired as an idea.

## Position In The Milestone

In `v0.88`, `Paper Sonata` should function as a flagship demo and proof surface.

It is not a new core architecture band.
It is the best bounded showcase for the temporal, PHI, instinct, and artifact-truth work already in scope.

## MVP Goal

Ship one bounded ADL demo that turns a synthetic research packet into a manuscript package
through explicit multi-agent roles and emits truthful runtime evidence alongside the paper artifacts.

The MVP is successful if it proves:
- ADL can coordinate a research-writing workflow through explicit roles
- intermediate artifacts are visible and reviewable
- the final manuscript package is coherent
- the run produces normal ADL proof surfaces
- the demo does not overclaim citation grounding or publication readiness

## Likely Deliverables

### Demo runner
Likely location:
- `adl/tools/demo_v088_paper_sonata.sh`

### Test surface
Likely location:
- `adl/tools/test_demo_v088_paper_sonata.sh`

### Demo fixtures
Likely locations:
- `demos/fixtures/paper_sonata/`
- `artifacts/v088/paper_sonata/` during runs

### Optional workflow definition
Depending on implementation approach:
- explicit ADL workflow file
- or a deterministic wrapper that drives the runtime in a bounded way

## Implementation Slices

### 1. Fixture Packet
Create a synthetic research packet that is:
- plausible
- non-sensitive
- rich enough to generate meaningful sections and critique

### 2. Role-Orchestration Path
Implement the bounded sequence for:
- conductor
- scholar
- analyst
- composer
- editor

### 3. Artifact Layout
Guarantee a fixed and reviewable output tree with:
- section outputs
- review outputs
- final draft
- manifest and trace evidence

### 4. Proof / Review Surface
Make the demo legible enough for:
- milestone reviewers
- investor-facing screenshots or summaries
- public proof-of-capability posts

### 5. Validation
Add a bounded test or smoke path that ensures:
- expected files are produced
- the artifact layout remains stable
- the manifest is coherent

## Minimum Concrete Outputs

The first real implementation should not ship unless it produces all of:
- one runnable entrypoint
- one fixture packet checked into the repo
- one stable output tree with role-attributable intermediate artifacts
- one manifest that enumerates stages and outputs
- one smoke/test path that fails if the artifact contract drifts

## Key Risks

- the demo becomes overly ambitious and stalls
- the demo overclaims citation or publication authority
- the artifact set becomes messy and hard to review
- the workflow produces a draft but not a believable multi-agent story

## Mitigations

- keep the first version fixture-based and synthetic
- keep the role set small
- keep the output contract fixed
- require explicit intermediate artifacts
- treat the editor/review pass as mandatory, not decorative

## Exit Criteria

The `v0.88` implementation slice is complete when:
- one bounded runner exists
- one bounded fixture packet exists
- the artifact layout is fixed and reviewable
- the demo matrix points to the real proof surface
- the resulting package is strong enough to function as a flagship milestone demo
