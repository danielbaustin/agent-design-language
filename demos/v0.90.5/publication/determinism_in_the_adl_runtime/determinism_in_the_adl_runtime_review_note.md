# Review Note: Determinism in the ADL Runtime

## Status

This first draft is ready for internal review.

It is not yet ready for external publication.

## What The Draft Does Well

- It defines determinism in a bounded, credible way instead of making impossible
  claims about perfect replay.
- It ties deterministic runtime behavior to trace, artifacts, and closeout
  truth, which is a real ADL differentiator.
- It explains why determinism matters specifically for agent systems rather than
  treating it as generic runtime hygiene.

## Main Remaining Gaps

### 1. External related work is still absent

The paper needs a later comparison pass against workflow engines, replay and
provenance systems, and adjacent agent-orchestration frameworks.

### 2. A compact worked example would improve it

One short issue/run/trace/artifact/closeout example would help readers
understand why ADL's determinism claim is practical rather than merely
architectural.

### 3. The control-plane argument should be checked for clarity

The idea that determinism extends into issue/PR/closeout truth is important,
but it may be new enough to readers that it benefits from one cleaner example
or figure.

## Recommended Next Revision Pass

1. add 4-8 external citations
2. add one worked runtime-plus-closeout example
3. add one small figure showing plan -> execution -> trace/artifact -> review
4. run one overclaim pass on replay language

## Publication Boundary

- submitted: no
- author-approved: no
- citation-complete: no
- initial-review-ready: yes
