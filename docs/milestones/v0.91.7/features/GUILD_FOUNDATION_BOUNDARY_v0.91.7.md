# Guild Foundation Boundary

## Metadata

- Feature Name: Guild Foundation Boundary
- Milestone Target: `v0.91.7`
- Status: `boundary_proven`
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: runtime boundary, governance handoff
- Proof Modes: tests, review

## Purpose

Define the smallest guild foundation that `v0.92` may consume without claiming
constitutional governance, polis authority, delegated authority, or public guild
product readiness.

## Scope

In scope:

- declarative guild identity record identifier;
- declarative member/role registry identifier;
- declarative governed membership event-log identifier;
- declarative moderation-escalation-hook identifier;
- declarative witness-evidence-reference identifier;
- declarative v0.93 governance-handoff-anchor identifier;
- executable non-claim validation.

Out of scope:

- constitutional citizenship;
- polis governance runtime;
- delegated governance authority;
- binding collective decision-making;
- public guild/community product readiness.

## Implemented Boundary

`#4755` adds `runtime_v2.guild_foundation_boundary.v1` in
`adl/src/runtime_v2/guild_foundation_boundary.rs`.

The boundary packet requires:

- `activation_posture = foundation_proof_for_v0_92_governance_handoff`;
- a fixed MVP foundation set for guild identity, member roles, membership
  events, moderation escalation, witness references, and v0.93 handoff;
- a fixed `v0.92` vocabulary allowlist naming birthday governance context,
  witness-evidence-routing context, community-memory boundary language, and
  future governance issue inputs without implementing those behaviors;
- deferred handoff rows for constitutional citizenship, polis governance,
  delegated authority, and public guild product readiness;
- promotion gates for any future stronger guild/governance claim: operator
  approval, tracked issue, bounded test plan, security/governance review,
  retained proof artifact, and public-claim review.

## v0.92 Consumption

`v0.92` may consume the guild foundation boundary only as declarative governance
handoff vocabulary and claim gates. It may not consume this proof as implemented
identity/witness routing, membership events, moderation hooks, completed
governance, citizenship, polis authority, delegated authority, binding
collective decision-making, or public product readiness.

## Validation And Review

Focused local proof:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_guild_foundation_boundary
```

The tests validate the handoff-only posture, deny allowlist expansion, preserve
required non-claims, fail closed on governance handoff drift, require promotion
gates, and assert stable path-safe JSON serialization.

## Non-Goals

- No constitutional governance implementation.
- No binding collective decision-making implementation.
- No public guild product readiness claim.
- No `v0.92` governance completion claim.
