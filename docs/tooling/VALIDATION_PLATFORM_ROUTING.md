# Validation Platform Routing

`adl/tools/validation_manager.sh` can emit a first-class routing contract for
the validation platform scheduler. The routing contract explains which platform
should run a selected validation profile and why other platforms are rejected.

For small, low-risk fixes, pair this routing contract with
[`DEVELOPER_THROUGHPUT_FAST_LANE.md`](DEVELOPER_THROUGHPUT_FAST_LANE.md) so the
selected platform, focused proof, FastWork posture, and PR-watching behavior
stay proportional to the changed surface.

The manager does not launch paid cloud resources. It only emits dry-run routing
truth and wrapper commands. Live AWS Spot or CodeBuild runs still require the
platform wrapper's explicit live-run flag or workflow trigger.

## Command

```bash
bash adl/tools/validation_manager.sh \
  --changed-files <changed-files> \
  --platform-routing \
  --json
```

To ask about one platform directly:

```bash
bash adl/tools/validation_manager.sh \
  --changed-files <changed-files> \
  --validation-platform aws_spot \
  --json
```

Valid platform choices are:

```text
auto
local
nessus
aws_spot
codebuild
wuji
```

## JSON Contract

The output profile includes `platform_routing`:

```json
{
  "schema_version": "adl.validation_platform_routing.v1",
  "requested_platform": "auto",
  "decision": "selected",
  "selected_platform": "local",
  "no_launch": true,
  "launch_policy": "validation-manager only emits routing decisions and dry-run commands; live cloud runs require platform wrappers with explicit --run",
  "candidates": []
}
```

Each candidate records:

- `platform`
- `decision`
- `reason`
- `cache_posture`
- `cost_posture`
- optional `command`
- optional `wrapper`
- optional `caveats`

## Platform Semantics

Local:

- default route for docs-only or tiny profiles
- no cloud cost
- uses the selected validation profile's local command

Nessus:

- eligible for one selected non-tiny deterministic or evidence-bound lane
- uses `bash adl/tools/run_nessus_remote_validation.sh`
- expects the remote target and sccache posture recorded in the Nessus wrapper
  docs

AWS Spot:

- eligible for non-tiny deterministic or evidence-bound profiles
- uses `bash adl/tools/run_aws_spot_remote_validation_lane.sh`
- emits a dry-run command with `--print-command`
- live proof must still show the retained warm EBS cache attached at
  `/mnt/adl-cache`
- live execution may incur Spot compute and retained EBS storage cost

CodeBuild:

- route for scalable repeated CodeFriend-style builds once `#4838` lands
- expected wrapper: `bash adl/tools/run_aws_codefriend_build_lane.sh`
- expected cache posture: stable target cache plus S3 `sccache`
- on branches that do not yet contain the wrapper, the candidate fails closed
  with dependency caveats for `#4838` / PR `#4865`

Wuji:

- currently fails closed for scheduler routing because wuji is ARM and requires
  a separate arm64 builder image before parity can be claimed
- uses linked local target cache posture once the arm64 image exists

## Non-Claims

- `platform_routing.no_launch=true` is routing proof only.
- A Spot dry-run command is not warm-EBS proof.
- A CodeBuild candidate is not live CodeBuild proof.
- A cache posture label is not enough; retained summaries must prove cache
  attachment or cache hits for live benchmark claims.
