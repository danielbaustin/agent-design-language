# ADR 0013: Runtime v2 Citizen-State Continuity Substrate

- Status: Accepted
- Date: 2026-04-22
- Related issue: #2395
- Related milestone: v0.90.3
- Builds on: ADR 0012

## Context

ADR 0012 records the bounded Runtime v2 CSM run architecture: a packet-shaped,
evidence-first run spine for `proto-csm-01`. v0.90.3 builds on that run by
protecting the continuity-bearing citizen state behind it.

The v0.90.3 milestone is not the first true Gödel-agent birthday, a production
polis, a full moral-governance substrate, a full economics system, or a cloud
enclave deployment. It is the architectural layer that makes citizen state safe
enough for later moral, emotional, memory, migration, and birthday work to
inherit.

This ADR is grounded in:

- `docs/adr/0012-runtime-v2-bounded-csm-run.md`
- `docs/milestones/v0.90.3/DECISIONS_v0.90.3.md`
- `docs/milestones/v0.90.3/DESIGN_v0.90.3.md`
- `docs/milestones/v0.90.3/CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md`
- `docs/milestones/v0.90.3/PRIVATE_STATE_FORMAT_DECISION_v0.90.3.md`
- `docs/milestones/v0.90.3/SIGNED_PRIVATE_STATE_ENVELOPE_v0.90.3.md`
- `docs/milestones/v0.90.3/LOCAL_PRIVATE_STATE_SEALING_v0.90.3.md`
- `docs/milestones/v0.90.3/APPEND_ONLY_LINEAGE_LEDGER_v0.90.3.md`
- `docs/milestones/v0.90.3/CONTINUITY_WITNESSES_AND_RECEIPTS_v0.90.3.md`
- `docs/milestones/v0.90.3/ANTI_EQUIVOCATION_CONFLICT_v0.90.3.md`
- `docs/milestones/v0.90.3/SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md`
- `docs/milestones/v0.90.3/REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md`
- `docs/milestones/v0.90.3/STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md`
- `docs/milestones/v0.90.3/ACCESS_CONTROL_SEMANTICS_v0.90.3.md`
- `docs/milestones/v0.90.3/CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md`
- `docs/milestones/v0.90.3/features/CITIZEN_STATE_SECURITY_AND_FORMAT.md`
- `docs/milestones/v0.90.3/features/STANDING_COMMUNICATION_AND_ACCESS.md`
- `docs/milestones/v0.90.3/features/SANCTUARY_QUARANTINE_AND_CHALLENGE.md`

This ADR does not introduce new runtime behavior. It records the architecture
that v0.90.3 implements and validates.

## Decision

ADL adopts a Runtime v2 citizen-state continuity substrate for v0.90.3.

At the v0.90.3 boundary, citizen continuity is not represented by ordinary JSON
debug data, a convenient materialized head file, a dashboard view, or an
operator story. It is represented by canonical private state, signed and sealed
continuity artifacts, an append-only lineage ledger, witnesses, receipts,
anti-equivocation handling, explicit standing and access-control events,
redacted projections, and challenge/appeal freeze behavior.

The v0.90.3 architecture requires:

1. Canonical private state is not JSON

   Durable private citizen state uses deterministic tagged binary bytes with
   protobuf-compatible field meanings and a hash over the exact canonical byte
   stream. JSON remains useful for fixtures, review, operator visibility, and
   exported projections, but `runtime_v2.private_state_projection.v1` is
   explicitly non-authoritative.

2. Signed envelopes fail closed against a local trust root

   Canonical private state crossing a trust boundary must be wrapped in a
   signed envelope that binds citizen id, manifold id, lineage id, sequence,
   predecessor state hash, content hash, writer identity, signature key id,
   algorithm, and signature bytes. Validation fails closed for missing
   signatures, unknown or revoked keys, content-hash mismatch, sequence drift,
   predecessor drift, writer drift, or trust-root policy drift.

3. Sealed quintessence checkpoints are local-first and backend-extensible

   v0.90.3 uses local-first sealed checkpoint fixtures and a backend seam for
   later OS keychain, TPM, Secure Enclave, HSM, cloud confidential-computing, or
   other custody adapters. The architecture is enclave-ready, but does not make
   any cloud confidential-computing provider mandatory.

