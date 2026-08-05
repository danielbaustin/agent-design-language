# Issue 5878 Design: WP-04.16 Distributed integration, adversarial, and native-platform proof

## Outcome And Boundary

Register and integrate the distributed module, then prove real multi-node Guardian behavior, API/WSS continuity, adversarial failures, and native macOS/Linux/Windows receipts. This child is one exclusive implementation slice under
WP-04-IMP issue #5862; it does not absorb sibling work or
receive completion credit from the #5821 architecture gate.

## Source Baseline

- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md` defines the milestone feature and claim boundary.
- `.csdlc/prepared/issues/5821/design.md` freezes the Guardian-owned architecture, threat model, dependency graph, and sixteen-child denominator.
- `adl-runtime/src/guardian.rs`, `adl-runtime/src/networking.rs`, `adl-runtime/src/topology.rs`, and `adl-runtime/src/runtime_api.rs` are current Runtime v3 integration authorities.
- `adl-runtime/tests/guardian_cli.rs` and `adl-runtime/tests/runtime_api_wss.rs` are retained launch and authenticated carrier proof inputs, not substitutes for this child's named proof.

## Owned Paths

- `adl-runtime/src/distributed/mod.rs`
- `adl-runtime/src/lib.rs`
- `adl-runtime/tests/distributed_guardian.rs`
- `adl/tools/validate_v092_distributed_guardian.sh`
- `adl/tools/validate_v092_distributed_native_receipts.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Design And Failure Semantics

Register and integrate the distributed module, then prove real multi-node Guardian behavior, API/WSS continuity, adversarial failures, and native macOS/Linux/Windows receipts. The implementation must preserve Guardian as process 0,
bounded queues and timeouts, authenticated transport, deterministic
projections, durable state authority, redaction, and fail-closed behavior.
Missing, stale, replayed, malformed, unauthorized, wrong-domain, or
resource-exhausted inputs remain explicit failures and never trigger an
insecure fallback.

## Dependencies

- WP-04.01 issue #5863
- WP-04.02 issue #5864
- WP-04.03 issue #5865
- WP-04.04 issue #5866
- WP-04.05 issue #5867
- WP-04.06 issue #5868
- WP-04.07 issue #5869
- WP-04.08 issue #5870
- WP-04.09 issue #5871
- WP-04.10 issue #5872
- WP-04.11 issue #5873
- WP-04.12 issue #5874
- WP-04.13 issue #5875
- WP-04.14 issue #5876
- WP-04.15 issue #5877
- WP-04-IMP issue #5862 coordinates ordering but owns no child product path.
- #5821 must be terminal before implementation binding.

## Proof Boundary

Exact distributed_guardian test and live validator launch production Guardians and kernels, exercise authenticated API/WSS, partition, fencing, migration, recovery, shutdown, and digest-bound native receipts.

The execution receipt must bind the exact source revision, exact argv,
nonzero selected test count, output and artifact SHA-256 digests, runner
identity, negative cases, and native platform identity where claimed.
Hand-authored status booleans, retained fixtures, and prose do not prove
working behavior.

## Rollback Responsibility

Remove module registration and distributed launch configuration, fence remote ownership, and prove the WP-03 single-node Guardian remains healthy from unchanged durable state.

## Estimate

Budget this bounded child at 8 elapsed hours, 90,000 reasoning tokens, and
90 minutes of focused validation and review. Replan before widening paths,
dependencies, proof surface, or rollback authority.

## Non-Goals

- Sibling WP-04 paths, WP-14 protocol reconciliation, consumer UI work, or v0.93 governance.
- Runtime v2 fallback, custom cryptography, plaintext transport, or unbounded queues.
- Completion credit from issue creation, architecture approval, fixtures, or self-attested receipts.
