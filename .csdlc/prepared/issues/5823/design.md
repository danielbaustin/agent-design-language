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
4. Prove Linux remote execution and local macOS behavior; exercise native
   Windows path/quoting and result fixtures, using live Windows execution only
   when an approved runner is available.
5. Retain failure, interruption, unreachable-provider, stale-revision,
   malformed-result, path-leakage, and cleanup-negative evidence.

## Invariants, Non-Goals, And Rollback

- Remote speed or provisioning is not validation proof.
- A remote result cannot claim a different revision or command profile.
- Network failure never makes local validation unavailable.
- No always-on fleet, broad CI migration, credential export, or provider lock-in
  is authorized.
- Rollback disables provider adapters and preserves the local command profile
  plus all retained results; remote cleanup must still complete or report a
  blocker.

## Proof Design

Focused contract tests cover request/result round trips, adapter selection,
provenance mismatch, malformed output, redaction, timeout, cancellation, and
no-network fallback. Platform lanes cover local macOS, remote Linux, and
Windows path/quoting fixtures or a live approved Windows runner. Existing AWS
and Nessus wrapper tests prove adapter compatibility, not the entire portable
contract.
