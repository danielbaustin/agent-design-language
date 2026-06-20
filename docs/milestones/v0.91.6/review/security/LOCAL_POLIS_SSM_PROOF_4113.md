# Local Polis AWS SSM Proof for `#4113`

## Scope

This packet records the first bounded AWS Systems Manager proof for the local polis host `wuji`.

The proof boundary is intentionally small:
- one host
- one managed-node enrollment path
- one read-only status command
- one CloudWatch log group
- one operator runbook and rollback path

This packet does not claim fleet rollout, autonomous repair, Session Manager shell operations, or any authority transfer from local polis state into AWS.

## Status

Current state: `completed_for_issue_scope`

What is complete:
- AWS-side hybrid activation was created and consumed for the proof host.
- Managed-node role evidence was captured for `ADLHybridSSMManagedNodeRole`, whose trust policy allows `ssm.amazonaws.com` to assume the role and whose attached managed policies are `AmazonSSMManagedInstanceCore` and `CloudWatchAgentServerPolicy`.
- Managed-node registration succeeded and the node appears in Systems Manager as `mi-0dd41a2b1cad222a0` with name `wuji` and `PingStatus: Online`.
- The dedicated CloudWatch log group exists: `/adl/local-polis-ssm/4113`.
- The bounded local status command exists at `adl/tools/polis_status_for_ssm.sh` and emits valid JSON via Python JSON serialization over a bounded set of host and git metadata fields.
- A bounded `AWS-RunShellScript` invocation successfully executed the wrapper on the managed node and published stdout to CloudWatch Logs.

## Bounded command surface

Tracked command surface:
- `adl/tools/polis_status_for_ssm.sh`

Expected behavior:
- emits valid JSON only
- reports host label, OS, repo presence, branch, short commit, and whether the SSM agent is installed
- does not emit credentials, activation secrets, model content, prompt content, or private path inventories beyond the explicitly required proof path

## Security and authority boundaries

The proof preserves these boundaries:
- AWS is an operations bridge, not the authority for local polis state.
- The command path is read-only.
- Tracked artifacts do not publish activation secrets, credential files, or private key material.
- Logging proof is restricted to bounded operational status, not repo content dumps or model content.
- The wrapper now explicitly tolerates root-owned SSM execution over a user-owned git worktree by using a scoped `git -c safe.directory=<worktree>` override for read-only branch/commit inspection.
- JSON serialization is delegated to Python's standard `json` module so host and git strings are escaped correctly for this proof surface.

## Operator runbook

### Prerequisites

- AWS-side proof setup completed for the approved account target.
- Local operator approval for one privileged package install on `wuji`.
- Local activation material available out of band and not tracked.

### Execution sequence

1. Install and register the Amazon SSM Agent on `wuji` using the prepared local helper.
2. Confirm the host appears in Systems Manager as a managed node.
3. Run one bounded read-only command through SSM that invokes `adl/tools/polis_status_for_ssm.sh` in the bound issue worktree.
4. Confirm the command result is sanitized and review-friendly.
5. Confirm the proof log stream exists under `/adl/local-polis-ssm/4113`.
6. Record the managed-node proof, command proof, and logging proof in this packet and the issue SOR.

### Rollback

If the proof needs to be rolled back:
- stop the local SSM agent service on `wuji`
- deregister the managed node if the host should no longer participate in the AWS operations plane
- remove the local SSM package if the host should not remain enrolled
- keep the tracked packet truthful about what was installed, removed, and left intentionally in place

## Live proof evidence

### Managed node registration

### IAM posture evidence

- Managed-node role: `ADLHybridSSMManagedNodeRole`
- Trust policy principal: `ssm.amazonaws.com`
- Attached managed policies:
  - `arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore`
  - `arn:aws:iam::aws:policy/CloudWatchAgentServerPolicy`
- This is the minimal posture proven in this issue bundle: Systems Manager managed-instance operation plus CloudWatch log delivery for the proof surface. The issue does not claim a custom least-privilege policy beyond those attached managed policies.

- Activation readiness after successful registration:
  - `RegistrationLimit: 1`
  - `RegistrationsCount: 1`
  - `DefaultInstanceName: wuji`
- Systems Manager inventory after registration:
  - `InstanceId: mi-0dd41a2b1cad222a0`
  - `Name: wuji`
  - `PlatformName: macOS`
  - `PingStatus: Online`

### Bounded Run Command proof

Successful invocation:
- `CommandId: 45099af7-836d-4b54-b1c7-3bae8a6ffee1`
- `DocumentName: AWS-RunShellScript`
- `ResponseCode: 0`
- `Status: Success`

Sanitized stdout returned by the managed node:

```json
{
  "schema_version": "adl.local_polis_status.v1",
  "generated_at_utc": "2026-06-20T03:22:13Z",
  "host_label": "wuji",
  "os_name": "macOS",
  "os_version": "26.5",
  "repo_name": "adl-wp-4113",
  "repo_present": true,
  "git_branch": "codex/4113-v0-91-6-tools-aws-ssm-implement-local-polis-ssm-proof",
  "git_commit_short": "ed862a65",
  "ssm_agent_installed": true
}
```

### CloudWatch log proof

CloudWatch output was enabled for the bounded command and wrote to:
- log group: `/adl/local-polis-ssm/4113`
- log stream: `45099af7-836d-4b54-b1c7-3bae8a6ffee1/mi-0dd41a2b1cad222a0/aws-runShellScript/stdout`

The CloudWatch event payload matched the sanitized stdout above.

## Implementation notes discovered during proof

The live proof uncovered and resolved three workflow-specific issues in the first wrapper cut:
- the original helper looked for activation variables under names that did not match the saved local activation file
- the first registration helper used the wrong managed-instance registration form by including the wrong assumptions from other registration modes
- the initial status wrapper underreported git state in a worktree because it treated `.git` as a directory and then hit root-vs-user git safety rules under SSM execution
- the initial status wrapper formatted JSON string fields with raw shell `%s` interpolation instead of a real JSON serializer

The final tracked wrapper handles bound worktrees correctly for this proof surface.

## Non-claims

This issue does not prove:
- multi-host enrollment
- fleet automation
- Session Manager shell access as an approved operator workflow
- S3 export as a required proof surface
- OpenTelemetry, observatory, or broader runtime observability integration
