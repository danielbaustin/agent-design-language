# Economics Inheritance And Authority Audit - v0.90.4

## Status

Landed in WP-02 / #2421.

## Purpose

v0.90.4 is the first contract-market milestone. It must consume the
citizen-state authority already proved in v0.90.3 instead of quietly redefining
who may act, inspect, freeze, review, or preserve evidence.

This audit maps the specific v0.90.3 authority surfaces that v0.90.4 inherits,
classifies whether each dependency is directly inherited, only fixture-backed,
still deferred, or blocked, and records the narrowing rules that keep the
economics layer bounded.

## Governing Rule

v0.90.4 inherits authority from v0.90.3. It does not mint new standing,
private-state access, continuity, quarantine release, or tool-execution
authority by placing those ideas inside contracts, bids, summaries, or market
fixtures.

The inherited baseline is:

- standing determines who may participate through which role boundary
- access control determines which sensitive paths require explicit authority
- projection remains redacted and non-authoritative
- challenge, freeze, appeal, and quarantine remain fail-closed safety surfaces
- continuity witnesses and receipts remain the basis for citizen-state
  legitimacy
- economics must not weaken continuity, privacy, evidence preservation, or due
  process

## Inherited v0.90.3 Surfaces

### Standing And Communication

Source proof:

- `docs/milestones/v0.90.3/STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md`
- `adl/src/runtime_v2/standing.rs`
- `adl/src/runtime_v2/tests/standing.rs`
- contract entrypoint: `runtime_v2_standing_contract`

What v0.90.4 inherits:

- citizens, guests, service actors, external actors, and naked actors remain
  distinct actor classes
- communication is not an inspection channel
- non-citizen participation must stay mediated and bounded

Contract-market implication:

- contracts and bids may name actor roles, but they do not grant citizen rights
- external counterparties in WP-08 must remain non-citizens by default
- summaries and review artifacts must not treat a message or submission as
  implied inspection authority

Classification: `inherited`

### Access-Control Semantics

Source proof:

- `docs/milestones/v0.90.3/ACCESS_CONTROL_SEMANTICS_v0.90.3.md`
- `adl/src/runtime_v2/access_control.rs`
- `adl/src/runtime_v2/tests/access_control.rs`
- contract entrypoint: `runtime_v2_access_control_contract`

What v0.90.4 inherits:

- sensitive paths require explicit, auditable authority events
- denied paths fail closed
- projection is visibility, not raw inspection
- challenge, appeal, quarantine, wake, and release are already modeled as
  authority-sensitive paths

Contract-market implication:

- contract award, acceptance, execution, cancellation, dispute, and completion
  must be modeled as explicit authority-checked transitions rather than as
  consequences of valid JSON
- contract or bid artifacts cannot grant inspection, decryption, migration,
  wake, release, or other citizen-state powers

Classification: `inherited`

### Redacted Observatory Projection Policy

Source proof:

- `docs/milestones/v0.90.3/REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md`
- `adl/src/runtime_v2/private_state_observatory.rs`
- `adl/src/runtime_v2/tests/private_state_observatory.rs`
- contract entrypoint: `runtime_v2_private_state_observatory_contract`

What v0.90.4 inherits:

- operator and reviewer visibility is redacted, read-only, and
  non-authoritative
- projections cannot wake, migrate, decrypt, release, or replace canonical
  citizen state
- public surfaces must remain narrower than operator/reviewer surfaces

Contract-market implication:

- market review summaries may cite evidence and status, but they must not become
  hidden authority channels
- v0.90.4 review output should remain reviewer-visible and redacted rather than
  exposing private citizen state, prompts, or tool arguments

Classification: `inherited`

### Sanctuary And Quarantine

Source proof:

- `docs/milestones/v0.90.3/SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md`
- `adl/src/runtime_v2/private_state_sanctuary.rs`
- `adl/src/runtime_v2/tests/private_state_sanctuary.rs`
- contract entrypoint: `runtime_v2_private_state_sanctuary_contract`

What v0.90.4 inherits:

- ambiguous or unsafe continuity transitions fail closed
- quarantine is not recovery success
- evidence preservation is mandatory
- release requires explicit review and continuity proof

Contract-market implication:

- contract-market transitions must preserve the same fail-closed posture
- disputed, revoked, or otherwise unsafe market actions must preserve reviewable
  evidence instead of silently advancing state
- contract-market lifecycle logic must not weaken citizen-state quarantine rules

Classification: `inherited`

### Continuity Witnesses And Receipts

Source proof:

- `docs/milestones/v0.90.3/CONTINUITY_WITNESSES_AND_RECEIPTS_v0.90.3.md`
- `adl/src/runtime_v2/private_state_witness.rs`
- `adl/src/runtime_v2/tests/private_state_witness.rs`
- contract entrypoint: `runtime_v2_private_state_witness_contract`

What v0.90.4 inherits:

- continuity-sensitive transitions require explicit witness evidence
- citizen-facing receipts explain continuity without leaking private material
- artifact references and evidence linkage are part of legitimacy, not just
  debugging output

Contract-market implication:

- market artifacts should preserve explicit trace and evidence references
- reviewable transition evidence matters for market state just as it does for
  citizen continuity
- v0.90.4 can borrow the evidence discipline, but it must not pretend a market
  trace bundle is the same thing as a citizen continuity witness

Classification: `fixture-backed`

Why only fixture-backed:

- v0.90.3 witnesses and receipts are citizen-state artifacts, not contract
  award or acceptance artifacts
- WP-06 through WP-14 must build market-native trace and review outputs on top
  of that discipline

