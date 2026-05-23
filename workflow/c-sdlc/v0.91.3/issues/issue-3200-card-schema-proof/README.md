# v0.91.3 WP-02 Card Proof Bundle

## Purpose

This tracked bundle promotes the closed `WP-02` final card set into a durable
reviewer-facing C-SDLC proof namespace so the first transition manifest can cite
tracked card records instead of pointing directly at local-only `.adl/`
execution surfaces.

## Namespace

- `workflow/c-sdlc/v0.91.3/issues/issue-3200-card-schema-proof/`
- `workflow/c-sdlc/v0.91.3/issues/issue-3200-card-schema-proof/cards/`

## Source Promotion Boundary

The files in `cards/` are tracked snapshots of the closed `#3200` issue-local
card bundle. The local `.adl/` copies remain execution-history inputs, and some
historical local derivation references are intentionally preserved inside the
promoted snapshots. This bundle is therefore the durable reviewer-facing anchor
for `v0.91.3` transition-manifest references, not a claim that the broader
tracked-workflow migration is already complete.

## Proof Surfaces

- `cards/sip.md`
- `cards/stp.md`
- `cards/spp.md`
- `cards/srp.md`
- `cards/sor.md`
