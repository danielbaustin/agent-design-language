# Structured Task Prompt

Template: 1.0.0

Issue: 5823

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver portable bounded runner with provenance and failover.

## Deliverables

- Typed portable request/result schema with exact revision, profile digest, platform, artifact, redaction, timeout, and cleanup fields
- Fail-closed local, Nessus, and AWS adapter selection
- No-network same-profile local fallback
- Repo-relative redacted logs, artifact digests, and machine-readable summaries
- Linux, macOS, Windows, timeout, cancellation, unreachable-provider, stale-revision, malformed-result, and cleanup proof
- Operator runbook and rollback boundary

## Acceptance

1. A typed portable request identifies exact revision, command profile digest, environment allowlist, resource and budget, artifact policy, timeout, and fallback
2. A typed result identifies adapter, platform, exact revision, profile digest, timing, exit, artifact digests, redaction, cleanup, and fallback outcome
3. Local, Nessus, and AWS adapters preserve one request/result contract and fail closed on unsupported or ambiguous selection
4. Network or provider failure runs or explicitly offers the same-profile local fallback without claiming remote proof
5. Stale revision, malformed result, timeout, cancellation, unreachable provider, path leakage, and incomplete cleanup are rejected or retained as blockers
6. Linux remote, local macOS, and Windows path/quoting or approved live-runner proof is retained with truthful fixture/live qualification
7. Machine output, adl_event diagnostics, secrets, paths, provenance, and artifacts obey the repository observability and privacy boundary
8. Focused adapter, negative, platform, no-network, diff, and exact-revision review proof passes

## Dependencies

- WP-02A issue #5801 complete with stable local command profiles and CI/PVF semantics
- Existing Nessus and AWS remote validation evidence and wrappers remain available as adapter inputs
- Agent Logic business AWS profile verified before any authorized AWS live lane

## Inputs

- .csdlc/prepared/issues/5823/design.md
- tools/aws_remote_validation/Cargo.toml
- tools/aws_remote_validation/src/aws_remote_validation.rs
- tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs
- adl/tools/run_aws_spot_remote_validation_lane.sh
- adl/tools/test_run_aws_spot_remote_validation_lane.sh
- adl/tools/run_nessus_remote_validation.sh
- adl/tools/test_run_nessus_remote_validation.sh
- docs/tooling/REMOTE_BUILD_HOW_TO.md
- docs/milestones/v0.91.7/review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md

## Non Goals

- Always-on runner fleet or broad CI migration
- Provider-specific behavior as the portable contract
- Remote speed, provisioning, or cache state as validation proof
- Credential export or arbitrary remote shell payloads
- Local validation dependency on network availability
- Unapproved AWS, personal-account, or paid execution
