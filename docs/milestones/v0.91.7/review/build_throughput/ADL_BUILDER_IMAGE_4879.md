# ADL Builder Image Proof for `#4879`

Issue: `#4879`
Platform scope: CodeBuild, AWS Spot EC2, Nessus, and local Docker-compatible
runners
Date: 2026-07-04

## Result

The reusable ADL builder image path is operational.

- Local image build: passed
- Local image smoke:
  - `rustc 1.96.1`
  - `cargo 1.96.1`
  - `sccache 0.16.0`
  - `aws-cli/2.35.15`
  - GNU `time`
  - `Ubuntu LLD 18.1.3`
- S3 transit object:
  - Bucket/key: `s3://adl-codefriend-build-cache/builder-images/adl-builder/v0.91.7/amd64/adl-builder-v0.91.7-amd64.tar`
  - Size: `493493248` bytes
  - Multipart ETag: `462318df1697019e779587560c057d00-59`
  - Local tar SHA256: `b92b9202b99a8cfd67e5a9c543129f779514a42956cac3fcd21a8c257a1dcbd3`
- ECR image:
  - Repository tag: `adl-builder:v0.91.7`
  - Digest: `sha256:66dc8ac03e3ea23583747f825856830923e8640df3f7a57830af15e8d691c7a2`
  - Reported image size: `494346902` bytes
- AWS-side importer:
  - CodeBuild project: `adl-builder-image-import`
  - Successful build id: `adl-builder-image-import:1d1645b7-a1be-4444-a35e-cc98d0524f77`
  - Build phase duration: `44s`
  - Provisioning duration: `7s`
  - Status: `SUCCEEDED`

## Operational Notes

Direct local Docker-to-ECR push was attempted first. It built the image
successfully and uploaded some layers, but the Docker client did not finalize
the tag reliably from the local Colima path. The durable path is now:

1. `docker save` the verified image to an ignored local tarball.
2. Upload the tarball to S3.
3. Use the privileged `adl-builder-image-import` CodeBuild project to
   `docker load` the S3 object and push the image to ECR from inside AWS.

The CodeBuild importer initially exposed two missing operational details:

- the importer project had to use the existing `/aws/codebuild/adl-codefriend-build`
  log group because the shared CodeBuild role already had access there
- the shared CodeBuild role needed narrowly scoped access to read the
  builder-image S3 prefix and push to the target ECR repository

Both are now captured in repo tooling.

Pre-PR review also found that Nessus builder-image mode should not depend on
host apt state. The runner now skips host `apt-get update` when
`ADL_NESSUS_BUILDER_IMAGE` or `--builder-image` is set, records that skip in
the run log, and has a fixture regression with failing host apt plus a passing
containerized command.

## Validation

Focused local validation:

- `bash adl/tools/test_adl_builder_image.sh`
- `bash adl/tools/test_import_adl_builder_image_from_s3_to_ecr.sh`
- `bash adl/tools/test_run_nessus_remote_validation.sh`
- `bash adl/tools/test_run_nessus_remote_validation.sh` includes builder-image
  mode with failing host apt to prove the containerized path is independent of
  unrelated host package repository state.
- `bash -n adl/tools/setup_adl_builder_image.sh adl/tools/import_adl_builder_image_from_s3_to_ecr.sh adl/tools/run_nessus_remote_validation.sh adl/tools/test_adl_builder_image.sh adl/tools/test_import_adl_builder_image_from_s3_to_ecr.sh adl/tools/test_run_nessus_remote_validation.sh`
- `git diff --check`

Live AWS validation:

- ECR repository `adl-builder` exists.
- S3 transit object exists at the key above.
- CodeBuild importer pushed ECR tag `v0.91.7` successfully.

## Follow-up Runtime Test

The original publication proof built, shipped, and imported the image, but did
not prove lane execution. A follow-up run on 2026-07-05 tested the image as a
runtime container and exposed two missing image dependencies:

- `/usr/bin/time`, required by benchmark helpers
- AWS CLI v2, required by CodeBuild credential export and S3-backed `sccache`

The Dockerfile now includes both dependencies, and local smoke verified:

- `rustc 1.96.1`
- `cargo 1.96.1`
- `sccache 0.16.0`
- `aws-cli/2.35.15`
- GNU `time`

Discarded nested-CodeBuild diagnostic:

- Project: `adl-codefriend-build`
- Build id: `adl-codefriend-build:f1021b3b-e25b-4a7f-823e-1f8d77da51e1`
- Compute: `BUILD_GENERAL1_XLARGE`
- Shape: CodeBuild standard image built `adl-builder:measure` inside AWS, then
  executed the benchmark command inside the fixed builder image with S3
  `sccache` credentials and key prefix passed through.
