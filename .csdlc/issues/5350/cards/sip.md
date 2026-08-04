# Structured Intent Prompt

Template: 1.0.0

Issue: 5350

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Compare pinned ADL v1 and ADL v2 behavior across the complete reviewed #5337 corpus, classify every mismatch, and bind current Runtime v3 and WP-10A live evidence without conflating their owners.

## Required Outcome

One deterministic exact-revision parity packet verifies both subjects and all corpus evidence, reports every case and behavior, contains zero unclassified mismatches, binds all ten Runtime v3 proof groups plus terminal WP-10A live proof, and blocks acceptance, soak, cutover, and deletion on any unsupported, invalid, or regression result.

## Scope

- exact immutable ADL v1 and ADL v2 subject identity manifests
- complete #5337 25-case, 75-observation, 23-behavior corpus comparison
- narrow #5337-declared normalization and deterministic equivalence/difference checks
- complete mismatch register with reviewed dispositions and zero unclassified rows
- Runtime v3 ten-group evidence overlay from #5591/#5592/#5589/#5590 plus #5341/#5349
- WP-10A #5497 and live #5501 evidence binding
- issue-local lifecycle, design, validation, review, and evidence contract

## Authority

- #5337 owns the incumbent corpus and normalization contract; #5350 consumes but does not rewrite it
- #5345 owns ADL v2 CLI and selector behavior; #5350 invokes exact installed binaries but owns no product behavior
- #5497/#5501 own distributed-workcell live proof and #5591/#5592/#5589/#5590/#5341/#5349 own Runtime v3 parity evidence
- #5361 is downstream Runtime v3 acceptance and consumes WP-11; it is not an execution prerequisite for WP-11
- This preparation owns only issue-local C-SDLC paths and authorizes no parity run, product edit, publication, selector change, cutover, deletion, Runtime v2 change, AWS, or credentials

## Assumptions

- none

## Operator Constraints

- Use installed typed C-SDLC v2 binaries only; no v1 wrappers, raw gh, AWS, or tracked main edits
- Preserve the preparation-only claim until every direct dependency is merged, typed closed_out, retained by receipt, and ancestral to the comparison revision
- Use the exact #5337 corpus and normalization rules without weakening exits, diagnostics, arrays, identities, signature verdicts, or sequential order
- Use /Volumes/FastWork for build, temporary, and comparison output
- Use no network or credentials during comparison and reject unknown command shapes
- Run bounded preparation review now and two read-only exact-revision shadows before future publication
