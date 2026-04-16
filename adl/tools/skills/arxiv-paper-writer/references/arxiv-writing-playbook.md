# arXiv Writing Playbook

Use this file after the main skill triggers and the source packet is concrete.

## Priorities

Prefer this order:
1. preserve source-packet truth
2. state the contribution clearly and narrowly
3. separate evidence-backed claims from hypotheses or motivation
4. surface citation gaps instead of filling them from memory
5. keep limitations visible
6. leave submission decisions to human authors

## Source Packet Checklist

Before drafting, identify:
- core problem statement
- contribution claim
- method or system description
- evaluation evidence
- limitations and failure modes
- known citations and missing citations
- author-approved terminology
- claims that are explicitly forbidden or not yet proven

## Manuscript Guidance

Bias toward:
- concise, testable contribution language
- clear abstract structure: problem, method, evidence, result, limitation
- section headings that reveal the argument
- related-work language that says what is known, unknown, or deferred
- limitations that reviewers can verify
- explicit TODO markers for author-owned facts

Avoid:
- inflated novelty claims
- invented comparisons to prior work
- fabricated benchmark results
- placeholder citations that look real
- claims that imply peer review, acceptance, or arXiv availability
- author, affiliation, funding, or acknowledgement details not present in the packet

## Claim Boundary Rules

Use these labels when reviewing claims:
- `SUPPORTED`: directly supported by the source packet
- `NEEDS_CITATION`: plausible but missing citation support
- `NEEDS_EVIDENCE`: requires experiment, proof, or repo-visible result
- `AUTHOR_DECISION`: requires human author confirmation
- `REMOVE_OR_WEAKEN`: too broad, unsupported, or misleading

Unsupported claims should appear in the review packet, not silently in the draft.

## Citation Rules

Allowed:
- cite works explicitly listed in the source packet
- preserve incomplete citation placeholders as gaps when clearly marked
- ask for missing bibliographic details

Not allowed:
- invent title, author, venue, DOI, arXiv id, or year
- claim a citation supports a statement without source-packet evidence
- run live web research unless the operator explicitly supplies a separate
  sourced research step

## Packet Guidance

A strong bounded packet usually includes:
- title options
- abstract draft
- section outline
- selected section drafts or full manuscript draft
- claim-boundary table
- citation-gap table
- reviewer notes
- explicit submission caveat

## Stop Rule

This skill stops at a reviewable manuscript packet.

It does not:
- submit to arXiv
- claim author approval
- claim peer review or acceptance
- fill citation gaps from memory
