# AWS CodeBuild Remote Validation Evaluation for `#4316`

Status: `gated_later_pilot_recommended`
Issue: `#4316`
Sprint umbrella: `#4310`
Date: 2026-06-21

## Scope

This packet evaluates AWS CodeBuild-hosted GitHub Actions runners as a future
remote validation option for ADL.

It does not:

- create AWS resources
- modify production GitHub Actions workflows
- publish AWS account IDs, GitHub tokens, or other secrets
- claim live performance or cost measurements that were not run in this issue

## Sources

Official docs:

- [AWS CodeBuild runner tutorial](https://docs.aws.amazon.com/codebuild/latest/userguide/action-runner.html)
- [AWS CodeBuild GitHub source connection modes](https://docs.aws.amazon.com/codebuild/latest/userguide/access-tokens-github-overview.html)
- [GitHub OIDC in AWS](https://docs.github.com/en/actions/how-tos/secure-your-work/security-harden-deployments/oidc-in-aws)
- [GitHub self-hosted runners](https://docs.github.com/en/actions/concepts/runners/self-hosted-runners)
- [GitHub secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)
- [AWS CodeBuild pricing](https://aws.amazon.com/codebuild/pricing/)

Repo evidence:

- [SCCACHE_LOCAL_VALIDATION_4311.md](../build_throughput/SCCACHE_LOCAL_VALIDATION_4311.md)
- [RUST_LINKER_LOCAL_VALIDATION_4312.md](../build_throughput/RUST_LINKER_LOCAL_VALIDATION_4312.md)
- [CARGO_TARGET_DIR_RELOCATION_4313.md](../build_throughput/CARGO_TARGET_DIR_RELOCATION_4313.md)
- [SAFE_BUILD_ARTIFACT_CLEANUP_4314.md](../build_throughput/SAFE_BUILD_ARTIFACT_CLEANUP_4314.md)
- [Issue #4317](https://github.com/danielbaustin/agent-design-language/issues/4317)

## Decision

Recommendation: do not start a CodeBuild pilot in `v0.91.6`.

Instead, keep CodeBuild in the Phase 2 candidate set and revisit it only as a
gated later pilot if long validation lanes remain a real bottleneck after the
local build-throughput wins from `#4311` through `#4314`.

Rationale:

- ADL already captured meaningful local wins in this sprint:
  - `#4311` measured a warm `sccache` rebuild path at `46.04s` versus a
    `68.10s` clean baseline for `adl-pr-doctor`
  - `#4312` measured `rust-lld` at `64.07s` versus `71.18s` for a clean build
  - `#4313` and `#4314` reduced disk-pressure and cleanup risk around large
    Rust artifacts
- existing follow-on `#4317` already captures an owned-runner proof path for
  the remote-runner abstraction before cloud rollout

Interpretation:

- CodeBuild is feasible
- CodeBuild is not yet the next narrowest proof step for ADL
- a later pilot should be narrow, optional, and isolated from the main CI lanes

## What AWS Officially Supports

AWS documents a first-party flow where CodeBuild receives GitHub Actions
workflow-job webhooks and starts an ephemeral self-hosted runner for each job
([AWS CodeBuild runner tutorial](https://docs.aws.amazon.com/codebuild/latest/userguide/action-runner.html)).

Key official behavior:

- the CodeBuild project type is a `Runner project`
- the webhook normally receives `WORKFLOW_JOB_QUEUED` events
- the workflow routes jobs with:

```yaml
runs-on:
  - codebuild-<project-name>-${{ github.run_id }}-${{ github.run_attempt }}
```

- if the project name in `runs-on` does not match the CodeBuild project name,
  CodeBuild will not process the webhook and the workflow may hang
- CodeBuild can override image, instance size, fleet, organization
  registration, enterprise registration, and runner group selection through
  extra labels
- by default, CodeBuild ignores the project buildspec for this runner flow
  unless `buildspec-override:true` is present
- each workflow job starts a build that runs one ephemeral GitHub Actions
  runner and terminates it when the job completes

Operational caveats from the AWS runner docs:

- if `PRE_BUILD` or `INSTALL` fails under `buildspec-override:true`, the runner
  never starts and the GitHub workflow job must be cancelled manually
- CodeBuild fetches the GitHub runner token during `DOWNLOAD_SOURCE`; if
  `PRE_BUILD` or `INSTALL` takes more than one hour, the token may expire
  before the runner starts

## GitHub and AWS Setup Surfaces

AWS requires a GitHub source connection for the CodeBuild project. Officially,
CodeBuild can connect to GitHub with the modes listed in
[GitHub and GitHub Enterprise Server access in CodeBuild](https://docs.aws.amazon.com/codebuild/latest/userguide/access-tokens-github-overview.html):

- a personal access token
- an OAuth app
- a Secrets Manager secret
- a GitHub App connection

For AWS access from workflow steps, GitHub documents OpenID Connect as the
preferred auth pattern for workflows that need to reach AWS APIs
([GitHub OIDC in AWS](https://docs.github.com/en/actions/how-tos/secure-your-work/security-harden-deployments/oidc-in-aws)):

- GitHub can issue an OIDC token instead of storing long-lived AWS credentials
  as GitHub secrets
- the workflow needs `permissions: id-token: write`
- AWS trust policy should constrain the `sub` claim so only the intended
  repository, branch, or environment can assume the role

Inference for ADL:

- prefer a GitHub App connection for the CodeBuild source relationship when
  available
- prefer GitHub OIDC plus tightly-scoped IAM roles for any AWS API access
  inside workflow steps
- avoid long-lived AWS keys in GitHub secrets for this lane unless a proven
  gap forces a narrower exception

## Proposed ADL Project Shape

If ADL runs a later pilot, the smallest plausible shape is:

- one dedicated runner project, for example
  `adl-validation-linux-medium`
- one dedicated workflow family for long or slow validation, for example
  `"[CI-CodeBuild] Slow Validation"`
- one dedicated workflow-job label:

```yaml
runs-on:
  - codebuild-adl-validation-linux-medium-${{ github.run_id }}-${{ github.run_attempt }}
```

- optional label overrides only when the lane truly needs them:
  - `image:<environment-type>-<image-identifier>`
  - `instance-size:<instance-size>`
  - `fleet:<fleet-name>`
  - `registration-group-id:<id>`

Recommended queue/use policy:

- keep `adl-ci` and `adl-coverage` on the existing fast/default path
- treat CodeBuild as a candidate lane only for long-running proof or
  validation work that is already painful locally
- start with one explicit workflow-name filter such as `\[CI-CodeBuild\]`
  instead of routing all workflow jobs through the runner project

## Security Boundary Analysis

GitHub's self-hosted runner docs note that self-hosted runners do not require a
clean instance for every job, while AWS documents that the CodeBuild-hosted
runner is ephemeral and terminates after a single workflow job
([GitHub self-hosted runners](https://docs.github.com/en/actions/concepts/runners/self-hosted-runners),
[AWS CodeBuild runner tutorial](https://docs.aws.amazon.com/codebuild/latest/userguide/action-runner.html)).

Interpretation:

- CodeBuild improves on the ordinary persistent self-hosted-runner story by
  using a per-job ephemeral runner
- that does not remove workflow trust risk, because the job still executes repo
  code and actions inside the runner environment

Security posture for an ADL pilot should therefore be:

1. keep the lane isolated from untrusted PR execution paths
2. avoid `pull_request_target` or similarly privileged flows for this runner
3. keep `GITHUB_TOKEN` permissions at minimum scope
4. pin third-party actions to full commit SHAs where possible
5. use GitHub OIDC for AWS access, not stored long-lived AWS keys
6. keep any runner group or project limited to the intended repository/workflow
   set

## Cost, Latency, and Observability Considerations

AWS pricing is usage-based:

- on-demand EC2 pricing charges by build duration in minutes
- on-demand Lambda pricing charges by build duration in seconds
- reserved EC2 fleets have a 60-minute minimum per instance
- Mac reserved instances have a 24-hour minimum
- additional charges may come from CloudWatch Logs, S3, KMS, and related AWS
  services

These pricing and billing characteristics come from the
[AWS CodeBuild pricing page](https://aws.amazon.com/codebuild/pricing/).

Interpretation for ADL:

- per-job cloud spend is easy to understand conceptually, but actual economics
  depend on the chosen compute type and the real long-lane runtime
- a narrow pilot is required before making strong cost claims for ADL
- reserved capacity is probably too large a first step for this repo's current
  evidence level

Latency considerations:

- every targeted GitHub job becomes a CodeBuild build submission
- queueing, webhook delivery, and runner startup become part of the lane tail
- project-name mismatch or webhook misrouting can present as a hanging GitHub
  job rather than a fast failure

Observability considerations:

- AWS says the GitHub workflow job logs remain visible in GitHub Actions
- AWS pricing also signals possible CloudWatch Logs usage and cost on the
  CodeBuild side
- ADL should treat GitHub job logs as the operator-facing surface and any AWS
  side logs as supplemental proof/debug material

## Recommended Pilot Gates

Revisit only if all of the following are true:

1. long validation still dominates developer wait time after the local changes
   from `#4311` through `#4314`
2. `#4317` or an equivalent owned-runner proof establishes the remote-runner
   abstraction on owned infrastructure first
3. the pilot is isolated to one named workflow and one runner project
4. GitHub-to-AWS auth is OIDC-based and least-privilege
5. the failure path is fail-closed and tail-friendly

## Recommended Outcome

`No pilot now; keep as a gated later pilot candidate.`

That is not a rejection of CodeBuild itself. It is a sequencing decision:

- Phase 1 local improvements were narrower and already useful
- the next proof step should stay narrower than cloud-runner rollout
- CodeBuild becomes more attractive only if the long validation tail remains
  stubborn after those earlier steps

## Non-Claims

- This packet does not claim CodeBuild is a bad fit in general.
- This packet does not claim ADL has measured live CodeBuild startup latency.
- This packet does not claim exact monthly AWS spend for ADL.
- This packet does not claim a GitHub App connection is the only valid source
  auth mode; it is a recommendation derived from the documented options plus
  least-privilege concerns.
- This packet does not approve live AWS resource creation in this issue.
