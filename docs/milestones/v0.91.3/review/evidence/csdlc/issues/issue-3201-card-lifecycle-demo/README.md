# v0.91.3 Card Lifecycle Demo Bundle

## Purpose

This tracked bundle is the first public `WP-03` proof surface for the
canonical C-SDLC card lifecycle:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

It exists to prove that the lifecycle can be represented in a durable, tracked
namespace rather than only inside local ignored `.adl/` issue bundles.

## Namespace

The target public namespace for durable issue-local card records in the first
slice is:

- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/`

This demo bundle uses:

- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/`

## Proof Surfaces

- `cards/sip.md`
- `cards/stp.md`
- `cards/spp.md`
- `cards/srp.md`
- `cards/sor.md`

## Validation Hooks

- `adl tooling validate-structured-prompt --type sip --phase bootstrap --input ...`
- `adl tooling validate-structured-prompt --type stp --input ...`
- `adl tooling validate-structured-prompt --type spp --input ...`
- `adl tooling validate-structured-prompt --type srp --input ...`
- `adl tooling validate-structured-prompt --type sor --phase completed --input ...`
- focused Rust tests under:
  - `adl/src/cli/tooling_cmd/tests/structured_prompt.rs`
  - `adl/src/cli/pr_cmd/doctor.rs`
