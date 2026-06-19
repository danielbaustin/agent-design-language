# CI Log Archive S3 Contract (#4225)

## Summary

ADL now has a bounded CI/build-log archive surface for preserving verbose GitHub Actions logs without storing raw logs in Git or leaving permanent local piles behind.

The contract is deliberately split:

- raw CI/build logs are evidence and belong in S3-compatible object storage
- small manifests and timing summaries are memory and can be tracked, reviewed, indexed, and consumed by ObsMem/community-memory surfaces

## Command

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- \
  tooling ci-log-archive summarize \
  --logs-dir <extracted-github-actions-logs> \
  --out <manifest.json> \
  --s3-prefix s3://<bucket>/<prefix> \
  --repo danielbaustin/agent-design-language \
  --pr <pr-number> \
  --run-id <github-actions-run-id> \
  --commit <sha>
```

Optional live upload requires an explicit raw log zip and configured AWS-compatible CLI credentials:

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- \
  tooling ci-log-archive summarize \
  --logs-dir <extracted-github-actions-logs> \
  --raw-zip <github-actions-logs.zip> \
  --upload \
  --out <manifest.json> \
  --s3-prefix s3://<bucket>/<prefix>
```

If `--upload` is not set, the manifest records `upload_not_run`. If `--upload` is set without `--raw-zip`, the command fails closed.

## Manifest Shape

The command writes `adl.ci_log_archive_manifest.v1` JSON with:

- `source.repo`
- `source.pr_number`
- `source.run_id`
- `source.commit`
- `archive.backend`
- `archive.raw_log_ref`
- `archive.upload_status`
- `archive.redaction_status`
- `archive.local_retention`
- `timing_summary.threshold_seconds`
- `timing_summary.entry_count`
- `timing_summary.over_threshold_count`
- `timing_summary.a_small_count`
- `timing_summary.b_large_count`
- `timing_entries[]`
- `consumption_policy`

## S3 Object Convention

Given:

- prefix: `s3://adl-ci-logs/v0.91.6`
- repo: `danielbaustin/agent-design-language`
- PR: `4152`
- run id: `27840922589`

The raw log object ref becomes:

```text
s3://adl-ci-logs/v0.91.6/danielbaustin-agent-design-language/pr-4152/run-27840922589/github-actions-logs.zip
```

This makes build logs addressable by project, PR, and GitHub Actions run without encoding local machine paths.

## A/B Validation-Lane Consumption

Every manifest classifies observed durations against the threshold, defaulting to `60` seconds:

- `A_small`: timing entry is at or below the threshold
- `B_large`: timing entry is greater than the threshold

#4223 should consume these manifests to update the validation-lane index. It should not require extracted logs or raw zip files to remain local after the manifest and S3 object are durable.

## Runtime And Memory Boundary

For runtime, ObsMem, and community memory:

- raw logs remain evidence
- manifests and summaries are memory
- raw logs are not ingested into ObsMem by default
- memory records preserve provenance to `archive.raw_log_ref`
- community-memory surfaces consume redacted summaries, not raw logs

## Redaction And Security Posture

The default manifest status is:

```text
not_redacted_private_archive_manifest_only
```

Allowed statuses are intentionally narrow:

- `not_redacted_private_archive_manifest_only`
- `redacted_private_archive`
- `redacted_review_safe_summary`

That means the raw object is intended for private evidence storage unless a later redaction lane certifies it for broader review or community-memory publication.

The command does not claim to redact raw logs. It records a validated redaction posture so later surfaces cannot mistake private evidence for publishable memory. Unknown redaction statuses fail closed instead of being copied into durable manifests.

## Local Retention Policy

Local logs are temporary input only. The durable surfaces are:

- S3 raw log object
- tracked manifest/summary

Local extracted logs and local zip files should be deleted by the caller once archive/upload and manifest production are complete.

## Failure Behavior

- Missing `--logs-dir`: fail.
- Non-S3 `--s3-prefix`: fail.
- `--upload` without `--raw-zip`: fail.
- Missing or failed `aws s3 cp`: fail.
- No upload requested: succeed with `upload_not_run`.

## Validation Performed

Focused Rust tests cover:

- GitHub step log duration extraction.
- Rust `finished in ...` summary extraction.
- `A_small` / `B_large` threshold classification.
- S3 object-ref construction.
- Fail-closed upload behavior when `--upload` lacks `--raw-zip`.

## Non-claims

This issue does not claim:

- automatic download of GitHub Actions logs
- automatic attachment to every PR closeout
- raw-log redaction
- ObsMem ingestion implementation
- a complete artifact lake