- Image build time inside AWS: `46s`
- CodeBuild phase time: `566s`
- Cold target pass:
  - command: `cargo build --manifest-path adl/Cargo.toml --locked --bin adl-pr-doctor`
    followed by `cargo test --manifest-path adl/Cargo.toml --locked --lib provider_communication -- --nocapture`
  - build: `260s`
  - test: `229s`
  - total: `489s`
  - result: `passed`, `21 passed; 0 failed; 1554 filtered out`
  - `sccache`: `845` compile requests, `2` hits, `764` misses,
    `0.26%` total hit rate, `0%` Rust hit rate
- Warm target pass:
  - build: `0s`
  - test: `0s`
  - total: `0s`
  - result: `passed`

Interpretation: the image runtime path works, and the mounted target directory
works for same-host repeated execution. The nested CodeBuild image shape did
not reuse the existing native CodeBuild S3 Rust cache; its cache key/path
posture produced a cold-cache result. Do not use the nested-container shape for
the steady-state lane, and do not report this row as a platform benchmark.

## Direct CodeBuild Image Fix

The correct CodeBuild path is to publish the fixed image once and run CodeBuild
directly on that ECR image. Rebuilding the image inside every benchmark run is
not the operational path.

One-time image publication:

- Build id: `adl-codefriend-build:fed2a740-aabd-47d1-9442-9343b02c884d`
- Tag: `adl-builder:v0.91.7-fixed`
- Result: `SUCCEEDED`
- Build phase duration: `88s`

Managed-image probe:

- Build id: `adl-codefriend-build:00a6ce14-aa97-48e2-bfec-5bf24f0fa6b7`
- Image: `aws/codebuild/standard:7.0`
- Result: `FAILED` before command execution
- Cause: CodeBuild reported that the managed image did not know a `rust`
  runtime; available runtimes were dotnet, golang, java, nodejs, php, python,
  and ruby.

Direct custom-image run:

- Build id: `adl-codefriend-build:f9da1c33-e6a3-4c1f-b917-93a9d170556c`
- Image: `adl-builder:v0.91.7-fixed`
- Compute: `BUILD_GENERAL1_XLARGE`
- Image pull: `SERVICE_ROLE`
- Privileged mode: `false`
- Rebuilt image during run: no
- Build phase duration: `195s`
- Provisioning duration: `22s`
- Command result: `passed`, `21 passed; 0 failed; 1554 filtered out`
- Benchmark line:
  `ADL_BUILD_PLATFORM_BENCHMARK platform=codebuild-xlarge-direct-fixed-builder-image build_seconds=119 test_seconds=76 total_seconds=195 status=passed`
- `sccache`: `845` compile requests, `763` hits, `3` misses, `99.61%` total
  hit rate, `99.18%` Rust hit rate

The live `adl-codefriend-build` project was left in the working direct-image
state after this run:

- image: `adl-builder:v0.91.7-fixed`
- compute: `BUILD_GENERAL1_XLARGE`
- privileged: `false`
- image pull credentials: `SERVICE_ROLE`

## Nessus Image Path Repair

The first Nessus fixed-image run failed because the WSL filesystem was full,
not because the image was built on Nessus. The container was compiling ADL
inside the mounted target cache when the host reported `No space left on
device`.

Disk repair:

- Before cleanup: `/dev/sdc` was `251G` used, `0` free, `100%`
- Removed only rebuildable ADL validation cache:
  `/root/adl-remote-runner/cache/target`
- Removed cache size: `81G`
- Immediately after cleanup: `69G` free
- After pulling `adl-builder:v0.91.7-fixed` and rerunning the benchmark:
  `59G` free
- Retained cache after rerun:
  - target cache: `7.5G`
  - sccache cache: `1.4G`

Fixed-image pull:

- Image: `adl-builder:v0.91.7-fixed`
- Pull result: `Downloaded newer image`
- Digest: `sha256:cacba822a59fe610cb2f963a107ba33e4bf4969322a1c8d6aa53fb6383de3adf`

Nessus image-backed benchmark:

- Run id: `nessus-fixed-builder-image-inline-20260705T060710Z`
- Image: `adl-builder:v0.91.7-fixed`
- Runtime: Docker
- Git ref: `3f8436beea30e261fc8bca85961abb014522f7e8`
- Result: `passed`
- Runner elapsed: `217s`
- Command result: `21 passed; 0 failed; 1554 filtered out`
- Benchmark line:
  `ADL_BUILD_PLATFORM_BENCHMARK platform=nessus-fixed-builder-image build_seconds=55 test_seconds=157 total_seconds=212 status=passed`

This is an image-backed Nessus result after target-cache cleanup. It is not a
warm-target best-case result, because the target cache had just been pruned to
restore disk headroom.
