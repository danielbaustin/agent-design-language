# Continuity Challenge, Appeal, Threat Model, And Economics Placement - v0.90.3

## Status

Landed in WP-13 / D11.

This proof adds bounded Runtime v2 artifacts for continuity challenge, freeze,
appeal/review, citizen-state threat coverage, and economics placement. It is a
citizen-state substrate proof, not a full constitutional court, economics
system, payment rail, or v0.92 identity/birthday implementation.

## Runtime Artifacts

WP-13 adds the following deterministic artifact schemas:

- `runtime_v2.continuity_challenge_artifact.v1`
- `runtime_v2.continuity_challenge_freeze_artifact.v1`
- `runtime_v2.continuity_appeal_review_artifact.v1`
- `runtime_v2.citizen_state_threat_model.v1`
- `runtime_v2.economics_placement_record.v1`

The fixture-backed artifact paths are:

- `adl/tests/fixtures/runtime_v2/challenge/challenge_artifact.json`
- `adl/tests/fixtures/runtime_v2/challenge/freeze_artifact.json`
- `adl/tests/fixtures/runtime_v2/challenge/appeal_review_artifact.json`
- `adl/tests/fixtures/runtime_v2/challenge/citizen_state_threat_model.json`
- `adl/tests/fixtures/runtime_v2/challenge/economics_placement_record.json`

The public contract entrypoint is:

```rust
runtime_v2_continuity_challenge_contract()
```

## Challenge And Freeze Semantics

The D11 challenge artifact binds the challenge to the WP-12 access event packet,
the WP-09 sanctuary/quarantine artifact, and the WP-10 redacted Observatory
projection. Challenge intake covers the two WP-13 proof paths:

- challenged wake
- challenged projection

The freeze artifact proves that a challenged wake or projection:

- blocks wake activation
- blocks projection publication
- does not advance the continuity sequence
- does not change the active citizen head
- does not disclose raw private state
- preserves review evidence
- requires review resolution before release

## Appeal / Review Boundary

The appeal/review artifact is intentionally bounded. It records due-process
shape and required resolution evidence, but it does not grant automatic release.

The prototype outcome remains:

```text
uphold_freeze_pending_valid_proof
```

Release requires new continuity proof or an explicit review-resolution artifact.
Appeal cannot bypass quarantine release requirements and cannot disclose raw
private state.

## Threat Model Coverage

The citizen-state threat model covers the required WP-13 abuse paths:

- insider/operator abuse
- compromised key
- malicious guest
- equivocation
- replay
- projection leakage
- unsafe release from quarantine

Each threat records actor class, attack path, required controls, detection
artifacts, and fail-closed behavior.

## Economics Placement

The economics placement record decides that v0.90.3 only needs a narrow
resource-stewardship bridge before v0.90.4.

Allowed v0.90.3 scope:

- record resource stewardship obligations for evidence retention
- refuse cost optimizations that weaken continuity guarantees
- surface operator review costs as governance facts
- defer market allocation to the economics milestone

Explicitly deferred to v0.90.4:

- markets
- payment rails
- bidding
- subcontracting
- inter-polis trade

The governing rule is that cost optimization must never override citizen
continuity, privacy, evidence preservation, or due process.

## Validation

Focused validation command:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_continuity_challenge -- --nocapture
```

The focused tests verify:

- all five D11 artifacts are generated from the public contract
- golden fixtures match byte-for-byte
- challenged destructive transition freezes safely
- challenged projection publication freezes safely
- active head cannot change while frozen
- appeal cannot release without resolution proof
- the required threat-model abuse paths are present
- economics placement does not implement markets or payment rails

## Claim Boundary

This proof lands the D11 continuity challenge, appeal/review shape, threat
model, and economics placement record for v0.90.3. It does not implement full
economics, payment settlement, a constitutional court, cloud-enclave dependence,
v0.92 migration/birthday semantics, or first true Godel-agent birth.
