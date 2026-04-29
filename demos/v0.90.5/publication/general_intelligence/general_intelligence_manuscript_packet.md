# Manuscript Packet: A Mathematical Theory of General Intelligence

## Metadata

- Skill mode: `normalize_existing_manuscript`
- Issue: `#2645`
- Source packet: `demos/v0.90.5/publication/general_intelligence/general_intelligence_source_packet.md`
- Primary manuscript source: `.adl/docs/TBD/A_MATHEMATICAL_THEORY_OF_GENERAL_INTELLIGENCE.md`
- Status: normalized review packet
- Submission state: not submitted; not publication-ready

## Title Posture

Current strong title:

**A Mathematical Theory of General Intelligence**

Current subtitle:

**State-Space Compression as a Variational Principle Over Problem
Representations**

Reviewer-safe alternate title:

**State-Space Compression as a Variational Framework for General Intelligence**

## Safer Thesis

This paper should currently be read as proposing a candidate mathematical
framework in which intelligence is modeled as efficiency at discovering and
using task-relevant state-space compressions under explicit representation,
loss, and resource assumptions.

That is strong enough to matter and bounded enough to survive review.

## Abstract Normalization

The current manuscript proposes a mathematical framework for general
intelligence grounded in state-space compression (SSC). The key move is to treat
SSC not only as a modeling tool for coarse-graining dynamical systems, but as a
variational problem over representations relevant to prediction and action. In
this framing, intelligence is associated with the efficiency with which an
agent discovers, evaluates, and commits to useful representations under
resource constraints. The manuscript introduces task-relative cost structures,
defines Compression Complexity Cost (CCC) as a lower-bound style task
difficulty term under explicit assumptions, and defines task-local and
distributional intelligence measures relative to those structures. The paper is
currently strongest as a conceptual and mathematical framework. It is not yet a
finished empirical theory and should not yet claim fully demonstrated
cross-species or cross-substrate comparability without further examples and
formal hardening.

## What The Current Manuscript Already Has

- a clear central mathematical direction
- a serious SSC-based reframing of intelligence
- a usable Lagrangian/objective formulation
- definitions for CCC and task-local intelligence
- a path from task-local measures to general intelligence over task
  distributions
- a meaningful link to problem solving and representation choice

## What Still Needs Hardening

- explicit assumptions on the admissible representation class
- edge-case rules for zero-cost, infeasible, and stochastic tasks
- stronger distinction between definitions, conjectures, and proven
  propositions
- one or two toy examples that make CCC and the ratio metric behave visibly
- a bounded related-work section that positions the paper against Legg-Hutter,
  MDL, information bottleneck, computational mechanics, and bounded rationality

## Section Readiness Summary

### 1. Introduction

Strong. The motivation and representation-centered framing are clear. The main
revision need is tonal: claims about "a mathematical theory of intelligence"
should remain explicitly proposed rather than already established.

### 2. Background: State-Space Compression

Strong as a bridge section. It already explains the SSC setup clearly. It
should retain close fidelity to the Wolpert framing and avoid sounding like SSC
was originally intended as a finished intelligence theory.

### 3. From Compression To Variational Principle

This is the conceptual center of the paper. It is promising and likely worth
keeping, but it should be phrased as a formulation or extension rather than a
derived inevitability unless a fuller formal argument is added.

### 4. Prediction And Intervention

Important and directionally strong. This section benefits from grounding the
framework in action rather than passive modeling. It should keep the causal
intervention language careful and cited.

### 5. Representation Space

Good, but dependent on stronger admissibility assumptions. Reviewers will want
to know what counts as a representation in `R_tau`, how broad that class is,
and whether the metric depends too strongly on that choice.

### 6. Problem Complexity (CCC)

Potentially powerful, but the most vulnerable section. CCC should be described
as task-intrinsic only relative to declared encodings, losses, resource models,
and admissible representation classes. The random-system and simple-system
claims especially need examples or weakening.

### 7. Intelligence Of An Agent

Structurally good, but the ratio metric needs explicit edge-case treatment. The
paper should say what happens when tasks are trivial, impossible, or solved by
stochastic methods with high variance.

### 8+. General Intelligence, Learning, Hierarchy, And Scope

Directionally exciting, but these sections should remain explicit about what is
defined, what is conjectured, and what remains future formalization.

## Claim Boundary Report

| Claim | Label | Recommended stance |
| --- | --- | --- |
| SSC can be extended into a variational framework over representations. | SUPPORTED_AS_PROPOSAL | say "we propose" or "we formulate" |
| Intelligence is efficiency at discovering and exploiting useful compressions under constraints. | AUTHOR_DECISION | acceptable as the paper's proposed definition |
| CCC is intrinsic problem difficulty. | NEEDS_EVIDENCE | only relative to declared task/cost/representation assumptions |
| `I(agent, tau)` provides task-local intelligence. | SUPPORTED_AS_DEFINITION | keep with explicit denominator and failure rules |
| `I > 1` is impossible. | CONDITIONAL_ONLY | true only under matched admissible classes and exact CCC assumptions |
| The framework enables neutral comparison across humans, animals, and AIs. | REMOVE_OR_WEAKEN | keep as a possible comparison framework under explicit assumptions |
| The metric is unbiased. | REMOVE_OR_WEAKEN | too strong; replace with "less anthropomorphic under explicit assumptions" |
| The paper has already established a finished mathematical theory of intelligence. | REMOVE_OR_WEAKEN | current paper is still a framework paper with open hardening needs |

## Citation Posture

The paper now has a usable citation backbone, but the review packet should keep
the following visible:

- SSC/Wolpert is the primary anchor
- universal-intelligence and Legg-Hutter comparisons are necessary neighbors,
  not optional ornaments
- MDL, information bottleneck, computational mechanics, bounded rationality,
  and causal intervention work all matter to reviewer credibility

## Reviewer Questions

1. Should the strong title stay now that the manuscript has a more explicit
   comparison principle, or should the safer subtitle-led version be preferred
   until examples are added?
2. Which two toy examples would most efficiently de-risk the metric?
3. How much of the later ADL operational-intelligence story belongs in this
   paper versus the SSC/ADL follow-on paper?

## Recommended Next Revision Pass

1. keep the strong current manuscript, but annotate every major section with
   definition / conjecture / conditional proposition labels
2. add at least one trivial-task and one nontrivial-task worked example
3. keep the ratio metric, but make failure and zero-cost conventions explicit
4. sharpen the related-work comparison with Legg-Hutter and MDL without letting
   the paper get dragged into old benchmark philosophy
