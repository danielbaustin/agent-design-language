# CodeBuild XLARGE Native S3 sccache Repeat Proof

Issue: `#4838`
Related work: `#4879`
Platform: AWS CodeBuild `BUILD_GENERAL1_XLARGE`
Date: 2026-07-04

This records the first successful CodeBuild repeated-build proof where
`sccache` used its native S3 backend instead of a CodeBuild local cache archive.

## Result

- Build id: `adl-codefriend-build:b801a66f-322e-41a5-8b2e-497a32d678a9`
- Status: `SUCCEEDED`
- CodeBuild phase time:
  - Provisioning: `4s`
  - Download source: `13s`
  - Install: `11s`
  - Build: `180s`
  - Post build: `32s`
- Benchmark line:
  - `ADL_BUILD_PLATFORM_BENCHMARK platform=codebuild build_seconds=101 test_seconds=79 total_seconds=180 status=passed`
- `sccache`:
  - Client version: `0.16.0`
  - Cache location: S3 bucket `adl-codefriend-build-cache`, prefix `codebuild/cache/sccache/x86_64-unknown-linux-gnu`
  - Compile requests: `845`
  - Compile requests executed: `770`
  - Cache hits: `764`
  - Cache hits (Rust): `366`
  - Cache misses: `2`
  - Cache misses (Rust): `2`
  - Cache hits rate: `99.74%`
  - Cache hits rate (Rust): `99.46%`
  - Cache read errors: `0`
  - Cache write errors: `0`

## Interpretation

The result proves the combined xlarge, native S3 `sccache`, `lld`,
`CARGO_INCREMENTAL=0`, and stable `/workspace` posture gives the expected
repeated-build speedup. It does not isolate a single causal variable. A custom
ADL builder image should reduce the remaining setup/install overhead while
preserving S3 `sccache` as the compiler artifact cache.

Follow-up issue `#4879` adds the shared builder-image path and records the
direct local Docker-to-ECR push as unreliable on this operator machine. The
preferred publication path is now S3 transit plus an AWS-side CodeBuild importer
so transfer retries are handled by the S3 upload path and ECR push happens
inside AWS.

Do not treat this proof as a release artifact for production binaries. It is
WP-06 validation-throughput evidence for debug validation builds.

## Current Re-Run

The same lane was re-applied to the live `adl-codefriend-build` project on
2026-07-05 with `BUILD_GENERAL1_XLARGE` and rerun through the repo wrapper.

- Build id: `adl-codefriend-build:5e4ab258-a121-4561-8196-61249e60f793`
- Status: `SUCCEEDED`
- CodeBuild wall-clock: `310s` from CodeBuild start to end
- Benchmark line:
  - `ADL_BUILD_PLATFORM_BENCHMARK platform=codebuild build_seconds=96 test_seconds=75 total_seconds=171 status=passed`
- `sccache`:
  - Client version: `0.16.0`
  - Cache location: S3 bucket `adl-codefriend-build-cache`, prefix `codebuild/cache/sccache/x86_64-unknown-linux-gnu`
  - Compile requests: `845`
  - Compile requests executed: `770`
  - Cache hits: `765`
  - Cache hits (Rust): `367`
  - Cache misses: `1`
  - Cache misses (Rust): `1`
  - Cache hits rate: `99.87%`
  - Cache hits rate (Rust): `99.73%`
  - Cache read errors: `0`
  - Cache write errors: `0`

This proves the current live CodeBuild project configuration, not only a stale
retained run. The install phase still downloads and installs `lld`/`clang`, so
the reusable builder image from `#4879` remains the expected next optimization
for setup time.