### Challenge, Appeal, Threat Model, And Economics Placement

Source proof:

- `docs/milestones/v0.90.3/CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md`
- `adl/src/runtime_v2/challenge.rs`
- `adl/src/runtime_v2/tests/challenge.rs`
- contract entrypoint: `runtime_v2_continuity_challenge_contract`

What v0.90.4 inherits:

- challenge and freeze exist as bounded due-process shapes
- appeal does not bypass quarantine release requirements
- threat modeling is required for safety-sensitive flows
- economics placement already decided that markets belong in v0.90.4 and must
  not override continuity or privacy guarantees

Contract-market implication:

- v0.90.4 may define market disputes, review summaries, and residual-risk
  reporting, but it should not pretend to implement a full constitutional court
- cost, resource, and evaluation logic must remain subordinate to continuity,
  privacy, and evidence-preservation rules

Classification: `fixture-backed`

Why only fixture-backed:

- the v0.90.3 challenge flow is about continuity disputes on wake and
  projection, not market-native bid disputes or contract adjudication
- v0.90.4 should reuse the fail-closed posture and evidence expectations while
  keeping market dispute behavior bounded

## Dependency Classification Summary

| Dependency surface | v0.90.4 need | Classification | Reason |
| --- | --- | --- | --- |
| standing and communication | participant roles, external-actor boundary, no silent rights escalation | inherited | v0.90.3 already proves actor classes and mediated communication limits |
| access-control semantics | explicit authority checks for sensitive actions | inherited | v0.90.3 already proves auditable authority events and fail-closed denial |
| redacted projection policy | reviewer/operator visibility without raw private-state disclosure | inherited | v0.90.3 already proves redacted non-authoritative visibility |
| sanctuary/quarantine | fail-closed handling for unsafe state changes and evidence preservation | inherited | v0.90.3 already proves ambiguous transitions stay blocked pending review |
| continuity witnesses and receipts | traceable legitimacy and explanatory evidence | fixture-backed | evidence discipline is inherited, but market-native witness/receipt shapes do not yet exist |
| challenge, appeal, and economics placement | bounded dispute posture and economics safety boundary | fixture-backed | fail-closed review model exists, but market-native adjudication remains to be added |
| payment rails, settlement, tax, invoicing, banking | economic settlement semantics | deferred | explicitly out of scope for v0.90.4 |
| governed tool execution, UTS, ACC, executor authority | executable tool authority | deferred | explicitly handed off to v0.90.5 |
| citizen-state rebinding, birthday, cross-polis continuity | expanded identity semantics | deferred | explicitly outside v0.90.4 scope |

No required v0.90.4 dependency is currently classified as `blocked`.

## Gap List

The inherited substrate is strong enough to start v0.90.4, but the following
gaps still need explicit v0.90.4 work:

1. Market-native authority artifacts do not yet exist.
   v0.90.3 proves citizen-state authority, not contract award, acceptance,
   cancellation, delegation, or completion authority. WP-03 through WP-07 must
   create those artifacts explicitly.

2. Contract-market trace outputs are not the same as continuity witnesses.
   v0.90.4 needs reviewable contract-market trace and summary artifacts without
   claiming they replace citizen continuity proofs.

3. Market disputes are not yet a full inherited surface.
   v0.90.3 challenge and appeal prove the fail-closed pattern, but they do not
   implement contract disputes, revoked counterparties, bid denials, or
   delegated-deliverable disputes.

4. External counterparties need explicit bounded participation rules.
   v0.90.3 proves standing boundaries, but WP-08 still needs the counterparty
   record, assurance, sponsorship, gateway, and revocation semantics.

5. Tool requirements must remain descriptive only.
   v0.90.4 can record that a contract or bid depends on tool-mediated work, but
   it cannot execute tools or grant tool authority. That remains deferred to
   v0.90.5.

## Narrowing Recommendation

v0.90.4 should proceed with the following narrowed authority rule set:

- treat contracts, bids, evaluations, and summaries as bounded market artifacts,
  not identity or standing grants
- require every market transition to carry explicit actor, authority basis,
  trace references, and fail-closed denial behavior
- keep reviewer/operator visibility redacted and non-authoritative
- treat market disputes as bounded review artifacts rather than full judicial
  resolution
- record tool requirements only as constraints, evidence requirements, or
  resource claims
- refuse any feature shape that would imply payment settlement, unrestricted
  operator inspection, quarantine release, or direct model-to-tool execution

## Validation

Referenced docs and code surfaces were checked directly in the repository.

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sanctuary -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_continuity_challenge -- --nocapture
```

Supplemental existence checks:

```bash
rg -n "runtime_v2_(standing|access_control|private_state_observatory|private_state_sanctuary|private_state_witness|continuity_challenge)_contract" adl/src/runtime_v2/contracts.rs
test -f docs/milestones/v0.90.3/STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md
test -f docs/milestones/v0.90.3/ACCESS_CONTROL_SEMANTICS_v0.90.3.md
test -f docs/milestones/v0.90.3/REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md
test -f docs/milestones/v0.90.3/SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md
test -f docs/milestones/v0.90.3/CONTINUITY_WITNESSES_AND_RECEIPTS_v0.90.3.md
test -f docs/milestones/v0.90.3/CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md
```

## Result

v0.90.4 may proceed. The authority substrate it needs is present, but it must
stay bounded:

- inherit citizen-state authority rather than redefining it
- build market-native schemas and transitions explicitly
- preserve redaction, evidence, and fail-closed review behavior
- keep tools, payments, and expanded identity semantics deferred
