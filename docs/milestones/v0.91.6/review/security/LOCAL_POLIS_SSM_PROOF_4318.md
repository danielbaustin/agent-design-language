# Local Polis AWS SSM Proof for `#4318` (`nessus.local`)

## Scope

This packet records the bounded AWS Systems Manager proof for the local polis
runtime/model host `nessus.local`.

The proof boundary is intentionally small:
- one Windows host
- one managed-node enrollment path
- one read-only status command
- one CloudWatch log group
- one host-specific runbook and rollback path

This packet does not claim fleet rollout, Session Manager shell operations,
CloudWatch Agent rollout, model upload, or any authority transfer from local
polis state into AWS.

## Status

Current state: `completed_for_issue_scope`

What is complete:
- A dedicated hybrid activation was created for `nessus`.
- `nessus.local` registered successfully as Systems Manager managed node
  `mi-0538c965d11eae809`.
- The node appears `Online` in Systems Manager with platform
  `Microsoft Windows 10 Home` and agent version `3.3.4624.0`.
- The dedicated CloudWatch log group exists:
  `/adl/local-polis-ssm/4318`.
- A tracked Windows wrapper now exists at
  `adl/tools/polis_status_for_ssm_windows.ps1` and emits valid JSON over a
  bounded set of host and repo-presence metadata fields.
- A bounded `AWS-RunPowerShellScript` invocation successfully executed the
  wrapper on the managed node and published stdout to CloudWatch Logs.

## Bounded command surface

Tracked command surface:
- `adl/tools/polis_status_for_ssm_windows.ps1`

Expected behavior:
- emits valid JSON only
- reports host label, OS, repo presence, branch, short commit, and whether the
  SSM agent is installed
- tolerates a host that does not keep a local ADL checkout by reporting
  `repo_present: false`
- does not emit activation material, credentials, account IDs, model contents,
  or private path inventories

## Security and authority boundaries

The proof preserves these boundaries:
- AWS is an operations bridge, not the authority for local polis state.
- The command path is read-only.
- Tracked artifacts do not publish activation codes, credential files, or
  private key material.
- CloudWatch proof is restricted to bounded operational status, not repo dumps,
  model content, or private path listings.
- The non-EC2 Windows registration path uses AWS's hybrid-node
  `ssm-setup-cli`, not the EC2-only installation flow.

## Operator runbook

### Prerequisites

- AWS-side hybrid activation creation rights in the approved account.
- Existing hybrid-node role `ADLHybridSSMManagedNodeRole`.
- Operator-approved admin login on `nessus.local`.
- Activation material created out of band and not tracked.

### Execution sequence

1. Create or confirm the dedicated CloudWatch log group
   `/adl/local-polis-ssm/4318`.
2. Create a single-use Systems Manager hybrid activation with default instance
   name `nessus` and role `ADLHybridSSMManagedNodeRole`.
3. Log into `nessus.local` with the approved admin account.
4. Download `ssm-setup-cli.exe` from the AWS regional S3 endpoint.
5. Run `ssm-setup-cli.exe -register` with the activation code, activation ID,
   and region `us-west-2`.
6. Confirm the host appears as an online managed node.
7. Copy the tracked wrapper to the host and invoke it once through
   `AWS-RunPowerShellScript` with CloudWatch output enabled.
8. Record the Systems Manager result, stdout, and CloudWatch log stream in this
   packet and the issue SOR.

### Rollback

If the proof needs to be rolled back:
- stop the local `AmazonSSMAgent` service on `nessus.local`
- deregister the managed node if the host should no longer participate in the
  AWS operations plane
- remove the local agent installation if the host should not remain enrolled
- remove the dedicated CloudWatch log group if policy requires it
- keep the tracked packet truthful about what was installed, removed, and left
  intentionally in place

## Live proof evidence

### Managed node registration

- Managed node: `mi-0538c965d11eae809`
- Name: `nessus`
- Platform: `Microsoft Windows 10 Home`
- Version: `10.0.19045`
- Ping status: `Online`
- Agent version: `3.3.4624.0`
- Registration date: `2026-06-20T17:32:58-07:00`

### IAM posture evidence

- Managed-node role: `ADLHybridSSMManagedNodeRole`
- Trust principal: `ssm.amazonaws.com`
- This issue reuses the same bounded role posture proven for `#4113` rather
  than widening permissions for `nessus.local`.

### Bounded Run Command proof

Successful invocation:
- CommandId: `1cba4d23-6ee4-42ad-b293-2042049458f0`
- DocumentName: `AWS-RunPowerShellScript`
- ResponseCode: `0`
- Status: `Success`

Sanitized stdout returned by the managed node:

```json
{
  "schema_version": "adl.local_polis_status.v1",
  "generated_at_utc": "2026-06-21T00:35:15Z",
  "host_label": "NESSUS",
  "os_name": "Windows",
  "os_version": "10.0.19045.0",
  "repo_name": "agent-design-language",
  "repo_present": false,
  "git_branch": "unknown",
  "git_commit_short": "unknown",
  "ssm_agent_installed": true
}
```

### CloudWatch log proof

CloudWatch output was enabled for the bounded command and wrote to:
- log group: `/adl/local-polis-ssm/4318`
- log stream:
  `1cba4d23-6ee4-42ad-b293-2042049458f0/mi-0538c965d11eae809/aws-runPowerShellScript/stdout`

The CloudWatch event payload matched the sanitized stdout above.

## Implementation notes discovered during proof

The live proof uncovered and resolved two host-specific issues:
- the first reused helper assumed the macOS `wuji` layout and looked for the
  activation file under the wrong Windows path
- the direct EC2-style Windows installer flow is not the correct hybrid-node
  path; the bounded proof switched to AWS's `ssm-setup-cli` hybrid
  registration flow instead

The final tracked wrapper is intentionally Windows-specific and keeps the same
JSON contract as the existing shell wrapper while tolerating a missing local
repo checkout.

## Non-claims

This issue does not prove:
- model inventory export from `nessus.local`
- local ADL checkout presence on the host
- Session Manager interactive shell operations
- CloudWatch Agent installation or host-level metrics shipping
- fleet automation
- provider execution, scheduler authority, or polis-state authority through AWS
