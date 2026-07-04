# AWS CodeFriend Build Lane - Issue #4838

## Status

Implemented with local contract proof and live AWS account-boundary proof.
Live CodeBuild execution is not claimed because the Agent Logic AWS account does
not yet contain a CodeBuild project for this lane.

## Implemented Surfaces

- `.github/workflows/aws-codefriend-build.yaml`
  - Manual `workflow_dispatch` only.
  - `dry-run` mode renders the CodeBuild request and uploads artifacts.
  - `start-build` mode requires GitHub OIDC, an account-hash secret, and a
    configured CodeBuild project.
  - Uses `contents: read` and `id-token: write`.
- `adl/tools/run_aws_codefriend_build_lane.sh`
  - Repo-native wrapper for dry-run and `codebuild start-build`.
  - Defaults local AWS authority to `agent-logic-admin` and `us-west-2`.
  - Verifies STS account identity by SHA-256 hash before live AWS work.
  - Does not print fresh account IDs, ARNs, user IDs, credentials, or tokens.
- `adl/tools/test_run_aws_codefriend_build_lane.sh`
  - Fake-AWS contract test for STS account hash check, CodeBuild request
    rendering, live `start-build` argument shape, dry-run no-AWS behavior,
    mismatch failure, and workflow trigger/secret contract.
- `docs/tooling/AWS_CODEFRIEND_BUILD_LANE.md`
  - Operator runbook for dry-run, live boundary, required GitHub configuration,
    and failure handling.

## Local Proof

Commands run from `/Users/daniel/git/agent-design-language/.worktrees/adl-wp-4838`:

```sh
bash -n adl/tools/run_aws_codefriend_build_lane.sh
bash -n adl/tools/test_run_aws_codefriend_build_lane.sh
python3 -m json.tool adl/config/validation_lane_selector.v0.91.6.json >/dev/null
bash adl/tools/test_run_aws_codefriend_build_lane.sh
bash adl/tools/test_select_validation_lanes.sh
bash -n adl/tools/*.sh
ruby -e 'require "yaml"; YAML.load_file(".github/workflows/aws-codefriend-build.yaml"); puts "PASS workflow yaml parse"'
```

Observed results:

- `PASS test_run_aws_codefriend_build_lane`
- `PASS test_select_validation_lanes`
- `PASS workflow yaml parse`

## AWS Account Boundary Proof

Command:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --dry-run \
  --check-account \
  --project-name adl-codefriend-build \
  --source-version HEAD \
  --out .adl/tmp/aws-codefriend-build/account-check-summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build/account-check
```

Observed result:

```text
PASS account_profile_resolved profile=agent-logic-admin account_matches_retained_proof=true
PASS aws_codefriend_build_dry_run project=adl-codefriend-build region=us-west-2 profile=agent-logic-admin
```

This proves the local AWS profile resolved to the same Agent Logic account hash
as the retained issue `#4603` proof. It did not start CodeBuild.

## Live CodeBuild Boundary

Bounded AWS read:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
aws codebuild batch-get-projects \
  --names adl-codefriend-build \
  --region us-west-2 \
  --output json
```

Observed result: no project was returned; `adl-codefriend-build` was reported in
`projectsNotFound`.

Bounded AWS read:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
aws codebuild list-projects \
  --region us-west-2 \
  --sort-by NAME \
  --output json
```

Observed result: `projects` was empty.

Therefore this issue does not claim live CodeBuild execution. Remaining setup
needed before `start-build` can prove end-to-end execution:

- create or designate the Agent Logic CodeBuild project
- configure the GitHub repository variable `AWS_CODEFRIEND_CODEBUILD_PROJECT`
- configure the GitHub repository secret `AWS_CODEFRIEND_BUILD_ROLE_ARN`
- configure the GitHub repository secret `AWS_CODEFRIEND_ACCOUNT_SHA256`

## Result Truth

The integrated repo lane is present and locally proven. Reviewers can dispatch a
safe dry-run workflow immediately after merge. A live AWS CodeBuild run is
fail-closed until the missing Agent Logic CodeBuild project and GitHub
configuration exist.
