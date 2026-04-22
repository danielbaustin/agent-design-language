# Redacted Observatory Projections - v0.90.3

## Status

Landed in WP-10 / #2336.

## Purpose

WP-10 makes private citizen-state continuity visible to operators and reviewers
without turning the Observatory into a raw private-state browser.

The proof covers four bounded projection audiences:

- operator
- reviewer
- public
- debug

Each audience receives a read-only projection with continuity status, evidence
references, explicit caveats, and denied actions. No audience receives raw
private citizen state, canonical private-state bytes, private section payloads,
private keys, sealed payload material, or authority to wake, migrate, decrypt,
release, or replace canonical state from the projection.

## Runtime Evidence

The Runtime v2 implementation introduces private-state Observatory projection
evidence in:

- `adl/src/runtime_v2/private_state_observatory.rs`
- `adl/src/runtime_v2/tests/private_state_observatory.rs`

The focused contract entrypoint is:

- `runtime_v2_private_state_observatory_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/observatory/private_state_redaction_policy.json`
- `adl/tests/fixtures/runtime_v2/observatory/private_state_projection_packet.json`
- `adl/tests/fixtures/runtime_v2/observatory/private_state_projection_report.md`
- `adl/tests/fixtures/runtime_v2/observatory/private_state_projection_negative_cases.json`

## Projection Policy

The redaction policy is
`runtime_v2.private_state_observatory_redaction_policy.v1`.

It binds to the WP-03 private-state projection schema and declares the
Observatory authority rule:

- projections are read-only visibility artifacts
- projections cannot wake a citizen
- projections cannot migrate a citizen
- projections cannot decrypt private state
- projections cannot release quarantine
- projections cannot replace canonical private citizen state

Globally redacted fields include raw private state, canonical bytes, private
memory contents, private identity contents, private section payloads, private
section digests, sealed payload material, private keys, and signature material.

WP-10 deliberately leaves `explicit_raw_private_state_allowances` empty.

## Projection Packet

The projection packet is
`runtime_v2.private_state_observatory_packet.v1`.

It includes:

- packet identity and source projection reference
- policy reference
- source private-state projection schema
- non-authoritative projection status
- audience projections for operator, reviewer, public, and debug
- operator continuity summary
- reviewer evidence references
- prohibited uses
- deterministic packet hash
- claim boundary

The continuity status preserves citizen id, manifold id, lineage id, state
sequence, source projection reference, witness set reference, citizen receipt
set reference, sanctuary/quarantine reference, and evidence references.

It intentionally exposes only the fact that source hash evidence is available,
not raw state material.

## Audience Boundaries

The operator projection shows continuity status, lineage, sequence, witness,
receipt, and sanctuary/quarantine references for read-only review. It does not
authorize release, wake, migration, decryption, or raw inspection.

The reviewer projection shows evidence references and claim boundaries so a
reviewer can follow the proof without receiving private-state contents.

The public projection remains minimal. It does not expose lineage id or source
state hash fields.

The debug projection exposes schema, artifact references, validation status,
redaction status, and non-authority status only. Debug mode is not a raw-state
escape hatch.

## Leakage Tests

The policy carries leakage probe tokens derived from the canonical private-state
artifact. The packet and report validators reject those tokens if they appear in
projection content.

Focused negative cases cover:

- injecting a canonical private-state token into a projection
- marking a projection authoritative
- allowing raw private state in the debug audience policy
- adding lineage or source-state-hash fields to the public projection
- adding raw-inspection claims to the operator report

## Report

The operator report is a Markdown projection surface. It summarizes packet
identity, continuity status, audience projections, evidence references,
prohibited uses, and claim boundaries.

It is not canonical state. It must not be used as private-state authority.

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_observatory -- --nocapture
```

Release-tail validation should also keep `cargo clippy --manifest-path
adl/Cargo.toml --all-targets -- -D warnings` green.

## Non-Claims

- This does not implement live Runtime v2 execution.
- This does not implement access-control grants.
- This does not implement unrestricted operator inspection.
- This does not make JSON projection authoritative.
- This does not implement standing, communication, challenge, appeal, or
  migration continuity.
- This does not implement the WP-14 inhabited Observatory flagship demo.
- This does not claim first true Godel-agent birth.
- This does not implement v0.92 identity rebinding.
