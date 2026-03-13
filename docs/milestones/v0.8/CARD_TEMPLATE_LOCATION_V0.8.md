# Output Card Template Location and Usage (v0.8)

This note records the canonical location and usage of the ADL output card template for v0.8 contributor workflow stability.

## Canonical Location

- Canonical source: `swarm/templates/cards/output_card_template.md`

## Current Tooling Usage

- Primary consumer: `swarm/tools/pr.sh`
- Primary variable: `OUTPUT_TEMPLATE="swarm/templates/cards/output_card_template.md"`
- Legacy fallback supported: `.adl/templates/output_card_template.md`

## Reconciliation Decision

- Contributors and automation should treat `swarm/templates/cards/output_card_template.md` as canonical.
- Legacy fallback is compatibility behavior only; it is not a second source of truth.
- Template updates should land in the canonical path and remain deterministic/machine-readable.

## Bounded Scope

This reconciliation does not redesign output-card schema content. It only clarifies canonical location, usage, and fallback semantics.