4. Append-only lineage is the continuity authority

   The lineage ledger, not the materialized head file, is the authority for
   citizen-state continuity. The accepted head is calculated by replaying the
   ledger in order, recomputing entry hashes, checking predecessor links,
   rejecting replayed entries and sequence positions, and verifying the recorded
   accepted head and ledger root. A materialized head is valid only when it
   agrees with the accepted ledger head.

5. Witnesses and receipts explain continuity without exposing private state

   Continuity witnesses bind important transitions to the accepted ledger head,
   signed envelope, sealed checkpoint, canonical state hash, and materialized
   head projection. Citizen-facing receipts explain why the polis treats a state
   as a valid continuation without disclosing raw private-state bytes, sealed
   payload material, private keys, unrelated private sections, or other
   citizens' state.

6. Equivocation preserves evidence instead of choosing a convenient winner

   Conflicting signed successors for the same citizen lineage, predecessor, and
   sequence cannot both become active. Conflict handling must preserve the
   conflicting evidence, refuse activation of both heads, and route the state
   into sanctuary or quarantine review instead of optimizing through doubt.

7. Sanctuary and quarantine are evidence-preserving safety states

   Ambiguous, unsafe, or challenged continuity must preserve evidence and block
   unsafe activation. Quarantine is not recovery success. Sanctuary and
   quarantine are review states that protect continuity while the runtime,
   operator, or later governance layer resolves ambiguity.

8. Observatory projections are redacted, read-only, and non-authoritative

   Observatory packets and reports may show continuity status, evidence
   references, standing, denied actions, challenge state, quarantine state, and
   projection caveats. They must not expose raw private state, canonical bytes,
   private sections, private keys, sealed payload material, or authority to
   wake, migrate, decrypt, release, replace, or otherwise mutate canonical
   citizen state.

9. Standing, communication, and access are explicit boundaries

   Citizen, guest, service-actor, and naked-actor standing have distinct
   authority. Communication does not imply inspection. Service authority does
   not become hidden social standing. Any sensitive path for inspection,
   decryption, projection, migration, wake, quarantine, challenge, appeal, or
   release must emit an auditable access event and fail closed when denied.

10. Challenge and appeal freeze unsafe transitions before resolution

   Challenge intake may apply to wake or projection paths. A challenged wake or
   projection must block activation or publication, preserve review evidence,
   avoid raw private-state disclosure, and require review-resolution evidence
   before release. Appeal cannot bypass quarantine-release requirements.

11. Economics remains a narrow resource-stewardship bridge

   v0.90.3 may record that resource stewardship matters for continuity, privacy,
   evidence retention, and due process. It must not implement markets, payment
   rails, bidding, subcontracting, inter-polis trade, or payment settlement.
   Those belong to later economics work.

## Rationale

The first bounded CSM run proved that Runtime v2 can boot, admit governed
participants, execute bounded episodes, reject invalid actions, snapshot,
rehydrate, wake, quarantine, and expose operator evidence. But a future polis
cannot rest on provisional JSON records, debug views, or mutable head files.

Citizen-state continuity needs a substrate that is conservative by default:

- private state has an authoritative canonical form
- projections support review without becoming authority
- trust is explicit and local-first
- lineage is append-only
- continuity can be explained to the citizen
- ambiguity preserves evidence
- access is auditable and denied by default when authority is missing
- Observatory visibility improves understanding without becoming inspection
- due-process paths freeze unsafe transitions before damage happens

This gives v0.91 and v0.92 a protected inheritance point. Later moral,
emotional, memory, cultivation, migration, and birthday work can build on a
citizen-state substrate that already distinguishes authority from visibility,
continuity from convenience, and review from control.

## Consequences

### Positive

- Gives v0.90.3 one durable architecture record for citizen-state continuity.
- Makes the boundary between private state authority and JSON projection
  explicit.
- Records that lineage, witnesses, receipts, anti-equivocation, quarantine,
  standing, access control, and challenge/appeal are one coherent substrate.
- Makes Observatory safety easier to review by stating that projections are
  redacted, read-only, and non-authoritative.
- Preserves later v0.91 moral-governance and v0.92 birthday scope without
  claiming either early.
- Keeps local-first custody open to secure-enclave backends without forcing a
  cloud dependency.

