# Issue Resource Telemetry V1 And S3 Archive Plan

## Metadata

- Milestone: `v0.91.6`
- Issue: `#4280`
- Status: `planned_contract`
- Scope: planning-only contract for per-issue machine/resource telemetry
- Proof mode: contract doc plus sanitized sample fixture

## Purpose

Define the first bounded contract for issue-level machine/resource telemetry so
ADL can collect CPU, memory, disk, GPU, process, and host context per issue
without committing raw local telemetry into Git and without leaking secrets or
machine-local details into reviewer-facing artifacts.

This document plans the contract and archive layout only. It does not claim
that live collection, private S3 upload, or multi-host rollout already exists.

## Inputs Used

- `#4276` Predictable Execution Fabric sprint packet and hybrid-lane policy
- `AGENTS.md` observability and stdout/stderr contract
- `docs/milestones/v0.91.5/review/logging_observability/CONTROL_PLANE_LOGGING_PROOF_3706.md`
- `docs/milestones/v0.91.6/review/ci_log_archive/CI_LOG_ARCHIVE_S3_CONTRACT_4225.md`
- `docs/milestones/v0.91.5/LOCAL_ADL_STATE_DISPOSITION_3473.md`
- `docs/milestones/v0.91.5/review/CHECKLIST_MINI_SPRINT_FUTURE_SESSION_RUNBOOK_3742.md`
- local runtime-root convention under `.adl/runs/`

## Contract Summary

`IssueResourceTelemetryV1` is an append-only local JSONL capture surface for
issue execution resource snapshots.

The contract is split intentionally:

- raw per-snapshot JSONL is local ignored evidence
- tracked docs may include only sanitized samples, summaries, and archive refs
- private S3 storage is the intended durable raw-evidence backend
- review-safe summaries are separate from raw telemetry capture

## Local Capture Path

The first local ignored capture path is:

```text
.adl/runs/issues/issue-<issue-number>/telemetry/issue_resource_telemetry.v1.jsonl
```

Optional local companion manifest path:

```text
.adl/runs/issues/issue-<issue-number>/telemetry/issue_resource_telemetry_manifest.v1.json
```

Why this path:

- it stays inside the existing local `.adl/runs/` execution root
- it keeps issue-scoped telemetry separate from sprint state and ordinary run
  artifacts
- it remains ignored by Git, matching the local-state policy for runtime
  evidence and temporary operational data

## Payload Shape

Each JSONL row records one bounded snapshot with:

- `schema_version`
  - exact value: `adl.issue_resource_telemetry.v1`
- `issue_number`
  - numeric issue id
- `issue_slug`
  - stable slug when available
- `captured_at`
  - RFC3339 UTC timestamp
- `capture_stage`
  - bounded lifecycle stage such as `issue_start`, `pre_validation`,
    `post_validation`, or `review_handoff`
- `host`
  - object with:
    - `label`
    - `classification`
    - `approval_state`
- `data_source`
  - object with:
    - `collector`
    - `sampling_scope`
    - `sampling_mode`
- `cpu`
  - structured object or exact string `not_available`
- `memory`
  - structured object or exact string `not_available`
- `disk`
  - array of bounded mount summaries or exact string `not_available`
- `gpu`
  - array of bounded device summaries or exact string `not_available`
- `process_summary`
  - bounded object or exact string `not_available`
- `archive`
  - object with:
    - `redaction_status`
    - `local_retention`
    - `private_archive_ref`

## Field Rules

### Host

`host.label` may contain an operator-approved stable host label such as `wuji`.
If that approval is missing, the label must be a redacted stable placeholder
such as `redacted_host`.

`host.classification` values:

- `operator_named_local_host`
- `operator_named_csm_host`
- `redacted_local_host`
- `redacted_remote_host`

`host.approval_state` values:

- `approved_label`
- `redacted_label`

### CPU

Structured CPU data should stay coarse and review-safe:

- `logical_cores`
- `load_avg_1m` when available
- `utilization_pct`

Do not record per-thread dumps, scheduler internals, or command-line flags.

