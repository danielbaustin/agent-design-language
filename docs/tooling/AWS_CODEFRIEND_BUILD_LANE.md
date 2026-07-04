# AWS CodeFriend Build Lane

## Status

Implemented for issue `#4838` as a manual GitHub Actions plus AWS CodeBuild
lane. The lane is intentionally separate from the AWS Spot EC2 remote validation
lane.

## Entry Points

- GitHub Actions workflow:
  `.github/workflows/aws-codefriend-build.yaml`
- Repo-native wrapper:
  `adl/tools/run_aws_codefriend_build_lane.sh`
- Local contract test:
  `adl/tools/test_run_aws_codefriend_build_lane.sh`

## Trigger Contract

The workflow only uses `workflow_dispatch`. It does not run on ordinary pull
requests or pushes.

Dispatch input `mode` controls the boundary:

- `dry-run` renders the CodeBuild request and uploads the request/summary
  artifact without calling AWS.
- `start-build` assumes the configured Agent Logic AWS role through GitHub OIDC,
  verifies the STS account hash, then calls `codebuild start-build`.

## Required GitHub Configuration

Repository variable:

- `AWS_CODEFRIEND_CODEBUILD_PROJECT`: CodeBuild project name.
- `AWS_CODEFRIEND_REGION`: optional region override; defaults to `us-west-2`.

Repository secrets:

- `AWS_CODEFRIEND_BUILD_ROLE_ARN`: OIDC role for starting the CodeFriend
  CodeBuild project.
- `AWS_CODEFRIEND_ACCOUNT_SHA256`: SHA-256 of the approved Agent Logic AWS
  account ID.

The workflow uses `contents: read` and `id-token: write`. It does not require a
long-lived AWS access key. The GitHub Actions path calls the wrapper with
`--profile env` so the AWS CLI uses the OIDC credentials exported by
`aws-actions/configure-aws-credentials`.

## Local AWS Account Contract

Local live runs default to:

```sh
ADL_AWS_PROFILE=agent-logic-admin
ADL_AWS_REGION=us-west-2
```

The wrapper compares the live STS account ID hash against the retained Agent
Logic account proof from issue `#4603` unless
`ADL_AWS_CODEFRIEND_ACCOUNT_SHA256` or `--expected-account-sha256` is provided.
It reports only whether the hash matched; it does not print fresh account IDs,
ARNs, user IDs, credentials, or token values.

## Local Dry-Run Proof

```sh
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --dry-run \
  --project-name adl-codefriend-build \
  --source-version HEAD \
  --env ADL_CODEFRIEND_BUILD_COMMAND='bash adl/tools/run_pr_fast_test_lane.sh' \
  --out .adl/tmp/aws-codefriend-build/summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build \
  --print-command
```

## Live Start-Build Boundary

Live execution may incur AWS CodeBuild charges and requires the Agent Logic
business account:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --run \
  --check-account \
  --project-name "$ADL_AWS_CODEFRIEND_CODEBUILD_PROJECT" \
  --source-version HEAD \
  --env ADL_CODEFRIEND_BUILD_COMMAND='bash adl/tools/run_pr_fast_test_lane.sh' \
  --out .adl/tmp/aws-codefriend-build/summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build
```

## Failure Handling

The lane fails closed when:

- the CodeBuild project is missing
- live mode is requested without the OIDC role secret or account-hash secret
- STS resolves to the wrong account hash
- `aws codebuild start-build` fails
- artifact upload cannot find generated artifacts

The resulting summary and request/response JSON are retained as workflow
artifacts for review.
