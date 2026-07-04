# CI Log Archive To S3

`adl tooling ci-log-archive summarize` turns extracted GitHub Actions logs into
a small timing manifest and can upload both the raw log zip and the manifest to
private S3 evidence storage.

## Command

```text
adl tooling ci-log-archive summarize \
  --logs-dir <extracted-github-actions-logs> \
  --raw-zip <github-actions-logs.zip> \
  --upload \
  --upload-manifest \
  --aws-profile agent-logic-admin \
  --out <manifest.json> \
  --s3-prefix s3://<bucket>/<prefix> \
  --repo danielbaustin/agent-design-language \
  --pr <pr-number> \
  --run-id <github-actions-run-id> \
  --commit <sha>
```

`--upload-manifest` requires `--upload`, so the manifest cannot claim durable S3
publication unless the raw log upload path is also explicit and successful.

## Archive Shape

The manifest schema is `adl.ci_log_archive_manifest.v1`.

- `archive.raw_log_ref` points to `github-actions-logs.zip`.
- `archive.manifest_ref` points to `ci-log-archive-manifest.v1.json`.
- `archive.upload_status` is `uploaded` only after the raw log zip upload
  succeeds.
- `archive.manifest_upload_status` is `uploaded` only after the manifest upload
  succeeds.
- `archive.redaction_status` records whether the archive is private evidence or
  review-safe summary material.

## AWS Boundary

ADL-related archive work should use the Agent Logic business AWS profile:
`agent-logic-admin`. Do not rely on the operator default profile for CI archive
proofs, and do not record AWS credentials, token contents, or local credential
paths in manifests or lifecycle cards.

Raw CI logs remain private evidence. Tracked manifests and redacted summaries
are the reviewable memory surfaces.

