# Source Packet: A Mathematical Theory of General Intelligence

## Metadata

- Packet: `general_intelligence_source_packet`
- Intended paper: `A Mathematical Theory of General Intelligence`
- Issue: `#2645`
- Status: normalization input for internal review
- Publication state: not submitted; not publication-ready

## Purpose

Normalize the already-live general-intelligence manuscript into the same
reviewable packet shape as the newer paper series, using the manuscript itself,
the claim/citation packet, and the prior arXiv review as primary evidence.

## Core Thesis

The paper proposes a candidate mathematical framework in which intelligence is
measured by an agent's efficiency at discovering and exploiting task-relevant
state-space compressions under explicit representation, loss, and resource
assumptions.

## Target Reader

- mathematically inclined AI and complex-systems readers
- reviewers interested in cross-substrate intelligence comparison
- readers willing to evaluate a framework paper rather than an empirical
  benchmark paper

## Primary Source Surfaces

### `.adl/docs/TBD/A_MATHEMATICAL_THEORY_OF_GENERAL_INTELLIGENCE.md`

Supported facts:

- the paper already has a serious mathematical skeleton
- the draft defines SSC-derived task/objective structures, CCC, task-local
  intelligence, and a distributional notion of general intelligence
- the current thesis is cross-substrate and representation-centered

### `.adl/docs/TBD/A_MATHEMATICAL_THEORY_OF_GENERAL_INTELLIGENCE_ARXIV_REVIEW.md`

Supported facts:

- the current draft is conceptually strong but not yet arXiv-ready
- major blockers are claim boundaries, citation support, formal assumptions, and
  examples
- several strong claims should be weakened from established result to proposed
  framework

### `.adl/docs/TBD/A_MATHEMATICAL_THEORY_OF_GENERAL_INTELLIGENCE_CITATION_AND_CLAIM_PACKET.md`

Supported facts:

- a safer one-line thesis already exists
- the claim map distinguishes definitions, conjectures, conditional
  propositions, and overclaims
- the current paper needs explicit admissible-representation assumptions,
  metric edge-case rules, and a stronger related-work pass

### `.adl/docs/TBD/GENERAL_INTELLIGENCE_PAPER_WORKFLOW.md`

Supported facts:

- the correct workflow order is claim/citation hardening, manuscript shaping,
  LaTeX conversion, then submission decisions
- the manuscript should not be treated as submission-ready merely because the
  `.tex` compiles

### `.adl/docs/TBD/intelligence/ADL_AND_INTELLIGENCE.md`

Supported facts:

- the broader ADL intelligence thesis is SSC-centered and cross-substrate in
  aspiration
- some stronger formulations in that note are still too broad for the paper and
  should remain inspiration rather than adopted claim text

### `.adl/docs/TBD/intelligence/ADL_INTELLIGENCE_METRIC.md`

Supported facts:

- ADL's later operational metric direction already emphasizes validation,
  cost, persistence, and improvement over time
- this helps clarify that the present paper is a framework/theory paper, not
  yet a finished runtime metric product

### `.adl/docs/whitepapers/The Many Faces of State Space Compression David Wolpert, Eric Libby, Joshua A. Grochow, and Simon DeDeo.md`

Supported facts:

- SSC is the paper's primary conceptual anchor
- the macrostate-choice and cost-tradeoff story should remain faithful to the
  Wolpert et al. framing

## Allowed Claims

| Claim | Status | Evidence |
| --- | --- | --- |
| The paper proposes a candidate mathematical framework for intelligence using SSC. | SUPPORTED | manuscript + review + claim packet |
| CCC and task-local intelligence are valid proposed definitions under declared assumptions. | SUPPORTED | manuscript + claim packet |
| The framework may support cross-substrate comparison when task, cost, and representation policies are explicit. | SUPPORTED_WITH_BOUNDARY | claim packet |
| The framework is already an unbiased, fully demonstrated intelligence metric. | REMOVE_OR_WEAKEN | prior review and claim packet reject this |
| The paper has already proven a final theory of intelligence. | REMOVE_OR_WEAKEN | should remain "we propose" / "we formulate" |

## Normalization Goals For This Issue

- preserve the current strong mathematical direction
- clearly separate definitions from conjectures and conditional propositions
- make the safer thesis the center of the review packet
- inherit the citation/claim packet instead of pretending the manuscript is
  already self-sufficient

## Citation And Evidence Gaps

- final verified bibliography still needs a reviewer-facing summary surface
- toy examples and sanity-check constructions remain a major missing ingredient
- explicit discussion of zero-cost, infeasible, and stochastic cases must stay
  visible in the review packet

## Recommended Packet Shape

1. normalized title posture
2. safer abstract
3. section-by-section readiness summary
4. claim boundary table
5. citation gap summary
6. explicit reviewer questions

## Boundary

This packet is for internal review normalization. It is not a new paper draft
from scratch, and it does not imply arXiv submission readiness.
