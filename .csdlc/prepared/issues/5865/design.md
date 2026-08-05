# Issue 5865 Design: WP-04.03 Maintained QUIC/TLS transport adapter

## Outcome And Boundary

Integrate a maintained QUIC/TLS stack with bounded authenticated channels and no custom cryptography or framing. This child is one exclusive implementation slice under
WP-04-IMP issue #5862; it does not absorb sibling work or
receive completion credit from the #5821 architecture gate.

## Source Baseline

- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md` defines the milestone feature and claim boundary.
- `.csdlc/prepared/issues/5821/design.md` freezes the Guardian-owned architecture, threat model, dependency graph, and sixteen-child denominator.
- `adl-runtime/src/guardian.rs`, `adl-runtime/src/networking.rs`, `adl-runtime/src/topology.rs`, and `adl-runtime/src/runtime_api.rs` are current Runtime v3 integration authorities.
- `adl-runtime/tests/guardian_cli.rs` and `adl-runtime/tests/runtime_api_wss.rs` are retained launch and authenticated carrier proof inputs, not substitutes for this child's named proof.

## Owned Paths

- `adl-runtime/src/distributed/transport.rs`
- `adl-runtime/tests/distributed_transport.rs`
- `adl-runtime/Cargo.toml`
- `adl-runtime/Cargo.lock`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-runtime-manifest-sequence-v1",
    "paths": [
      "adl-runtime/Cargo.toml",
      "adl-runtime/Cargo.lock"
    ],
    "issues": [
      5818,
      5865
    ],
    "order": [
      5818,
      5865
    ]
  }
]
```

## Design And Failure Semantics

Integrate a maintained QUIC/TLS stack with bounded authenticated channels and no custom cryptography or framing. The implementation must preserve Guardian as process 0,
bounded queues and timeouts, authenticated transport, deterministic
projections, durable state authority, redaction, and fail-closed behavior.
Missing, stale, replayed, malformed, unauthorized, wrong-domain, or
resource-exhausted inputs remain explicit failures and never trigger an
insecure fallback.

## Dependencies

- WP-04.02 issue #5864
- WP-04-IMP issue #5862 coordinates ordering but owns no child product path.
- #5821 must be terminal before implementation binding.

## Proof Boundary

Exact nextest target distributed_transport proves mutual authentication, channel bounds, cancellation, malformed-frame denial, peer mismatch, and dependency-lock parity.

The execution receipt must bind the exact source revision, exact argv,
nonzero selected test count, output and artifact SHA-256 digests, runner
identity, negative cases, and native platform identity where claimed.
Hand-authored status booleans, retained fixtures, and prose do not prove
working behavior.

## Rollback

Remove the distributed transport feature and restore the prior manifest and lockfile while retaining the single-node Runtime API.

## Estimate

Budget this bounded QUIC/TLS transport-adapter child under the typed medium
profile: 6 elapsed hours, 80,000 reasoning tokens, and 60 minutes of focused
validation and review. Handshake, framing, authentication, and transport-failure
proof stay within one adapter boundary; replan before widening paths,
dependencies, proof surface, or rollback authority.

## Non-Goals

- Sibling WP-04 paths, WP-14 protocol reconciliation, consumer UI work, or v0.93 governance.
- Runtime v2 fallback, custom cryptography, plaintext transport, or unbounded queues.
- Completion credit from issue creation, architecture approval, fixtures, or self-attested receipts.
