# Structured Output Record

Template: 1.0.0

Issue: 5342

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Implemented bounded ADL v2 stable record contracts, exact canonical bytes, real Ed25519 signed envelopes, external trust policy, atomic replay control, strict channel decoding, and fresh-process verification.

## Artifacts

- adl-v2/crates/adl-records
- .csdlc/prepared/issues/5342/validate-records.sh

## Execution

- Added deny-unknown-fields stable error, event, trace, execution-result, and artifact contracts with explicit limits
- Added exact tagged canonical grammar, SHA-256 payload identities, complete Ed25519 preimage binding, and strict duplicate-key channel decoding
- Added immutable external trust policy plus caller-owned replay guard with duplicate, rollback, and capacity rejection
- Added deterministic fresh-process, tamper, bounds, schema, trust, COTS, scope, LoC, formatting, and strict lint proof

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5342/validate-records.sh",
      "all"
    ],
    "purpose": "Prove records, canonical bytes, real signatures, external trust, replay, strict channels, tamper rejection, COTS, scope, LoC, formatting, strict Clippy, and fresh-process determinism.",
    "outcome": "passed",
    "evidence_ref": "Implementation commit 77c354522: 880 implementation LoC; 460 test/fixture LoC; 9 integration tests plus fresh-process harness; all-target tests, tamper target, fmt, strict Clippy, exact COTS, dependency receipts/ancestry, scope, and claim collision checks passed on /Volumes/FastWork."
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5342/validate-records.sh",
      "all",
      "081d52bdd0c073801503f2b6f0f4f9e0d4f2432e"
    ],
    "purpose": "Prove records, canonical bytes, real signatures, external trust, replay, strict channels, tamper rejection, COTS, scope, LoC, formatting, strict Clippy, and fresh-process determinism.",
    "outcome": "passed",
    "evidence_ref": "Exact head 081d52bdd0c073801503f2b6f0f4f9e0d4f2432e: 1039 implementation LoC; 878 test/fixture LoC; 10 records tests and 5 tamper/channel tests; schema diff, all-target tests, repeated tamper target, fmt, strict Clippy, exact COTS, dependency receipts/ancestry, scope, and claim collision checks passed on /Volumes/FastWork in 2 seconds."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-v2/crates/adl-records/Cargo.toml",
      "--all-targets",
      "--locked"
    ],
    "purpose": "Prove portable record contracts, deterministic canonical bytes, real Ed25519 signatures, external trust policy, replay protection, strict channel decoding, and tamper rejection at the exact merged head.",
    "outcome": "passed",
    "evidence_ref": "Exact merged PR #5628 head 5f48b0854cd36f64bef38ed0d09a09d84fa2e158: 10 records tests and 5 tamper/channel tests passed with zero failures on the existing FastWork target."
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
