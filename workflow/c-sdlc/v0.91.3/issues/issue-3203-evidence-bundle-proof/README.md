# v0.91.3 WP-05 Evidence Bundle Card Proof Bundle

## Purpose

This tracked bundle promotes the closed `WP-05` final card set into a durable
reviewer-facing C-SDLC proof namespace so ObsMem handoff records can cite exact
final `SRP` and `SOR` source records instead of only supporting packet
summaries.

## Namespace

- `workflow/c-sdlc/v0.91.3/issues/issue-3203-evidence-bundle-proof/`
- `workflow/c-sdlc/v0.91.3/issues/issue-3203-evidence-bundle-proof/cards/`

## Source Promotion Boundary

The files in `cards/` are tracked snapshots of the closed `#3203` issue-local
card bundle. Local `.adl/` copies remain derivation and execution surfaces, and
some historical local derivation references remain embedded inside the promoted
snapshots. This tracked bundle is the durable reviewer-facing provenance anchor
for `v0.91.3` ObsMem handoff citations, not a claim that the full tracked
workflow-state migration is already self-contained.

## Primary Provenance Surfaces

- `cards/srp.md`
- `cards/sor.md`

## Additional Context Surfaces

- `cards/sip.md`
- `cards/stp.md`
- `cards/spp.md`
