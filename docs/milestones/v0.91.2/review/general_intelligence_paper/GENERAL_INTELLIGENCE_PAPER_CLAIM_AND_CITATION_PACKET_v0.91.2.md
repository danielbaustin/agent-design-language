# General Intelligence Paper Claim And Citation Packet

## Status

Tracked `WP-14` claim/citation packet for the separate
`general-intelligence-paper` repository.

## Source Boundary

This packet summarizes the current manuscript state from the separate paper
repo, rather than treating the older `.adl/docs/TBD/general-intelligence-paper/`
copy in this repository as canonical.

Current canonical manuscript surfaces:

- `general-intelligence-paper/A_MATHEMATICAL_THEORY_OF_GENERAL_INTELLIGENCE.tex`
- `general-intelligence-paper/A_MATHEMATICAL_THEORY_OF_GENERAL_INTELLIGENCE.bib`
- `general-intelligence-paper/A_MATHEMATICAL_THEORY_OF_GENERAL_INTELLIGENCE_CITATION_AND_CLAIM_PACKET.md`
- `general-intelligence-paper/A_MATHEMATICAL_THEORY_OF_GENERAL_INTELLIGENCE_ARXIV_REVIEW.md`
- `general-intelligence-paper/GENERAL_INTELLIGENCE_PAPER_WORKFLOW.md`

## Current Thesis Posture

Safest current thesis:

> We formulate a candidate mathematical framework in which intelligence is
> measured by an agent's efficiency at discovering and using task-relevant
> state-space compressions under explicit representation, loss, and resource
> assumptions.

This remains the preferred public posture over stronger variants that imply a
fully settled theory of general intelligence.

## Claim Boundary Summary

Current claim classes retained from the paper repo packet:

- `SUPPORTED`
- `NEEDS_CITATION`
- `NEEDS_EVIDENCE`
- `AUTHOR_DECISION`
- `REMOVE_OR_WEAKEN`

Most important currently stable claims:

- state-space compression is the conceptual parent surface
- the paper proposes CCC as a task-relative difficulty measure
- agent intelligence is framed as ratio-to-task-optimum under shared policy
- the framework is explicitly resource-aware and task-relative

Most important still-bounded claims:

- universal or cross-substrate comparison claims must stay conditional
- claims of bias elimination or benchmark independence remain too strong
- claims that all problem solving is state-space compression remain too broad
- any claim that the paper has already "shown" a full mathematical theory is
  still too strong for public use

## Citation Posture

The manuscript now has a functioning `.bib` and clean citation build after a
full rebuild in the separate paper repo.

Primary citation backbone already identified:

- Wolpert / Libby / Grochow / DeDeo on state-space compression
- Cover and Thomas on information theory
- Tishby / Pereira / Bialek on information bottleneck
- Pearl on causality
- Sutton and Barto on reinforcement learning
- Legg / Hutter and AIXI-adjacent intelligence measures
- Rissanen / Grunwald on MDL
- Simon and related bounded-rationality sources
- Wolpert / Macready on no-free-lunch

## Current Mechanical State

- manuscript source update to `v4`: present in the separate repo
- bibliography repair: complete enough for a clean citation build
- PDF build: succeeds in the separate repo
- remaining build warnings: layout-level warnings may remain, but not the prior
  missing-citation failure mode

## Non-Claims

- This packet does not claim external peer review.
- This packet does not claim the theorem set is fully audited for correctness.
- This packet does not promote the paper to publication-ready canonical status.
