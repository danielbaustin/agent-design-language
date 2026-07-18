# WP-13 Guild Foundation Boundary (#4755)

## Scope

`#4755` proves a declarative MVP guild foundation boundary that `v0.92` may
consume for birthday governance context without absorbing v0.93 constitutional
governance. Its claim status is `boundary_proven`, not `integrated_proven`.

## Implemented Surface

- Runtime module:
  `adl/src/runtime_v2/guild_foundation_boundary.rs`
- Runtime tests:
  `adl/src/runtime_v2/tests/guild_foundation_boundary.rs`
- Feature doc:
  `docs/milestones/v0.91.7/features/GUILD_FOUNDATION_BOUNDARY_v0.91.7.md`

## Decision

Guild foundation evidence is available to `v0.92` as handoff context only.
The module validates identifiers, allowlists, deferrals, and promotion gates; it
does not produce or consume guild identity records, membership events,
moderation hooks, or witness-routing records.

`v0.92` may consume:

- birthday governance context;
- declarative identity/witness-routing vocabulary only;
- community-memory boundary language;
- future governance issue inputs.

`v0.92` may not consume:

- constitutional citizenship;
- polis governance runtime;
- delegated governance authority;
- binding collective decision-making;
- public guild product readiness;
- completed governance claims.

## Validation Plan

Focused local proof:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_guild_foundation_boundary
```

The tests validate the exact MVP foundation surface set, the fixed
handoff-context allowlist, governance-handoff deferrals, promotion gates,
non-claims, and stable path-safe JSON serialization.

## Residual Risk

This packet does not implement v0.93 constitutional governance, polis decision
authority, delegated authority, voting, social contract, or public product
readiness. That is intentional scope control for `v0.92`; any promotion needs a
new tracked issue, retained proof, security/governance review, and public-claim
review.
