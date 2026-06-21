# Local Polis AWS SSM Proof for `#4319` (`opticon.local`)

## Scope

This packet records the bounded AWS Systems Manager proof for the local polis
storage/cache/operations host `opticon.local`.

The proof boundary is intentionally small:
- one QTS host
- one managed-node enrollment path
- one read-only status command
- one CloudWatch log group
- one QTS-specific repair/runbook path

This packet does not claim fleet rollout, Session Manager shell operations,
CloudWatch Agent rollout, storage-content export, SCR authority transfer, or
AWS authority over local polis state.

Related durable operator guide:
- `docs/tooling/QNAP_QTS_SSM_ONBOARDING.md`

## Status

Current state: `completed_for_issue_scope`

What is complete:
- `opticon.local` registered successfully in the approved AWS account as
  Systems Manager managed node `mi-01e6404947adc65ce`.
- The node appears `Online` in Systems Manager with platform `QTS 5.0.1` and
  agent version `3.3.4624.0`.
- The dedicated CloudWatch log group exists: `/adl/local-polis-ssm/4319`.
- A tracked QTS-safe wrapper now exists at
  `adl/tools/polis_status_for_ssm_qts.sh` and emits valid JSON over a bounded
  host/agent metadata set without requiring a repo checkout on-host.
- A bounded `AWS-RunShellScript` invocation successfully executed that wrapper
  on the managed node and published stdout to CloudWatch Logs.

## Bounded command surface

Tracked command surfaces:
- `adl/tools/polis_status_for_ssm.sh`
- `adl/tools/polis_status_for_ssm_qts.sh`

Expected behavior for the QTS wrapper:
- emits valid JSON only
- reports host label, OS, repo presence, branch, short commit, and whether the
  SSM agent is installed
- keeps repo fields fail-closed (`repo_present: false`, branch/commit unknown)
  when the host does not keep a local ADL checkout
- does not emit activation material, credentials, account IDs, private path
  inventories, storage contents, or archive contents

## Security and authority boundaries

The proof preserves these boundaries:
- AWS is an operations bridge, not the authority for local polis state.
- The command path is read-only.
- Tracked artifacts do not publish activation codes, credential files, account
  IDs, or private key material.
- CloudWatch proof is restricted to bounded operational status, not repo dumps,
  storage listings, or archive content.
- The QTS host requires direct command execution; shell-wrapped `/bin/sh -lc`
  probes are not a safe proof surface because they invoke the QTS console
  management menu.

## Operator runbook

### Prerequisites

- AWS-side hybrid activation creation rights in the approved account.
- Existing hybrid-node role `ADLHybridSSMManagedNodeRole`.
- Operator-approved admin/root login on `opticon.local`.
- Activation material created out of band and not tracked.

### Execution sequence

1. Create or confirm the dedicated CloudWatch log group
   `/adl/local-polis-ssm/4319`.
2. Create a single-use Systems Manager hybrid activation with default instance
   name `opticon` and role `ADLHybridSSMManagedNodeRole`.
3. Log into `opticon.local` with the approved admin/root account.
4. Download the Linux ARM64 SSM RPM and extract the required binaries to a
   persistent host path under `/share/Public/ssm-stage/usr/bin`.
5. Install `/opt/aws/ssm/amazon-ssm-agent`, register the host, and verify the
   managed node appears online.
6. Repair the QTS worker layout by symlinking `/usr/bin/ssm-agent-worker`,
   `/usr/bin/ssm-document-worker`, `/usr/bin/ssm-session-worker`,
   `/usr/bin/ssm-session-logger`, and `/usr/bin/ssm-cli` to the extracted
   staged binaries under `/share/Public/ssm-stage/usr/bin`.
7. Start the core agent with BusyBox `setsid` so QTS does not immediately tear
   the process down with the calling shell session.
8. Copy the tracked QTS wrapper to `/share/Public/adl-4319-polis-status-for-ssm-qts.sh`.
9. Run one bounded `AWS-RunShellScript` invocation against that direct wrapper
   path with CloudWatch output enabled.
10. Record the Systems Manager result, stdout, and CloudWatch log stream in this
    packet and the issue SOR.

For reusable operator guidance outside this issue packet, see:
- `docs/tooling/QNAP_QTS_SSM_ONBOARDING.md`

### Rollback

If the proof needs to be rolled back:
- stop the local SSM agent process on `opticon.local`
- deregister the managed node if the host should no longer participate in the
  AWS operations plane
- remove `/opt/aws/ssm/amazon-ssm-agent` and the `/usr/bin/ssm-*` symlinks if
  the host should not remain enrolled
- remove the staged extraction tree under `/share/Public/ssm-stage` if policy
  requires it
- keep the tracked packet truthful about what was installed, symlinked, removed,
  and left intentionally in place

## Live proof evidence

### Managed node registration

- Managed node: `mi-01e6404947adc65ce`
- Name: `opticon`
- Platform: `QTS`
- Version: `5.0.1`
- Ping status: `Online`
- Agent version: `3.3.4624.0`
- Registration date: `2026-06-20T18:56:46-07:00`

### IAM posture evidence

- Managed-node role: `ADLHybridSSMManagedNodeRole`
- This issue reuses the same bounded role posture proven for `#4113` rather
  than widening permissions for `opticon.local`.

### Bounded Run Command proof

Successful invocation:
- CommandId: `3b1c2b5d-721e-4be1-862f-a7f0c3947236`
- DocumentName: `AWS-RunShellScript`
- ResponseCode: `0`
- Status: `Success`

Sanitized stdout returned by the managed node:

```json
{
  "schema_version": "adl.local_polis_status.v1",
  "generated_at_utc": "2026-06-21T02:17:08Z",
  "host_label": "Opticon",
  "os_name": "QTS",
  "os_version": "5.0.1",
  "repo_name": "not_applicable",
  "repo_present": false,
  "git_branch": "unknown",
  "git_commit_short": "unknown",
  "ssm_agent_installed": true
}
```

### CloudWatch log proof

CloudWatch output was enabled for the bounded command and wrote to:
- log group: `/adl/local-polis-ssm/4319`
- log stream:
  `3b1c2b5d-721e-4be1-862f-a7f0c3947236/mi-01e6404947adc65ce/aws-runShellScript/stdout`

The CloudWatch event payload matched the sanitized stdout above.

## Implementation notes discovered during proof

The live proof uncovered and resolved three QTS-specific issues:
- the initial AWS account context was wrong on the operator machine until the
  proof was re-run under the intended `agent-logic-admin` profile
- the first partial worker installation silently truncated QTS worker binaries
  because `/usr` ran out of space; direct symlinks into the full staged copies
  under `/share/Public/ssm-stage/usr/bin` fixed the document/session worker
  crash path
- generic shell-wrapped probes such as `/bin/sh -lc "echo 1"` invoked the QTS
  console management menu instead of behaving like a normal shell proof surface;
  direct executable/script paths are required for deterministic SSM Run Command
  proof on this host

## Non-claims

This issue does not prove:
- local ADL checkout presence on `opticon.local`
- storage or cache inventory export
- storage contents, archive listings, or SCR private-file export
- Session Manager interactive shell operations as an approved operator workflow
- CloudWatch Agent installation or host-level metrics shipping
- fleet automation
- provider execution, scheduler authority, or polis-state authority through AWS