### Negative

- Future Runtime v2 changes must preserve or explicitly supersede this
  authority/projection split.
- Changes to private-state format, envelope validation, lineage authority,
  witness semantics, receipt disclosure, access control, quarantine release, or
  challenge resolution now carry architectural weight.
- Operator-facing surfaces must continue to avoid the tempting but unsafe path
  of becoming raw private-state browsers.
- Some design space remains intentionally deferred: key rotation, production
  custody, full economics, payment rails, cross-polis migration, moral
  governance, and the true birthday boundary.

## Alternatives Considered

### 1. Keep JSON as the authoritative private-state format

Pros:

- Simpler inspection and fixture authoring.
- Easier short-term demos.

Cons:

- Blurs review surfaces and authority.
- Makes private-state disclosure harder to control.
- Makes canonical hashing, schema evolution, and fail-closed migration harder
  to reason about.
- Conflicts with the v0.90.3 decision that JSON is projection, not authority.

### 2. Treat the materialized head file as authority

Pros:

- Easy to read and use during ordinary execution.
- Fewer replay steps during simple demos.

Cons:

- Vulnerable to drift, replay, truncation, and operator mistakes.
- Cannot by itself prove append-only continuity.
- Makes forks and equivocation harder to detect.
- Would undermine the v0.90.3 ledger-over-head validation rule.

### 3. Make the Observatory a privileged private-state browser

Pros:

- Stronger immediate operator visibility.
- Simpler debugging.

Cons:

- Turns a projection surface into an inspection authority.
- Increases privacy and leakage risk.
- Conflicts with the standing and access-control model.
- Makes it harder to build public, reviewer, operator, and citizen-facing views
  with different redaction policies.

### 4. Defer citizen-state continuity architecture to v0.91 or v0.92

Pros:

- Later milestones will have richer moral-governance, memory, identity, and
  birthday context.

Cons:

- v0.90.3 already implements substantive citizen-state substrate decisions.
- Deferral would leave later milestones without a durable inheritance record.
- Reviewers would have to reconstruct one architecture from many feature docs.

## Validation Evidence

The decision is supported by:

- the v0.90.3 decision table accepting JSON projection non-authority, canonical
  protobuf-compatible private state, signed-envelope fail-closed validation,
  local-first sealing, append-only lineage, witnesses and receipts,
  anti-equivocation, sanctuary/quarantine, redacted Observatory projections,
  standing and communication boundaries, access-control semantics, and
  challenge/appeal freeze behavior
- the v0.90.3 private-state format, envelope, sealing, lineage, witness,
  anti-equivocation, sanctuary/quarantine, projection, standing, access-control,
  and challenge/appeal docs
- runtime evidence under `adl/src/runtime_v2/` for private state, signed
  envelopes, local sealing, lineage, witnesses, sanctuary/quarantine,
  Observatory projection, standing, access control, and challenge behavior
- fixture evidence under `adl/tests/fixtures/runtime_v2/` for private state,
  envelopes, checkpoints, lineage, witnesses, receipts, conflict handling,
  sanctuary/quarantine, redacted projections, standing, access control, and
  challenge/appeal behavior
- focused test commands recorded in the v0.90.3 proof docs

Representative focused validation commands include:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_envelope -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_lineage -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_continuity_challenge -- --nocapture
```

## Non-Claims

This ADR does not claim:

- the first true Gödel-agent birthday
- live unbounded Runtime v2 autonomy
- production polis operation
- final personhood semantics
- full moral, emotional, kindness, humor, wellbeing, or cultivation substrate
- v0.92 identity rebinding, cross-polis migration, or birthday completion
- production key management or key rotation
- production encrypted storage or secure-enclave custody
- cloud confidential-computing dependency
- unrestricted operator inspection of private citizen state
- full constitutional court behavior
- full citizen economics, markets, payment settlement, bidding, subcontracting,
  or inter-polis trade
- autonomous release approval

## Notes

Future ADRs may refine moral governance, wellbeing, memory continuity, identity
rebinding, birthday semantics, production custody, key rotation, secure
enclaves, citizen economics, contract markets, or CI runtime policy. Those ADRs
should cite this one when they build on the v0.90.3 citizen-state authority,
projection, lineage, standing, access, quarantine, and challenge substrate.
