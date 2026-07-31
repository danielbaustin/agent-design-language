# Issue 5666 Design

Status: ready for bounded implementation.

## Intent

Issue #5666 adds a small, publication-safe developer throughput fast-lane policy
for low-risk ADL fixes. The work is documentation and focused contract proof
only; it does not change runtime/product behavior, CI workflows, provider
execution, GitHub lifecycle authority, or release gates.

## Scope

The protected implementation surface is:

- `docs/tooling/DEVELOPER_THROUGHPUT_FAST_LANE.md`
- `docs/tooling/VALIDATION_PLATFORM_ROUTING.md`
- `adl/tools/test_developer_throughput_fast_lane.sh`
- issue-local `.csdlc` records and evidence for #5666

`docs/architecture/VALIDATION_LANE_SELECTOR.md` remains the selector authority
and is referenced, not edited, because another lifecycle claim previously
covered architecture paths.

## Policy Requirements

The fast-lane policy must:

- define proportional issue classes and eligibility
- preserve typed C-SDLC v2 as the lifecycle authority
- require FastWork or another declared external build root when the operator
  requires it
- forbid silent local-disk fallback
- require changed-state-only PR watching and no GitHub waiting when no action is
  possible
- define escalation and stop conditions for runtime, provider, security,
  publication, closeout, release-gate, or ambiguous changes
- state non-claims so focused proof cannot be used for broad runtime/product
  completion

## Validation

Local proof is intentionally narrow:

- `bash adl/tools/test_developer_throughput_fast_lane.sh`
- `git diff --check`

The shell contract checks for required policy strings, selector reference, and
routing-doc linkage. It is not a runtime or CI proof.

## Publication Boundary

Publication requires exact-head review after implementation. Any review finding
that changes the policy, design packet, validation evidence, or lifecycle truth
requires revalidation and a fresh typed review before publication.
