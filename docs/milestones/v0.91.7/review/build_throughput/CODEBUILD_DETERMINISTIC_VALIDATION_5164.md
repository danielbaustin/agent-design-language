# CodeBuild Deterministic Validation Proof

Issue: `#5164`

Date: 2026-07-11

Platform: AWS CodeBuild `BUILD_GENERAL1_XLARGE`, Agent Logic business account

Final source SHA: `9089a4595ffe62b5a81ddba4ce9aec1513592c10`

## Operational Contract

The live CodeBuild project uses the immutable ADL builder image and performs no
per-job tool installation. Preflight proved Rust `1.96.1`, cargo-nextest
`0.9.140`, sccache `0.16.0`, LLD `18.1.3`, zstd `1.5.5`, and the requested
source SHA before validation.

The broad lane:

- normalizes tracked source mtimes to the resolved commit epoch
- restores one exact-SHA target archive through a single-version S3 read
- verifies the archive against SHA-256 object metadata before extraction
- runs `cargo nextest run --no-run` before testing
- atomically republishes the target through a temporary object and server-side
  promotion
- runs all tests with 18 nextest workers and no fail-fast
- streams redacted CloudWatch events with forward-token polling
- drains the terminal log and fails closed when required proof markers are
  absent

The target archive was reduced from the earlier 14.85 GB diagnostic shape to
approximately 2.25 GB by setting `CARGO_PROFILE_TEST_DEBUG=0`. This remains an
unoptimized test build but omits debugger symbols not consumed by validation.

## Seed Proof

Retained local summary:

```text
.adl/local-artifacts/issue-5164-codebuild/final-seed-9089a4595/summary.json
```

Result:

- CodeBuild status: `SUCCEEDED`
- BUILD phase: `142s`
- focused regression: `18 passed`, `22,609 skipped`
- Rust sccache hit rate: `100%`
- target cache publication: checksum verified and atomically promoted
- retained-log redaction verification: passed

## Consecutive Broad Proofs

Both runs used the same final SHA, builder image, buildspec, XLARGE shape,
18-worker nextest command, and exact target-cache key.

| Run | BUILD phase | Nextest time | Result | Compile requests | Cache restore/save |
|---|---:|---:|---|---:|---|
| Broad 1 | 344s | 263.269s | 22,609 passed; 18 skipped | 0 | verified / verified |
| Broad 2 | 352s | 262.689s | 22,609 passed; 18 skipped | 0 | verified / verified |

Retained local summaries and redacted logs:

```text
.adl/local-artifacts/issue-5164-codebuild/final-broad-1-9089a4595/
.adl/local-artifacts/issue-5164-codebuild/final-broad-2-9089a4595/
```

Each machine-readable summary records:

- `build_status: SUCCEEDED`
- `build_succeeded: true`
- `retained_log_redaction_verified: true`
- `self_verification_passed: true`
- verified preflight, prebuilt toolchain, source-mtime normalization, prepare,
  build-command completion, target restore, and target save markers

The final S3 inspection found one stable target object of `2,249,397,877`
bytes and zero temporary `.upload-*` objects for the final cache key.

## Result

The final two-run counter is `2/2`. The CodeBuild broad-nextest lane is live,
cached, deterministic for an exact source SHA, self-verifying, redacted, and
does not install build tools inside jobs.
