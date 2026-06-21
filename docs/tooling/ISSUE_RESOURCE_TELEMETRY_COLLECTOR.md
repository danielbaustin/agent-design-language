# Issue Resource Telemetry Collector

`#4298` adds the first bounded local collector for issue resource telemetry and
`#4299` extends it with private archive and multi-host rollout support.

## Command

```text
adl tooling issue-resource-telemetry collect \
  --issue <number> \
  --issue-slug <slug> \
  --capture-stage <issue_start|pre_validation|post_validation|review_handoff|custom_stage> \
  [--host-label wuji] \
  [--process <role:pid>] \
  [--pid-file-process <role:path>] \
  [--captured-at <rfc3339>] \
  [--repo-root <path>] \
  [--out <path>] \
  [--json]
```

```text
adl tooling issue-resource-telemetry archive \
  --issue <number> \
  --issue-slug <slug> \
  --input <telemetry.jsonl> \
  --manifest-out <manifest.json> \
  --s3-prefix s3://<bucket>/<prefix> \
  [--repo owner/repo] \
  [--host-label <label>] \
  --captured-at <rfc3339> \
  [--redaction-status <status>] \
  [--upload] \
  [--upload-manifest] \
  [--repo-root <path>] \
  [--json]
```

## Local Output Path

Default local ignored path:

```text
.adl/runs/issues/issue-<issue-number>/telemetry/issue_resource_telemetry.v1.jsonl
```

The collector appends one JSONL row per invocation.

## Capture Stages

Recommended bounded stages:

- `issue_start`
- `pre_validation`
- `post_validation`
- `review_handoff`

Custom snake_case stage names are allowed when the issue needs a narrower local
handoff point.

## Process Inputs

Process summaries stay bounded and never record raw command lines.

- `--process <role:pid>` tracks one explicit pid.
- `--pid-file-process <role:path>` resolves one explicit pid file.
- If no process input is supplied, the collector records its own process as
  role `collector`.

## Privacy Rules

- Approved labels currently include local host `wuji` and CSM labels
  `opticon` and `nessus`.
- Unknown host labels are emitted as bounded redacted label `redacted_host`
  instead of failing open.
- Absolute host paths are rejected from the emitted row.
- GPU remains exact string `not_available` in this first slice.
- Raw command lines, env vars, usernames, and home-directory fragments are not
  recorded.

## Archive Rules

- Raw JSONL stays private evidence and must not be committed to Git.
- Archive refs use
  `issues/issue-<issue-number>/host-<host-ref>/capture-<capture-token>/...`
  under the provided S3 prefix.
- `capture-token` is a normalized UTC second-resolution form of
  `--captured-at`, for example `2026-06-20T09-30-00Z`.
- Allowed redaction statuses are:
  - `not_redacted_private_archive_manifest_only`
  - `redacted_private_archive`
  - `redacted_review_safe_summary`
- Unknown redaction statuses fail closed.
- `--upload-manifest` requires `--upload` so the manifest cannot claim a
  durable manifest object without the raw telemetry upload path.
