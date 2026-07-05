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
