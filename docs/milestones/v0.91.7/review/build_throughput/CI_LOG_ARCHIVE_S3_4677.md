# CI Log Archive S3 Proof (#4677)

## Summary

#4677 promotes the existing `ci-log-archive` summarizer into an integrated S3
archive surface for CI logs.

Implemented behavior:

- `--upload-manifest` uploads the generated manifest alongside the raw log zip.
- `--upload-manifest` fails closed unless `--upload` is also set.
- `--aws-profile <profile>` lets the command use the approved
  `agent-logic-admin` profile instead of relying on the operator default AWS
  profile.
- The manifest records both `archive.raw_log_ref` and `archive.manifest_ref`,
  plus separate raw-log and manifest upload statuses.

## Live Proof

Local synthetic GitHub Actions logs were summarized and uploaded under the
Agent Logic AWS profile using a private S3 proof prefix.

Observed manifest fields:

```yaml
schema_version: adl.ci_log_archive_manifest.v1
archive.upload_status: uploaded
archive.manifest_upload_status: uploaded
timing_summary.over_threshold_count: 1
```

The proof used a synthetic raw log zip, not a production GitHub Actions log
archive. It proves the upload path, manifest upload path, S3 key convention,
timing summary generation, and redaction/status recording without committing
raw logs or local temporary paths.

## Validation

Focused validation:

```text
cargo test --manifest-path adl/Cargo.toml ci_log_archive -- --nocapture
```

This covered duration extraction, lane classification, S3 ref construction,
upload fail-closed behavior, and the new manifest-upload guard.

```text
cargo fmt --manifest-path adl/Cargo.toml --all --check
```

This verified Rust formatting after the implementation.

## Non-Claims

- This proof does not claim automatic GitHub Actions log download.
- This proof does not claim raw log redaction.
- This proof does not claim every PR automatically archives logs.
- This proof does not ingest raw CI logs into ObsMem or community memory.

