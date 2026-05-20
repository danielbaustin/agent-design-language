# Publication Review Gates - v0.91.2

## Purpose

This packet records the minimum review and approval gates for any paper or
article work promoted from backlog into an active drafting lane.

It exists to prevent two common failures:

- treating formatting as evidence of readiness,
- treating local drafts as if they were already approved for publication.

## Shared Gates

Every paper or article should pass these gates before it is considered ready
for public release:

1. Source packet gate
   - source materials are named explicitly
   - canonical repo evidence is separated from speculative or local-only notes

2. Claim-boundary gate
   - definitions, design claims, empirical claims, conjectures, and future-work
     claims are separated
   - unsupported claims are marked or removed

3. Citation gate
   - citations are either verified or explicitly marked as missing / unresolved
   - no fabricated citations

4. Review gate
   - one structural review
   - one claim/citation review
   - one overclaim/publication-boundary review

5. Approval gate
   - explicit author/operator decision is recorded
   - no autonomous submission or publication

## arXiv-Specific Gates

The local skill schema and authoring notes imply these additional paper gates:

- manuscript packet exists and is bounded by a source packet
- printable output is not treated as submission readiness by itself
- author metadata, acknowledgements, and funding are completed separately
- submission remains outside the writing-skill boundary

## Medium-Specific Gates

For articles, the minimum gates are:

- article brief or demo-backed brief exists
- audience and house style are explicit
- forbidden claims are named
- stop-before-publish remains true

## Review Outcomes

Allowed statuses for this program surface:

- `backlog_only`
- `drafting_allowed`
- `review_required`
- `approved_for_publication_decision`

Disallowed interpretation:

- no surface should be read as `published` unless a later issue explicitly says
  so with real publication evidence

## Validation Questions

Reviewers should be able to answer:

- Is this item still backlog, or has it entered drafting?
- Are the claims bounded by sources?
- Are citations verified, missing, or still under review?
- Has a human actually approved any publication step?
