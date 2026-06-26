# v0.91.6 Runtime AWS SSM Health Proof for `#4545`

## Scope

This packet records the bounded live AWS proof for `#4545`:
- verify the intended AWS account/profile for the local polis fleet
- classify the CloudWatch log path truthfully
- verify live SSM node health for `wuji`, `nessus`, and `opticon`

This packet does not claim:
- scheduler integration
- provider execution through SSM
- fleet cleanup beyond the named residuals
- host metrics shipping or CloudWatch Agent rollout

## Status

Current state: `completed_for_issue_scope`

What is complete:
- The intended AWS account/profile for this surface was verified as
  `agent-logic-admin`.
- The default profile was confirmed to be the wrong account for this issue's
  fleet proof and only exposed stale legacy managed nodes.
- `wuji`, `nessus`, and `opticon` were each verified through one bounded live
  SSM command on June 26, 2026 under the `agent-logic-admin` profile.
- CloudWatch output was enabled for each bounded command and a matching log
  stream was observed in the retained log groups.
- `wuji` required a narrow wrapper hardening fix so the local status script no
  longer fails when `HOME` is unset under SSM execution.

## Account posture

- Wrong-account symptom:
  the default AWS profile only listed stale managed nodes unrelated to the
  intended current local polis fleet.
- Intended account truth:
  `AWS_PROFILE=agent-logic-admin` in `us-west-2` exposed the expected current
  host set for this issue: `wuji`, `nessus`, and `opticon`.

This issue therefore classifies the AWS profile posture as:
- `default`: wrong account for this proof surface
- `agent-logic-admin`: correct account for this proof surface

## CloudWatch classification

CloudWatch classification: `live_proven`

Observed retained log groups:
- `/adl/local-polis-ssm/4113`
- `/adl/local-polis-ssm/4318`
- `/adl/local-polis-ssm/4319`

For the June 26, 2026 bounded re-verification:
- each host command reported one matching CloudWatch log stream under the
  expected retained group
- no SNS mutation was required or claimed by this issue

## Host health classification

### `wuji`

Classification: `healthy_after_wrapper_fix`

Live proof:
- `wuji` appeared `Online` in Systems Manager under `agent-logic-admin`
- the bounded `AWS-RunShellScript` invocation succeeded after the wrapper fix
- stdout returned valid JSON from `adl/tools/polis_status_for_ssm.sh`
- CloudWatch output was observed in `/adl/local-polis-ssm/4113`

Issue-local fix:
- `adl/tools/polis_status_for_ssm.sh` now derives the repo root from the script
  path first and only falls back to `HOME` if needed
- this avoids strict-shell failure when SSM launches the command with `HOME`
  unset

### `nessus`

Classification: `healthy`

Live proof:
- `nessus` appeared `Online` in Systems Manager under `agent-logic-admin`
- the bounded `AWS-RunPowerShellScript` invocation succeeded
- stdout reported the `AmazonSSMAgent` service in the running state
- CloudWatch output was observed in `/adl/local-polis-ssm/4318`

### `opticon`

Classification: `healthy`

Live proof:
- one `opticon` managed-node registration appeared `Online` in Systems Manager
  under `agent-logic-admin`
- the bounded `AWS-RunShellScript` invocation against the retained
  `/share/Public/adl-4319-polis-status-for-ssm-qts.sh` path succeeded
- stdout returned valid JSON from the QTS wrapper
- CloudWatch output was observed in `/adl/local-polis-ssm/4319`

Cleanup completed:
- the stale duplicate `opticon` registration that was still showing
  `ConnectionLost` was deregistered under `agent-logic-admin`
- post-cleanup fleet verification shows one live `opticon` managed node only

## Commands run

Focused commands used for the retained proof included:
- `AWS_PROFILE=agent-logic-admin AWS_REGION=us-west-2 aws ssm describe-instance-information --filters Key=ResourceType,Values=ManagedInstance --output json`
  Verified the correct fleet appears only under the intended account.
- `AWS_PROFILE=agent-logic-admin AWS_REGION=us-west-2 aws logs describe-log-groups --log-group-name-prefix /adl/local-polis-ssm/4113 --output json`
  Verified the retained `wuji` CloudWatch log group exists.
- `AWS_PROFILE=agent-logic-admin AWS_REGION=us-west-2 aws logs describe-log-groups --log-group-name-prefix /adl/local-polis-ssm/4318 --output json`
  Verified the retained `nessus` CloudWatch log group exists.
- `AWS_PROFILE=agent-logic-admin AWS_REGION=us-west-2 aws logs describe-log-groups --log-group-name-prefix /adl/local-polis-ssm/4319 --output json`
  Verified the retained `opticon` CloudWatch log group exists.
- bounded `send-command` / `get-command-invocation` / `describe-log-streams`
  checks for each host
  Verified live SSM execution status plus CloudWatch stream presence.
- `env -u HOME bash adl/tools/polis_status_for_ssm.sh`
  Reproduced the `wuji` SSM environment locally and proved the wrapper hardening
  before re-running the live host command.

## Residuals

- The default AWS profile remains a wrong-account footgun for this proof
  surface and should not be used for the local polis fleet checks.

## Non-claims

This issue does not prove:
- scheduler authority through AWS
- provider/model execution through SSM
- Session Manager interactive-shell approval
- CloudWatch Agent host metrics rollout
- duplicate managed-node cleanup automation
