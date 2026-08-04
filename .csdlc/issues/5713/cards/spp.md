# Structured Planning Prompt

Template: 1.0.0

Issue: 5713

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Initialize and bind #5713, add explicit TLS mode configuration plus rcgen local self-signed bootstrap, prove the focused behavior, obtain exact GPT-5.5 review if available, publish a ready PR closing #5713, shepherd CI, merge, and verify issue closure.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #5713 in its dedicated issue worktree",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement explicit managed_external/local_self_signed bootstrap config and validation",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement rcgen local certificate bootstrap, reuse, locking, and atomic replacement",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Add focused Rust tests and documentation",
    "acceptance_ids": [
      "AC-8",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused validation and exact GPT-5.5 pre-PR review; fix actionable findings",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S6",
    "action": "Publish, shepherd green CI, merge, and verify GitHub issue closure",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "completed"
  }
]

## Invariants

- Externally managed certificate paths are read-only to Runtime v3 bootstrap
- Local self-signed certificates are state-root scoped and configuration-selected
- TLS verification is never disabled or weakened
- Private key material is never committed, logged, or copied to evidence
- The implementation is cross-platform Rust, with Unix-only permission strengthening behind cfg

## Risks

- Implicit local certificate generation could weaken production defaults if mode validation is loose
- Certificate replacement could leave mismatched key/cert material if writes are not atomic
- Concurrent bootstrap could race and regenerate identity
- SAN parsing could omit IP addresses or server-auth usage
- Tests could copy private key bytes into durable evidence if logs are careless

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5713/retained/design.md

Digest: d626e42ed3b513388440e66f1786d5249d021bea11a547050c3d152dd10f310d

## Diagram

.csdlc/issues/5713/retained/diagram.mmd

Digest: d7b04cc84f7bd1c53dbdbcc6e45b74368527a228dd07d227ffac9f8eff7fcc0a

## Stop Conditions

- Any claim collision on Runtime v3 TLS paths
- Any need to mutate primary main, #5733, WP-21, AWS CA, or production credentials
- Any failed focused proof or actionable exact-review finding
- Exact GPT-5.5 review remains unavailable before publication

## Handoff

Proceed only after doctor readiness.
