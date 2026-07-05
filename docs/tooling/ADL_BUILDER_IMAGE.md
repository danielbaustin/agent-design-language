# ADL Builder Image

The ADL builder image is the shared WP-06 validation toolchain image for
repeatable Rust validation lanes. It is intended for CodeBuild, AWS Spot EC2,
Nessus, and local Docker-compatible runners.

The image is defined at `adl/docker/adl-builder/Dockerfile` and includes:

- Rust toolchain with `rustfmt` and `clippy`
- `sccache` 0.16
- `clang` and `lld`
- AWS CLI v2 for CodeBuild credential export and S3-backed `sccache`
- GNU `time` for benchmark wrappers that use `/usr/bin/time`
- common validation helper dependencies such as `git`, `jq`, `python3`, and
  OpenSSH client tools

## Setup

Build or publish the image with:

```sh
adl/tools/setup_adl_builder_image.sh \
  --image-uri <image-uri> \
  --build
```

For the Agent Logic AWS account, use the business profile:

```sh
AWS_PROFILE=agent-logic-admin \
adl/tools/setup_adl_builder_image.sh \
  --ecr-repository adl-builder \
  --tag v0.91.7 \
  --ensure-ecr \
  --push \
  --write-env .adl/local/adl-builder-image.env
```

The generated env file exports one image URI under the lane-specific names:

- `ADL_BUILDER_IMAGE`
- `ADL_AWS_CODEFRIEND_IMAGE`
- `ADL_AWS_SPOT_BUILDER_IMAGE`
- `ADL_NESSUS_BUILDER_IMAGE`
- `ADL_LOCAL_BUILDER_IMAGE`

Do not commit generated env files or account-specific image URIs.

If Docker's direct ECR push path is unreliable from the operator machine, use
S3 as the transit layer:

```sh
docker save <image-uri> -o .adl/local/builder-image/adl-builder-v0.91.7-amd64.tar
AWS_PROFILE=agent-logic-admin \
aws s3 cp \
  .adl/local/builder-image/adl-builder-v0.91.7-amd64.tar \
  s3://adl-codefriend-build-cache/builder-images/adl-builder/v0.91.7/amd64/adl-builder-v0.91.7-amd64.tar
AWS_PROFILE=agent-logic-admin \
adl/tools/import_adl_builder_image_from_s3_to_ecr.sh \
  --s3-uri s3://adl-codefriend-build-cache/builder-images/adl-builder/v0.91.7/amd64/adl-builder-v0.91.7-amd64.tar \
  --image-uri <ecr-image-uri> \
  --ensure-role-policy \
  --create-project \
  --start
```

That importer uses a privileged, purpose-specific CodeBuild project to
`docker load` the S3 tar and push it to ECR from inside AWS.

The importer project is `adl-builder-image-import`. It uses the existing
CodeBuild service role, the existing `/aws/codebuild/adl-codefriend-build` log
group, and a narrowly scoped inline role policy for the builder-image S3 prefix
and ECR push operations.

## Cache Contract

The image removes repeated toolchain setup from validation runs. It does not
replace compiler-output caching.

- CodeBuild should use native S3 `sccache` for compiler artifacts.
- AWS Spot EC2 should keep using the warm EBS cache for retained dependency and
  target artifacts.
- Nessus should keep using its persistent remote cache root when not running in
  a container; when `ADL_NESSUS_BUILDER_IMAGE` is set, the runner mounts the
  persistent target and sccache directories into the container.
- Local Docker runs should mount target and sccache directories explicitly.
- Wuji is ARM64. Do not use the current amd64-only `adl-builder:v0.91.7-fixed`
  image as a wuji benchmark through QEMU emulation; publish an arm64 or
  multi-arch image first.

## Nessus

Nessus can execute a validation command inside the image when Docker or Podman
is available:

```sh
ADL_NESSUS_BUILDER_IMAGE=<image-uri> \
adl/tools/run_nessus_remote_validation.sh \
  --command 'bash adl/tools/test_select_validation_lanes.sh' \
  --git-ref origin/main \
  --local-artifact-dir artifacts/nessus-builder-run
```

The run summary records `builder_image`, `builder_runtime`, and
`resolved_builder_runtime`.

## CodeBuild And Spot

The CodeBuild lane should consume `ADL_AWS_CODEFRIEND_IMAGE` as the custom
environment image directly. Do not rebuild the image inside each CodeBuild run;
publish a versioned ECR tag once, configure CodeBuild with
`imagePullCredentialsType=SERVICE_ROLE`, and run validation commands in that
environment. The Spot lane should consume `ADL_AWS_SPOT_BUILDER_IMAGE`
when the instance has Docker or Podman available; otherwise it can continue
using the image as the canonical setup definition while keeping the warm EBS
cache path.

The image path is intentionally separate from cache selection. A custom image
plus native S3 `sccache` is the expected fast repeated CodeBuild configuration.
If CodeBuild runs the image in a nested container, pass the CodeBuild role
credentials and the exact S3 `sccache` bucket, region, and key prefix into the
container. A successful nested-container run on 2026-07-05 proved execution but
only reached a 0.26% total hit rate and 0% Rust hit rate, so that shape is not
the preferred fast path until its cache key/path mismatch is fixed.

The AWS-managed `aws/codebuild/standard:7.0` image is not a drop-in Rust image
for this lane. A 2026-07-05 probe with `runtime-versions: rust: latest` failed
before execution because that managed image did not advertise a Rust runtime.
Use the ADL ECR image for no-install Rust validation.

## Launching The Next Build

Use the repo wrappers rather than rebuilding command lines by hand:

```sh
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --run \
  --check-account \
  --project-name adl-codefriend-build \
  --source-version <branch-or-sha> \
  --wait
```

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --run \
  --check-account \
  --git-ref <branch-or-sha> \
  --command 'bash adl/tools/run_build_platform_benchmark.sh --platform aws_spot --cache-posture warm_ebs_cache' \
  --instance-type m7a.2xlarge \
  --json
```

For Nessus, keep using `adl/tools/run_nessus_remote_validation.sh` with
`ADL_NESSUS_BUILDER_IMAGE` set. For wuji, wait for an arm64 or multi-arch image
before claiming an image-backed Docker benchmark.
