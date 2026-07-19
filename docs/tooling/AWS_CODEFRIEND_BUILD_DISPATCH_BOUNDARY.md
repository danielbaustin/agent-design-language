# AWS CodeFriend Build Dispatch Boundary

The `aws-codefriend-build` workflow is a manual, operator-dispatched path.
Only repository maintainers with GitHub Actions workflow-dispatch permission
should use it. It is not a pull-request trigger and it is not an unattended
provider endpoint.

## Allowed commands

The workflow input is treated as untrusted text at both boundaries:

1. `adl/tools/run_aws_codefriend_build_lane.sh` validates the command before it
   is placed in the CodeBuild request.
2. The generated CodeBuild buildspec validates it again immediately before
   execution.

The only accepted build commands are:

- `bash adl/tools/run_pr_fast_test_lane.sh`
- `bash adl/tools/run_pr_fast_coverage_lane.sh`
- `bash adl/tools/run_authoritative_coverage_lane.sh`
- the pinned 18-thread `cargo nextest` command emitted by `--full-nextest`

Shell composition, arbitrary interpreters, command substitution, and additional
arguments are rejected. A rejected command emits
`classification=unapproved_build_command` and the build stops before compiling.

## Trust and retained evidence

- Live runs require the approved Agent Logic account hash check and an explicit
  source branch, tag, or commit; `HEAD` is rejected.
- The workflow retains the redacted request, response, status, and CloudWatch
  log artifacts for 14 days.
- Account IDs, ARNs, access keys, authorization values, and provider-like
  secrets are redacted before logs are retained or printed.
- The build emits source, image, toolchain, cache, and command markers; the
  wrapper reports them as self-verification evidence rather than inferring
  success from the CodeBuild status alone.
- This boundary does not authorize arbitrary AWS actions or make model output
  authoritative. It governs one bounded build/validation command.

The local contract proof is:

```text
bash adl/tools/test_run_aws_codefriend_build_lane.sh
```
