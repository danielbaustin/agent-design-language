# Deep Agents Comparative Proof

## Purpose

Define the bounded `v0.88` comparative proof surface derived from
`.adl/docs/TBD/DEEP_AGENTS_AND_ADL.md`.

This is not a generic competitor commentary document.

Its job is to make one specific reviewer-facing claim legible:

ADL can present a bounded deep-agent-style workflow packet while also preserving explicit
artifact provenance, review surfaces, and externalized cognitive state by reference.

## Bounded Claim

Filesystem-style deep-agent demos are often compelling because they show:
- visible role outputs
- intermediate files
- persistence outside the prompt

ADL should show the same practical strength while adding:
- durable artifact contracts
- explicit provenance for each stage
- state externalization by reference rather than by prompt re-embedding
- a reviewer-oriented surface explaining what can actually be audited

The `v0.88` comparative slice does not claim that ADL solves general scientific autonomy or
that every big-company demo is worthless.

It claims something smaller and stronger:

ADL turns a visible multi-agent workflow into a reviewable runtime proof surface.

## Why It Belongs In v0.88

This comparative slice belongs in `v0.88` because the milestone already centers on:
- temporal structure
- execution-policy truth
- artifact reviewability
- bounded multi-agent proof

`Paper Sonata` is the flagship expression of that story.
This comparative proof surface helps reviewers understand why `Paper Sonata` is not merely a
filesystem workflow with nicer rhetoric.

## Reviewer Question

The key reviewer question is:

What does ADL prove beyond “agents wrote some files in a directory”?

The bounded `v0.88` answer should be:

- the workflow emits inspectable intermediate outputs
- the artifact packet is accompanied by provenance and reference surfaces
- the review packet explains what can be audited directly
- large state is externalized and referenced rather than re-embedded opaquely

## Canonical Proof Row

Primary comparative proof row:

- command: `bash adl/tools/demo_v088_deep_agents_comparative_proof.sh`
- artifact root: `artifacts/v088/deep_agents_comparative_proof/`
- primary reviewer artifact: `comparative_proof.md`

This row is intentionally deterministic and fixture-backed.
It is a supporting comparative packet, not the flagship runtime workflow.

## Artifact Set

Minimum proof packet:
- `comparative_proof.md`
- `comparative_manifest.json`
- `filesystem_style_packet/`
- `adl_comparative_surface/provenance_manifest.json`
- `adl_comparative_surface/reference_map.json`
- `adl_comparative_surface/reviewer_checklist.md`

## Comparative Framing

The packet should make one clean contrast:

### Filesystem-style deep-agent surface
- visible files
- role decomposition
- persistence outside the prompt

### ADL comparative surface
- visible files
- role decomposition
- persistence outside the prompt
- plus explicit provenance
- plus explicit reference mapping
- plus an audit packet that says what the reviewer can verify
- plus explicit artifact references instead of hidden prompt stuffing

## Relationship To Paper Sonata

`Paper Sonata` remains the flagship `v0.88` demo.

This comparative proof surface is a support row for `WP-13`, not a second flagship.
Its role is to make the `Paper Sonata` story easier to explain and easier to review:

- `Paper Sonata` shows the bounded manuscript workflow
- this row shows the ADL comparison in a small deterministic packet without reusing the separate multi-agent demo

## Non-Goals

- general competitor benchmarking
- vendor scorecards
- broad AI strategy commentary
- replacing `Paper Sonata`
- claiming that the live discussion demo is itself the whole `v0.88` story

## Success Criteria

This bounded comparative slice succeeds if:
- a reviewer can run one command
- inspect one comparative packet
- understand the ADL difference in under a few minutes
- see that the comparison is grounded in artifacts and trace, not just prose
