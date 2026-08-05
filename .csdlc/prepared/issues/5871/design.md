# Issue 5871 Design: WP-04.09 Signed capability advertisements

## Outcome And Boundary

Implement bounded signed capability advertisements that are evidence inputs, never direct authority grants. This child is one exclusive implementation slice under
WP-04-IMP issue #5862; it does not absorb sibling work or
receive completion credit from the #5821 architecture gate.

## Source Baseline

- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md` defines the milestone feature and claim boundary.
- `.csdlc/prepared/issues/5821/design.md` freezes the Guardian-owned architecture, threat model, dependency graph, and sixteen-child denominator.
- `adl-runtime/src/guardian.rs`, `adl-runtime/src/networking.rs`, `adl-runtime/src/topology.rs`, and `adl-runtime/src/runtime_api.rs` are current Runtime v3 integration authorities.
- `adl-runtime/tests/guardian_cli.rs` and `adl-runtime/tests/runtime_api_wss.rs` are retained launch and authenticated carrier proof inputs, not substitutes for this child's named proof.

## Exclusive Owned Paths

- `adl-runtime/src/distributed/capability_advertisement.rs`
- `adl-runtime/tests/distributed_capability_advertisement.rs`

No other WP-04 child may edit these paths. This child may read sibling and
upstream contracts but may not widen its claim. WP-04.16 alone owns final
module registration and integrated proof paths.

## Design And Failure Semantics

Implement bounded signed capability advertisements that are evidence inputs, never direct authority grants. The implementation must preserve Guardian as process 0,
bounded queues and timeouts, authenticated transport, deterministic
projections, durable state authority, redaction, and fail-closed behavior.
Missing, stale, replayed, malformed, unauthorized, wrong-domain, or
resource-exhausted inputs remain explicit failures and never trigger an
insecure fallback.

## Dependencies

- WP-04.03 issue #5865
- WP-04-IMP issue #5862 coordinates ordering but owns no child product path.
- #5821 must be terminal before implementation binding.

## Proof Boundary

Exact nextest target distributed_capability_advertisement proves signatures, expiry, replay, wrong signer, oversize, redaction, and deterministic projection.

The execution receipt must bind the exact source revision, exact argv,
nonzero selected test count, output and artifact SHA-256 digests, runner
identity, negative cases, and native platform identity where claimed.
Hand-authored status booleans, retained fixtures, and prose do not prove
working behavior.

## Rollback Responsibility

Withdraw issue-created advertisements and make placement treat missing capability data as unavailable.

## Estimate

Budget this bounded child at 8 elapsed hours, 90,000 reasoning tokens, and
90 minutes of focused validation and review. Replan before widening paths,
dependencies, proof surface, or rollback authority.

## Non-Goals

- Sibling WP-04 paths, WP-14 protocol reconciliation, consumer UI work, or v0.93 governance.
- Runtime v2 fallback, custom cryptography, plaintext transport, or unbounded queues.
- Completion credit from issue creation, architecture approval, fixtures, or self-attested receipts.