### Memory

Structured memory data should use:

- `total_bytes`
- `available_bytes`
- `used_bytes`
- `pressure_state` when available

### Disk

Each disk entry should be a bounded mount summary only:

- `mount_label`
- `filesystem_class`
- `total_bytes`
- `available_bytes`
- `used_bytes`

Do not record raw device serials or machine-local mount paths outside
operator-approved stable labels.

### GPU

GPU data is optional. When unsupported, use exact string `not_available`.

When available, each entry should use bounded fields only:

- `device_class`
- `vendor`
- `memory_total_bytes`
- `memory_used_bytes`
- `utilization_pct`

Do not record raw PCI addresses, driver debug output, or unbounded inventory
blobs.

### Process Summary

The process summary exists to describe issue-related resource pressure without
recording raw process internals.

Allowed fields:

- `tracked_process_count`
- `heavy_processes`
  - bounded array of:
    - `role`
    - `executable_basename`
    - `pid`
    - `cpu_pct`
    - `rss_bytes`

Forbidden fields:

- full command lines
- environment variables
- working directories
- open-file lists
- token-bearing tool arguments

## Unsupported Metric Rule

If a metric family cannot be sampled safely or portably, the field must be the
exact string:

```text
not_available
```

Unsupported metrics must not silently disappear and must not be replaced with
invented zero values.

## Archive Contract

The intended private S3 object convention follows the same evidence-first shape
used by `#4225`:

```text
s3://<bucket>/<prefix>/<owner-repo>/issues/issue-<issue-number>/host-<host-ref>/capture-<captured-at>/issue_resource_telemetry.v1.jsonl
```

Example:

```text
s3://adl-issue-telemetry/v0.91.6/danielbaustin-agent-design-language/issues/issue-4280/host-wuji/capture-2026-06-20T09:30:00Z/issue_resource_telemetry.v1.jsonl
```

Optional companion manifest:

```text
s3://<bucket>/<prefix>/<owner-repo>/issues/issue-<issue-number>/host-<host-ref>/capture-<captured-at>/issue_resource_telemetry_manifest.v1.json
```

This layout keeps issue id, approved host ref, and capture time addressable
without encoding local usernames or absolute host paths.

## Redaction And Hygiene Rules

The contract is fail-closed on privacy:

- no absolute host paths
- no raw command lines
- no environment variables
- no secrets or tokens
- no private prompt or model output content
- no usernames or home-directory fragments unless intentionally replaced by
  approved stable labels

Archive `redaction_status` should reuse the narrow statuses already established
for private evidence storage:

- `not_redacted_private_archive_manifest_only`
- `redacted_private_archive`
- `redacted_review_safe_summary`

Unknown statuses should fail closed.

## Local Retention Policy

Local JSONL capture is temporary operational evidence.

Durable intended surfaces are:

- private S3 raw telemetry object
- optional private archive manifest
- tracked review-safe sample or summary only when redaction rules are satisfied

The local JSONL file should be removable after durable archive and any required
summary extraction are complete.

## Follow-On Implementation Routing

This planning issue exposes two concrete follow-ons:

1. wuji-first local collector (`#4298`)
   - implement bounded local issue-resource sampling on operator-approved host
     `wuji`
   - write local ignored JSONL rows at the path defined here
   - prove `not_available` handling where GPU or other metrics are absent

2. private archive and multi-host rollout (`#4299`)
   - add manifest production and private S3 archive upload for the JSONL file
   - expand the host-label/redaction model to additional approved CSM machines
   - keep review-safe summaries separate from raw private evidence

## Validation For This Planning Slice

- parse the sample JSONL fixture successfully
- confirm the fixture uses repo-relative or redacted refs only
- confirm unsupported fields are explicit `not_available`
- confirm the contract makes non-claims about live collection and live upload

## Non-Claims

This issue does not claim:

- live telemetry collection is implemented
- S3 upload is implemented
- multi-host rollout is implemented
- every machine has GPU visibility
- telemetry is ready for public reviewer ingestion
