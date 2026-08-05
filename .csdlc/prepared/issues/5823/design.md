# WP-06 Portable Remote Validation Runner Design

## Outcome And Existing Substrate

Issue #5823 provides one provider-neutral validation request, provenance, and
result contract over the existing local, Nessus, and AWS remote-validation
surfaces. Current implementation evidence includes
`tools/aws_remote_validation/`, `adl/tools/run_nessus_remote_validation.sh`,
`adl/tools/run_aws_spot_remote_validation_lane.sh`, and retained v0.91.7 remote
builder proof. Those provider-specific paths remain adapters; none becomes the
portable contract by itself.

Local execution remains authoritative and usable when no network or remote
provider is available. AWS execution, when selected, uses the approved
`agent-logic-admin` business profile and requires explicit run authorization.

## Portable Contract

A typed request identifies repository-relative checkout, exact revision,
declared command profile, environment allowlist, resource class, budget,
artifact policy, timeout, and fallback disposition. A typed result records
adapter identity, host platform, exact revision, command profile digest,
timestamps, exit status, artifact digests, redaction status, cleanup status,
and whether local fallback ran.

No adapter accepts arbitrary secret values in the request. Machine-readable
results use stdout; human `adl_event` diagnostics use stderr. Durable evidence
contains repo-relative paths and redacts host/user paths and credentials.

## Execution Design

1. Extract the common request/result and adapter boundary from current local,
   Nessus, and AWS behavior without duplicating provider orchestration.
2. Add deterministic request validation, revision/provenance checks, artifact
   collection, timeout/cancellation, and cleanup semantics.
3. Implement fail-closed adapter selection and an explicit no-network local
   fallback that runs the same command profile.
4. Prove native Linux remote execution and native local macOS behavior. Both
   are mandatory live gates. Exercise Windows path/quoting and result handling
   with a native approved Windows runner when available, otherwise with
   deterministic fixtures explicitly qualified as non-native Windows proof.
5. Retain failure, interruption, unreachable-provider, stale-revision,
   malformed-result, path-leakage, and cleanup-negative evidence.

## Invariants And Non-Goals

- Remote speed or provisioning is not validation proof.
- A remote result cannot claim a different revision or command profile.
- Network failure never makes local validation unavailable.
- No always-on fleet, broad CI migration, credential export, or provider lock-in
  is authorized.

## Rollback

Disable the portable provider adapters and restore the unchanged local command
profile as the sole execution path. Cancel or clean up every remote job and
temporary resource, preserving request/result receipts and failure evidence;
incomplete cleanup remains a blocker rather than a successful rollback. Rerun
the local no-network fallback, provenance-mismatch, redaction, and platform
matrix contracts before remote execution can be re-enabled.

## Proof Design

Focused contract tests cover request/result round trips, adapter selection,
provenance mismatch, malformed output, redaction, timeout, cancellation, and
no-network fallback. Platform lanes cover local macOS, remote Linux, and
Windows path/quoting fixtures or a live approved Windows runner. A platform
matrix cannot pass with a blocked Linux or macOS row, and no row may use an
unqualified `blocked_with_evidence` success. Existing AWS
and Nessus wrapper tests prove adapter compatibility, not the entire portable
contract.
## Owned Paths

- `tools/remote_validation/Cargo.toml`
- `tools/remote_validation/Cargo.lock`
- `tools/remote_validation/src/lib.rs`
- `tools/remote_validation/src/bin/adl-remote-validation.rs`
- `tools/remote_validation/tests/contract.rs`
- `tools/aws_remote_validation/src/aws_remote_validation.rs`
- `tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs`
- `tools/aws_remote_validation/tests/portable_adapter.rs`
- `adl/tools/run_nessus_remote_validation.sh`
- `adl/tools/test_run_nessus_remote_validation.sh`
- `adl/tools/run_aws_spot_remote_validation_lane.sh`
- `adl/tools/test_run_aws_spot_remote_validation_lane.sh`
- `.csdlc/evidence/5823`
- `.csdlc/prepared/issues/5823/validate-platform-matrix.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.
