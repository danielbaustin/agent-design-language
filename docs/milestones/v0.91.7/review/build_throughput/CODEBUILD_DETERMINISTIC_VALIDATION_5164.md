# CodeBuild Deterministic Validation Proof

Issue: `#5164`

Date: 2026-07-11

Platform: AWS CodeBuild `BUILD_GENERAL1_XLARGE`, Agent Logic business account

Final live-tested implementation SHA: `eec86b3592edbb464828597cd914f6b5e53106ac`

The tracked proof update that records these results is an evidence-only tail;
it does not change the live-tested build or runtime implementation.

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

## Final-Head Proof Sequence

An initial final-head run on `1e941f5858a09f25c043b41f0e1ed35cd2a0451a`
ran the complete suite and exposed one stale healthy-Chronosense readiness
assertion: 22,697 tests passed, one failed, and 18 were skipped. The assertion
was corrected, its positive and negative neighborhood passed locally, and the
final two-run sequence restarted from zero on `eec86b359...`.

Both final runs used the same exact SHA, immutable builder image, buildspec,
XLARGE shape, 18-worker nextest command, and target-cache key.

| Run | Total wall time | BUILD phase | Prepare | Nextest | Result | Compile activity | Target cache |
|---|---:|---:|---:|---:|---|---|---|
| Final 1 | 490.593s | 462s | 165s | 267.374s | 22,698 passed; 18 skipped | 982 requests; 1 compilation; 99.87% aggregate hit rate | exact-SHA miss; checksum-verified atomic save |
| Final 2 | 365.575s | 357s | 0.38s | 268.414s | 22,698 passed; 18 skipped | 0 requests; 0 compilations | checksum-verified exact-SHA hit |

Retained local summaries and redacted logs:

```text
.adl/local-artifacts/issue-5164-codebuild/final-head-broad-1-eec86b359/
.adl/local-artifacts/issue-5164-codebuild/final-head-broad-2-eec86b359/
```

Each summary records `build_status: SUCCEEDED`, `build_succeeded: true`,
`self_verification_passed: true`, and retained-log redaction success. Each live
preflight verified the requested source SHA, digest-pinned image, Rust 1.96.1,
cargo-nextest 0.9.140, sccache 0.16.0, LLD 18.1.3, zstd 1.5.5, and
`per_job_install=false` before validation.

At the current Oregon on-demand XLARGE rate of `$0.0798` per build minute,
rounded up per run, estimated CodeBuild compute was `$0.7182` for Final 1 and
`$0.5586` for Final 2. S3 cache operations and CloudWatch logging are separate
small usage-dependent charges.

## Earlier Implementation Proof

Before the final rebase/runtime-auth compatibility tail, the lane also passed
two consecutive broad runs on `9089a4595ffe62b5a81ddba4ce9aec1513592c10`:

- CodeBuild status: `SUCCEEDED`
- Broad 1: BUILD `344s`; nextest `263.269s`; 22,609 passed; 18 skipped
- Broad 2: BUILD `352s`; nextest `262.689s`; 22,609 passed; 18 skipped
- both runs recorded zero compile requests and verified target restore/save

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

The final-head two-run counter is `2/2`. The CodeBuild broad-nextest lane is
live, cached, deterministic for an exact source SHA, self-verifying, redacted,
and does not install build tools inside jobs.
